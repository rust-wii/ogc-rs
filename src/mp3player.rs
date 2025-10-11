use alloc::sync::Arc;
use libc::c_void;

use crate::{asnd::Asnd, ffi};

pub struct MP3Player {
    asnd: Arc<Asnd>,
}

impl MP3Player {
    pub fn new(asnd: Asnd) -> Self {
        unsafe {
            ffi::MP3Player_Init();
        }
        Self {
            asnd: Arc::new(asnd),
        }
    }

    pub fn play_buffer(&mut self, buffer: &[u8]) {
        unsafe {
            ffi::MP3Player_PlayBuffer(
                buffer.as_ptr().cast::<c_void>(),
                buffer.len().try_into().unwrap(),
                None,
            );
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "libogc2")] {
            pub fn is_playing(&self) -> bool {
                unsafe { ffi::MP3Player_IsPlaying() != 0 }
            }
        } else if #[cfg(feature = "libogc")] {
            pub fn is_playing(&self) -> bool {
                unsafe { ffi::MP3Player_IsPlaying() }
            }
        }
    }

    pub fn volume(&mut self, volume: u32) {
        unsafe { ffi::MP3Player_Volume(volume) }
    }

    pub fn stop(&mut self) {
        unsafe { ffi::MP3Player_Stop() }
    }
}
