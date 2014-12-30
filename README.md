winapi-rs
=========
This crate provides FFI bindings for WinAPI. They are gathered by hand using the very latest official SDK from Microsoft.

If this crate is missing something you need, please hit me up on IRC (WalrusPony on #rust on Mozilla IRC) or open an issue on Github if you hate IRC, and I'll add it in.

This crate makes use of Cargo features to conditionally include the function bindings for each winapi library. See `Cargo.toml` for a list of these features.
