use state::ReduceError;
use std::error::Error;
use std::fmt;
use timeline::TimelineError;

#[derive(Debug)]
/// The errors defined by the actions library.
pub enum ActionsError {
    /// An error caused by a timeline.
    Timeline(TimelineError),
    /// An error caused by updating a state.
    Reduce(ReduceError),
}

impl fmt::Display for ActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActionsError::Timeline(ref err) => err.fmt(f),
            ActionsError::Reduce(ref err) => err.fmt(f),
        }
    }
}

impl Error for ActionsError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ActionsError::Timeline(ref err) => Some(err),
            ActionsError::Reduce(ref err) => Some(err),
        }
    }
}

impl From<TimelineError> for ActionsError {
    fn from(err: TimelineError) -> ActionsError {
        ActionsError::Timeline(err)
    }
}

impl From<ReduceError> for ActionsError {
    fn from(err: ReduceError) -> ActionsError {
        ActionsError::Reduce(err)
    }
}
