#include "../../chunkwm/src/common/accessibility/window.cpp"

extern "C" macos_window *axlib_construct_window(macos_application *Application, AXUIElementRef WindowRef) {
    return AXLibConstructWindow(Application, WindowRef);
}
extern "C" macos_window *axlib_copy_window(macos_window *Window) {
    return AXLibCopyWindow(Window);
}
extern "C" void axlib_destroy_window(macos_window *Window) {
    return AXLibDestroyWindow(Window);
}

extern "C" bool axlib_is_window_standard(macos_window *Window) {
    return AXLibIsWindowStandard(Window);
}
extern "C" bool axlib_window_has_role(macos_window *Window, CFTypeRef Role) {
    return AXLibWindowHasRole(Window, Role);
}

struct macos_window_arr {
    size_t len;
    macos_window **arr;
};

extern "C" macos_window_arr axlib_window_list_for_application(macos_application *Application) {
    macos_window_arr Arr;
    macos_window **WindowList = AXLibWindowListForApplication(Application);
    Arr.len = sizeof(WindowList) / sizeof(macos_window *);
    Arr.arr = WindowList;
    return Arr;
}

extern "C" void axlib_add_flags(macos_window *Window, uint32_t Flag) {
    AXLibAddFlags(Window, Flag);
}

extern "C" void axlib_clear_flags(macos_window *Window, uint32_t Flag) {
    AXLibClearFlags(Window, Flag);
}

extern "C" bool axlib_has_flags(macos_window *Window, uint32_t Flag) {
    return AXLibHasFlags(Window, Flag);
}
