# ogc-rs

![Crates.io](https://img.shields.io/crates/v/ogc-rs)

A Rust wrapper library for devkitPro's [libogc](https://github.com/devkitPro/libogc).

To get started, you'll first need to install the following dependencies on your system:
* [Rust, along with rustup and cargo](https://www.rust-lang.org/tools/install)
* the Clang compiler
  * from your local package manager,
  * or from [LLVM themselves](https://clang.llvm.org/get_started.html)
* [devkitPro toolchain](https://devkitpro.org/wiki/Getting_Started)

### Nix (optional)

If you use [Nix](https://nixos.org/) with flakes enabled:

```sh
$ nix develop
```

This provides a nightly Rust toolchain (with `rust-src`), Clang/libclang for bindgen,
and common host build tools. Cross-compiling for the Wii still requires the
devkitPro toolchain as documented below; the flake covers the host-side Rust
environment only.

Then you'll need to fork this repo and `git clone` your fork into your local machine.

When that's done, do the following:

```sh
$ cd ogc-rs
$ rustup override set nightly
$ rustup component add rust-src
$ cargo check
```

If everything's working properly, `cargo check` should run successfully.

See the [Wii testing project](https://github.com/rust-wii/testing-project) for an example on how to use this library.

## Structure

This repository is organized as follows:

* `ogc-rs`: Safe, idiomatic wrapper around `ogc-sys`.
* `ogc-sys`: Low-level, unsafe bindings to libogc.

## License

See [LICENSE](LICENSE) for more details.
