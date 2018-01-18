//! This module contains the application type and its implementations.

use raw::*;
use std::ffi;

/// The `Application` struct.
#[derive(Debug)]
pub struct Application(ApplicationRef);

impl Application {
    pub fn get_element(&self) -> AXUIElementRef {
        unsafe { (*self.0).element }
    }

    pub fn get_observer(&self) -> RawObserver {
        unsafe { (*self.0).observer }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr((*self.0).name)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_pid(&self) -> i32 {
        unsafe { (*self.0).pid }
    }

    pub fn get_process_serial_number(&self) -> ProcessSerialNumber {
        unsafe { (*self.0).process_serial_number }
    }
}

impl From<ApplicationRef> for Application {
    fn from(application_ref: ApplicationRef) -> Application {
        Application(application_ref)
    }
}

impl<'a> From<&'a mut RawApplication> for Application {
    fn from(raw_app: &mut RawApplication) -> Application {
        Application(raw_app)
    }
}

impl Into<Application> for RawApplication {
    fn into(mut self) -> Application {
        Application(&mut self)
    }
}
