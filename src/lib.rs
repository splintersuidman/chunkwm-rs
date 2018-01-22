#![feature(external_doc)]
#![doc(include = "../README.md")]

extern crate core_foundation;
extern crate core_graphics;

pub mod common;

mod bridge;
pub use bridge::*;

mod raw;
pub use raw::*;

#[macro_use]
mod macros;

pub mod prelude;
