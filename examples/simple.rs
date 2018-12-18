use actions::State;

struct Counter(u32);

enum CounterAction {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum CounterError {
    MinValueReached,
    MaxValueReached,
}

impl State for Counter {
    // Define what actions influence this State
    type Action = CounterAction;
    type Error = CounterError;

    // The function that applies the action!
    fn apply(&mut self, action: &CounterAction) -> Result<(), CounterError> {
        match action {
            CounterAction::Increment => match self.0.checked_add(1) {
                Some(new_value) => self.0 = new_value,
                None => Err(CounterError::MaxValueReached)?,
            },
            CounterAction::Decrement => match self.0.checked_sub(1) {
                Some(new_value) => self.0 = new_value,
                None => Err(CounterError::MinValueReached)?,
            },
        };
        Ok(())
    }
}

fn test() -> Result<(), CounterError> {
    // Create a new counter with an initial value of 0.
    let mut counter = Counter(0);

    counter.apply(&CounterAction::Increment)?;
    counter.apply(&CounterAction::Increment)?;
    counter.apply(&CounterAction::Decrement)?;
    assert_eq!(1, counter.0);

    counter.apply(&CounterAction::Decrement)?;

    // This should cause our own error message to be printed,
    // because the counter uses an unsigned integer (cannot be negative).
    counter.apply(&CounterAction::Decrement)?;

    Ok(())
}

pub fn main() {
    if let Err(e) = test() {
        // In a real application you should handle the error gracefully.
        eprintln!("Error: {:?}", e);
    };
}
