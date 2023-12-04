# rust_playground

This repository experiments with graphing in the plotters crate using cargo-script, which is discussed in 3424 of the Rust RFC.

<https://internals.rust-lang.org/t/pre-rfc-cargo-script-for-everyone/18639/1>

<https://docs.rs/plotters/latest/plotters/>

Since we are still writing with RFC functionality, we need to switch this directory to Rust's Nightly in order to use it.

Execute the following command directly under the directory to overwrite the settings in the directory so that the rustup toolchain selects Nightly.

```
rustup update
rustup install nightly
rustup override set nightly
```