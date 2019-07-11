//! The ``audio`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the audio functions found in ``audio.h``.

use crate::{raw_to_string, raw_to_strings, OgcError, Primitive, Result, ToPrimitive};
use std::{ffi::c_void, ptr};

/// Represents the audio service.
/// No audio control can be done until an instance of this struct is created.
/// This service can only be created once!
///
/// The service exits when all instances of this struct go out of scope.
pub struct Audio(());

/// Implementation of the audio service.
impl Audio {
    /// Initialization of the audio service.
    pub fn init() -> Audio {
        unsafe {
            // For now this is a mutable null pointer.
            // libogc is fine with this, but this should be changed in the future.
            let r = ogc_sys::AUDIO_Init(ptr::null_mut());

            Audio(())
        }
    }

    /// Get streaming volume on the left channel.
    fn get_volume_left() -> u8 {
        unsafe {
            ogc_sys::AUDIO_GetStreamVolLeft();
        }
    }

    /// Set streaming volume on the left channel.
    fn set_volume_left(volume: u8) {
        unsafe {
            ogc_sys::AUDIO_SetStreamVolLeft(volume);
        }
    }

    /// Get streaming volume on the right channel.
    fn get_volume_right() -> u8 {
        unsafe {
            ogc_sys::AUDIO_GetStreamVolRight();
        }
    }

    /// Set streaming volume on the right channel.
    fn set_volume_right(volume: u8) {
        unsafe {
            ogc_sys::AUDIO_SetStreamVolRight(volume);
        }
    }
}
