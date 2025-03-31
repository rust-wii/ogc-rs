#![no_std]
#![no_main]
use core::mem::ManuallyDrop;

use bit_field::BitField;
use ogc_rs::ffi::GX_TEXMAP_NULL;
use ogc_rs::{
    ffi::{
        GX_CLR_RGBA, GX_COLOR0A0, GX_PASSCLR, GX_POS_XYZ, GX_RGBA8, GX_S16, GX_TEXCOORDNULL,
<<<<<<< Updated upstream
        GX_TEXMAP_NULL, GX_VA_CLR0, GX_VA_POS, TB_BUS_CLOCK,
    },
    gu::{Gu, RotationAxis},
    gx::{
        experimental::{enable_write_pipe, move_to_write_pipe_address, Fifo},
        types::VtxDest,
        CmpFn, Color, CullMode, Gx, Primitive, ProjectionType, VtxAttr,
=======
        GX_VA_CLR0, GX_VA_POS, TB_BUS_CLOCK,
    },
    gu::{Gu, RotationAxis},
    gx::{
        experimental::Fifo, types::VtxDest, CmpFn, Color, CullMode, Gx, Primitive, ProjectionType,
        VtxAttr,
>>>>>>> Stashed changes
    },
    video::Video,
};

extern crate alloc;

