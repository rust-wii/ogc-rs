use core::{convert::TryInto, ffi::c_void};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};
use ogc_rs::{
    ffi::{
        GX_CLR_RGBA, GX_COLOR0A0, GX_F32, GX_PASSCLR, GX_PNMTX0, GX_POS_XYZ, GX_RGBA8,
        GX_TEVSTAGE0, GX_TEXCOORD0, GX_TEXMAP0, GX_TEX_ST, GX_VTXFMT0,
    },
    gx::types::{Gamma, PixelFormat, VtxDest, ZFormat},
    prelude::*,
};

pub struct Display;
impl Display {
    pub fn new(fifo_size: usize) -> Self {
        Gx::init(fifo_size);
        Self
    }

    pub fn flush(&self, framebuffer: *mut c_void) {
        Gx::draw_done();
        Gx::set_z_mode(true, CmpFn::LessEq, true);
        unsafe {
            Gx::copy_disp(framebuffer, true);
        }
    }

    pub fn setup(&self, rc: &mut RenderConfig) {
        let mut ident: Mat3x4 = Mat3x4::IDENTITY;
        Gx::set_copy_clear(Color::with_alpha(0, 0, 0, 0), 0x00ffffff);
        Gx::set_pixel_fmt(PixelFormat::RGB8_Z24, ZFormat::LINEAR);
        Gx::set_viewport(
            0.0,
            0.0,
            rc.framebuffer_width as f32,
            rc.embed_framebuffer_height as f32,
            0.0,
            0.0,
        );

        let yscale =
            Gx::get_y_scale_factor(rc.embed_framebuffer_height, rc.extern_framebuffer_height);
        let extern_framebuffer_height = Gx::set_disp_copy_y_scale(yscale) as u16;

        let half_aspect_ratio = rc.vi_height == 2 * rc.extern_framebuffer_height;

        Gx::set_disp_copy_src(0, 0, rc.framebuffer_width, rc.embed_framebuffer_height);
        Gx::set_disp_copy_dst(rc.framebuffer_width, extern_framebuffer_height);
        Gx::set_copy_filter(
            rc.anti_aliasing != 0,
            &mut rc.sample_pattern,
            true,
            &mut rc.v_filter,
        );
        Gx::set_field_mode(rc.field_rendering != 0, half_aspect_ratio);
        Gx::set_disp_copy_gamma(Gamma::ONE_ZERO);

        //Clear VTX
        Gx::clear_vtx_desc();
        Gx::inv_vtx_cache();
        Gx::invalidate_tex_all();

        Gx::set_vtx_desc(VtxAttr::Tex0, VtxDest::NONE);
        Gx::set_vtx_desc(VtxAttr::Pos, VtxDest::DIRECT);
        Gx::set_vtx_desc(VtxAttr::Color0, VtxDest::DIRECT);

        Gx::set_vtx_attr_fmt(0, VtxAttr::Pos, GX_POS_XYZ as _, GX_F32 as _, 0);
        Gx::set_vtx_attr_fmt(0, VtxAttr::Tex0, GX_TEX_ST as _, GX_F32 as _, 0);
        Gx::set_vtx_attr_fmt(0, VtxAttr::Color0, GX_CLR_RGBA as _, GX_RGBA8 as _, 0);
        Gx::set_z_mode(true, CmpFn::LessEq, true);

        Gx::set_num_chans(1);
        Gx::set_num_tex_gens(1);
        Gx::set_tev_op(GX_TEVSTAGE0 as _, GX_PASSCLR as _);
        Gx::set_tev_order(
            GX_TEVSTAGE0 as _,
            GX_TEXCOORD0 as _,
            GX_TEXMAP0 as _,
            GX_COLOR0A0 as _,
        );
        ident.gu_translation_apply((0., 0., -100.));
        ident.load_as_pos_mtx(GX_PNMTX0 as _);
        let mut perspective: Mat4 = Mat4::gu_ortho(
            0.0,
            rc.embed_framebuffer_height as f32,
            0.0,
            rc.framebuffer_width as f32,
            0.0,
            1000.0,
        );
        perspective.load_as_proj_mat(ProjectionType::Orthographic);

        Gx::set_viewport(
            0.0,
            0.0,
            rc.framebuffer_width as f32,
            rc.embed_framebuffer_height as f32,
            0.0,
            1.0,
        );
        Gx::set_blend_mode(
            BlendMode::Blend,
            BlendCtrl::SrcAlpha,
            BlendCtrl::InvSrcAlpha,
            LogicOp::Clear,
        );
        Gx::set_alpha_update(true);
        Gx::set_alpha_compare(CmpFn::Greater, 0, AlphaOp::And, CmpFn::Always, 0);
        Gx::set_color_update(true);
        Gx::set_cull_mode(CullMode::None);

        Gx::enable_clip();
        Gx::set_scissor(
            0,
            0,
            rc.framebuffer_width.into(),
            rc.embed_framebuffer_height.into(),
        );
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((x @ 0..=639, y @ 0..=527)) = coord.try_into() {
                let poke_x: u32 = x;
                let poke_y: u32 = y;
                Gx::poke_argb(
                    poke_x as u16,
                    poke_y as u16,
                    Color::new(color.r(), color.g(), color.b()),
                )
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        Gx::begin(Primitive::Quads, GX_VTXFMT0 as _, 4);
        Gx::position_3f32(area.top_left.x as _, area.top_left.y as _, 0.0);

        Gx::color_4u8(color.r(), color.g(), color.b(), 255);

        Gx::position_3f32(
            area.bottom_right().unwrap().x as _,
            area.top_left.y as _,
            0.0,
        );

        Gx::color_4u8(color.r(), color.g(), color.b(), 255);

        Gx::position_3f32(
            area.bottom_right().unwrap().x as _,
            area.bottom_right().unwrap().y as _,
            0.0,
        );

        Gx::color_4u8(color.r(), color.g(), color.b(), 255);

        Gx::position_3f32(
            area.top_left.x as _,
            area.bottom_right().unwrap().y as _,
            0.0,
        );

        Gx::color_4u8(color.r(), color.g(), color.b(), 255);

        Gx::end();

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(640, 528)
    }
}
