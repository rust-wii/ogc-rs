//! The ``gx`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the graphics functions found in ``gx.h``.

use crate::ffi::{self, Mtx as Mtx34, Mtx44};
use core::ffi::c_void;

/// Helper function for `Gx::init`
pub fn gp_fifo(fifo_size: usize) -> *mut c_void {
    unsafe {
        let gp_fifo = crate::mem_cached_to_uncached!(libc::memalign(32, fifo_size));
        libc::memset(gp_fifo, 0, fifo_size);
        gp_fifo
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }
}

/// Backface culling mode.
///
/// Primitives in which the vertex order is clockwise to the viewer are considered front-facing.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum CullMode {
    /// Do not cull any primitives.
    None = ffi::GX_CULL_NONE as _,

    /// Cull front-facing primitives.
    Front = ffi::GX_CULL_FRONT as _,

    /// Cull back-facing primitives.
    Back = ffi::GX_CULL_BACK as _,

    /// Cull all primitives.
    All = ffi::GX_CULL_ALL as _,
}

/// Comparison functions.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum CmpFn {
    Never = ffi::GX_NEVER as _,
    Less = ffi::GX_LESS as _,
    Equal = ffi::GX_EQUAL as _,
    LessEq = ffi::GX_LEQUAL as _,
    Greater = ffi::GX_GREATER as _,
    NotEq = ffi::GX_NEQUAL as _,
    GreaterEq = ffi::GX_GEQUAL as _,
    Always = ffi::GX_ALWAYS as _,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
/// Alpha combining operations.
pub enum AlphaOp {
    And = ffi::GX_AOP_AND as _,
    Or = ffi::GX_AOP_OR as _,
    Xnor = ffi::GX_AOP_XNOR as _,
    Xor = ffi::GX_AOP_XOR as _,
}

/// Collection of primitive types that can be drawn by the GP.
///
/// Which type you use depends on your needs; however, performance can increase by using triangle
/// strips or fans instead of discrete triangles.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Primitive {
    /// Draws a series of unconnected quads. Every four vertices completes a quad. Internally, each
    /// quad is translated into a pair of triangles.
    Quads = ffi::GX_QUADS as _,

    /// Draws a series of unconnected triangles. Three vertices make a single triangle.
    Triangles = ffi::GX_TRIANGLES as _,

    /// Draws a series of triangles. Each triangle (besides the first) shares a side with the
    /// previous triangle. Each vertex (besides the first two) completes a triangle.
    TriangleStrip = ffi::GX_TRIANGLESTRIP as _,

    /// Draws a single triangle fan. The first vertex is the "centerpoint". The second and third
    /// vertex complete the first triangle. Each subsequent vertex completes another triangle which
    /// shares a side with the previous triangle (except the first triangle) and has the
    // centerpoint vertex as one of the vertices.
    TriangleFan = ffi::GX_TRIANGLEFAN as _,

    /// Draws a series of unconnected line segments. Each pair of vertices makes a line.
    Lines = ffi::GX_LINES as _,

    /// Draws a series of lines. Each vertex (besides the first) makes a line between it and the
    /// previous.
    LineStrip = ffi::GX_LINESTRIP as _,

    /// Draws a series of points. Each vertex is a single point.
    Points = ffi::GX_POINTS as _,
}

/// Specifies which blending operation to use.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum BlendMode {
    /// Write input directly to EFB
    None = ffi::GX_BM_NONE as _,

    /// Blend using blending equation
    Blend = ffi::GX_BM_BLEND as _,

    /// Blend using bitwise operation
    Logic = ffi::GX_BM_LOGIC as _,

    /// Input subtracts from existing pixel
    Subtract = ffi::GX_BM_SUBTRACT as _,
}

/// Destination (`dst`) acquires the value of one of these operations, given in Rust syntax.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum LogicOp {
    /// `src & dst`
    And = ffi::GX_LO_AND as _,
    /// `0`
    Clear = ffi::GX_LO_CLEAR as _,
    /// `src`
    Copy = ffi::GX_LO_COPY as _,
    /// `!(src ^ dst)`
    Equiv = ffi::GX_LO_EQUIV as _,
    /// `!dst`
    Inv = ffi::GX_LO_INV as _,
    /// `!src & dst`
    InvAnd = ffi::GX_LO_INVAND as _,
    /// `!src`
    InvCopy = ffi::GX_LO_INVCOPY as _,
    /// `!src | dst`
    InvOr = ffi::GX_LO_INVOR as _,
    /// `!(src & dst)`
    Nand = ffi::GX_LO_NAND as _,
    /// `dst`
    Nop = ffi::GX_LO_NOOP as _,
    /// `!(src | dst)`
    Nor = ffi::GX_LO_NOR as _,
    /// `src | dst`
    Or = ffi::GX_LO_OR as _,
    /// `src & !dst`
    RevAnd = ffi::GX_LO_REVAND as _,
    /// `src | !dst`
    RevOr = ffi::GX_LO_REVOR as _,
    /// `1`
    Set = ffi::GX_LO_SET as _,
    /// `src ^ dst`
    Xor = ffi::GX_LO_XOR as _,
}

