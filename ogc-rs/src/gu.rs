/// Represents the gu service.
pub struct Gu;

type Mtx44 = [[f32; 4]; 4];

impl Gu {
    /// Sets a 4x4 matrix for orthographic projection.
    /// See [guOrtho](https://libogc.devkitpro.org/gu_8h.html#acce7b8b77ff8c321fbc6a797ea307541) for more.
    pub fn ortho(mut mt: Mtx44, t: f32, b: f32, l: f32, r: f32, n: f32, f: f32) {
        unsafe {
            ogc_sys::guOrtho(&mut mt[0], t, b, l, r, n, f)
        }
    }

    /// Sets a 4x4 perspective projection matrix from field of view and aspect ratio parameters.
    /// See [guPerspective](https://libogc.devkitpro.org/gu_8h.html#af22f5e7e20c24dc11f2d58dfb64cdc95) for more.
    pub fn perspective(mut mt: Mtx44, fovy: f32, aspect: f32, n: f32, f: f32) {
        unsafe {
            ogc_sys::guPerspective(&mut mt[0], fovy, aspect, n, f)
        }
    }

    pub fn mtx_identity(mut mt: Mtx44) {
        unsafe {
            ogc_sys::guMtx44Identity(&mut mt[0])
        }
    }

    pub fn mtx_trans_apply(mut src: Mtx44, mut dst: Mtx44, x_t: f32, y_t: f32, z_t: f32) {
        unsafe {
            ogc_sys::c_guMtxTransApply(&mut src[0], &mut dst[0], x_t, y_t, z_t)
        }
    }
}
