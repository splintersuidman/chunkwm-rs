//! The `api` module contains the raw `chunkwm_api` struct, and contains a `CVar` struct to create a
//! handle to a specific `CVar`.

use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::marker::PhantomData;

#[repr(C)]
pub enum LogLevel {
    Debug = 0,
    Warn = 1,
    Error = 2,
}

/// The `API` struct is a raw type (i.e. the same as used in C code), and contains methods for
/// creating, finding, and getting `CVar`s.
///
/// You can keep the `API` in your event handler struct, but you can also use
/// [`CVar`](./struct.CVar.html) to create handles to these `CVar`s.
#[repr(C)]
pub struct API {
    _update_cvar: unsafe extern "C" fn(*const c_char, *const c_char),
    _acquire_cvar: unsafe extern "C" fn(*const c_char) -> *const c_char,
    _find_cvar: unsafe extern "C" fn(*const c_char) -> bool,
    _plugin_broadcast: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void, usize),
    _log: unsafe extern "C" fn(level: LogLevel, format: *const c_char),
}

impl API {
    /// Checks whether a CVar has been declared.
    pub fn cvar_exists(&self, name: &str) -> bool {
        let name: *const c_char = CString::new(name).unwrap().into_raw();

        unsafe { (self._find_cvar)(name) }
    }

    /// Creates a new CVar with a value.
    ///
    /// Every value type that implements `Display` can be used as a `CVar`, since the value is
    /// formatted as a string, and then passed to the C API.
    pub fn create_cvar<V: Display>(&self, name: &str, value: &V) {
        if self.cvar_exists(name) {
            return;
        }

        let name = CString::new(name).unwrap().into_raw();
        let value = CString::new(format!("{}", value)).unwrap().into_raw();

        unsafe { (self._update_cvar)(name, value) }
    }

    /// Updates a CVar.
    ///
    /// Every value type that implements `Display` can be used as a `CVar`, since the value is
    /// formatted as a string, and then passed to the C API.
    pub fn update_cvar<V: Display>(&self, name: &str, value: &V) {
        let name = CString::new(name).unwrap().into_raw();
        let value = CString::new(format!("{}", value)).unwrap().into_raw();

        unsafe { (self._update_cvar)(name, value) }
    }

    /// Gets a CVar.
    ///
    /// The type of the value has to be specified (using the turbofish `::<>`), because the value is
    /// received as a string and then parsed to the specified type.
    pub fn get_cvar<T: FromStr>(&self, name: &str) -> Result<T, <T as FromStr>::Err> {
        let name = CString::new(name).unwrap().into_raw();
        let value = unsafe {
            let v = (self._acquire_cvar)(name);
            CStr::from_ptr(v).to_string_lossy().into_owned()
        };

        value.parse::<T>()
    }

    /// Log with a specified log level. A newline will be added to the message, so no need to do
    /// that yourself.
    pub fn log<S: AsRef<str>>(&self, level: LogLevel, message: S) {
        let message = format!("{}\n", message.as_ref());
        unsafe { (self._log)(level, CString::new(message).unwrap().into_raw()) }
    }
}

/// The `CVar` struct allows you to easily create a handle to `CVars`, that might be updated. You
/// can use all types that implement `Display` and `FromStr` as value type for the `CVar`. You can
/// therefore easily implement your own `CVar` compatible types.
///
/// ## Example
///
/// ```
/// # #[macro_use] extern crate chunkwm;
/// # use chunkwm::prelude::*;
/// pub struct Plugin {
///     global_desktop_mode: CVar<String>,
///     bsp_spawn_left: CVar<NumericBool>,
/// }
///
/// impl HandleEvent for Plugin {
///     fn new(api: &'static API) -> Plugin {
///         let global_desktop_mode = CVar::new("global_desktop_mode", &api).unwrap();
///         let bsp_spawn_left = CVar::new("bsp_spawn_left", &api).unwrap();
///         Plugin { global_desktop_mode, bsp_spawn_left }
///     }
///
///     fn handle(&mut self, _: Event) {
///         self.global_desktop_mode.set_value(&"bsp".to_owned());
///         assert_eq!(self.global_desktop_mode.get_value().unwrap(), "bsp".to_owned());
///
///         self.bsp_spawn_left.set_value(&NumericBool::from(false));
///         assert_eq!(self.bsp_spawn_left.get_value().unwrap().value, false);
///     }
///     // some methods omitted
///     # subscribe!();
///     # name!("");
///     # version!("");
///     # fn shutdown(&self) {}
/// }
/// # fn main() {}
/// ```
pub struct CVar<T: FromStr + Display> {
    /// The `CVar`'s name.
    pub name: &'static str,
    /// The reference to the api.
    api: &'static API,
    // `_value` is needed for the `T` type.
    value_marker: PhantomData<T>,
}

impl<T: FromStr + Display> CVar<T> {
    /// Create a new `CVar` handle without a value.
    pub fn new(name: &'static str, api: &'static API) -> Result<Self, &'static str> {
        let value = api.get_cvar::<T>(name);
        match value {
            Ok(_) => Ok(CVar {
                name,
                api: api,
                value_marker: PhantomData::default(),
            }),
            Err(_) => Err("could not find cvar"),
        }
    }

    /// Create a new `CVar` handle with a value. The given value will be set.
    pub fn with_value(name: &'static str, value: &T, api: &'static API) -> Self {
        api.create_cvar(name, value);
        CVar {
            name,
            api: api,
            value_marker: PhantomData::default(),
        }
    }

    /// Set the value of the `CVar`.
    pub fn set_value(&mut self, value: &T) {
        self.api.update_cvar(self.name, &value);
    }

    /// Get the value of the `CVar`.
    pub fn get_value(&mut self) -> Result<T, <T as FromStr>::Err> {
        let value = self.api.get_cvar::<T>(self.name)?;
        Ok(value)
    }
}

/// The `NumericBool` is often used in a chunkwm config. It is a boolean type that is either 0 or
/// 1 (false or true respectively).
/// This struct allows you to create a `CVar` that is a numeric bool, and easily get its value.
///
/// ## Example
/// ```rust
/// # use chunkwm::api::NumericBool;
/// use std::str::FromStr;
///
/// let a = NumericBool::from(false);
/// // ChunkWM uses strings to pass values internally.
/// let b = NumericBool::from_str("1").unwrap();
///
/// assert_eq!(a.value, false);
/// assert_eq!(b.value, true);
/// ```
#[derive(Clone, Copy)]
pub struct NumericBool {
    pub value: bool,
}

impl Display for NumericBool {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let v: u8 = self.into();
        write!(f, "{}", v)
    }
}

impl From<bool> for NumericBool {
    fn from(v: bool) -> Self {
        NumericBool { value: v }
    }
}

impl<'a> From<&'a NumericBool> for u8 {
    fn from(v: &NumericBool) -> u8 {
        if v.value {
            1
        } else {
            0
        }
    }
}

impl Into<u8> for NumericBool {
    fn into(self) -> u8 {
        if self.value {
            1
        } else {
            0
        }
    }
}

impl FromStr for NumericBool {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(NumericBool::from(false)),
            "1" => Ok(NumericBool::from(true)),
            _ => Err("could not "),
        }
    }
}