/// Performance counter 0 is used to measure attributes dealing with geometry and primitives, such
/// as triangle counts and clipping ratios.
///
/// `Perf0::Xf*` measure how many GP cycles are spent in each stage of the XF.
///
/// The triangle metrics (`Perf0::Triangles*`) allow counting triangles under specific conditions
/// or with specific attributes.
///
/// `Perf0::Triangles*Tex` count triangles based on the number of texture coordinates supplied;
/// `Perf0::Triangles*Clr` count triangles based on the number of colors supplied.
///
/// The quad metrics allow you to count the number of quads (2x2 pixels) the GP processes. The term
/// coverage is used to indicate how many pixels in the quad are actually part of the triangle
/// being rasterized. For example, a coverage of 4 means all pixels in the quad intersect the
/// triangle. A coverage of 1 indicates that only 1 pixel in the quad intersected the triangle.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Perf0 {
    /// Average quad count; average based on what is unknown
    AvgQuadCnt = ffi::GX_PERF0_AVG_QUAD_CNT,
    /// Number of GP clocks spent clipping.
    ClipClks = ffi::GX_PERF0_CLIP_CLKS,
    ClipRatio = ffi::GX_PERF0_CLIP_RATIO,
    /// Number of vertices that were clipped by the GP.
    ClipVtx = ffi::GX_PERF0_CLIP_VTX,
    /// Number of GP clocks that have elapsed since the previous call to `Gx::read_gp0_metric()`.
    Clocks = ffi::GX_PERF0_CLOCKS,
    /// Disables performance measurement for perf0 and resets the counter.
    None = ffi::GX_PERF0_NONE,
    /// Number of quads having zero coverage.
    Quad0Cvg = ffi::GX_PERF0_QUAD_0CVG,
    /// Number of quads with 1 pixel coverage.
    Quad1Cvg = ffi::GX_PERF0_QUAD_1CVG,
    /// Number of quads with 2 pixel coverage.
    Quad2Cvg = ffi::GX_PERF0_QUAD_2CVG,
    /// Number of quads with 3 pixel coverage.
    Quad3Cvg = ffi::GX_PERF0_QUAD_3CVG,
    /// Number of quads with 4 pixel coverage.
    Quad4Cvg = ffi::GX_PERF0_QUAD_4CVG,
    /// Number of quads having coverage greater than zero.
    QuadNon0Cvg = ffi::GX_PERF0_QUAD_NON0CVG,
    /// Number of triangles.
    Triangles = ffi::GX_PERF0_TRIANGLES,
    Triangles0Clr = ffi::GX_PERF0_TRIANGLES_0CLR,
    Triangles0Tex = ffi::GX_PERF0_TRIANGLES_0TEX,
    Triangles1Clr = ffi::GX_PERF0_TRIANGLES_1CLR,
    Triangles1Tex = ffi::GX_PERF0_TRIANGLES_1TEX,
    Triangles2Clr = ffi::GX_PERF0_TRIANGLES_2CLR,
    Triangles2Tex = ffi::GX_PERF0_TRIANGLES_2TEX,
    Triangles3Tex = ffi::GX_PERF0_TRIANGLES_3TEX,
    Triangles4Tex = ffi::GX_PERF0_TRIANGLES_4TEX,
    Triangles5Tex = ffi::GX_PERF0_TRIANGLES_5TEX,
    Triangles6Tex = ffi::GX_PERF0_TRIANGLES_6TEX,
    Triangles7Tex = ffi::GX_PERF0_TRIANGLES_7TEX,
    Triangles8Tex = ffi::GX_PERF0_TRIANGLES_8TEX,
    /// Number of triangles that *failed* the front-face/back-face culling test.
    TrianglesCulled = ffi::GX_PERF0_TRIANGLES_CULLED,
    /// Number of triangles that *passed* the front-face/back-face culling test.
    TrianglesPassed = ffi::GX_PERF0_TRIANGLES_PASSED,
    /// Number of triangles that are scissored.
    TrianglesScissored = ffi::GX_PERF0_TRIANGLES_SCISSORED,
    /// Number of vertices processed by the GP.
    Vertices = ffi::GX_PERF0_VERTICES,
    /// Number of cycles the bottom of the pipe (result combiner) is busy.
    XfBotClks = ffi::GX_PERF0_XF_BOT_CLKS,
    /// Number of cycles the lighting engine is busy.
    XfLitClks = ffi::GX_PERF0_XF_LIT_CLKS,
    /// Number of cycles are spent loading XF state registers.
    XfRegldClks = ffi::GX_PERF0_XF_REGLD_CLKS,
    /// Number of cycles the XF reads the state registers.
    XfRegrdClks = ffi::GX_PERF0_XF_REGRD_CLKS,
    /// Number of cycles the XF is waiting on input. If the XF is waiting a large percentage of the
    /// total time, it may indicate that the CPU is not supplying data fast enough to keep the GP
    /// busy.
    XfWaitIn = ffi::GX_PERF0_XF_WAIT_IN,
    /// Number of cycles the XF waits to send its output to the rest of the GP pipeline. If the XF
    /// cannot output, it may indicate that the GP is currently fill-rate limited.
    XfWaitOut = ffi::GX_PERF0_XF_WAIT_OUT,
    /// Number of cycles the transform engine is busy.
    XfXfrmClks = ffi::GX_PERF0_XF_XFRM_CLKS,
}

