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
        use std::os::raw::{c_char, c_void};
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

        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_send_event(handler_ptr: *mut $struct_ident, event: *const c_char, data: *mut c_void) -> bool {
            let _handler = &mut *handler_ptr;
            let event = ffi::CStr::from_ptr(event).to_string_lossy().into_owned();

            let handled: Result<(), ChunkWMError> = match event.as_str() {
                "chunkwm_export_application_launched" => {
                    _handler.handle(Event::ApplicationLaunched(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_application_terminated" => {
                    _handler.handle(Event::ApplicationTerminated(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_application_activated" => {
                    _handler.handle(Event::ApplicationActivated(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_application_deactivated" => {
                    _handler.handle(Event::ApplicationDeactivated(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_application_hidden" => {
                    _handler.handle(Event::ApplicationHidden(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_application_unhidden" => {
                    _handler.handle(Event::ApplicationUnhidden(Box::new((data as ApplicationRef).into())))
                }
                "chunkwm_export_window_created" => {
                    _handler.handle(Event::WindowCreated(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_destroyed" => {
                    _handler.handle(Event::WindowDestroyed(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_focused" => {
                    _handler.handle(Event::WindowFocused(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_moved" => {
                    _handler.handle(Event::WindowMoved(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_resized" => {
                    _handler.handle(Event::WindowResized(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_minimized" => {
                    _handler.handle(Event::WindowMinimized(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_deminimized" => {
                    _handler.handle(Event::WindowDeminimized(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_window_title_changed" => {
                    _handler.handle(Event::WindowTitleChanged(Box::new((data as WindowRef).into())))
                }
                "chunkwm_export_display_added" => {
                    _handler.handle(Event::DisplayAdded(*(data as *mut DisplayID)))
                }
                "chunkwm_export_display_removed" => {
                    _handler.handle(Event::DisplayRemoved(*(data as *mut DisplayID)))
                }
                "chunkwm_export_display_moved" => {
                    _handler.handle(Event::DisplayMoved(*(data as *mut DisplayID)))
                }
                "chunkwm_export_display_resized" => {
                    _handler.handle(Event::DisplayResized(*(data as *mut DisplayID)))
                }
                "chunkwm_export_display_changed" => {
                    _handler.handle(Event::DisplayChanged)
                }
                "chunkwm_export_space_changed" => {
                    _handler.handle(Event::SpaceChanged)
                }
                "chunkwm_daemon_command" => {
                    _handler.handle(Event::DaemonCommand((data as PayloadRef).into()))
                }
                _ => {
                    _handler.handle(Event::Other(event))
                }
            };

            match handled {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    false
                }
            }
        }
    }};
}
