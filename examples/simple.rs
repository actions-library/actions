extern crate actions;

use actions::Error;
use actions::Reduce;

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
    let mut c = Counter::default();
    c.apply_action(&CounterAction::SetValue(5))?;
    c.apply_action(&CounterAction::Increment)?;
    c.apply_action(&CounterAction::Increment)?;
    c.apply_action(&CounterAction::Decrement)?;
    assert_eq!(c.value, 6);

    Ok(())
}

fn main() {
    manipulate_counter().unwrap();
}
