use action::{Merge, MergeResult};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ChainError {}

impl Error for ChainError {}

impl fmt::Display for ChainError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

/// A chain of actions.
///
/// Can be used to store 'macros'.
#[derive(Clone)]
pub struct Chain<Action>
where
    Action: Merge,
{
    actions: Vec<Action>,
}

impl<Action: Merge> Chain<Action> {
    /// Create a new chain of actions.
    ///
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Merges as much actions in the chain as possible.
    ///
    /// For example, if the chain contains an action that cancels out the
    /// previous action, both of the actions can be removed from the chain.
    pub fn compress(&mut self) {
        /* 
            TODO: improve algorithm! Right now, it is more like a proof of concept.
            The whole content of this funciton is nasty!
            
            Optimizations:
                - Check for the index of the last 'Overwrites' result first 
                  (from the back) and start from that index.
                - Right now the last action is merged but just added. That
                  is a problem. Imagine this:
                  100 unmergable actions and 1 overwriting action at the end.
                  It should merge to 1 action but it doesn't using this
                  algorithm.
                - It is ran once now, but it should go over the actions multiple times
                  and stop if there are no changes made in the last run. Riight?
                - Other optimizations?
            And I think there is some performance to gain by better aligningment
            of the data in memory (cache!) and of course calculating asynchonously.
        */
        let length_old = self.actions.len();

        if length_old <= 1 {
            return;
        }

        let mut result: Vec<Option<Action>> = vec![None; length_old];
        let mut i_write = 0;
        let mut i_read = 0;

        while i_read < length_old {
            let current_action = &self.actions[i_read];

            if i_write == 0 {
                result[i_write] = Some(self.actions[i_read].clone());
                i_write += 1;
            } else {
                match current_action.merge(&result[i_write - 1].clone().unwrap()) {
                    MergeResult::Unmergable => {
                        result[i_write] = Some(current_action.clone());
                        i_write += 1;
                    }
                    MergeResult::CancelsOut => {
                        i_write -= 1;
                    }
                    MergeResult::Overwrites => {
                        result[0] = Some(current_action.clone());
                        i_write = 1;
                    }
                    MergeResult::Merged(action) => {
                        result[i_write - 1] = Some(action);
                    }
                };
            }

            i_read += 1;
        }

        result.truncate(i_write);

        // println!("Compressed actionchain from {} to {}", length_old, result.len());

        self.actions = result.into_iter().map(|x| x.unwrap()).collect();
    }

    /// Push an action to the chain.
    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// The amount of action currently in the chain.
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// The actions in the chain.
    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }
}

impl<Action> From<Vec<Action>> for Chain<Action>
where
    Action: Merge,
{
    fn from(vector: Vec<Action>) -> Chain<Action> {
        Self { actions: vector }
    }
}

impl<Action> Into<Vec<Action>> for Chain<Action>
where
    Action: Merge,
{
    fn into(self) -> Vec<Action> {
        self.actions
    }
}
