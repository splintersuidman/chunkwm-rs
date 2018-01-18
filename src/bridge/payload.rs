//! This module contains the payload type and its implementations.

use raw::*;
use std::ffi;

/// The `Payload` struct.
#[derive(Debug)]
pub struct Payload(PayloadRef);

impl Payload {
    pub fn get_sock_fd(&self) -> i32 {
        unsafe { (*self.0).sock_fd }
    }

    /// Get the command like in `chunkc plugin::command message`.
    pub fn get_command(&self) -> String {
        unsafe { ffi::CStr::from_ptr((*self.0).command).to_string_lossy().into_owned() }
    }

    /// Get the message like in `chunkc plugin::command message`.
    pub fn get_message(&self) -> String {
        unsafe { ffi::CStr::from_ptr((*self.0).message).to_string_lossy().into_owned() }
    }
}

impl Into<Payload> for RawPayload {
    fn into(mut self) -> Payload {
        Payload(&mut self)
    }
}
