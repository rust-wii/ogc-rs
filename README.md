# ogc-rs

![Crates.io](https://img.shields.io/crates/v/ogc-rs)

A Rust wrapper library for devkitPro's [libogc](https://github.com/devkitPro/libogc).

See the [Wii testing project](https://github.com/rust-wii/testing-project) for an example on how to use this library.

## Installation

To get started, you'll first need to install the following dependencies on your system:
* [Rust, along with rustup and cargo](https://www.rust-lang.org/tools/install)
* the Clang compiler
  * from your local package manager,
  * or from [LLVM themselves](https://clang.llvm.org/get_started.html)
* [devkitPro toolchain](https://devkitpro.org/wiki/Getting_Started)

Then you'll need to fork this repo and `git clone` your fork into your local machine.

When that's done, do the following:

```sh
$ cd ogc-rs
$ cargo check
```

The [`rust-toolchain.toml`](rust-toolchain.toml) file specifies the nightly Rust toolchain to use, which will automatically download and set up the toolchain for you.

If everything's working properly, `cargo check` should run successfully.

### Nix Flake

Alternatively, if you use the Nix package manager with the Flakes feature enabled, we offer a [`flake.nix`](flake.nix) that provides a convenient way to build and use this library.

The flake will read the [`rust-toolchain.toml`](rust-toolchain.toml) file to automatically install and set up a nightly Rust toolchain using the [`rust-overlay` flake](https://github.com/oxalica/rust-overlay).
Additionally, the devkitPro toolchain is automatically installed for you using devkitPro's official Docker images that are patched to run in a Nix environment.

To get started, fork this repo and `git clone` your fork into your local machine.
Once it's finished, `cd` into the `ogc-rs` directory and run `nix develop` to enter the development shell.
If everything is working, running `cargo check` should succeed.

## Structure

This repository is organized as follows:

* `ogc-rs`: Safe, idiomatic wrapper around `ogc-sys`.
* `ogc-sys`: Low-level, unsafe bindings to libogc.

## License

See [LICENSE](LICENSE) for more details.
