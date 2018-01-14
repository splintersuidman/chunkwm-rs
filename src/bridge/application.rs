//! This module contains the application type and its implementations.

use raw::*;
use std::ffi;
use core_foundation::base::CFTypeRef;

type AXUIElementRef = CFTypeRef;

/// The `Application` struct.
#[derive(Debug)]
pub struct Application {
    pub element: AXUIElementRef,
    pub observer: RawObserver,
    /// The application's name.
    pub name: String,
    /// The application's process id.
    pub pid: i32,
    pub process_serial_number: ProcessSerialNumber,
}

impl<'a> From<&'a RawApplication> for Application {
    fn from(raw_app: &RawApplication) -> Application {
        Application {
            element: raw_app.element,
            observer: raw_app.observer,
            name: unsafe {
                ffi::CStr::from_ptr(raw_app.name)
                    .to_string_lossy()
                    .into_owned()
            },
            pid: raw_app.pid,
            process_serial_number: raw_app.process_serial_number,
        }
    }
}

impl Into<Application> for RawApplication {
    fn into(self) -> Application {
        let name = unsafe {
            ffi::CStr::from_ptr(self.name)
                .to_string_lossy()
                .into_owned()
        };

        Application {
            element: self.element,
            observer: self.observer,
            name,
            pid: self.pid,
            process_serial_number: self.process_serial_number,
        }
    }
}

impl Into<RawApplication> for Application {
    fn into(self) -> RawApplication {
        let name = ffi::CString::new(self.name).unwrap().into_raw();
        RawApplication {
            element: self.element,
            observer: self.observer,
            name,
            pid: self.pid,
            process_serial_number: self.process_serial_number,
        }
    }
}
