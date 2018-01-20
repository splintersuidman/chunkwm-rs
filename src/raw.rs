use core_graphics::geometry::{CGPoint, CGSize};
use std::os::raw::{c_char, c_int, c_ulong};
use core_foundation::base::CFTypeRef;
use core_foundation::string::CFStringRef;

pub type AXUIElementRef = CFTypeRef;
// NOTE(splintah): sizeof(AXError) == 4, docs list signed values.
pub type AXError = i32;
pub type PID = c_int;

/// The raw observer.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RawObserver {
    ///
    pub observer: AXUIElementRef,
    ///
    pub enabled: bool,
    ///
    pub valid: bool,
}

/// The process serial number. Both used as a raw type and as a Rust type.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ProcessSerialNumber {
    ///
    pub high_long: c_ulong,
    ///
    pub low_long: c_ulong,
}

/// The raw application representation. This is used in the C/C++ plugin, and shouldn't really be
/// necessary for you to use in your Rust code.
#[repr(C)]
pub struct RawApplication {
    ///
    pub element: AXUIElementRef,
    ///
    pub observer: RawObserver,
    /// The application's name.
    pub name: *const c_char,
    /// Process id.
    pub pid: PID,
    /// Process serial number.
    pub process_serial_number: ProcessSerialNumber,
}

pub type ApplicationRef = *mut RawApplication;

/// The raw window representation. This is used in the C/C++ plugin, and shouldn't really be
/// necessary for you to use in your Rust code.
#[repr(C)]
pub struct RawWindow {
    ///
    pub element: AXUIElementRef,
    ///
    pub main_role: CFStringRef,
    ///
    pub sub_role: CFStringRef,
    /// The 'owner' application of the window, i.e. the application it belongs to.
    pub owner: ApplicationRef,
    /// The window's id.
    pub id: u32,
    /// The window's name.
    pub name: *const c_char,
    /// The flags on the window.
    // TODO: Flags is `volatile`; maybe this should be a pointer?
    pub flags: u32,
    /// The window's level.
    pub level: u32,
    /// The window's position.
    pub position: CGPoint,
    /// The window's size.
    pub size: CGSize,
}

pub type WindowRef = *mut RawWindow;

/// The raw payload representation. This is used in the C/C++ plugin, and shouldn't really be
/// necessary for you to use in your Rust code.
#[repr(C)]
pub struct RawPayload {
    pub sock_fd: c_int,
    pub command: *const c_char,
    pub message: *const c_char,
}

pub type PayloadRef = *mut RawPayload;
