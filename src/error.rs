use component::ApplyError;
use std::error::Error;
use std::fmt;
use timeline::TimelineError;

#[derive(Debug)]
/// The errors defined by the actions library.
pub enum ActionsError {
    /// An error caused by a timeline.
    Timeline(TimelineError),
    /// An error caused by applying an action.
    Apply(ApplyError),
}

impl fmt::Display for ActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionsError::Timeline(err) => err.fmt(f),
            ActionsError::Apply(err) => err.fmt(f),
        }
    }
}

impl Error for ActionsError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            ActionsError::Timeline(err) => Some(err),
            ActionsError::Apply(err) => Some(err),
        }
    }
}

impl From<TimelineError> for ActionsError {
    fn from(err: TimelineError) -> ActionsError {
        ActionsError::Timeline(err)
    }
}

impl From<ApplyError> for ActionsError {
    fn from(err: ApplyError) -> ActionsError {
        ActionsError::Apply(err)
    }
}
