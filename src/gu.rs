//! The ``gu`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the matrix subsystem functions found in ``gu.h``.

use core::fmt;

use ffi::guQuaternion;

use num_traits::Float;

use crate::{
    ffi::{self, guVector, Mtx as Mtx34, Mtx44},
    gx::{self, Gx},
};

#[repr(u8)]
pub enum RotationAxis {
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
}

/// Represents the gu service.
pub struct Gu;

impl Gu {
    //guVec -> Vec3
    pub fn vec_half_angle(a_vec: &mut guVector, b_vec: &mut guVector, ab_vec: &mut guVector) {
        unsafe { ffi::guVecHalfAngle(a_vec, b_vec, ab_vec) }
    }

    pub fn vec_add(a_vec: &mut guVector, b_vec: &mut guVector, ab_vec: &mut guVector) {
        unsafe { ffi::c_guVecAdd(a_vec, b_vec, ab_vec) }
    }

    pub fn vec_sub(a_vec: &mut guVector, b_vec: &mut guVector, ab_vec: &mut guVector) {
        unsafe { ffi::c_guVecSub(a_vec, b_vec, ab_vec) }
    }

    pub fn vec_scale(src: &mut guVector, dest: &mut guVector, scale: f32) {
        unsafe { ffi::c_guVecScale(src, dest, scale) }
    }

    pub fn vec_normalize(vector: &mut guVector) {
        unsafe { ffi::c_guVecNormalize(vector) }
    }

    pub fn vec_mult(mat: &mut Mtx34, src: &mut guVector, dest: &mut guVector) {
        unsafe { ffi::c_guVecMultiply(mat.as_mut_ptr().cast(), src, dest) }
    }

    pub fn vec_cross(a_vec: &mut guVector, b_vec: &mut guVector, ab_vec: &mut guVector) {
        unsafe { ffi::c_guVecCross(a_vec, b_vec, ab_vec) }
    }

    pub fn vec_mult_sr(mat: &mut Mtx34, src: &mut guVector, dest: &mut guVector) {
        unsafe { ffi::c_guVecMultiplySR(mat.as_mut_ptr().cast(), src, dest) }
    }

    pub fn vec_dot(a_vec: &mut guVector, b_vec: &mut guVector) -> f32 {
        unsafe { ffi::c_guVecDotProduct(a_vec, b_vec) }
    }

    //guQuaternion > Quat
    pub fn quat_add(
        a_quat: &mut guQuaternion,
        b_quat: &mut guQuaternion,
        ab_quat: &mut guQuaternion,
    ) {
        unsafe { ffi::c_guQuatAdd(a_quat, b_quat, ab_quat) }
    }

    pub fn quat_sub(
        a_quat: &mut guQuaternion,
        b_quat: &mut guQuaternion,
        ab_quat: &mut guQuaternion,
    ) {
        unsafe { ffi::c_guQuatSub(a_quat, b_quat, ab_quat) }
    }

    pub fn quat_norm(quaternion: &mut guQuaternion, dest: &mut guQuaternion) {
        unsafe { ffi::c_guQuatNormalize(quaternion, dest) }
    }
    pub fn quat_inverse(quaternion: &mut guQuaternion, dest: &mut guQuaternion) {
        unsafe { ffi::c_guQuatInverse(quaternion, dest) }
    }

    pub fn frustrum(
        mt: &mut Mtx44,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        z_near: f32,
        z_far: f32,
    ) {
        unsafe {
            ffi::guFrustum(
                mt.as_mut_ptr().cast(),
                top,
                bottom,
                left,
                right,
                z_near,
                z_far,
            )
        }
    }

    /// Sets a 4x4 perspective projection matrix from field of view and aspect ratio parameters.
    /// See [guPerspective](https://libogc.devkitpro.org/gu_8h.html#af22f5e7e20c24dc11f2d58dfb64cdc95) for more.
    pub fn perspective(mt: &mut Mtx44, fovy: f32, aspect: f32, near: f32, far: f32) {
        unsafe { ffi::guPerspective(mt as *mut _, fovy, aspect, near, far) }
    }

    /// Sets a 4x4 matrix for orthographic projection.
    /// See [guOrtho](https://libogc.devkitpro.org/gu_8h.html#acce7b8b77ff8c321fbc6a797ea307541) for more.
    pub fn ortho(
        mt: &mut Mtx44,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        near: f32,
        far: f32,
    ) {
        unsafe { ffi::guOrtho(mt as *mut _, top, bottom, left, right, near, far) }
    }

