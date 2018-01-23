//! The `payload` module contains the payload type and its implementations.

use ChunkWMError;
use raw::*;
use std::ffi;

/// The `Payload` struct.
#[derive(Debug)]
pub struct Payload(PayloadRef);

impl Payload {
    pub fn payload_ref(&self) -> Result<PayloadRef, ChunkWMError> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    pub fn sock_fd(&self) -> Result<i32, ChunkWMError> {
        unsafe { Ok((*self.payload_ref()?).sock_fd) }
    }

    /// Get the command like in `chunkc plugin::command message`.
    pub fn command(&self) -> Result<String, ChunkWMError> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.payload_ref()?).command)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Get the message like in `chunkc plugin::command message`.
    pub fn message(&self) -> Result<String, ChunkWMError> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.payload_ref()?).message)
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

impl From<PayloadRef> for Payload {
    fn from(payload_ref: PayloadRef) -> Payload {
        Payload(payload_ref)
    }
}
