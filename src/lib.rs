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

mod merge;
pub use self::merge::{Merge, MergeResult};

mod chain;
pub use self::chain::Chain;

mod error;
pub use self::error::Error;

mod state;
pub use self::state::{InverseResult, State};

mod timeline;
pub use self::timeline::Timeline;
