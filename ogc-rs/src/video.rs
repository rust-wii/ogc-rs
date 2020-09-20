//! The ``video`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around video functions.

use crate::{mem_cached_to_uncached, system::System};
use alloc::boxed::Box;
use core::{convert::TryFrom, ffi::c_void, mem, ptr};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub struct RenderConfig {
    pub tv_type: u32,
    pub framebuffer_width: u16,
    pub embed_framebuffer_height: u16,
    pub extern_framebuffer_height: u16,
    pub vi_x_origin: u16,
    pub vi_y_origin: u16,
    pub vi_width: u16,
    pub vi_height: u16,
    pub extern_framebuffer_mode: u32,
    pub field_rendering: u8,
    pub anti_aliasing: u8,
    pub sample_pattern: [[u8; 2usize]; 12usize],
    pub v_filter: [u8; 7usize],
}

impl Into<*mut ogc_sys::GXRModeObj> for RenderConfig {
    fn into(self) -> *mut ogc_sys::GXRModeObj {
        Box::into_raw(Box::new(ogc_sys::GXRModeObj {
            viTVMode: self.tv_type,
            fbWidth: self.framebuffer_width,
            efbHeight: self.embed_framebuffer_height,
            xfbHeight: self.extern_framebuffer_height,
            viXOrigin: self.vi_x_origin,
            viYOrigin: self.vi_y_origin,
            viWidth: self.vi_width,
            viHeight: self.vi_height,
            xfbMode: self.extern_framebuffer_mode,
            field_rendering: self.field_rendering,
            aa: self.anti_aliasing,
            sample_pattern: self.sample_pattern,
            vfilter: self.v_filter,
        }))
    }
}

impl Into<RenderConfig> for *mut ogc_sys::GXRModeObj {
    fn into(self) -> RenderConfig {
        // i'll do this the right way one day
        // :gun:
        unsafe {
            RenderConfig {
                tv_type: (*self).viTVMode,
                framebuffer_width: (*self).fbWidth,
                embed_framebuffer_height: (*self).efbHeight,
                extern_framebuffer_height: (*self).xfbHeight,
                vi_x_origin: (*self).viXOrigin,
                vi_y_origin: (*self).viYOrigin,
                vi_width: (*self).viWidth,
                vi_height: (*self).viHeight,
                extern_framebuffer_mode: (*self).xfbMode,
                field_rendering: (*self).field_rendering,
                anti_aliasing: (*self).aa,
                sample_pattern: (*self).sample_pattern,
                v_filter: (*self).vfilter,
            }
        }
    }
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TVMode {
    /// Used in NA / JPN
    ViNtsc = 0,
    /// Used in Europe
    ViPal = 1,
    /// Similar to NTSC, Used in Brazil
    ViMpal = 2,
    /// Debug Mode for NA / JPN - Special Decoder Needed
    ViDebug = 3,
    /// Debug mode for EU - Special Decoder Needed
    ViDebugPal = 4,
    /// RGB 60Hz, 480 lines (same timing + aspect as NTSC) used in Europe
    ViEuRgb60 = 5,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ViField {
    ViLowerField = 0,
    ViUpperField = 1,
}

/// Represents the video service.
pub struct Video {
    pub render_config: RenderConfig,
    pub framebuffer: *mut c_void,
}

impl Video {
    pub fn init() -> Self {
        unsafe {
            ogc_sys::VIDEO_Init();

            let r_mode = ogc_sys::VIDEO_GetPreferredMode(ptr::null_mut()).into();

            Self {
                render_config: r_mode,
                framebuffer: mem_cached_to_uncached!(System::allocate_framebuffer(
                    Self::get_preferred_mode().into()
                )),
            }
        }
    }

    pub fn clear_framebuffer(&mut self, rconf: RenderConfig, colour: u32) {
        unsafe {
            ogc_sys::VIDEO_ClearFrameBuffer(rconf.into(), self.framebuffer, colour);
        }
    }

    pub fn get_preferred_mode() -> RenderConfig {
        unsafe { ogc_sys::VIDEO_GetPreferredMode(ptr::null_mut()).into() }
    }

    pub fn configure(render_config: RenderConfig) {
        unsafe {
            ogc_sys::VIDEO_Configure(render_config.into());
        }
    }

    pub fn flush() {
        unsafe {
            ogc_sys::VIDEO_Flush();
        }
    }

    pub fn get_current_line() {
        unsafe {
            ogc_sys::VIDEO_GetCurrentLine();
        }
    }

    pub fn get_tv_mode() -> TVMode {
        let mode = unsafe { ogc_sys::VIDEO_GetCurrentTvMode() };
        TVMode::try_from(mode).unwrap()
    }

    pub fn get_next_field() -> ViField {
        let next_field = unsafe { ogc_sys::VIDEO_GetNextField() };
        ViField::try_from(next_field).unwrap()
    }

    pub fn is_component_cable() -> bool {
        let component = unsafe { ogc_sys::VIDEO_HaveComponentCable() };
        component == 1
    }

    pub fn set_black(is_black: bool) {
        unsafe {
            ogc_sys::VIDEO_SetBlack(is_black);
        }
    }

    pub fn set_next_framebuffer(framebuffer: *mut c_void) {
        unsafe {
            ogc_sys::VIDEO_SetNextFramebuffer(framebuffer);
        }
    }

    pub fn set_next_right_framebuffer(framebuffer: *mut c_void) {
        unsafe {
            ogc_sys::VIDEO_SetNextRightFramebuffer(framebuffer);
        }
    }

    pub fn register_post_retrace_callback<F>(callback: Box<F>)
    where
        F: Fn(u32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ogc_sys::VIDEO_SetPostRetraceCallback(Some(code));
        }
    }

    pub fn register_pre_retrace_callback<F>(callback: Box<F>)
    where
        F: Fn(u32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ogc_sys::VIDEO_SetPreRetraceCallback(Some(code));
        }
    }

    pub fn wait_vsync() {
        unsafe {
            ogc_sys::VIDEO_WaitVSync();
        }
    }
}