/// Performance counter 1 is used for measuring texturing and caching performance as well as FIFO
/// performance.
///
/// `Perf1::Tc*` can be used to compute the texture cache (TC) miss rate. The `TcCheck*` parameters
/// count how many texture cache lines are accessed for each pixel. In the worst case, for a
/// mipmap, up to 8 cache lines may be accessed to produce one textured pixel. `Perf1::TcMiss`
/// counts how many of those accesses missed the texture cache. To compute the miss rate, divide
/// `Perf1::TcMiss` by the sum of all four `Perf1::TcCheck*` values.
///
/// `Perf1::Vc*` count different vertex cache stall conditions.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Perf1 {
    /// Number of lines (32B) read from called display lists.
    CallReq = ffi::GX_PERF1_CALL_REQ,
    /// Number of GP clocks that have elapsed since the last call to `Gx::read_gp1_metric()`.
    Clocks = ffi::GX_PERF1_CLOCKS,
    /// Counts all requests (32B/request) from the GP Command Processor (CP). It should be equal to
    /// the sum of counts returned by `Perf1::FifoReq`, `Perf1::CallReq`, and `Perf1::VcMissReq`.
    CpAllReq = ffi::GX_PERF1_CP_ALL_REQ,
    /// Number of lines (32B) read from the GP FIFO.
    FifoReq = ffi::GX_PERF1_FIFO_REQ,
    /// Disables performance measurement for perf1 and resets the counter.
    None = ffi::GX_PERF1_NONE,
    TcCheck12 = ffi::GX_PERF1_TC_CHECK1_2,
    TcCheck34 = ffi::GX_PERF1_TC_CHECK3_4,
    TcCheck56 = ffi::GX_PERF1_TC_CHECK5_6,
    TcCheck78 = ffi::GX_PERF1_TC_CHECK7_8,
    /// Number of texture cache misses in total?
    TcMiss = ffi::GX_PERF1_TC_MISS,
    /// Number of texels processed by the GP.
    Texels = ffi::GX_PERF1_TEXELS,
    /// Number of clocks that the texture unit (TX) is idle.
    TxIdle = ffi::GX_PERF1_TX_IDLE,
    /// Number of GP clocks the TX unit is stalled waiting for main memory.
    TxMemStall = ffi::GX_PERF1_TX_MEMSTALL,
    /// Number of GP clocks spent writing to state registers in the TX unit.
    TxRegs = ffi::GX_PERF1_TX_REGS,
    VcAllStalls = ffi::GX_PERF1_VC_ALL_STALLS,
    VcElemqFull = ffi::GX_PERF1_VC_ELEMQ_FULL,
    VcMemreqFull = ffi::GX_PERF1_VC_MEMREQ_FULL,
    /// Number vertex cache miss request. Each miss requests a 32B transfer from main memory.
    VcMissReq = ffi::GX_PERF1_VC_MISS_REQ,
    VcMissqFull = ffi::GX_PERF1_VC_MISSQ_FULL,
    VcMissrepFull = ffi::GX_PERF1_VC_MISSREP_FULL,
    VcStatus7 = ffi::GX_PERF1_VC_STATUS7,
    VcStreamBufLow = ffi::GX_PERF1_VC_STREAMBUF_LOW,
    /// Number of vertices processed by the GP.
    Vertices = ffi::GX_PERF1_VERTICES,
 }

/// Each pixel (source or destination) is multiplied by any of these controls.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum BlendCtrl {
    /// framebuffer alpha
    DstAlpha = ffi::GX_BL_DSTALPHA as _,
    /// 1.0 - (framebuffer alpha)
    InvDstAlpha = ffi::GX_BL_INVDSTALPHA as _,
    /// 1.0 - (source alpha)
    InvSrcAlpha = ffi::GX_BL_INVSRCALPHA as _,
    /// 1.0 - (source color)
    InvSrcColor = ffi::GX_BL_INVSRCCLR as _,
    /// 1.0
    One = ffi::GX_BL_ONE as _,
    /// source alpha
    SrcAlpha = ffi::GX_BL_SRCALPHA as _,
    /// source color
    SrcColor = ffi::GX_BL_SRCCLR as _,
    /// 0.0
    Zero = ffi::GX_BL_ZERO as _,
}

/// Compressed Z format.
///
/// See [`Gx::set_pixel_fmt()`] for details.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ZCompress {
    Linear = ffi::GX_ZC_LINEAR as _,
    Near = ffi::GX_ZC_NEAR as _,
    Mid = ffi::GX_ZC_MID as _,
    Far = ffi::GX_ZC_FAR as _,
}

/// Object containing information on a light.
#[repr(transparent)]
pub struct LightObj(ogc_sys::GXLightObj);

/// Type of the brightness decreasing function by distance.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DistFn {
    Off    = ffi::GX_DA_OFF as _,
    Gentle = ffi::GX_DA_GENTLE as _,
    Medium = ffi::GX_DA_MEDIUM as _,
    Steep  = ffi::GX_DA_STEEP as _,
}

/// Spot illumination distribution function
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum SpotFn {
    Off   = ffi::GX_SP_OFF as _,
    Flat  = ffi::GX_SP_FLAT as _,
    Cos   = ffi::GX_SP_COS as _,
    Cos2  = ffi::GX_SP_COS2 as _,
    Sharp = ffi::GX_SP_SHARP as _,
    Ring1 = ffi::GX_SP_RING1 as _,
    Ring2 = ffi::GX_SP_RING2 as _,
}

impl LightObj {
    /// Sets coefficients used in the lighting attenuation calculation in a given light object.
    ///
    /// The parameters `a0`, `a1`, and `a2` are used for angular (spotlight) attenuation. The
    /// coefficients `k0`, `k1`, and `k2` are used for distance attenuation. The attenuation
    /// function is:
    ///
    /// `atten = clamp0(a2^2 * aattn^2 + a1 * aattn + a0) / (k2 * d^2 + k1 * d + k0)`
    ///
    /// where `aattn` is the cosine of the angle between the light direction and the vector from
    /// the light position to the vertex, and `d` is the distance from the light position to the
    /// vertex when the channel attenuation function is `GX_AF_SPOT`. The light color will be
    /// multiplied by the `atten` factor when the attenuation function for the color channel
    /// referencing this light is set to `GX_AF_SPOT` (see [`GX_SetChanCtrl()`]).
    ///
    /// # Note
    /// The convenience function [`LightObj::set_spot_attn()`] can be used to set the angle
    /// attenuation coefficents based on several spot light types. The convenience function
    /// [`LightObj::set_dist_attn()`] can be used to set the distance attenuation coefficients using
    /// one of several common attenuation functions.
    ///
    /// The convenience macro [`GX_InitLightShininess()`] can be used to set the attenuation
    /// parameters for specular lights.
    ///
    /// When the channel attenuation function is set to `GX_AF_SPEC`, the `aattn` and `d` parameter
    /// are equal to the dot product of the eye-space vertex normal and the half-angle vector set
    /// by [`LightObj::set_specular_dir()`].
    pub fn set_attn(&mut self, a0: f32, a1: f32, a2: f32, k0: f32, k1: f32, k2: f32) {
        unsafe { ogc_sys::GX_InitLightAttn(&mut self.0, a0, a1, a2, k0, k1, k2) }
    }

