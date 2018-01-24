use raw::*;
use core_graphics::display::{CGDirectDisplayID, CGRect, CGSize, CGPoint};
use core_foundation::string::CFStringRef;
use std::os::raw::{c_int, c_uint};
use display::*;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_axlib_construct_display"]
    pub fn construct_display(id: CGDirectDisplayID, arrangement: c_uint) -> DisplayRef;

    #[link_name = "\u{1}_axlib_destroy_display"]
    pub fn destroy_display(display: DisplayRef);

    #[link_name = "\u{1}_axlib_display_count"]
    pub fn display_count() -> c_uint;

    #[link_name = "\u{1}_axlib_active_cgsspaceid"]
    pub fn active_spaceid(display_ref: CFStringRef) -> CGSSpaceID;

    #[link_name = "\u{1}_axlib_active_space_display"]
    pub fn active_space_with_display(display_ref: CFStringRef) -> SpaceRef;

    #[link_name = "\u{1}_axlib_active_space_space"]
    pub fn is_space_active(space: *mut SpaceRef) -> bool;

    #[link_name = "\u{1}_axlib_destroy_space"]
    pub fn destroy_space(space: SpaceRef);

    #[link_name = "\u{1}_axlib_get_display_identifier"]
    pub fn get_display_identifier(id: CGDirectDisplayID) -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_from_arrangement"]
    pub fn get_display_id_from_arrangement(arrangement: c_uint) -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_from_space"]
    pub fn get_display_id_from_space(space: CGSSpaceID) -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_from_window"]
    pub fn get_display_id_from_window(window_id: u32) -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_from_window_rect"]
    pub fn get_display_id_from_window_rect(position: CGPoint, size: CGSize) -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_for_right_most_display"]
    pub fn get_display_id_for_right_most_display() -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_for_left_most_display"]
    pub fn get_display_id_for_left_most_display() -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_identifier_for_bottom_most_display"]
    pub fn get_display_id_for_bottom_most_display() -> CFStringRef;

    #[link_name = "\u{1}_axlib_get_display_bounds"]
    pub fn get_display_bounds(display_ref: CFStringRef) -> CGRect;

    #[link_name = "\u{1}_axlib_is_display_changing_spaces"]
    pub fn is_display_changing_spaces(display_ref: CFStringRef) -> bool;

    #[link_name = "\u{1}_axlib_cgsspaceid_to_desktop_id"]
    pub fn cgsspaceid_to_desktop_id(space_id: CGSSpaceID, out_arrangement: *mut c_uint, out_desktop_id: *mut c_uint) -> bool;

    #[link_name = "\u{1}_axlib_cgsspaceid_from_desktop_id"]
    pub fn cgsspaceid_from_desktop_id(desktop_id: c_uint, out_arrangement: *mut c_uint, out_desktop_id: *mut c_uint) -> bool;

    #[link_name = "\u{1}_axlib_spaces_for_display_with_count"]
    pub fn spaces_for_display_with_count(display_ref: CFStringRef, count: *mut c_int) -> *mut c_int;

    #[link_name = "\u{1}_axlib_spaces_for_display"]
    pub fn spaces_for_display(display_ref: CFStringRef) -> *mut SpaceRef;

    #[link_name = "\u{1}_axlib_spaces_for_window"]
    pub fn spaces_for_window(window_id: u32) -> *mut SpaceRef;

    #[link_name = "\u{1}_axlib_space_move_window"]
    pub fn space_move_window(space_id: CGSSpaceID, window_id: u32);

    #[link_name = "\u{1}_axlib_space_add_window"]
    pub fn space_add_window(space_id: CGSSpaceID, window_id: u32);

    #[link_name = "\u{1}_axlib_space_remove_window"]
    pub fn space_remove_window(space_id: CGSSpaceID, window_id: u32);

    #[link_name = "\u{1}_axlib_space_has_window"]
    pub fn space_has_window(space_id: CGSSpaceID, window_id: u32) -> bool;

    #[link_name = "\u{1}_axlib_sticky_window"]
    pub fn is_window_sticky(window_id: u32) -> bool;

    #[link_name = "\u{1}_axlib_is_menubar_auto_hide_enabled"]
    pub fn is_menubar_auto_hide_enabled() -> bool;

    #[link_name = "\u{1}_axlib_is_dock_auto_hide_enabled"]
    pub fn is_dock_auto_hide_enabled() -> bool;

    #[link_name = "\u{1}_axlib_get_dock_orientation"]
    pub fn get_dock_orientation() -> DockOrientation;

    #[link_name = "\u{1}_axlib_get_dock_tile_size"]
    pub fn get_dock_tile_size() -> usize;
}
