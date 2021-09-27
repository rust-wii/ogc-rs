//! The ``gu`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the matrix subsystem functions found in ``gu.h``.

use libm::tanf;

use crate::{
    ffi::{self, guVector, Mtx as Mtx34, Mtx44},
    gx::{self, Gx},
};

/// Represents the gu service.
pub struct Gu;

impl Gu {
    /// Sets a 4x4 matrix for orthographic projection.
    /// See [guOrtho](https://libogc.devkitpro.org/gu_8h.html#acce7b8b77ff8c321fbc6a797ea307541) for more.
    pub fn ortho(mt: &mut Mtx44, t: f32, b: f32, l: f32, r: f32, n: f32, f: f32) {
        unsafe { ffi::guOrtho(mt as *mut _, t, b, l, r, n, f) }
    }

    /// Sets a 4x4 perspective projection matrix from field of view and aspect ratio parameters.
    /// See [guPerspective](https://libogc.devkitpro.org/gu_8h.html#af22f5e7e20c24dc11f2d58dfb64cdc95) for more.
    pub fn perspective(mt: &mut Mtx44, fovy: f32, aspect: f32, n: f32, f: f32) {
        unsafe { ffi::guPerspective(mt as *mut _, fovy, aspect, n, f) }
    }

    /// Sets a world-space to camera-space transformation matrix.
    /// See [guLookAt](https://libogc.devkitpro.org/gu_8h.html#a3ed1b8f80bc0ab13879bd8ce7c16f5ee) for more.
    pub fn look_at(
        mt: &mut Mtx34,
        cam_pos: &mut guVector,
        cam_up: &mut guVector,
        target: &mut guVector,
    ) {
        unsafe {
            ffi::guLookAt(
                mt as *mut _,
                cam_pos as *mut _,
                cam_up as *mut _,
                target as *mut _,
            )
        }
    }

    pub fn mtx_concat(a: &mut Mtx34, b: &mut Mtx34, ab: &mut Mtx34) {
        unsafe { ffi::c_guMtxConcat(a as *mut _, b as *mut _, ab as *mut _) }
    }

    pub fn mtx_identity(mt: &mut Mtx34) {
        unsafe { ffi::c_guMtxIdentity(mt as *mut _) }
    }

    pub fn mtx_trans_apply(src: &mut Mtx34, dst: &mut Mtx34, x_t: f32, y_t: f32, z_t: f32) {
        unsafe { ffi::c_guMtxTransApply(src as *mut _, dst as *mut _, x_t, y_t, z_t) }
    }
}

