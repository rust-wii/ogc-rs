#![no_std]
#![no_main]

use ogc_rs::{
    gu::{Gu, RotationAxis},
    gx::{self, CmpFn, Color, CullMode, Primitive, ProjectionType, VtxAttr, VtxAttrIn},
    video::Video,
};

extern crate alloc;

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let vi = Video::init();
    let mut config = Video::get_preferred_mode();

    // Add letterbox on PAL to avoid stretching
    if config.extern_framebuffer_height > config.embed_framebuffer_height {
        config.vi_y_origin = (config.extern_framebuffer_height - config.embed_framebuffer_height) / 2;
        config.vi_height = 480;
        config.extern_framebuffer_height = 480;
    }

    Video::configure(&config);
    unsafe { Video::set_next_framebuffer(vi.framebuffer) };
    Video::set_black(false);
    Video::flush();

    let _fifo = gx::init(256 * 1024);
    // Set values to use when video is flipped / cleared
    gx::set_copy_clear(Color::new(0x00, 0x00, 0x00), 0x00_FF_FF_FF);

    gx::set_viewport(
        0.0,
        0.0,
        config.framebuffer_width.into(),
        config.embed_framebuffer_height.into(),
        0.,
        1.,
    );
    gx::set_disp_copy_y_scale(1.0);
    gx::set_scissor(
        0,
        0,
        config.framebuffer_width.into(),
        config.embed_framebuffer_height.into(),
    );
    gx::set_disp_copy_src(
        0,
        0,
        config.framebuffer_width,
        config.embed_framebuffer_height,
    );
    gx::set_disp_copy_dst(config.framebuffer_width, config.extern_framebuffer_height);
    gx::set_copy_filter(
        (config.anti_aliasing != 0).then_some(config.sample_pattern),
        Some(config.v_filter),
    );

    let val = config.vi_height != 2 * config.extern_framebuffer_height;

    gx::set_field_mode(config.field_rendering != 0, val);
    gx::set_cull_mode(CullMode::None);
    unsafe { gx::copy_disp(vi.framebuffer, true) };

    let mut mat = [[0.; 4]; 4];
    Gu::perspective(&mut mat, 60., 4. / 3., 10., 300.);
    gx::load_projection_mtx(&mat, ProjectionType::Perspective);
    gx::clear_vtx_desc();
    gx::set_vtx_desc(VtxAttr::Pos, VtxAttrIn::Index8);
    gx::set_vtx_desc(VtxAttr::Color0, VtxAttrIn::Index8);
    gx::set_vtx_attr_fmt(0, VtxAttr::Pos, gx::POS_XYZ, gx::S16, 0);
    gx::set_vtx_attr_fmt(0, VtxAttr::Color0, gx::CLR_RGBA, gx::RGBA8, 0);

    let positions: [[i16; 3]; 3] = [[0, 15, 0], [-15, -15, 0], [15, -15, 0]];
    let colors: [[u8; 4]; 3] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]];

    gx::set_array(VtxAttr::Pos, &positions);
    gx::set_array(VtxAttr::Color0, &colors);

    gx::set_num_chans(1);
    gx::set_num_tex_gens(0);

    gx::set_tev_order(
        0,
        None,
        false,
        None,
        Some(gx::ColorChannel::Color0A0),
    );
    gx::set_tev_op(0, gx::TevMode::PassColor);

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
        gx::load_pos_mtx_imm(mdl_mtx, 0);

        gx::begin(Primitive::Triangles, 0, 3);
        gx::pos1_x8(0);
        gx::color1_x8(0);
        gx::pos1_x8(1);
        gx::color1_x8(1);
        gx::pos1_x8(2);
        gx::color1_x8(2);

        /*
                gx::position_3i16(0, 15, 0);
                gx::color_4u8(255, 0, 0, 255);
                gx::position_3i16(-15, -15, 0);
                gx::color_4u8(0, 255, 0, 255);
                gx::position_3i16(15, -15, 0);
                gx::color_4u8(0, 0, 255, 255);
        */
        gx::end();

        gx::draw_done();
        gx::set_z_mode(true, CmpFn::LessEq, true);
        gx::set_color_update(true);
        unsafe { gx::copy_disp(vi.framebuffer, true) };
        gx::flush();

        Video::wait_vsync();
        i += 1;
    }
}
