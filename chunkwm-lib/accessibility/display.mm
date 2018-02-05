#include "../../chunkwm/src/common/accessibility/display.mm"

extern "C" macos_display *axlib_construct_display(CGDirectDisplayID Id, unsigned Arrangement) {
	return AXLibConstructDisplay(Id, Arrangement);
}
extern "C" void axlib_destroy_display(macos_display *Display) {
	AXLibDestroyDisplay(Display);
}
extern "C" macos_display **axlib_display_list(unsigned *Count) {
	return AXLibDisplayList(Count);
}
extern "C" unsigned axlib_display_count() {
	return AXLibDisplayCount();
}

extern "C" CGSSpaceID axlib_active_cgsspaceid(CFStringRef DisplayRef) {
	return AXLibActiveCGSSpaceID(DisplayRef);
}
extern "C" macos_space *axlib_active_space_display(CFStringRef DisplayRef) {
	return AXLibActiveSpace(DisplayRef);
}
extern "C" bool axlib_active_space_space(macos_space **Space) {
	return AXLibActiveSpace(Space);
}
extern "C" void axlib_destroy_space(macos_space *Space) {
	AXLibDestroySpace(Space);
}

extern "C" CFStringRef axlib_get_display_identifier(CGDirectDisplayID Id) {
	return AXLibGetDisplayIdentifier(Id);
}
extern "C" CFStringRef axlib_get_display_identifier_from_arrangement(unsigned Arrangement) {
	return AXLibGetDisplayIdentifierFromArrangement(Arrangement);
}
extern "C" CFStringRef axlib_get_display_identifier_from_space(CGSSpaceID Space) {
	return AXLibGetDisplayIdentifierFromSpace(Space);
}
extern "C" CFStringRef axlib_get_display_identifier_from_window(uint32_t WindowId) {
	return AXLibGetDisplayIdentifierFromWindow(WindowId);
}
extern "C" CFStringRef axlib_get_display_identifier_from_window_rect(CGPoint Position, CGSize Size) {
	return AXLibGetDisplayIdentifierFromWindowRect(Position, Size);
}
extern "C" CFStringRef axlib_get_display_identifier_for_main_display() {
	return AXLibGetDisplayIdentifierForMainDisplay();
}
extern "C" CFStringRef axlib_get_display_identifier_for_right_most_display() {
	return AXLibGetDisplayIdentifierForRightMostDisplay();
}
extern "C" CFStringRef axlib_get_display_identifier_for_left_most_display() {
	return AXLibGetDisplayIdentifierForLeftMostDisplay();
}
extern "C" CFStringRef axlib_get_display_identifier_for_bottom_most_display() {
	return AXLibGetDisplayIdentifierForBottomMostDisplay();
}

extern "C" CGRect axlib_get_display_bounds(CFStringRef DisplayRef) {
	return AXLibGetDisplayBounds(DisplayRef);
}
extern "C" bool axlib_is_display_changing_spaces(CFStringRef DisplayRef) {
	return AXLibIsDisplayChangingSpaces(DisplayRef);
}

extern "C" bool axlib_cgsspaceid_to_desktop_id(CGSSpaceID SpaceId, unsigned *OutArrangement, unsigned *OutDesktopId) {
	return AXLibCGSSpaceIDToDesktopID(SpaceId, OutArrangement, OutDesktopId);
}
extern "C" bool axlib_cgsspaceid_from_desktop_id(unsigned DesktopId, unsigned *OutArrangement, CGSSpaceID *OutSpaceId) {
	return AXLibCGSSpaceIDFromDesktopID(DesktopId, OutArrangement, OutSpaceId);
}

extern "C" int *axlib_spaces_for_display_with_count(CFStringRef DisplayRef, int *Count) {
	return AXLibSpacesForDisplay(DisplayRef, Count);
}
extern "C" macos_space **axlib_spaces_for_display(CFStringRef DisplayRef) {
	return AXLibSpacesForDisplay(DisplayRef);
}
extern "C" macos_space **axlib_spaces_for_window(uint32_t WindowId) {
	return AXLibSpacesForWindow(WindowId);
}
extern "C" void axlib_space_move_window(CGSSpaceID SpaceId, uint32_t WindowId) {
	AXLibSpaceMoveWindow(SpaceId, WindowId);
}
extern "C" void axlib_space_add_window(CGSSpaceID SpaceId, uint32_t WindowId) {
	AXLibSpaceAddWindow(SpaceId, WindowId);
}
extern "C" void axlib_space_remove_window(CGSSpaceID SpaceId, uint32_t WindowId) {
	AXLibSpaceRemoveWindow(SpaceId, WindowId);
}
extern "C" bool axlib_space_has_window(CGSSpaceID SpaceId, uint32_t WindowId) {
	return AXLibSpaceHasWindow(SpaceId, WindowId);
}
extern "C" bool axlib_sticky_window(uint32_t WindowId) {
	return AXLibStickyWindow(WindowId);
}

extern "C" bool axlib_is_menubar_auto_hide_enabled() {
	return AXLibIsMenuBarAutoHideEnabled();
}
extern "C" bool axlib_is_dock_auto_hide_enabled() {
	return AXLibIsDockAutoHideEnabled();
}

extern "C" macos_dock_orientation axlib_get_dock_orientation() {
	return AXLibGetDockOrientation();
}
extern "C" size_t axlib_get_dock_tile_size() {
	return AXLibGetDockTileSize();
}
