#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![deny(
    bare_trait_objects,
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

//! # Actions
//! `actions` is a library that helps you getting control over the state of you program.
//! Using actions allows you to
//! - **Undo** and **redo** actions!
//! - Create *macro's* which merge multiple actions into one chain of actions!

mod error;
pub use error::ActionsError as Error;

/// A component describes (a part of) the state of a program at any given moment in time.
mod component;
pub use component::Component;

mod timeline;
pub use timeline::Timeline;

/// An action is a datastructure that describes how a state should change internally.
mod action;
pub use action::{Merge, MergeResult};

mod chain;
pub use chain::Chain;