//! This module contains the window type and its implementations.

use application::*;
use raw::*;
use std::ffi;
use core_graphics::geometry::{CGPoint, CGSize};
use core_foundation::base::{CFTypeRef, TCFType};
use core_foundation::string::CFString;

#[cfg(feature = "accessibility")]
use common::accessibility::element;
#[cfg(feature = "accessibility")]
use common::accessibility::window;

type AXUIElementRef = CFTypeRef;

/// The flags on a `Window`.
#[derive(Debug)]
pub enum WindowFlag {
    InitMinimized,
    Movable,
    Resizable,
    Minimized,
    Float,
    Sticky,
    Invalid,
    ForeTile,
}

impl WindowFlag {
    /// Create a `Vec` of `WindowFlag`s from a `u32`.
    pub fn from(flag: u32) -> Vec<WindowFlag> {
        let mut flags = Vec::new();

        if flag & 1 != 0 {
            flags.push(WindowFlag::InitMinimized);
        }
        if flag & (1 << 1) != 0 {
            flags.push(WindowFlag::Movable);
        }
        if flag & (1 << 2) != 0 {
            flags.push(WindowFlag::Resizable);
        }
        if flag & (1 << 3) != 0 {
            flags.push(WindowFlag::Minimized);
        }
        if flag & (1 << 4) != 0 {
            flags.push(WindowFlag::Float);
        }
        if flag & (1 << 5) != 0 {
            flags.push(WindowFlag::Sticky);
        }
        if flag & (1 << 6) != 0 {
            flags.push(WindowFlag::Invalid);
        }
        if flag & (1 << 7) != 0 {
            flags.push(WindowFlag::ForeTile);
        }

        flags
    }
}

/// The `Window` struct.
#[derive(Debug)]
pub struct Window(WindowRef);

impl Window {
    /// Destroy the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn list_for_application(application: &Application) -> Result<Vec<Window>, &'static str> {
        let window: &[WindowRef] =
            unsafe { window::window_list_for_application(application.get_application_ref()?) };
        let window: Vec<WindowRef> = window.to_vec();
        Ok(window
            .iter()
            .map(|app_ref| Window::from(*app_ref))
            .collect())
    }

    /// Get the raw window pointer.
    pub unsafe fn get_window_ref(&self) -> Result<WindowRef, &'static str> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err("null pointer")
        }
    }

    /// Get element.
    pub fn get_element(&self) -> Result<AXUIElementRef, &'static str> {
        unsafe { Ok((*self.get_window_ref()?).element) }
    }

    /// Get main role.
    pub fn get_main_role(&self) -> Result<String, &'static str> {
        unsafe {
            Ok(CFString::wrap_under_get_rule((*self.get_window_ref()?).main_role).to_string())
        }
    }

    /// Get sub role.
    pub fn get_sub_role(&self) -> Result<String, &'static str> {
        unsafe { Ok(CFString::wrap_under_get_rule((*self.get_window_ref()?).sub_role).to_string()) }
    }

    /// Get owner.
    pub fn get_owner(&self) -> Result<Application, &'static str> {
        unsafe { Ok(Application::from((*self.get_window_ref()?).owner)) }
    }

    /// Get name.
    pub fn get_name(&self) -> Result<String, &'static str> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.get_window_ref()?).name)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Get id.
    pub fn get_id(&self) -> Result<u32, &'static str> {
        unsafe { Ok((*self.get_window_ref()?).id) }
    }

    /// Get flags.
    pub fn get_flags(&self) -> Result<Vec<WindowFlag>, &'static str> {
        unsafe { Ok(WindowFlag::from((*self.get_window_ref()?).flags)) }
    }

    /// Get level.
    pub fn get_level(&self) -> Result<u32, &'static str> {
        unsafe { Ok((*self.get_window_ref()?).level) }
    }

    /// Get position.
    pub fn get_position(&self) -> Result<CGPoint, &'static str> {
        unsafe { Ok((*self.get_window_ref()?).position) }
    }

    /// Get size.
    pub fn get_size(&self) -> Result<CGSize, &'static str> {
        unsafe { Ok((*self.get_window_ref()?).size) }
    }

    /// Check whether window is minimized.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_minimized(&self) -> Result<bool, &'static str> {
        unsafe { Ok(element::is_window_minimized(self.get_element()?)) }
    }

    /// Check whether window is resizable.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_resizable(&self) -> Result<bool, &'static str> {
        unsafe { Ok(element::is_window_resizable(self.get_element()?)) }
    }

    /// Check whether window is movable.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_movable(&self) -> Result<bool, &'static str> {
        unsafe { Ok(element::is_window_movable(self.get_element()?)) }
    }

    /// Check whether window is fullscreen.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_fullscreen(&self) -> Result<bool, &'static str> {
        unsafe { Ok(element::is_window_fullscreen(self.get_element()?)) }
    }

    /// Set the position of the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn set_position(&self, x: f32, y: f32) -> Result<(), &'static str> {
        if unsafe { element::set_window_position(self.get_element()?, x, y) } {
            Ok(())
        } else {
            Err("could not run set_position successfully")
        }
    }

    /// Set the size of the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn set_size(&self, width: f32, height: f32) -> Result<(), &'static str> {
        if unsafe { element::set_window_size(self.get_element()?, width, height) } {
            Ok(())
        } else {
            Err("could not run set_size successfully")
        }
    }

    /// Close the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn close(&self) -> Result<(), &'static str> {
        unsafe { element::close_window(self.get_element()?) };
        Ok(())
    }

    /// Destroy the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), &'static str> {
        unsafe { window::destroy_window(self.get_window_ref()?) };
        Ok(())
    }

    /// Check whether the window is standard.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_standard(&self) -> Result<bool, &'static str> {
        unsafe { Ok(window::is_window_standard(self.get_window_ref()?)) }
    }

    /// Add a flag to the window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn add_flag(&self, flag: WindowFlag) -> Result<(), &'static str> {
        unsafe { window::add_flags(self.get_window_ref()?, flag as u32) };
        Ok(())
    }

    /// Check whether the window has a flag.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn has_flag(&self, flag: WindowFlag) -> Result<bool, &'static str> {
        unsafe { Ok(window::has_flags(self.get_window_ref()?, flag as u32)) }
    }

    /// Remove a flag to from window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn remove_flag(&self, flag: WindowFlag) -> Result<(), &'static str> {
        unsafe { window::clear_flags(self.get_window_ref()?, flag as u32) };
        Ok(())
    }
}

#[cfg(feature = "accessibility")]
impl Clone for Window {
    fn clone(&self) -> Self {
        unsafe { Window::from(window::copy_window(self.0)) }
    }
}

impl Into<Window> for RawWindow {
    fn into(mut self) -> Window {
        Window(&mut self)
    }
}

impl<'a> From<&'a mut RawWindow> for Window {
    fn from(raw_window: &mut RawWindow) -> Window {
        Window(&mut *raw_window)
    }
}

impl From<WindowRef> for Window {
    fn from(window_ref: WindowRef) -> Window {
        Window(window_ref)
    }
}