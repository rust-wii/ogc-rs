#![no_std]

mod obj;
use core::f32::consts::PI;
use core::mem::ManuallyDrop;
use ogc_rs::ffi::{GX_F32, GX_NRM_XYZ, GX_TEX_ST, GX_VA_NRM, GX_VA_TEX0};
use ogc_rs::gu::RotationAxis;
use ogc_rs::input::{Button, ControllerPort, ControllerType, Input};
use ogc_rs::{alloc_aligned_buffer, print};

use ogc_rs::{
    ffi::{GX_COLOR0A0, GX_MODULATE, GX_POS_XYZ, GX_TEXCOORD0, GX_TEXMAP0, GX_TF_CMPR, GX_VA_POS},
    gu::Gu,
    gx::{
        types::VtxDest, CmpFn, Color, CullMode, Gx, Primitive, ProjectionType, TexFilter, Texture,
        VtxAttr, WrapMode,
    },
    println,
    video::Video,
};

extern crate alloc;
use alloc::vec::Vec;
const WHITE_BYTES: &[u8] = include_bytes!("../white.png");

#[repr(align(32))]
#[derive(Clone, Copy)]
pub struct Align32<T>(pub T);

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let Ok(obj) = obj::from_bytes(include_bytes!("./assets/untitled.obj")) else {
        panic!()
    };

    let vi = Video::init();
    let mut config = Video::get_preferred_mode();

    Video::configure(&config);
    unsafe { Video::set_next_framebuffer(vi.framebuffer) };
    Video::set_black(false);
    Video::flush();

    let _fifo = ManuallyDrop::new(Gx::init(256 * 1024));
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

    let val = config.vi_height != 2 * config.extern_framebuffer_height;

    Gx::set_field_mode(config.field_rendering != 0, val);
    Gx::set_cull_mode(CullMode::None);
    unsafe { Gx::copy_disp(vi.framebuffer, true) };

    let mut mat = [[0.; 4]; 4];
    Gu::perspective(&mut mat, 60., 4. / 3., 1., 1000.);
    Gx::load_projection_mtx(&mat, ProjectionType::Perspective);

    Gx::inv_vtx_cache();
    Gx::clear_vtx_desc();
    Gx::set_vtx_desc(VtxAttr::Pos, VtxDest::INDEX16);
    Gx::set_vtx_desc(VtxAttr::Nrm, VtxDest::INDEX8);
    Gx::set_vtx_desc(VtxAttr::Tex0, VtxDest::INDEX8);
    Gx::set_vtx_attr_fmt(0, VtxAttr::Pos, GX_POS_XYZ, GX_F32, 0);
    Gx::set_vtx_attr_fmt(0, VtxAttr::Nrm, GX_NRM_XYZ, GX_F32, 0);
    Gx::set_vtx_attr_fmt(0, VtxAttr::Tex0, GX_TEX_ST, GX_F32, 0);

    let indices: Vec<(usize, Option<usize>, Option<usize>)> = obj
        .indices()
        .unwrap()
        .flatten()
        //.map(|index| u16::try_from(index).unwrap())
        .collect();

    println!("{:?}", indices);

    let positions: Vec<[f32; 3]> = obj.vertices().unwrap().collect::<Vec<[f32; 3]>>();
    println!("{:?}", positions);

    let normals: Vec<[f32; 3]> = obj.normals().unwrap().collect();

    let tex: Vec<[f32; 2]> = obj.texcoords().unwrap().collect::<Vec<[f32; 2]>>();

    Gx::set_array(
        GX_VA_POS,
        &positions,
        core::mem::size_of::<[f32; 3]>().try_into().unwrap(),
    );

    Gx::set_array(
        GX_VA_NRM,
        &normals,
        core::mem::size_of::<[f32; 3]>().try_into().unwrap(),
    );
    Gx::set_array(
        GX_VA_TEX0,
        &tex,
        core::mem::size_of::<[f32; 2]>().try_into().unwrap(),
    );

    let header = minipng::decode_png_header(WHITE_BYTES).unwrap();
    let mut work_buf = alloc::vec![0; header.required_bytes_rgba8bpc()];
    let mut rgba_bytes = minipng::decode_png(WHITE_BYTES, &mut work_buf).unwrap();
    rgba_bytes.convert_to_rgba8bpc().unwrap();
    let texture_bytes = gctex::encode(
        gctex::TextureFormat::CMPR,
        rgba_bytes.pixels(),
        header.width(),
        header.height(),
    );

    let buf = alloc_aligned_buffer(&texture_bytes);

    let mut texr = Texture::new(
        &buf,
        header.width().try_into().unwrap(),
        header.height().try_into().unwrap(),
        GX_TF_CMPR.try_into().unwrap(),
        WrapMode::Clamp,
        WrapMode::Clamp,
        false,
    );
    texr.set_filter_mode(TexFilter::Near, TexFilter::Near);
    Gx::load_texture(&texr, GX_TEXMAP0.try_into().unwrap());

    Gx::set_num_chans(1);
    Gx::set_num_tex_gens(1);

    Gx::set_tev_order(
        0,
        GX_TEXCOORD0.try_into().unwrap(),
        GX_TEXMAP0,
        GX_COLOR0A0.try_into().unwrap(),
    );
    Gx::set_tev_op(0, GX_MODULATE.try_into().unwrap());

    println!("Finished Setup");

    Gx::flush();
    let mut i: u16 = 0;

    Input::init(ControllerType::Gamecube);
    let input = Input::new(ControllerType::Gamecube, ControllerPort::One);

    loop {
        Input::update(ControllerType::Gamecube);

        if input.is_button_down(Button::Start) {
            break 0;
        }

        Gx::inv_vtx_cache();
        Gx::invalidate_tex_all();

        Gx::load_texture(&texr, GX_TEXMAP0.try_into().unwrap());

        Gx::set_viewport(
            0.0,
            0.0,
            config.framebuffer_width.into(),
            config.embed_framebuffer_height.into(),
            0.,
            1.,
        );

        let mut mtx = [[0.; 4]; 3];
        let mut rot_mtx = [[0.; 4]; 3];
        let mut mdl_mtx = [[0.; 4]; 3];
        let mut mdl2_mtx = [[0.; 4]; 3];

        Gu::mtx_identity(&mut mtx);
        Gu::mtx_identity(&mut rot_mtx);
        Gu::mtx_identity(&mut mdl_mtx);

        Gu::mtx_rotation_radians(&mut rot_mtx, RotationAxis::Y, f32::from(i) * (PI / 180.));
        //        Rotation + Identity = Rotation;
        Gu::mtx_concat(&mut rot_mtx, &mut mdl_mtx, &mut mdl2_mtx);
        // // Rotation + Translation = Model;
        Gu::mtx_translation_apply(&mut mdl2_mtx, &mut mdl_mtx, (0., 0., -10.0));

        // Load Model
        Gx::load_pos_mtx_imm(&mut mdl_mtx, 0);
        Gx::set_cull_mode(CullMode::None);
        Gx::begin(Primitive::Triangles, 0, indices.len().try_into().unwrap());
        // v / vt / vn
        for index in indices.iter() {
            Gx::position1x16(u16::try_from(index.0).unwrap());
            // let [x, y, z] = positions[*index];
            // Gx::position_3f32(x, y, z);
            if let Some(idx) = index.2 {
                Gx::position1x8(idx.try_into().unwrap());
            } else {
                panic!()
            }

            if let Some(idx) = index.1 {
                Gx::position1x8(idx.try_into().unwrap());
            } else {
                panic!()
            }
        }

        Gx::end();

        Gx::draw_done();
        Gx::set_z_mode(true, CmpFn::LessEq, true);
        Gx::set_color_update(true);
        unsafe { Gx::copy_disp(vi.framebuffer, true) };
        Gx::flush();

        Video::wait_vsync();
        i = i.wrapping_add(1);
    }
}
