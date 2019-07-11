//! The ``audio`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the audio functions found in ``audio.h``.

use crate::{FromPrimitive, Primitive, ToPrimitive};
use std::{mem, ptr};

/// Represents the audio service.
/// No audio control can be done until an instance of this struct is created.
/// This service can only be created once!
///
/// The service exits when all instances of this struct go out of scope.
pub struct Audio(());

/// The play state of the ``audio`` service.
#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum PlayState {
    Started = 1,
    Stopped = 0,
}

/// The sample rate of the ``audio`` service.
#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum SampleRate {
    FortyEightKhz = 1,
    ThirtySixKhz = 0,
}

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

    /// Register a user callback function for the ``audio`` streaming interface.
    fn register_stream_callback<F>(callback: Box<F>)
    where
        F: Fn(u32) -> (),
    {
        unsafe {
            // TODO: Check if this implementation can be changed.
            let ptr = Box::into_raw(callback);
            let code: extern "C" fn(smp_cnt: u32) = mem::transmute(ptr);
            // TODO: Do something with the returned callback.
            let _ = ogc_sys::AUDIO_RegisterStreamCallback(Some(code));
        }
    }

    /// Reset the stream sample count register.
    fn reset_sample_count() {
        unsafe {
            ogc_sys::AUDIO_ResetStreamSampleCnt();
        }
    }

    /// Set the sample count for the stream trigger.
    fn set_trigger_count(count: u32) {
        unsafe {
            ogc_sys::AUDIO_SetStreamTrigger(count);
        }
    }

    /// Get streaming sample rate.
    fn get_samplerate() -> SampleRate {
        unsafe {
            let r = ogc_sys::AUDIO_GetStreamSampleRate();
            SampleRate::from_u32(r).unwrap()
        }
    }

    /// Get the sampling rate for the DSP interface.
    fn get_dsp_samplerate() -> SampleRate {
        unsafe {
            let r = ogc_sys::AUDIO_GetDSPSampleRate();
            SampleRate::from_u32(r).unwrap()
        }
    }

    /// Set the sample rate for the streaming audio interface.
    fn set_samplerate(samplerate: SampleRate) {
        unsafe {
            ogc_sys::AUDIO_SetStreamSampleRate(samplerate.to_u32().unwrap());
        }
    }

    /// Set the sampling rate for the DSP interface.
    fn set_dsp_samplerate(samplerate: SampleRate) {
        unsafe {
            ogc_sys::AUDIO_SetDSPSampleRate(samplerate.to_u8().unwrap());
        }
    }

    /// Get the play state from the streaming audio interface.
    fn get_playstate() -> PlayState {
        unsafe {
            let r = ogc_sys::AUDIO_GetStreamPlayState();
            PlayState::from_u32(r).unwrap()
        }
    }

    /// Set the play state for the streaming audio interface.
    fn set_playstate(playstate: PlayState) {
        unsafe {
            ogc_sys::AUDIO_SetStreamPlayState(playstate.to_u32().unwrap());
        }
    }

    /// Get streaming volume on the left channel.
    fn get_volume_left() -> u8 {
        unsafe { ogc_sys::AUDIO_GetStreamVolLeft() }
    }

    /// Set streaming volume on the left channel.
    fn set_volume_left(volume: u8) {
        unsafe { ogc_sys::AUDIO_SetStreamVolLeft(volume) }
    }

    /// Get streaming volume on the right channel.
    fn get_volume_right() -> u8 {
        unsafe { ogc_sys::AUDIO_GetStreamVolRight() }
    }

    /// Set streaming volume on the right channel.
    fn set_volume_right(volume: u8) {
        unsafe { ogc_sys::AUDIO_SetStreamVolRight(volume) }
    }
}
