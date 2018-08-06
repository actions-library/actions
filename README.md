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

## Getting Started
You can find the tutorials [here!](https://actions-library.github.io/tutorials/).

## Please help!
There are so many ways you can help:
- Implement this library in practice and report what about the API and the documention is unclear or can be improved
- If you feel this library misses a feature, please open an issue!
- Fix bugs which will probably appear in the library :)
- If you still not sure how to help, open an issue

## Questions?
Open an issue at GitHub!
