/// Helper function for `Gx::init`
pub fn gp_fifo(fifo_size: usize) -> *mut libc::c_void {
    unsafe {
        let gp_fifo = libc::memalign(32, fifo_size);
        libc::memset(gp_fifo, 0, fifo_size);
        gp_fifo
    }
}

type Mtx34 = [[f32; 4]; 3];
type Mtx44 = [[f32; 4]; 4];

/// Represents the GX service.
pub struct Gx;

impl Gx {
    /// Initializes the graphics processor to its initial state.
    /// See [GX_Init](https://libogc.devkitpro.org/gx_8h.html#aea24cfd5f8f2b168dc4f60d4883a6a8e) for more.
    pub fn init(gp_fifo: *mut libc::c_void, fifo_size: u32) -> *mut ogc_sys::GXFifoObj {
        assert_eq!(0, fifo_size % 32);
        unsafe {
            ogc_sys::GX_Init(gp_fifo, fifo_size)
        }
    }

    /// Sets color and Z value to clear the EFB to during copy operations.
    /// See [GX_SetCopyClear](https://libogc.devkitpro.org/gx_8h.html#a17265aefd7e64820de53abd9113334bc) for more.
    pub fn set_copy_clear(background: (u8, u8, u8, u8), z_value: u32) {
        let (r, g, b, a) = background;
        let background = ogc_sys::_gx_color {r, g, b, a};
        unsafe {
            ogc_sys::GX_SetCopyClear(background, z_value)
        }
    }

    /// Sets the viewport rectangle in screen coordinates.
    /// See [GX_SetViewport](https://libogc.devkitpro.org/gx_8h.html#aaccd37675da5a22596fad756c73badc2) for more.
    pub fn set_viewport(x_orig: f32, y_orig: f32, wd: f32, hd: f32, near_z: f32, far_z: f32) {
        unsafe {
            ogc_sys::GX_SetViewport(x_orig, y_orig, wd, hd, near_z, far_z)
        }
    }

    /// Calculates an appropriate Y scale factor value for GX_SetDispCopyYScale() based on the height of the EFB and the height of the XFB.
    /// See [GX_GetYScaleFactor](https://libogc.devkitpro.org/gx_8h.html#a1558cf7d2eb9a6690fee4b64c4fc5a8e) for more.
    pub fn get_y_scale_factor(efb_height: u16, xfb_height: u16) -> f32 {
        unsafe {
            ogc_sys::GX_GetYScaleFactor(efb_height, xfb_height)
        }
    }

    /// Sets the vertical scale factor for the EFB to XFB copy operation.
    /// See [GX_SetDispCopyYScale](https://libogc.devkitpro.org/gx_8h.html#a1a4ebb4e742f4ce2f010768e09e07c48) for more.
    pub fn set_disp_copy_y_scale(y_scale: f32) -> u32 {
        unsafe {
            ogc_sys::GX_SetDispCopyYScale(y_scale)
        }
    }

    /// Sets the scissor rectangle.
    /// See [GX_SetScissor](https://libogc.devkitpro.org/gx_8h.html#a689bdd17fc74bf86a4c4f00418a2c596) for more.
    pub fn set_scissor(x_origin: u32, y_origin: u32, wd: u32, hd: u32) {
        unsafe {
            ogc_sys::GX_SetScissor(x_origin, y_origin, wd, hd)
        }
    }

    /// Sets the source parameters for the EFB to XFB copy operation.
    /// See [GX_SetDispCopySrc](https://libogc.devkitpro.org/gx_8h.html#a979d8db7abbbc2e9a267f5d1710ac588) for more.
    pub fn set_disp_copy_src(left: u16, top: u16, wd: u16, hd: u16) {
        assert_eq!(0, left % 2);
        assert_eq!(0, top % 2);
        assert_eq!(0, wd % 2);
        assert_eq!(0, hd % 2);
        unsafe {
            ogc_sys::GX_SetDispCopySrc(left, top, wd, hd)
        }
    }

    /// Sets the witth and height of the display buffer in pixels.
    /// See [GX_SetDispCopyDst](https://libogc.devkitpro.org/gx_8h.html#ab6f639059b750e57af4c593ba92982c5) for more.
    pub fn set_disp_copy_dst(width: u16, height: u16) {
        assert_eq!(0, width % 16);
        unsafe {
            ogc_sys::GX_SetDispCopyDst(width, height)
        }
    }

    /// Sets the subpixel sample patterns and vertical filter coefficients used to filter subpixels into pixels.
    /// See [GX_SetCopyFilter](https://libogc.devkitpro.org/gx_8h.html#afd65b7e5f2040ddb3352649efde72faf) for more.
    pub fn set_copy_filter(aa: u8, mut sample_pattern: [[u8; 2]; 12], vf: u8, mut v_filter: [u8; 7]) {
        unsafe {
            ogc_sys::GX_SetCopyFilter(aa, &mut sample_pattern[0], vf, &mut v_filter[0])
        }
    }

    /// Controls various rasterization and texturing parameters that relate to field-mode and double-strike rendering.
    /// See [GX_SetFieldMode](https://libogc.devkitpro.org/gx_8h.html#a13f0df0011d04c3d986135e800fbcd21) for more.
    pub fn set_field_mode(field_mode: u8, half_aspect_ratio: u8) {
        unsafe {
            ogc_sys::GX_SetFieldMode(field_mode, half_aspect_ratio)
        }
    }

    /// Sets the format of pixels in the Embedded Frame Buffer (EFB).
    /// See [GX_SetPixelFmt](https://libogc.devkitpro.org/gx_8h.html#a018d9b0359f9689ac41f44f0b2374ffb) for more.
    pub fn set_pixel_fmt(pix_fmt: u8, z_fmt: u8) {
        unsafe {
            ogc_sys::GX_SetPixelFmt(pix_fmt, z_fmt)
        }
    }

    /// Enables or disables culling of geometry based on its orientation to the viewer.
    /// See [GX_SetCullMode](https://libogc.devkitpro.org/gx_8h.html#adb4b17c39b24073c3e961458ecf02e87) for more.
    pub fn set_cull_mode(mode: u8) {
        unsafe {
            ogc_sys::GX_SetCullMode(mode)
        }
    }

    /// Copies the embedded framebuffer (EFB) to the external framebuffer(XFB) in main memory.
    /// See [GX_CopyDisp](https://libogc.devkitpro.org/gx_8h.html#a9ed0ae3f900abb6af2e930dff7a6bc28) for more.
    pub fn copy_disp(dest: *mut libc::c_void, clear: u8) {
        unsafe {
            ogc_sys::GX_CopyDisp(dest, clear)
        }
    }

    /// Sets the gamma correction applied to pixels during EFB to XFB copy operation.
    /// See [GX_SetDispCopyGamma](https://libogc.devkitpro.org/gx_8h.html#aa8e5bc962cc786b2049345fa698d4efa) for more.
    pub fn set_disp_copy_gamma(gamma: u8) {
        unsafe {
            ogc_sys::GX_SetDispCopyGamma(gamma)
        }
    }

