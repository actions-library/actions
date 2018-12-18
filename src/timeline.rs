use crate::state::{InverseResult, State};

use std::fmt;
use std::ops::Deref;

use crate::chain::Chain;

enum Breadcrumb<S: State> {
    FullCopy(Box<S>),
    Action(S::Action),
}

/// The `Timeline` wraps around a `State` and makes it accessible by using actions only.
///
/// It stores a history of actions, so that actions can be easily undone or redone.
///
/// It is **highly recommended** to implement `fn inverse(&self, action)` on the state.
/// If the inverse function is not implemented, the Timeline will create
/// a full clone of the state *on the heap* before every action.
pub struct Timeline<S: State + Clone> {
    timeline: Vec<(S::Action, Breadcrumb<S>)>,
    available_undos: usize,
    current_state: S,
}

#[derive(Debug)]
pub enum TimelineError<E> {
    NothingToUndo,
    NothingToRedo,
    ApplyError(E),
}

impl<'a, E: fmt::Display> fmt::Display for TimelineError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimelineError::NothingToUndo => write!(f, "No actions left to undo."),
            TimelineError::NothingToRedo => write!(f, "No actions left to redo."),
            TimelineError::ApplyError(e) => write!(f, "Applying action failed: {}", e),
        }
    }
}

impl<S: State + Clone> Timeline<S>
where
    S::Action: Clone,
{
    /// Create a new `Timeline` which wraps around an *item*.
    ///
    /// # Arguments
    ///
    /// - state: A datastructure containing the state.
    pub fn new(state: S) -> Self {
        Self {
            timeline: Vec::new(),
            current_state: state,
            available_undos: 0,
        }
    }

    /// Get a reference to the current state.
    /// 
    /// Can be useful to inspect state.
    pub fn current_state(&self) -> &S {
        &self.current_state
    }

    /// Apply an action.
    pub fn apply<'a>(&mut self, action: <S as State>::Action) -> Result<(), TimelineError<S::Error>>
    where
        S: 'a,
    {
        let breadcrumb = match self.current_state.inverse(&action) {
            InverseResult::Action(inverse) => Breadcrumb::Action(inverse),
            InverseResult::FullCopyRequired => {
                Breadcrumb::FullCopy(Box::new(self.current_state.clone()))
            }
        };

        match self.current_state.apply(&action) {
            Ok(()) => {
                assert!(self.timeline.len() >= self.available_undos);

                if self.timeline.len() == self.available_undos {
                    self.timeline.push((action, breadcrumb));
                } else
                /*if self.timeline.len() > self.available_undos*/
                {
                    self.timeline[self.available_undos] = (action, breadcrumb);
                    self.timeline.truncate(self.available_undos + 1);
                }

                self.available_undos += 1;
            }
            Err(e) => return Err(TimelineError::ApplyError(e)),
        };
        Ok(())
    }

    /// Apply a chain of commands.
    ///
    /// Undoing undoes the actions in the chain one by one.
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error.
    pub fn apply_chain(&mut self, chain: &Chain<S::Action>) -> Result<(), TimelineError<S::Error>> {
        for action in chain.actions() {
            // TODO: What to do when an error is returned mid-chain??
            self.apply(action.clone())?;
        }

        Ok(())
    }

    /// Go one step back in history.
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error if 
    /// there are no actions left to undo.
    pub fn undo(&mut self) -> Result<(), TimelineError<S::Error>> {
        if self.available_undos == 0 {
            return Err(TimelineError::NothingToUndo);
        };

        match self.timeline[self.available_undos - 1].1 {
            Breadcrumb::Action(ref action) => {
                self.current_state
                    .apply(&action)
                    .map_err(TimelineError::ApplyError)?;
            }
            Breadcrumb::FullCopy(ref state) => {
                self.current_state = state.deref().clone();
            }
        };

        self.available_undos -= 1;

        Ok(())
    }

    /// Go one step forward in history.
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error if 
    /// there are no actions left to redo.
    pub fn redo(&mut self) -> Result<(), TimelineError<S::Error>> {
        if self.timeline.len() == self.available_undos {
            return Err(TimelineError::NothingToRedo);
        };

        let index = self.timeline.len() - self.available_undos;

        self.current_state
            .apply(&self.timeline[index].0)
            .map_err(TimelineError::ApplyError)?;

        self.available_undos += 1;

        Ok(())
    }

    /// Return the number of undo's that can be performed.
    pub fn undos_remaining(&self) -> usize {
        self.available_undos
    }

    /// Return the number of redo's that can be performed.
    pub fn redos_remaining(&self) -> usize {
        self.timeline.len() - self.available_undos
    }
}
