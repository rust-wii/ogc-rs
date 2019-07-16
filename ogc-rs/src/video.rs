use std::ffi;
use system::System;

struct RenderConfig {
    tv_type: u32,
    framebuffer_width: u16,
    embed_framebuffer_height: u16,
    extern_framebuffer_height: u16,
    vi_x_origin: u16,
    vi_y_origin: u16,
    vi_width: u16,
    vi_height: u16,
    extern_framebuffer_mode: u32,
    field_rendering: u8,
    anti_aliasing: u8,
    sample_pattern: [[u8; 2u32]; 12u32],
    v_filter: [u8; 7u32],
}

impl Into<*mut ogc_sys::GXRModeObj> for &mut RenderConfig {
    fn into(self) -> *mut ogc_sys::GXRModeObj {
        Box::into_raw(Box::new(ogc_sys::GXRModeObj {
            viTVMode:        self.tv_type,
            fbWidth:         self.framebuffer_width,
            efbHeight:       self.embed_framebuffer_height,
            xfbHeight:       self.extern_framebuffer_height,
            viXOrigin:       self.vi_x_origin,
            viYOrigin:       self.vi_y_origin,
            viWidth:         self.vi_width,
            viHeight:        self.vi_height,
            xfbMode:         self.extern_framebuffer_mode,
            field_rendering: self.field_rendering,
            aa:              self.anti_aliasing,
            sample_pattern:  self.sample_pattern,
            vfilter:         self.v_filter
        }))
    }
}

impl Into<RenderConfig> for *mut ogc_sys::GXRModeObj {
    fn into(self) -> RenderConfig {
        RenderConfig {
            tv_type: self.viTVMode,
            framebuffer_width: self.fbWidth,
            embed_framebuffer_height: self.efbHeight,
            extern_framebuffer_height: self.xfbHeight,
            vi_x_origin: self.viXOrigin,
            vi_y_origin: self.viYOrigin,
            vi_width: self.viWidth,
            vi_height: self.viHeight,
            extern_framebuffer_mode: self.xfbMode,
            field_rendering: self.field_rendering,
            anti_aliasing: self.aa,
            sample_pattern: self.sample_pattern,
            v_filter: self.vfilter
        }
    }
}

enum tv_mode {
    vi_ntsc = 0, //Used in NA / JPN
    vi_pal = 1, //Used in Europe
    vi_mpal = 2, //Similar to NTSC, Used in Brazil
    vi_debug = 3, //Debug Mode for NA / JPN - Special Decoder Needed
    vi_debug_pal = 4, //Debug mode for EU - Special Decoder Needed
    vi_eu_rgb_60 = 5 //RGB 60Hz, 480 lines (same timing + aspect as NTSC) used in Europe
}

enum field {
    vi_lower_field = 0,
    vi_upper_field = 1
}

struct Video {  
    pub render_config: render_config,
    pub framebuffer: std::ffi::c_void,
}

impl Video {

    pub fn init() -> () {
        unsafe {
            ogc_sys::VIDEO_Init();
        }

        Video(get_preferred_mode(), utils::cached_to_uncached!(System::allocate_framebuffer(get_preferred_mode())));
    }

    pub fn clear_framebuffer(&mut self, rconf: render_config, colour: u32) -> () {
        unsafe {
            ogc_sys::VIDEO_ClearFrameBuffer(rconf, self.framebuffer, colour);
        }
    }

    pub fn get_preferred_mode() -> RenderConfig {
        unsafe {
            ogc_sys::VIDEO_GetPreferredMode(std::ptr::null_mut()).into()
        }
    }

    pub fn configure(render_config: RenderConfig) -> () {
        unsafe {
            ogc_sys::VIDEO_Configure(render_config.into());
        }
    }

    pub fn flush() -> () {
        unsafe {
            ogc_sys::VIDEO_Flush();
        }
    }

    pub fn get_current_line() -> () {
        unsafe {
            ogc_sys::VIDEO_GetCurrentLine();
        }
    }

    pub fn get_tv_mode() -> tv_mode {
        unsafe {
            let mode = ogc_sys::VIDEO_GetCurrentTvMode();
        }
        tv_mode::from_u32(mode).unwrap();
    }

    pub fn get_next_field() -> field {
        unsafe {
            let next_field = ogc_sys::VIDEO_GetNextField();
        }
        field::from_u32(next_field).unwrap();
    }

    pub fn is_component_cable() -> bool {
        unsafe {
            let component = ogc_sys::VIDEO_HaveComponentCable();
        }

        bool::from_u32(component).unwrap();
    }

    pub fn set_black(is_black: bool) -> () {
        unsafe {
            ogc_sys::VIDEO_SetBlack(is_black);
        }
    }

    pub fn set_next_framebuffer(framebuffer: *mut std::ffi::c_void) -> () {
        unsafe {
            ogc_sys::VIDEO_SetNextFramebuffer(framebuffer);
        }
    }

    pub fn set_next_right_framebuffer(framebuffer: *mut std::ffi::c_void) -> () {
        unsafe {
            ogc_sys::VIDEO_SetNextRightFramebuffer(framebuffer);
        }
    }

    pub fn register_post_retrace_callback<F>(callback: Box<F>) where F: Fn(u32) -> (), {
        unsafe {
            let ptr = Box::into_raw(callback);
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ogc_sys::VIDEO_SetPostRetraceCallback(Some(code));
        }
    }

    pub fn register_pre_retrace_callback<F>(callback: Box<F>) where F: Fn(u32) -> (), {
        unsafe {
            let ptr = Box::into_raw(callback);
            let code: extern "C" fn(vi_retrace_callback: u32) = mem::transmute(ptr);

            let _ = ogc_sys::VIDEO_SetPreRetraceCallback(Some(code));
        }
    }

    pub fn wait_vsync() -> () {
        unsafe {
            ogc_sys::VIDEO_WaitVSync();
        }
    }

}