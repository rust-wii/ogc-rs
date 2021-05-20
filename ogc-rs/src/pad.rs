//! The ``pad`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the pad functions.

// use alloc::boxed::Box;
// use core::{mem, ptr};
use enum_primitive::*;

/// ```rust
/// let pad = Pad::init();
/// let buttons_down = pad.buttons_down(Controller::One);
/// if buttons_down & Button::A {
///     return 0;
/// }
/// ```
pub struct Pad;

enum_primitive! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Controller {
        One = 0,
        Two = 1,
        Three = 2,
        Four = 3,
    }
}

enum_primitive! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Button {
        Left = 1,
        Right = 2,
        Down = 4,
        Up = 8,
        Z = 16,
        R = 32,
        L = 64,
        A = 256,
        B = 512,
        X = 1024,
        Y = 2048,
        Start = 4096,
    }
}

impl Pad {
    pub fn init() -> Self {
        unsafe {
            ogc_sys::PAD_Init();
            ogc_sys::PAD_ScanPads();
            Pad
        }
    }

    pub fn buttons_down(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsDown(controller as i32)
        }
    }

    pub fn buttons_held(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsHeld(controller as i32)
        }
    }

    pub fn buttons_up(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsUp(controller as i32)
        }
    }
}

// pub struct Audio;

// enum_primitive! {
//     /// The play state of the ``audio`` service.
//     #[derive(Debug, Eq, PartialEq)]
//     pub enum PlayState {
//         Started = 1,
//         Stopped = 0,
//     }
// }

// enum_primitive! {
//     /// The sample rate of the ``audio`` service.
//     #[derive(Debug, Eq, PartialEq)]
//     pub enum SampleRate {
//         FortyEightKhz = 1,
//         ThirtySixKhz = 0,
//     }
// }

// /// Implementation of the audio service.
// impl Audio {
//     /// Initialization of the audio service.
//     pub fn init() -> Self {
//         unsafe {
//             // For now this is a mutable null pointer.
//             // libogc is fine with this, but this should be changed in the future.
//             ogc_sys::AUDIO_Init(ptr::null_mut());

//             Self
//         }
//     }

//     /// Initialize an audio DMA transfer.
//     fn init_dma(data: &[u8]) {
//         unsafe {
//             // libogc has strict restrictions on data alignment and length.
//             assert_eq!(
//                 32,
//                 mem::align_of_val(data),
//                 "Data is not aligned correctly."
//             );
//             assert_eq!(0, data.len() % 32, "Data length is not a multiple of 32.");

//             ogc_sys::AUDIO_InitDMA(data.as_ptr() as u32, data.len() as u32);
//         }
//     }

//     /// Start the audio DMA operation.
//     ///
//     /// Starts to transfer the data from main memory to the audio interface through DMA.
//     /// This call should follow the call to ``init_dma`` which is used to initialize DMA transfers.
//     fn start_dma() {
//         unsafe {
//             ogc_sys::AUDIO_StartDMA();
//         }
//     }

//     /// Stop the previously started audio DMA operation.
//     fn stop_dma() {
//         unsafe {
//             ogc_sys::AUDIO_StopDMA();
//         }
//     }

//     /// Register a user callback function for the ``audio`` streaming interface.
//     fn register_stream_callback<F>(callback: Box<F>)
//     where
//         F: Fn(u32) -> (),
//     {
//         // TODO: Check if this implementation can be changed.
//         let ptr = Box::into_raw(callback);

//         unsafe {
//             let code: extern "C" fn(smp_cnt: u32) = mem::transmute(ptr);
//             // TODO: Do something with the returned callback.
//             let _ = ogc_sys::AUDIO_RegisterStreamCallback(Some(code));
//         }
//     }

//     /// Register a user callback function for the audio DMA interface.
//     ///
//     /// This callback will be called whenever the audio DMA requests new data.
//     /// Internally the DMA buffers are double buffered.
//     fn register_dma_callback<F>(callback: Box<F>)
//     where
//         F: Fn() -> (),
//     {
//         // TODO: Check if this implementation can be changed.
//         let ptr = Box::into_raw(callback);

