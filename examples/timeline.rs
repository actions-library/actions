use actions::{Error as ActionsError, InverseResult, State, Timeline};

#[derive(Default, Clone, Debug)]
struct Counter {
    value: i32,
}

#[derive(Clone, Debug, Copy)]
enum CounterAction {
    Increment,
    Decrement,
    SetValue(i32),
}

#[derive(Debug)]
enum CounterError {
    MinValueReached,
    MaxValueReached,
}

impl State for Counter {
    type Action = CounterAction;
    type Error = CounterError;

    fn apply(&mut self, action: &Self::Action) -> Result<(), CounterError> {
        match action {
            CounterAction::Increment => match self.value.checked_add(1) {
                Some(new_value) => self.value = new_value,
                None => Err(CounterError::MaxValueReached)?,
            },
            CounterAction::Decrement => match self.value.checked_sub(1) {
                Some(new_value) => self.value = new_value,
                None => Err(CounterError::MinValueReached)?,
            },
            CounterAction::SetValue(val) => self.value = *val,
        };
        Ok(())
    }

    // Implementing the inverse function is optional.
    // The Timeline uses it to undo actions.
    // 
    // If the inverse function is not implemented, the Timeline will create
    // a full copy of the state *on the heap* before every action.
    fn inverse(&self, action: &CounterAction) -> InverseResult<CounterAction> {
        let inverse = match action {
            CounterAction::Increment => CounterAction::Decrement,
            CounterAction::Decrement => CounterAction::Increment,
            CounterAction::SetValue(_) => CounterAction::SetValue(self.value),
        };

        InverseResult::Action(inverse)
    }
}

fn manipulate_counter() -> Result<(), ActionsError<Counter>> {
    let mut ctr = Timeline::new(Counter::default());
    assert_eq!(ctr.current_state().value, 0);

    ctr.apply(CounterAction::SetValue(5))?;
    ctr.apply(CounterAction::Increment)?;
    ctr.apply(CounterAction::Decrement)?;
    ctr.undo()?;
    ctr.undo()?;

    assert_eq!(ctr.current_state().value, 5);

    Ok(())
}

fn main() {
    if let Err(err) = manipulate_counter() {
        eprintln!("An error occured: {:?}", err);
    }
}
