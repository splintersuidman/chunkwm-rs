# `chunkwm-rs`
A Rust 'bridge' for creating [chunkwm](https://github.com/koekeishiya/chunkwm) plugins.

## Usage
Add the following your `Cargo.toml`:
```toml
chunkwm = { git = "https://github.com/splintah/chunkwm-rs" }
```
You then have to compile it as a `cdylib`, and use the `plugin.cpp` file from the [Rust plugin template](https://github.com/splintah/chunkwm-rs-template).

If you want to get the `Makefile` template and a small Rust library template, see the [Rust plugin template](https://github.com/splintah/chunkwm-rs-template).

## Documentation
There are two ways to see the rendered documentation:
- Clone this repository, `cd` into it, and run `cargo doc`. The documentation is now available in `./target/doc/chunkwm/index.html`.
- Create your own library depending on this library, run `cargo doc`. The documentation is now available in `./target/doc/chunkwm/index.html`.
