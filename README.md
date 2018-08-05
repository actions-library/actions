# Actions
![actions](https://docs.rs/actions/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/actions-library/actions.rs/master/LICENSE)

Software without side-effects. Redo and Undo. Macro's.

No unsafe blocks!

**Beware, it is very beta right now.
See it as a proof of concept in this stage.**

### Wanna help?
There are so many ways you can help:
- Implement this library in practice and report what about the API and the documention is unclear or can be improved
- If you feel this library misses a feature, please open an issue!
- Fix bugs which will probably appear in the library :)
- If you still not sure how to help, open an issue

### Questions?

Open an issue!

## The idea
Some structures describe the state of an application. Those structures should not be freely modifiable. Instead, they should be given `Action`s.

**An action is a piece of data that describes how the state of the program should change.**

An action can be *applied* to a datastructure. **An action can also be merged with another action.**

## Merging actions
Imagine for example this scenario.

You have got a very simple program. A counter.
The user can enter an operation (increment or decrement) an a value. The user could, for example say: *increment* the counter by *5*. Then the user decrements the counter by *4*. Lastly, the user performs an *increment* of *3*.

We can use an enum to represent those actions.

```rust
enum CounterAction {
    IncrementBy(i32),
    DecrementBy(i32)
}
```

But the user is not satisfied. The user wants to automate this time-consuming process. Simple!

We just **record the actions and store them**. If the user wants to perform all 3 actions, the user can just press one button to perform all 3 actions. They will be executed sequentially.

```rust
let actions = vec![
    CounterAction::IncrementBy(5),
    CounterAction::DecrementBy(4),
    CounterAction::IncrementBy(3)
];
```

"**That is dumb, it could just be 1 single action**", you say? Yes it is dumb. That is why `actions` enables you to merge actions by creating a `Chain` of actions. **You provide the merge-logic**. This is how that would look like for our example:


```rust
impl Merge for Action {
    fn merge(&self, previous: &CounterAction) -> MergeResult<Self> {
        match self {
            // If the current action is incrementing by val
            CounterAction::IncrementBy(val) => match previous {
                // If the previous action was decrementing by val2
                CounterAction::DecrementBy(val2) => 
                    // Then return a new 'merged' action which
                    // increments by (val - val2)
                    MergeResult::Merged(
                        CounterAction::IncrementBy(val - val2)
                    ),
                // Etc..
                CounterAction::IncrementBy(val2) =>
                    MergeResult::Merged(
                        CounterAction::IncrementBy(val + val2)
                    ),
            },

            CounterAction::DecrementBy(val) => match previous {
                CounterAction::IncrementBy(val2) =>
                    MergeResult::Merged(
                        CounterAction::IncrementBy(val2 - val)
                    ),
                
                CounterAction::DecrementBy(val2) =>
                    MergeResult::Merged(
                        CounterAction::DecrementBy(val + val2)
                    )
            },
        }
    }
}
```

**Please note that this merge is not the optimal solution.**, because it is possible to end up with either `CounterAction::IncrementBy(0)` or `CounterAction::DecrementBy(0)`, which could be entirely removed.

To solve that issue, you can return other `MergeResult`s. You can return `MergeResult::CancelsOut`, for example. You can read more about the different `MergeResult`s in the documentation.
