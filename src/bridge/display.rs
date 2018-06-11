//! The `display` module contains functions for handling displays and spaces.

#[cfg(feature = "accessibility")]
use bridge::window::Window;
use core_foundation::string::CFStringRef;
use core_graphics::display::CGDirectDisplayID;
use raw::*;
use ChunkWMError;

#[cfg(feature = "accessibility")]
use common::accessibility::display;

/// The way the dock is oriented.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum DockOrientation {
    Top = 1,
    Bottom = 2,
    Left = 3,
    Right = 4,
}

/// The space type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum SpaceType {
    /// Space is handled by user.
    User,
    /// Space is a fullscreen application.
    Fullscreen,
    /// Space is handled by sytem.
    System,
    Unknown,
}

/// The `Display` struct.
pub struct Display(DisplayRef);

impl Display {
    /// Get the number of displays.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn count() -> u32 {
        unsafe { display::display_count() }
    }

    /// Get the raw `DisplayRef`.
    pub fn display_ref(&self) -> Result<DisplayRef, ChunkWMError> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Get the display's identifier (`CFStringRef`).
    pub fn display_identifier_ref(&self) -> Result<CFStringRef, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).display_ref) }
    }

    /// Get the display's id.
    pub fn id(&self) -> Result<CGDirectDisplayID, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).id) }
    }

    /// Get the display arrangement.
    pub fn arrangement(&self) -> Result<u32, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).arrangement) }
    }

    /// Get the x coordinate of the display.
    pub fn x(&self) -> Result<f32, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).x) }
    }

    /// Get the y coordinate of the display.
    pub fn y(&self) -> Result<f32, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).y) }
    }

    /// Get the display's width.
    pub fn width(&self) -> Result<f32, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).width) }
    }

    /// Get the display's height.
    pub fn height(&self) -> Result<f32, ChunkWMError> {
        unsafe { Ok((*self.display_ref()?).height) }
    }

    /// Destroy the display.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), ChunkWMError> {
        unsafe { display::destroy_display(self.display_ref()?) };
        Ok(())
    }
}

/// The `Space` struct.
pub struct Space(SpaceRef);

impl Space {
    /// Get the raw `SpaceRef`.
    pub unsafe fn space_ref(&self) -> Result<SpaceRef, ChunkWMError> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Get the identifier (`CFStringRef`).
    pub fn space_identifier_ref(&self) -> Result<CFStringRef, ChunkWMError> {
        unsafe { Ok((*self.space_ref()?).space_ref) }
    }

    /// Get the space's id.
    pub fn id(&self) -> Result<CGSSpaceID, ChunkWMError> {
        unsafe { Ok((*self.space_ref()?).id) }
    }

    /// Get the space type.
    pub fn space_type(&self) -> Result<SpaceType, ChunkWMError> {
        unsafe { Ok((*self.space_ref()?).space_type) }
    }

    /// Check whether the space is active.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn is_active(&self) -> Result<bool, ChunkWMError> {
        unsafe { Ok(display::is_space_active(&mut self.space_ref()?)) }
    }

    /// Destroy the space.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), ChunkWMError> {
        unsafe { display::destroy_space(self.space_ref()?) };
        Ok(())
    }

    /// Check whether the space contains a window.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn has_window(&self, window: Window) -> Result<bool, ChunkWMError> {
        unsafe { Ok(display::space_has_window(self.id()?, window.id()?)) }
    }
}