//         unsafe {
//             let code: extern "C" fn() = mem::transmute(ptr);
//             // TODO: Do something with the returned callback.
//             let _ = ogc_sys::AUDIO_RegisterDMACallback(Some(code));
//         }
//     }

//     /// Get the count of bytes, left to play, from the audio DMA interface.
//     fn get_dma_bytes_left() -> u32 {
//         unsafe { ogc_sys::AUDIO_GetDMABytesLeft() }
//     }

//     /// Get the audio DMA flag.
//     fn get_dma_enable_flag() -> u16 {
//         unsafe { ogc_sys::AUDIO_GetDMAEnableFlag() }
//     }

//     /// Get the DMA transfer length configured in the audio DMA interface.
//     fn get_dma_length() -> u32 {
//         unsafe { ogc_sys::AUDIO_GetDMALength() }
//     }

//     /// Get the main memory address for the DMA operation.
//     fn get_dma_address() -> u32 {
//         unsafe { ogc_sys::AUDIO_GetDMAStartAddr() }
//     }

//     /// Reset the stream sample count register.
//     fn reset_sample_count() {
//         unsafe {
//             ogc_sys::AUDIO_ResetStreamSampleCnt();
//         }
//     }

//     /// Set the sample count for the stream trigger.
//     fn set_trigger_count(count: u32) {
//         unsafe {
//             ogc_sys::AUDIO_SetStreamTrigger(count);
//         }
//     }

//     /// Get streaming sample rate.
//     fn get_samplerate() -> SampleRate {
//         unsafe {
//             let r = ogc_sys::AUDIO_GetStreamSampleRate();
//             SampleRate::from_u32(r).unwrap()
//         }
//     }

//     /// Get the sampling rate for the DSP interface.
//     fn get_dsp_samplerate() -> SampleRate {
//         let r = unsafe { ogc_sys::AUDIO_GetDSPSampleRate() };
//         SampleRate::from_u32(r).unwrap()
//     }

//     /// Set the sample rate for the streaming audio interface.
//     fn set_samplerate(samplerate: SampleRate) {
//         unsafe {
//             ogc_sys::AUDIO_SetStreamSampleRate(samplerate.to_u32().unwrap());
//         }
//     }

//     /// Set the sampling rate for the DSP interface.
//     fn set_dsp_samplerate(samplerate: SampleRate) {
//         unsafe {
//             ogc_sys::AUDIO_SetDSPSampleRate(samplerate.to_u8().unwrap());
//         }
//     }

//     /// Get the play state from the streaming audio interface.
//     fn get_playstate() -> PlayState {
//         let r = unsafe { ogc_sys::AUDIO_GetStreamPlayState() };
//         PlayState::from_u32(r).unwrap()
//     }

//     /// Set the play state for the streaming audio interface.
//     fn set_playstate(playstate: PlayState) {
//         unsafe {
//             ogc_sys::AUDIO_SetStreamPlayState(playstate.to_u32().unwrap());
//         }
//     }

//     /// Get streaming volume on the left channel.
//     fn get_volume_left() -> u8 {
//         unsafe { ogc_sys::AUDIO_GetStreamVolLeft() }
//     }

//     /// Set streaming volume on the left channel.
//     fn set_volume_left(volume: u8) {
//         unsafe { ogc_sys::AUDIO_SetStreamVolLeft(volume) }
//     }

//     /// Get streaming volume on the right channel.
//     fn get_volume_right() -> u8 {
//         unsafe { ogc_sys::AUDIO_GetStreamVolRight() }
//     }

//     /// Set streaming volume on the right channel.
//     fn set_volume_right(volume: u8) {
//         unsafe { ogc_sys::AUDIO_SetStreamVolRight(volume) }
//     }
// }
