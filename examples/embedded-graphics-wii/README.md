# Embedded Graphics showcase for [ogc-rs](https://github.com/rust-wii/ogc-rs)

Implements some of the [embedded-graphics](https://crates.io/crates/embedded-graphics) API for the Wii.

Run `cargo build` to compile.

You can run this program on a Wii on your local network by using [`wiiload`](https://wiibrew.org/wiki/Wiiload) on your PC, and loading up the Homebrew Channel on the Wii.

If the [Dolphin emulator](https://dolphin-emu.org/) is installed, run `dolphin-emu-nogui -e target/powerpc-unknown-eabi/debug/embedded-graphics-wii.elf` to emulate the program directly.

If necessary, use `elf2dol` to convert the binary to a format that Wii & Gamecube can use.
