use actions::State;
use actions::{Chain, Merge, MergeResult};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Clone, Debug)]
enum CounterAction {
    Increment,
    Decrement,
    SetValue(i32),
    Multiply(i32),
    Divide(i32),
}

#[derive(Debug)]
enum CounterError {
    MinValueReached,
    MaxValueReached,
    WouldOverflow,
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

            CounterAction::Divide(value1) => match previous {
                CounterAction::Divide(value2) => {
                    MergeResult::Merged(CounterAction::Divide(*value1 * *value2))
                }

                CounterAction::Multiply(value2) => {
                    if value2 % value1 == 0 {
                        MergeResult::Merged(CounterAction::Multiply(*value2 / *value1))
                    } else {
                        MergeResult::Unmergable
                    }
                }
                _ => MergeResult::Unmergable,
            },

            CounterAction::Multiply(1) => MergeResult::Merged(previous.clone()),

            CounterAction::Multiply(value1) => match previous {
                CounterAction::Multiply(value2) => {
                    MergeResult::Merged(CounterAction::Multiply(value1 * value2))
                }
                CounterAction::Divide(value2) => {
                    if value1 % value2 == 0 {
                        MergeResult::Merged(CounterAction::Multiply(*value1 / *value2))
                    } else {
                        MergeResult::Unmergable
                    }
                }
                _ => MergeResult::Unmergable,
            },

            CounterAction::SetValue(_) => MergeResult::Overwrites,
        }
    }
}

impl State for Counter {
    type Action = CounterAction;
    type Error = CounterError;

    fn apply(&mut self, action: &CounterAction) -> Result<(), CounterError> {
        match action {
            CounterAction::Increment => {
                if self.value == <i32>::max_value() {
                    return Err(CounterError::MaxValueReached);
                }
                self.value += 1;
            }
            CounterAction::Decrement => {
                if self.value == <i32>::min_value() {
                    return Err(CounterError::MinValueReached);
                }
                self.value -= 1;
            }
            CounterAction::SetValue(v) => {
                self.value = *v;
            }
            CounterAction::Divide(v) => {
                self.value /= v;
            }
            CounterAction::Multiply(v) => {
                // Check for overflow
                match self.value.checked_mul(*v) {
                    Some(new_value) => self.value = new_value,
                    None => {
                        return Err(CounterError::WouldOverflow);
                    }
                };
            }
        }

        Ok(())
    }
}

fn merge_actions() -> Result<(), CounterError> {
    let mut chain = Chain::with_capacity(10);

    chain.push(CounterAction::SetValue(5));
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.push(CounterAction::Multiply(2));
    chain.push(CounterAction::Multiply(2));
    chain.push(CounterAction::Divide(4));

    println!(
        "Length of chain before compressing (merging actions): {}",
        chain.len()
    );

    chain.compress();
    chain.compress();

    println!("Length of chain after compressing: {}", chain.len());
    println!("Chain: {:#?}", chain);

    Ok(())
}

fn main() {
    if let Err(e) = merge_actions() {
        eprintln!("Error: {:?}", e);
    }
}
