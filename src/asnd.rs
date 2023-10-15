//! The ``asnd`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the audio functions found in ``asndlib.h``.

use crate::{ffi, OgcError, Result};
use alloc::format;
use core::time::Duration;

macro_rules! if_not {
    ($valid:ident => $error_output:expr, $var:ident $(,)*) => {
        if $var == ffi::$valid as _ {
            Ok(())
        } else {
            Err(OgcError::Audio(format!($error_output, $var)))
        }
    };
}

/// Voice Options Callback Type
pub type VoiceOptionsCallback = Option<unsafe extern "C" fn(i32)>;

/// Options to be passed when creating a new voice.
///
/// # Examples
///
/// Create `VoiceOptions` with voice slot 2 and format Mono16Bit:
///
/// ```rust
/// let options = VoiceOptions::new().voice(2).format(VoiceFormat::Mono16BitBe);
/// ```
pub struct VoiceOptions {
    voice: u32,
    format: VoiceFormat,
    pitch: u32,
    delay: u32,
    volume_left: u8,
    volume_right: u8,
    callback: VoiceOptionsCallback,
}

impl Default for VoiceOptions {
    fn default() -> Self {
        VoiceOptions::new()
    }
}

impl VoiceOptions {
    /// Create this struct with sensible default values.
    pub fn new() -> Self {
        Self {
            voice: 0,
            format: VoiceFormat::Stereo16BitBe,
            pitch: 48000,
            delay: 0,
            volume_left: 255,
            volume_right: 255,
            callback: None,
        }
    }

    /// Voice slot to use for this sound. Valid values are `0..16` non-inclusive.
    #[must_use]
    pub fn voice(mut self, voice: u32) -> Self {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        self.voice = voice;
        self
    }

    /// Format to use for this sound.
    #[must_use]
    pub fn format(mut self, format: VoiceFormat) -> Self {
        self.format = format;
        self
    }

    /// Frequency to use, in Hz.
    #[must_use]
    pub fn pitch(mut self, pitch: u32) -> Self {
        self.pitch = pitch;
        self
    }

    /// Delay to wait before playing, in milliseconds.
    #[must_use]
    pub fn delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    /// Voice volume of the left channel.
    #[must_use]
    pub fn volume_left(mut self, volume_left: u8) -> Self {
        self.volume_left = volume_left;
        self
    }

    /// Voice volume of the right channel.
    #[must_use]
    pub fn volume_right(mut self, volume_right: u8) -> Self {
        self.volume_right = volume_right;
        self
    }

    /// Optional callback function to use.
    #[must_use]
    pub fn callback(mut self, callback: Option<unsafe extern "C" fn(i32)>) -> Self {
        self.callback = callback;
        self
    }
}

/// Source voice format.
#[repr(i32)]
pub enum VoiceFormat {
    Mono8Bit = ffi::VOICE_MONO_8BIT as _,
    Mono8BitU = ffi::VOICE_MONO_8BIT_U as _,
    Mono16BitLe = ffi::VOICE_MONO_16BIT_LE as _,
    Mono16BitBe = ffi::VOICE_MONO_16BIT_BE as _,
    Stereo8Bit = ffi::VOICE_STEREO_8BIT as _,
    Stereo8BitU = ffi::VOICE_STEREO_8BIT_U as _,
    Stereo16BitBe = ffi::VOICE_STEREO_16BIT_BE as _,
    Stereo16BitLe = ffi::VOICE_STEREO_16BIT_LE as _,
}

/// Represents the asnd service.
/// This service can only be created once!
/// If you use `Asnd::init()`, you cannot do `Audio::init()`.
/// Only one of them can be used at a time.
pub struct Asnd;

/// Implementation of the asnd service.
impl Asnd {
    /// Initializes the asnd lib and fixes the hardware sample rate to 48000hz.
    pub fn init() -> Self {
        unsafe {
            ffi::ASND_Init();
        }

        Self
    }