    /// Sets shininess of a per-vertex specular light.
    ///
    /// In reality, shininess is a property of the material being lit, not the light. However, in
    /// the Graphics Processor, the specular calculation is implemented by reusing the diffuse
    /// angle/distance attenuation function, so shininess is determined by the light attenuation
    /// parameters (see [`LightObj::set_attn()`]). Note that the equation is attempting to
    /// approximate the function `(N*H)^shininess`. Since the attenuation equation is only a ratio
    /// of quadratics, a true exponential function is not possible. To enable the specular
    /// calculation, you must set the attenuation parameter of the lighting channel to `GX_AF_SPEC`
    /// using [`GX_SetChanCtrl()`].
    pub fn set_shininess(&mut self, shininess: f32) {
        self.set_attn(0.0, 0.0, 1.0, shininess / 2.0, 0.0, 1.0 - shininess / 2.0);
    }

    /// Sets coefficients used in the lighting angle attenuation calculation in a given light
    /// object.
    ///
    /// The parameters `a0`, `a1`, and `a2` are used for angular (spotlight) attenuation. The
    /// attenuation function is:
    ///
    /// `atten = clamp0(a2^2 * cos(theta)^2 + a1 * cos(theta) + a0) / (k2 * d^2 + k1 * d + k0)`
    ///
    /// where `cos(theta)` is the cosine of the angle between the light normal and the vector from
    /// the light position to the vertex, and `d` is the distance from the light position to the
    /// vertex. The `k0-2` coefficients can be set using [`LightObj::set_attn_k()`]. You can set
    /// both the `a0-2` and `k0-2` coefficients can be set using [`LightObj::set_attn()`]. The
    /// light color will be multiplied by the atten factor when the attenuation function for the
    /// color channel referencing this light is set to `GX_AF_SPOT` (see [`GX_SetChanCtrl()`]).
    ///
    /// # Note
    /// The convenience function [`LightObj::set_spot_attn()`] can be used to set the angle
    /// attenuation coefficents based on several spot light types. The convenience function
    /// [`LightObj::set_dist_attn()`] can be used to set the distance attenuation coefficients using
    /// one of several common attenuation functions.
    pub fn set_attn_a(&mut self, a0: f32, a1: f32, a2: f32) {
        unsafe { ogc_sys::GX_InitLightAttnA(&mut self.0, a0, a1, a2) }
    }

    /// Sets coefficients used in the lighting distance attenuation calculation in a given light object.
    ///
    /// The coefficients `k0`, `k1`, and `k2` are used for distance attenuation. The attenuation
    /// function is:
    ///
    /// `atten = clamp0(a2^2 * cos(theta)^2 + a1 * cos(theta) + a0) / (k2 * d^2 + k1 * d + k0)`
    ///
    /// where `cos(theta)` is the cosine of the angle between the light normal and the vector from
    /// the light position to the vertex, and `d` is the distance from the light position to the
    /// vertex. The `a0-2` coefficients can be set using [`LightObj::set_attn_a()`]. You can set
    /// both the `a0-2` and `k0-2` coefficients can be set using [`LightObj::set_attn()`]. The
    /// light color will be multiplied by the atten factor when the attenuation function for the
    /// color channel referencing this light is set to `GX_AF_SPOT` (see `GX_SetChanCtrl()`).
    ///
    /// # Note
    /// The convenience function [`LightObj::set_spot_attn()`] can be used to set the angle
    /// attenuation coefficents based on several spot light types. The convenience function
    /// [`LightObj::set_dist_attn()`] can be used to set the distance attenuation coefficients
    /// using one of several common attenuation functions.
    pub fn set_attn_k(&mut self, k0: f32, k1: f32, k2: f32) {
        unsafe { ogc_sys::GX_InitLightAttnK(&mut self.0, k0, k1, k2) }
    }
    /*
    /// Sets the color of the light in the light object.
    pub fn set_color(&mut self, color: GxColor) {
        unsafe { ogc_sys::GX_InitLightColor(&mut self.0, color) }
    }
    */
    /// Sets the direction of a light in the light object.
    ///
    /// This direction is used when the light object is used as spotlight or a specular light (see
    /// the `attn_fn` parameter of [`GX_SetChanCtrl()`]).
    ///
    /// # Note
    /// The coordinate space of the light normal should be consistent with a vertex normal
    /// transformed by a normal matrix; i.e., it should be transformed to view space.
    ///
    /// This function does not set the direction of parallel directional diffuse lights. If you
    /// want parallel diffuse lights, you may put the light position very far from every objects to
    /// be lit. (See [`LightObj::set_pos()`] and [`GX_SetChanCtrl()`])
    pub fn set_direction(&mut self, nx: f32, ny: f32, nz: f32) {
        unsafe { ogc_sys::GX_InitLightDir(&mut self.0, nx, ny, nz) }
    }

    /// Sets coefficients for distance attenuation in a light object.
    ///
    /// This function uses three easy-to-control parameters instead of `k0`, `k1`, and `k2` in
    /// [`LightObj::set_attn()`].
    ///
    /// In this function, you can specify the brightness on an assumed reference point. The
    /// parameter `ref_dist` is distance between the light and the reference point. The parameter
    /// `ref_brite` specifies ratio of the brightness on the reference point. The value for
    /// `ref_dist` should be greater than 0 and that for ref_brite should be within
    /// `0 < ref_brite < 1`, otherwise distance attenuation feature is turned off. The parameter
    /// `dist_fn` defines type of the brightness decreasing curve by distance; `DistFn::Off` turns
    /// distance attenuation feature off.
    ///
    /// # Note
    /// If you want more flexible control, it is better to use [`LightObj::set_attn()`] and
    /// calculate appropriate coefficients.
    ///
    /// This function sets parameters only for distance attenuation. Parameters for angular
    /// attenuation should be set by using [`LightObj::set_spot_attn()`] or
    /// [`LightObj::set_attn_a()`].
    pub fn set_dist_attn(&mut self, ref_dist: f32, ref_brite: f32, dist_fn: DistFn) {
        unsafe { ogc_sys::GX_InitLightDistAttn(&mut self.0, ref_dist, ref_brite, dist_fn as u8) }
    }

