//! The ``pad`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the gamecube controller functions found in ``pad.h``.

use crate::{video::RenderConfig, OgcError, Result};
use alloc::boxed::Box;
use core::{ffi::c_void, mem, time::Duration};
use num_enum::IntoPrimitive;

/// Represents the system service.
/// The initialization of this service is done in the crt0 startup code.
pub struct System;

/// OS Reset Types
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ResetTypes {
    Restart = 0,
    HotReset = 1,
    Shutdown = 2,
    ReturnToMenu = 3,
    PowerOff = 4,
    PowerOffStandby = 5,
    PowerOffIdle = 6,
}

/// OS Memory Protection Modes
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MemoryProtectModes {
    ProtectNone = 0,
    ProtectRead = 1,
    ProtectWrite = 2,
    ProtectReadWrite = 1 | 2,
}

/// OS Memory Protection Channels
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MemoryProtectChannels {
    ChannelZero = 0,
    ChannelOne = 1,
    ChannelTwo = 2,
    ChannelThree = 3,
    All = 4,
}

/// System Font Header Structure
pub struct FontHeader {
    pub font_type: u16,
    pub first_char: u16,
    pub last_char: u16,
    pub inval_char: u16,
    pub asc: u16,
    pub desc: u16,
    pub width: u16,
    pub leading: u16,
    pub cell_dimensions: (u16, u16),
    pub sheet_size: u32,
    pub sheet_format: u16,
    pub sheet_colrow: (u16, u16),
    pub sheet_dimensions: (u16, u16),
    pub width_table: u16,
    pub sheet_image: u32,
    pub sheet_fullsize: u32,
}

impl Into<*mut ogc_sys::sys_fontheader> for &mut FontHeader {
    fn into(self) -> *mut ogc_sys::sys_fontheader {
        Box::into_raw(Box::new(ogc_sys::sys_fontheader {
            font_type: self.font_type,
            first_char: self.first_char,
            last_char: self.last_char,
            inval_char: self.inval_char,
            asc: self.asc,
            desc: self.desc,
            width: self.width,
            leading: self.leading,
            cell_width: self.cell_dimensions.0,
            cell_height: self.cell_dimensions.1,
            sheet_size: self.sheet_size,
            sheet_format: self.sheet_format,
            sheet_column: self.sheet_colrow.0,
            sheet_row: self.sheet_colrow.1,
            sheet_width: self.sheet_dimensions.0,
            sheet_height: self.sheet_dimensions.1,
            width_table: self.width_table,
            sheet_image: self.sheet_image,
            sheet_fullsize: self.sheet_fullsize,
            c0: 0,
            c1: 0,
            c2: 0,
            c3: 0,
        }))
    }
}

/// Implementation of the system service.
impl System {
    /// Allocate cacheline aligned memory for the external
    /// framebuffer based on the rendermode object.
    ///
    /// This function returns a pointer to the framebuffer's startaddress which
    /// is aligned to a 32 byte boundary.
    pub fn allocate_framebuffer(render_mode: RenderConfig) -> *mut c_void {
        unsafe { ogc_sys::SYS_AllocateFramebuffer(render_mode.into()) }
    }

    /// Create and initialize sysalarm structure.
    pub fn create_alarm(context: &mut u32) -> Result<()> {
        let r = unsafe { ogc_sys::SYS_CreateAlarm(context) };

        if r < 0 {
            Err(OgcError::System("system failed to create alarm".into()))
        } else {
            Ok(())
        }
    }

    /// Cancel the alarm, but do not remove from the list of contexts.
    pub fn cancel_alarm(context: u32) -> Result<()> {
        let r = unsafe { ogc_sys::SYS_CancelAlarm(context) };

        if r < 0 {
            Err(OgcError::System("system failed to cancel alarm".into()))
        } else {
            Ok(())
        }
    }

    /// Remove the given alarm context from the list of contexts and destroy it.
    pub fn remove_alarm(context: u32) -> Result<()> {
        let r = unsafe { ogc_sys::SYS_RemoveAlarm(context) };

        if r < 0 {
            Err(OgcError::System("system failed to remove alarm".into()))
        } else {
            Ok(())
        }
    }

