use raw::*;
use std::os::raw::c_char;

#[repr(C)]
struct ApplicationArray {
    len: usize,
    arr: *const ApplicationRef,
}

#[link(name = "Carbon", kind = "framework")]
#[link(name = "Cocoa", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_axlib_construct_focused_application"]
    pub fn get_focused_application() -> ApplicationRef;

    #[link_name = "\u{1}_axlib_construct_application"]
    pub fn construct_application(
        psn: ProcessSerialNumber,
        pid: PID,
        name: *const c_char,
    ) -> ApplicationRef;

    #[link_name = "\u{1}_axlib_destroy_application"]
    pub fn destroy_application(application: ApplicationRef);

    #[link_name = "\u{1}_axlib_running_processes"]
    fn axlib_running_processes(process_flags: u32) -> ApplicationArray;
}

pub unsafe fn get_running_processes<'a>(process_flags: u32) -> &'a [ApplicationRef] {
    let app_array = axlib_running_processes(process_flags);
    ::std::slice::from_raw_parts::<'a, ApplicationRef>(app_array.arr, app_array.len)
}
