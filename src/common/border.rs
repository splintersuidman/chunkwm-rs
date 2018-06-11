//! The `border` module contains a wrapper for the C methods that handle borders.

use std::os::raw::{c_int, c_uint};
use ChunkWMError;

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
    #[link_name = "\u{1}_create_border_window"]
    fn create_border_window(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        border_width: c_int,
        border_radius: c_int,
        border_color: c_uint,
    ) -> BorderWindowRef;

    #[link_name = "\u{1}_update_border_window_rect"]
    fn update_border_window_rect(border: BorderWindowRef, x: c_int, y: c_int, w: c_int, h: c_int);
    #[link_name = "\u{1}_update_border_window_color"]
    fn update_border_window_color(border: BorderWindowRef, color: c_uint);
    #[link_name = "\u{1}_update_border_window_width"]
    fn update_border_window_width(border: BorderWindowRef, width: c_int);
    #[link_name = "\u{1}_destroy_border_window"]
    fn destroy_border_window(border: BorderWindowRef);
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
        let border_window_ref =
            unsafe { create_border_window(x, y, w, h, border_width, border_radius, border_color) };
        Border(border_window_ref)
    }

    /// Set the border rectangle.
    pub fn set_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Result<(), ChunkWMError> {
        if !self.0.is_null() {
            unsafe { update_border_window_rect(self.0, x, y, w, h) }
            Ok(())
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Set the border color. The color format is 0xRRGGBBAA.
    ///
    /// # Warning: (maybe) do not use (yet).
    /// TODO(splintah): (signal: 11, SIGSEGV: invalid memory reference)...
    pub fn set_color(&self, color: u32) -> Result<(), ChunkWMError> {
        if !self.0.is_null() {
            unsafe { update_border_window_color(self.0, color) }
            Ok(())
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Set the border width.
    ///
    /// # Warning: (maybe) do not use (yet).
    /// TODO(splintah): (signal: 11, SIGSEGV: invalid memory reference)...
    pub fn set_width(&self, width: i32) -> Result<(), ChunkWMError> {
        if !self.0.is_null() {
            unsafe { update_border_window_width(self.0, width) }
            Ok(())
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Destroy the border.
    pub fn destroy(&self) {
        if !self.0.is_null() {
            unsafe { destroy_border_window(self.0) }
        }
    }
}

impl Drop for Border {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { destroy_border_window(self.0) }
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
    fn update_border_rect() {
        let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
        assert!(border.set_rect(100, 100, 100, 100).is_ok());
    }

    // #[test]
    // fn update_border_color() {
    //     let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
    //     assert!(border.set_color(0x00FF00FF).is_ok());
    // }

    // #[test]
    // fn update_width() {
    //     let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
    //     assert!(border.set_width(10).is_ok())
    // }

    #[test]
    fn destroy_border() {
        let border = Border::new(0, 0, 100, 100, 5, 5, 0xFF0000FF);
        border.destroy();
    }
}