    pub fn mtx44_identity(mt: &mut Mtx44) {
        unsafe { ffi::guMtx44Identity(mt.as_mut_ptr().cast()) }
    }

    pub fn mtx44_copy(src: &mut Mtx44, dst: &mut Mtx44) {
        unsafe { ffi::guMtx44Copy(src.as_mut_ptr().cast(), dst.as_mut_ptr().cast()) }
    }

    pub fn mtx44_inverse(src: &mut Mtx44, inverse: &mut Mtx44) {
        unsafe {
            ffi::guMtx44Inverse(src.as_mut_ptr().cast(), inverse.as_mut_ptr().cast());
        }
    }

    pub fn light_frustum(
        mt: &mut Mtx34,
        frust_box: (f32, f32, f32, f32),
        z_near: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) {
        unsafe {
            ffi::guLightFrustum(
                mt.as_mut_ptr().cast(),
                frust_box.0,
                frust_box.1,
                frust_box.2,
                frust_box.3,
                z_near,
                scale.0,
                scale.1,
                translation.0,
                translation.1,
            )
        }
    }

    pub fn light_perspective(
        mt: &mut Mtx34,
        fov_y: f32,
        aspect_ratio: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) {
        unsafe {
            ffi::guLightPerspective(
                mt.as_mut_ptr().cast(),
                fov_y,
                aspect_ratio,
                scale.0,
                scale.1,
                translation.0,
                translation.1,
            )
        }
    }

    pub fn light_ortho(
        mt: &mut Mtx34,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) {
        unsafe {
            ffi::guLightOrtho(
                mt.as_mut_ptr().cast(),
                top,
                bottom,
                left,
                right,
                scale.0,
                scale.1,
                translation.0,
                translation.1,
            )
        }
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
    pub fn mtx_identity(mt: &mut Mtx34) {
        unsafe { ffi::c_guMtxIdentity(mt.as_mut_ptr().cast()) }
    }

    pub fn mtx_concat(a: &mut Mtx34, b: &mut Mtx34, ab: &mut Mtx34) {
        unsafe { ffi::c_guMtxConcat(a as *mut _, b as *mut _, ab as *mut _) }
    }

    pub fn mtx_scale(mt: &mut Mtx34, scale: (f32, f32, f32)) {
        unsafe { ffi::c_guMtxScale(mt.as_mut_ptr().cast(), scale.0, scale.1, scale.2) }
    }

    pub fn mtx_scale_apply(src: &mut Mtx34, dst: &mut Mtx34, scale: (f32, f32, f32)) {
        unsafe {
            ffi::c_guMtxScaleApply(
                src.as_mut_ptr().cast(),
                dst.as_mut_ptr().cast(),
                scale.0,
                scale.1,
                scale.2,
            )
        }
    }

    pub fn mtx_apply_scale(src: &mut Mtx34, dst: &mut Mtx34, scale: (f32, f32, f32)) {
        unsafe {
            ffi::c_guMtxApplyScale(
                src.as_mut_ptr().cast(),
                dst.as_mut_ptr().cast(),
                scale.0,
                scale.1,
                scale.2,
            )
        }
    }

    pub fn mtx_translation(mt: &mut Mtx34, translation: (f32, f32, f32)) {
        unsafe {
            ffi::c_guMtxTrans(
                mt.as_mut_ptr().cast(),
                translation.0,
                translation.1,
                translation.2,
            )
        }
    }

    pub fn mtx_translation_apply(src: &mut Mtx34, dst: &mut Mtx34, translation: (f32, f32, f32)) {
        unsafe {
            ffi::c_guMtxTransApply(
                src.as_mut_ptr().cast(),
                dst.as_mut_ptr().cast(),
                translation.0,
                translation.1,
                translation.2,
            )
        }
    }

    pub fn mtx_apply_translation(src: &mut Mtx34, dst: &mut Mtx34, translation: (f32, f32, f32)) {
        unsafe {
            ffi::c_guMtxApplyTrans(
                src.as_mut_ptr().cast(),
                dst.as_mut_ptr().cast(),
                translation.0,
                translation.1,
                translation.2,
            )
        }
    }

    pub fn mtx_inverse(src: &mut Mtx34, inverse: &mut Mtx34) {
        unsafe {
            ffi::c_guMtxInverse(src.as_mut_ptr().cast(), inverse.as_mut_ptr().cast());
        }
    }

    pub fn mtx_inv_xpose(src: &mut Mtx34, xpose: &mut Mtx34) {
        unsafe {
            ffi::c_guMtxInvXpose(src.as_mut_ptr().cast(), xpose.as_mut_ptr().cast());
        }
    }

    pub fn mtx_transpose(src: &mut Mtx34, xpose: &mut Mtx34) {
        unsafe {
            ffi::c_guMtxTranspose(src.as_mut_ptr().cast(), xpose.as_mut_ptr().cast());
        }
    }

    pub fn mtx_rotation_radians(mt: &mut Mtx34, axis: RotationAxis, rot_radians: f32) {
        unsafe { ffi::c_guMtxRotRad(mt.as_mut_ptr().cast(), axis as u8, rot_radians) }
    }

    pub fn mtx_rotation_trig(mt: &mut Mtx34, axis: RotationAxis, sin: f32, cos: f32) {
        unsafe { ffi::c_guMtxRotTrig(mt.as_mut_ptr().cast(), axis as u8, sin, cos) }
    }

    pub fn mtx_rotation_axis_radians(mt: &mut Mtx34, axis: &mut guVector, rot_radians: f32) {
        unsafe { ffi::c_guMtxRotAxisRad(mt.as_mut_ptr().cast(), axis, rot_radians) }
    }

    pub fn mtx_reflect(mt: &mut Mtx34, point: &mut guVector, normal: &mut guVector) {
        unsafe { ffi::c_guMtxReflect(mt.as_mut_ptr().cast(), point, normal) }
    }

    pub fn mtx_quaternion(mt: &mut Mtx34, quaternion: &mut guQuaternion) {
        unsafe { ffi::c_guMtxQuat(mt.as_mut_ptr().cast(), quaternion) }
    }
}

//TODO: Add Mat4 inverse to match gu.c
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat4([[f32; 4]; 4]);
impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub const ZERO: Mat4 = Mat4([[0.0; 4]; 4]);

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