    /// Sets the attribute format (vtxattr) for a single attribute in the Vertex Attribute Table (VAT).
    /// See [GX_SetVtxAttrFmt](https://libogc.devkitpro.org/gx_8h.html#a87437061debcc0457b6b6dc2eb021f23) for more.
    pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: u32, comptype: u32, compsize: u32, frac: u32) {
        unsafe {
            ogc_sys::GX_SetVtxAttrFmt(vtxfmt, vtxattr, comptype, compsize, frac)
        }
    }

    /// Sets the number of color channels that are output to the TEV stages.
    /// See [GX_SetNumChans](https://libogc.devkitpro.org/gx_8h.html#a390c37e594986403c623df2bed61c2b2) for more.
    pub fn set_num_chans(num: u8) {
        unsafe {
            ogc_sys::GX_SetNumChans(num)
        }
    }

    /// Sets the number of texture coordinates that are generated and available for use in the Texture Environment TEV stages.
    /// See [GX_SetNumTexGens](https://libogc.devkitpro.org/gx_8h.html#a55a79a1688d3a6957ee0c37d6323d159) for more.
    pub fn set_num_tex_gens(nr: u32) {
        unsafe {
            ogc_sys::GX_SetNumTexGens(nr)
        }
    }

    /// Simplified function to set various TEV parameters for this tevstage based on a predefined combiner mode.
    /// See [GX_SetTevOp](https://libogc.devkitpro.org/gx_8h.html#a68554713cdde7b45ae4d5ce156239cf8) for more.
    pub fn set_tev_op(tevstage: u8, mode: u8) {
        unsafe {
            ogc_sys::GX_SetTevOp(tevstage, mode)
        }
    }

    /// Specifies the texture and rasterized color that will be available as inputs to this TEV tevstage.
    /// See [GX_SetTevOrder](https://libogc.devkitpro.org/gx_8h.html#ae64799e52298de39efc74bf989fc57f5) for more.
    pub fn set_tev_order(tevstage: u8, texcoord: u8, texmap: u32, color: u8) {
        unsafe {
            ogc_sys::GX_SetTevOrder(tevstage, texcoord, texmap, color)
        }
    }

    /// Specifies how texture coordinates are generated.
    /// See [GX_SetTexCoordGen](https://libogc.devkitpro.org/gx_8h.html#a7d3139b693ace5587c3224e7df2d8245) for more.
    pub fn set_tex_coord_gen(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32) {
        unsafe {
            ogc_sys::GX_SetTexCoordGen(texcoord, tgen_typ, tgen_src, mtxsrc)
        }
    }

    /// Invalidates the current caches of the Texture Memory (TMEM).
    /// See [GX_InvalidateTexAll](https://libogc.devkitpro.org/gx_8h.html#a1e5666740bcd3c9325dd2b82006621ee) for more.
    pub fn invalidate_tex_all() {
        unsafe {
            ogc_sys::GX_InvalidateTexAll()
        }
    }

    /// Loads the state describing a texture into one of eight hardware register sets.
    /// See [GX_LoadTexObj](https://libogc.devkitpro.org/gx_8h.html#ad6388b0e4a0f2ffb5daa16a8851fa567) for more.
    pub fn load_tex_obj(obj: *mut ogc_sys::GXTexObj, mapid: u8) {
        unsafe {
            ogc_sys::GX_LoadTexObj(obj, mapid)
        }
    }

    /// Sets the projection matrix.
    /// See [GX_LoadProjectionMtx](https://libogc.devkitpro.org/gx_8h.html#a241a1301f006ed04b7895c051959f64e) for more.
    pub fn load_projection_mtx(mut mt: Mtx44, p_type: u8) {
        unsafe {
            ogc_sys::GX_LoadProjectionMtx(&mut mt[0], p_type)
        }
    }

    /// Invalidates the vertex cache.
    /// See [GX_InvVtxCache](https://libogc.devkitpro.org/gx_8h.html#a188bc7f388f971bc845dded41a24d1dc) for more.
    pub fn inv_vtx_cache() {
        unsafe {
            ogc_sys::GX_InvVtxCache()
        }
    }

    /// Clears all vertex attributes of the current vertex descriptor to GX_NONE.
    /// See [GX_ClearVtxDesc](https://libogc.devkitpro.org/gx_8h.html#acf1f933c4c653e399106e8ac244fabd0) for more.
    pub fn clear_vtx_desc() {
        unsafe {
            ogc_sys::GX_ClearVtxDesc()
        }
    }

    /// Sets the type of a single attribute (attr) in the current vertex descriptor.
    /// See [GX_SetVtxDesc](https://libogc.devkitpro.org/gx_8h.html#af41b45011ae731ae5697b26b2bf97e2f) for more.
    pub fn set_vtx_desc(attr: u8, v_type: u8) {
        unsafe {
            ogc_sys::GX_SetVtxDesc(attr, v_type)
        }
    }

    /// Used to load a 3x4 modelview matrix mt into matrix memory at location pnidx.
    /// See [GX_LoadPosMtxImm](https://libogc.devkitpro.org/gx_8h.html#a90349e713128a1fa4fd6048dcab7b5e7) for more.
    pub fn load_pos_mtx_imm(mut mt: Mtx34, pnidx: u32) {
        unsafe {
            ogc_sys::GX_LoadPosMtxImm(&mut mt[0], pnidx)
        }
    }

    /// Sends a DrawDone command to the GP and stalls until its subsequent execution.
    /// See [GX_DrawDone](https://libogc.devkitpro.org/gx_8h.html#a00f07b60ae2124fe027a82d7d9ae64b0) for more.
    pub fn draw_done() {
        unsafe {
            ogc_sys::GX_DrawDone()
        }
    }

    /// Sets the Z-buffer compare mode.
    /// See [GX_SetZMode](https://libogc.devkitpro.org/gx_8h.html#a2af0d050f56ef45dd25d0db18909fa00) for more.
    pub fn set_z_mode(enable: u8, func: u8, update_enable: u8) {
        unsafe {
            ogc_sys::GX_SetZMode(enable, func, update_enable)
        }
    }

    /// Determines how the source image, generated by the graphics processor, is blended with the Embedded Frame Buffer (EFB).
    /// See [GX_SetBlendMode](https://libogc.devkitpro.org/gx_8h.html#a1d9c43b161f3c5a30b9fd8ea182c8eb6) for more.
    pub fn set_blend_mode(b_type: u8, src_fact: u8, dst_fact: u8, op: u8) {
        unsafe {
            ogc_sys::GX_SetBlendMode(b_type, src_fact, dst_fact, op)
        }
    }

    /// Enables or disables alpha-buffer updates of the Embedded Frame Buffer (EFB).
    /// See [GX_SetAlphaUpdate](https://libogc.devkitpro.org/gx_8h.html#ac238051bda896c8bb11802184882a2a0) for more.
    pub fn set_alpha_update(enable: u8) {
        unsafe {
            ogc_sys::GX_SetAlphaUpdate(enable)
        }
    }

    /// Enables or disables color-buffer updates when rendering into the Embedded Frame Buffer (EFB).
    /// See [GX_SetColorUpdate](https://libogc.devkitpro.org/gx_8h.html#a3978e3b08198e52d7cea411e90ece3e5) for more.
    pub fn set_color_update(enable: u8) {
        unsafe {
            ogc_sys::GX_SetColorUpdate(enable)
        }
    }

    pub fn tex_coord_2f32(s: f32, t: f32) {
        unsafe {
            ogc_sys::GX_TexCoord2f32(s, t)
        }
    }
}

