mod helpers;
use crate::helpers::counter::*;

use actions::State;

#[test]
fn apply() {
    let mut c = Counter::default();
    assert_eq!(c.0, 0);

    c.apply(&CounterAction::Increment).unwrap();
    assert_eq!(c.0, 1);

    c.apply(&CounterAction::Decrement).unwrap();
    assert_eq!(c.0, 0);

    c.apply(&CounterAction::SetValue(10)).unwrap();
    assert_eq!(c.0, 10);
}
