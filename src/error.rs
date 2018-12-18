use crate::state::State;
use crate::timeline::TimelineError;
use std::error::Error as StdError;
use std::fmt;

/// An errortype that is used by any function in the actions-library that can return an error.
pub enum Error<S: State> {
    /// An error concerning a `Timeline`.
    Timeline(TimelineError<S::Error>),
}

impl<S: State> From<TimelineError<S::Error>> for Error<S>
where
    S::Error: fmt::Debug,
{
    fn from(source: TimelineError<S::Error>) -> Error<S> {
        Error::Timeline(source)
    }
}

impl<S: State> fmt::Debug for Error<S>
where
    S::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Timeline(e) => write!(f, "Error::Timeline( {:?} )", e),
        }
    }
}

impl<S: State> fmt::Display for Error<S>
where
    S::Error: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Timeline(e) => write!(f, "[Timeline error]: {}", e),
        }
    }
}

// Implement std::Error for Error if the API consumer has implemented the
// required traits on their own error-type.
//
// The API consumer will not have to do anything special for this.
impl<S: State> StdError for Error<S>
where
    S: fmt::Debug,
    S::Error: fmt::Display + fmt::Debug,
{
}
