extern crate actions;

use actions::Component;
use actions::Error;

struct Counter(i32);

#[derive(Clone)]
enum CounterAction {
    Increment,
    Decrement,
}

impl Component for Counter {
    // Define what actions influence this component
    type Action = CounterAction;

    // The function that applies the action!
    // This is where the magic happens.
    fn apply(&mut self, action: &CounterAction) -> Result<Option<Self::Action>, Error> {
        match action {
            CounterAction::Increment => {
                self.0 += 1;
                Ok(Some(CounterAction::Decrement))
            }
            CounterAction::Decrement => {
                self.0 -= 1;
                Ok(Some(CounterAction::Increment))
            }
        }
    }
}

fn test() -> Result<(), Error> {
    // Create a new counter with an initial value of 0.
    let mut counter = Counter(0);

    counter.apply(&CounterAction::Increment)?;
    assert_eq!(counter.0, 1);

    counter.apply(&CounterAction::Increment)?;
    assert_eq!(counter.0, 2);

    counter.apply(&CounterAction::Decrement)?;
    assert_eq!(counter.0, 1);
    Ok(())
}

pub fn main() {
    test().unwrap();
}
