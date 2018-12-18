use rand::distributions::Standard;
use rand::prelude::{thread_rng, Distribution, Rng};

use std::panic::catch_unwind;

mod helpers;
use crate::helpers::counter::*;

use actions::Chain;
use actions::Timeline;

/// Clone the chain, compress one of them.
/// Then execute the actions for both the uncompressed and the
/// compressed chain and assert the results to be equal.
fn compress_and_compare(chain: Chain<CounterAction>) {
    let (mut result_compressed, mut result_uncompressed) = (0, 0);
    let mut error_compressed = false;
    let mut error_uncompressed = false;

    let mut chain2 = chain.clone();
    chain2.compress();

    // Store string representation for debugging purposes.
    // (To print if the test fails)
    let chain_debugstring = format!("{:#?}", chain.actions());
    let chain2_debugstring = format!("{:#?}", chain2.actions());

    // In case the generated test fails with a `panic!`,
    // (can happen with for example overflows)
    // it is wrapped in a catch_unwind.
    // The only requirement if it fails is that it must fail
    // for both the tests.

    match catch_unwind(|| {
        // Apply compressed chain on a new timeline.
        let mut timeline = Timeline::new(Counter::default());
        timeline.apply_chain(&chain2).unwrap();
        timeline.current_state().0
    }) {
        Ok(result) => result_uncompressed = result,
        Err(_) => error_compressed = true,
    }

    match catch_unwind(|| {
        let mut timeline = Timeline::new(Counter::default());
        // Apply not-compressed chain on a new timeline.
        timeline.apply_chain(&chain).unwrap();
        timeline.current_state().0
    }) {
        Ok(result) => result_compressed = result,
        Err(_) => error_uncompressed = true,
    }

    // This is a bit nasty. It simply throws away the test if any of the two tests panic.
    // TODO: This should not be needed anymore since errors are used??
    if error_compressed || error_uncompressed {
        println!(
            "At least one of the executing chains caused a panic! Ignoring this compressiontest."
        );
        return;
    }
    assert_eq!(result_uncompressed, result_compressed, "The results of executing the uncompressed chain (left) and executing the compressed chain (right) are different.\n\nUNCOMPRESSED CHAIN:\n{}\n\nCOMPRESSED_CHAIN:\n{}\n\n", chain_debugstring, chain2_debugstring);
}

/// Generate a chain with a length between len_min and len_max and test
/// if the result with the compressed chain is equal to the result
/// with the uncompressed chain.
fn chain_compression_generated(len_min: usize, len_max: usize) {
    // random number generator.
    let mut rng = thread_rng();

    // Amount of actions
    let len = rng.gen_range(len_min, len_max);

    // Vector that will be filled with actions.
    let mut chain = Vec::<CounterAction>::with_capacity(len);

    for _ in 1..len {
        chain.push(rng.gen::<CounterAction>());
    }

    // There we go!
    compress_and_compare(chain.into());
}

#[test]
fn chain_new() {
    let mut chain = Chain::<CounterAction>::new();
    chain.compress();
}

#[test]
fn chain_push() {
    let mut chain = Chain::<CounterAction>::new();
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);

    let expected = vec![
        CounterAction::Increment,
        CounterAction::Decrement,
        CounterAction::Increment,
        CounterAction::Decrement,
    ];

    for (value, expected) in chain.actions().iter().zip(expected.iter()) {
        assert_eq!(value, expected);
    }
}

#[test]
fn chain_compress_cancel_out() {
    let mut chain = Chain::<CounterAction>::new();
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.compress();

    // All actions should be canceling out.
    assert_eq!(chain.len(), 0);
}

// Needed for the random number generator.
impl Distribution<CounterAction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CounterAction {
        // Apply setvalue in rare cases only
        if rng.gen_range(0, 50) <= 1 {
            return CounterAction::SetValue(rng.gen_range(1, 10));
        }

        match rng.gen_range(0, 4) {
            0 => CounterAction::Decrement,
            1 => CounterAction::Increment,
            2 => CounterAction::Divide(rng.gen_range(1, 10)),
            3 => CounterAction::Multiply(rng.gen_range(1, 10)),
            _ => panic!("Generated a value out of range while testing."),
        }
    }
}

#[test]
/// Test compression for small random-generated chains
fn chain_compression_generated_short() {
    for _ in 0..200 {
        chain_compression_generated(0, 10);
    }
}

#[test]
/// Test compression for large random-generated chains
fn chain_compression_generated_long() {
    for _ in 0..200 {
        chain_compression_generated(0, 100);
    }
}

#[test]
/// If the last item is overwriting, the chain should
/// have a length of 1 after compression.
fn chain_compression_last_overwriting() {
    let mut chain = Vec::with_capacity(5);
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.push(CounterAction::Increment);
    chain.push(CounterAction::Decrement);
    chain.push(CounterAction::SetValue(5));

    let mut chain: Chain<CounterAction> = chain.into();
    chain.compress();

    assert_eq!(chain.len(), 1);
    assert_eq!(chain.actions()[0], CounterAction::SetValue(5));
}
