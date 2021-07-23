//! The ``gx`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the graphics functions found in ``gx.h``.

use core::ffi::c_void;
use ogc_sys::{Mtx as Mtx34, Mtx44};

/// Helper function for `Gx::init`
pub fn gp_fifo(fifo_size: usize) -> *mut c_void {
    unsafe {
        let gp_fifo = crate::mem_cached_to_uncached!(libc::memalign(32, fifo_size));
        libc::memset(gp_fifo, 0, fifo_size);
        gp_fifo
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }
}

/// Represents the GX service.
pub struct Gx;

impl Gx {
    /// Initializes the graphics processor to its initial state.
    /// See [GX_Init](https://libogc.devkitpro.org/gx_8h.html#aea24cfd5f8f2b168dc4f60d4883a6a8e) for more.
    pub fn init(gp_fifo: *mut c_void, fifo_size: u32) -> *mut ogc_sys::GXFifoObj {
        // SAFETY: Both `fifo_size` and `gp_fifo` is aligned to a 32-byte boundary.
        assert_eq!(0, fifo_size % 32);
        unsafe { ogc_sys::GX_Init(gp_fifo, fifo_size) }
    }

    /// Sets color and Z value to clear the EFB to during copy operations.
    /// See [GX_SetCopyClear](https://libogc.devkitpro.org/gx_8h.html#a17265aefd7e64820de53abd9113334bc) for more.
    pub fn set_copy_clear(background: Color, z_value: u32) {
        let Color(r, g, b, a) = background;
        let background = ogc_sys::_gx_color { r, g, b, a };
        unsafe { ogc_sys::GX_SetCopyClear(background, z_value) }
    }

    /// Sets the viewport rectangle in screen coordinates.
    /// See [GX_SetViewport](https://libogc.devkitpro.org/gx_8h.html#aaccd37675da5a22596fad756c73badc2) for more.
    pub fn set_viewport(x_orig: f32, y_orig: f32, wd: f32, hd: f32, near_z: f32, far_z: f32) {
        unsafe { ogc_sys::GX_SetViewport(x_orig, y_orig, wd, hd, near_z, far_z) }
    }

    /// Calculates an appropriate Y scale factor value for GX_SetDispCopyYScale() based on the height of the EFB and the height of the XFB.
    /// See [GX_GetYScaleFactor](https://libogc.devkitpro.org/gx_8h.html#a1558cf7d2eb9a6690fee4b64c4fc5a8e) for more.
    pub fn get_y_scale_factor(efb_height: u16, xfb_height: u16) -> f32 {
        unsafe { ogc_sys::GX_GetYScaleFactor(efb_height, xfb_height) }
    }

    /// Sets the vertical scale factor for the EFB to XFB copy operation.
    /// See [GX_SetDispCopyYScale](https://libogc.devkitpro.org/gx_8h.html#a1a4ebb4e742f4ce2f010768e09e07c48) for more.
    pub fn set_disp_copy_y_scale(y_scale: f32) -> u32 {
        unsafe { ogc_sys::GX_SetDispCopyYScale(y_scale) }
    }

    /// Sets the scissor rectangle.
    /// See [GX_SetScissor](https://libogc.devkitpro.org/gx_8h.html#a689bdd17fc74bf86a4c4f00418a2c596) for more.
    pub fn set_scissor(x_origin: u32, y_origin: u32, wd: u32, hd: u32) {
        unsafe { ogc_sys::GX_SetScissor(x_origin, y_origin, wd, hd) }
    }

    /// Sets the source parameters for the EFB to XFB copy operation.
    /// See [GX_SetDispCopySrc](https://libogc.devkitpro.org/gx_8h.html#a979d8db7abbbc2e9a267f5d1710ac588) for more.
    pub fn set_disp_copy_src(left: u16, top: u16, wd: u16, hd: u16) {
        assert_eq!(0, left % 2);
        assert_eq!(0, top % 2);
        assert_eq!(0, wd % 2);
        assert_eq!(0, hd % 2);
        unsafe { ogc_sys::GX_SetDispCopySrc(left, top, wd, hd) }
    }

    /// Sets the witth and height of the display buffer in pixels.
    /// See [GX_SetDispCopyDst](https://libogc.devkitpro.org/gx_8h.html#ab6f639059b750e57af4c593ba92982c5) for more.
    pub fn set_disp_copy_dst(width: u16, height: u16) {
        assert_eq!(0, width % 16);
        unsafe { ogc_sys::GX_SetDispCopyDst(width, height) }
    }

