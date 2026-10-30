use core::{convert::TryInto, ffi::c_void};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};
use ogc_rs::{
    ffi::GX_PNMTX0,
    gx::{self, Gamma, PixelFormat, VtxAttrIn, ZFormat},
    prelude::*,
};

pub struct Display;
impl Display {
    pub fn new(fifo_size: usize) -> Self {
        gx::init(fifo_size);
        Self
    }

    pub fn flush(&self, framebuffer: *mut c_void) {
        gx::draw_done();
        gx::set_z_mode(true, CmpFn::LessEq, true);
        unsafe {
            gx::copy_disp(framebuffer, true);
        }
    }

    pub fn setup(&self, rc: &mut RenderConfig) {
        let mut ident: Mat3x4 = Mat3x4::IDENTITY;
        gx::set_copy_clear(Color::with_alpha(0, 0, 0, 0), 0x00ffffff);
        gx::set_pixel_fmt(PixelFormat::Rgb8Z24, ZFormat::Linear);
        gx::set_viewport(
            0.0,
            0.0,
            rc.framebuffer_width as f32,
            rc.embed_framebuffer_height as f32,
            0.0,
            0.0,
        );

        let yscale =
            gx::get_y_scale_factor(rc.embed_framebuffer_height, rc.extern_framebuffer_height);
        let extern_framebuffer_height = gx::set_disp_copy_y_scale(yscale) as u16;

        let half_aspect_ratio = rc.vi_height == 2 * rc.extern_framebuffer_height;

        gx::set_disp_copy_src(0, 0, rc.framebuffer_width, rc.embed_framebuffer_height);
        gx::set_disp_copy_dst(rc.framebuffer_width, extern_framebuffer_height);
        gx::set_copy_filter(
            (rc.anti_aliasing != 0).then_some(rc.sample_pattern),
            Some(rc.v_filter),
        );
        gx::set_field_mode(rc.field_rendering != 0, half_aspect_ratio);
        gx::set_disp_copy_gamma(Gamma::Mode10);

        //Clear VTX
        gx::clear_vtx_desc();
        gx::invalidate_vtx_cache();
        gx::invalidate_tex_all();

        gx::set_vtx_desc(VtxAttr::Tex0, VtxAttrIn::None);
        gx::set_vtx_desc(VtxAttr::Pos, VtxAttrIn::Direct);
        gx::set_vtx_desc(VtxAttr::Color0, VtxAttrIn::Direct);

        gx::set_vtx_attr_fmt(0, VtxAttr::Pos, gx::POS_XYZ, gx::F32, 0);
        gx::set_vtx_attr_fmt(0, VtxAttr::Tex0, gx::TEX_ST, gx::F32, 0);
        gx::set_vtx_attr_fmt(0, VtxAttr::Color0, gx::CLR_RGBA, gx::RGBA8, 0);
        gx::set_z_mode(true, CmpFn::LessEq, true);

        gx::set_num_chans(1);
        gx::set_num_tex_gens(1);
        gx::set_tev_op(0, TevMode::PassColor);
        gx::set_tev_order(0, Some(0), false, Some(0), Some(ColorChannel::Color0A0));
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

        gx::set_viewport(
            0.0,
            0.0,
            rc.framebuffer_width as f32,
            rc.embed_framebuffer_height as f32,
            0.0,
            1.0,
        );
        gx::set_blend_mode(
            BlendMode::Blend,
            BlendCtrl::SrcAlpha,
            BlendCtrl::InvSrcAlpha,
            LogicOp::Clear,
        );
        gx::set_alpha_update(true);
        gx::set_alpha_compare(CmpFn::Greater, 0, AlphaOp::And, CmpFn::Always, 0);
        gx::set_color_update(true);
        gx::set_cull_mode(CullMode::None);

        gx::enable_clip();
        gx::set_scissor(
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
                gx::poke_argb(
                    poke_x as u16,
                    poke_y as u16,
                    Color::new(color.r(), color.g(), color.b()),
                )
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        gx::begin(Primitive::Quads, 0, 4);
        gx::pos3_f32(area.top_left.x as _, area.top_left.y as _, 0.0);

        gx::color4_u8(color.r(), color.g(), color.b(), 255);

        gx::pos3_f32(
            area.bottom_right().unwrap().x as _,
            area.top_left.y as _,
            0.0,
        );

        gx::color4_u8(color.r(), color.g(), color.b(), 255);

        gx::pos3_f32(
            area.bottom_right().unwrap().x as _,
            area.bottom_right().unwrap().y as _,
            0.0,
        );

        gx::color4_u8(color.r(), color.g(), color.b(), 255);

        gx::pos3_f32(
            area.top_left.x as _,
            area.bottom_right().unwrap().y as _,
            0.0,
        );

        gx::color4_u8(color.r(), color.g(), color.b(), 255);

        gx::end();

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(640, 528)
    }
}
