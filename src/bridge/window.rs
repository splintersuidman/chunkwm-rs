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

        if flag & (1 << 0) != 0 {
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
pub struct Window {
    ///
    pub element: AXUIElementRef,
    /// The window's main role.
    pub main_role: String,
    /// The window's main role.
    pub sub_role: String,
    /// The 'owner' application of the window, i.e. the application it belongs to.
    pub owner: Application,
    /// The window's name.
    pub name: String,
    /// The window's id.
    pub id: u32,
    /// The flags on the window.
    pub flags: Vec<WindowFlag>,
    /// The window's level.
    pub level: u32,
    /// The window's position.
    pub position: CGPoint,
    /// The window's size.
    pub size: CGSize,
}

impl Into<Window> for RawWindow {
    fn into(self) -> Window {
        let owner: &RawApplication =
            unsafe { &*self.owner };

        Window {
            element: self.element,
            main_role: unsafe { CFString::wrap_under_get_rule(self.main_role).to_string() },
            sub_role: unsafe { CFString::wrap_under_get_rule(self.sub_role).to_string() },
            owner: owner.into(),
            name: unsafe {
                ffi::CStr::from_ptr(self.name)
                    .to_string_lossy()
                    .into_owned()
            },
            id: self.id,
            flags: WindowFlag::from(self.flags),
            level: self.level,
            position: self.position,
            size: self.size,
        }
    }
}

impl<'a> From<&'a RawWindow> for Window {
    fn from(raw_window: &RawWindow) -> Window {
        let owner: &RawApplication = unsafe { &*raw_window.owner };

        Window {
            element: raw_window.element,
            main_role: unsafe {
                CFString::wrap_under_get_rule(raw_window.main_role).to_string()
            },
            sub_role: unsafe {
                CFString::wrap_under_get_rule(raw_window.sub_role).to_string()
            },
            owner: owner.into(),
            name: unsafe {
                ffi::CStr::from_ptr(raw_window.name)
                    .to_string_lossy()
                    .into_owned()
            },
            id: raw_window.id,
            flags: WindowFlag::from(raw_window.flags),
            level: raw_window.level,
            position: raw_window.position,
            size: raw_window.size,
        }
    }
}
