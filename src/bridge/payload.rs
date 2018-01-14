//! This module contains the payload type and its implementations.

use raw::*;
use std::ffi;

/// The `Payload` struct.
#[derive(Debug)]
pub struct Payload {
    ///
    pub sock_fd: i32,
    /// The command like in `chunkc plugin::command message`.
    pub command: String,
    /// The message like in `chunkc plugin::command message`.
    pub message: String,
}

impl Into<Payload> for RawPayload {
    fn into(self) -> Payload {
        let command = unsafe {
            ffi::CStr::from_ptr(self.command)
                .to_string_lossy()
                .into_owned()
        };
        let message = unsafe {
            ffi::CStr::from_ptr(self.message)
                .to_string_lossy()
                .into_owned()
        };

        Payload {
            sock_fd: self.sock_fd,
            command,
            message,
        }
    }
}
