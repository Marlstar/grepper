#![allow(clippy::needless_return)]

pub mod grep;
pub mod files;
mod args; pub use args::{ARGS, CWD};
mod error; pub use error::Error;
