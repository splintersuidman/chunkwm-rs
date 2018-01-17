#ifndef CHUNKWM_COMMON_BORDER_H
#define CHUNKWM_COMMON_BORDER_H

struct border_window
{
    int Width;
    int Radius;
    unsigned Color;
};

extern "C" border_window *CreateBorderWindow(int X, int Y, int W, int H, int BorderWidth, int BorderRadius, unsigned int BorderColor);
extern "C" void UpdateBorderWindowRect(border_window *Border, int X, int Y, int W, int H);
extern "C" void UpdateBorderWindowColor(border_window *Border, unsigned Color);
extern "C" void DestroyBorderWindow(border_window *Border);

#endif
