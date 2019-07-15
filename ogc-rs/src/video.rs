use std::ffi;

struct render_config {
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
struct Video {
    pub rmode: render_config,
    pub framebuffer: std::ffi::c_void,
}

impl Video {
    pub fn init() -> () {
        unsafe {
            ogc_sys::VIDEO_Init();
        }

        Video(())
    }

    pub fn clear_framebuffer(rconf: render_config, framebuffer: std::ffi::c_void, colour: u32) -> () {
        unsafe {
            ogc_sys::VIDEO_ClearFrameBuffer(rconf, framebuffer, colour);
        }
    }

    
}