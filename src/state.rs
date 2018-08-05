use std::error::Error as stdError;
use error::ActionsError;
use std::fmt;

#[derive(Debug)]
/// An error that occures while reducing.
pub struct ReduceError(pub Box<dyn stdError>);

impl fmt::Display for ReduceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReduceError(ref err) => err.fmt(f),
        }
    }
}

impl stdError for ReduceError {
    fn cause(&self) -> Option<&dyn stdError> {
        match *self {
            ReduceError(ref err) => Some(err.as_ref())
        }
    }
}

/// Reduce is a trait that should be implemented for any datatype that describes the state of your application.

pub trait Reduce<Error: stdError + 'static = ActionsError> {
    /// The type of the action.
    /// The action could be of any type but using an `enum` is encouraged.
    type Action: Clone;

    /// Apply an action and return the inverse of that action.
    ///
    /// # Arguments
    ///
    /// * `action` - Data that describes an action.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate actions;
    /// use actions::Reduce;
    /// use actions::Error;
    /// 
    /// #[derive(Clone)]
    /// struct Counter {
    ///     counter: i32
    /// }
    ///
    /// #[derive(Clone)]
    /// enum CounterAction {
    ///     Increment,
    ///     Decrement,
    /// }
    ///
    /// impl Reduce for Counter {
    ///     type Action = CounterAction;
    ///
    ///     fn apply_action(&mut self, action: &Self::Action)->Result<Option<Self::Action>,Error>
    ///     {
    ///         let inverse = match action {
    ///             CounterAction::Increment => {
    ///                 self.counter += 1;
    ///                 Some(CounterAction::Decrement)
    ///             },
    ///             CounterAction::Decrement => {
    ///                 self.counter -= 1;
    ///                 Some(CounterAction::Increment)
    ///             }
    ///         };
    ///         Ok(inverse)
    ///     }
    /// }
    /// ```
    fn apply_action(&mut self, action: &Self::Action) -> Result<Option<Self::Action>, Error>;
}