/// Contains all constants used with GX.
pub mod constants {
    pub const GX_FALSE: u32 = 0;
    pub const GX_TRUE: u32 = 1;
    pub const GX_DISABLE: u32 = 0;
    pub const GX_ENABLE: u32 = 1;
    pub const GX_CLIP_DISABLE: u32 = 1;
    pub const GX_CLIP_ENABLE: u32 = 0;
    pub const GX_FIFO_MINSIZE: u32 = 65536;
    pub const GX_FIFO_HIWATERMARK: u32 = 16384;
    pub const GX_FIFO_OBJSIZE: u32 = 128;
    pub const GX_PERSPECTIVE: u32 = 0;
    pub const GX_ORTHOGRAPHIC: u32 = 1;
    pub const GX_MT_NULL: u32 = 0;
    pub const GX_MT_XF_FLUSH: u32 = 1;
    pub const GX_MT_DL_SAVE_CTX: u32 = 2;
    pub const GX_XF_FLUSH_NONE: u32 = 0;
    pub const GX_XF_FLUSH_SAFE: u32 = 1;
    pub const GX_COLOR0: u32 = 0;
    pub const GX_COLOR1: u32 = 1;
    pub const GX_ALPHA0: u32 = 2;
    pub const GX_ALPHA1: u32 = 3;
    pub const GX_COLOR0A0: u32 = 4;
    pub const GX_COLOR1A1: u32 = 5;
    pub const GX_COLORZERO: u32 = 6;
    pub const GX_ALPHA_BUMP: u32 = 7;
    pub const GX_ALPHA_BUMPN: u32 = 8;
    pub const GX_COLORNULL: u32 = 255;
    pub const GX_MTX3x4: u32 = 0;
    pub const GX_MTX2x4: u32 = 1;
    pub const GX_VTXFMT0: u32 = 0;
    pub const GX_VTXFMT1: u32 = 1;
    pub const GX_VTXFMT2: u32 = 2;
    pub const GX_VTXFMT3: u32 = 3;
    pub const GX_VTXFMT4: u32 = 4;
    pub const GX_VTXFMT5: u32 = 5;
    pub const GX_VTXFMT6: u32 = 6;
    pub const GX_VTXFMT7: u32 = 7;
    pub const GX_MAXVTXFMT: u32 = 8;
    pub const GX_NONE: u32 = 0;
    pub const GX_DIRECT: u32 = 1;
    pub const GX_INDEX8: u32 = 2;
    pub const GX_INDEX16: u32 = 3;
    pub const GX_U8: u32 = 0;
    pub const GX_S8: u32 = 1;
    pub const GX_U16: u32 = 2;
    pub const GX_S16: u32 = 3;
    pub const GX_F32: u32 = 4;
    pub const GX_RGB565: u32 = 0;
    pub const GX_RGB8: u32 = 1;
    pub const GX_RGBX8: u32 = 2;
    pub const GX_RGBA4: u32 = 3;
    pub const GX_RGBA6: u32 = 4;
    pub const GX_RGBA8: u32 = 5;
    pub const GX_POS_XY: u32 = 0;
    pub const GX_POS_XYZ: u32 = 1;
    pub const GX_NRM_XYZ: u32 = 0;
    pub const GX_NRM_NBT: u32 = 1;
    pub const GX_NRM_NBT3: u32 = 2;
    pub const GX_CLR_RGB: u32 = 0;
    pub const GX_CLR_RGBA: u32 = 1;
    pub const GX_TEX_S: u32 = 0;
    pub const GX_TEX_ST: u32 = 1;
    pub const GX_VA_PTNMTXIDX: u32 = 0;
    pub const GX_VA_TEX0MTXIDX: u32 = 1;
    pub const GX_VA_TEX1MTXIDX: u32 = 2;
    pub const GX_VA_TEX2MTXIDX: u32 = 3;
    pub const GX_VA_TEX3MTXIDX: u32 = 4;
    pub const GX_VA_TEX4MTXIDX: u32 = 5;
    pub const GX_VA_TEX5MTXIDX: u32 = 6;
    pub const GX_VA_TEX6MTXIDX: u32 = 7;
    pub const GX_VA_TEX7MTXIDX: u32 = 8;
    pub const GX_VA_POS: u32 = 9;
    pub const GX_VA_NRM: u32 = 10;
    pub const GX_VA_CLR0: u32 = 11;
    pub const GX_VA_CLR1: u32 = 12;
    pub const GX_VA_TEX0: u32 = 13;
    pub const GX_VA_TEX1: u32 = 14;
    pub const GX_VA_TEX2: u32 = 15;
    pub const GX_VA_TEX3: u32 = 16;
    pub const GX_VA_TEX4: u32 = 17;
    pub const GX_VA_TEX5: u32 = 18;
    pub const GX_VA_TEX6: u32 = 19;
    pub const GX_VA_TEX7: u32 = 20;
    pub const GX_POSMTXARRAY: u32 = 21;
    pub const GX_NRMMTXARRAY: u32 = 22;
    pub const GX_TEXMTXARRAY: u32 = 23;
    pub const GX_LIGHTARRAY: u32 = 24;
    pub const GX_VA_NBT: u32 = 25;
    pub const GX_VA_MAXATTR: u32 = 26;
    pub const GX_VA_NULL: u32 = 255;
    pub const GX_POINTS: u32 = 184;
    pub const GX_LINES: u32 = 168;
    pub const GX_LINESTRIP: u32 = 176;
    pub const GX_TRIANGLES: u32 = 144;
    pub const GX_TRIANGLESTRIP: u32 = 152;
    pub const GX_TRIANGLEFAN: u32 = 160;
    pub const GX_QUADS: u32 = 128;
    pub const GX_SRC_REG: u32 = 0;
    pub const GX_SRC_VTX: u32 = 1;
    pub const GX_LIGHT0: u32 = 1;
    pub const GX_LIGHT1: u32 = 2;
    pub const GX_LIGHT2: u32 = 4;
    pub const GX_LIGHT3: u32 = 8;
    pub const GX_LIGHT4: u32 = 16;
    pub const GX_LIGHT5: u32 = 32;
    pub const GX_LIGHT6: u32 = 64;
    pub const GX_LIGHT7: u32 = 128;
    pub const GX_MAXLIGHT: u32 = 256;
    pub const GX_LIGHTNULL: u32 = 0;
    pub const GX_DF_NONE: u32 = 0;
    pub const GX_DF_SIGNED: u32 = 1;
    pub const GX_DF_CLAMP: u32 = 2;
    pub const GX_AF_SPEC: u32 = 0;
    pub const GX_AF_SPOT: u32 = 1;
    pub const GX_AF_NONE: u32 = 2;
    pub const GX_PNMTX0: u32 = 0;
    pub const GX_PNMTX1: u32 = 3;
    pub const GX_PNMTX2: u32 = 6;
    pub const GX_PNMTX3: u32 = 9;
    pub const GX_PNMTX4: u32 = 12;
    pub const GX_PNMTX5: u32 = 15;
    pub const GX_PNMTX6: u32 = 18;
    pub const GX_PNMTX7: u32 = 21;
    pub const GX_PNMTX8: u32 = 24;
    pub const GX_PNMTX9: u32 = 27;
    pub const GX_TEXMTX0: u32 = 30;
    pub const GX_TEXMTX1: u32 = 33;
    pub const GX_TEXMTX2: u32 = 36;
    pub const GX_TEXMTX3: u32 = 39;
    pub const GX_TEXMTX4: u32 = 42;
    pub const GX_TEXMTX5: u32 = 45;
    pub const GX_TEXMTX6: u32 = 48;
    pub const GX_TEXMTX7: u32 = 51;
    pub const GX_TEXMTX8: u32 = 54;
    pub const GX_TEXMTX9: u32 = 57;
    pub const GX_IDENTITY: u32 = 60;
    pub const GX_DTTMTX0: u32 = 64;
    pub const GX_DTTMTX1: u32 = 67;
    pub const GX_DTTMTX2: u32 = 70;
    pub const GX_DTTMTX3: u32 = 73;
    pub const GX_DTTMTX4: u32 = 76;
    pub const GX_DTTMTX5: u32 = 79;
    pub const GX_DTTMTX6: u32 = 82;
    pub const GX_DTTMTX7: u32 = 85;
    pub const GX_DTTMTX8: u32 = 88;
    pub const GX_DTTMTX9: u32 = 91;
    pub const GX_DTTMTX10: u32 = 94;
    pub const GX_DTTMTX11: u32 = 97;
    pub const GX_DTTMTX12: u32 = 100;
    pub const GX_DTTMTX13: u32 = 103;
    pub const GX_DTTMTX14: u32 = 106;
    pub const GX_DTTMTX15: u32 = 109;
    pub const GX_DTTMTX16: u32 = 112;
    pub const GX_DTTMTX17: u32 = 115;
    pub const GX_DTTMTX18: u32 = 118;
    pub const GX_DTTMTX19: u32 = 121;
    pub const GX_DTTIDENTITY: u32 = 125;
    pub const GX_TEXCOORD0: u32 = 0;
    pub const GX_TEXCOORD1: u32 = 1;
    pub const GX_TEXCOORD2: u32 = 2;
    pub const GX_TEXCOORD3: u32 = 3;
    pub const GX_TEXCOORD4: u32 = 4;
    pub const GX_TEXCOORD5: u32 = 5;
    pub const GX_TEXCOORD6: u32 = 6;
    pub const GX_TEXCOORD7: u32 = 7;
    pub const GX_MAXCOORD: u32 = 8;
    pub const GX_TEXCOORDNULL: u32 = 255;
    pub const _GX_TF_ZTF: u32 = 16;
    pub const _GX_TF_CTF: u32 = 32;
    pub const GX_TF_I4: u32 = 0;
    pub const GX_TF_I8: u32 = 1;
    pub const GX_TF_IA4: u32 = 2;
    pub const GX_TF_IA8: u32 = 3;
    pub const GX_TF_RGB565: u32 = 4;
    pub const GX_TF_RGB5A3: u32 = 5;
    pub const GX_TF_RGBA8: u32 = 6;
    pub const GX_TF_CI4: u32 = 8;
    pub const GX_TF_CI8: u32 = 9;
    pub const GX_TF_CI14: u32 = 10;
    pub const GX_TF_CMPR: u32 = 14;
    pub const GX_TL_IA8: u32 = 0;
    pub const GX_TL_RGB565: u32 = 1;
    pub const GX_TL_RGB5A3: u32 = 2;
    pub const GX_CTF_R4: u32 = 32;
    pub const GX_CTF_RA4: u32 = 34;
    pub const GX_CTF_RA8: u32 = 35;
    pub const GX_CTF_YUVA8: u32 = 38;
    pub const GX_CTF_A8: u32 = 39;
    pub const GX_CTF_R8: u32 = 40;
    pub const GX_CTF_G8: u32 = 41;
    pub const GX_CTF_B8: u32 = 42;
    pub const GX_CTF_RG8: u32 = 43;
    pub const GX_CTF_GB8: u32 = 44;
    pub const GX_TF_Z8: u32 = 17;
    pub const GX_TF_Z16: u32 = 19;
    pub const GX_TF_Z24X8: u32 = 22;
    pub const GX_CTF_Z4: u32 = 48;
    pub const GX_CTF_Z8M: u32 = 57;
    pub const GX_CTF_Z8L: u32 = 58;
    pub const GX_CTF_Z16L: u32 = 60;
    pub const GX_TF_A8: u32 = 39;
    pub const GX_TLUT_16: u32 = 1;
    pub const GX_TLUT_32: u32 = 2;
    pub const GX_TLUT_64: u32 = 4;
    pub const GX_TLUT_128: u32 = 8;
    pub const GX_TLUT_256: u32 = 16;
    pub const GX_TLUT_512: u32 = 32;
    pub const GX_TLUT_1K: u32 = 64;
    pub const GX_TLUT_2K: u32 = 128;
    pub const GX_TLUT_4K: u32 = 256;
    pub const GX_TLUT_8K: u32 = 512;
    pub const GX_TLUT_16K: u32 = 1024;
    pub const GX_ZT_DISABLE: u32 = 0;
    pub const GX_ZT_ADD: u32 = 1;
    pub const GX_ZT_REPLACE: u32 = 2;
    pub const GX_MAX_ZTEXOP: u32 = 3;
    pub const GX_TG_MTX3x4: u32 = 0;
    pub const GX_TG_MTX2x4: u32 = 1;
    pub const GX_TG_BUMP0: u32 = 2;
    pub const GX_TG_BUMP1: u32 = 3;
    pub const GX_TG_BUMP2: u32 = 4;
    pub const GX_TG_BUMP3: u32 = 5;
    pub const GX_TG_BUMP4: u32 = 6;
    pub const GX_TG_BUMP5: u32 = 7;
    pub const GX_TG_BUMP6: u32 = 8;
    pub const GX_TG_BUMP7: u32 = 9;
    pub const GX_TG_SRTG: u32 = 10;
    pub const GX_TG_POS: u32 = 0;
    pub const GX_TG_NRM: u32 = 1;
    pub const GX_TG_BINRM: u32 = 2;
    pub const GX_TG_TANGENT: u32 = 3;
    pub const GX_TG_TEX0: u32 = 4;
    pub const GX_TG_TEX1: u32 = 5;
    pub const GX_TG_TEX2: u32 = 6;
    pub const GX_TG_TEX3: u32 = 7;
    pub const GX_TG_TEX4: u32 = 8;
    pub const GX_TG_TEX5: u32 = 9;
    pub const GX_TG_TEX6: u32 = 10;
    pub const GX_TG_TEX7: u32 = 11;
    pub const GX_TG_TEXCOORD0: u32 = 12;
    pub const GX_TG_TEXCOORD1: u32 = 13;
    pub const GX_TG_TEXCOORD2: u32 = 14;
    pub const GX_TG_TEXCOORD3: u32 = 15;
    pub const GX_TG_TEXCOORD4: u32 = 16;
    pub const GX_TG_TEXCOORD5: u32 = 17;
    pub const GX_TG_TEXCOORD6: u32 = 18;
    pub const GX_TG_COLOR0: u32 = 19;
    pub const GX_TG_COLOR1: u32 = 20;
    pub const GX_NEVER: u32 = 0;
    pub const GX_LESS: u32 = 1;
    pub const GX_EQUAL: u32 = 2;
    pub const GX_LEQUAL: u32 = 3;
    pub const GX_GREATER: u32 = 4;
    pub const GX_NEQUAL: u32 = 5;
    pub const GX_GEQUAL: u32 = 6;
    pub const GX_ALWAYS: u32 = 7;
    pub const GX_CLAMP: u32 = 0;
    pub const GX_REPEAT: u32 = 1;
    pub const GX_MIRROR: u32 = 2;
    pub const GX_MAXTEXWRAPMODE: u32 = 3;
    pub const GX_BM_NONE: u32 = 0;
    pub const GX_BM_BLEND: u32 = 1;
    pub const GX_BM_LOGIC: u32 = 2;
    pub const GX_BM_SUBTRACT: u32 = 3;
    pub const GX_MAX_BLENDMODE: u32 = 4;
    pub const GX_BL_ZERO: u32 = 0;
    pub const GX_BL_ONE: u32 = 1;
    pub const GX_BL_SRCCLR: u32 = 2;
    pub const GX_BL_INVSRCCLR: u32 = 3;
    pub const GX_BL_SRCALPHA: u32 = 4;
    pub const GX_BL_INVSRCALPHA: u32 = 5;
    pub const GX_BL_DSTALPHA: u32 = 6;
    pub const GX_BL_INVDSTALPHA: u32 = 7;
    pub const GX_BL_DSTCLR: u32 = 2;
    pub const GX_BL_INVDSTCLR: u32 = 3;
    pub const GX_LO_CLEAR: u32 = 0;
    pub const GX_LO_AND: u32 = 1;
    pub const GX_LO_REVAND: u32 = 2;
    pub const GX_LO_COPY: u32 = 3;
    pub const GX_LO_INVAND: u32 = 4;
    pub const GX_LO_NOOP: u32 = 5;
    pub const GX_LO_XOR: u32 = 6;
    pub const GX_LO_OR: u32 = 7;
    pub const GX_LO_NOR: u32 = 8;
    pub const GX_LO_EQUIV: u32 = 9;
    pub const GX_LO_INV: u32 = 10;
    pub const GX_LO_REVOR: u32 = 11;
    pub const GX_LO_INVCOPY: u32 = 12;
    pub const GX_LO_INVOR: u32 = 13;
    pub const GX_LO_NAND: u32 = 14;
    pub const GX_LO_SET: u32 = 15;
    pub const GX_TO_ZERO: u32 = 0;
    pub const GX_TO_SIXTEENTH: u32 = 1;
    pub const GX_TO_EIGHTH: u32 = 2;
    pub const GX_TO_FOURTH: u32 = 3;
    pub const GX_TO_HALF: u32 = 4;
    pub const GX_TO_ONE: u32 = 5;
    pub const GX_MAX_TEXOFFSET: u32 = 6;
    pub const GX_MODULATE: u32 = 0;
    pub const GX_DECAL: u32 = 1;
    pub const GX_BLEND: u32 = 2;
    pub const GX_REPLACE: u32 = 3;
    pub const GX_PASSCLR: u32 = 4;
    pub const GX_CC_CPREV: u32 = 0;
    pub const GX_CC_APREV: u32 = 1;
    pub const GX_CC_C0: u32 = 2;
    pub const GX_CC_A0: u32 = 3;
    pub const GX_CC_C1: u32 = 4;
    pub const GX_CC_A1: u32 = 5;
    pub const GX_CC_C2: u32 = 6;
    pub const GX_CC_A2: u32 = 7;
    pub const GX_CC_TEXC: u32 = 8;
    pub const GX_CC_TEXA: u32 = 9;
    pub const GX_CC_RASC: u32 = 10;
    pub const GX_CC_RASA: u32 = 11;
    pub const GX_CC_ONE: u32 = 12;
    pub const GX_CC_HALF: u32 = 13;
    pub const GX_CC_KONST: u32 = 14;
    pub const GX_CC_ZERO: u32 = 15;
    pub const GX_CA_APREV: u32 = 0;
    pub const GX_CA_A0: u32 = 1;
    pub const GX_CA_A1: u32 = 2;
    pub const GX_CA_A2: u32 = 3;
    pub const GX_CA_TEXA: u32 = 4;
    pub const GX_CA_RASA: u32 = 5;
    pub const GX_CA_KONST: u32 = 6;
    pub const GX_CA_ZERO: u32 = 7;
    pub const GX_TEVSTAGE0: u32 = 0;
    pub const GX_TEVSTAGE1: u32 = 1;
    pub const GX_TEVSTAGE2: u32 = 2;
    pub const GX_TEVSTAGE3: u32 = 3;
    pub const GX_TEVSTAGE4: u32 = 4;
    pub const GX_TEVSTAGE5: u32 = 5;
    pub const GX_TEVSTAGE6: u32 = 6;
    pub const GX_TEVSTAGE7: u32 = 7;
    pub const GX_TEVSTAGE8: u32 = 8;
    pub const GX_TEVSTAGE9: u32 = 9;
    pub const GX_TEVSTAGE10: u32 = 10;
    pub const GX_TEVSTAGE11: u32 = 11;
    pub const GX_TEVSTAGE12: u32 = 12;
    pub const GX_TEVSTAGE13: u32 = 13;
    pub const GX_TEVSTAGE14: u32 = 14;
    pub const GX_TEVSTAGE15: u32 = 15;
    pub const GX_MAX_TEVSTAGE: u32 = 16;
    pub const GX_TEV_ADD: u32 = 0;
    pub const GX_TEV_SUB: u32 = 1;
    pub const GX_TEV_COMP_R8_GT: u32 = 8;
    pub const GX_TEV_COMP_R8_EQ: u32 = 9;
    pub const GX_TEV_COMP_GR16_GT: u32 = 10;
    pub const GX_TEV_COMP_GR16_EQ: u32 = 11;
    pub const GX_TEV_COMP_BGR24_GT: u32 = 12;
    pub const GX_TEV_COMP_BGR24_EQ: u32 = 13;
    pub const GX_TEV_COMP_RGB8_GT: u32 = 14;
    pub const GX_TEV_COMP_RGB8_EQ: u32 = 15;
    pub const GX_TEV_COMP_A8_GT: u32 = 14;
    pub const GX_TEV_COMP_A8_EQ: u32 = 15;
    pub const GX_TB_ZERO: u32 = 0;
    pub const GX_TB_ADDHALF: u32 = 1;
    pub const GX_TB_SUBHALF: u32 = 2;
    pub const GX_MAX_TEVBIAS: u32 = 3;
    pub const GX_TC_LINEAR: u32 = 0;
    pub const GX_TC_GE: u32 = 1;
    pub const GX_TC_EQ: u32 = 2;
    pub const GX_TC_LE: u32 = 3;
    pub const GX_MAX_TEVCLAMPMODE: u32 = 4;
    pub const GX_CS_SCALE_1: u32 = 0;
    pub const GX_CS_SCALE_2: u32 = 1;
    pub const GX_CS_SCALE_4: u32 = 2;
    pub const GX_CS_DIVIDE_2: u32 = 3;
    pub const GX_MAX_TEVSCALE: u32 = 4;
    pub const GX_TEVPREV: u32 = 0;
    pub const GX_TEVREG0: u32 = 1;
    pub const GX_TEVREG1: u32 = 2;
    pub const GX_TEVREG2: u32 = 3;
    pub const GX_MAX_TEVREG: u32 = 4;
    pub const GX_CULL_NONE: u32 = 0;
    pub const GX_CULL_FRONT: u32 = 1;
    pub const GX_CULL_BACK: u32 = 2;
    pub const GX_CULL_ALL: u32 = 3;
    pub const GX_TEXMAP0: u32 = 0;
    pub const GX_TEXMAP1: u32 = 1;
    pub const GX_TEXMAP2: u32 = 2;
    pub const GX_TEXMAP3: u32 = 3;
    pub const GX_TEXMAP4: u32 = 4;
    pub const GX_TEXMAP5: u32 = 5;
    pub const GX_TEXMAP6: u32 = 6;
    pub const GX_TEXMAP7: u32 = 7;
    pub const GX_MAX_TEXMAP: u32 = 8;
    pub const GX_TEXMAP_NULL: u32 = 255;
    pub const GX_TEXMAP_DISABLE: u32 = 256;
    pub const GX_AOP_AND: u32 = 0;
    pub const GX_AOP_OR: u32 = 1;
    pub const GX_AOP_XOR: u32 = 2;
    pub const GX_AOP_XNOR: u32 = 3;
    pub const GX_MAX_ALPHAOP: u32 = 4;
    pub const GX_KCOLOR0: u32 = 0;
    pub const GX_KCOLOR1: u32 = 1;
    pub const GX_KCOLOR2: u32 = 2;
    pub const GX_KCOLOR3: u32 = 3;
    pub const GX_KCOLOR_MAX: u32 = 4;
    pub const GX_TEV_KCSEL_1: u32 = 0;
    pub const GX_TEV_KCSEL_7_8: u32 = 1;
    pub const GX_TEV_KCSEL_3_4: u32 = 2;
    pub const GX_TEV_KCSEL_5_8: u32 = 3;
    pub const GX_TEV_KCSEL_1_2: u32 = 4;
    pub const GX_TEV_KCSEL_3_8: u32 = 5;
    pub const GX_TEV_KCSEL_1_4: u32 = 6;
    pub const GX_TEV_KCSEL_1_8: u32 = 7;
    pub const GX_TEV_KCSEL_K0: u32 = 12;
    pub const GX_TEV_KCSEL_K1: u32 = 13;
    pub const GX_TEV_KCSEL_K2: u32 = 14;
    pub const GX_TEV_KCSEL_K3: u32 = 15;
    pub const GX_TEV_KCSEL_K0_R: u32 = 16;
    pub const GX_TEV_KCSEL_K1_R: u32 = 17;
    pub const GX_TEV_KCSEL_K2_R: u32 = 18;
    pub const GX_TEV_KCSEL_K3_R: u32 = 19;
    pub const GX_TEV_KCSEL_K0_G: u32 = 20;
    pub const GX_TEV_KCSEL_K1_G: u32 = 21;
    pub const GX_TEV_KCSEL_K2_G: u32 = 22;
    pub const GX_TEV_KCSEL_K3_G: u32 = 23;
    pub const GX_TEV_KCSEL_K0_B: u32 = 24;
    pub const GX_TEV_KCSEL_K1_B: u32 = 25;
    pub const GX_TEV_KCSEL_K2_B: u32 = 26;
    pub const GX_TEV_KCSEL_K3_B: u32 = 27;
    pub const GX_TEV_KCSEL_K0_A: u32 = 28;
    pub const GX_TEV_KCSEL_K1_A: u32 = 29;
    pub const GX_TEV_KCSEL_K2_A: u32 = 30;
    pub const GX_TEV_KCSEL_K3_A: u32 = 31;
    pub const GX_TEV_KASEL_1: u32 = 0;
    pub const GX_TEV_KASEL_7_8: u32 = 1;
    pub const GX_TEV_KASEL_3_4: u32 = 2;
    pub const GX_TEV_KASEL_5_8: u32 = 3;
    pub const GX_TEV_KASEL_1_2: u32 = 4;
    pub const GX_TEV_KASEL_3_8: u32 = 5;
    pub const GX_TEV_KASEL_1_4: u32 = 6;
    pub const GX_TEV_KASEL_1_8: u32 = 7;
    pub const GX_TEV_KASEL_K0_R: u32 = 16;
    pub const GX_TEV_KASEL_K1_R: u32 = 17;
    pub const GX_TEV_KASEL_K2_R: u32 = 18;
    pub const GX_TEV_KASEL_K3_R: u32 = 19;
    pub const GX_TEV_KASEL_K0_G: u32 = 20;
    pub const GX_TEV_KASEL_K1_G: u32 = 21;
    pub const GX_TEV_KASEL_K2_G: u32 = 22;
    pub const GX_TEV_KASEL_K3_G: u32 = 23;
    pub const GX_TEV_KASEL_K0_B: u32 = 24;
    pub const GX_TEV_KASEL_K1_B: u32 = 25;
    pub const GX_TEV_KASEL_K2_B: u32 = 26;
    pub const GX_TEV_KASEL_K3_B: u32 = 27;
    pub const GX_TEV_KASEL_K0_A: u32 = 28;
    pub const GX_TEV_KASEL_K1_A: u32 = 29;
    pub const GX_TEV_KASEL_K2_A: u32 = 30;
    pub const GX_TEV_KASEL_K3_A: u32 = 31;
    pub const GX_TEV_SWAP0: u32 = 0;
    pub const GX_TEV_SWAP1: u32 = 1;
    pub const GX_TEV_SWAP2: u32 = 2;
    pub const GX_TEV_SWAP3: u32 = 3;
    pub const GX_MAX_TEVSWAP: u32 = 4;
    pub const GX_CH_RED: u32 = 0;
    pub const GX_CH_GREEN: u32 = 1;
    pub const GX_CH_BLUE: u32 = 2;
    pub const GX_CH_ALPHA: u32 = 3;
    pub const GX_INDTEXSTAGE0: u32 = 0;
    pub const GX_INDTEXSTAGE1: u32 = 1;
    pub const GX_INDTEXSTAGE2: u32 = 2;
    pub const GX_INDTEXSTAGE3: u32 = 3;
    pub const GX_MAX_INDTEXSTAGE: u32 = 4;
    pub const GX_ITF_8: u32 = 0;
    pub const GX_ITF_5: u32 = 1;
    pub const GX_ITF_4: u32 = 2;
    pub const GX_ITF_3: u32 = 3;
    pub const GX_MAX_ITFORMAT: u32 = 4;
    pub const GX_ITB_NONE: u32 = 0;
    pub const GX_ITB_S: u32 = 1;
    pub const GX_ITB_T: u32 = 2;
    pub const GX_ITB_ST: u32 = 3;
    pub const GX_ITB_U: u32 = 4;
    pub const GX_ITB_SU: u32 = 5;
    pub const GX_ITB_TU: u32 = 6;
    pub const GX_ITB_STU: u32 = 7;
    pub const GX_MAX_ITBIAS: u32 = 8;
    pub const GX_ITM_OFF: u32 = 0;
    pub const GX_ITM_0: u32 = 1;
    pub const GX_ITM_1: u32 = 2;
    pub const GX_ITM_2: u32 = 3;
    pub const GX_ITM_S0: u32 = 5;
    pub const GX_ITM_S1: u32 = 6;
    pub const GX_ITM_S2: u32 = 7;
    pub const GX_ITM_T0: u32 = 9;
    pub const GX_ITM_T1: u32 = 10;
    pub const GX_ITM_T2: u32 = 11;
    pub const GX_ITW_OFF: u32 = 0;
    pub const GX_ITW_256: u32 = 1;
    pub const GX_ITW_128: u32 = 2;
    pub const GX_ITW_64: u32 = 3;
    pub const GX_ITW_32: u32 = 4;
    pub const GX_ITW_16: u32 = 5;
    pub const GX_ITW_0: u32 = 6;
    pub const GX_MAX_ITWRAP: u32 = 7;
    pub const GX_ITBA_OFF: u32 = 0;
    pub const GX_ITBA_S: u32 = 1;
    pub const GX_ITBA_T: u32 = 2;
    pub const GX_ITBA_U: u32 = 3;
    pub const GX_MAX_ITBALPHA: u32 = 4;
    pub const GX_ITS_1: u32 = 0;
    pub const GX_ITS_2: u32 = 1;
    pub const GX_ITS_4: u32 = 2;
    pub const GX_ITS_8: u32 = 3;
    pub const GX_ITS_16: u32 = 4;
    pub const GX_ITS_32: u32 = 5;
    pub const GX_ITS_64: u32 = 6;
    pub const GX_ITS_128: u32 = 7;
    pub const GX_ITS_256: u32 = 8;
    pub const GX_MAX_ITSCALE: u32 = 9;
    pub const GX_FOG_NONE: u32 = 0;
    pub const GX_FOG_PERSP_LIN: u32 = 2;
    pub const GX_FOG_PERSP_EXP: u32 = 4;
    pub const GX_FOG_PERSP_EXP2: u32 = 5;
    pub const GX_FOG_PERSP_REVEXP: u32 = 6;
    pub const GX_FOG_PERSP_REVEXP2: u32 = 7;
    pub const GX_FOG_ORTHO_LIN: u32 = 10;
    pub const GX_FOG_ORTHO_EXP: u32 = 12;
    pub const GX_FOG_ORTHO_EXP2: u32 = 13;
    pub const GX_FOG_ORTHO_REVEXP: u32 = 14;
    pub const GX_FOG_ORTHO_REVEXP2: u32 = 15;
    pub const GX_FOG_LIN: u32 = 2;
    pub const GX_FOG_EXP: u32 = 4;
    pub const GX_FOG_EXP2: u32 = 5;
    pub const GX_FOG_REVEXP: u32 = 6;
    pub const GX_FOG_REVEXP2: u32 = 7;
    pub const GX_PF_RGB8_Z24: u32 = 0;
    pub const GX_PF_RGBA6_Z24: u32 = 1;
    pub const GX_PF_RGB565_Z16: u32 = 2;
    pub const GX_PF_Z24: u32 = 3;
    pub const GX_PF_Y8: u32 = 4;
    pub const GX_PF_U8: u32 = 5;
    pub const GX_PF_V8: u32 = 6;
    pub const GX_PF_YUV420: u32 = 7;
    pub const GX_ZC_LINEAR: u32 = 0;
    pub const GX_ZC_NEAR: u32 = 1;
    pub const GX_ZC_MID: u32 = 2;
    pub const GX_ZC_FAR: u32 = 3;
    pub const GX_CLAMP_NONE: u32 = 0;
    pub const GX_CLAMP_TOP: u32 = 1;
    pub const GX_CLAMP_BOTTOM: u32 = 2;
    pub const GX_GM_1_0: u32 = 0;
    pub const GX_GM_1_7: u32 = 1;
    pub const GX_GM_2_2: u32 = 2;
    pub const GX_COPY_PROGRESSIVE: u32 = 0;
    pub const GX_COPY_INTLC_EVEN: u32 = 2;
    pub const GX_COPY_INTLC_ODD: u32 = 3;
    pub const GX_READ_00: u32 = 0;
    pub const GX_READ_FF: u32 = 1;
    pub const GX_READ_NONE: u32 = 2;
    pub const GX_TEXCACHE_32K: u32 = 0;
    pub const GX_TEXCACHE_128K: u32 = 1;
    pub const GX_TEXCACHE_512K: u32 = 2;
    pub const GX_TEXCACHE_NONE: u32 = 3;
    pub const GX_DA_OFF: u32 = 0;
    pub const GX_DA_GENTLE: u32 = 1;
    pub const GX_DA_MEDIUM: u32 = 2;
    pub const GX_DA_STEEP: u32 = 3;
    pub const GX_SP_OFF: u32 = 0;
    pub const GX_SP_FLAT: u32 = 1;
    pub const GX_SP_COS: u32 = 2;
    pub const GX_SP_COS2: u32 = 3;
    pub const GX_SP_SHARP: u32 = 4;
    pub const GX_SP_RING1: u32 = 5;
    pub const GX_SP_RING2: u32 = 6;
    pub const GX_NEAR: u32 = 0;
    pub const GX_LINEAR: u32 = 1;
    pub const GX_NEAR_MIP_NEAR: u32 = 2;
    pub const GX_LIN_MIP_NEAR: u32 = 3;
    pub const GX_NEAR_MIP_LIN: u32 = 4;
    pub const GX_LIN_MIP_LIN: u32 = 5;
    pub const GX_ANISO_1: u32 = 0;
    pub const GX_ANISO_2: u32 = 1;
    pub const GX_ANISO_4: u32 = 2;
    pub const GX_MAX_ANISOTROPY: u32 = 3;
    pub const GX_VC_POS: u32 = 0;
    pub const GX_VC_NRM: u32 = 1;
    pub const GX_VC_CLR0: u32 = 2;
    pub const GX_VC_CLR1: u32 = 3;
    pub const GX_VC_TEX0: u32 = 4;
    pub const GX_VC_TEX1: u32 = 5;
    pub const GX_VC_TEX2: u32 = 6;
    pub const GX_VC_TEX3: u32 = 7;
    pub const GX_VC_TEX4: u32 = 8;
    pub const GX_VC_TEX5: u32 = 9;
    pub const GX_VC_TEX6: u32 = 10;
    pub const GX_VC_TEX7: u32 = 11;
    pub const GX_VC_ALL: u32 = 15;
    pub const GX_PERF0_VERTICES: u32 = 0;
    pub const GX_PERF0_CLIP_VTX: u32 = 1;
    pub const GX_PERF0_CLIP_CLKS: u32 = 2;
    pub const GX_PERF0_XF_WAIT_IN: u32 = 3;
    pub const GX_PERF0_XF_WAIT_OUT: u32 = 4;
    pub const GX_PERF0_XF_XFRM_CLKS: u32 = 5;
    pub const GX_PERF0_XF_LIT_CLKS: u32 = 6;
    pub const GX_PERF0_XF_BOT_CLKS: u32 = 7;
    pub const GX_PERF0_XF_REGLD_CLKS: u32 = 8;
    pub const GX_PERF0_XF_REGRD_CLKS: u32 = 9;
    pub const GX_PERF0_CLIP_RATIO: u32 = 10;
    pub const GX_PERF0_TRIANGLES: u32 = 11;
    pub const GX_PERF0_TRIANGLES_CULLED: u32 = 12;
    pub const GX_PERF0_TRIANGLES_PASSED: u32 = 13;
    pub const GX_PERF0_TRIANGLES_SCISSORED: u32 = 14;
    pub const GX_PERF0_TRIANGLES_0TEX: u32 = 15;
    pub const GX_PERF0_TRIANGLES_1TEX: u32 = 16;
    pub const GX_PERF0_TRIANGLES_2TEX: u32 = 17;
    pub const GX_PERF0_TRIANGLES_3TEX: u32 = 18;
    pub const GX_PERF0_TRIANGLES_4TEX: u32 = 19;
    pub const GX_PERF0_TRIANGLES_5TEX: u32 = 20;
    pub const GX_PERF0_TRIANGLES_6TEX: u32 = 21;
    pub const GX_PERF0_TRIANGLES_7TEX: u32 = 22;
    pub const GX_PERF0_TRIANGLES_8TEX: u32 = 23;
    pub const GX_PERF0_TRIANGLES_0CLR: u32 = 24;
    pub const GX_PERF0_TRIANGLES_1CLR: u32 = 25;
    pub const GX_PERF0_TRIANGLES_2CLR: u32 = 26;
    pub const GX_PERF0_QUAD_0CVG: u32 = 27;
    pub const GX_PERF0_QUAD_NON0CVG: u32 = 28;
    pub const GX_PERF0_QUAD_1CVG: u32 = 29;
    pub const GX_PERF0_QUAD_2CVG: u32 = 30;
    pub const GX_PERF0_QUAD_3CVG: u32 = 31;
    pub const GX_PERF0_QUAD_4CVG: u32 = 32;
    pub const GX_PERF0_AVG_QUAD_CNT: u32 = 33;
    pub const GX_PERF0_CLOCKS: u32 = 34;
    pub const GX_PERF0_NONE: u32 = 35;
    pub const GX_PERF1_TEXELS: u32 = 0;
    pub const GX_PERF1_TX_IDLE: u32 = 1;
    pub const GX_PERF1_TX_REGS: u32 = 2;
    pub const GX_PERF1_TX_MEMSTALL: u32 = 3;
    pub const GX_PERF1_TC_CHECK1_2: u32 = 4;
    pub const GX_PERF1_TC_CHECK3_4: u32 = 5;
    pub const GX_PERF1_TC_CHECK5_6: u32 = 6;
    pub const GX_PERF1_TC_CHECK7_8: u32 = 7;
    pub const GX_PERF1_TC_MISS: u32 = 8;
    pub const GX_PERF1_VC_ELEMQ_FULL: u32 = 9;
    pub const GX_PERF1_VC_MISSQ_FULL: u32 = 10;
    pub const GX_PERF1_VC_MEMREQ_FULL: u32 = 11;
    pub const GX_PERF1_VC_STATUS7: u32 = 12;
    pub const GX_PERF1_VC_MISSREP_FULL: u32 = 13;
    pub const GX_PERF1_VC_STREAMBUF_LOW: u32 = 14;
    pub const GX_PERF1_VC_ALL_STALLS: u32 = 15;
    pub const GX_PERF1_VERTICES: u32 = 16;
    pub const GX_PERF1_FIFO_REQ: u32 = 17;
    pub const GX_PERF1_CALL_REQ: u32 = 18;
    pub const GX_PERF1_VC_MISS_REQ: u32 = 19;
    pub const GX_PERF1_CP_ALL_REQ: u32 = 20;
    pub const GX_PERF1_CLOCKS: u32 = 21;
    pub const GX_PERF1_NONE: u32 = 22;
    pub const GX_TLUT0: u32 = 0;
    pub const GX_TLUT1: u32 = 1;
    pub const GX_TLUT2: u32 = 2;
    pub const GX_TLUT3: u32 = 3;
    pub const GX_TLUT4: u32 = 4;
    pub const GX_TLUT5: u32 = 5;
    pub const GX_TLUT6: u32 = 6;
    pub const GX_TLUT7: u32 = 7;
    pub const GX_TLUT8: u32 = 8;
    pub const GX_TLUT9: u32 = 9;
    pub const GX_TLUT10: u32 = 10;
    pub const GX_TLUT11: u32 = 11;
    pub const GX_TLUT12: u32 = 12;
    pub const GX_TLUT13: u32 = 13;
    pub const GX_TLUT14: u32 = 14;
    pub const GX_TLUT15: u32 = 15;
    pub const GX_BIGTLUT0: u32 = 16;
    pub const GX_BIGTLUT1: u32 = 17;
    pub const GX_BIGTLUT2: u32 = 18;
    pub const GX_BIGTLUT3: u32 = 19;
    pub const GX_MAX_VTXDESC: u32 = 26;
    pub const GX_MAX_VTXDESC_LISTSIZE: u32 = 27;
    pub const GX_MAX_VTXATTRFMT: u32 = 26;
    pub const GX_MAX_VTXATTRFMT_LISTSIZE: u32 = 27;
    pub const GX_MAX_Z24: u32 = 16777215;
}
