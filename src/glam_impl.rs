use glam::{Mat4, Vec4};
use libm::tanf;


pub trait GxProjection {
    fn orthographic_rh_gx(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self;
    fn perspective_rh_gx(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self;
    fn to_gx_projection(&self) -> [[f32; 4]; 4];
}

impl GxProjection for Mat4 {
    fn orthographic_rh_gx(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self {
        let right_left_aspect = 1.0 / (right - left);
        let top_bottom_aspect = 1.0 / (top - bottom);
        let plane = 1.0 / (z_far - z_near);

        
        
        Self::from_cols(
            Vec4::new(2.0*top_bottom_aspect, 0.0, 0.0, -(top+bottom)*top_bottom_aspect),
            Vec4::new(0.0, -(2.0)*right_left_aspect, 0., -(right+left) * right_left_aspect),
            Vec4::new(0.0, 0.0, -(1.0)*plane, -(z_far)*plane),
            Vec4::new(0.0, 0.0, 0.0, 1.0)
        )


    }
    
    fn perspective_rh_gx(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let cot = 1.0/tanf(fov_y_radians);
        let inv_fn = 1.0/(z_far-z_near);

        Self::from_cols(
            Vec4::new(cot/aspect_ratio,0.0, 0.0, 0.0),
            Vec4::new(0.0, cot, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -(z_near)*inv_fn, -(z_far*z_near)*inv_fn),
            Vec4::new(0.0, 0.0, -1.0, 0.0)
        )
    }

    fn to_gx_projection(&self) -> [[f32; 4]; 4] {
        [self.x_axis.to_array(), self.y_axis.to_array(), self.z_axis.to_array(), self.w_axis.to_array()]
    }

}
