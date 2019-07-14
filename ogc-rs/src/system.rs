//! The ``system`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the OS functions found in ``system.h``.

use std::{mem, ptr};

/// Represents the system service.
/// The initialization of this service is done in the crt0 startup code.
pub struct System(());

/// Implementation of the system service.
impl System {
    /// Get system time.
    pub fn system_time() -> u64 {
        unsafe {
            ogc_sys::SYS_Time()
        }
    }
}