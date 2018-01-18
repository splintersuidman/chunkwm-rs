//! The border module contains a wrapper for the C methods.

use std::os::raw::{c_int, c_uint};
use std::ptr;

#[repr(C)]
#[derive(Debug)]
struct BorderWindow {
    pub width: c_int,
    pub radius: c_int,
    pub color: c_uint,
}

type BorderWindowRef = *mut BorderWindow;

#[link(name = "Cocoa", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_CreateBorderWindow"]
    fn CreateBorderWindow(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        border_width: c_int,
        border_radius: c_int,
        border_color: c_uint,
    ) -> BorderWindowRef;

    #[link_name = "\u{1}_UpdateBorderWindowRect"]
    fn UpdateBorderWindowRect(border: BorderWindowRef, x: c_int, y: c_int, w: c_int, h: c_int);
    #[link_name = "\u{1}_UpdateBorderWindowColor"]
    fn UpdateBorderWindowColor(border: BorderWindowRef, color: c_uint);
    #[link_name = "\u{1}_DestroyBorderWindow"]
    fn DestroyBorderWindow(border: BorderWindowRef);
}

/// This struct is the wrapper for the border methods.
pub struct Border(BorderWindowRef);

impl Border {
    /// Create a new border. The color format is 0xRRGGBBAA.
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        border_width: i32,
        border_radius: i32,
        border_color: u32,
    ) -> Self {
        let border_window_ref = unsafe {
            CreateBorderWindow(x, y, w, h, border_width, border_radius, border_color)
        };
        Border(border_window_ref)
    }

    /// Set the border rectangle.
    pub fn set_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            Err("couldn't set rect: outlived pointer")
        } else {
            unsafe { UpdateBorderWindowRect(self.0, x, y, w, h) }
            Ok(())
        }
    }

    /// Set the border color. The color format is 0xRRGGBBAA.
    pub fn set_color(&self, color: u32) -> Result<(), &'static str> {
        if self.0 != ptr::null_mut() {
            Err("couldn't set color: outlived pointer")
        } else {
            unsafe { UpdateBorderWindowColor(self.0, color) }
            Ok(())
        }
    }

    /// Destroy the border.
    pub fn destroy(&self) {
        if self.0 != ptr::null_mut() {
            unsafe { DestroyBorderWindow(self.0) }
        }
    }
}

impl Drop for Border {
    fn drop(&mut self) {
        if self.0 != ptr::null_mut() {
            unsafe { DestroyBorderWindow(self.0) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_border() {
        let _ = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
    }

    #[test]
    fn update_border() {
        // NOTE(splintah): in this test, the BorderWindowRef is outlived by the struct. In plugins
        // this should not be a problem.
        let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
        assert!(border.set_rect(100, 100, 100, 100).is_err());
        assert!(border.set_color(0x00FF00FF).is_err());
    }

    #[test]
    fn destroy_border() {
        let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
        border.destroy();
    }
}
