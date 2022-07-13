//! The ``video`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around video functions.

use crate::{ffi, mem_cached_to_uncached, system::System};
use alloc::boxed::Box;
use core::{convert::TryFrom, ffi::c_void, mem, ptr};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone)]
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

impl From<&RenderConfig> for *mut ffi::GXRModeObj {
    fn from(cfg: &RenderConfig) -> *mut ffi::GXRModeObj {
        Box::into_raw(Box::new(ffi::GXRModeObj {
            viTVMode: cfg.tv_type,
            fbWidth: cfg.framebuffer_width,
            efbHeight: cfg.embed_framebuffer_height,
            xfbHeight: cfg.extern_framebuffer_height,
            viXOrigin: cfg.vi_x_origin,
            viYOrigin: cfg.vi_y_origin,
            viWidth: cfg.vi_width,
            viHeight: cfg.vi_height,
            xfbMode: cfg.extern_framebuffer_mode,
            field_rendering: cfg.field_rendering,
            aa: cfg.anti_aliasing,
            sample_pattern: cfg.sample_pattern,
            vfilter: cfg.v_filter,
        }))
    }
}

impl From<&ffi::GXRModeObj> for RenderConfig {
    fn from(obj: &ffi::GXRModeObj) -> Self {
        RenderConfig {
            tv_type: (*obj).viTVMode,
            framebuffer_width: (*obj).fbWidth,
            embed_framebuffer_height: (*obj).efbHeight,
            extern_framebuffer_height: (*obj).xfbHeight,
            vi_x_origin: (*obj).viXOrigin,
            vi_y_origin: (*obj).viYOrigin,
            vi_width: (*obj).viWidth,
            vi_height: (*obj).viHeight,
            extern_framebuffer_mode: (*obj).xfbMode,
            field_rendering: (*obj).field_rendering,
            anti_aliasing: (*obj).aa,
            sample_pattern: (*obj).sample_pattern,
            v_filter: (*obj).vfilter,
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
            ffi::VIDEO_Init();

            let r_mode = ffi::VIDEO_GetPreferredMode(ptr::null_mut())
                .as_ref()
                .unwrap()
                .into();

            Self {
                render_config: r_mode,
                framebuffer: mem_cached_to_uncached!(System::allocate_framebuffer(
                    &Self::get_preferred_mode()
                )),
            }
        }
    }

    pub fn clear_framebuffer(&mut self, rconf: &RenderConfig, colour: u32) {
        unsafe {
            ffi::VIDEO_ClearFrameBuffer(rconf.into(), self.framebuffer, colour);
        }
    }

    pub fn get_preferred_mode() -> RenderConfig {
        unsafe {
            ffi::VIDEO_GetPreferredMode(ptr::null_mut())
                .as_ref()
                .unwrap()
                .into()
        }
    }

    pub fn configure(render_config: &RenderConfig) {
        unsafe {
            ffi::VIDEO_Configure(render_config.into());
        }
    }

    pub fn flush() {
        unsafe {
            ffi::VIDEO_Flush();
        }
    }

    pub fn get_current_line() {
        unsafe {
            ffi::VIDEO_GetCurrentLine();
        }
    }

    pub fn get_tv_mode() -> TVMode {
        let mode = unsafe { ffi::VIDEO_GetCurrentTvMode() };
        TVMode::try_from(mode).unwrap()
    }

    pub fn get_next_field() -> ViField {
        let next_field = unsafe { ffi::VIDEO_GetNextField() };
        ViField::try_from(next_field).unwrap()
    }

    pub fn is_component_cable() -> bool {
        let component = unsafe { ffi::VIDEO_HaveComponentCable() };
        component == 1
    }

    pub fn set_black(is_black: bool) {
        unsafe {
            ffi::VIDEO_SetBlack(is_black);
        }
    }

    /// # Safety
    ///
    /// The user must ensure this pointer to to valid framebuffer data
    pub unsafe fn set_next_framebuffer(framebuffer: *mut c_void) {
        ffi::VIDEO_SetNextFramebuffer(framebuffer);
    }

    /// # Safety
    ///
    /// The user must ensure this pointer to to valid framebuffer data
    pub unsafe fn set_next_right_framebuffer(framebuffer: *mut c_void) {
        ffi::VIDEO_SetNextRightFramebuffer(framebuffer);
    }

    pub fn register_post_retrace_callback<F>(callback: Box<F>)
    where
        F: Fn(u32),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ffi::VIDEO_SetPostRetraceCallback(Some(code));
        }
    }

    pub fn register_pre_retrace_callback<F>(callback: Box<F>)
    where
        F: Fn(u32),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ffi::VIDEO_SetPreRetraceCallback(Some(code));
        }
    }

    pub fn wait_vsync() {
        unsafe {
            ffi::VIDEO_WaitVSync();
        }
    }
}
