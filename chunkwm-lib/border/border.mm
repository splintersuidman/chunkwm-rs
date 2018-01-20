#include "../../chunkwm/src/common/border/border.mm"

extern "C" border_window *create_border_window(int X, int Y, int W, int H, int BorderWidth, int BorderRadius, unsigned int BorderColor) {
    return CreateBorderWindow(X, Y, W, H, BorderWidth, BorderRadius, BorderColor);
};
extern "C" void update_border_window_rect(border_window *Border, int X, int Y, int W, int H) {
    UpdateBorderWindowRect(Border, X, Y, W, H);
};
extern "C" void update_border_window_color(border_window *Border, unsigned Color) {
    UpdateBorderWindowColor(Border, Color);
};
extern "C" void destroy_border_window(border_window *Border) {
    DestroyBorderWindow(Border);
};
