use crate::merge::{Merge, MergeResult};

#[derive(Clone, Debug)]
/// A 'chain' of actions.
///
/// Represents a number of consecutive actions.
pub struct Chain<Action: Sized>
where
    Action: Clone,
{
    chain: Vec<Action>,
}

impl<A: Sized + Clone> Chain<A> {
    /// Return the underlying vector of actions.
    pub fn actions(&self) -> &[A] {
        &self.chain
    }

    /// Compress the chain.
    ///
    /// Executing the compressed chain should result in the exact
    /// same mutation of the data as the original chain.
    pub fn compress(&mut self)
    where
        A: Merge,
    {
        let length_old = self.chain.len();

        if length_old <= 1 {
            return;
        }

        let mut result: Vec<Option<A>> = vec![None; length_old];
        let mut i_write = 0;
        let mut i_read = 0;

        while i_read < length_old {
            let current_action = &self.chain[i_read];

            if i_write == 0 {
                result[i_write] = Some(self.chain[i_read].clone());
                i_write += 1;
            } else {
                match (*current_action).merge(&result[i_write - 1].clone().unwrap()) {
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

        self.chain = result.into_iter().map(|x| x.unwrap()).collect();
    }

    /// Return a new empty chain of actions.
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }

    /// Return a new empty chain of actions with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chain: Vec::with_capacity(capacity),
        }
    }

    /// Add an action to the chain.
    ///
    /// The action will always be added to the back of the chain.
    pub fn push(&mut self, action: A) {
        self.chain.push(action);
    }

    /// Return the length of the chain (the number of actions it contains).
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Clear the chain.
    ///
    /// This will remove any action currenty in the chain.
    /// After calling this function, the length of the chain will equal 0.
    pub fn clear(&mut self) {
        self.chain.clear()
    }
}

impl<A: Sized + Clone> Into<Vec<A>> for Chain<A> {
    fn into(self) -> Vec<A> {
        self.chain
    }
}

impl<A: Sized + Clone> From<Vec<A>> for Chain<A> {
    fn from(vec: Vec<A>) -> Self {
        Self { chain: vec }
    }
}