    /// Sets the subpixel sample patterns and vertical filter coefficients used to filter subpixels into pixels.
    /// See [GX_SetCopyFilter](https://libogc.devkitpro.org/gx_8h.html#afd65b7e5f2040ddb3352649efde72faf) for more.
    pub fn set_copy_filter(
        aa: u8,
        sample_pattern: &mut [[u8; 2]; 12],
        vf: u8,
        v_filter: &mut [u8; 7],
    ) {
        unsafe { ogc_sys::GX_SetCopyFilter(aa, sample_pattern as *mut _, vf, v_filter as *mut _) }
    }

    /// Controls various rasterization and texturing parameters that relate to field-mode and double-strike rendering.
    /// See [GX_SetFieldMode](https://libogc.devkitpro.org/gx_8h.html#a13f0df0011d04c3d986135e800fbcd21) for more.
    pub fn set_field_mode(field_mode: u8, half_aspect_ratio: u8) {
        unsafe { ogc_sys::GX_SetFieldMode(field_mode, half_aspect_ratio) }
    }

    /// Sets the format of pixels in the Embedded Frame Buffer (EFB).
    /// See [GX_SetPixelFmt](https://libogc.devkitpro.org/gx_8h.html#a018d9b0359f9689ac41f44f0b2374ffb) for more.
    pub fn set_pixel_fmt(pix_fmt: u8, z_fmt: u8) {
        unsafe { ogc_sys::GX_SetPixelFmt(pix_fmt, z_fmt) }
    }

    /// Enables or disables culling of geometry based on its orientation to the viewer.
    /// See [GX_SetCullMode](https://libogc.devkitpro.org/gx_8h.html#adb4b17c39b24073c3e961458ecf02e87) for more.
    pub fn set_cull_mode(mode: u8) {
        unsafe { ogc_sys::GX_SetCullMode(mode) }
    }

    /// Copies the embedded framebuffer (EFB) to the external framebuffer(XFB) in main memory.
    /// See [GX_CopyDisp](https://libogc.devkitpro.org/gx_8h.html#a9ed0ae3f900abb6af2e930dff7a6bc28) for more.
    pub fn copy_disp(dest: *mut c_void, clear: u8) {
        unsafe { ogc_sys::GX_CopyDisp(dest, clear) }
    }

    /// Sets the gamma correction applied to pixels during EFB to XFB copy operation.
    /// See [GX_SetDispCopyGamma](https://libogc.devkitpro.org/gx_8h.html#aa8e5bc962cc786b2049345fa698d4efa) for more.
    pub fn set_disp_copy_gamma(gamma: u8) {
        unsafe { ogc_sys::GX_SetDispCopyGamma(gamma) }
    }

