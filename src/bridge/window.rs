//! This module contains the window type and its implementations.

use application::*;
use raw::*;
use std::ffi;
use core_graphics::geometry::{CGPoint, CGSize};
use core_foundation::base::{CFTypeRef, TCFType};
use core_foundation::string::CFString;

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
    pub fn get_element(&self) -> AXUIElementRef {
        unsafe { (*self.0).element }
    }

    pub fn get_main_role(&self) -> String {
        unsafe { CFString::wrap_under_get_rule((*self.0).main_role).to_string() }
    }

    pub fn get_sub_role(&self) -> String {
        unsafe { CFString::wrap_under_get_rule((*self.0).sub_role).to_string() }
    }

    pub fn get_owner(&self) -> Application {
        unsafe { Application::from(((*self.0).owner)) }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr((*self.0).name)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_id(&self) -> u32 {
        unsafe { (*self.0).id }
    }

    pub fn get_flags(&self) -> Vec<WindowFlag> {
        unsafe { WindowFlag::from((*self.0).flags) }
    }

    pub fn get_level(&self) -> u32 {
        unsafe { (*self.0).level }
    }

    pub fn get_position(&self) -> CGPoint {
        unsafe { (*self.0).position }
    }

    pub fn get_size(&self) -> CGSize {
        unsafe { (*self.0).size }
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
