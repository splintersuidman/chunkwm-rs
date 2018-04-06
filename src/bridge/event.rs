//! The `event` module contains types and traits for event handling.

use ChunkWMError;
use api::*;
use application::*;
use core_graphics::display::CGDirectDisplayID;
use payload::*;
use window::*;

/// The display identifier.
pub type DisplayID = CGDirectDisplayID;

/// The `HandleEvent` trait should be implemented on your Plugin struct.
pub trait HandleEvent {
    /// Create a new instance of `Self`. This is run at the plugin's startup.
    fn new(api: API) -> Self;
    /// Subscribe to events.
    fn subscribe() -> &'static [Subscription];
    /// Handle an `Event`.
    fn handle(&mut self, event: Event) -> Result<(), ChunkWMError>;
    /// Is run when the plugin has been stopped.
    fn shutdown(&self);
}

/// The `Event` enum allows for easy pattern matching on events.
pub enum Event {
    ApplicationLaunched(Application),
    ApplicationTerminated(Application),
    ApplicationActivated(Application),
    ApplicationDeactivated(Application),
    ApplicationHidden(Application),
    ApplicationUnhidden(Application),
    WindowCreated(Window),
    WindowDestroyed(Window),
    WindowFocused(Window),
    WindowMoved(Window),
    WindowResized(Window),
    WindowMinimized(Window),
    WindowDeminimized(Window),
    WindowTitleChanged(Window),
    DisplayAdded(DisplayID),
    DisplayRemoved(DisplayID),
    DisplayMoved(DisplayID),
    DisplayResized(DisplayID),
    DisplayChanged,
    SpaceChanged,
    DaemonCommand(Payload),
    Other(String),
}

/// The `Subscription` enum is used to subscribe to chunkwm events.
// NOTE(splintah): order must be the same as the C version.
#[repr(C)]
pub enum Subscription {
    ApplicationLaunched,
    ApplicationTerminated,
    ApplicationActivated,
    ApplicationDeactivated,
    ApplicationHidden,
    ApplicationUnhidden,

    SpaceChanged,
    DisplayChanged,

    DisplayAdded,
    DisplayRemoved,
    DisplayMoved,
    DisplayResized,

    WindowCreated,
    WindowDestroyed,
    WindowFocused,
    WindowMoved,
    WindowResized,
    WindowMinimized,
    WindowDeminimized,
    WindowTitleChanged,
    // Count,
}

impl Subscription {
    /// Subscribe to all events.
    pub fn all() -> &'static [Subscription] {
        &[
            Subscription::ApplicationLaunched,
            Subscription::ApplicationTerminated,
            Subscription::ApplicationActivated,
            Subscription::ApplicationDeactivated,
            Subscription::ApplicationHidden,
            Subscription::ApplicationUnhidden,
            Subscription::SpaceChanged,
            Subscription::DisplayChanged,
            Subscription::DisplayAdded,
            Subscription::DisplayRemoved,
            Subscription::DisplayMoved,
            Subscription::DisplayResized,
            Subscription::WindowCreated,
            Subscription::WindowDestroyed,
            Subscription::WindowFocused,
            Subscription::WindowMoved,
            Subscription::WindowResized,
            Subscription::WindowMinimized,
            Subscription::WindowDeminimized,
            Subscription::WindowTitleChanged,
        ]
    }
}
