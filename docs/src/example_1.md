# Counter

This tutorial will introduce the two most important concepts of the `actions` library:

A **State** describes (a part) of the state of an application.<br>
**Actions** describe how the state of an application should change.

#### Some setup
First make sure you are using the `actions` crate. Make sure to have `actions` as a dependency in your `Cargo.toml` file. If you are unfamiliar with Cargo, check out the [Cargo book](https://doc.rust-lang.org/cargo/index.html)).

```toml
[dependencies]
actions = "0.2.0"
```

Then make sure to specify `actions` as a dependency in your project, and import the `State` trait.

```rust
{{#include ../../examples/simple.rs:1}}
```

## Building the counter
One of the easiest examples for explaining `actions` is a counter. The counter has two buttons: an decrement button, and an increment button.

<img src="./counter.svg" style="width:100%" alt="An example of what a counter in an application might look like"></img>

Our task is to write the logic for the counter. Easy!

### Step one: Defining the State
One of the most important aspect of the counter is storing the value displayed in the counter. We will use an integer to store the value.

As the counter holds just one value, it can be described like this:

```rust
{{#include ../../examples/simple.rs:3}}
```

### Step two: Defining the actions
Maybe even more important than the counter are the two buttons.

The decrement button should decrement the counter by 1, and the increment button should increment the counter by 1.

Incrementing and decrementing are **actions**. It is immediately clear how executing those actions influences the **State** and therefore the state of the application.

Let's convert these actions into Rust-code.

```rust
{{#include ../../examples/simple.rs:5:8}}
```

That's it! You've defined your actions.

### Step three: Defining the behaviour of the State
Step three is defining how the state of the State should change when an action is executed.

#### Handling errors
To make sure the counter does not go past the maximum or minimum value, we will return an eror if it would. Errors are easily represented using an enum:

```rust
{{#include ../../examples/simple.rs:11:14}}
```

We will use this error-type later.

#### Implementing the `State` trait
`Counter` is a `State`, as it describes the state of our application. Therefore, we will implement the `State` trait for it.

```rust
{{#include ../../examples/simple.rs:16:35}}
```

Note that the apply-function is returning the inverse of the action that was performed. This can be used to 'undo' the action (will be discussed in another tutorial).

If the action has no effect on the state of the State, the apply function should return `None`.

### Testing the counter
That was all you need for the logic of the counter. Now, let's test it.

```rust
{{#include ../../examples/simple.rs:37:53}}
```

It works! You can find the final code from this tutorial <a href="https://github.com/actions-library/actions/blob/master/examples/simple.rs" target="_blank">here</a>.