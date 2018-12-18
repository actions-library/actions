use actions::InverseResult;
use actions::State;

use actions::{Merge, MergeResult};

#[derive(Clone, Default, Debug)]
pub struct Counter(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum CounterAction {
    Increment,
    Decrement,
    SetValue(u32),
    Divide(u32),
    Multiply(u32),
}

#[derive(Debug)]
pub enum CounterError {
    MinValueReached,
    MaxValueReached,
    WouldOverflow,
}

use std::fmt;
impl fmt::Display for CounterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CounterError::MinValueReached => write!(
                f,
                "Tried to decrement while the value of the counter was equal to 0."
            ),
            CounterError::MaxValueReached => write!(
                f,
                "Tried to increment while the maximum value of the counter was reached: {}.",
                <u32>::max_value()
            ),
            CounterError::WouldOverflow => write!(
                f,
                "Tried to divide or multiply but the value of the counter would overflow."
            ),
        }
    }
}

impl State for Counter {
    // Define what actions influence this State
    type Action = CounterAction;
    type Error = CounterError;

    // The function that applies the action!
    // This is where the magic happens.
    fn apply(&mut self, action: &CounterAction) -> Result<(), CounterError> {
        match action {
            CounterAction::Increment => {
                if self.0 == <u32>::max_value() {
                    return Err(CounterError::MaxValueReached);
                }
                self.0 += 1;
            }
            CounterAction::Decrement => {
                if self.0 == <u32>::min_value() {
                    return Err(CounterError::MinValueReached);
                }
                self.0 -= 1;
            }
            CounterAction::SetValue(v) => {
                self.0 = *v;
            }
            CounterAction::Divide(v) => {
                self.0 /= v;
            }
            CounterAction::Multiply(v) => {
                // Check for overflow
                match self.0.checked_mul(*v) {
                    Some(new_value) => self.0 = new_value,
                    None => {
                        return Err(CounterError::WouldOverflow);
                    }
                };
            }
        }

        Ok(())
    }

    fn inverse(&self, action: &CounterAction) -> InverseResult<CounterAction> {
        match action {
            CounterAction::Increment => InverseResult::Action(CounterAction::Decrement),
            CounterAction::Decrement => InverseResult::Action(CounterAction::Increment),
            CounterAction::Multiply(v) => InverseResult::Action(CounterAction::Divide(*v)),
            CounterAction::Divide(v) => InverseResult::Action(CounterAction::Multiply(*v)),
            CounterAction::SetValue(_) => InverseResult::FullCopyRequired,
        }
    }
}

impl Merge for CounterAction {
    fn merge(&self, previous: &CounterAction) -> MergeResult<Self> {
        match self {
            CounterAction::Increment => match previous {
                CounterAction::Decrement => MergeResult::CancelsOut,
                _ => MergeResult::Unmergable,
            },
            CounterAction::Decrement => match previous {
                CounterAction::Increment => MergeResult::CancelsOut,
                _ => MergeResult::Unmergable,
            },
            CounterAction::Divide(v1) => match previous {
                CounterAction::Divide(v2) => MergeResult::Merged(CounterAction::Divide(*v1 * *v2)),
                CounterAction::Multiply(v2) => {
                    if v2 % v1 == 0 {
                        MergeResult::Merged(CounterAction::Multiply(*v2 / *v1))
                    } else {
                        MergeResult::Unmergable
                    }
                }
                _ => MergeResult::Unmergable,
            },
            CounterAction::Multiply(_) => MergeResult::Unmergable,
            CounterAction::SetValue(_) => MergeResult::Overwrites,
        }
    }
}
