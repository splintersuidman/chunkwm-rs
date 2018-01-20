//! This module contains the window type and its implementations.

use application::*;
use raw::*;
use std::ffi;
use std::ptr;
use core_graphics::geometry::{CGPoint, CGSize};
use core_foundation::base::{CFTypeRef, TCFType};
use core_foundation::string::CFString;

#[cfg(feature = "accessibility")]
use common::accessibility::element;

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
    /// Get element.
    pub fn get_element(&self) -> Result<AXUIElementRef, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).element })
        } else {
            Err("could not get element")
        }
    }

    /// Get main role.
    pub fn get_main_role(&self) -> Result<String, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { CFString::wrap_under_get_rule((*self.0).main_role).to_string() })
        } else {
            Err("could not get main_role")
        }
    }

    /// Get sub role.
    pub fn get_sub_role(&self) -> Result<String, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { CFString::wrap_under_get_rule((*self.0).sub_role).to_string() })
        } else {
            Err("could not get sub_role")
        }
    }

    /// Get owner.
    pub fn get_owner(&self) -> Result<Application, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { Application::from(((*self.0).owner)) })
        } else {
            Err("could not get owner")
        }
    }

    /// Get name.
    pub fn get_name(&self) -> Result<String, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe {
                ffi::CStr::from_ptr((*self.0).name)
                    .to_string_lossy()
                    .into_owned()
            })
        } else {
            Err("could not get name")
        }
    }

    /// Get id.
    pub fn get_id(&self) -> Result<u32, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).id })
        } else {
            Err("could not get id")
        }
    }

    /// Get flags.
    pub fn get_flags(&self) -> Result<Vec<WindowFlag>, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { WindowFlag::from((*self.0).flags) })
        } else {
            Err("could not get flags")
        }
    }

    /// Get level.
    pub fn get_level(&self) -> Result<u32, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).level })
        } else {
            Err("could not get level")
        }
    }

    /// Get position.
    pub fn get_position(&self) -> Result<CGPoint, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).position })
        } else {
            Err("could not get position")
        }
    }

    /// Get size.
    pub fn get_size(&self) -> Result<CGSize, &'static str> {
        if self.0 != ptr::null_mut() {
            Ok(unsafe { (*self.0).size })
        } else {
            Err("could not get size")
        }
    }

    /// Check whether window is minimized.
    #[cfg(feature = "accessibility")]
    pub fn is_minimized(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::is_window_minimized(self.get_element()?) } {
                Ok(())
            } else {
                Err("could not run is_minimized successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Check whether window is resizable.
    #[cfg(feature = "accessibility")]
    pub fn is_resizable(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::is_window_resizable(self.get_element()?) } {
                Ok(())
            } else {
                Err("could not run is_resizable successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Check whether window is movable.
    #[cfg(feature = "accessibility")]
    pub fn is_movable(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::is_window_movable(self.get_element()?) } {
                Ok(())
            } else {
                Err("could not run is_movable successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Check whether window is fullscreen.
    #[cfg(feature = "accessibility")]
    pub fn is_fullscreen(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::is_window_fullscreen(self.get_element()?) } {
                Ok(())
            } else {
                Err("could not run is_fullscreen successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Set the position of the window.
    #[cfg(feature = "accessibility")]
    pub fn set_position(&self, x: f32, y: f32) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::set_window_position(self.get_element()?, x, y) } {
                Ok(())
            } else {
                Err("could not run set_position successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Set the size of the window.
    #[cfg(feature = "accessibility")]
    pub fn set_size(&self, width: f32, height: f32) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            if unsafe { element::set_window_size(self.get_element()?, width, height) } {
                Ok(())
            } else {
                Err("could not run set_size successfully")
            }
        } else {
            Err("null pointer")
        }
    }

    /// Close the window.
    #[cfg(feature = "accessibility")]
    pub fn close(&self) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            unsafe { element::close_window(self.get_element()?) };
            Ok(())
        } else {
            Err("null pointer")
        }
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
