extern crate actions;

mod helpers;
use helpers::counter::*;

use actions::Timeline;

enum Perform {
    Action(CounterAction),
    Undo,
    Redo,
}

/// Perform a change (Action, Undo or Redo) and check (after the change was Performed)
/// if the value of the counter is equal to the expected value.
fn change_and_expect(t: &mut Timeline<Counter>, perform: Perform, expected_value: i32) {
    match perform {
        Perform::Action(action) => t.apply(&action).unwrap(),
        Perform::Undo => t.undo().unwrap(),
        Perform::Redo => t.redo().unwrap(),
    };
    assert_eq!(t.current().value, expected_value);
}

#[test]
fn timeline_forward() {
    let mut t = Timeline::new(Counter::default());
    let test_values = vec![
        (CounterAction::Increment, 1),
        (CounterAction::Increment, 2),
        (CounterAction::Increment, 3),
        (CounterAction::Increment, 4),
        (CounterAction::Decrement, 3),
        (CounterAction::Decrement, 2),
        (CounterAction::Increment, 3),
        (CounterAction::SetValue(5), 5),
        (CounterAction::SetValue(7), 7),
        (CounterAction::Decrement, 6),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, Perform::Action(action), expected_value);
    }
}

#[test]
fn timeline_undo() {
    let mut t = Timeline::new(Counter::default());
    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
        (Perform::Undo, 2),
        (Perform::Undo, 1),
        (Perform::Action(CounterAction::SetValue(5)), 5),
        (Perform::Undo, 1),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }
}

#[test]
#[should_panic]
fn timeline_undo_2() {
    let mut t = Timeline::new(Counter::default());
    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
        (Perform::Undo, 2),
        (Perform::Undo, 1),
        (Perform::Undo, 0),
        // Should fail because there is no action to be undone
        (Perform::Undo, -1),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }
}

#[test]
fn timeline_redo() {
    let mut t = Timeline::new(Counter::default());
    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
        (Perform::Undo, 2),
        (Perform::Undo, 1),
        (Perform::Redo, 2),
        (Perform::Redo, 3),
        (Perform::Action(CounterAction::Increment), 4),
        (Perform::Undo, 3), // If this fails, there is probably an error in the *undo*.
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }
}

#[test]
fn timeline_undos_remaining() {
    let mut t = Timeline::new(Counter::default());
    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }

    assert_eq!(t.undos_remaining(), 3);
}

#[test]
fn timeline_redos_remaining() {
    let mut t = Timeline::new(Counter::default());

    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
        (Perform::Action(CounterAction::SetValue(-5)), -5),
        (Perform::Undo, 3),
        (Perform::Undo, 2),
        (Perform::Undo, 1),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }

    // After undoing 3 times, there should be 3 redo's available.
    assert_eq!(t.redos_remaining(), 3);
}

#[test]
fn timeline_redos_remaining_2() {
    let mut t = Timeline::new(Counter::default());

    let test_values = vec![
        (Perform::Action(CounterAction::Increment), 1),
        (Perform::Action(CounterAction::Increment), 2),
        (Perform::Action(CounterAction::Increment), 3),
        (Perform::Undo, 2),
        (Perform::Undo, 1),
        // Performing an action while not at the latest action should
        // make the timeline 'discard the future'...
        (Perform::Action(CounterAction::SetValue(5)), 5),
    ];

    for (action, expected_value) in test_values {
        change_and_expect(&mut t, action, expected_value);
    }
    // ... So there should not be any redo's remaining.
    assert_eq!(t.redos_remaining(), 0);
}