//TODO: Add Mat4 inverse to match gu.c
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat4([[f32; 4]; 4]);
impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4 {
        0: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub const ZERO: Mat4 = Mat4 { 0: [[0.0; 4]; 4] };

    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        &self.0
    }

    pub fn as_array_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.0
    }

    pub fn gu_frustrum(
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);
        let plane = 1.0 / (z_far - z_near);

        Mat4 {
            0: [
                [
                    (2.0 * z_near) * right_left_aspect,
                    0.0,
                    (right + left) * right_left_aspect,
                    0.0,
                ],
                [
                    0.0,
                    (2.0 * z_near) * top_bottom_aspect,
                    (top + bottom) * top_bottom_aspect,
                    0.0,
                ],
                [0.0, 0.0, -(z_near) * plane, -(z_far * z_near) * plane],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn gu_perspective(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let fov_y_radians = (fov_y * 0.5) * 0.017453292;
        let cot = 1.0 / tanf(fov_y_radians);
        let plane = 1.0 / (z_far - z_near);

        Mat4 {
            0: [
                [cot / aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, cot, 0.0, 0.0],
                [0.0, 0.0, -(z_near) * plane, -(z_far * z_near) * plane],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn gu_ortho(top: f32, bottom: f32, left: f32, right: f32, z_near: f32, z_far: f32) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);
        let plane = 1.0 / (z_far - z_near);

        Self {
            0: [
                [
                    2.0 * right_left_aspect,
                    0.0,
                    0.0,
                    -(right + left) * right_left_aspect,
                ],
                [
                    0.0,
                    2.0 * top_bottom_aspect,
                    0.0,
                    -(top + bottom) * top_bottom_aspect,
                ],
                [0.0, 0.0, -(1.0) * plane, -(z_far) * plane],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn load_as_proj_mat(&mut self, p: gx::ProjectionType) {
        Gx::load_projection_mtx(self.as_array_mut(), p);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat3x4([[f32; 4]; 3]);
impl Mat3x4 {
    pub const IDENTITY: Mat3x4 = Mat3x4 {
        0: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ],
    };

    pub const ZERO: Mat3x4 = Mat3x4 { 0: [[0.0; 4]; 3] };

    pub fn as_array(&self) -> &[[f32; 4]; 3] {
        &self.0
    }

    pub fn as_array_mut(&mut self) -> &mut [[f32; 4]; 3] {
        &mut self.0
    }

    pub fn gu_light_frustrum(
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        z_near: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);

        Mat3x4 {
            0: [
                [
                    ((2.0 * z_near) * right_left_aspect) * scale.0,
                    0.0,
                    (((right + left) * right_left_aspect) * scale.0) - translation.0,
                    0.0,
                ],
                [
                    0.0,
                    ((2.0 * z_near) * top_bottom_aspect) * scale.1,
                    (((top + bottom) * top_bottom_aspect) * scale.1) - translation.1,
                    0.0,
                ],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn gu_light_perspective(
        fov_y: f32,
        aspect_ratio: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) -> Self {
        let fov_y_radians = (fov_y * 0.5) * 0.017453292;
        let cot = 1.0 / tanf(fov_y_radians);

        Mat3x4 {
            0: [
                [(cot / aspect_ratio) * scale.0, 0.0, -translation.0, 0.0],
                [0.0, cot * scale.1, -translation.1, 0.0],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn gu_light_ortho(
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);

        Self {
            0: [
                [
                    2.0 * right_left_aspect * scale.0,
                    0.0,
                    0.0,
                    (-(right + left) * right_left_aspect * scale.0) + translation.0,
                ],
                [
                    0.0,
                    2.0 * top_bottom_aspect * scale.1,
                    0.0,
                    (-(top + bottom) * top_bottom_aspect * scale.0) + translation.1,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn gu_look_at(pos: (f32, f32, f32), up: (f32, f32, f32), target: (f32, f32, f32)) -> Self {
        let mut look = Mat3x4::IDENTITY;

        Gu::look_at(
            look.as_array_mut(),
            &mut guVector {
                x: pos.0,
                y: pos.1,
                z: pos.2,
            },
            &mut guVector {
                x: up.0,
                y: up.1,
                z: up.2,
            },
            &mut guVector {
                x: target.0,
                y: target.1,
                z: target.2,
            },
        );

        look
    }
    pub fn gu_translation_apply(&mut self, translation: (f32, f32, f32)) {
        self.0[0][3] += self.0[0][0] * translation.0
            + self.0[0][1] * translation.1
            + self.0[0][2] * translation.2;
        self.0[1][3] += self.0[1][0] * translation.0
            + self.0[1][1] * translation.1
            + self.0[1][2] * translation.2;
        self.0[2][3] += self.0[2][0] * translation.0
            + self.0[2][1] * translation.1
            + self.0[2][2] * translation.2
            
    }

    pub fn concat(&mut self, other: &mut Mat3x4) {
        Gu::mtx_concat(
            self.clone().as_array_mut(),
            other.as_array_mut(),
            self.as_array_mut(),
        );
    }

    pub fn load_as_modelview(&mut self, pnidx: u32) {
        Gx::load_pos_mtx_imm(self.as_array_mut(), pnidx);
    }
}
