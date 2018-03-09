/// A macro to specify the plugin's name.
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
///     # fn new(_: API) -> Self { Plugin }
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
///     # fn new(_: API) -> Self { Plugin }
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
///     # fn new(_: API) -> Self { Plugin }
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
