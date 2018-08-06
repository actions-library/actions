use actions::Error;
use actions::Component;
use actions::{Merge, MergeResult};

extern crate rand;

#[derive(Default, Clone)]
pub struct Counter {
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CounterAction {
    Increment,
    Decrement,
    SetValue(i32),
    Multiply(i32),
    Divide(i32),
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
            CounterAction::SetValue(_) => MergeResult::Overwrites,
            CounterAction::Multiply(_) => match previous {
                // CounterAction::Multiply(prev_val) => MergeResult::Merged(CounterAction::Multiply(prev_val * val)),
                // CounterAction::Divide(prev_val) => MergeResult::Merged(CounterAction::Divide(prev_val * val)),
                _ => MergeResult::Unmergable
            }
            _ => MergeResult::Unmergable,
        }
    }
}

impl Component for Counter {
    type Action = CounterAction;

    fn apply(&mut self, action: &Self::Action) -> Result<Option<Self::Action>, Error> {
        let inverse = match action {
            CounterAction::Increment => {
                self.value += 1;
                Some(CounterAction::Decrement)
            }
            CounterAction::Decrement => {
                self.value -= 1;
                Some(CounterAction::Increment)
            }
            CounterAction::SetValue(val) => {
                let old_value = self.value;
                self.value = *val;
                Some(CounterAction::SetValue(old_value))
            }
            CounterAction::Multiply(val) => {
                self.value *= val;
                Some(CounterAction::Divide(*val))
            }
            CounterAction::Divide(val) => {
                self.value /= val;
                Some(CounterAction::Multiply(*val))
            }
        };
        Ok(inverse)
    }
}
