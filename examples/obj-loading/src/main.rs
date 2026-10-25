#![no_std]
#![no_main]

mod obj;
use core::f32::consts::PI;
use ogc_rs::gu::RotationAxis;
use ogc_rs::input::{Button, ControllerPort, ControllerType, Input};
use ogc_rs::{alloc_aligned_buffer, print};

use ogc_rs::{
    gu::Gu,
    gx::{
        self, VtxAttrIn, CmpFn, Color, CullMode, Primitive, ProjectionType, TexFilter,
        Texture, VtxAttr, WrapMode,
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
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let Ok(obj) = obj::from_bytes(include_bytes!("./assets/untitled.obj")) else {
        panic!()
    };

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
    gx::set_copy_clear(Color::BLACK, 0x00_FF_FF_FF);

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
    Gu::perspective(&mut mat, 60., 4. / 3., 1., 1000.);
    gx::load_projection_mtx(&mat, ProjectionType::Perspective);

    gx::invalidate_vtx_cache();
    gx::clear_vtx_desc();
    gx::set_vtx_desc(VtxAttr::Pos, VtxAttrIn::Index16);
    gx::set_vtx_desc(VtxAttr::Nrm, VtxAttrIn::Index8);
    gx::set_vtx_desc(VtxAttr::Tex0, VtxAttrIn::Index8);
    gx::set_vtx_attr_fmt(0, VtxAttr::Pos, gx::POS_XYZ, gx::F32, 0);
    gx::set_vtx_attr_fmt(0, VtxAttr::Nrm, gx::NRM_XYZ, gx::F32, 0);
    gx::set_vtx_attr_fmt(0, VtxAttr::Tex0, gx::TEX_ST, gx::F32, 0);

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

    gx::set_array(VtxAttr::Pos, &positions);
    gx::set_array(VtxAttr::Nrm, &normals);
    gx::set_array(VtxAttr::Tex0, &tex);

    let header = minipng::decode_png_header(WHITE_BYTES).unwrap();
    let mut work_buf = alloc::vec![0; header.required_bytes_rgba8bpc()];
    let mut rgba_bytes = minipng::decode_png(WHITE_BYTES, &mut work_buf).unwrap();
    rgba_bytes.convert_to_rgba8bpc().unwrap();
    let tex_size = gctex::compute_image_size(
        gctex::TextureFormat::CMPR,
        header.width(),
        header.height(),
    ) as usize;
    let mut texture_bytes = ogc_rs::utils::Buf32::new(tex_size);
    gctex::encode_into(
        gctex::TextureFormat::CMPR,
        &mut texture_bytes,
        rgba_bytes.pixels(),
        header.width(),
        header.height(),
    );

    let buf = alloc_aligned_buffer(&texture_bytes);

    let mut texr = Texture::<!>::new(
        &buf,
        header.width().try_into().unwrap(),
        header.height().try_into().unwrap(),
        gx::TexFormat::CMPR,
        (WrapMode::Clamp, WrapMode::Clamp),
        false,
    );
    texr.set_filter_mode(TexFilter::Near, TexFilter::Near);
    gx::load_texture(&mut texr, 0);

    gx::set_num_chans(1);
    gx::set_num_tex_gens(1);

    gx::set_tev_order(
        0,
        Some(0),
        false,
        Some(0),
        Some(gx::ColorChannel::Color0A0),
    );
    gx::set_tev_op(0, gx::TevMode::Modulate);

    println!("Finished Setup");

    gx::flush();
    let mut i: u16 = 0;

    Input::init(ControllerType::Gamecube);
    let input = Input::new(ControllerType::Gamecube, ControllerPort::One);

    loop {
        Input::update(ControllerType::Gamecube);

        if input.is_button_down(Button::Start) {
            break 0;
        }

        gx::invalidate_vtx_cache();
        gx::invalidate_tex_all();

        gx::load_texture(&mut texr, 0);

        gx::set_viewport(
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
        gx::load_pos_mtx_imm(mdl_mtx, 0);
        gx::set_cull_mode(CullMode::None);
        gx::begin(Primitive::Triangles, 0, indices.len().try_into().unwrap());
        // v / vt / vn
        for index in indices.iter() {
            gx::pos1_x16(u16::try_from(index.0).unwrap());
            // let [x, y, z] = positions[*index];
            // gx::position_3f32(x, y, z);
            if let Some(idx) = index.2 {
                gx::pos1_x8(idx.try_into().unwrap());
            } else {
                panic!()
            }

            if let Some(idx) = index.1 {
                gx::pos1_x8(idx.try_into().unwrap());
            } else {
                panic!()
            }
        }

        gx::end();

        gx::draw_done();
        gx::set_z_mode(true, CmpFn::LessEq, true);
        gx::set_color_update(true);
        unsafe { gx::copy_disp(vi.framebuffer, true) };
        gx::flush();

        Video::wait_vsync();
        i = i.wrapping_add(1);
    }
}
