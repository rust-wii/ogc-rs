//! The ``gu`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the matrix subsystem functions found in ``gu.h``.

/// Represents the gu service.
pub struct Gu;

use ogc_sys::guVector;
use ogc_sys::{Mtx as Mtx34, Mtx44};

impl Gu {
    /// Sets a 4x4 matrix for orthographic projection.
    /// See [guOrtho](https://libogc.devkitpro.org/gu_8h.html#acce7b8b77ff8c321fbc6a797ea307541) for more.
    pub fn ortho(mt: &mut Mtx44, t: f32, b: f32, l: f32, r: f32, n: f32, f: f32) {
        unsafe { ogc_sys::guOrtho(mt as *mut _, t, b, l, r, n, f) }
    }

    /// Sets a 4x4 perspective projection matrix from field of view and aspect ratio parameters.
    /// See [guPerspective](https://libogc.devkitpro.org/gu_8h.html#af22f5e7e20c24dc11f2d58dfb64cdc95) for more.
    pub fn perspective(mt: &mut Mtx44, fovy: f32, aspect: f32, n: f32, f: f32) {
        unsafe { ogc_sys::guPerspective(mt as *mut _, fovy, aspect, n, f) }
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
            ogc_sys::guLookAt(
                mt as *mut _,
                cam_pos as *mut _,
                cam_up as *mut _,
                target as *mut _,
            )
        }
    }

    pub fn mtx_concat(a: &mut Mtx34, b: &mut Mtx34, ab: &mut Mtx34) {
        unsafe { ogc_sys::c_guMtxConcat(a as *mut _, b as *mut _, ab as *mut _) }
    }

    pub fn mtx_identity(mt: &mut Mtx34) {
        unsafe { ogc_sys::c_guMtxIdentity(mt as *mut _) }
    }

    pub fn mtx_trans_apply(src: &mut Mtx34, dst: &mut Mtx34, x_t: f32, y_t: f32, z_t: f32) {
        unsafe { ogc_sys::c_guMtxTransApply(src as *mut _, dst as *mut _, x_t, y_t, z_t) }
    }
}
