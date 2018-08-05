extern crate actions;

mod helpers;
use helpers::counter::*;

use actions::Reduce;

#[test]
fn reduce() {
    let mut reverse_stack = vec![];

    let mut c = Counter::default();
    assert_eq!(c.value, 0);

    let execute =
        |stack: &mut Vec<CounterAction>, counter: &mut Counter, action: &CounterAction| {
            {
                stack.push(counter.apply_action(action).unwrap().unwrap())
            }
        };

    execute(&mut reverse_stack, &mut c, &CounterAction::Increment);
    assert_eq!(c.value, 1);

    execute(&mut reverse_stack, &mut c, &CounterAction::Decrement);
    assert_eq!(c.value, 0);

    c.apply_action(&reverse_stack.pop().unwrap()).unwrap();
    assert_eq!(c.value, 1);

    c.apply_action(&reverse_stack.pop().unwrap()).unwrap();
    assert_eq!(c.value, 0);

    execute(&mut reverse_stack, &mut c, &CounterAction::SetValue(10));
    assert_eq!(c.value, 10);

    execute(&mut reverse_stack, &mut c, &CounterAction::SetValue(5));
    assert_eq!(c.value, 5);

    c.apply_action(&reverse_stack.pop().unwrap()).unwrap();
    assert_eq!(c.value, 10);

    c.apply_action(&reverse_stack.pop().unwrap()).unwrap();
    assert_eq!(c.value, 0);
}