    /// Set the alarm parameters for a one-shot alarm, add to the list of alarms and start.
    pub fn set_alarm<F>(context: u32, fire_time: Duration, callback: Box<F>) -> Result<()>
    where
        F: Fn(u32, *mut c_void) -> (),
    {
        unsafe {
            // Convert Duration to timespec
            let timespec: *const ogc_sys::timespec = &ogc_sys::timespec {
                tv_sec: fire_time.as_secs() as i64,
                tv_nsec: fire_time.as_nanos() as i32,
            };

            // TODO: Check if this implementation can be changed.
            let ptr = Box::into_raw(callback);
            let code: extern "C" fn(alarm: u32, cb_arg: *mut c_void) = mem::transmute(ptr);
            let r = ogc_sys::SYS_SetAlarm(context, timespec, Some(code), 0 as *mut c_void);

            if r < 0 {
                Err(OgcError::System("system failed to set alarm".into()))
            } else {
                Ok(())
            }
        }
    }

    /// Set the alarm parameters for a periodic alarm, add to the list of alarms and start.
    /// The alarm and interval persists as long as SYS_CancelAlarm() isn't called.
    pub fn set_periodic_alarm<F>(
        context: u32,
        time_start: Duration,
        time_period: Duration,
        callback: Box<F>,
    ) -> Result<()>
    where
        F: Fn(u32, *mut c_void) -> (),
    {
        unsafe {
            // Convert Duration to timespec
            let timespec_start: *const ogc_sys::timespec = &ogc_sys::timespec {
                tv_sec: time_start.as_secs() as i64,
                tv_nsec: time_start.as_nanos() as i32,
            };

            let timespec_period: *const ogc_sys::timespec = &ogc_sys::timespec {
                tv_sec: time_period.as_secs() as i64,
                tv_nsec: time_period.as_nanos() as i32,
            };

            // TODO: Check if this implementation can be changed.
            let ptr = Box::into_raw(callback);
            let code: extern "C" fn(alarm: u32, cb_arg: *mut c_void) = mem::transmute(ptr);
            let r = ogc_sys::SYS_SetPeriodicAlarm(
                context,
                timespec_start,
                timespec_period,
                Some(code),
                0 as *mut c_void,
            );

            if r < 0 {
                Err(OgcError::System(
                    "system failed to set periodic alarm".into(),
                ))
            } else {
                Ok(())
            }
        }
    }

    /// Init Font
    pub fn init_font(font_header: &mut FontHeader) {
        unsafe {
            let _ = ogc_sys::SYS_InitFont(font_header.into());
        }
    }

    /// Get Font Texel
    pub fn get_font_texel(c: i32, image: *mut c_void, position: i32, stride: i32, width: &mut i32) {
        unsafe {
            ogc_sys::SYS_GetFontTexel(c, image, position, stride, width);
        }
    }

    /// Get Font Texture
    pub fn get_font_texture(
        c: i32,
        image: *mut *mut c_void,
        xpos: &mut i32,
        ypos: &mut i32,
        width: &mut i32,
    ) {
        unsafe {
            ogc_sys::SYS_GetFontTexture(c, image, xpos, ypos, width);
        }
    }

    /// Get Font Encoding
    pub fn get_font_encoding() -> u32 {
        unsafe { ogc_sys::SYS_GetFontEncoding() }
    }

    /// Get Arena 1 Lo
    pub fn get_arena_1_lo() -> *mut c_void {
        unsafe { ogc_sys::SYS_GetArena1Lo() }
    }

    /// Set Arena 1 Lo
    pub fn set_arena_1_lo(new_lo: *mut c_void) {
        unsafe { ogc_sys::SYS_SetArena1Lo(new_lo) }
    }

    /// Get Arena 1 Hi
    pub fn get_arena_1_hi() -> *mut c_void {
        unsafe { ogc_sys::SYS_GetArena1Hi() }
    }

    /// Set Arena 1 Hi
    pub fn set_arena_1_hi(new_hi: *mut c_void) {
        unsafe { ogc_sys::SYS_SetArena1Hi(new_hi) }
    }

