use core_graphics::geometry::{CGPoint, CGSize};
use core_graphics::display::CGDirectDisplayID;
use core_foundation::base::CFTypeRef;
use core_foundation::string::CFStringRef;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_ulong, c_void};
use display::SpaceType;
use bridge::event::Subscription;
use bridge::api::API;

/// The function type used for the plugin's init function.
pub type PluginBoolFunc = extern "C" fn(api: API) -> bool;
/// The function type used for the plugin's deinit function.
pub type PluginVoidFunc = extern "C" fn();
/// The function type used for the plugin's main function.
pub type PluginMainFunc = extern "C" fn(node: *const c_char, data: *mut c_void) -> bool;

/// The raw type `ChunkWM` uses for a plugin.
#[repr(C)]
pub struct ChunkWMPlugin {
    pub init: PluginBoolFunc,
    pub deinit: PluginVoidFunc,
    pub run: PluginMainFunc,
    pub subscriptions: *const Subscription,
    pub subscription_count: c_uint,
}

/// The reference to UI elements.
pub type AXUIElementRef = CFTypeRef;
/// An error type. This is not often used in Rust code.
// NOTE(splintah): sizeof(AXError) == 4, docs list signed values.
pub type AXError = i32;
/// The process identifier type.
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

/// A reference to an `RawApplication`.
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

/// A reference to a `RawWindow`.
pub type WindowRef = *mut RawWindow;

/// The raw payload representation. This is used in the C/C++ plugin, and shouldn't really be
/// necessary for you to use in your Rust code.
#[repr(C)]
pub struct RawPayload {
    pub sock_fd: c_int,
    pub command: *const c_char,
    pub message: *const c_char,
}

/// A reference to a `RawPayload`.
pub type PayloadRef = *mut RawPayload;

/// The raw `Display`.
#[repr(C)]
pub struct RawDisplay {
    pub display_ref: CFStringRef,
    pub id: CGDirectDisplayID,
    pub arrangement: c_uint,
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}

/// A reference to a `RawDisplay`.
pub type DisplayRef = *mut RawDisplay;

pub type CGSSpaceID = c_int;
pub type CGSSpaceType = SpaceType;

/// The raw `Space`.
#[repr(C)]
pub struct RawSpace {
    pub space_ref: CFStringRef,
    pub id: CGSSpaceID,
    pub space_type: CGSSpaceType,
}

/// A reference to a `RawSpace`.
pub type SpaceRef = *mut RawSpace;