    /// Sets the position of the light in the light object.
    ///
    /// The GameCube graphics hardware supports local diffuse lights. The position of the light\
    /// should be in the same space as a transformed vertex position (i.e., view space).
    ///
    /// # Note
    /// Although the hardware doesn't support parallel directional diffuse lights, it is possible
    /// to get "almost parallel" lights by setting sufficient large values to position parameters
    /// (x, y and z) which makes the light position very far away from objects to be lit and all
    /// rays considered almost parallel.
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ogc_sys::GX_InitLightPos(&mut self.0, x, y, z) }
    }

    /// Sets coefficients for angular (spotlight) attenuation in light object.
    ///
    /// This function uses two easy-to-control parameters instead of `a0`, `a1`, and `a2` on
    /// [`LightObj::set_attn()`].
    ///
    /// The parameter `cut_off` specifies cutoff angle of the spotlight by degree. The spotlight
    /// works while the angle between the ray for a vertex and the light direction given by
    /// [`LightObj::set_direction()`] is smaller than this cutoff angle. The value for `cut_off`
    /// should be within `0 < cut_off <= 90.0`, otherwise given light object doesn't become a
    /// spotlight.
    ///
    /// The parameter `spotfn` defines type of the illumination distribution within cutoff angle.
    /// The value `SpotFn::Off` turns spotlight feature off even if color channel setting is using
    /// `GX_AF_SPOT` (see [`GX_SetChanCtrl()`]).
    ///
    /// # Note
    /// This function can generate only some kind of simple spotlights. If you want more flexible
    /// control, it is better to use [`LightObj::set_attn()`] and calculate appropriate
    /// coefficients.
    ///
    /// This function sets parameters only for angular attenuation. Parameters for distance
    /// attenuation should be set by using [`LightObj::set_dist_attn()`] or
    /// [`LightObj::set_attn_k()`].
    pub fn set_spot_attn(&mut self, cut_off: f32, spotfn: SpotFn) {
        unsafe { ogc_sys::GX_InitLightSpot(&mut self.0, cut_off, spotfn as u8) }
    }

    /// Sets the direction of a specular light in the light object.
    ///
    /// This direction is used when the light object is used only as specular light. The coordinate
    /// space of the light normal should be consistent with a vertex normal transformed by a normal
    /// matrix; i.e., it should be transformed to view space.
    ///
    /// # Note
    /// This function should be used if and only if the light object is used as specular light. One
    /// specifies a specular light in [`GX_SetChanCtrl()`] by setting the [Attenuation function] to
    /// `GX_AF_SPEC`. Furthermore, one must not use [`LightObj::set_direction()`] or
    /// [`LightObj::set_pos()`] to set up a light object which will be used as a specular light
    /// since these functions will destroy the information set by [`LightObj::set_specular_dir()`].
    /// In contrast to diffuse lights (including spotlights) that are considered local lights, a
    /// specular light is a parallel light (i.e. the specular light is infinitely far away such
    /// that all the rays of the light are parallel), and thus one can only specify directional
    /// information.
    pub fn set_specular_dir(&mut self, nx: f32, ny: f32, nz: f32) {
        unsafe { ogc_sys::GX_InitSpecularDir(&mut self.0, nx, ny, nz) }
    }

    /// Sets the direction and half-angle vector of a specular light in the light object.
    ///
    /// These vectors are used when the light object is used only as specular light. In contrast to
    /// [`LightObj::set_specular_dir()`], which caclulates half-angle vector automatically by
    /// assuming the view vector as (0, 0, 1), this function allows users to specify half-angle
    /// vector directly as input arguments. It is useful to do detailed control for orientation of
    /// highlights.
    ///
    /// See also [`LightObj::set_specular_dir()`].
    pub fn set_specular_dir_ha(&mut self, nx: f32, ny: f32, nz: f32, hx: f32, hy: f32, hz: f32) {
        unsafe { ogc_sys::GX_InitSpecularDirHA(&mut self.0, nx, ny, nz, hx, hy, hz) }
    }
}

/// Represents the GX service.
pub struct Gx;

impl Gx {
    /// Initializes the graphics processor to its initial state.
    /// See [GX_Init](https://libogc.devkitpro.org/gx_8h.html#aea24cfd5f8f2b168dc4f60d4883a6a8e) for more.
    pub fn init(gp_fifo: *mut c_void, fifo_size: u32) -> *mut ffi::GXFifoObj {
        // SAFETY: Both `fifo_size` and `gp_fifo` is aligned to a 32-byte boundary.
        assert_eq!(0, fifo_size % 32);
        unsafe { ffi::GX_Init(gp_fifo, fifo_size) }
    }

    /// Sets color and Z value to clear the EFB to during copy operations.
    /// See [GX_SetCopyClear](https://libogc.devkitpro.org/gx_8h.html#a17265aefd7e64820de53abd9113334bc) for more.
    pub fn set_copy_clear(background: Color, z_value: u32) {
        let Color(r, g, b, a) = background;
        let background = ffi::_gx_color { r, g, b, a };
        unsafe { ffi::GX_SetCopyClear(background, z_value) }
    }

    /// Sets the viewport rectangle in screen coordinates.
    /// See [GX_SetViewport](https://libogc.devkitpro.org/gx_8h.html#aaccd37675da5a22596fad756c73badc2) for more.
    pub fn set_viewport(x_orig: f32, y_orig: f32, wd: f32, hd: f32, near_z: f32, far_z: f32) {
        unsafe { ffi::GX_SetViewport(x_orig, y_orig, wd, hd, near_z, far_z) }
    }

    /// Calculates an appropriate Y scale factor value for GX_SetDispCopyYScale() based on the height of the EFB and the height of the XFB.
    /// See [GX_GetYScaleFactor](https://libogc.devkitpro.org/gx_8h.html#a1558cf7d2eb9a6690fee4b64c4fc5a8e) for more.
    pub fn get_y_scale_factor(efb_height: u16, xfb_height: u16) -> f32 {
        unsafe { ffi::GX_GetYScaleFactor(efb_height, xfb_height) }
    }

