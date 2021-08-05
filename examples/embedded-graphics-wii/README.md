# Embedded Graphics showcase for [ogc-rs](https://github.com/rust-wii/ogc-rs)

Implements some of the [embedded-graphics](https://crates.io/crates/embedded-graphics) API for the Wii.

`cargo +nightly build +Zbuild-std=core,alloc --target powerpc-unknown-eabi.json` to compile. 
`elf2dol` is used to convert them to a format that Wii & Gamecube can use. 