        Mat4([
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
        ])
    }

    pub fn gu_perspective(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let fov_y_radians = (fov_y * 0.5) * 0.017453292;
        let cot = 1.0 / fov_y_radians.tan();
        let plane = 1.0 / (z_far - z_near);

        Mat4([
            [cot / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, cot, 0.0, 0.0],
            [0.0, 0.0, -(z_near) * plane, -(z_far * z_near) * plane],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }

    pub fn gu_ortho(top: f32, bottom: f32, left: f32, right: f32, z_near: f32, z_far: f32) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);
        let plane = 1.0 / (z_far - z_near);

        Self([
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
            [0.0, 0.0, -plane, -(z_far) * plane],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn load_as_proj_mat(&mut self, p: gx::ProjectionType) {
        Gx::load_projection_mtx(self.as_array_mut(), p);
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]
             [{}, {}, {}, {}]
             [{}, {}, {}, {}]
             [{}, {}, {}, {}]",
            self.0[0][0],
            self.0[0][1],
            self.0[0][2],
            self.0[0][3],
            self.0[1][0],
            self.0[1][1],
            self.0[1][2],
            self.0[1][3],
            self.0[2][0],
            self.0[2][1],
            self.0[2][2],
            self.0[2][3],
            self.0[3][0],
            self.0[3][1],
            self.0[3][2],
            self.0[3][3],
        )
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat3x4([[f32; 4]; 3]);
impl Mat3x4 {
    pub const IDENTITY: Mat3x4 = Mat3x4([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ]);

    pub const ZERO: Mat3x4 = Mat3x4([[0.0; 4]; 3]);

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

        Mat3x4([
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
        ])
    }

    pub fn gu_light_perspective(
        fov_y: f32,
        aspect_ratio: f32,
        scale: (f32, f32),
        translation: (f32, f32),
    ) -> Self {
        let fov_y_radians = (fov_y * 0.5) * 0.017453292;
        let cot = 1.0 / fov_y_radians.tan();

        Mat3x4([
            [(cot / aspect_ratio) * scale.0, 0.0, -translation.0, 0.0],
            [0.0, cot * scale.1, -translation.1, 0.0],
            [0.0, 0.0, -1.0, 0.0],
        ])
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

        Self([
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
        ])
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

    pub fn load_as_pos_mtx(&mut self, pnidx: u32) {
        Gx::load_pos_mtx_imm(self.as_array_mut(), pnidx);
    }
    pub fn load_as_nrm_mtx(&mut self, pnidx: u32) {
        Gx::load_nrm_mtx_imm(self.as_array_mut(), pnidx);
    }
    pub fn load_as_tex_mtx(&mut self, pnidx: u32) {
        Gx::load_tex_mtx_imm(self.as_array_mut(), pnidx);
    }
}
