use action::Merge;
use chain::Chain;
use error::ActionsError;
use component::{Component, ApplyError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TimelineError {
    NothingToUndo,
    NothingToRedo,
}

impl Error for TimelineError {}

impl fmt::Display for TimelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimelineError::NothingToUndo => {
                write!(f, "Nothing to undo: There are no actions left to undo.")
            }
            TimelineError::NothingToRedo => {
                write!(f, "Nothing to redo: There are no actions left to redo.")
            }
        }
    }
}

/// Wraps data and makes it accessible by using actions only.
///
/// Allows for undoing and redoing.
pub struct Timeline<T: Component> {
    item: T,
    action_stack: Vec<T::Action>,
    inv_action_stack: Vec<T::Action>,
    action_index: i32,
    max_index: i32,
}

impl<T: Component> Timeline<T> {
    /// Create a new timeline which wraps an *item*.
    ///
    /// # Arguments
    ///
    /// - item: A datastructure containing a state.
    pub fn new(item: T) -> Self {
        Self {
            item,
            action_stack: Vec::new(),
            inv_action_stack: Vec::new(),
            action_index: -1,
            max_index: -1,
        }
    }

    /// Apply an action.
    ///
    /// The action can be undone and redone after applying.
    ///
    /// # Arguments
    /// - action: The action to apply
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error which occured during applying.
    pub fn apply(&mut self, action: &T::Action) -> Result<(), ActionsError> {
        // Increase the index of the current action.
        let action_inverse = match self.item.apply(action) {
            Ok(action) => action,
            Err(e) => return Err(ApplyError(Box::new(e)).into()),
        };

        // If a state-change was commited
        if let Some(inverse) = action_inverse {
            let current_length = self.action_stack.len() as i32;
            self.action_index += 1;

            // println!("applying while the current length is {}", current_length);
            if self.action_index < current_length {
                let index = self.action_index as usize;
                // Overwrite the action currently at that index.
                self.action_stack[index] = action.clone();
                // Put the returned inverse onto the rev_action_stack.
                self.inv_action_stack[index] = inverse;
            } else {
                // Push a copy of the action to the action_stack.
                self.action_stack.push(action.clone());
                // Push the returned inverse onto the rev_action_stack.
                self.inv_action_stack.push(inverse);
            }

            // Do not allow to redo past this point.
            self.max_index = self.action_index;
        }
        Ok(())
    }

    /// Apply a chain of commands.
    /// Undoing undo's the actions in the chain individually.
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error.
    pub fn apply_chain(&mut self, chain: Chain<T::Action>) -> Result<(), ActionsError>
    where
        <T as Component>::Action: Merge,
    {
        for action in chain.actions() {
            self.apply(action)?;
        }

        Ok(())
    }

    /// Undo the last performed action.
    ///
    /// # Return
    /// A result containing either an empty `Ok` or an error.
    ///
    /// An error can be caused by the reducer throwing an error,
    /// or because there are no actions to undo.
    pub fn undo(&mut self) -> Result<(), ActionsError> {
        if self.action_index == -1 {
            return Err(TimelineError::NothingToUndo.into());
        }

        // Get a reference to the undo-action and apply it.
        let action_undo = &self.inv_action_stack[self.action_index as usize];

        match self.item.apply(action_undo) {
            Ok(_) => (),
            Err(err) => return Err(ApplyError(Box::new(err)).into()),
        };

        self.action_index -= 1;

        Ok(())
    }

    /// Redo the action performed after the current action.
    pub fn redo(&mut self) -> Result<(), ActionsError> {
        self.action_index += 1;

        if self.action_index > self.action_stack.len() as i32 {
            return Err(TimelineError::NothingToRedo.into());
        }

        // Get a reference to the redo-action.
        let action_redo = &self.action_stack[self.action_index as usize];

        match self.item.apply(action_redo) {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplyError(Box::new(err)).into()),
        }
    }

    /// The amount of undo-actions that can be performed.
    pub fn undos_remaining(&self) -> i32 {
        self.action_index + 1
    }

    /// The amount of redo-actions that can be performed.
    pub fn redos_remaining(&self) -> i32 {
        self.max_index - self.action_index
    }

    /// An immutable reference to the current state of the item.
    pub fn current(&self) -> &T {
        &self.item
    }
}