    /// Sets the vertical scale factor for the EFB to XFB copy operation.
    /// See [GX_SetDispCopyYScale](https://libogc.devkitpro.org/gx_8h.html#a1a4ebb4e742f4ce2f010768e09e07c48) for more.
    pub fn set_disp_copy_y_scale(y_scale: f32) -> u32 {
        unsafe { ffi::GX_SetDispCopyYScale(y_scale) }
    }

    /// Sets the scissor rectangle.
    /// See [GX_SetScissor](https://libogc.devkitpro.org/gx_8h.html#a689bdd17fc74bf86a4c4f00418a2c596) for more.
    pub fn set_scissor(x_origin: u32, y_origin: u32, wd: u32, hd: u32) {
        unsafe { ffi::GX_SetScissor(x_origin, y_origin, wd, hd) }
    }

    /// Sets the source parameters for the EFB to XFB copy operation.
    /// See [GX_SetDispCopySrc](https://libogc.devkitpro.org/gx_8h.html#a979d8db7abbbc2e9a267f5d1710ac588) for more.
    pub fn set_disp_copy_src(left: u16, top: u16, wd: u16, hd: u16) {
        assert_eq!(0, left % 2);
        assert_eq!(0, top % 2);
        assert_eq!(0, wd % 2);
        assert_eq!(0, hd % 2);
        unsafe { ffi::GX_SetDispCopySrc(left, top, wd, hd) }
    }

    /// Sets the witth and height of the display buffer in pixels.
    /// See [GX_SetDispCopyDst](https://libogc.devkitpro.org/gx_8h.html#ab6f639059b750e57af4c593ba92982c5) for more.
    pub fn set_disp_copy_dst(width: u16, height: u16) {
        assert_eq!(0, width % 16);
        unsafe { ffi::GX_SetDispCopyDst(width, height) }
    }

    /// Sets the subpixel sample patterns and vertical filter coefficients used to filter subpixels into pixels.
    /// See [GX_SetCopyFilter](https://libogc.devkitpro.org/gx_8h.html#afd65b7e5f2040ddb3352649efde72faf) for more.
    pub fn set_copy_filter(
        aa: bool,
        sample_pattern: &mut [[u8; 2]; 12],
        vf: bool,
        v_filter: &mut [u8; 7],
    ) {
        unsafe {
            ffi::GX_SetCopyFilter(
                aa as u8,
                sample_pattern as *mut _,
                vf as u8,
                v_filter as *mut _,
            )
        }
    }

    /// Controls various rasterization and texturing parameters that relate to field-mode and double-strike rendering.
    /// See [GX_SetFieldMode](https://libogc.devkitpro.org/gx_8h.html#a13f0df0011d04c3d986135e800fbcd21) for more.
    pub fn set_field_mode(field_mode: bool, half_aspect_ratio: bool) {
        unsafe { ffi::GX_SetFieldMode(field_mode as u8, half_aspect_ratio as u8) }
    }

    /// Sets the format of pixels in the Embedded Frame Buffer (EFB).
    /// See [GX_SetPixelFmt](https://libogc.devkitpro.org/gx_8h.html#a018d9b0359f9689ac41f44f0b2374ffb) for more.
    pub fn set_pixel_fmt(pix_fmt: u8, z_fmt: ZCompress) {
        unsafe { ffi::GX_SetPixelFmt(pix_fmt, z_fmt as u8) }
    }

    /// Enables or disables culling of geometry based on its orientation to the viewer.
    ///
    /// Primitives in which the vertex order is clockwise to the viewer are considered front-facing.
    ///
    /// See [GX_SetCullMode](https://libogc.devkitpro.org/gx_8h.html#adb4b17c39b24073c3e961458ecf02e87) for more.
    pub fn set_cull_mode(mode: CullMode) {
        unsafe { ffi::GX_SetCullMode(mode as u8) }
    }

    /// Copies the embedded framebuffer (EFB) to the external framebuffer(XFB) in main memory.
    /// See [GX_CopyDisp](https://libogc.devkitpro.org/gx_8h.html#a9ed0ae3f900abb6af2e930dff7a6bc28) for more.
    pub fn copy_disp(dest: *mut c_void, clear: bool) {
        unsafe { ffi::GX_CopyDisp(dest, clear as u8) }
    }

    /// Sets the gamma correction applied to pixels during EFB to XFB copy operation.
    /// See [GX_SetDispCopyGamma](https://libogc.devkitpro.org/gx_8h.html#aa8e5bc962cc786b2049345fa698d4efa) for more.
    pub fn set_disp_copy_gamma(gamma: u8) {
        unsafe { ffi::GX_SetDispCopyGamma(gamma) }
    }

