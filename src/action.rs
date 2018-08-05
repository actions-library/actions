/// The result of merging two actions.
pub enum MergeResult<Action: Merge> {
    /// The actions are not mergable into one action.
    Unmergable,
    /// The actions are merged into one action.
    Merged(Action),
    /// The action cancels out the effect of the previous action.
    CancelsOut,
    /// The action fully overwrites the previous action.
    /// In this case, it does not matter how the previous
    /// action influenced the state.
    Overwrites,
}

/// Trait that enables actions to merge.
pub trait Merge
where
    Self: Sized + Clone
{
    /// Merge two actions.
    /// Caution! The order of actions matters!
    /// **The _previous_ action is executed first, then &self.**
    ///
    /// # Arguments
    /// previous: The previous action.
    fn merge(&self, previous: &Self) -> MergeResult<Self>;
}
