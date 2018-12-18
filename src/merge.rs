/// The result of trying to merge two actions.
pub enum MergeResult<Action: Sized> {
    /// The action cancels out the effect of the previous action.
    CancelsOut,
    /// The actions cannot be merged.
    Unmergable,
    /// The action fully overwrites the whole state.
    /// In this case, it does not matter how the previous
    /// action influenced the state. It is overwritten anyway.
    Overwrites,
    /// The actions are merged into one action.
    Merged(Action),
}

/// Trait that enables actions to merge actions.
pub trait Merge
where
    Self: Sized,
{
    /// Merge two actions.
    /// Caution! The order of actions matters!
    /// **The _previous_ action is executed first, then &self.**
    ///
    /// # Arguments
    /// previous: The previous action.
    fn merge(&self, previous: &Self) -> MergeResult<Self>;
}
