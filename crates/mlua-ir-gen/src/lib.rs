//! Declarations from an mlua-ir schema. Everything here reads the IR and nothing else: the same functions would
//! work on a `schema.json` written by any other runtime.

mod common;
mod diff;
mod examples;
pub mod luau;
pub mod ts;

pub use diff::{Change, Level, diff};
pub use examples::{Example, examples};