#[no_mangle]
fn main() {
    let vi = Video::init();
    let mut config = Video::get_preferred_mode();

    Video::configure(&config);
    unsafe { Video::set_next_framebuffer(vi.framebuffer) };
    Video::set_black(false);
    Video::flush();
    //
    // let fifo = ManuallyDrop::new(Gx::init(256 * 1024));
    // // Set values to use when video is flipped / cleared

    let mut fifo = ManuallyDrop::new(Fifo::new(256 * 1024).expect("Fifo building failed"));
    fifo.init();

    const TB_BUS_CLOCK: u32 = 243000000;
    let res = TB_BUS_CLOCK / 500;

    fifo.write_bp_register(0x0f, 0xff);
    fifo.write_bp_register(0x69, *(res >> 11 & 0x00_FF_FF_FF).set_bit(10, true));

    fifo.write_bp_register(0x0f, 0xff);
    fifo.write_bp_register(0x46, *(res / 4224).set_bit(9, true));

    for i in 0..8 {
        fifo.write_cp_register(0x80 + i, 0x8000_0000);
    }

    fifo.write_xf_register(0x1000, 0x3f);
    fifo.write_xf_register(0x1012, 0x1);
    fifo.write_bp_register(0x58, 0x0f);

    fifo.write_cp_register(0x20, 0x00);
    fifo.write_xf_register(0x1006, 0x00);

    fifo.write_bp_register(0x23, 0x00);
    fifo.write_bp_register(0x24, 0x00);
    fifo.write_bp_register(0x67, 0x00);

    fifo.write_bp_register(0x0f, 0x00);

    fifo.write_bp_register(0x8c, 0x0d8000);
    fifo.write_bp_register(0x90, 0x0dc000);
    fifo.write_bp_register(0x8d, 0x0d8800);
    fifo.write_bp_register(0x91, 0x0dc800);
    fifo.write_bp_register(0x8e, 0x0d9000);
    fifo.write_bp_register(0x92, 0x0dd000);
    fifo.write_bp_register(0x8f, 0x0d9800);
    fifo.write_bp_register(0x93, 0x0dd800);

    //  Set_TextureImage0-3, GXTexMapID=4-7 tmem_offset=00010000, cache_width=32 kb, cache_height=32 kb, image_type=cached
    fifo.write_bp_register(0xac, 0x0da000);
    fifo.write_bp_register(0xb0, 0x0dc400);
    fifo.write_bp_register(0xad, 0x0da800);
    fifo.write_bp_register(0xb1, 0x0dcc00);
    fifo.write_bp_register(0xae, 0x0db000);
    fifo.write_bp_register(0xb2, 0x0dd400);
    fifo.write_bp_register(0xaf, 0x0db800);
    fifo.write_bp_register(0xb3, 0x0ddc00);

    fifo.set_copy_clear(Color::with_alpha(0x0, 0x0, 0x0, 0xff), 0x00_FF_FF_FF);

    fifo.flush();

    loop {}

<<<<<<< Updated upstream
    //    let fifo = ManuallyDrop::new(Gx::init(256 * 1024));
    let mut fifo = ManuallyDrop::new(Fifo::<262144>::new().unwrap());
    fifo.set_as_cpu_fifo().unwrap();
    fifo.set_as_gpu_fifo().unwrap();
    fifo.link_cpu_gpu_fifo().unwrap();

    unsafe {
        move_to_write_pipe_address(0x0C00_8000);
        enable_write_pipe();
        //Mask out all Indirect Tev Stages
        fifo.load_bp_reg(0x0f, &[0, 0, 0, 0xFF]);

        let value: u32 = TB_BUS_CLOCK / 500;
        let mut reg = bitfrob::u32_with_bit(10, 0u32, true);
        reg = bitfrob::u32_with_value(11, 24, reg, value);

        fifo.load_bp_reg(0x69, &reg.to_be_bytes());

        let mut other_reg = value / 4224;
        other_reg = bitfrob::u32_with_bit(9, other_reg, true);
        fifo.load_bp_reg(0x46, &other_reg.to_be_bytes());

        // Set VCacheEnhance on VAT
        let mut value = 0;
        value = bitfrob::u32_with_bit(31, value, true);
        for idx in 0x80..=0x87 {
            fifo.load_cp_reg(idx, &value.to_be_bytes());
        }

        fifo.load_cp_reg(0x20, &0u32.to_be_bytes());
        fifo.load_xf_reg(0x1006, &0u32.to_be_bytes());

        fifo.load_bp_reg(0x23, &0u32.to_be_bytes());
        fifo.load_bp_reg(0x24, &0u32.to_be_bytes());
        fifo.load_bp_reg(0x67, &0u32.to_be_bytes());

        // Clear Tex Indirect Mask at end
        fifo.load_bp_reg(0x0f, &[0, 0, 0, 0x0]);

        //Default texture setup
        let mut default_tex_reg = 0u32;
        const IMAGE_TYPE_CACHED: u32 = 0;
        const CACHE_SIZE_32KB: u32 = 3;
        const TEXTURE_MEM_OFFSET: u32 = 0;
        default_tex_reg = bitfrob::u32_with_bit(21, default_tex_reg, IMAGE_TYPE_CACHED != 0);
        default_tex_reg = bitfrob::u32_with_value(18, 20, default_tex_reg, CACHE_SIZE_32KB);
        default_tex_reg = bitfrob::u32_with_value(15, 17, default_tex_reg, CACHE_SIZE_32KB);
        default_tex_reg = bitfrob::u32_with_value(0, 14, default_tex_reg, TEXTURE_MEM_OFFSET);

        for index in 0x8c..=0x8f {
            fifo.load_bp_reg(index, &default_tex_reg.to_be_bytes());
        }

        for index in 0x90..=0x93 {
            fifo.load_bp_reg(index, &default_tex_reg.to_be_bytes());
        }

        default_tex_reg = bitfrob::u32_with_value(0, 14, default_tex_reg, 0x00010000);

        for index in 0xac..=0xaf {
            fifo.load_bp_reg(index, &default_tex_reg.to_be_bytes());
        }
=======
    Gx::set_copy_clear(Color::new(0x00, 0x00, 0x00), 0x00_FF_FF_FF);
>>>>>>> Stashed changes

        for index in 0xb0..=0xb3 {
            fifo.load_bp_reg(index, &default_tex_reg.to_be_bytes());
        }

        fifo.set_copy_clear(&[0, 0, 0, 0xff], 0xFF_FF_FF);

        fifo.write_bytes(&[0x48]);
    }

    // // Set values to use when video is flipped / cleared
    //    Gx::set_copy_clear(Color::new(0x00, 0x00, 0x00), 0x00_FF_FF_FF);
    //
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
    //    Gx::flush();
    // // let mut mat = [[0.; 4]; 4];
    // Gu::perspective(&mut mat, 60., 4. / 3., 10., 300.);
    // Gx::load_projection_mtx(&mat, ProjectionType::Perspective);
    // Gx::clear_vtx_desc();
    // Gx::set_vtx_desc(VtxAttr::Pos, VtxDest::INDEX8);
    // Gx::set_vtx_desc(VtxAttr::Color0, VtxDest::INDEX8);
    // Gx::set_vtx_attr_fmt(0, VtxAttr::Pos, GX_POS_XYZ, GX_S16, 0);
    // Gx::set_vtx_attr_fmt(0, VtxAttr::Color0, GX_CLR_RGBA, GX_RGBA8, 0);
    //
    // let positions: [[i16; 3]; 3] = [[0, 15, 0], [-15, -15, 0], [15, -15, 0]];
    // let colors: [[u8; 4]; 3] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]];
    //
    // Gx::set_array(
    //     GX_VA_POS,
    //     &positions,
    //     core::mem::size_of::<[i16; 3]>().try_into().unwrap(),
    // );
    //
    // Gx::set_array(
    //     GX_VA_CLR0,
    //     &colors,
    //     core::mem::size_of::<[u8; 4]>().try_into().unwrap(),
    // );
    //
    // Gx::set_num_chans(1);
    // Gx::set_num_tex_gens(0);
    //
    // Gx::set_tev_order(
    //     0,
    //     GX_TEXCOORDNULL.try_into().unwrap(),
    //     GX_TEXMAP_NULL,
    //     GX_COLOR0A0.try_into().unwrap(),
    // );
    // Gx::set_tev_op(0, GX_PASSCLR.try_into().unwrap());
    //
    // let mut i: u16 = 0;
    // loop {
    //     let mut mtx = [[0.; 4]; 3];
    //     let mut rot_mtx = [[0.; 4]; 3];
    //     let mut mdl_mtx = [[0.; 4]; 3];
    //     let mut mdl2_mtx = [[0.; 4]; 3];
    //
    //     Gu::mtx_identity(&mut mtx);
    //     Gu::mtx_identity(&mut rot_mtx);
    //     Gu::mtx_identity(&mut mdl_mtx);
    //
    //     Gu::mtx_rotation_radians(
    //         &mut rot_mtx,
    //         RotationAxis::Y,
    //         f32::from(i) * (3.14159 / 180.),
    //     );
    //     // Rotation + Identity = Rotation;
    //     Gu::mtx_concat(&mut rot_mtx, &mut mdl_mtx, &mut mdl2_mtx);
    //     // Rotation + Translation = Model;
    //     Gu::mtx_translation_apply(&mut mdl2_mtx, &mut mdl_mtx, (0., 0., -50.));
    //     // Load Model
    //     Gx::load_pos_mtx_imm(&mut mdl_mtx, 0);
    //
    //     Gx::begin(Primitive::Triangles, 0, 3);
    //     Gx::position1x8(0);
    //     Gx::color1x8(0);
    //     Gx::position1x8(1);
    //     Gx::color1x8(1);
    //     Gx::position1x8(2);
    //     Gx::color1x8(2);
    //
    //     /*
    //             Gx::position_3i16(0, 15, 0);
    //             Gx::color_4u8(255, 0, 0, 255);
    //             Gx::position_3i16(-15, -15, 0);
    //             Gx::color_4u8(0, 255, 0, 255);
    //             Gx::position_3i16(15, -15, 0);
    //             Gx::color_4u8(0, 0, 255, 255);
    //     */
    //     Gx::end();
    //
    //     Gx::draw_done();
    //     Gx::set_z_mode(true, CmpFn::LessEq, true);
    //     Gx::set_color_update(true);
    //     unsafe { Gx::copy_disp(vi.framebuffer, true) };
    //     Gx::flush();
    //
    //     Video::wait_vsync();
    //     i += 1;
    // }
    //
    loop {}
}
