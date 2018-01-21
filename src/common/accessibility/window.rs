use core_foundation::base::CFTypeRef;
use raw::*;

#[repr(C)]
struct WindowArr {
    len: usize,
    arr: *const WindowRef,
}

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_axlib_construct_window"]
    pub fn construct_window(application: ApplicationRef, window_ref: AXUIElementRef) -> WindowRef;

    #[link_name = "\u{1}_axlib_copy_window"]
    pub fn copy_window(window: WindowRef) -> WindowRef;

    #[link_name = "\u{1}_axlib_destroy_window"]
    pub fn destroy_window(window: WindowRef);

    #[link_name = "\u{1}_axlib_is_window_standard"]
    pub fn is_window_standard(window: WindowRef) -> bool;

    #[link_name = "\u{1}_axlib_window_has_role"]
    pub fn window_has_role(window: WindowRef, role: CFTypeRef) -> bool;

    #[link_name = "\u{1}_axlib_window_list_for_application"]
    fn axlib_window_list_for_application(application: ApplicationRef) -> WindowArr;

    #[link_name = "\u{1}_axlib_add_flags"]
    pub fn add_flags(window: WindowRef, flag: u32);

    #[link_name = "\u{1}_axlib_clear_flags"]
    pub fn clear_flags(window: WindowRef, flag: u32);

    #[link_name = "\u{1}_axlib_has_flags"]
    pub fn has_flags(window: WindowRef, flag: u32) -> bool;
}

pub unsafe fn window_list_for_application<'a>(application: ApplicationRef) -> &'a [WindowRef] {
    let window_array = axlib_window_list_for_application(application);
    ::std::slice::from_raw_parts::<'a, WindowRef>(window_array.arr, window_array.len)
}
