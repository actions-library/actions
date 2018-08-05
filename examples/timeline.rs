extern crate actions;

use actions::Reduce;
use actions::{Error, Timeline};

#[derive(Default, Clone)]
struct Counter {
    value: i32,
}

#[derive(Clone)]
enum CounterAction {
    Increment,
    Decrement,
    SetValue(i32),
}

impl Reduce for Counter {
    type Action = CounterAction;

    fn apply_action(&mut self, action: &Self::Action) -> Result<Option<Self::Action>, Error> {
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
        };
        Ok(inverse)
    }
}

fn manipulate_counter() -> Result<(), Error> {
    let mut ctr = Timeline::new(Counter::default());
    assert_eq!(ctr.current().value, 0);

    ctr.apply(&CounterAction::SetValue(5))?;
    ctr.apply(&CounterAction::Increment)?;
    ctr.undo()?;

    assert_eq!(ctr.current().value, 5);

    Ok(())
}

fn main() {
    if let Err(err) = manipulate_counter() {
        eprintln!("An error occured:\n{}", err);
    }
}
