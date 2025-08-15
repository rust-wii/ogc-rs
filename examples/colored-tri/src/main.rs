#![no_std]
#![no_mangle]

use core::mem::ManuallyDrop;

use ogc_rs::{
    ffi::{
        GX_CLR_RGBA, GX_COLOR0A0, GX_PASSCLR, GX_POS_XYZ, GX_RGBA8, GX_S16, GX_TEXCOORDNULL,
        GX_TEXMAP_NULL, GX_VA_CLR0, GX_VA_POS,
    },
    gu::{Gu, RotationAxis},
    gx::{types::VtxDest, CmpFn, Color, CullMode, Gx, Primitive, ProjectionType, VtxAttr},
    video::Video,
};

extern crate alloc;

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let vi = Video::init();
    let mut config = Video::get_preferred_mode();

    Video::configure(&config);
    unsafe { Video::set_next_framebuffer(vi.framebuffer) };
    Video::set_black(false);
    Video::flush();

    let fifo = ManuallyDrop::new(Gx::init(256 * 1024));
    // Set values to use when video is flipped / cleared
    Gx::set_copy_clear(Color::new(0x00, 0x00, 0x00), 0x00_FF_FF_FF);

    Gx::set_viewport(
        0.0,
        0.0,
        config.framebuffer_width.into(),
        config.embed_framebuffer_height.into(),
        0.,
        1.,
    );
    Gx::set_disp_copy_y_scale(
        (config.extern_framebuffer_height / config.embed_framebuffer_height).into(),
    );
    Gx::set_scissor(
        0,
        0,
        config.framebuffer_width.into(),
        config.embed_framebuffer_height.into(),
    );
    Gx::set_disp_copy_src(
        0,
        0,
        config.framebuffer_width,
        config.embed_framebuffer_height,
    );
    Gx::set_disp_copy_dst(config.framebuffer_width, config.extern_framebuffer_height);
    Gx::set_copy_filter(
        config.anti_aliasing != 0,
        &mut config.sample_pattern,
        true,
        &mut config.v_filter,
    );

    let val = if config.vi_height == 2 * config.extern_framebuffer_height {
        false
    } else {
        true
    };

    Gx::set_field_mode(config.field_rendering != 0, val);
    Gx::set_cull_mode(CullMode::None);
    unsafe { Gx::copy_disp(vi.framebuffer, true) };

    let mut mat = [[0.; 4]; 4];
    Gu::perspective(&mut mat, 60., 4. / 3., 10., 300.);
    Gx::load_projection_mtx(&mat, ProjectionType::Perspective);
    Gx::clear_vtx_desc();
    Gx::set_vtx_desc(VtxAttr::Pos, VtxDest::INDEX8);
    Gx::set_vtx_desc(VtxAttr::Color0, VtxDest::INDEX8);
    Gx::set_vtx_attr_fmt(0, VtxAttr::Pos, GX_POS_XYZ, GX_S16, 0);
    Gx::set_vtx_attr_fmt(0, VtxAttr::Color0, GX_CLR_RGBA, GX_RGBA8, 0);

    let positions: [[i16; 3]; 3] = [[0, 15, 0], [-15, -15, 0], [15, -15, 0]];
    let colors: [[u8; 4]; 3] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]];

    Gx::set_array(
        GX_VA_POS,
        &positions,
        core::mem::size_of::<[i16; 3]>().try_into().unwrap(),
    );

    Gx::set_array(
        GX_VA_CLR0,
        &colors,
        core::mem::size_of::<[u8; 4]>().try_into().unwrap(),
    );

    Gx::set_num_chans(1);
    Gx::set_num_tex_gens(0);

    Gx::set_tev_order(
        0,
        GX_TEXCOORDNULL.try_into().unwrap(),
        GX_TEXMAP_NULL,
        GX_COLOR0A0.try_into().unwrap(),
    );
    Gx::set_tev_op(0, GX_PASSCLR.try_into().unwrap());

    let mut i: u16 = 0;
    loop {
        let mut mtx = [[0.; 4]; 3];
        let mut rot_mtx = [[0.; 4]; 3];
        let mut mdl_mtx = [[0.; 4]; 3];
        let mut mdl2_mtx = [[0.; 4]; 3];

        Gu::mtx_identity(&mut mtx);
        Gu::mtx_identity(&mut rot_mtx);
        Gu::mtx_identity(&mut mdl_mtx);

        Gu::mtx_rotation_radians(
            &mut rot_mtx,
            RotationAxis::Y,
            f32::from(i) * (3.14159 / 180.),
        );
        // Rotation + Identity = Rotation;
        Gu::mtx_concat(&mut rot_mtx, &mut mdl_mtx, &mut mdl2_mtx);
        // Rotation + Translation = Model;
        Gu::mtx_translation_apply(&mut mdl2_mtx, &mut mdl_mtx, (0., 0., -50.));
        // Load Model
        Gx::load_pos_mtx_imm(&mut mdl_mtx, 0);

        Gx::begin(Primitive::Triangles, 0, 3);
        Gx::position1x8(0);
        Gx::color1x8(0);
        Gx::position1x8(1);
        Gx::color1x8(1);
        Gx::position1x8(2);
        Gx::color1x8(2);

        /*
                Gx::position_3i16(0, 15, 0);
                Gx::color_4u8(255, 0, 0, 255);
                Gx::position_3i16(-15, -15, 0);
                Gx::color_4u8(0, 255, 0, 255);
                Gx::position_3i16(15, -15, 0);
                Gx::color_4u8(0, 0, 255, 255);
        */
        Gx::end();

        Gx::draw_done();
        Gx::set_z_mode(true, CmpFn::LessEq, true);
        Gx::set_color_update(true);
        unsafe { Gx::copy_disp(vi.framebuffer, true) };
        Gx::flush();

        Video::wait_vsync();
        i += 1;
    }
}
