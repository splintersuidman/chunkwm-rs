use core_foundation::base::CFTypeRef;
use core_foundation::string::CFStringRef;
use core_graphics::geometry::{CGPoint, CGSize};
use raw::*;
use std::os::raw::{c_char, c_float};

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_axlib_get_window_id"]
    pub fn get_window_id(window_ref: AXUIElementRef) -> u32;

    #[link_name = "\u{1}_axlib_is_window_minimized"]
    pub fn is_window_minimized(window_ref: AXUIElementRef) -> bool;

    #[link_name = "\u{1}_axlib_is_window_resizable"]
    pub fn is_window_resizable(window_ref: AXUIElementRef) -> bool;

    #[link_name = "\u{1}_axlib_is_window_movable"]
    pub fn is_window_movable(window_ref: AXUIElementRef) -> bool;

    #[link_name = "\u{1}_axlib_is_window_fullscreen"]
    pub fn is_window_fullscreen(window_ref: AXUIElementRef) -> bool;

    #[link_name = "\u{1}_axlib_set_window_position"]
    pub fn set_window_position(window_ref: AXUIElementRef, x: c_float, y: c_float) -> bool;

    #[link_name = "\u{1}_axlib_set_window_size"]
    pub fn set_window_size(window_ref: AXUIElementRef, width: c_float, height: c_float) -> bool;

    #[link_name = "\u{1}_axlib_set_window_fullscreen"]
    pub fn set_window_fullscreen(window_ref: AXUIElementRef, fullscreen: bool) -> bool;

    #[link_name = "\u{1}_axlib_close_window"]
    pub fn close_window(window_ref: AXUIElementRef);

    #[link_name = "\u{1}_axlib_get_window_property"]
    pub fn get_window_property(window_ref: AXUIElementRef, property: CFStringRef) -> CFTypeRef;

    #[link_name = "\u{1}_axlib_set_window_property"]
    pub fn set_window_property(
        window_ref: AXUIElementRef,
        property: CFStringRef,
        value: CFTypeRef,
    ) -> AXError;

    #[link_name = "\u{1}_axlib_get_focused_window"]
    pub fn get_focused_window(application_ref: AXUIElementRef) -> AXUIElementRef;

    #[link_name = "\u{1}_axlib_set_focused_window"]
    pub fn set_focused_window(window_ref: AXUIElementRef);

    #[link_name = "\u{1}_axlib_get_focused_application"]
    pub fn get_focused_application() -> AXUIElementRef;

    #[link_name = "\u{1}_axlib_set_focused_application_psn"]
    pub fn set_focused_application_psn(psn: ProcessSerialNumber);

    #[link_name = "\u{1}_axlib_set_focused_application_pid"]
    pub fn set_focused_application_pid(psn: PID);

    #[link_name = "\u{1}_axlib_get_window_title"]
    pub fn get_window_title(window_ref: AXUIElementRef) -> *const c_char;

    #[link_name = "\u{1}_axlib_get_window_position"]
    pub fn get_window_position(window_ref: AXUIElementRef) -> CGPoint;

    #[link_name = "\u{1}_axlib_get_window_size"]
    pub fn get_window_size(window_ref: AXUIElementRef) -> CGSize;

    #[link_name = "\u{1}_axlib_get_window_role"]
    pub fn get_window_role(window_ref: AXUIElementRef, role: *const CFStringRef) -> bool;

    #[link_name = "\u{1}_axlib_get_window_subrole"]
    pub fn get_window_subrole(window_ref: AXUIElementRef, subrole: *const CFStringRef) -> bool;

    #[link_name = "\u{1}_axlib_get_cursor_pos"]
    pub fn get_cursor_position() -> CGPoint;
}
