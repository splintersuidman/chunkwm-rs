/// This macro makes it easy to subscribe to events.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
/// impl HandleEvent for Plugin {
///     # fn new(_: API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
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
///     # fn new(_: API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
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

/// The macro that exports the functions and values `ChunkWM` uses.
///
/// **NOTE**: the file name, plugin name and plugin version **must**
/// - be a byte literal, you can achieve this using `b"string"`;
/// - contain a `'\0'` at the end.
///
/// See [this comment on GitHub](https://github.com/koekeishiya/chunkwm/issues/122#issuecomment-371910155)
/// for more information.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin;
///
/// chunkwm_plugin!{
///     Plugin,
///     file: b"file name\0",
///     name: b"plugin name\0",
///     version: b"plugin version\0"
/// }
///
/// impl HandleEvent for Plugin {
///     # fn new(_: API) -> Self { Plugin }
///     # fn handle(&mut self, _: Event) -> Result<(), ChunkWMError> { Ok(()) }
///     # fn shutdown(&self) {}
///     # subscribe!();
///     // methods omitted
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! chunkwm_plugin {
    {
        $struct_ident:ident,
        file: $file_name:expr,
        name: $plugin_name:expr,
        version: $plugin_version:expr
} => {pub mod __export {
        extern crate chunkwm;
        use super::$struct_ident;
        use chunkwm::prelude::*;
        use chunkwm::raw::{ChunkWMPlugin, ChunkWMPluginDetails};
        use std::ffi;
        use std::os::raw::{c_char, c_uint, c_void};

        static mut PLUGIN: Option<$struct_ident> = None;

        pub extern "C" fn chunkwm_plugin_main(node: *const c_char, data: *mut c_void) -> bool {
            use chunkwm::event::DisplayID;
            let event = unsafe { ffi::CStr::from_ptr(node).to_string_lossy().into_owned() };

            let event = match event.as_str() {
                "chunkwm_export_application_launched" => Event::ApplicationLaunched(data.into()),
                "chunkwm_export_application_terminated" => {
                    Event::ApplicationTerminated(data.into())
                }
                "chunkwm_export_application_activated" => Event::ApplicationActivated(data.into()),
                "chunkwm_export_application_deactivated" => {
                    Event::ApplicationDeactivated(data.into())
                }
                "chunkwm_export_application_hidden" => Event::ApplicationHidden(data.into()),
                "chunkwm_export_application_unhidden" => Event::ApplicationUnhidden(data.into()),
                "chunkwm_export_window_created" => Event::WindowCreated(data.into()),
                "chunkwm_export_window_destroyed" => Event::WindowDestroyed(data.into()),
                "chunkwm_export_window_focused" => Event::WindowFocused(data.into()),
                "chunkwm_export_window_moved" => Event::WindowMoved(data.into()),
                "chunkwm_export_window_resized" => Event::WindowResized(data.into()),
                "chunkwm_export_window_minimized" => Event::WindowMinimized(data.into()),
                "chunkwm_export_window_deminimized" => Event::WindowDeminimized(data.into()),
                "chunkwm_export_window_sheet_created" => Event::WindowSheetCreated(data.into()),
                "chunkwm_export_window_title_changed" => Event::WindowTitleChanged(data.into()),
                "chunkwm_export_display_added" => unsafe {
                    Event::DisplayAdded(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_removed" => unsafe {
                    Event::DisplayRemoved(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_moved" => unsafe {
                    Event::DisplayMoved(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_resized" => unsafe {
                    Event::DisplayResized(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_changed" => Event::DisplayChanged,
                "chunkwm_export_space_changed" => Event::SpaceChanged,
                "chunkwm_daemon_command" => Event::DaemonCommand(data.into()),
                _ => Event::Other(event),
            };

            unsafe {
                if let Some(ref mut plugin) = PLUGIN {
                    match plugin.handle(event) {
                        Ok(_) => true,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            false
                        }
                    }
                } else {
                    eprintln!("Error: plugin was not initialised.");
                    return false;
                }
            }
        }

        pub extern "C" fn chunkwm_plugin_init(api: API) -> bool {
            unsafe {
                PLUGIN = Some($struct_ident::new(api));
            }
            true
        }

        pub extern "C" fn chunkwm_plugin_deinit() {
            unsafe {
                if let Some(ref plugin) = PLUGIN {
                    plugin.shutdown();
                }
            }
        }

        pub unsafe extern "C" fn chunkwm_init_plugin_vtable(plugin: *mut ChunkWMPlugin) {
            (*plugin).init = chunkwm_plugin_init;
            (*plugin).deinit = chunkwm_plugin_deinit;
            (*plugin).run = chunkwm_plugin_main;
        }

        pub unsafe extern "C" fn chunkwm_init_plugin_subscriptions(plugin: *mut ChunkWMPlugin) {
            (*plugin).subscriptions = $struct_ident::subscribe().as_ptr();
            (*plugin).subscription_count = $struct_ident::subscribe().len() as c_uint;
        }

        #[no_mangle]
        pub extern "C" fn GetPlugin() -> *mut ChunkWMPlugin {
            use std::sync::{Once, ONCE_INIT};
            static INIT: Once = ONCE_INIT;

            static mut SINGLETON: ChunkWMPlugin = {
                use std::ptr;
                extern "C" fn _init(_api: API) -> bool {
                    false
                }
                extern "C" fn _deinit() {}
                extern "C" fn _run(_node: *const c_char, _data: *mut c_void) -> bool {
                    false
                }
                ChunkWMPlugin {
                    init: _init,
                    deinit: _deinit,
                    run: _run,
                    subscriptions: ptr::null_mut(),
                    subscription_count: 0,
                }
            };

            unsafe {
                INIT.call_once(|| {
                    chunkwm_init_plugin_vtable(&mut SINGLETON);
                    chunkwm_init_plugin_subscriptions(&mut SINGLETON);
                });

                &mut SINGLETON
            }
        }

        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static Exports: ChunkWMPluginDetails = ChunkWMPluginDetails {
            magic: [
                b'c' as c_char,
                b'h' as c_char,
                b'w' as c_char,
                b'm' as c_char,
                b'p' as c_char,
                b'l' as c_char,
            ],
            api_version: 8,
            file_name: $file_name as *const u8,
            plugin_name: $plugin_name as *const u8,
            plugin_version: $plugin_version as *const u8,
            initialize: GetPlugin,
        };
    }};
}