    /// Sets the attribute format (vtxattr) for a single attribute in the Vertex Attribute Table (VAT).
    /// See [GX_SetVtxAttrFmt](https://libogc.devkitpro.org/gx_8h.html#a87437061debcc0457b6b6dc2eb021f23) for more.
    pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: u32, comptype: u32, compsize: u32, frac: u32) {
        unsafe { ogc_sys::GX_SetVtxAttrFmt(vtxfmt, vtxattr, comptype, compsize, frac) }
    }

    /// Sets the number of color channels that are output to the TEV stages.
    /// See [GX_SetNumChans](https://libogc.devkitpro.org/gx_8h.html#a390c37e594986403c623df2bed61c2b2) for more.
    pub fn set_num_chans(num: u8) {
        unsafe { ogc_sys::GX_SetNumChans(num) }
    }

    /// Sets the number of texture coordinates that are generated and available for use in the Texture Environment TEV stages.
    /// See [GX_SetNumTexGens](https://libogc.devkitpro.org/gx_8h.html#a55a79a1688d3a6957ee0c37d6323d159) for more.
    pub fn set_num_tex_gens(nr: u32) {
        unsafe { ogc_sys::GX_SetNumTexGens(nr) }
    }

    /// Simplified function to set various TEV parameters for this tevstage based on a predefined combiner mode.
    /// See [GX_SetTevOp](https://libogc.devkitpro.org/gx_8h.html#a68554713cdde7b45ae4d5ce156239cf8) for more.
    pub fn set_tev_op(tevstage: u8, mode: u8) {
        unsafe { ogc_sys::GX_SetTevOp(tevstage, mode) }
    }

    /// Specifies the texture and rasterized color that will be available as inputs to this TEV tevstage.
    /// See [GX_SetTevOrder](https://libogc.devkitpro.org/gx_8h.html#ae64799e52298de39efc74bf989fc57f5) for more.
    pub fn set_tev_order(tevstage: u8, texcoord: u8, texmap: u32, color: u8) {
        unsafe { ogc_sys::GX_SetTevOrder(tevstage, texcoord, texmap, color) }
    }

    /// Specifies how texture coordinates are generated.
    /// See [GX_SetTexCoordGen](https://libogc.devkitpro.org/gx_8h.html#a7d3139b693ace5587c3224e7df2d8245) for more.
    pub fn set_tex_coord_gen(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32) {
        unsafe { ogc_sys::GX_SetTexCoordGen(texcoord, tgen_typ, tgen_src, mtxsrc) }
    }

    /// Invalidates the current caches of the Texture Memory (TMEM).
    /// See [GX_InvalidateTexAll](https://libogc.devkitpro.org/gx_8h.html#a1e5666740bcd3c9325dd2b82006621ee) for more.
    pub fn invalidate_tex_all() {
        unsafe { ogc_sys::GX_InvalidateTexAll() }
    }

    /// Loads the state describing a texture into one of eight hardware register sets.
    /// See [GX_LoadTexObj](https://libogc.devkitpro.org/gx_8h.html#ad6388b0e4a0f2ffb5daa16a8851fa567) for more.
    pub fn load_tex_obj(obj: &mut ogc_sys::GXTexObj, mapid: u8) {
        unsafe { ogc_sys::GX_LoadTexObj(obj, mapid) }
    }

    /// Sets the projection matrix.
    /// See [GX_LoadProjectionMtx](https://libogc.devkitpro.org/gx_8h.html#a241a1301f006ed04b7895c051959f64e) for more.
    pub fn load_projection_mtx(mt: &mut Mtx44, p_type: u8) {
        unsafe { ogc_sys::GX_LoadProjectionMtx(mt as *mut _, p_type) }
    }

    /// Invalidates the vertex cache.
    /// See [GX_InvVtxCache](https://libogc.devkitpro.org/gx_8h.html#a188bc7f388f971bc845dded41a24d1dc) for more.
    pub fn inv_vtx_cache() {
        unsafe { ogc_sys::GX_InvVtxCache() }
    }

    /// Clears all vertex attributes of the current vertex descriptor to GX_NONE.
    /// See [GX_ClearVtxDesc](https://libogc.devkitpro.org/gx_8h.html#acf1f933c4c653e399106e8ac244fabd0) for more.
    pub fn clear_vtx_desc() {
        unsafe { ogc_sys::GX_ClearVtxDesc() }
    }

    /// Sets the type of a single attribute (attr) in the current vertex descriptor.
    /// See [GX_SetVtxDesc](https://libogc.devkitpro.org/gx_8h.html#af41b45011ae731ae5697b26b2bf97e2f) for more.
    pub fn set_vtx_desc(attr: u8, v_type: u8) {
        unsafe { ogc_sys::GX_SetVtxDesc(attr, v_type) }
    }

    /// Used to load a 3x4 modelview matrix mt into matrix memory at location pnidx.
    /// See [GX_LoadPosMtxImm](https://libogc.devkitpro.org/gx_8h.html#a90349e713128a1fa4fd6048dcab7b5e7) for more.
    pub fn load_pos_mtx_imm(mt: &mut Mtx34, pnidx: u32) {
        unsafe { ogc_sys::GX_LoadPosMtxImm(mt as *mut _, pnidx) }
    }

    /// Sends a DrawDone command to the GP and stalls until its subsequent execution.
    /// See [GX_DrawDone](https://libogc.devkitpro.org/gx_8h.html#a00f07b60ae2124fe027a82d7d9ae64b0) for more.
    pub fn draw_done() {
        unsafe { ogc_sys::GX_DrawDone() }
    }

    /// Sets the Z-buffer compare mode.
    /// See [GX_SetZMode](https://libogc.devkitpro.org/gx_8h.html#a2af0d050f56ef45dd25d0db18909fa00) for more.
    pub fn set_z_mode(enable: u8, func: u8, update_enable: u8) {
        unsafe { ogc_sys::GX_SetZMode(enable, func, update_enable) }
    }

    /// Determines how the source image, generated by the graphics processor, is blended with the Embedded Frame Buffer (EFB).
    /// See [GX_SetBlendMode](https://libogc.devkitpro.org/gx_8h.html#a1d9c43b161f3c5a30b9fd8ea182c8eb6) for more.
    pub fn set_blend_mode(b_type: u8, src_fact: u8, dst_fact: u8, op: u8) {
        unsafe { ogc_sys::GX_SetBlendMode(b_type, src_fact, dst_fact, op) }
    }

    /// Enables or disables alpha-buffer updates of the Embedded Frame Buffer (EFB).
    /// See [GX_SetAlphaUpdate](https://libogc.devkitpro.org/gx_8h.html#ac238051bda896c8bb11802184882a2a0) for more.
    pub fn set_alpha_update(enable: u8) {
        unsafe { ogc_sys::GX_SetAlphaUpdate(enable) }
    }

    /// Enables or disables color-buffer updates when rendering into the Embedded Frame Buffer (EFB).
    /// See [GX_SetColorUpdate](https://libogc.devkitpro.org/gx_8h.html#a3978e3b08198e52d7cea411e90ece3e5) for more.
    pub fn set_color_update(enable: u8) {
        unsafe { ogc_sys::GX_SetColorUpdate(enable) }
    }

    /// Sets the array base pointer and stride for a single attribute.
    /// See [GX_SetArray](https://libogc.devkitpro.org/gx_8h.html#a5164fc6aa2a678d792af80d94bfa1ec2) for more.
    pub fn set_array(attr: u32, ptr: *mut c_void, stride: u8) {
        unsafe { ogc_sys::GX_SetArray(attr, ptr, stride) }
    }

    /// Begins drawing of a graphics primitive.
    /// See [GX_Begin](https://libogc.devkitpro.org/gx_8h.html#ac1e1239130a33d9fae1352aee8d2cab9) for more.
    pub fn begin(primitive: u8, vtxfmt: u8, vtxcnt: u16) {
        unsafe { ogc_sys::GX_Begin(primitive, vtxfmt, vtxcnt) }
    }

    /// Sets the parameters for the alpha compare function which uses the alpha output from the last active TEV stage.
    /// See [Gx_SetAlphaCompare](https://libogc.devkitpro.org/gx_8h.html#a23ac269062a1b2c2efc8ad5aae24b26a) for more.
    pub fn set_alpha_compare(comp0: u8, ref0: u8, aop: u8, comp1: u8, ref1: u8) {
        unsafe { ogc_sys::GX_SetAlphaCompare(comp0, ref0, aop, comp1, ref1) }
    }

    /// Sets the parameters for the alpha compare function which uses the alpha output from the last active TEV stage.
    /// See [GX_SetClipMode](https://libogc.devkitpro.org/gx_8h.html#a3d348d7af8ded25b57352e956f43d974) for more.
    pub fn set_clip_mode(mode: u8) {
        unsafe { ogc_sys::GX_SetClipMode(mode) }
    }

    /// Allows the CPU to write color directly to the Embedded Frame Buffer (EFB) at position x, y.
    /// See [GX_PokeARGB](https://libogc.devkitpro.org/gx_8h.html#a5038d2f65e7959d64c68dcb1855353d8) for more.
    pub fn poke_argb(x: u16, y: u16, color: Color) {
        assert!(x < 640, "x must be less than 640, currently {}", x);
        assert!(y < 528, "y must be less than 527, currently {}", y);
        let Color(r, g, b, a) = color;
        let color = ogc_sys::_gx_color { r, g, b, a };
        unsafe {
            ogc_sys::GX_PokeARGB(x, y, color);
        }
    }

    pub fn position_3f32(x: f32, y: f32, z: f32) {
        unsafe {
            ogc_sys::GX_Position3f32(x, y, z);
        }
    }

    pub fn color_1u32(clr: u32) {
        unsafe {
            ogc_sys::GX_Color1u32(clr);
        }
    }

    pub fn flush() {
        unsafe { ogc_sys::GX_Flush() }
    }

    pub fn end() {
        unsafe { ogc_sys::GX_End() }
    }

    pub fn position1x8(index: u8) {
        unsafe { ogc_sys::GX_Position1x8(index) }
    }

    pub fn color1x8(index: u8) {
        unsafe { ogc_sys::GX_Color1x8(index) }
    }

    pub fn tex_coord_2f32(s: f32, t: f32) {
        unsafe { ogc_sys::GX_TexCoord2f32(s, t) }
    }
}
