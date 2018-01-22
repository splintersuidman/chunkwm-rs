//! The `application` module contains the application type and its implementations.

use raw::*;
use std::ffi;
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
        let applications: &[ApplicationRef] =
            unsafe { application::get_running_processes(0) };
        let applications: Vec<ApplicationRef> = applications.to_vec();
        Ok(applications
            .iter()
            .map(|app_ref| Application::from(*app_ref))
            .collect())
    }

    /// Get the raw window pointer.
    pub unsafe fn get_application_ref(&self) -> Result<ApplicationRef, &'static str> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err("null pointer")
        }
    }

    /// Destroy the application.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), &'static str> {
        unsafe { application::destroy_application(self.get_application_ref()?) };
        Ok(())
    }

    /// Get the application's element.
    pub fn get_element(&self) -> Result<AXUIElementRef, &'static str> {
        unsafe { Ok((*self.get_application_ref()?).element) }
    }

    /// Get the application's observer.
    pub fn get_observer(&self) -> Result<RawObserver, &'static str> {
        unsafe { Ok((*self.get_application_ref()?).observer) }
    }

    /// Get the application's name.
    pub fn get_name(&self) -> Result<String, &'static str> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.get_application_ref()?).name)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Get the application's pid.
    pub fn get_pid(&self) -> Result<i32, &'static str> {
        unsafe { Ok((*self.get_application_ref()?).pid) }
    }

    /// Get the application's process serial number.
    pub fn get_process_serial_number(&self) -> Result<ProcessSerialNumber, &'static str> {
        unsafe { Ok((*self.get_application_ref()?).process_serial_number) }
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
