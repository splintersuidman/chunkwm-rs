//! The `common` module contains some functions chunkwm contains for working with applications,
//! windows, borders, et cetera.
//!
//! **Warning**: these functions are `unsafe`, to use a safe(r) wrapper, use the methods defined on
//! `Application` and `Window`.

#[cfg(feature = "accessibility")]
pub mod accessibility;
#[cfg(feature = "border")]
pub mod border;
