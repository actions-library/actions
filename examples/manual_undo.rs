use actions::InverseResult;
use actions::State;

struct Counter(u32);

#[derive(Debug)]
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
        }

        Ok(())
    }

    fn inverse(&self, action: &CounterAction) -> InverseResult<CounterAction> {
        let inverse = match action {
            CounterAction::Increment => CounterAction::Decrement,
            CounterAction::Decrement => CounterAction::Increment,
        };

        InverseResult::Action(inverse)
    }
}

fn test() -> Result<(), CounterError> {
    // Create a new counter with an initial value of 0.
    let mut counter = Counter(0);
    let mut undo = vec![];

    // ACTION 1
    let action = CounterAction::Increment;
    if let InverseResult::Action(inverse) = counter.inverse(&action) {
        undo.push(inverse);
    }
    counter.apply(&action)?;
    assert_eq!(counter.0, 1);
    println!("After action \"{:?}\": {}", action, counter.0);

    // ACTION 2
    let action = CounterAction::Increment;
    if let InverseResult::Action(inverse) = counter.inverse(&action) {
        undo.push(inverse);
    }
    counter.apply(&action)?;
    assert_eq!(counter.0, 2);
    println!("After action \"{:?}\": {}", action, counter.0);

    // ACTION 3
    let action = CounterAction::Increment;
    if let InverseResult::Action(inverse) = counter.inverse(&action) {
        undo.push(inverse);
    }
    counter.apply(&action)?;
    assert_eq!(counter.0, 3);
    println!("After action \"{:?}\": {}", action, counter.0);

    counter.apply(&undo.pop().unwrap())?;
    println!("After undo: {}", counter.0);

    counter.apply(&undo.pop().unwrap())?;
    println!("After undo: {}", counter.0);

    counter.apply(&undo.pop().unwrap())?;
    println!("After undo: {}", counter.0);

    Ok(())
}

pub fn main() {
    if let Err(e) = test() {
        eprintln!("Error: {:?}", e);
    };
}
