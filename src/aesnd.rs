use core::time::Duration;

use crate::ffi;
use alloc::boxed::Box;
use ffi::AESNDPB;
use libc::c_void;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum AudioFormat {
    VoiceMono8 = ffi::VOICE_MONO8,
    VoiceStereo8 = ffi::VOICE_STEREO8,
    VoiceMono16 = ffi::VOICE_MONO16,
    VoiceStereo16 = ffi::VOICE_STEREO16,
    VoiceMono8U = ffi::VOICE_MONO8_UNSIGNED,
    VoiceStereo8U = ffi::VOICE_STEREO8_UNSIGNED,
    VoiceMono16U = ffi::VOICE_MONO16_UNSIGNED,
    VoiceStereo16U = ffi::VOICE_STEREO16_UNSIGNED,
}

pub type VoiceCallback = Option<Box<fn(*mut AESNDPB, u32, *mut c_void)>>;
pub type AudioCallback = Option<Box<fn(*mut c_void, u32, *mut c_void)>>;

pub struct Aesnd;

impl Aesnd {
    pub fn init() -> Self {
        unsafe {
            ffi::AESND_Init();
        }
        Self
    }

    pub fn reset() {
        unsafe {
            ffi::AESND_Reset();
        }
    }

    pub fn set_pause(pause: bool) {
        unsafe {
            ffi::AESND_Pause(pause);
        }
    }

    pub fn pause() {
        Self::set_pause(true);
    }

    pub fn unpause() {
        Self::set_pause(false);
    }

    pub fn get_dsp_process_time() -> Duration {
        Duration::from_nanos(unsafe { ffi::AESND_GetDSPProcessTime().into() })
    }

    pub fn get_dsp_process_usage() -> f32 {
        unsafe { ffi::AESND_GetDSPProcessUsage() }
    }

    pub fn register_audio_callback<F>(callback: Option<unsafe extern "C" fn(*mut c_void, u32, *mut c_void)>) {
        unsafe {
           ffi::AESND_RegisterAudioCallbackWithArg(callback, core::ptr::null_mut());
        }
    }

    pub fn set_voice_stop(play_state: &mut AESNDPB, stop: bool) {
        unsafe {
            ffi::AESND_SetVoiceStop(play_state, stop);
        }
    }

    pub fn set_voice_mute(play_state: &mut AESNDPB, mute: bool) {
        unsafe {
            ffi::AESND_SetVoiceMute(play_state, mute);
        }
    }

    pub fn set_voice_loop(play_state: &mut AESNDPB, loop_: bool) {
        unsafe {
            ffi::AESND_SetVoiceLoop(play_state, loop_);
        }
    }

    pub fn set_voice_format(play_state: &mut AESNDPB, format: AudioFormat) {
        unsafe {
            ffi::AESND_SetVoiceFormat(play_state, format as u32);
        }
    }

    pub fn set_voice_stream(play_state: &mut AESNDPB, stream: bool) {
        unsafe {
            ffi::AESND_SetVoiceStream(play_state, stream);
        }
    }

    pub fn set_voice_frequency(play_state: &mut AESNDPB, frequency: f32) {
        unsafe {
            ffi::AESND_SetVoiceFrequency(play_state, frequency);
        }
    }

    pub fn set_voice_volume(play_state: &mut AESNDPB, volume: (f32, f32)) {
        unsafe {
            ffi::AESND_SetVoiceVolume(
                play_state,
                (volume.0 * 255.0) as u16,
                (volume.1 * 255.0) as u16,
            );
        }
    }

    pub fn set_voice_delay(play_state: &mut AESNDPB, delay: u32) {
        unsafe {
            ffi::AESND_SetVoiceDelay(play_state, delay);
        }
    }

    pub fn set_voice_buffer(play_state: &mut AESNDPB, buffer: &[u8]) {
        //if already aligned just use the buffer.
        if buffer.as_ptr().align_offset(32) == 0 && buffer.len() % 32 == 0 {
            unsafe {
                ffi::AESND_SetVoiceBuffer(
                    play_state,
                    buffer.as_ptr() as *const c_void,
                    buffer.len().try_into().unwrap(),
                );
            }
        } else {
            // othersize copy and allocate a buffer for AESND :)
            let align_buf = crate::utils::alloc_aligned_buffer(buffer);
            assert!(
                align_buf.len() % 32 == 0,
                "Buffer is not padded to 32 bytes"
            );
            unsafe {
                ffi::AESND_SetVoiceBuffer(
                    play_state,
                    align_buf.as_ptr() as *const c_void,
                    align_buf.len().try_into().unwrap(),
                );
            }
        }
    }

    pub fn play_voice(
        play_state: &mut AESNDPB,
        format: AudioFormat,
        buffer: &[u8],
        frequency: f32,
        delay: u32,
        loop_: bool,
    ) {
        if buffer.as_ptr().align_offset(32) == 0 && buffer.len() % 32 == 0 {
            unsafe {
                ffi::AESND_PlayVoice(
                    play_state,
                    format as u32,
                    buffer.as_ptr() as *const c_void,
                    buffer.len().try_into().unwrap(),
                    frequency,
                    delay,
                    loop_,
                );
            }
        } else {
            let align_buf = crate::utils::alloc_aligned_buffer(buffer);
            assert!(
                align_buf.len() % 32 == 0,
                "Buffer is not padded to 32 bytes"
            );
            unsafe {
                ffi::AESND_PlayVoice(
                    play_state,
                    format as u32,
                    align_buf.as_ptr() as *const c_void,
                    align_buf.len().try_into().unwrap(),
                    frequency,
                    delay,
                    loop_,
                );
            }
        }
    }

    pub fn register_voice_callback(
        play_state: &mut AESNDPB,
        callback: Option<unsafe extern "C" fn(*mut AESNDPB, u32, *mut c_void)>,
    ) {
        unsafe {
            ffi::AESND_RegisterVoiceCallbackWithArg(play_state, callback, core::ptr::null_mut());
        }
    }

    pub fn new_playstate() -> AESNDPB {
        unsafe { *ffi::AESND_AllocateVoiceWithArg(None, core::ptr::null_mut()) }
    }
}
