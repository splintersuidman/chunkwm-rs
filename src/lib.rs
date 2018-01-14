/**
 * NOTE(splintah):
 * Maybe implement more from `common`?
 * Easy way of passing methods: create a struct containing important methods (like `API`).
 *
 * `accessibility/element`: probably.
 * `border`: maybe.
 * `config/tokenize` (whitespace splitter): not going to implement (easier in Rust).
 * `dispatch` (event taps): not going to implement.
 * `ipc` (socket): not going to implement.
 */
extern crate core_foundation;
extern crate core_graphics;

mod bridge;
mod raw;
#[macro_use]
mod macros;
pub mod prelude;

pub use bridge::*;
pub use raw::*;
