//! This module contains the application type and its implementations.

use raw::*;
use std::ffi;
use std::ptr;
#[cfg(feature = "accessibility")]
use common::accessibility::application;

/// The `Application` struct.
#[derive(Debug)]
pub struct Application(ApplicationRef);

impl Application {
    /// Get the focused application.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn get_focused() -> Result<Application, &'static str> {
        let application = unsafe { application::get_focused_application() };
        Ok(application.into())
    }

    /// Get all running processes.
    /// Needed features: `accessibility`.
    // TODO(splintah): ProcessFlags type?
    #[cfg(feature = "accessibility")]
    pub fn get_processes() -> Result<Vec<Application>, &'static str> {
        let applications: &[ApplicationRef] = unsafe { application::get_running_processes(0) };
        let applications: Vec<ApplicationRef> = applications.to_vec();
        Ok(applications
            .iter()
            .map(|app_ref| Application::from(*app_ref))
            .collect())
    }

    /// Get the raw window pointer.
    pub unsafe fn get_application_ref(&self) -> Result<ApplicationRef, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(self.0)
        } else {
            Err("null pointer")
        }
    }

    /// Destroy the application.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            unsafe { application::destroy_application(self.0) };
            Ok(())
        } else {
            Err("null pointer")
        }
    }

    /// Get the application's element.
    pub fn get_element(&self) -> Result<AXUIElementRef, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).element })
        } else {
            Err("null pointer")
        }
    }

    /// Get the application's observer.
    pub fn get_observer(&self) -> Result<RawObserver, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).observer })
        } else {
            Err("null pointer")
        }
    }

    /// Get the application's name.
    pub fn get_name(&self) -> Result<String, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe {
                ffi::CStr::from_ptr((*self.0).name)
                    .to_string_lossy()
                    .into_owned()
            })
        } else {
            Err("null pointer")
        }
    }

    /// Get the application's pid.
    pub fn get_pid(&self) -> Result<i32, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).pid })
        } else {
            Err("null pointer")
        }
    }

    /// Get the application's process serial number.
    pub fn get_process_serial_number(&self) -> Result<ProcessSerialNumber, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).process_serial_number })
        } else {
            Err("null pointer")
        }
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