    /// De-initializes the asnd lib. This is also called when `Asnd` gets dropped.
    pub fn end() {
        unsafe {
            ffi::ASND_End();
        }
    }

    /// Pauses if true and resumes if false.
    pub fn pause(should_pause: bool) {
        unsafe {
            ffi::ASND_Pause(should_pause as i32);
        }
    }

    /// Returns true if paused, false if not paused.
    pub fn is_paused() -> bool {
        unsafe { ffi::ASND_Is_Paused() > 0 }
    }

    /// Returns the global time in milliseconds. Time is updated from the IRQ.
    pub fn get_time() -> u32 {
        unsafe { ffi::ASND_GetTime() }
    }

    /// Returns the global sample counter. Can be used to implement timers with high precision.
    pub fn get_sample_counter() -> u32 {
        unsafe { ffi::ASND_GetSampleCounter() }
    }

    /// Returns the samples sent from the IRQ in one tick.
    pub fn get_samples_per_tick() -> u32 {
        unsafe { ffi::ASND_GetSamplesPerTick() }
    }

    /// Sets the global time, in milliseconds.
    pub fn set_time(time: u32) {
        unsafe {
            ffi::ASND_SetTime(time);
        }
    }

    /// Sets a global callback for general purposes. It is called by the IRQ.
    pub fn set_callback<F>(callback: Option<unsafe extern "C" fn()>) {
        unsafe {
            ffi::ASND_SetCallback(callback);
        }
    }

    /// Returs the current audio rate. Default is 48000hz.
    pub fn get_audio_rate() -> i32 {
        unsafe { ffi::ASND_GetAudioRate() }
    }

    /// Sets a PCM voice to play. This function stops one previous voice. Use
    /// `Asnd::status_voice()` to test status. The voices are played in 16-bit stereo,
    /// regardless of source format. The buffer MUST be aligned and padded to 32 bytes.
    pub fn set_voice(options: VoiceOptions, sound_buffer: &mut [u8]) -> Result<()> {
        Self::validate_buffer(sound_buffer);

        let err = unsafe {
            ffi::ASND_SetVoice(
                options.voice as i32,
                options.format as i32,
                options.pitch as i32,
                options.delay as i32,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as i32,
                options.volume_left as i32,
                options.volume_right as i32,
                options.callback,
            )
        };

        if_not!(SND_OK => "Asnd::set_voice() failed with error {}!", err)
    }

    /// Sets a PCM voice to play infinitely. See `Asnd::set_voice()` as it is largely identical.
    /// The buffer MUST be aligned and padded to 32 bytes.
    pub fn set_infinite_voice(options: VoiceOptions, sound_buffer: &mut [u8]) -> Result<()> {
        Self::validate_buffer(sound_buffer);

        let err = unsafe {
            ffi::ASND_SetInfiniteVoice(
                options.voice as i32,
                options.format as i32,
                options.pitch as i32,
                options.delay as i32,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as i32,
                options.volume_left as i32,
                options.volume_right as i32,
            )
        };

        if_not!(SND_OK => "Asnd::set_infinite_voice() failed with error {}", err)
    }

    /// Adds a PCM voice to play from the second buffer. Sound buffer must be 32-byte
    /// aligned and have same sample format as first buffer. This must only be called after
    /// `Asnd::set_voice()`, which must return `Ok()`.
    /// The buffer MUST be aligned and padded to 32 bytes.
    fn add_voice(voice: u32, sound_buffer: &mut [u8]) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        Self::validate_buffer(sound_buffer);

        let err = unsafe {
            ffi::ASND_AddVoice(
                voice as i32,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as i32,
            )
        };

