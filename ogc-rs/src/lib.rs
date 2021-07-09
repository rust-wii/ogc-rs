//! # ogc-rs
//!
//! ``ogc-rs`` is a safe, idiomatic wrapper around ``ogc-sys``.
//!
//! ``ogc-rs`` provides many features from libogc such as:
//!
//! * ``network``: Provides TCP networking for the Wii.
//! * ``audio``: Provides functions for audio on the Wii.
//! * ``fs``: Provides functions for manipulating the filesystem on the Wii.
//! * ``system``: Provides OS functions for the Wii.
//! * ``console``: Provides console functions for the Wii.
//! * ``input``: Provides an interface for reading input from devices on the Wii.
//! * ``video``: Provides functions for video output on the Wii.
//! * ``gx``: Provides an opengl-like interface for rendering on the Wii.
//!
//! ``ogc-rs`` also provides runtime functions and an allocator for ``no_std``
//! environments.
//!
//! ## Features
//! You can currently enable the following features:
//!
//! - `ffi`: provides access to `ogc-sys` for functions that are not implemented yet.
//! - `pad`: enables support for GameCube controllers.
//!
//! For instance, to enable the `pad` module, add following to `Cargo.toml`:
//!
//! ```toml
//! [dependencies.ogc-rs]
//! version = "*"
//! features = ["pad"]
//! ```

#![no_std]
#![allow(dead_code)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
use bitflags::bitflags;

// Custom Error Implementation
pub mod error;
pub use error::{OgcError, Result};

// Networking Implementation
pub mod network;

// Audio Implementation
pub mod audio;

// Console Implementation
pub mod console;

// System Implementation
pub mod system;

/// Video Implementation
pub mod video;

/// Debugging Functions
pub mod debug;

// Utility Functions
pub mod utils;
pub use utils::*;

// Runtime Functions
pub mod runtime;

// Gamecube Pad Functions
#[cfg(feature = "pad")]
pub mod pad;

// FFI
#[cfg(feature = "ffi")]
pub extern crate ogc_sys as ffi;

/// Prelude
pub mod prelude {
    // alloc Export
    pub use alloc::boxed::Box;
    pub use alloc::string::{String, ToString};
    pub use alloc::{vec, vec::Vec};

    // Export Services
    pub use crate::console::*;
    pub use crate::debug::*;
    #[cfg(feature = "pad")]
    pub use crate::pad::*;
    pub use crate::system::*;
    pub use crate::video::*;
    pub use crate::{print, println};

    // Global Allocator
    use crate::runtime::OGCAllocator;

    #[global_allocator]
    static GLOBAL_ALLOCATOR: OGCAllocator = OGCAllocator;
}
