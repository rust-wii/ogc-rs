//! The ``wpad`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the wii controller functions found in ``wpad.h``.

use alloc::boxed::Box;
use core::mem;

/// Represents the wpad service. No wii remotes can be read until an instance of
/// this struct is created. This service can only be created once!
pub struct Wpad;

/// The remote to be read for the `wpad` service.
#[derive(Copy, Clone)]
pub enum Remote {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

impl PartialEq<Remote> for u32 {
    fn eq(&self, other: &Remote) -> bool {
        *self == *other as u32
    }
}

/// The button to be checked for the `wpad` service.
#[derive(Copy, Clone)]
pub enum WpadButton {
    One = 1,
    Two = 2,
    B = 4,
    A = 8,
    Minus = 16,
    Home = 128,
    Left = 256,
    Right = 512,
    Down = 1024,
    Up = 2048,
    Plus = 4096,
    Z = 65536,
    C = 131072,
}

impl PartialEq<WpadButton> for u32 {
    fn eq(&self, other: &WpadButton) -> bool {
        *self == *other as u32
    }
}

impl Wpad {
    /// Initialization of the wpad service.
    pub fn init() -> Self {
        unsafe {
            ogc_sys::WPAD_Init();
            Self
        }
    }

    /// Scan all pads. Must be called every time before checking buttons.
    pub fn scan_pads() -> i32 {
        unsafe { ogc_sys::WPAD_ScanPads() }
    }

    pub fn buttons_down(remote: Remote) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsDown(remote as i32) }
    }

    pub fn buttons_held(remote: Remote) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsHeld(remote as i32) }
    }

    pub fn buttons_up(remote: Remote) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsHeld(remote as i32) }
    }

    /// Registers a battery dead callback function.
    pub fn set_battery_dead_callback<F>(callback: Box<F>)
    where
        F: Fn(i32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(i32) = mem::transmute(ptr);
            ogc_sys::WPAD_SetBatteryDeadCallback(Some(code));
        }
    }

    /// Registers a power button callback function.
    pub fn set_power_button_callback<F>(callback: Box<F>)
    where
        F: Fn(i32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(i32) = mem::transmute(ptr);
            ogc_sys::WPAD_SetPowerButtonCallback(Some(code));
        }
    }

    fn accel() {}
    fn battery_level() {}
    fn control_speaker() {}
    fn data() {}
    fn disconnect() {}
    fn dropped_events() {}
    fn encode_data() {}
    fn expansion() {}
    fn flush() {}
    fn g_force() {}
    fn get_status() {}
    fn ir() {}
    fn is_speaker_enabled() {}
    fn orientation() {}
    fn probe() {}
    fn read_event() {}
    fn read_pending() {}
    fn rumble() {}
    fn send_stream_data() {}
    fn set_data_format() {}
    fn set_event_bufs() {}
    fn set_idle_thresholds() {}
    fn set_idle_timeout() {}
    fn set_motion_plus() {}
    fn set_vres() {}
    fn shutdown() {}
}
