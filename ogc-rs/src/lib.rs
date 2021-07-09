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

// Gu Implementation
pub mod gu;

// Runtime Functions
pub mod runtime;

// Gx Implementation
pub mod gx;

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
    pub use crate::gu::*;
    pub use crate::system::*;
    pub use crate::video::*;
    pub use crate::gx::*;
    pub use crate::{print, println};

    // Global Allocator
    use crate::runtime::OGCAllocator;

    #[global_allocator]
    static GLOBAL_ALLOCATOR: OGCAllocator = OGCAllocator;
}