    /// Sets the attribute format (vtxattr) for a single attribute in the Vertex Attribute Table (VAT).
    /// See [GX_SetVtxAttrFmt](https://libogc.devkitpro.org/gx_8h.html#a87437061debcc0457b6b6dc2eb021f23) for more.
    pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: u32, comptype: u32, compsize: u32, frac: u32) {
        unsafe { ffi::GX_SetVtxAttrFmt(vtxfmt, vtxattr, comptype, compsize, frac) }
    }

    /// Sets the number of color channels that are output to the TEV stages.
    /// See [GX_SetNumChans](https://libogc.devkitpro.org/gx_8h.html#a390c37e594986403c623df2bed61c2b2) for more.
    pub fn set_num_chans(num: u8) {
        unsafe { ffi::GX_SetNumChans(num) }
    }

    /// Sets the number of texture coordinates that are generated and available for use in the Texture Environment TEV stages.
    /// See [GX_SetNumTexGens](https://libogc.devkitpro.org/gx_8h.html#a55a79a1688d3a6957ee0c37d6323d159) for more.
    pub fn set_num_tex_gens(nr: u32) {
        unsafe { ffi::GX_SetNumTexGens(nr) }
    }

    /// Simplified function to set various TEV parameters for this tevstage based on a predefined combiner mode.
    /// See [GX_SetTevOp](https://libogc.devkitpro.org/gx_8h.html#a68554713cdde7b45ae4d5ce156239cf8) for more.
    pub fn set_tev_op(tevstage: u8, mode: u8) {
        unsafe { ffi::GX_SetTevOp(tevstage, mode) }
    }

    /// Specifies the texture and rasterized color that will be available as inputs to this TEV tevstage.
    /// See [GX_SetTevOrder](https://libogc.devkitpro.org/gx_8h.html#ae64799e52298de39efc74bf989fc57f5) for more.
    pub fn set_tev_order(tevstage: u8, texcoord: u8, texmap: u32, color: u8) {
        unsafe { ffi::GX_SetTevOrder(tevstage, texcoord, texmap, color) }
    }

    /// Specifies how texture coordinates are generated.
    /// See [GX_SetTexCoordGen](https://libogc.devkitpro.org/gx_8h.html#a7d3139b693ace5587c3224e7df2d8245) for more.
    pub fn set_tex_coord_gen(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32) {
        unsafe { ffi::GX_SetTexCoordGen(texcoord, tgen_typ, tgen_src, mtxsrc) }
    }

    /// Invalidates the current caches of the Texture Memory (TMEM).
    /// See [GX_InvalidateTexAll](https://libogc.devkitpro.org/gx_8h.html#a1e5666740bcd3c9325dd2b82006621ee) for more.
    pub fn invalidate_tex_all() {
        unsafe { ffi::GX_InvalidateTexAll() }
    }

    /// Loads the state describing a texture into one of eight hardware register sets.
    /// See [GX_LoadTexObj](https://libogc.devkitpro.org/gx_8h.html#ad6388b0e4a0f2ffb5daa16a8851fa567) for more.
    pub fn load_tex_obj(obj: &mut ffi::GXTexObj, mapid: u8) {
        unsafe { ffi::GX_LoadTexObj(obj, mapid) }
    }

    /// Sets the projection matrix.
    /// See [GX_LoadProjectionMtx](https://libogc.devkitpro.org/gx_8h.html#a241a1301f006ed04b7895c051959f64e) for more.
    pub fn load_projection_mtx(mt: &mut Mtx44, p_type: u8) {
        unsafe { ffi::GX_LoadProjectionMtx(mt as *mut _, p_type) }
    }

    /// Invalidates the vertex cache.
    /// See [GX_InvVtxCache](https://libogc.devkitpro.org/gx_8h.html#a188bc7f388f971bc845dded41a24d1dc) for more.
    pub fn inv_vtx_cache() {
        unsafe { ffi::GX_InvVtxCache() }
    }

    /// Clears all vertex attributes of the current vertex descriptor to GX_NONE.
    /// See [GX_ClearVtxDesc](https://libogc.devkitpro.org/gx_8h.html#acf1f933c4c653e399106e8ac244fabd0) for more.
    pub fn clear_vtx_desc() {
        unsafe { ffi::GX_ClearVtxDesc() }
    }

    /// Sets the type of a single attribute (attr) in the current vertex descriptor.
    /// See [GX_SetVtxDesc](https://libogc.devkitpro.org/gx_8h.html#af41b45011ae731ae5697b26b2bf97e2f) for more.
    pub fn set_vtx_desc(attr: u8, v_type: u8) {
        unsafe { ffi::GX_SetVtxDesc(attr, v_type) }
    }

    /// Used to load a 3x4 modelview matrix mt into matrix memory at location pnidx.
    /// See [GX_LoadPosMtxImm](https://libogc.devkitpro.org/gx_8h.html#a90349e713128a1fa4fd6048dcab7b5e7) for more.
    pub fn load_pos_mtx_imm(mt: &mut Mtx34, pnidx: u32) {
        unsafe { ffi::GX_LoadPosMtxImm(mt as *mut _, pnidx) }
    }

    /// Sends a DrawDone command to the GP and stalls until its subsequent execution.
    /// See [GX_DrawDone](https://libogc.devkitpro.org/gx_8h.html#a00f07b60ae2124fe027a82d7d9ae64b0) for more.
    pub fn draw_done() {
        unsafe { ffi::GX_DrawDone() }
    }

    /// Sets the Z-buffer compare mode.
    /// See [GX_SetZMode](https://libogc.devkitpro.org/gx_8h.html#a2af0d050f56ef45dd25d0db18909fa00) for more.
    pub fn set_z_mode(enable: bool, func: CmpFn, update_enable: bool) {
        unsafe { ffi::GX_SetZMode(enable as u8, func as u8, update_enable as u8) }
    }

    /// Determines how the source image, generated by the graphics processor, is blended with the Embedded Frame Buffer (EFB).
    /// See [GX_SetBlendMode](https://libogc.devkitpro.org/gx_8h.html#a1d9c43b161f3c5a30b9fd8ea182c8eb6) for more.
    pub fn set_blend_mode(
        b_type: BlendMode,
        src_fact: BlendCtrl,
        dst_fact: BlendCtrl,
        op: LogicOp,
    ) {
        unsafe { ffi::GX_SetBlendMode(b_type as u8, src_fact as u8, dst_fact as u8, op as u8) }
    }

    /// Enables or disables alpha-buffer updates of the Embedded Frame Buffer (EFB).
    /// See [GX_SetAlphaUpdate](https://libogc.devkitpro.org/gx_8h.html#ac238051bda896c8bb11802184882a2a0) for more.
    pub fn set_alpha_update(enable: bool) {
        unsafe { ffi::GX_SetAlphaUpdate(enable as u8) }
    }

    /// Enables or disables color-buffer updates when rendering into the Embedded Frame Buffer (EFB).
    /// See [GX_SetColorUpdate](https://libogc.devkitpro.org/gx_8h.html#a3978e3b08198e52d7cea411e90ece3e5) for more.
    pub fn set_color_update(enable: bool) {
        unsafe { ffi::GX_SetColorUpdate(enable as u8) }
    }

    /// Sets the array base pointer and stride for a single attribute.
    /// See [GX_SetArray](https://libogc.devkitpro.org/gx_8h.html#a5164fc6aa2a678d792af80d94bfa1ec2) for more.
    pub fn set_array(attr: u32, ptr: *mut c_void, stride: u8) {
        unsafe { ffi::GX_SetArray(attr, ptr, stride) }
    }

    /// Begins drawing of a graphics primitive.
    /// See [GX_Begin](https://libogc.devkitpro.org/gx_8h.html#ac1e1239130a33d9fae1352aee8d2cab9) for more.
    pub fn begin(primitive: Primitive, vtxfmt: u8, vtxcnt: u16) {
        unsafe { ffi::GX_Begin(primitive as u8, vtxfmt, vtxcnt) }
    }

    /// Sets the parameters for the alpha compare function which uses the alpha output from the last active TEV stage.
    /// See [Gx_SetAlphaCompare](https://libogc.devkitpro.org/gx_8h.html#a23ac269062a1b2c2efc8ad5aae24b26a) for more.
    pub fn set_alpha_compare(comp0: CmpFn, ref0: u8, aop: AlphaOp, comp1: CmpFn, ref1: u8) {
        unsafe { ffi::GX_SetAlphaCompare(comp0 as u8, ref0, aop as u8, comp1 as u8, ref1) }
    }

    /// Sets the parameters for the alpha compare function which uses the alpha output from the last active TEV stage.
    /// See [GX_SetClipMode](https://libogc.devkitpro.org/gx_8h.html#a3d348d7af8ded25b57352e956f43d974) for more.
    pub fn set_clip_mode(mode: u8) {
        unsafe { ffi::GX_SetClipMode(mode) }
    }

    /// Wrapper around set_clip_mode, since its a simple enable or disbale.
    pub fn enable_clip() {
        Gx::set_clip_mode(ffi::GX_CLIP_ENABLE as u8);
    }

    ///Wrapper around set_clip_mode, since it a simple disable or enable.
    pub fn disable_clip() {
        Gx::set_clip_mode(ffi::GX_CLIP_DISABLE as u8);
    }

    /// Allows the CPU to write color directly to the Embedded Frame Buffer (EFB) at position x, y.
    /// See [GX_PokeARGB](https://libogc.devkitpro.org/gx_8h.html#a5038d2f65e7959d64c68dcb1855353d8) for more.
    pub fn poke_argb(x: u16, y: u16, color: Color) {
        assert!(x < 640, "x must be less than 640, currently {}", x);
        assert!(y < 528, "y must be less than 527, currently {}", y);
        let Color(r, g, b, a) = color;
        let color = ffi::_gx_color { r, g, b, a };
        unsafe {
            ffi::GX_PokeARGB(x, y, color);
        }
    }

    pub fn position_3f32(x: f32, y: f32, z: f32) {
        unsafe {
            ffi::GX_Position3f32(x, y, z);
        }
    }

    pub fn position_3u16(x: u16, y: u16, z: u16) {
        unsafe {
            ffi::GX_Position3u16(x, y, z);
        }
    }

    pub fn position_3i16(x: i16, y: i16, z: i16) {
        unsafe {
            ffi::GX_Position3s16(x, y, z);
        }
    }

    pub fn position_3u8(x: u8, y: u8, z: u8) {
        unsafe {
            ffi::GX_Position3u8(x, y, z);
        }
    }

    pub fn position_3i8(x: i8, y: i8, z: i8) {
        unsafe {
            ffi::GX_Position3s8(x, y, z);
        }
    }

    pub fn position_2f32(x: f32, y: f32) {
        unsafe {
            ffi::GX_Position2f32(x, y);
        }
    }

    pub fn position_2u16(x: u16, y: u16) {
        unsafe {
            ffi::GX_Position2u16(x, y);
        }
    }

    pub fn position_2i16(x: i16, y: i16) {
        unsafe {
            ffi::GX_Position2s16(x, y);
        }
    }

    pub fn position_2u8(x: u8, y: u8) {
        unsafe {
            ffi::GX_Position2u8(x, y);
        }
    }

    pub fn position_2i8(x: i8, y: i8) {
        unsafe {
            ffi::GX_Position2s8(x, y);
        }
    }

    pub fn position1x8(index: u8) {
        unsafe { ffi::GX_Position1x8(index) }
    }

    pub fn position1x16(index: u16) {
        unsafe { ffi::GX_Position1x16(index) }
    }

    pub fn color_4u8(r: u8, b: u8, g: u8, a: u8) {
        unsafe {
            ffi::GX_Color4u8(r, g, b, a);
        }
    }

    pub fn color_3u8(r: u8, b: u8, g: u8) {
        unsafe {
            ffi::GX_Color3u8(r, g, b);
        }
    }

    pub fn color_3f32(r: f32, g: f32, b: f32) {
        unsafe {
            ffi::GX_Color3f32(r, g, b);
        }
    }

    pub fn color_1u32(clr: u32) {
        unsafe {
            ffi::GX_Color1u32(clr);
        }
    }

    pub fn color_1u16(clr: u16) {
        unsafe {
            ffi::GX_Color1u16(clr);
        }
    }

    pub fn color1x8(index: u8) {
        unsafe {
            ffi::GX_Color1x8(index);
        }
    }

    pub fn color1x16(index: u16) {
        unsafe {
            ffi::GX_Color1x16(index);
        }
    }

    ///Helper functions to just pass in a color object
    pub fn color_color(clr: Color) {
        unsafe {
            ffi::GX_Color4u8(clr.0, clr.1, clr.2, clr.3);
        }
    }

    pub fn tex_coord_2f32(s: f32, t: f32) {
        unsafe { ffi::GX_TexCoord2f32(s, t) }
    }

    pub fn flush() {
        unsafe { ffi::GX_Flush() }
    }

    pub fn end() {
        unsafe { ffi::GX_End() }
    }
}