        if_not!(SND_OK => "Asnd::add_voice() failed with error {}", err)
    }

    /// Stops the selected voice. If the voice is used in song mode, you need to
    /// assign the samples with `Asnd::set_song_sample_voice()`.
    pub fn stop_voice(voice: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ffi::ASND_StopVoice(voice as i32) };
        if_not!(SND_OK => "Asnd::stop_voice() failed with error {}", err)
    }

    /// Pauses the selected voice. Can also be used to resume voice.
    pub fn pause_voice(voice: u32, pause: bool) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ffi::ASND_PauseVoice(voice as i32, pause as i32) };
        if_not!(SND_OK => "Asnd::pause_voice() failed with error {}", err)
    }

    /// Returns the state of the selected voice.
    pub fn status_voice(voice: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ffi::ASND_StatusVoice(voice as i32) };
        if_not!(SND_WORKING => "Asnd::status_voice() failed with error {}", err)
    }

    /// Returns the first unused voice. Fails if no voices are available.
    pub fn get_first_unused_voice() -> Result<u32> {
        let err = unsafe { ffi::ASND_GetFirstUnusedVoice() };
        match err {
            x if x < 16 => Ok(x as u32),
            _ => Err(OgcError::Audio(format!(
                "Asnd::get_first_unused_voice() failed with error {}",
                err
            ))),
        }
    }

    /// Changes the voice-pitch in real time. This function can be used to
    /// create audio effects such as Doppler effect simulation.
    pub fn change_pitch_voice(voice: u32, pitch: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ffi::ASND_ChangePitchVoice(voice as i32, pitch as i32) };
        if_not!(SND_OK => "Asnd::change_pitch_voice() failed with error {}", err)
    }

    /// Changes the voice volume in real time. This function can be used to create
    /// audio effects like distance attenuation.
    pub fn change_volume_voice(voice: u32, volume_left: u8, volume_right: u8) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe {
            ffi::ASND_ChangeVolumeVoice(voice as i32, volume_left as i32, volume_right as i32)
        };
        if_not!(SND_OK => "Asnd::change_volume_voice() failed with error {}", err)
    }

    /// Returns the voice tick counter. This value represents the number of ticks
    /// since this voice started to play, sans delay time. If the lib is initialized with
    /// `INIT_RATE=48000`, a return value of 24000 is equal to 0.5 seconds.
    pub fn get_tick_counter_voice(voice: u32) -> u32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ffi::ASND_GetTickCounterVoice(voice as i32) }
    }

    /// Returns the voice playback time. This value represents the time in milliseconds
    /// since this voice started playing.
    pub fn get_timer_voice(voice: u32) -> u32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ffi::ASND_GetTimerVoice(voice as i32) }
    }

    /// Tests if a pointer is in use by a voice as a buffer.
    /// This must be the same pointer sent to `Asnd::add_voice()` or `Asnd::set_voice()`.
    /// Returns 0 if the pointer is unused.
    /// Returns 1 if the pointer is used as a buffer.
    /// Returns `ogc_sys::SND_INVALID` if invalid.
    pub fn test_pointer<T>(voice: u32, pointer: *mut T) -> i32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ffi::ASND_TestPointer(voice as i32, pointer as *mut _) }
    }

    /// Tests to determine if the voice is ready to receive a new buffer sample
    /// with `Asnd::add_voice()`. Returns true if voice is ready.
    pub fn test_voice_buffer_ready(voice: u32) -> bool {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ffi::ASND_TestVoiceBufferReady(voice as i32) > 0 }
    }

    /// Returns the DSP usage, in percent `(0..=100)`.
    pub fn get_dsp_percent_use() -> u32 {
        unsafe { ffi::ASND_GetDSP_PercentUse() }
    }

    /// Returns DSP process time, in nano seconds.
    pub fn get_dsp_process_time() -> Duration {
        unsafe { Duration::from_nanos(ffi::ASND_GetDSP_ProcessTime().into()) }
    }

    fn validate_buffer(sound_buffer: &mut [u8]) {
        assert_eq!(
            0,
            sound_buffer.as_ptr().align_offset(32),
            "Data is not aligned correctly."
        );
        assert_eq!(
            0,
            sound_buffer.len() % 32,
            "Data length is not a multiple of 32."
        );
    }
}

impl Drop for Asnd {
    fn drop(&mut self) {
        Self::end();
    }
}
