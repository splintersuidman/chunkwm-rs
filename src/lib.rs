#![feature(external_doc)]
#![doc(include = "../README.md")]

extern crate core_foundation;
extern crate core_graphics;

/// The error type that is used for functions that return a `Result`.
#[derive(Debug)]
pub enum ChunkWMError {
    /// The raw pointer is outlived.
    NullPointer,
    /// A `CVar` cannot be found.
    CVarNotFound(&'static str),
    /// Something could not be parsed from a string.
    ParseError(&'static str),
    /// An error occured in a chunkwm function.
    Internal(&'static str),
}

use std::fmt;
use std::error::Error;
impl fmt::Display for ChunkWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChunkWMError: {:?}", self)
    }
}

impl Error for ChunkWMError {
    fn description(&self) -> &str {
        match *self {
            ChunkWMError::NullPointer => "The raw pointer is outlived.",
            ChunkWMError::CVarNotFound(_) => "A `CVar` cannot be found.",
            ChunkWMError::ParseError(_) => "Something could not be parsed from a string.",
            ChunkWMError::Internal(_) => "An error occured in a chunkwm function.",
        }
    }
}

pub mod common;

mod bridge;
pub use bridge::*;

mod raw;
pub use raw::*;

#[macro_use]
mod macros;

pub mod prelude;
