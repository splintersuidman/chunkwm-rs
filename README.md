# chunkwm-rs [![Build Status](https://travis-ci.org/splintah/chunkwm-rs.svg?branch=master)][travis]
A Rust 'bridge' for creating [chunkwm] plugins.

## Features
- [x] Event handler.
- [x] Easy API.
- [x] `CVar` support.
- [x] Subscriptions.
- [x] Border methods (feature `border`, see [Cargo features](#cargo-features)).
- [x] Accessibility methods (feature `accessibility`, see [Cargo features](#cargo-features)).

## Usage
Add the following your `Cargo.toml`:
```toml
[dependencies]
chunkwm = { git = "https://github.com/splintah/chunkwm-rs" }
```
You then have to compile it as a `cdylib`, and use the `plugin.cpp` file from the [Rust plugin template](https://github.com/splintah/chunkwm-rs-template).

If you want to get the `Makefile` template and a small Rust library template, see the [Rust plugin template](https://github.com/splintah/chunkwm-rs-template).

## Documentation
There are two ways to see the rendered documentation:
- Clone this repository, `cd` into it, and run `cargo doc`. The documentation is now available in `./target/doc/chunkwm/index.html`.
- Create your own library depending on this library, run `cargo doc`. The documentation is now available in `./target/doc/chunkwm/index.html`.

## Cargo features
There are two features that toggle the compilation of the C/C++ library: `border` and `accessibility`.
The `border` features gives you access to the `chunkwm::common::border` path.
The `border` features gives you access to the `chunkwm::common::accessibility` path, and enables some extra methods on `Window` and `Application`.

To use these features:
```toml
[dependencies]
chunkwm = { git = "https://github.com/splintah/chunkwm-rs", features = ["border", "accessibility"] }
```

You can, of course, only use the features you're interested in.

## TODO / Contributing
I want to add the functions the chunkwm api provides to the Rust plugin, but that takes some time.
Contributions are very welcome!

[Rust plugin template]: https://github.com/splintah/chunkwm-rs-template
[chunkwm]: https://github.com/koekeishiya/chunkwm
[travis]: https://travis-ci.org/splintah/chunkwm-rs