    /// Get Arena 1 Size
    pub fn get_arena_1_size() -> u32 {
        unsafe { ogc_sys::SYS_GetArena1Size() }
    }

    /// Get Arena 2 Lo
    pub fn get_arena_2_lo() -> *mut c_void {
        unsafe { ogc_sys::SYS_GetArena2Lo() }
    }

    /// Set Arena 2 Lo
    pub fn set_arena_2_lo(new_lo: *mut c_void) {
        unsafe { ogc_sys::SYS_SetArena2Lo(new_lo) }
    }

    /// Get Arena 2 Hi
    pub fn get_arena_2_hi() -> *mut c_void {
        unsafe { ogc_sys::SYS_GetArena2Hi() }
    }

    /// Set Arena 2 Hi
    pub fn set_arena_2_hi(new_hi: *mut c_void) {
        unsafe { ogc_sys::SYS_SetArena2Hi(new_hi) }
    }

    /// Get Arena 2 Size
    pub fn get_arena_2_size() -> u32 {
        unsafe { ogc_sys::SYS_GetArena2Size() }
    }

    /// Set Wireless ID
    pub fn set_wireless_id(channel: u32, id: u16) {
        unsafe {
            ogc_sys::SYS_SetWirelessID(channel, id);
        }
    }

    /// Get Wireless ID
    pub fn get_wireless_id(channel: u32) -> u16 {
        unsafe { ogc_sys::SYS_GetWirelessID(channel) }
    }

    /// Start PMC
    pub fn start_pmc(mcr0: u32, mcr1: u32) {
        unsafe {
            ogc_sys::SYS_StartPMC(mcr0, mcr1);
        }
    }

    /// Dump PMC
    pub fn dump_pmc() {
        unsafe {
            ogc_sys::SYS_DumpPMC();
        }
    }

    /// Stop PMC
    pub fn stop_pmc() {
        unsafe {
            ogc_sys::SYS_StopPMC();
        }
    }

    /// Reset PMC
    pub fn reset_pmc() {
        unsafe {
            ogc_sys::SYS_ResetPMC();
        }
    }

    /// Reset System
    pub fn reset_system(reset: i32, reset_type: ResetTypes, force_menu: i32) {
        unsafe {
            ogc_sys::SYS_ResetSystem(reset, reset_type.into(), force_menu);
        }
    }

    /// Reset Button Down
    pub fn reset_button_down() -> u32 {
        unsafe { ogc_sys::SYS_ResetButtonDown() }
    }

    /// Set Reset Callback
    pub fn set_reset_callback<F>(callback: Box<F>) {
        // TODO: Check if this implementation can be changed.
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(irq: u32, ctx: *mut c_void) = mem::transmute(ptr);
            // TODO: Do something with the returned callback.
            let _ = ogc_sys::SYS_SetResetCallback(Some(code));
        }
    }

    /// Set Power Callback
    pub fn set_power_callback<F>(callback: Box<F>) {
        // TODO: Check if this implementation can be changed.
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn() = mem::transmute(ptr);
            // TODO: Do something with the returned callback.
            let _ = ogc_sys::SYS_SetPowerCallback(Some(code));
        }
    }

    /// Protect Range
    pub fn protect_range(
        channel: MemoryProtectChannels,
        address: u32,
        bytes: u32,
        control: MemoryProtectModes,
    ) {
        unsafe {
            ogc_sys::SYS_ProtectRange(
                channel.into(),
                address as *mut c_void,
                bytes,
                control.into(),
            );
        }
    }

    /// Switch Fiber
    pub fn switch_fiber(arg0: u32, arg1: u32, arg2: u32, arg3: u32, pc: u32, newsp: u32) {
        unsafe {
            ogc_sys::SYS_SwitchFiber(arg0, arg1, arg2, arg3, pc, newsp);
        }
    }

    /// Get Hollywood Revision
    pub fn get_hollywood_revision() -> u32 {
        unsafe { ogc_sys::SYS_GetHollywoodRevision() }
    }

    /// Get system time.
    pub fn system_time() -> u64 {
        unsafe { ogc_sys::SYS_Time() }
    }
}
