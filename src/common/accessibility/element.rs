use core_graphics::geometry::{CGPoint, CGSize};
use core_foundation::base::CFTypeRef;
use core_foundation::string::CFStringRef;
use std::os::raw::{c_float, c_char};
use raw::*;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    // uint32_t axlib_get_window_id(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_get_window_id"]
    pub fn get_window_id(window_ref: AXUIElementRef) -> u32;

    // bool axlib_is_window_minimized(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_is_window_minimized"]
    pub fn is_window_minimized(window_ref: AXUIElementRef) -> bool;

    // bool axlib_is_window_resizable(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_is_window_resizable"]
    pub fn is_window_resizable(window_ref: AXUIElementRef) -> bool;

    // bool axlib_is_window_movable(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_is_window_movable"]
    pub fn is_window_movable(window_ref: AXUIElementRef) -> bool;

    // bool axlib_is_window_fullscreen(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_is_window_fullscreen"]
    pub fn is_window_fullscreen(window_ref: AXUIElementRef) -> bool;

    // bool axlib_set_window_position(AXUIElementRef WindowRef, float X, float Y) {
    #[link_name = "\u{1}_axlib_set_window_position"]
    pub fn set_window_position(window_ref: AXUIElementRef, x: c_float, y: c_float) -> bool;

    // bool axlib_set_window_size(AXUIElementRef WindowRef, float Width, float Height) {
    #[link_name = "\u{1}_axlib_set_window_size"]
    pub fn set_window_size(window_ref: AXUIElementRef, width: c_float, height: c_float) -> bool;

    // bool axlib_set_window_fullscreen(AXUIElementRef WindowRef, bool Fullscreen) {
    #[link_name = "\u{1}_axlib_set_window_fullscreen"]
    pub fn set_window_fullscreen(window_ref: AXUIElementRef, fullscreen: bool) -> bool;

    // void axlib_close_window(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_close_window"]
    pub fn close_window(window_ref: AXUIElementRef);

    // CFTypeRef axlib_get_window_property(AXUIElementRef WindowRef, CFStringRef Property) {
    #[link_name = "\u{1}_axlib_get_window_property"]
    pub fn get_window_property(window_ref: AXUIElementRef, property: CFStringRef) -> CFTypeRef;

    // AXError axlib_set_window_property(AXUIElementRef WindowRef, CFStringRef Property, CFTypeRef Value) {
    #[link_name = "\u{1}_axlib_set_window_property"]
    pub fn set_window_property(window_ref: AXUIElementRef, property: CFStringRef, value: CFTypeRef) -> AXError;

    // AXUIElementRef axlib_get_focused_window(AXUIElementRef ApplicationRef) {
    #[link_name = "\u{1}_axlib_get_focused_window"]
    pub fn get_focused_window(application_ref: AXUIElementRef) -> AXUIElementRef;

    // void axlib_set_focused_window(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_set_focused_window"]
    pub fn set_focused_window(window_ref: AXUIElementRef);

    // AXUIElementRef axlib_get_focused_application() {
    #[link_name = "\u{1}_axlib_get_focused_application"]
    pub fn get_focused_application() -> AXUIElementRef;

    // void axlib_set_focused_application_psn(ProcessSerialNumber PSN) {
    #[link_name = "\u{1}_axlib_set_focused_application_psn"]
    pub fn set_focused_application_psn(psn: ProcessSerialNumber);

    // void axlib_set_focused_application_pid(pid_t PID) {
    #[link_name = "\u{1}_axlib_set_focused_application_pid"]
    pub fn set_focused_application_pid(psn: PID);

    // char *axlib_get_window_title(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_get_window_title"]
    pub fn get_window_title(window_ref: AXUIElementRef) -> *const c_char;

    // CGPoint axlib_get_window_position(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_get_window_position"]
    pub fn get_window_position(window_ref: AXUIElementRef) -> CGPoint;

    // CGSize axlib_get_window_size(AXUIElementRef WindowRef) {
    #[link_name = "\u{1}_axlib_get_window_size"]
    pub fn get_window_size(window_ref: AXUIElementRef) -> CGSize;

    // bool axlib_get_window_role(AXUIElementRef WindowRef, CFStringRef *Role) {
    #[link_name = "\u{1}_axlib_get_window_role"]
    pub fn get_window_role(window_ref: AXUIElementRef, role: *const CFStringRef) -> bool;

    // bool axlib_get_window_subrole(AXUIElementRef WindowRef, CFStringRef *Subrole) {
    #[link_name = "\u{1}_axlib_get_window_subrole"]
    pub fn get_window_subrole(window_ref: AXUIElementRef, subrole: *const CFStringRef) -> bool;

    // CGPoint axlib_get_cursor_pos() {
    #[link_name = "\u{1}_axlib_get_cursor_pos"]
    pub fn get_cursor_position() -> CGPoint;
}
