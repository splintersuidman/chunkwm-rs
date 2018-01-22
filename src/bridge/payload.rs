//! The `payload` module contains the payload type and its implementations.

use raw::*;
use std::ffi;

/// The `Payload` struct.
#[derive(Debug)]
pub struct Payload(PayloadRef);

impl Payload {
    pub fn get_payload_ref(&self) -> Result<PayloadRef, &'static str> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err("null pointer")
        }
    }

    pub fn get_sock_fd(&self) -> Result<i32, &'static str> {
        unsafe { Ok((*self.get_payload_ref()?).sock_fd) }
    }

    /// Get the command like in `chunkc plugin::command message`.
    pub fn get_command(&self) -> Result<String, &'static str> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.get_payload_ref()?).command)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Get the message like in `chunkc plugin::command message`.
    pub fn get_message(&self) -> Result<String, &'static str> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.get_payload_ref()?).message)
                .to_string_lossy()
                .into_owned())
        }
    }
}

impl Into<Payload> for RawPayload {
    fn into(mut self) -> Payload {
        Payload(&mut self)
    }
}
