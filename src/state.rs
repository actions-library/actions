/// State is a trait that should be implemented for any datatype that describes the state of your application.
pub trait State {
    /// The type of the Action that describes how this state can be modified.
    /// The action could be of any type but using an `enum` is encouraged.
    type Action: Sized;

    /// The type of the Error that can be returned when trying to apply an
    /// action.
    type Error;

    /// Apply an action.
    ///
    /// # Arguments
    ///
    /// * `action` - Data that describes an action.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// #
    /// # extern crate actions;
    /// # use actions::State;
    /// #
    /// # struct Counter {
    /// #     value: i32
    /// # }
    /// #
    /// # struct CounterError {}
    /// #
    /// # #[derive(Clone)]
    /// # enum CounterAction {
    /// #     Increment,
    /// #     Decrement,
    /// # }
    /// #
    /// # impl State for Counter {
    /// #     type Action = CounterAction;
    /// #     type Error = CounterError;
    /// #
    ///     fn apply(&mut self, action: &Self::Action) -> Result<(), CounterError>
    ///     {
    ///         match action {
    ///             CounterAction::Increment => {
    ///                 self.value += 1;
    ///             },
    ///             CounterAction::Decrement => {
    ///                 self.value -= 1;
    ///             }
    ///         };
    ///         Ok(())
    ///     }
    /// # }
    /// ```
    fn apply(&mut self, action: &Self::Action) -> Result<(), Self::Error>;

    /// Take a action and return the action that undos that action.
    ///
    /// Implementing the inverse function is optional.
    /// The Timeline uses it to undo actions.
    fn inverse(&self, _action: &Self::Action) -> InverseResult<Self::Action> {
        InverseResult::FullCopyRequired
    }
}

/// The result of getting a inverse of an action.
pub enum InverseResult<A>
where
    A: Sized,
{
    /// An action that fully reverses the effect.
    Action(A),
    /// Should be used in the case that it is less expensive to store a full
    /// copy of the original than to revert the action.
    FullCopyRequired,
}
