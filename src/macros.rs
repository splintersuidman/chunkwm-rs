/// A macro to specify the plugin's name.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// impl HandleEvent for Plugin {
///     # fn new(_: &'static API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # subscribe_all!();
///     # version!("");
///     name!("my-awesome-plugin");
///     // some methods omitted
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! name {
    ($name:expr) => {
        fn name() -> &'static str { $name }
    };
}

/// A macro to specify the plugin's version.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// impl HandleEvent for Plugin {
///     # fn new(_: &'static API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # subscribe_all!();
///     # name!("");
///     version!("1.2.3");
///     // some methods omitted
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! version {
    ($version:expr) => {
        fn version() -> &'static str { $version }
    };
}

/// This macro makes it easy to subscribe to events.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// impl HandleEvent for Plugin {
///     # fn new(_: &'static API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # name!("");
///     # version!("");
///     // some methods omitted
///     subscribe!(Subscription::WindowFocused, Subscription::ApplicationLaunched);
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! subscribe {
    ($($subscription:expr),*) => {
        fn subscribe() -> &'static [chunkwm::event::Subscription] {
            &[
                $(
                    $subscription,
                )*
            ]
        }
    };
}

/// A macro to subscribe to all events.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// impl HandleEvent for Plugin {
///     # fn new(_: &'static API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # name!("");
///     # version!("");
///     // some methods omitted
///     subscribe_all!();
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! subscribe_all {
    () => {
        fn subscribe() -> &'static [chunkwm::event::Subscription] {
            Subscription::all()
        }
    };
}

/// This macro creates the bridge to the C/C++ plugin. When writing a handler this should always be
/// done. You should pass your plugin struct's identifier to this macro.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// create_c_bridge!(Plugin);
/// impl HandleEvent for Plugin {
///     // methods omitted
///     # fn new(_: &'static API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # name!("");
///     # version!("");
///     # subscribe_all!();
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! create_c_bridge {
    ($struct_ident:ident) => {pub mod _export {
        use chunkwm::prelude::*;
        use super::$struct_ident;
        use std::os::raw::c_char;
        use std::ffi;
        use std::mem::transmute;

        #[repr(C)]
        pub struct SubscriptionArray {
            len: usize,
            arr: *const Subscription,
        }

        // Create a new handler and return it. In C/C++ a `void *` can be used.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_create_handler(api: &'static API) -> *mut $struct_ident {
            let _handler = transmute(Box::new($struct_ident::new(&api)));
            _handler
        }

        //
        #[no_mangle]
        pub extern "C" fn chunkwm_rust_get_name() -> *const c_char {
            ffi::CString::new($struct_ident::name()).unwrap().into_raw()
        }

        //
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_get_version() -> *const c_char {
            ffi::CString::new($struct_ident::version())
                .unwrap()
                .into_raw()
        }

        //
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_subscribe_to_events() -> SubscriptionArray {
            let subscriptions = $struct_ident::subscribe();
            SubscriptionArray {
                len: subscriptions.len(),
                arr: subscriptions.as_ptr(),
            }
        }

        // Runs `shutdown` on the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_shutdown_handler(ptr: *mut $struct_ident) {
            let _handler = &mut *ptr;
            _handler.shutdown();
        }

        // Send events containing an application to the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event_with_application(
            ptr: *mut $struct_ident,
            event: *const c_char,
            application: RawApplication,
        ) {
            let _handler = &mut *ptr;
            let application: Box<Application> = Box::new(application.into());
            let event = ffi::CStr::from_ptr(event).to_string_lossy().into_owned();

            match event.as_str() {
                "chunkwm_export_application_launched" => {
                    _handler.handle(Event::ApplicationLaunched(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_application_terminated" => {
                    _handler.handle(Event::ApplicationTerminated(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_application_activated" => {
                    _handler.handle(Event::ApplicationActivated(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_application_deactivated" => {
                    _handler.handle(Event::ApplicationDeactivated(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_application_hidden" => {
                    _handler.handle(Event::ApplicationHidden(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_application_unhidden" => {
                    _handler.handle(Event::ApplicationUnhidden(application))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                _ => {
                    _handler.handle(Event::Other(event))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
            };
        }

        // Send events containing a window to the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event_with_window(
            ptr: *mut $struct_ident,
            event: *const c_char,
            window: RawWindow,
        ) {
            let _handler = &mut *ptr;
            let window: Box<Window> = Box::new(window.into());
            let event = ffi::CStr::from_ptr(event).to_string_lossy().into_owned();

            match event.as_str() {
                "chunkwm_export_window_created" => {
                    _handler.handle(Event::WindowCreated(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_destroyed" => {
                    _handler.handle(Event::WindowDestroyed(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_focused" => {
                    _handler.handle(Event::WindowFocused(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_moved" => {
                    _handler.handle(Event::WindowMoved(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_resized" => {
                    _handler.handle(Event::WindowResized(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_minimized" => {
                    _handler.handle(Event::WindowMinimized(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_window_deminimized" => {
                    _handler.handle(Event::WindowDeminimized(window))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                _ => {
                    _handler.handle(Event::Other(event))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
            };
        }

        // Send events containing a display to the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event_with_display(
            ptr: *mut $struct_ident,
            event: *const c_char,
            display: DisplayID,
        ) {
            let _handler = &mut *ptr;
            let event = ffi::CStr::from_ptr(event).to_string_lossy().into_owned();

            match event.as_str() {
                "chunkwm_export_display_added" => {
                    _handler.handle(Event::DisplayAdded(display))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_display_removed" => {
                    _handler.handle(Event::DisplayRemoved(display))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_display_moved" => {
                    _handler.handle(Event::DisplayMoved(display))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_display_resized" => {
                    _handler.handle(Event::DisplayResized(display))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                _ => {
                    _handler.handle(Event::Other(event))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
            };
        }

        // Send events containing payloads to the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event_with_daemon_command(
            ptr: *mut $struct_ident,
            payload: RawPayload,
        ) {
            let _handler = &mut *ptr;
            let payload: Payload = payload.into();

            _handler.handle(Event::DaemonCommand(payload))
                .unwrap_or_else(|e| eprintln!("{}", e));
        }

        // Send events containing 'nothing' or unknown events to the event handler.
        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event_with_nothing(
            ptr: *mut $struct_ident,
            event: *const c_char,
        ) {
            let _handler = &mut *ptr;
            let event = ffi::CStr::from_ptr(event).to_string_lossy().into_owned();

            match event.as_str() {
                "chunkwm_export_display_changed" => {
                    _handler.handle(Event::DisplayChanged)
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                "chunkwm_export_space_changed" => {
                    _handler.handle(Event::SpaceChanged)
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
                _ => {
                    _handler.handle(Event::Other(event))
                        .unwrap_or_else(|e| eprintln!("{}", e))
                }
            }
        }
    }};
}
