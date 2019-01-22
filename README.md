# cfg-if

[![Build Status](https://travis-ci.com/alexcrichton/cfg-if.svg?branch=master)](https://travis-ci.com/alexcrichton/cfg-if)

[Documentation](https://docs.rs/cfg-if)

A macro to ergonomically define an item depending on a large number of #[cfg]
parameters. Structured like an if-else chain, the first matching branch is the
item that gets emitted.

```toml
[dependencies]
cfg-if = "0.1"
```

The `use_core` feature is enabled by default and builds the crate with libcore
by using the `#![no_std]` attribute. When this feature is disabled, this crate
is built without libcore support by using the `#![no_core]` attribute - this
makes use of the `#![feature(no_core)]` and requires a nightly version of Rust.

## Example

```rust
#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        fn foo() { /* fallback implementation */ }
    }
}

fn main() {
    foo();
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
