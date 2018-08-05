# Actions
[![Latest Version](https://img.shields.io/crates/v/actions.svg)](https://crates.io/crates/actions)
[![Actions](https://docs.rs/actions/badge.svg)](https://docs.rs/actions/)
[![License](https://img.shields.io/crates/l/actions.svg)](https://raw.githubusercontent.com/actions-library/actions/master/LICENSE)
[![dependency status](https://deps.rs/repo/github/actions-library/actions/status.svg)](https://deps.rs/repo/github/actions-library/actions)

Beware, this library is pretty beta right now.
See it as a proof of concept at this stage.
<br>

## Key features
Read more about these key features [here!](https://github.com/actions-library/actions/blob/master/goals.md).

#### Software without side-effects
- Reduces the risk of bugs
- Code becomes modular!
    - An element/struct can be taken out without any issues, no worries about what it affects: it only affects things it owns
- Without side-effects it is much easier to reason about the code and what it does.

#### Redo and Undo
Undo and redo functionality is an essential requirement for a lot of software. Users heavily rely on it.

#### Creating a `Chain` of actions
> For example, if you are writing an editor for a game-engine. Whenever the user saves, the current chain could be **compressed an stored to the drive** (where size matters). It could then be used to show the differences between saves to the user (the **minimal single actions required to get to the new state**: "You moved this object", etc.).

## Please help!
There are so many ways you can help:
- Implement this library in practice and report what about the API and the documention is unclear or can be improved
- If you feel this library misses a feature, please open an issue!
- Fix bugs which will probably appear in the library :)
- If you still not sure how to help, open an issue

## Questions?

Open an issue at GitHub!

_______

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
enum MyAction {
    Add(u8),
    Subtract(u8)
}
```

But the user is not satisfied. The user wants to automate this time-consuming process. Simple!

We just **record the actions and store them**. If the user wants to perform all 3 actions, the user can just press one button to perform all 3 actions. They will be executed sequentially.

```rust
let actions = vec![
    MyAction::Add(5),
    MyAction::Subtract(4),
    MyAction::Add(3)
];
```

"**That is dumb, it could just be 1 single action**", you say? Yes it is dumb. That is why `actions` enables you to merge actions by creating a `Chain` of actions. **You provide the merge-logic**. This is how that would look like for our example:


```rust
impl Merge for Action {
    fn merge(&self, previous: &MyAction) -> MergeResult<Self> {
        match self {
            // If the current action is incrementing by val
            MyAction::Add(val) => match previous {
                // If the previous action was decrementing by val2
                MyAction::Subtract(val2) => 
                    // Then return a new 'merged' action which
                    // increments by (val - val2)
                    MergeResult::Merged(
                        MyAction::Add(val - val2)
                    ),
                // Etc..
                MyAction::IncrementBy(val2) =>
                    MergeResult::Merged(
                        MyAction::Add(val + val2)
                    ),
            },

            MyAction::DecrementBy(val) => match previous {
                MyAction::IncrementBy(val2) =>
                    MergeResult::Merged(
                        MyAction::Add(val2 - val)
                    ),
                
                MyAction::DecrementBy(val2) =>
                    MergeResult::Merged(
                        MyAction::Subtract(val + val2)
                    )
            },
        }
    }
}
```

**Please note that this merge is not the optimal solution.**, because it is possible to end up with either `CounterAction::Add(0)` or `CounterAction::Subtract(0)`, which could be entirely removed.

To solve that issue, you can return other `MergeResult`s. You can return `MergeResult::CancelsOut`, for example. You can read more about the different `MergeResult`s in the documentation.
