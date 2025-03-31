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
#![feature(asm_experimental_arch)]

extern crate alloc;

pub mod pad;

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

pub mod sync;

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

<<<<<<< Updated upstream
mod interrupts {
    use bit_field::BitField;

    fn get_msr() -> u32 {
        let msr: u32;
        unsafe { core::arch::asm!("mfmsr {}", out(reg) msr) };
        msr
    }

    fn set_msr(msr: u32) {
        unsafe { core::arch::asm!("mtmsr {}", in(reg) msr) };
    }

    pub fn disable() -> u32 {
        let restore_state = get_msr();
        let mut msr = restore_state;
        // Set External Interrupts false
        msr.set_bit(15, false);
        set_msr(msr);
        return restore_state;
    }

    pub fn enable(restore_state: u32) {
        set_msr(restore_state);
    }
}

#[cfg(feature = "critical-section-wii")]
mod sync {
    use bit_field::BitField;

    struct WiiCriticalSection;

    critical_section::set_impl!(WiiCriticalSection);

    fn get_msr() -> u32 {
        let msr: u32;
        unsafe { core::arch::asm!("mfmsr {}", out(reg) msr) };
        msr
    }

    fn set_msr(msr: u32) {
        unsafe { core::arch::asm!("mtmsr {}", in(reg) msr) };
    }

    unsafe impl critical_section::Impl for WiiCriticalSection {
        unsafe fn acquire() -> critical_section::RawRestoreState {
            let restore_state = get_msr();
            let mut msr = restore_state;
            // Set External Interrupts false
            msr.set_bit(15, false);
            set_msr(msr);
            restore_state
        }

        unsafe fn release(restore_state: critical_section::RawRestoreState) {
            set_msr(restore_state);
        }
    }
}
=======
pub mod arch;
>>>>>>> Stashed changes

///Prelude
pub mod prelude {
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

mod test {

    struct Func<Args, Ret>(fn(Args) -> Ret);

    impl<Args, Ret> Func<Args, Ret> {
        fn cast<Args2, Ret2>(self) -> Func<Args2, Ret2> {
            unsafe { core::mem::transmute(self) }
        }

        unsafe fn call(&self, args: Args) -> Ret {
            (self.0)(args)
        }
    }
}
