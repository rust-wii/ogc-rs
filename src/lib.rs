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
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![feature(slice_ptr_get)]
#![feature(allocator_api)]
#![feature(strict_provenance)]
#![feature(asm_experimental_arch)]

extern crate alloc;

/// Interprocess Control / IOS Implementation
///
/// This module provides various low level functions to help with opening and using the underlying
/// `IOS` subsystems
pub mod ios;

// Custom Error Implementation
pub mod error;
pub use error::{OgcError, Result};

// Networking Implementation
pub mod network;

// Audio Implementation
pub mod audio;

//MP3Player impl
pub mod mp3player;

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

// ASND Implementation
pub mod asnd;

// AESND Implmentation
pub mod aesnd;

// Input Implementation
pub mod input;

// Light-Weight Process implementation
pub mod lwp;

// LWP Mutex implementation
pub mod mutex;

// Cache operations.
pub mod cache;

// TPL implementation
pub mod tpl;

pub mod time;

#[cfg(feature = "glam_compat")]
pub mod glam_impl;

// FFI
cfg_if::cfg_if! {
    if #[cfg(feature = "ffi")] {
        pub use ogc_sys as ffi;
    } else {
        use ogc_sys as ffi;
    }
}

//#[cfg(feature = "mmio")]
cfg_if::cfg_if! {
    if #[cfg(feature = "mmio")] {
        pub mod mmio;
    } else {
        mod mmio;
    }
}

///Prelude
pub mod prelude {
    // alloc Export
    pub use alloc::boxed::Box;
    pub use alloc::string::{String, ToString};
    pub use alloc::{vec, vec::Vec};

    // Export Services
    pub use crate::asnd::*;
    pub use crate::console::*;
    pub use crate::debug::*;
    pub use crate::gu::*;
    pub use crate::gx::*;
    pub use crate::input::*;
    pub use crate::system::*;

    pub use crate::video::*;
    pub use crate::{print, println};

    // Global Allocator
    use crate::runtime::OGCAllocator;

    #[global_allocator]
    static GLOBAL_ALLOCATOR: OGCAllocator = OGCAllocator;
}
