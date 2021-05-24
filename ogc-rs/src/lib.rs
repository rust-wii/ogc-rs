//! # ogc-rs
//!
//! ``ogc-rs`` is a safe, idiomatic wrapper around ``ogc-sys``.

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

// Gx Implementation
pub mod gx;

// Gu Implementation
pub mod gu;

// Tpl Implementation
pub mod tpl;

// Pad Implementation
pub mod pad;

// WPad Implementation
mod wpad;

// Console Implementation
pub mod console;

// System Implementation
pub mod system;

// Video Implementation
pub mod video;

// Debugging Functions
pub mod debug;

// Utility Functions
pub mod utils;
pub use utils::*;

// Runtime Functions
pub mod runtime;

// Prelude
pub mod prelude {
    // alloc Export
    pub use alloc::boxed::Box;
    pub use alloc::string::{String, ToString};
    pub use alloc::{vec, vec::Vec};
    pub use ogc_sys;

    // Export Services
    pub use crate::console::*;
    pub use crate::debug::*;
    pub use crate::system::*;
    pub use crate::video::*;
    pub use crate::pad::*;
    pub use crate::wpad::*;
    pub use crate::gx::{*, constants::*};
    pub use crate::gu::*;
    pub use crate::tpl::*;
    pub use crate::{print, println};

    // Global Allocator
    use crate::runtime::OGCAllocator;

    #[global_allocator]
    static GLOBAL_ALLOCATOR: OGCAllocator = OGCAllocator;
}
