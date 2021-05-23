pub mod utils {
    pub fn gp_fifo(boundary: usize, fifo_size: usize) -> *mut libc::c_void {
        assert_eq!(32, boundary);
        unsafe {
            let gp_fifo = libc::memalign(boundary, fifo_size);
            libc::memset(gp_fifo, 0, fifo_size);
            gp_fifo
        }
    }
}

pub struct GxColor(u8, u8, u8, u8);

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
    pub fn set_copy_clear(background: GxColor, z_value: u32) {
        let GxColor(r, g, b, a) = background;
        let background = ogc_sys::_gx_color {r, g, b, a};
        unsafe {
            ogc_sys::GX_SetCopyClear(background, z_value)
        }
    }

    /// Sets the viewport rectangle in screen coordinates.
    /// See [GX_SetViewport](https://libogc.devkitpro.org/gx_8h.html#aaccd37675da5a22596fad756c73badc2) for more.
    pub fn set_viewport(x_origin: f32, y_origin: f32, width: f32, height: f32, near_z: f32, far_z: f32) {
        unsafe {
            ogc_sys::GX_SetViewport(x_origin, y_origin, width, height, near_z, far_z)
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
    pub fn set_scissor(x_origin: u32, y_origin: u32, width: u32, height: u32) {
        unsafe {
            ogc_sys::GX_SetScissor(x_origin, y_origin, width, height)
        }
    }

    /// Sets the source parameters for the EFB to XFB copy operation.
    /// See [GX_SetDispCopySrc](https://libogc.devkitpro.org/gx_8h.html#a979d8db7abbbc2e9a267f5d1710ac588) for more.
    pub fn set_disp_copy_src(left: u16, top: u16, width: u16, height: u16) {
        assert_eq!(0, left % 2);
        assert_eq!(0, top % 2);
        assert_eq!(0, width % 2);
        assert_eq!(0, height % 2);
        unsafe {
            ogc_sys::GX_SetDispCopySrc(left, top, width, height)
        }
    }

    /// Sets the witdth and height of the display buffer in pixels.
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
    // pub fn SetFieldMode(rmode->field_rendering,((rmode->viHeight==2*rmode->xfbHeight)?GX_ENABLE:GX_DISABLE)) {}
    // pub fn SetPixelFmt(GX_PF_RGB565_Z16, GX_ZC_LINEAR) {}
    // pub fn SetPixelFmt(GX_PF_RGB8_Z24, GX_ZC_LINEAR) {}
    // pub fn SetCullMode(GX_CULL_NONE) {}
    // pub fn CopyDisp(frameBuffer[fb],GX_TRUE) {}
    // pub fn SetDispCopyGamma(GX_GM_1_0) {}
    // pub fn SetVtxAttrFmt(GX_VTXFMT0, GX_VA_POS, GX_POS_XY, GX_F32, 0) {}
    // pub fn SetVtxAttrFmt(GX_VTXFMT0, GX_VA_TEX0, GX_TEX_ST, GX_F32, 0) {}
    // pub fn SetNumChans(1) {}
    // pub fn SetNumTexGens(1) {}
    // pub fn SetTevOp(GX_TEVSTAGE0, GX_REPLACE) {}
    // pub fn SetTevOrder(GX_TEVSTAGE0, GX_TEXCOORD0, GX_TEXMAP0, GX_COLOR0A0) {}
    // pub fn SetTexCoordGen(GX_TEXCOORD0, GX_TG_MTX2x4, GX_TG_TEX0, GX_IDENTITY) {}
    // pub fn InvalidateTexAll() {}
    // pub fn LoadTexObj(&texObj, GX_TEXMAP0) {}
    // pub fn LoadProjectionMtx(perspective, GX_ORTHOGRAPHIC) {}
    // pub fn InvVtxCache() {}
    // pub fn InvalidateTexAll() {}
    // pub fn ClearVtxDesc() {}
    // pub fn SetVtxDesc(GX_VA_POS, GX_DIRECT) {}
    // pub fn SetVtxDesc(GX_VA_TEX0, GX_DIRECT) {}
    // pub fn LoadPosMtxImm(GXmodelView2D,GX_PNMTX0) {}
    // pub fn DrawDone() {}
    // pub fn SetZMode(GX_TRUE, GX_LEQUAL, GX_TRUE) {}
    // pub fn SetBlendMode(GX_BM_BLEND, GX_BL_SRCALPHA, GX_BL_INVSRCALPHA, GX_LO_CLEAR) {}
    // pub fn SetAlphaUpdate(GX_TRUE) {}
    // pub fn SetColorUpdate(GX_TRUE) {}
    // pub fn CopyDisp(frameBuffer[fb],GX_TRUE) {}
    // pub fn TexCoord2f32(texCoords[texIndex],texCoords[texIndex+1]) {}
}
