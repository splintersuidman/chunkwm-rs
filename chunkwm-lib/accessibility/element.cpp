#include "../../chunkwm/src/common/accessibility/element.cpp"

// extern "C" AXError _AXUIElementGetWindow(AXUIElementRef, uint32_t *WID);
extern "C" uint32_t axlib_get_window_id(AXUIElementRef WindowRef) {
    return AXLibGetWindowID(WindowRef);
}

extern "C" bool axlib_is_window_minimized(AXUIElementRef WindowRef) {
    return AXLibIsWindowMinimized(WindowRef);
}
extern "C" bool axlib_is_window_resizable(AXUIElementRef WindowRef) {
    return AXLibIsWindowResizable(WindowRef);
}
extern "C" bool axlib_is_window_movable(AXUIElementRef WindowRef) {
    return AXLibIsWindowMovable(WindowRef);
}
extern "C" bool axlib_is_window_fullscreen(AXUIElementRef WindowRef) {
    return AXLibIsWindowFullscreen(WindowRef);
}

extern "C" bool axlib_set_window_position(AXUIElementRef WindowRef, float X, float Y) {
    return AXLibSetWindowPosition(WindowRef, X, Y);
}
extern "C" bool axlib_set_window_size(AXUIElementRef WindowRef, float Width, float Height) {
    return AXLibSetWindowSize(WindowRef, Width, Height);
}
extern "C" bool axlib_set_window_fullscreen(AXUIElementRef WindowRef, bool Fullscreen) {
    return AXLibSetWindowFullscreen(WindowRef, Fullscreen);
}
extern "C" void axlib_close_window(AXUIElementRef WindowRef) {
    AXLibCloseWindow(WindowRef);
}

extern "C" CFTypeRef axlib_get_window_property(AXUIElementRef WindowRef, CFStringRef Property) {
    return AXLibGetWindowProperty(WindowRef, Property);
}
extern "C" AXError axlib_set_window_property(AXUIElementRef WindowRef, CFStringRef Property, CFTypeRef Value) {
    return AXLibSetWindowProperty(WindowRef, Property, Value);
}

extern "C" AXUIElementRef axlib_get_focused_window(AXUIElementRef ApplicationRef) {
    return AXLibGetFocusedWindow(ApplicationRef);
}
extern "C" void axlib_set_focused_window(AXUIElementRef WindowRef) {
    return AXLibSetFocusedWindow(WindowRef);
}

extern "C" AXUIElementRef axlib_get_focused_application() {
    return AXLibGetFocusedApplication();
}
extern "C" void axlib_set_focused_application_psn(ProcessSerialNumber PSN) {
    AXLibSetFocusedApplication(PSN);
}
extern "C" void axlib_set_focused_application_pid(pid_t PID) {
    AXLibSetFocusedApplication(PID);
}

extern "C" char *axlib_get_window_title(AXUIElementRef WindowRef) {
    return AXLibGetWindowTitle(WindowRef);
}
extern "C" CGPoint axlib_get_window_position(AXUIElementRef WindowRef) {
    return AXLibGetWindowPosition(WindowRef);
}
extern "C" CGSize axlib_get_window_size(AXUIElementRef WindowRef) {
    return AXLibGetWindowSize(WindowRef);
}

extern "C" bool axlib_get_window_role(AXUIElementRef WindowRef, CFStringRef *Role) {
    return AXLibGetWindowRole(WindowRef, Role);
}
extern "C" bool axlib_get_window_subrole(AXUIElementRef WindowRef, CFStringRef *Subrole) {
    return AXLibGetWindowSubrole(WindowRef, Subrole);
}

extern "C" CGPoint axlib_get_cursor_pos() {
    return AXLibGetCursorPos();
}
// char *CopyCFStringToC(CFStringRef String);
