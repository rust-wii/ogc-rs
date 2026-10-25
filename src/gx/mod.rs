//! Provides an OpenGL-like interface for rendering on the console.
//!
//! For brevity, only the differences between this library and its `ogc_sys` equivalent
//! (if any) will be documented, along with any safety invariants to keep in mind. Any
//! behavior related to libogc itself will be linked to at the end of the item's
//! description.

use core::ffi::c_void;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use ogc_sys::GXRModeObj;
use voladdress::{Safe, VolAddress};

use crate::ffi;
use crate::gu::{Mtx33, Mtx34, Mtx44};
use crate::{cache, lwp};
use crate::utils::{Buf32, mem};

pub use crate::ffi::WGPipe;
pub use crate::ffi::Vtx;
pub use crate::ffi::GXVtxAttrFmt as VtxAttrFmt;
pub use crate::ffi::GXVtxDesc as VtxDesc;
pub use crate::ffi::GXBreakPtCallback as RawBreakPtCallback;
pub use crate::ffi::GXDrawDoneCallback as RawDrawDoneCallback;
pub use crate::ffi::GXDrawSyncCallback as RawDrawSyncCallback;

/// Write-gather pipe for writing and sending attribute data to the GP.
/// 
/// This is here in case the `gx::pos*()`, `gx::color*()`, etc. functions don't cover all
/// your needs.
/// 
/// Relevant tips given by the [WiiBrew wiki's Developer Tips page]:
/// > If your GX application hangs as soon as you begin drawing with `GX_Begin()`, here
/// > are a few possible causes:
/// > - incorrect vertex formats - the vertex format specified with `GX_SetVtxAttrFmt()`
/// >   must match what your app sends between `GX_Begin()` and `GX_End()`.
/// > - incorrect vertex count - the vertex count specified by `GX_Begin()` must match the
/// >   vertex count your app sends between `GX_Begin()` and `GX_End()`.
/// > - incorrect vertex data order - vertex data is order dependent - the order GX
/// >   expects vertex data is: position, normal, color, bi-normals, and texture
/// >   coordinates.
/// > - surpassing the maximum number of vertex per between `GX_Begin()` and `GX_End()`
/// >   block which is 65,536 (not counting normals/colours).
/// 
/// [WiiBrew wiki's Developer Tips page]: https://wiibrew.org/wiki/Developer_tips#GX_Tips
pub const WG_PIPE: VolAddress<WGPipe, (), Safe> = unsafe { VolAddress::new(0xCC00_8000) };

// Number of components in an attribute

/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const CLR_RGB: u32 = ffi::GX_CLR_RGB;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const CLR_RGBA: u32 = ffi::GX_CLR_RGBA;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const NRM_NBT: u32 = ffi::GX_NRM_NBT;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const NRM_NBT3: u32 = ffi::GX_NRM_NBT3;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const NRM_XYZ: u32 = ffi::GX_NRM_XYZ;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const POS_XY: u32 = ffi::GX_POS_XY;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const POS_XYZ: u32 = ffi::GX_POS_XYZ;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const TEX_S: u32 = ffi::GX_TEX_S;
/// Specifies the number of components of an attribute. Used in the third argument of
/// [`set_vtx_attr_fmt()`].
pub const TEX_ST: u32 = ffi::GX_TEX_ST;

// Attribute component types

/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGB565: u32 = ffi::GX_RGB565;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGB8: u32 = ffi::GX_RGB8;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGBA4: u32 = ffi::GX_RGBA4;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGBA6: u32 = ffi::GX_RGBA6;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGBA8: u32 = ffi::GX_RGBA8;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const RGBX8: u32 = ffi::GX_RGBX8;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const U8: u32 = ffi::GX_U8;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const U16: u32 = ffi::GX_U16;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const S8: u32 = ffi::GX_S8;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const S16: u32 = ffi::GX_S16;
/// Specifies the type of an attribute. Used in the fourth argument of
/// [`set_vtx_attr_fmt()`].
pub const F32: u32 = ffi::GX_F32;

static IN_DISPLAY_LIST: AtomicBool = AtomicBool::new(false);
static WGPIPE_REDIRECTED: AtomicBool = AtomicBool::new(false);

static INIT_FIFO: AtomicPtr<RawFifo> = AtomicPtr::null();
static BREAKPT_CB: AtomicPtr<unsafe fn()> = AtomicPtr::null();
static DRAW_DONE_CB: AtomicPtr<fn()> = AtomicPtr::null();
static DRAW_SYNC_CB: AtomicPtr<fn(u16)> = AtomicPtr::null();

mod regs;
pub mod types;

/// Stores 8 bits for each red, green, blue, and alpha component.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Color(ffi::GXColor);

impl Color {
    /// Opaque black.
    pub const BLACK: Self = Self::new(0, 0, 0);
    /// Opaque white.
    pub const WHITE: Self = Self::new(255, 255, 255);
    
    /// Takes a byte for red, green, and blue, and returns a color. Defaults to an alpha
    /// value of 255.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::with_alpha(r, g, b, 255)
    }

    /// Takes a byte for red, green, blue, and alpha, and returns a color.
    pub const fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(ffi::GXColor { r, g, b, a })
    }
    
    pub(crate) const fn as_gxcolor(self) -> ffi::GXColor {
        self.0
    }
}

/// Stores 10 bits for each red, green, blue, and alpha component.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct ColorS10(ffi::GXColorS10);

impl ColorS10 {
    /// Takes a byte for red, green, and blue, and returns a color. Defaults to an alpha
    /// value of 255.
    pub const fn new(r: i16, g: i16, b: i16) -> Self {
        Self::with_alpha(r, g, b, 255)
    }

    /// Takes a byte for red, green, blue, and alpha, and returns a color.
    pub const fn with_alpha(r: i16, g: i16, b: i16, a: i16) -> Self {
        Self(ffi::GXColorS10 { r, g, b, a })
    }
    
    pub(crate) const fn as_gxcolors10(self) -> ffi::GXColorS10 {
        self.0
    }
}

/// Color channel ID
/// 
/// `Color0` and `Alpha0` are controlled separately for lighting, but they are rasterized
/// together as one RGBA color, `Color0A0`. Same for `Color1A1`. Since there's only one
/// rasterizer for color in the GP, one color must be chosen for each stage in the TEV
/// unit by using [`set_tev_order()`].
/// 
/// By default, if only one color is enabled in the current vertex descriptor (either
/// `VtxAttr::Color0` or `VtxAttr::Color1`), it will be directed to the `Color0A0`
/// channel. To use `Color1A1`, both color attributes must be enabled.
/// 
/// `AlphaBump` and `AlphaBumpN` allow using the bump-alpha component from an indirect
/// texture unit as a rasterized color input. Since bump-alpha only contains 5 bits of
/// data, `AlphaBump` shifts them to the higher bits, giving a range of 0-248, whereas
/// `AlphaBumpN` can normalize this to a range of 0-255. See [`set_tev_indirect()`] for
/// how to configure bump-alpha.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ColorChannel {
    /// RGB portion of the rasterized color.
    Color0 = ffi::GX_COLOR0,
    /// RGB portion of the second rasterized color, if any.
    Color1 = ffi::GX_COLOR1,
    /// Alpha portion of the rasterized color.
    Alpha0 = ffi::GX_ALPHA0,
    /// Alpha portion of the second rasterized color, if any.
    Alpha1 = ffi::GX_ALPHA1,
    /// Complete RGBA rasterized color.
    Color0A0 = ffi::GX_COLOR0A0,
    /// Complete RGBA of the second rasterized color, if any.
    Color1A1 = ffi::GX_COLOR1A1,
    
    /// Always get zero as a rasterized color.
    ColorZero = ffi::GX_COLORZERO,
    /// Uses bump-alpha component from indirect texture unit as a rasterized color input.
    /// Gives a value range of 0-248.
    AlphaBump = ffi::GX_ALPHA_BUMP,
    /// Uses bump alpha component from indirect texture unit as rasterized color input.
    /// This normalizes the values of `AlphaBump`, giving a range of 0-255.
    AlphaBumpN = ffi::GX_ALPHA_BUMPN,
}

/// Backface culling mode.
///
/// Primitives in which the vertex order is clockwise to the viewer are
/// considered front-facing.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum CullMode {
    /// Do not cull any primitives.
    None = ffi::GX_CULL_NONE,

    /// Cull front-facing primitives.
    Front = ffi::GX_CULL_FRONT,

    /// Cull back-facing primitives.
    Back = ffi::GX_CULL_BACK,

    /// Cull all primitives.
    All = ffi::GX_CULL_ALL,
}

/// Comparison relations used when comparing Z buffer values.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum CmpFn {
    Never = ffi::GX_NEVER,
    Less = ffi::GX_LESS,
    Equal = ffi::GX_EQUAL,
    LessEq = ffi::GX_LEQUAL,
    Greater = ffi::GX_GREATER,
    NotEq = ffi::GX_NEQUAL,
    GreaterEq = ffi::GX_GEQUAL,
    Always = ffi::GX_ALWAYS,
}

/// Alpha combining operations.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum AlphaOp {
    And = ffi::GX_AOP_AND,
    Or = ffi::GX_AOP_OR,
    Xnor = ffi::GX_AOP_XNOR,
    Xor = ffi::GX_AOP_XOR,
}

/// Collection of primitive types that can be drawn by the GP.
///
/// Which type you use depends on your needs; however, performance can increase by using triangle
/// strips or fans instead of discrete triangles.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Primitive {
    /// Draws a series of unconnected quads. Every four vertices completes a quad. Internally, each
    /// quad is translated into a pair of triangles.
    Quads = ffi::GX_QUADS,

    /// Draws a series of unconnected triangles. Three vertices make a single triangle.
    Triangles = ffi::GX_TRIANGLES,

    /// Draws a series of triangles. Each triangle (besides the first) shares a side with the
    /// previous triangle. Each vertex (besides the first two) completes a triangle.
    TriangleStrip = ffi::GX_TRIANGLESTRIP,

    /// Draws a single triangle fan. The first vertex is the "centerpoint". The second and third
    /// vertex complete the first triangle. Each subsequent vertex completes another triangle which
    /// shares a side with the previous triangle (except the first triangle) and has the
    // centerpoint vertex as one of the vertices.
    TriangleFan = ffi::GX_TRIANGLEFAN,

    /// Draws a series of unconnected line segments. Each pair of vertices makes a line.
    Lines = ffi::GX_LINES,

    /// Draws a series of lines. Each vertex (besides the first) makes a line between it and the
    /// previous.
    LineStrip = ffi::GX_LINESTRIP,

    /// Draws a series of points. Each vertex is a single point.
    Points = ffi::GX_POINTS,
}

/// Specifies which blending operation to use.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum BlendMode {
    /// Write input directly to EFB
    None = ffi::GX_BM_NONE,

    /// Blend using blending equation
    Blend = ffi::GX_BM_BLEND,

    /// Blend using bitwise operation
    Logic = ffi::GX_BM_LOGIC,

    /// Input subtracts from existing pixel
    Subtract = ffi::GX_BM_SUBTRACT,
}

/// Destination (`dst`) acquires the value of one of these operations.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum LogicOp {
    /// `src & dst`
    And = ffi::GX_LO_AND,
    /// `0`
    Clear = ffi::GX_LO_CLEAR,
    /// `src`
    Copy = ffi::GX_LO_COPY,
    /// `!(src ^ dst)`
    Equiv = ffi::GX_LO_EQUIV,
    /// `!dst`
    Inv = ffi::GX_LO_INV,
    /// `!src & dst`
    InvAnd = ffi::GX_LO_INVAND,
    /// `!src`
    InvCopy = ffi::GX_LO_INVCOPY,
    /// `!src | dst`
    InvOr = ffi::GX_LO_INVOR,
    /// `!(src & dst)`
    Nand = ffi::GX_LO_NAND,
    /// `dst`
    Nop = ffi::GX_LO_NOOP,
    /// `!(src | dst)`
    Nor = ffi::GX_LO_NOR,
    /// `src | dst`
    Or = ffi::GX_LO_OR,
    /// `src & !dst`
    RevAnd = ffi::GX_LO_REVAND,
    /// `src | !dst`
    RevOr = ffi::GX_LO_REVOR,
    /// `1`
    Set = ffi::GX_LO_SET,
    /// `src ^ dst`
    Xor = ffi::GX_LO_XOR,
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
    /// Number of GP clocks that have elapsed since the previous call to [`read_gp_metric()`].
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
    /// Number of GP clocks that have elapsed since the last call to [`read_gp_metric()`].
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

/// Vertex cache performance counter
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum VCacheAttr {
    Position = ffi::GX_VC_POS,
    Normal = ffi::GX_VC_NRM,
    Color0 = ffi::GX_VC_CLR0,
    Color1 = ffi::GX_VC_CLR1,
    Texture0 = ffi::GX_VC_TEX0,
    Texture1 = ffi::GX_VC_TEX1,
    Texture2 = ffi::GX_VC_TEX2,
    Texture3 = ffi::GX_VC_TEX3,
    Texture4 = ffi::GX_VC_TEX4,
    Texture5 = ffi::GX_VC_TEX5,
    Texture6 = ffi::GX_VC_TEX6,
    Texture7 = ffi::GX_VC_TEX7,
    All = ffi::GX_VC_ALL,
}

/// Blending control
/// 
/// Each pixel (source or destination) is multiplied by any of these controls.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum BlendCtrl {
    /// framebuffer alpha
    DstAlpha = ffi::GX_BL_DSTALPHA,
    /// 1.0 - (framebuffer alpha)
    InvDstAlpha = ffi::GX_BL_INVDSTALPHA,
    /// 1.0 - (source alpha)
    InvSrcAlpha = ffi::GX_BL_INVSRCALPHA,
    /// 1.0 - (source color)
    InvSrcColor = ffi::GX_BL_INVSRCCLR,
    /// 1.0
    One = ffi::GX_BL_ONE,
    /// source alpha
    SrcAlpha = ffi::GX_BL_SRCALPHA,
    /// source color
    SrcColor = ffi::GX_BL_SRCCLR,
    /// 0.0
    Zero = ffi::GX_BL_ZERO,
}

/// Compressed Z format.
///
/// See [`set_pixel_fmt()`] for details.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ZFormat {
    Linear = ffi::GX_ZC_LINEAR,
    Near = ffi::GX_ZC_NEAR,
    Mid = ffi::GX_ZC_MID,
    Far = ffi::GX_ZC_FAR,
}

/// Color channel source
/// 
/// Specifies whether the input source color for a color channel comes from a register or a vertex.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ColorChanSrc {
    Register = ffi::GX_SRC_REG,
    Vertex = ffi::GX_SRC_VTX,
}

/// Diffuse function for lights in a color channel
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum DiffFn {
    None = ffi::GX_DF_NONE,
    Signed = ffi::GX_DF_SIGNED,
    Clamp = ffi::GX_DF_CLAMP,
}

/// Attenuation function for lights in a color channel
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum AttnFn {
    /// No attenuation
    None = ffi::GX_AF_NONE,
    /// Specular computation
    Spec = ffi::GX_AF_SPEC,
    /// Spot light attenuation
    Spot = ffi::GX_AF_SPOT,
}

/// Raw GX FIFO for queuing graphics commands.
///
/// The Graphics FIFO is the mechanism used to communicate graphics commands from the CPU to the
/// Graphics Processor (GP). The FIFO base pointer is 32-byte aligned and the size must be a
/// multiple of 32 bytes.
///
/// The CPU's write-gather pipe is used to write data to the FIFO. Therefore, the FIFO
/// memory area must be forced out of the CPU cache prior to being used.
/// [`cache::dc_invalidate_range()`] may be used for this purpose. Due to the mechanics of
/// flushing the write-gather pipe, the FIFO memory area should be at least 32 bytes larger
/// than the maximum expected amount of data stored. Up to 32 NOPs may be written at the end
/// during flushing.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RawFifo(ffi::GXFifoObj);

/// First-In, First-Out (FIFO) data structure with an internal buffer for queuing
/// graphics commands.
#[derive(Debug, Clone)]
pub struct Fifo {
    inner: RawFifo,
    buf: Buf32,
}

impl Deref for Fifo {
    type Target = RawFifo;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Fifo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Fifo {
    /// Constructs a new `Fifo` with the minimum size.
    pub fn new() -> Self {
        Self::with_size(RawFifo::MIN_SIZE)
    }

    /// Constructs a new `Fifo` with the given size.
    ///
    /// If the given size is less than the minimum, the minimum size is used.
    pub fn with_size(size: usize) -> Self {
        let mut fifo = core::mem::MaybeUninit::zeroed();
        let mut buf = Buf32::new(size.max(RawFifo::MIN_SIZE));

        // SAFETY:
        // + original libogc source suggests that available init functions don't
        //   completely initialize the fifo, so it's been zeroed() above just in
        //   case.
        // + all other safety ensured by Buf32.
        unsafe {
            ffi::GX_InitFifoBase(
                fifo.as_mut_ptr(),
                buf.as_mut_ptr().map_addr(mem::to_uncached).cast(),
                buf.len() as u32,
            );
            Fifo {
                inner: RawFifo(fifo.assume_init()),
                buf,
            }
        }
    }
}

impl RawFifo {
    /// The minimum allowed size for a FIFO.
    pub const MIN_SIZE: usize = ffi::GX_FIFO_MINSIZE as usize;
    
    /// Get the base address for a given *fifo*.
    pub fn base(&self) -> *mut u8 {
        unsafe { ffi::GX_GetFifoBase(self as *const _ as *mut _) as *mut _ }
    }

    /// Returns number of cache lines in the FIFO.
    ///
    /// See [`ogc_sys::GX_GetFifoCount()`] for more.
    pub fn cache_lines(&self) -> usize {
        unsafe { ffi::GX_GetFifoCount(&self.0) as usize }
    }

    /// Get the size of a given FIFO.
    pub fn len(&self) -> usize {
        unsafe { ffi::GX_GetFifoSize(&self.0) as usize }
    }

    /// Returns a non-zero value if the write pointer has passed the top of the FIFO.
    ///
    /// See [`ogc_sys::GX_GetFifoWrap()`] for more.
    pub fn wrap_status(&self) -> u8 {
        unsafe { ffi::GX_GetFifoWrap(&self.0) }
    }

    /// Returns the current value of the Graphics FIFO read and write pointers.
    ///
    /// See [`ogc_sys::GX_GetFifoPtrs()`] for more.
    pub fn pointers(&self) -> (*const u8, *mut u8) {
        let mut rd_ptr = core::ptr::null_mut();
        let mut wt_ptr = core::ptr::null_mut();
        unsafe {
            ffi::GX_GetFifoPtrs(&self.0, &mut rd_ptr, &mut wt_ptr);
        }
        (rd_ptr.cast(), wt_ptr.cast())
    }
    
    /// Describes the area of main memory that will be used for this fifo.
    /// 
    /// See [`ogc_sys::GX_InitFifoBase()`] for more.
    /// 
    /// # Safety
    /// - This method should NOT be called from a `Fifo`, as it manages the internal
    ///   buffer itself.
    /// - `base` must be aligned to a 32-byte boundary.
    /// - `base` must be at least `RawFifo::MIN_SIZE` bytes long.
    /// - `base` length must be a multiple of 32.
    /// 
    /// To meet these requirements manually, consider using [`utils::Buf32`] and
    /// [`RawFifo::MIN_SIZE`].
    pub unsafe fn set_base(&mut self, base: &mut [u8]) {
        debug_assert!(base.as_mut_ptr().is_aligned_to(32));
        debug_assert!(base.len().is_multiple_of(32));
        debug_assert!(base.len() >= Self::MIN_SIZE);
        unsafe {
            ffi::GX_InitFifoBase(&mut self.0, base.as_mut_ptr().cast(), base.len() as u32)
        }
    }
    
    /// Sets the high and low water mark for the fifo.
    ///
    /// See [`ogc_sys::GX_InitFifoLimits()`] for more.
    pub fn set_limits(&mut self, hiwatermark: u32, lowatermark: u32) {
        debug_assert!(hiwatermark.is_multiple_of(32));
        debug_assert!(lowatermark.is_multiple_of(32));
        debug_assert!(hiwatermark < self.len() as u32);
        debug_assert!(lowatermark < self.len() as u32);
        unsafe { ffi::GX_InitFifoLimits(&mut self.0, hiwatermark, lowatermark) }
    }

    /// Sets the FIFO's read and write pointers.
    /// 
    /// See [`ogc_sys::GX_InitFifoPtrs()`] for more.
    /// 
    /// # Safety
    /// - `rd_ptr` and `wt_ptr` must be 32-byte aligned.
    /// - `rd_ptr` and `wt_ptr` must point within the same allocation as the FIFO's base
    ///   pointer.
    pub unsafe fn set_pointers(&mut self, rd_ptr: *const u8, wt_ptr: *mut u8) {
        debug_assert!(rd_ptr.is_aligned_to(32));
        debug_assert!(wt_ptr.is_aligned_to(32));
        unsafe { ffi::GX_InitFifoPtrs(&mut self.0, rd_ptr as *mut _, wt_ptr as *mut _) }
    }
}

/// Stores data about a light
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Light(ffi::GXLightObj);

/// Type of the brightness decreasing function by distance.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum DistFn {
    Off = ffi::GX_DA_OFF,
    Gentle = ffi::GX_DA_GENTLE,
    Medium = ffi::GX_DA_MEDIUM,
    Steep = ffi::GX_DA_STEEP,
}

/// Spot illumination distribution function
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum SpotFn {
    Off = ffi::GX_SP_OFF,
    Flat = ffi::GX_SP_FLAT,
    Cos = ffi::GX_SP_COS,
    Cos2 = ffi::GX_SP_COS2,
    Sharp = ffi::GX_SP_SHARP,
    Ring1 = ffi::GX_SP_RING1,
    Ring2 = ffi::GX_SP_RING2,
}

impl Light {
    /// Creates a white spotlight with the given normal at the view-space origin, and with angular
    /// and distance attenuation turned off.
    ///
    /// If needed, these are the default angle (*a*) and distance (*k*) coefficients:
    ///
    /// + a<sub>0</sub> = 1, a<sub>1</sub> = 0, a<sub>2</sub> = 0
    /// + k<sub>0</sub> = 1, k<sub>1</sub> = 0, k<sub>2</sub> = 0
    pub fn new_spotlight(nx: f32, ny: f32, nz: f32) -> Self {
        let mut light = core::mem::MaybeUninit::zeroed();
        // SAFETY: According to libogc source, light structs have 5 parts that must be initialized:
        // + position: set to view-space origin with GX_InitLightPos()
        // + color: set to white with GX_InitLightColor().
        // + direction/half-angle vector: set to the given values with GX_InitLightDir().
        // + attenuation: set to documented defaults with GX_InitLightAttn().
        // + padding: taken care of with zeroed() above.
        unsafe {
            ffi::GX_InitLightPos(light.as_mut_ptr(), 0.0, 0.0, 0.0);
            ffi::GX_InitLightColor(light.as_mut_ptr(), Color::new(255, 255, 255).0);
            ffi::GX_InitLightDir(light.as_mut_ptr(), nx, ny, nz);
            ffi::GX_InitLightAttn(light.as_mut_ptr(), 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            Self(light.assume_init())
        }
    }

    /// Creates a white specular light with the given normal, and with angular and distance
    /// attenuation turned off.
    ///
    /// If needed, these are the default angle (*a*) and distance (*k*) coefficients:
    ///
    /// + a<sub>0</sub> = 1, a<sub>1</sub> = 0, a<sub>2</sub> = 0
    /// + k<sub>0</sub> = 1, k<sub>1</sub> = 0, k<sub>2</sub> = 0
    pub fn new_specular(nx: f32, ny: f32, nz: f32) -> Self {
        let mut light = core::mem::MaybeUninit::zeroed();
        // SAFETY: According to libogc source, light structs have 5 parts that must be initialized:
        // + position: set by GX_InitSpecularDir()
        // + color: set to white by GX_InitLightColor().
        // + direction/half-angle vector: set to the given values by GX_InitSpecularDir().
        // + attenuation: set by GX_InitLightAttn() to values documented above.
        // + padding: taken care of with zeroed() above.
        unsafe {
            ffi::GX_InitLightColor(light.as_mut_ptr(), Color::new(255, 255, 255).0);
            ffi::GX_InitSpecularDir(light.as_mut_ptr(), nx, ny, nz);
            ffi::GX_InitLightAttn(light.as_mut_ptr(), 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            Self(light.assume_init())
        }
    }

    /// Sets coefficients used in the lighting attenuation calculation in a given light object.
    /// 
    /// See [`ogc_sys::GX_InitLightAttn()`] for more.
    pub fn attn(&mut self, a0: f32, a1: f32, a2: f32, k0: f32, k1: f32, k2: f32) -> &mut Self {
        unsafe {
            ogc_sys::GX_InitLightAttn(&mut self.0, a0, a1, a2, k0, k1, k2);
        }
        self
    }

    /// Sets shininess of a per-vertex specular light.
    ///
    /// In reality, shininess is a property of the material being lit, not the light. However, in
    /// the Graphics Processor, the specular calculation is implemented by reusing the diffuse
    /// angle/distance attenuation function, so shininess is determined by the light attenuation
    /// parameters (see [`Light::attn()`]). Note that the equation is attempting to
    /// approximate the function `(N*H)^shininess`. Since the attenuation equation is only a ratio
    /// of quadratics, a true exponential function is not possible. To enable the specular
    /// calculation, you must set the attenuation parameter of the lighting channel to
    /// `AttnFn::Spec` using [`set_channel_controls()`].
    /// 
    /// This is a reimplementation of the `GX_InitLightShininess()` macro function in libogc.
    pub fn shininess(&mut self, shininess: f32) -> &mut Self {
        self.attn(0.0, 0.0, 1.0, shininess / 2.0, 0.0, 1.0 - shininess / 2.0)
    }

    /// Sets coefficients used in the lighting angle attenuation calculation in a given light
    /// object.
    /// 
    /// See [`ogc_sys::GX_InitLightAttnA()`] for more.
    pub fn attn_a(&mut self, a0: f32, a1: f32, a2: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightAttnA(&mut self.0, a0, a1, a2);
        }
        self
    }

    /// Sets coefficients used in the lighting distance attenuation calculation in a given light
    /// object.
    /// 
    /// See [`ogc_sys::GX_InitLightAttnK()`] for more.
    pub fn attn_k(&mut self, k0: f32, k1: f32, k2: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightAttnK(&mut self.0, k0, k1, k2);
        }
        self
    }

    /// Sets the color of the light in the light object.
    /// 
    /// See [`ogc_sys::GX_InitLightColor()`] for more.
    pub fn color(&mut self, color: Color) -> &mut Self {
        unsafe {
            ffi::GX_InitLightColor(&mut self.0, color.0);
        }
        self
    }

    /// Sets the direction of a light in the light object.
    /// 
    /// See [`ogc_sys::GX_InitLightDir()`] for more.
    pub fn dir(&mut self, nx: f32, ny: f32, nz: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightDir(&mut self.0, nx, ny, nz);
        }
        self
    }

    /// Sets coefficients for distance attenuation in a light object.
    /// 
    /// See [`ogc_sys::GX_InitLightDistAttn()`] for more.
    pub fn dist_attn(&mut self, ref_dist: f32, ref_brite: f32, dist_fn: DistFn) -> &mut Self {
        unsafe {
            ffi::GX_InitLightDistAttn(&mut self.0, ref_dist, ref_brite, dist_fn as u8);
        }
        self
    }

    /// Sets the position of the light in the light object.
    /// 
    /// See [`ogc_sys::GX_InitLightPos()`] for more.
    pub fn pos(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightPos(&mut self.0, x, y, z);
        }
        self
    }

    /// Sets coefficients for angular (spotlight) attenuation in light object.
    /// 
    /// See [`ogc_sys::GX_InitLightSpot()`] for more.
    pub fn spot_attn(&mut self, cut_off: f32, spotfn: SpotFn) -> &mut Self {
        unsafe {
            ffi::GX_InitLightSpot(&mut self.0, cut_off, spotfn as u8);
        }
        self
    }

    /// Sets the direction of a specular light in the light object.
    /// 
    /// See [`ogc_sys::GX_InitSpecularDir()`] for more.
    pub fn specular_dir(&mut self, nx: f32, ny: f32, nz: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitSpecularDir(&mut self.0, nx, ny, nz);
        }
        self
    }

    /// Sets the direction and half-angle vector of a specular light in the light object.
    /// 
    /// See [`ogc_sys::GX_InitSpecularDirHA()`] for more.
    pub fn specular_dir_ha(
        &mut self,
        nx: f32,
        ny: f32,
        nz: f32,
        hx: f32,
        hy: f32,
        hz: f32,
    ) -> &mut Self {
        unsafe {
            ffi::GX_InitSpecularDirHA(&mut self.0, nx, ny, nz, hx, hy, hz);
        }
        self
    }
}

/// Texture filter types
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum TexFilter {
    /// Point sampling, no mipmap
    Near = ffi::GX_NEAR,
    /// Point sampling, linear mipmap
    NearMipLin = ffi::GX_NEAR_MIP_LIN,
    /// Point sampling, discrete mipmap
    NearMipNear = ffi::GX_NEAR_MIP_NEAR,
    /// Trilinear filtering
    LinMipLin = ffi::GX_LIN_MIP_LIN,
    /// Bilinear filtering, discrete mipmap
    LinMipNear = ffi::GX_LIN_MIP_NEAR,
    /// Bilinear filtering, no mipmap
    Linear = ffi::GX_LINEAR,
}

impl From<u8> for TexFilter {
    fn from(x: u8) -> Self {
        match x as u32 {
            ffi::GX_NEAR => TexFilter::Near,
            ffi::GX_NEAR_MIP_LIN => TexFilter::NearMipLin,
            ffi::GX_NEAR_MIP_NEAR => TexFilter::NearMipNear,
            ffi::GX_LIN_MIP_LIN => TexFilter::LinMipLin,
            ffi::GX_LIN_MIP_NEAR => TexFilter::LinMipNear,
            ffi::GX_LINEAR => TexFilter::Linear,
            _ => panic!("invalid texture filter type")
        }
    }
}

/// Texture wrap modes
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum WrapMode {
    Clamp = ffi::GX_CLAMP,
    Repeat = ffi::GX_REPEAT,
    Mirror = ffi::GX_MIRROR,
}

impl From<u8> for WrapMode {
    fn from(x: u8) -> Self {
        match x as u32 {
            ffi::GX_CLAMP => WrapMode::Clamp,
            ffi::GX_REPEAT => WrapMode::Repeat,
            ffi::GX_MIRROR => WrapMode::Mirror,
            _ => panic!("invalid wrap mode")
        }
    }
}

/// Texture formats that are supported.
///
/// For more information, see the [Image Formats page on the Custom Mario Kart Wiiki](https://mkwiiki.org/wiki/Image_Formats)
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum TexFormat {
    /// 4 bpp: 4-bit intensity; 8x8 block
    I4 = ffi::GX_TF_I4,
    /// 8 bpp: 8-bit intensity; 8x4 block
    I8 = ffi::GX_TF_I8,
    /// 8 bpp: 4-bit alpha, 4-bit intensity; 8x4 block
    /// `AAAAIIII`
    IA4 = ffi::GX_TF_IA4,
    /// 16 bpp: 8-bit alpha, 8-bit intensity; 4x4 block
    /// `AAAAAAAA IIIIIIII`
    IA8 = ffi::GX_TF_IA8,

    /// 16 bpp: 5-bit red, 6-bit green, 5-bit blue; 4x4 block
    /// `RRRRRGGG GGGBBBBB`
    RGB565 = ffi::GX_TF_RGB565,
    /// 16 bpp, 4x4 block
    /// * `1RRRRRGG GGGBBBBB`: 5-bit red, 5-bit green, 5-bit blue
    /// * `0AAARRRR GGGGBBBB`: 3-bit alpha, 4-bit red, 4-bit green, 4-bit blue
    RGB5A3 = ffi::GX_TF_RGB5A3,
    /// 32 bpp, 4x4 block
    /// ```
    /// AAAAAAAA RRRRRRRR, AAAAAAAA RRRRRRRR, ...
    /// AAAAAAAA RRRRRRRR, AAAAAAAA RRRRRRRR, ...
    /// GGGGGGGG BBBBBBBB, GGGGGGGG BBBBBBBB, ...
    /// GGGGGGGG BBBBBBBB, GGGGGGGG BBBBBBBB, ...
    /// ```
    RGBA8 = ffi::GX_TF_RGBA8,

    /// 4 bpp: 4-bit color index; 8x8 block, indices only; palletes: IA8, RGB565, RGB5A3
    CI4 = ffi::GX_TF_CI4,
    /// 8 bpp: 8-bit color index; 8x4 block, indices only; palletes: IA8, RGB565, RGB5A3
    CI8 = ffi::GX_TF_CI8,
    /// 16 bpp, 8x4 block, indices only; palletes: IA8, RGB565, RGB5A3
    /// `XXCCCCCC CCCCCCCC`: 2-bit unused, 14-bit color index
    CI14 = ffi::GX_TF_CI14,

    /// [DXT1-compressed](https://en.wikipedia.org/wiki/S3_Texture_Compression#DXT1) form
    CMPR = ffi::GX_TF_CMPR,

    /// For copying 4 bits from red
    CR4 = ffi::GX_CTF_R4,
    /// For copying 4 bits from red, 4 bits from alpha
    CRA4 = ffi::GX_CTF_RA4,
    /// For copying 8 bits from red, 8 bits from alpha
    CRA8 = ffi::GX_CTF_RA8,
    CYUVA8 = ffi::GX_CTF_YUVA8,
    /// For copying 8 bits from alpha
    CA8 = ffi::GX_CTF_A8,
    /// For copying 8 bits from red
    CR8 = ffi::GX_CTF_R8,
    /// For copying 8 bits from green
    CG8 = ffi::GX_CTF_G8,
    /// For copying 8 bits from blue
    CB8 = ffi::GX_CTF_B8,
    /// For copying 8 bits from red, 8 bits from green
    CRG8 = ffi::GX_CTF_RG8,
    /// For copying 8 bits from green, 8 bits from blue
    CGB8 = ffi::GX_CTF_GB8,
    /// For copying 4 upper bits from Z
    CZ4 = ffi::GX_CTF_Z4,
    /// For copying the middle 8 bits of Z
    CZ8M = ffi::GX_CTF_Z8M,
    /// For copying the lower 8 bits of Z
    CZ8L = ffi::GX_CTF_Z8L,
    /// For copying the lower 16 bits of Z
    CZ16L = ffi::GX_CTF_Z16L,
}


impl From<u32> for TexFormat {
    fn from(x: u32) -> Self {
        match x {
            ffi::GX_TF_I4 => TexFormat::I4,
            ffi::GX_TF_I8 => TexFormat::I8,
            ffi::GX_TF_IA4 => TexFormat::IA4,
            ffi::GX_TF_IA8 => TexFormat::IA8,
            ffi::GX_TF_RGB565 => TexFormat::RGB565,
            ffi::GX_TF_RGB5A3 => TexFormat::RGB5A3,
            ffi::GX_TF_RGBA8 => TexFormat::RGBA8,
            ffi::GX_TF_CI4 => TexFormat::CI4,
            ffi::GX_TF_CI8 => TexFormat::CI8,
            ffi::GX_TF_CI14 => TexFormat::CI14,
            ffi::GX_TF_CMPR => TexFormat::CMPR,
            ffi::GX_CTF_R4 => TexFormat::CR4,
            ffi::GX_CTF_RA4 => TexFormat::CRA4,
            ffi::GX_CTF_RA8 => TexFormat::CRA8,
            ffi::GX_CTF_YUVA8 => TexFormat::CYUVA8,
            ffi::GX_CTF_A8 => TexFormat::CA8,
            ffi::GX_CTF_R8 => TexFormat::CR8,
            ffi::GX_CTF_G8 => TexFormat::CG8,
            ffi::GX_CTF_B8 => TexFormat::CB8,
            ffi::GX_CTF_RG8 => TexFormat::CRG8,
            ffi::GX_CTF_GB8 => TexFormat::CGB8,
            ffi::GX_CTF_Z4 => TexFormat::CZ4,
            ffi::GX_CTF_Z8M => TexFormat::CZ8M,
            ffi::GX_CTF_Z8L => TexFormat::CZ8L,
            ffi::GX_CTF_Z16L => TexFormat::CZ16L,
            _ => panic!("invalid texture format"),
	    }
    }
}

/// Texture offset value
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TexOffset {
    ToZero = ffi::GX_TO_ZERO,
    ToSixteenth = ffi::GX_TO_SIXTEENTH,
    ToEighth = ffi::GX_TO_EIGHTH,
    ToFourth = ffi::GX_TO_FOURTH,
    ToHalf = ffi::GX_TO_HALF,
    ToOne = ffi::GX_TO_ONE,
}

/// Raw GX texture without a managed buffer.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RawTexture {
    inner: ffi::GXTexObj,
}

/// Texture with an internal image data buffer.
///
/// This implements `Deref` and `DerefMut` for [`RawTexture`], which means a `Texture` may
/// be used anywhere a `RawTexture` is referenced. This is similar to using a `&Vec<u8>`
/// in place of a `&[u8]`, for example.
/// 
/// User data can also be stored in this texture with `set_user_data()`.
#[derive(Debug, Clone)]
pub struct Texture<U = !> {
    inner: RawTexture,
    img: Buf32,
    user: Option<U>,
}

impl<T> Texture<T> {
    /// Used to initialize or change a texture object for non-color index textures.
    ///
    /// See [`ogc_sys::GX_InitTexObj()`] for more.
    pub fn new(
        img: &[u8],
        width: u16,
        height: u16,
        format: TexFormat,
        wrap: (WrapMode, WrapMode),
        mipmap: bool,
    ) -> Texture {
        let mut img_data = Buf32::new(img.len());
        let mut texture = core::mem::MaybeUninit::zeroed();

        // populate image data
        for (src, dest) in img.iter().zip(img_data.as_mut_slice().iter_mut()) {
            *dest = *src;
        }

        // error in debug mode when dimensions are too big.
        // in release it doesn't matter; libogc bit-masks the upper bits out.
        debug_assert!(width <= 1024, "max width for texture is 1024");
        debug_assert!(height <= 1024, "max height for texture is 1024");

        // SAFETY:
        // * ffi::GX_InitTexObj():
        //   * img_data is aligned to 32B boundary by design.
        // * texture.assume_init():
        //   * texture is initialized to zero above.
        unsafe {
            cache::data_cache_flush(img);
        }
        unsafe {
            ffi::GX_InitTexObj(
                texture.as_mut_ptr() as *mut _,
                img_data.as_mut_ptr() as *mut _,
                width,
                height,
                format as u8,
                wrap.0 as u8,
                wrap.1 as u8,
                mipmap as u8,
            );

            Texture {
                inner: texture.assume_init(),
                img: img_data,
                user: None,
            }
        }
    }

    /// Used to initialize or change a texture object when the texture is color index format.
    ///
    /// See [`ogc_sys::GX_InitTexObjCI()`] for more.
    pub fn with_color_idx(
        img: &[u8],
        width: u16,
        height: u16,
        format: TexFormat,
        wrap: (WrapMode, WrapMode),
        mipmap: bool,
        tlut_name: u32,
    ) -> Texture {
        let mut img_data = Buf32::new(img.len());
        let mut texture = core::mem::MaybeUninit::zeroed();

        // populate image data
        for (src, dest) in img.iter().zip(img_data.as_mut_slice().iter_mut()) {
            *dest = *src;
        }

        // error in debug mode when dimensions are too big.
        // in release it doesn't matter; libogc bit-masks the upper bits out.
        debug_assert!(width <= 1024, "max texture width is 1024, got {}", width);
        debug_assert!(height <= 1024, "max texture height is 1024, got {}", height);

        // error in debug mode when mipmaps sizes aren't a power of 2.
        // not sure what this does in release mode
        if mipmap {
            debug_assert!(width.is_power_of_two(), "mipmap texture width must be power of 2");
            debug_assert!(height.is_power_of_two(), "mipmap texture height must be power of 2");
        }

        // SAFETY:
        // * ffi::GX_InitTexObj():
        //   * img_data is aligned to 32B boundary by design.
        // * texture.assume_init():
        //   * texture is initialized to zero above.
        unsafe {
            cache::data_cache_flush(img);
        }
        unsafe {
            ffi::GX_InitTexObjCI(
                texture.as_mut_ptr() as *mut _,
                img_data.as_mut_ptr() as *mut _,
                width,
                height,
                format as u8,
                wrap.0 as u8,
                wrap.1 as u8,
                mipmap as u8,
                tlut_name,
            );

            Texture {
                inner: texture.assume_init(),
                img: img_data,
                user: None,
            }
        }
    }

    /// Returns a reference to the texture data.
    pub fn buf(&self) -> &Buf32 {
        //unsafe { ffi::GX_GetTexObjData(&self.inner) }
        &self.img
    }

    /// Returns a mutable reference to the texture data.
    ///
    /// Make sure to flush the cache after modifying the data by calling
    /// [`TexRegion::invalidate()`] on the corresponding region for this texture, or
    /// [`gx::invalidate_tex_all()`] to invalidate all texture caches.
    pub fn buf_mut(&mut self) -> &mut Buf32 {
        &mut self.img
    }
    
    /// Returns the user data stored in this texture, if any.
    pub fn user_data(&self) -> Option<&T> {
        self.user.as_ref()
    }
    
    /// Returns the user data stored in this texture, if any.
    pub fn user_data_mut(&mut self) -> Option<&mut T> {
        self.user.as_mut()
    }
    
    /// Sets the user data stored in this texture.
    pub fn set_user_data(&mut self, t: T) {
        self.user = Some(t);
    }
}

impl<T> Deref for Texture<T> {
    type Target = RawTexture;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Texture<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl RawTexture {
    /// Returns the amount of memory in bytes needed to store a texture of the given size
    /// and format.
    ///
    /// See [`ogc_sys::GX_GetTexBufferSize()`] for more.
    pub fn get_buffer_size(wd: u16, ht: u16, fmt: u32, mipmap: bool, maxlod: u8) -> usize {
        unsafe { ffi::GX_GetTexBufferSize(wd, ht, fmt, mipmap as u8, maxlod) as usize }
    }
    
    /// Returns a physical pointer to the texture data.
    pub fn data_ptr(&self) -> *const u8 {
        unsafe { ffi::GX_GetTexObjData(&self.inner).cast() }
    }

    /// Returns the texture height.
    pub fn height(&self) -> u16 {
        unsafe { ffi::GX_GetTexObjHeight(&self.inner) }
    }

    /// Returns the texture width.
    pub fn width(&self) -> u16 {
        unsafe { ffi::GX_GetTexObjWidth(&self.inner) }
    }

    /// Returns `true` if the texture's mipmap flag is enabled.
    pub fn is_mipmapped(&self) -> bool {
        unsafe { ffi::GX_GetTexObjMipMap(&self.inner) != 0 }
    }

    /// Returns the pixel format for the given texture.
    pub fn format(&self) -> TexFormat {
        unsafe { ffi::GX_GetTexObjFmt(&self.inner).into() }
    }

    /// Returns the wrap modes (horizontal and vertical) for this texture.
    pub fn wrap_mode(&self) -> (WrapMode, WrapMode) {
        unsafe {
            let wrap_s = ffi::GX_GetTexObjWrapS(&self.inner).into();
            let wrap_t = ffi::GX_GetTexObjWrapT(&self.inner).into();
            (wrap_s, wrap_t)
        }
    }

	/// Returns the minimum and maximum filter modes for the given texture.
    pub fn filter_mode(&self) -> (TexFilter, TexFilter) {
        let mut minfilt = 0;
        let mut maxfilt = 0;
        unsafe { ffi::GX_GetTexObjFilterMode(&self.inner, &mut minfilt, &mut maxfilt) }
        (minfilt.into(), maxfilt.into())
    }

	/// Returns the minimum and maximum LOD values for the given texture.
    pub fn lod(&self) -> (f32, f32) {
        let mut minlod = 0.0;
        let mut maxlod = 0.0;
        unsafe { ffi::GX_GetTexObjLOD(&self.inner, &mut minlod, &mut maxlod) }
        (minlod, maxlod)
    }

    /// Allows one to modify the image data pointer for an existing texture object.
    ///
    /// # Safety
    /// * This should not be called from a `Texture`, as it already uses an internal
    ///   buffer to manage image data.
    /// * The size and format of the new data must agree with those of this texture.
    /// * `img_ptr` must be 32-byte aligned.
    pub unsafe fn set_data_ptr(&mut self, img_ptr: *const u8) {
        debug_assert!(img_ptr.is_aligned_to(32));
        unsafe { ffi::GX_InitTexObjData(&mut self.inner, img_ptr.cast_mut().cast()) }
    }

    /// Enables bias clamping for texture LOD.
    ///
    /// If set to `true`, the sum of LOD and `lodbias` (given in [`Texture::set_lod_bias()`])
    /// is clamped so that it is never less than the minimum extent of the pixel projected in
    /// texture space. This prevents over-biasing the LOD when the polygon is perpendicular to the
    /// view direction.
    pub fn set_bias_clamp(&mut self, enable: bool) {
        unsafe { ffi::GX_InitTexObjBiasClamp(&mut self.inner, enable as u8) }
    }

    /// Changes LOD computing mode.
    ///
    /// When set to `true`, the LOD is computed using adjacent texels; when `false`, diagonal
    /// texels are used instead. This should be set to `true` if you use bias clamping (see
    /// [`Texture::set_bias_clamp()`]) or anisotropic filtering (`GX_ANISO_2` or `GX_ANISO_4` for
    /// [`Texture::set_max_aniso()`] argument).
    pub fn set_edge_lod(&mut self, enable: bool) {
        unsafe { ffi::GX_InitTexObjEdgeLOD(&mut self.inner, enable as u8) }
    }

    /// Sets the filter mode for a texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjFilterMode()`] for more.
    pub fn set_filter_mode(&mut self, minfilt: TexFilter, magfilt: TexFilter) {
        debug_assert!(
            matches!(magfilt, TexFilter::Near | TexFilter::Linear),
            "magfilt can only be `TexFilter::Near` or `TexFilter::Linear`"
        );
        unsafe { ffi::GX_InitTexObjFilterMode(&mut self.inner, minfilt as u8, magfilt as u8) }
    }

    /// Sets texture Level Of Detail (LOD) controls explicitly for a texture object.
    ///
    /// See [`ogc_sys::GX_InitTexObjLOD()`] for more.
    pub fn set_lod(
        &mut self,
        filters: (TexFilter, TexFilter),
        lod_range: (f32, f32),
        lodbias: f32,
        biasclamp: bool,
        edgelod: bool,
        maxaniso: u8,
    ) {
        debug_assert!(
            (0.0..=10.0).contains(&lod_range.0),
            "valid range for min LOD is 0.0 to 10.0"
        );
        debug_assert!(
            (0.0..=10.0).contains(&lod_range.1),
            "valid range for max LOD is 0.0 to 10.0"
        );
        debug_assert!(
            matches!(filters.1, TexFilter::Near | TexFilter::Linear),
            "magfilt can only be `TexFilter::Near` or `TexFilter::Linear`"
        );
        debug_assert!(
            !(biasclamp || maxaniso == 1 || maxaniso == 2) || edgelod,
            "`edgelod` should be set if `biasclamp` is set or `maxaniso` is set to `GX_ANISO_2` \
            or `GX_ANISO_4`."
        );
        unsafe {
            ffi::GX_InitTexObjLOD(
                &mut self.inner,
                filters.0 as u8,
                filters.1 as u8,
                lod_range.0,
                lod_range.1,
                lodbias,
                biasclamp as u8,
                edgelod as u8,
                maxaniso,
            );
        }
    }

    /// Sets the LOD bias for a given texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjLODBias()`] for more.
    pub fn set_lod_bias(&mut self, lodbias: f32) {
        unsafe { ffi::GX_InitTexObjLODBias(&mut self.inner, lodbias) }
    }

    /// Sets the maximum anisotropic filter to use for a texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjMaxAniso()`] for more.
    pub fn set_max_aniso(&mut self, maxaniso: u8) {
        unsafe { ffi::GX_InitTexObjMaxAniso(&mut self.inner, maxaniso) }
    }

    /// Sets the maximum LOD for a given texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjMaxLOD()`] for more.
    pub fn set_max_lod(&mut self, maxlod: f32) {
        debug_assert!(
            (0.0..=10.0).contains(&maxlod),
            "valid range for max LOD is 0.0 to 10.0"
        );
        unsafe { ffi::GX_InitTexObjMaxLOD(&mut self.inner, maxlod) }
    }

    /// Sets the minimum LOD for a given texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjMinLOD()`] for more.
    pub fn set_min_lod(&mut self, minlod: f32) {
        debug_assert!(
            (0.0..=10.0).contains(&minlod),
            "valid range for min LOD is 0.0 to 10.0"
        );
        unsafe { ffi::GX_InitTexObjMinLOD(&mut self.inner, minlod) }
    }

    /// Sets the TLUT that is associated with this texture.
    ///
    /// See [`ogc_sys::GX_InitTexObjTlut()`] for more.
    pub fn set_tlut(&mut self, tlut_name: u32) {
        unsafe { ffi::GX_InitTexObjTlut(&mut self.inner, tlut_name) }
    }

    /// Sets the texture coordinate wrap modes for this texture.
    pub fn set_wrap_mode(&mut self, wrap_s: WrapMode, wrap_t: WrapMode) {
        unsafe { ffi::GX_InitTexObjWrapMode(&mut self.inner, wrap_s as u8, wrap_t as u8) }
    }

    pub(crate) fn as_ptr(&self) -> *const RawTexture {
        self as *const _
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut RawTexture {
        self as *mut _
    }
}

/// Size of texture cache regions.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TexCacheSize {
    /// 32 kilobytes
    Small = ffi::GX_TEXCACHE_32K,
    /// 128 kilobytes
    Medium = ffi::GX_TEXCACHE_128K,
    /// 512 kilobytes
    Large = ffi::GX_TEXCACHE_512K,
    None = ffi::GX_TEXCACHE_NONE,
}

/// Stores information on a texture cache region.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RawTexRegion(ffi::GXTexRegion);

/// A texture cache region with managed internal buffers
#[derive(Debug, Clone)]
pub struct TexRegion {
    inner: RawTexRegion,
    buf_even: Buf32,
    buf_odd: Buf32,
}

impl Deref for TexRegion {
    type Target = RawTexRegion;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TexRegion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TexRegion {
    /// Initializes a texture memory (TMEM) region object for cache.
    ///
    /// See [`ogc_sys::GX_InitTexCacheRegion()`] for more.
    pub fn new(size_even: TexCacheSize, size_odd: TexCacheSize, is_32b_mipmap: bool) -> Self {
        // libogc doesn't initialize all fields
        let mut gxtexregion = MaybeUninit::zeroed();
        let mut buf_even = Buf32::new(match size_even {
            TexCacheSize::Large => 512 * 1024,
            TexCacheSize::Medium => 128 * 1024,
            _ => 32 * 1024,
        });
        let mut buf_odd = Buf32::new(match size_odd {
            TexCacheSize::Large => 512 * 1024,
            TexCacheSize::Medium => 128 * 1024,
            _ => 32 * 1024,
        });
        unsafe {
            ffi::GX_InitTexCacheRegion(
                gxtexregion.as_mut_ptr(),
                is_32b_mipmap as u8,
                buf_even.as_mut_ptr() as u32,
                size_even as _,
                buf_odd.as_mut_ptr() as u32,
                size_odd as _,
            );

            TexRegion {
                inner: RawTexRegion(gxtexregion.assume_init()),
                buf_even,
                buf_odd,
            }
        }
    }

    /// Initializes a Texture Memory (TMEM) region object for preloading.
    ///
    /// See [`ogc_sys::GX_InitTexPreloadRegion()`] for more.
    pub fn new_preloaded(size_even: TexCacheSize, size_odd: TexCacheSize) -> Self {
        // libogc doesn't initialize all fields
        let mut gxtexregion = MaybeUninit::zeroed();
        let mut buf_even = Buf32::new(match size_even {
            TexCacheSize::Large => 512 * 1024,
            TexCacheSize::Medium => 128 * 1024,
            _ => 32 * 1024,
        });
        let mut buf_odd = Buf32::new(match size_odd {
            TexCacheSize::Large => 512 * 1024,
            TexCacheSize::Medium => 128 * 1024,
            _ => 32 * 1024,
        });
        unsafe {
            ffi::GX_InitTexPreloadRegion(
                gxtexregion.as_mut_ptr(),
                buf_even.as_mut_ptr() as u32,
                size_even as _,
                buf_odd.as_mut_ptr() as u32,
                size_odd as _,
            );

            TexRegion {
                inner: RawTexRegion(gxtexregion.assume_init()),
                buf_even,
                buf_odd,
            }
        }
    }
}

impl RawTexRegion {
    /// Invalidates the texture cache in Texture Memory (TMEM) described by region.
    ///
    /// See [`ogc_sys::GX_InvalidateTexRegion()`] for more.
    pub fn invalidate(&mut self) {
        unsafe { ffi::GX_InvalidateTexRegion(&mut self.0) }
    }
}

/// Format of the entries in the TLUT
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TlutFormat {
    IA8 = ffi::GX_TL_IA8,
    RGB565 = ffi::GX_TL_RGB565,
    RGB5A3 = ffi::GX_TL_RGB5A3,
}


/// Texture Look-Up Table with an internal buffer
#[derive(Debug, Clone)]
pub struct Tlut {
    inner: ffi::GXTlutObj,
    buf: Buf32,
}

impl Tlut {
    /// Initializes a Texture Look-Up Table (TLUT) object.
    ///
    /// See [`ogc_sys::GX_InitTlutObj()`] for more.
    pub fn new(mut lut: Buf32, fmt: TlutFormat, entries: u16) -> Self {
        // libogc doesn't check entries is within range. unsure of consequences, so let's just
        // play it safe.
        assert!(entries <= 16384);
        let mut obj = core::mem::MaybeUninit::uninit();
        unsafe {
            ffi::GX_InitTlutObj(obj.as_mut_ptr(), lut.as_mut_ptr().cast(), fmt as _, entries);
            // SAFETY: per libogc source, it zeroes and then initializes obj itself.
            Tlut {
                inner: obj.assume_init(),
                buf: lut,
            }
        }
    }
}


/// Vertex attribute array type
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum VtxAttr {
    PtnMtxIdx = ffi::GX_VA_PTNMTXIDX,
    Tex0MtxIdx = ffi::GX_VA_TEX0MTXIDX,
    Tex1MtxIdx = ffi::GX_VA_TEX1MTXIDX,
    Tex2MtxIdx = ffi::GX_VA_TEX2MTXIDX,
    Tex3MtxIdx = ffi::GX_VA_TEX3MTXIDX,
    Tex4MtxIdx = ffi::GX_VA_TEX4MTXIDX,
    Tex5MtxIdx = ffi::GX_VA_TEX5MTXIDX,
    Tex6MtxIdx = ffi::GX_VA_TEX6MTXIDX,
    Tex7MtxIdx = ffi::GX_VA_TEX7MTXIDX,
    Pos = ffi::GX_VA_POS,
    Nrm = ffi::GX_VA_NRM,
    Color0 = ffi::GX_VA_CLR0,
    Color1 = ffi::GX_VA_CLR1,
    Tex0 = ffi::GX_VA_TEX0,
    Tex1 = ffi::GX_VA_TEX1,
    Tex2 = ffi::GX_VA_TEX2,
    Tex3 = ffi::GX_VA_TEX3,
    Tex4 = ffi::GX_VA_TEX4,
    Tex5 = ffi::GX_VA_TEX5,
    Tex6 = ffi::GX_VA_TEX6,
    Tex7 = ffi::GX_VA_TEX7,
    PosMtxArray = ffi::GX_POSMTXARRAY,
    NrmMtxArray = ffi::GX_NRMMTXARRAY,
    TexMtxArray = ffi::GX_TEXMTXARRAY,
    LightArray = ffi::GX_LIGHTARRAY,
    /// Normal, binormal, tangent
    Nbt = ffi::GX_VA_NBT,
    #[doc(hidden)]
    MaxAttr = ffi::GX_VA_MAXATTR,
    Null = ffi::GX_VA_NULL,
}

/// Specifies which perspective type to use
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ProjectionType {
    Perspective = ffi::GX_PERSPECTIVE,
    Orthographic = ffi::GX_ORTHOGRAPHIC,
}

#[derive(Clone, Copy, Debug)]
pub struct GpStatus {
    /// `true` if high watermark has been passed, or in the case when the CPU and GP FIFOs are the
    /// same, then `true` if the current GX thread is suspended.
    pub overhi: bool,
    /// `true` if low watermark has been passed.
    pub underlo: bool,
    /// `true` if the GP read unit is idle.
    pub read_idle: bool,
    /// `true` if all commands have been flushed to XF.
    pub cmd_idle: bool,
    /// `true` if FIFO has reached a breakpoint and GP reads have been stopped.
    pub brkpt: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RenderMode(GXRModeObj);

impl RenderMode {
    /// Takes a given render mode and returns a version that is reduced in size to account for overscan.
    ///
    /// See [`ogc_sys::GX_AdjustForOverscan()`] for more.
    pub fn adjust_for_overscan(mut self, hor: u16, ver: u16) -> Self {
        let mut rmout = core::mem::MaybeUninit::uninit();
        unsafe {
            ffi::GX_AdjustForOverscan(&mut self.0, rmout.as_mut_ptr(), hor, ver);
            RenderMode(rmout.assume_init())
        }
    }
}

/// Fog equation control
#[repr(u32)]
pub enum FogType {
    None = ffi::GX_FOG_NONE,
    PerspLin = ffi::GX_FOG_PERSP_LIN,
    PerspExp = ffi::GX_FOG_PERSP_EXP,
    PerspExp2 = ffi::GX_FOG_PERSP_EXP2,
    PerspRevExp = ffi::GX_FOG_PERSP_REVEXP,
    PerspRevExp2 = ffi::GX_FOG_PERSP_REVEXP2,
    OrthoLin = ffi::GX_FOG_ORTHO_LIN,
    OrthoExp = ffi::GX_FOG_ORTHO_EXP,
    OrthoExp2 = ffi::GX_FOG_ORTHO_EXP2,
    OrthoRevExp = ffi::GX_FOG_ORTHO_REVEXP,
    OrthoRevExp2 = ffi::GX_FOG_ORTHO_REVEXP2,
}

/// Fog range adjustment parameter table.
/// 
/// This stores an array of 10 range points in `u4.8` fixed-point format.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FogAdjTable(ffi::GXFogAdjTbl);

impl FogAdjTable {
    /// Generates the standard range adjustment table.
    /// 
    /// See [`ogc_sys::GX_InitFogAdjTable()`] for more.
    pub fn new(width: u16, mut projmtx: [[f32; 4]; 4]) -> Self {
        let mut table = MaybeUninit::uninit();
        unsafe {
            ffi::GX_InitFogAdjTable(table.as_mut_ptr(), width, projmtx.as_mut_ptr());
            FogAdjTable(table.assume_init())
        }
    }
    
    pub fn as_array(&self) -> &[u16; 10] {
        &self.0.r
    }
    
    pub fn as_array_mut(&mut self) -> &mut [u16; 10] {
        &mut self.0.r
    }
}

#[repr(u32)]
pub enum MtxType {
    M3x4 = ffi::GX_MTX3x4,
    M2x4 = ffi::GX_MTX2x4,
}

/// Initializes the graphics processor to its initial state.
/// 
/// This function takes a size for the FIFO buffer, and returns an atomic pointer to a
/// raw FIFO that uses that buffer. It will initialize a `Buf32`, wrap it with
/// `ManuallyDrop`, pass it to libogc, and wrap the FIFO pointer it returns in an
/// `AtomicPtr`. This is due to the FIFO being in static memory, which requires atomics
/// for safe access.
/// 
/// If this function has already been called, it just returns the same pointer again.
///
/// See [`ogc_sys::GX_Init()`] for more.
pub fn init(size: usize) -> AtomicPtr<RawFifo> {
    let init_fifo = INIT_FIFO.load(Ordering::Acquire);
    
    // check if GX has already been initialized
    if ! init_fifo.is_null() {
        return AtomicPtr::new(init_fifo);
    }
    
    // keep buf around with ManuallyDrop, otherwise it will be deallocated
    // by the end of the function.
    let mut buf = ManuallyDrop::new(Buf32::new(size.max(RawFifo::MIN_SIZE)));

    // SAFETY: all safety is ensured by Buf32.
    let gxfifo = unsafe {
        ffi::GX_Init(
            buf.as_mut_ptr().map_addr(mem::to_uncached).cast(),
            buf.len() as u32,
        )
    };

    // Mark GX as initialized.
    let fifo_ptr = gxfifo.cast();
    INIT_FIFO.store(fifo_ptr, Ordering::Release);
    AtomicPtr::new(fifo_ptr)
}

/// Return a copy of the currently attached GP FIFO.
///
/// See [`ogc_sys::GX_GetGPFifo()`] for more.
pub fn get_gp_fifo() -> Option<RawFifo> {
    if INIT_FIFO.load(Ordering::Acquire).is_null() {
        None
    } else {
        let mut rawfifo: MaybeUninit<RawFifo> = MaybeUninit::zeroed();
        unsafe {
            ffi::GX_GetGPFifo(rawfifo.as_mut_ptr().cast());
            Some(rawfifo.assume_init())
        }
    }
}

/// Attaches a FIFO to the GP.
///
/// See [`ogc_sys::GX_SetGPFifo()`] for more.
pub fn set_gp_fifo(fifo: Option<&RawFifo>) {
    let fifo_ptr = match fifo {
        Some(fifo) => fifo,
        None => ptr::null(),
    };
    unsafe { ffi::GX_SetGPFifo(fifo_ptr.cast_mut().cast()) }
}

/// Return a copy of the currently attached CPU FIFO.
///
/// See [`ogc_sys::GX_GetCPUFifo()`] for more.
pub fn get_cpu_fifo() -> Option<RawFifo> {
    if INIT_FIFO.load(Ordering::Acquire).is_null() {
        None
    } else {
        let mut rawfifo: MaybeUninit<RawFifo> = MaybeUninit::zeroed();
        unsafe {
            ffi::GX_GetCPUFifo(rawfifo.as_mut_ptr().cast());
            Some(rawfifo.assume_init())
        }
    }
}

/// Attaches a FIFO to the CPU.
///
/// See [`ogc_sys::GX_SetCPUFifo()`] for more.
pub fn set_cpu_fifo(fifo: Option<&RawFifo>) {
    let fifo_ptr = match fifo {
        Some(fifo) => fifo,
        None => ptr::null(),
    };
    unsafe { ffi::GX_SetCPUFifo(fifo_ptr.cast_mut().cast()) }
}

/// Returns the current GX thread.
/// 
/// See [`ogc_sys::GX_GetCurrentGXThread()`] for more.
pub fn get_current_gx_thread() -> lwp::Thread {
    lwp::Thread::new(unsafe { ffi::GX_GetCurrentGXThread() })
}

/// Aborts the current frame.
/// 
/// See [`ogc_sys::GX_AbortFrame()`] for more.
pub fn abort_frame() {
    unsafe { ffi::GX_AbortFrame() }
}

/// Clears the bounding box values before a new image is drawn.
///
/// The graphics hardware keeps track of a bounding box of pixel coordinates that are drawn in
/// the Embedded Frame Buffer (EFB).
pub fn clear_bounding_box() {
    unsafe { ffi::GX_ClearBoundingBox() }
}

/// Clears the two virtual GP performance counters to zero.
///
/// When in display list mode, this is a no-op.
///
/// See [`ogc_sys::GX_ClearGPMetric()`] for more.
pub fn clear_gp_metric() {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        unsafe { ffi::GX_ClearGPMetric() }
    }
}

/// Clears the Vertex Cache performance counter.
/// 
/// See [`ogc_sys::GX_ClearVCacheMetric()`] for more.
pub fn clear_vcache_metric() {
    unsafe { ffi::GX_ClearVCacheMetric() }
}

/// Sets a breakpoint that causes the GP to halt when encountered.
///
/// See [`ogc_sys::GX_EnableBreakPt()`] for more.
pub fn enable_breakpt(break_pt: *mut u8) {
    unsafe { ffi::GX_EnableBreakPt(break_pt as _) }
}

/// Allows reads from the FIFO currently attached to the Graphics Processor (GP) to resume.
///
/// See [`ogc_sys::GX_DisableBreakpt()`] for more.
pub fn disable_breakpt() {
    unsafe { ffi::GX_DisableBreakPt() }
}

/// Initialize the transformation unit (XF) rasterizer unit (RAS) to take performance
/// measurements.
///
/// # Safety
/// This function should be avoided; use the GP performance metric functions instead.
pub unsafe fn init_xf_ras_metric() {
    unsafe { ffi::GX_InitXfRasMetric() }
}

/// Reads the current status of the GP.
/// 
/// See [`ogc_sys::GX_GetGPStatus()`] for more.
pub fn get_gp_status() -> GpStatus {
    let mut ret = (0, 0, 0, 0, 0);
    unsafe {
        ffi::GX_GetGPStatus(&mut ret.0, &mut ret.1, &mut ret.2, &mut ret.3, &mut ret.4);
    }
    GpStatus {
        overhi: ret.0 != 0,
        underlo: ret.1 != 0,
        read_idle: ret.2 != 0,
        cmd_idle: ret.3 != 0,
        brkpt: ret.4 != 0,
    }
}

/// Loads a light object into a set of hardware registers associated with a Light ID.
///
/// See [`ogc_sys::GX_LoadLightObj()`] for more.
pub fn load_light(lit_obj: &Light, lit_id: u8) {
    unsafe { ffi::GX_LoadLightObj(&lit_obj.0, lit_id) }
}

/// Instructs the GP to fetch the light object at *litobjidx* from an array.
///
/// See [`ogc_sys::GX_LoadLightObjIdx()`] for more.
pub fn load_light_idx(litobjidx: u32, litid: u8) {
    unsafe { ffi::GX_LoadLightObjIdx(litobjidx, litid) }
}

/// Causes the GPU to wait for the pipe to flush.
///
/// When in a display list, this is a no-op.
///
/// See [`ogc_sys::GX_PixModeSync()`] for more.
pub fn pix_mode_sync() {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        unsafe { ffi::GX_PixModeSync() }
    }
}

/// Temporarily points the CPU's write-gather pipe at a new location.
/// 
/// If the pipe has already been redirected, this will return `None`. Otherwise, it will
/// return a pointer that can be written to with `ptr::write_volatile()`, which will be
/// sent to a destination buffer.
/// 
/// See [`ogc_sys::GX_RedirectWriteGatherPipe()`] for more.
/// 
/// # Safety
/// **No GX commands can be called until the write-gather pipe is restored. You MUST call
/// [`restore_write_gather_pipe()`] before calling this function again.**
pub unsafe fn redirect_write_gather_pipe(ptr: *mut u8) -> Option<*mut u8> {
    debug_assert!(ptr.is_aligned_to(32));
    if ! WGPIPE_REDIRECTED.swap(true, Ordering::AcqRel) {
        Some(unsafe { ffi::GX_RedirectWriteGatherPipe(ptr.cast()).cast() })
    } else {
        None
    }
}

/// Restores the write-gather pipe.
///
/// See [`ogc_sys::GX_RestoreWriteGatherPipe()`] for more.
pub fn restore_write_gather_pipe() {
    WGPIPE_REDIRECTED.store(false, Ordering::Release);
    unsafe { ffi::GX_RestoreWriteGatherPipe() }
}

/// Sends a DrawDone command to the GP.
///
/// See [`ogc_sys::GX_SetDrawDone()`] for more.
pub fn set_draw_done() {
    unsafe { ffi::GX_SetDrawDone() }
}

/// Inserts a synchronization command into the graphics FIFO. When the Graphics Processor sees
/// this command, it will allow the texture pipeline to flush before continuing.
///
/// See [`ogc_sys::GX_TexModeSync()`] for more.
pub fn tex_mode_sync() {
    unsafe { ffi::GX_TexModeSync() }
}

/// Stalls until DrawDone is encountered by the GP.
///
/// It means all graphics commands sent before this *DrawDone* command have executed and the
/// last pixel has been written to the frame buffer. You may want to execute some non-graphics
/// operations between executing [`gx::set_draw_done()`] and this function, but if you simply
/// want to wait and have nothing to execute, you can use [`gx::draw_done()`].
///
/// # Note
/// This function is normally used in immediate mode (see [`gx::set_cpu_fifo()`]). In
/// multibuffer mode, sending the 'done' command is separated from polling the 'done' status
/// (see [`gx::set_draw_done()`] and [`gx::wait_draw_done()`]).
/// 
/// See [`ogc_sys::GX_WaitDrawDone()`] for more.
pub fn wait_draw_done() {
    unsafe { ffi::GX_WaitDrawDone() }
}

/// Sets whether Z buffering happens before or after texturing.
///
/// Normally, Z buffering should happen before texturing, as this enables better performance by
/// not texturing pixels that are not visible; however, when alpha compare is used, Z buffering
/// must be done after texturing (see [`gx::set_alpha_compare()`]).
/// 
/// See [`ogc_sys::GX_SetZCompLoc()`] for more.
pub fn set_zcomp_loc(before_tex: bool) {
    unsafe { ffi::GX_SetZCompLoc(before_tex as u8) }
}

/// Sets color and Z value to clear the EFB to during copy operations.
/// 
/// See [`ogc_sys::GX_SetCopyClear()`] for more.
pub fn set_copy_clear(background: Color, z_value: u32) {
    unsafe { ffi::GX_SetCopyClear(background.0, z_value) };
}

/// Sets the viewport rectangle in screen coordinates.
/// 
/// See [`ogc_sys::GX_SetViewport()`] for more.
pub fn set_viewport(x_orig: f32, y_orig: f32, wd: f32, hd: f32, near_z: f32, far_z: f32) {
    unsafe { ffi::GX_SetViewport(x_orig, y_orig, wd, hd, near_z, far_z) }
}

/// Calculates an appropriate Y scale factor value for [`set_disp_copy_y_scale()`] based
/// on the height of the EFB and the height of the XFB.
/// 
/// See [`ogc_sys::GX_GetYScaleFactor()`] for more.
pub fn get_y_scale_factor(efb_height: u16, xfb_height: u16) -> f32 {
    debug_assert!((2..=528).contains(&efb_height));
    debug_assert!(efb_height.is_multiple_of(2));
    debug_assert!((2..=1024).contains(&xfb_height));
    debug_assert!(xfb_height >= efb_height);
    unsafe { ffi::GX_GetYScaleFactor(efb_height, xfb_height) }
}

/// Sets the vertical scale factor for the EFB to XFB copy operation.
/// 
/// See [`ogc_sys::GX_SetDispCopyYScale()`] for more.
pub fn set_disp_copy_y_scale(y_scale: f32) -> u32 {
    unsafe { ffi::GX_SetDispCopyYScale(y_scale) }
}

/// Sets the scissor rectangle.
/// 
/// See [`ogc_sys::GX_SetScissor()`] for more.
pub fn set_scissor(x_origin: u32, y_origin: u32, wd: u32, hd: u32) {
    unsafe { ffi::GX_SetScissor(x_origin, y_origin, wd, hd) }
}

/// Sets the source parameters for the EFB to XFB copy operation.
/// 
/// See [`ogc_sys::GX_SetDispCopySrc()`] for more.
pub fn set_disp_copy_src(left: u16, top: u16, wd: u16, ht: u16) {
    debug_assert!(left % 2 == 0, "left is not divisible by 2");
    debug_assert!(top % 2 == 0, "top is not divisible by 2");
    debug_assert!(wd % 2 == 0, "width is not divisible by 2");
    debug_assert!(ht % 2 == 0, "height is not divisible by 2");
    unsafe { ffi::GX_SetDispCopySrc(left, top, wd, ht) }
}

/// Sets the width and height of the display buffer in pixels.
/// 
/// See [`ogc_sys::GX_SetDispCopyDst()`] for more.
pub fn set_disp_copy_dst(width: u16, height: u16) {
    debug_assert!(
        width.is_multiple_of(16),
        "width must be a multiple of 16, got {width}"
    );
    unsafe { ffi::GX_SetDispCopyDst(width, height) }
}

/// Sets the subpixel sample patterns and vertical filter coefficients used to filter
/// subpixels into pixels.
/// 
/// See [`ogc_sys::GX_SetCopyFilter()`] for more.
pub fn set_copy_filter(
    sample_pattern: Option<[[u8; 2]; 12]>,
    vfilter: Option<[u8; 7]>,
) {
    if let Some(sp) = sample_pattern {
        debug_assert!(
            sp.iter().flatten().all(|x| (1..=11).contains(x)),
            "valid range for sample coordinate points is 1..=11, got {sp:?}"
        );
    }
    if let Some(vf) = vfilter {
        debug_assert!(
            vf.iter().all(|x| (0..=63).contains(x)),
            "valid range for vertical filter coefficients is 0..=63, got {vf:?}"
        );
        let sum: u8 = vf.iter().sum();
        debug_assert_eq!(
            sum,
            64,
            "sum of vertical filter coefficients should be 64, got {sum}"
        );
    }
    
    unsafe {
        ffi::GX_SetCopyFilter(
            sample_pattern.is_some() as u8,
            sample_pattern.unwrap_or_default().as_mut_ptr(),
            vfilter.is_some() as u8,
            vfilter.unwrap_or_default().as_mut_ptr()
        )
    }
}

/// Sets the lighting controls for a particular color channel.
/// 
/// See [`ogc_sys::GX_SetChanCtrl()`] for more.
pub fn set_channel_controls(
    channel: ColorChannel,
    enable: bool,
    ambsrc: ColorChanSrc,
    matsrc: ColorChanSrc,
    litmask: u8,
    diff_fn: DiffFn,
    attn_fn: AttnFn,
) {
    unsafe {
        ffi::GX_SetChanCtrl(
            channel as i32,
            enable as u8,
            ambsrc as u8,
            matsrc as u8,
            litmask,
            diff_fn as u8,
            attn_fn as u8,
        );
    }
}

/// Controls various rasterization and texturing parameters that relate to field-mode and
/// double-strike rendering.
/// 
/// See [`ogc_sys::GX_SetFieldMode()`] for more.
pub fn set_field_mode(field_mode: bool, half_aspect_ratio: bool) {
    unsafe { ffi::GX_SetFieldMode(field_mode as u8, half_aspect_ratio as u8) }
}

/// Sets the format of pixels in the Embedded Frame Buffer (EFB).
/// 
/// See [`ogc_sys::GX_SetPixelFmt()`] for more.
pub fn set_pixel_fmt(pix_fmt: PixelFormat, z_fmt: ZFormat) {
    unsafe { ffi::GX_SetPixelFmt(pix_fmt as u8, z_fmt as u8) }
}

/// Enables or disables culling of geometry based on its orientation to the viewer.
///
/// Primitives in which the vertex order is clockwise to the viewer are considered front-facing.
///
/// See [`ogc_sys::GX_SetCullMode()`] for more.
pub fn set_cull_mode(mode: CullMode) {
    unsafe { ffi::GX_SetCullMode(mode as u8) }
}

/// Sets the current GX thread to the calling thread.
///
/// The new thread should be the thread that will be responsible for
/// generating graphics data. By default, the GX thread is the thread that
/// invoked [`gx::init()`]; however, it may be changed by calling this
/// function.
///
/// Returns the previous GX thread ID.
///
/// # Note
/// It is a programming error to change GX thread while the current GX
/// thread is suspended by a high water mark interrupt. This indicates that
/// you have two threads about to generate GX data.
///
/// When graphics data is being generated in immediate mode (that is, the
/// CPU FIFO = GP FIFO, and the GP is actively consuming data), the high
/// watermark may be triggered. When this happens, the high watermark
/// interrupt handler will suspend the GX thread, thus preventing any
/// further graphics data from being generated. The low watermark interrupt
/// handler will resume the thread.
/// 
/// See [`ogc_sys::GX_SetCurrentGXThread()`] for more.
pub fn set_current_gx_thread() -> lwp::Thread {
    lwp::Thread::new(unsafe { ffi::GX_SetCurrentGXThread() })
}

/// Copies the embedded framebuffer (EFB) to the external framebuffer(XFB) in main memory.
///
/// # Safety
/// The user must ensure the dest ptr is a valid pointer,
///
/// See [`ogc_sys::GX_CopyDisp()`] for more.
pub unsafe fn copy_disp(dest: *mut c_void, clear: bool) {
    unsafe { ffi::GX_CopyDisp(dest, clear as u8) }
}

/// Sets the gamma correction applied to pixels during EFB to XFB copy operation.
/// 
/// See [`ogc_sys::GX_SetDispCopyGamma()`] for more.
pub fn set_disp_copy_gamma(gamma: Gamma) {
    unsafe { ffi::GX_SetDispCopyGamma(gamma as _) }
}

/// Sets the attribute format for a single attribute in the Vertex Attribute Table (VAT).
/// 
/// See [`ogc_sys::GX_SetVtxAttrFmt()`] for more.
pub fn set_vtx_attr_fmt(vtxfmt: u8, vtxattr: VtxAttr, comptype: u32, compsize: u32, frac: u32) {
    // this is debug_assert because libogc just uses the lowest 3 bits
    debug_assert!(
        vtxfmt < ffi::GX_MAXVTXFMT as u8,
        "index out of bounds: the len is {} but the index is {}",
        ffi::GX_MAXVTXFMT,
        vtxfmt,
    );
    unsafe { ffi::GX_SetVtxAttrFmt(vtxfmt, vtxattr as u32, comptype, compsize, frac) }
}

/// Sets the number of color channels that are output to the TEV stages.
/// 
/// See [`ogc_sys::GX_SetNumChans()`] for more.
pub fn set_num_chans(num: u8) {
    unsafe { ffi::GX_SetNumChans(num) }
}

/// Sets the number of texture coordinates that are generated and available for use in the
/// Texture Environment TEV stages.
/// 
/// See [`ogc_sys::GX_SetNumTexGens()`] for more.
pub fn set_num_tex_gens(nr: u32) {
    unsafe { ffi::GX_SetNumTexGens(nr) }
}

/// Simplified function to set various TEV parameters for this tevstage based on a
/// predefined combiner mode.
/// 
/// See [`ogc_sys::GX_SetTevOp()`] for more.
pub fn set_tev_op(tevstage: u8, mode: u8) {
    unsafe { ffi::GX_SetTevOp(tevstage, mode) }
}

/// Specifies the texture and rasterized color that will be available as inputs to
/// this TEV tevstage.
/// 
/// Instead of having `texmap` be a `u32` to account for the `GX_TEXMAP_DISABLE` flag,
/// this function splits it into its own `bool` that can be set. If you don't use this
/// flag, you can safely set `disable_texmap_lookup` to false.
/// 
/// See [`ogc_sys::GX_SetTevOrder()`] for more.
pub fn set_tev_order(
    tevstage: u8,
    texcoord: Option<u8>,
    disable_texmap_lookup: bool,
    texmap: Option<u8>,
    color: Option<ColorChannel>
) {
    debug_assert!(tevstage <= 15);
    debug_assert!(if let Some(x) = texcoord { x <= 7 } else { true });
    debug_assert!(if let Some(x) = texmap { x <= 7 } else { true });
    let disable_flag = if disable_texmap_lookup { ffi::GX_TEXMAP_DISABLE } else { 0 };
    unsafe {
        ffi::GX_SetTevOrder(
            tevstage,
            texcoord.unwrap_or(ffi::GX_TEXCOORDNULL as u8),
            texmap.unwrap_or(ffi::GX_TEXMAPNULL as u8) as u32 | disable_flag,
            color.map(|ch| ch as u8).unwrap_or(ffi::GX_COLORNULL as u8)
        )
    }
}

/// Specifies how texture coordinates are generated.
/// 
/// See [`ogc_sys::GX_SetTexCoordGen()`] for more.
pub fn set_tex_coord_gen(texcoord: u16, tgen_typ: u32, tgen_src: u32, mtxsrc: u32) {
    debug_assert!([30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60].contains(&mtxsrc));
    unsafe { ffi::GX_SetTexCoordGen(texcoord, tgen_typ, tgen_src, mtxsrc) }
}

/// Invalidates the current caches of the Texture Memory (TMEM).
///
/// See [`ogc_sys::GX_InvalidateTexAll()`] for more.
pub fn invalidate_tex_all() {
    unsafe { ffi::GX_InvalidateTexAll() }
}

/// Loads the state describing a texture into one of eight hardware register sets.
///
/// See [`ogc_sys::GX_LoadTexObj()`] for more.
///
/// # Safety
/// If the texture is a color-index texture, you **must** load the associated TLUT (using
/// [`load_tlut()`]) before calling this function.
pub fn load_texture(obj: &mut RawTexture, mapid: u8) {
    unsafe { ffi::GX_LoadTexObj(&mut obj.inner, mapid) }
}

/// Sets the projection matrix.
/// 
/// See [`ogc_sys::GX_LoadProjectionMtx()`] for more.
pub fn load_projection_mtx(matrix: &Mtx44, projection: ProjectionType) {
    unsafe { ffi::GX_LoadProjectionMtx(matrix.as_ptr().cast_mut(), projection as u8) }
}

/// Clears all vertex attributes of the current vertex descriptor to `VtxAttrIn::None`.
///
/// See [`ogc_sys::GX_ClearVtxDesc()`] for more.
pub fn clear_vtx_desc() {
    unsafe { ffi::GX_ClearVtxDesc() }
}

/// Sets the type of a single attribute in the current vertex descriptor.
/// 
/// See [`ogc_sys::GX_SetVtxDesc()`] for more.
pub fn set_vtx_desc(attr: VtxAttr, v_type: VtxAttrIn) {
    unsafe { ffi::GX_SetVtxDesc(attr as u8, v_type as _) }
}

/// Used to load a 3x4 modelview matrix into matrix memory at location `pnidx`.
/// 
/// See [`ogc_sys::GX_LoadPosMtxImm()`] for more.
pub fn load_pos_mtx_imm(mut mt: Mtx34, pnidx: u32) {
    unsafe { ffi::GX_LoadPosMtxImm(mt.as_mut_ptr(), pnidx) }
}

/// Loads a 3x4 modelview matrix at index `mtxidx` from the array in main memory.
/// 
/// See [`ogc_sys::GX_LoadPosMtxIdx()`] for more.
pub fn load_pos_mtx_idx(mtxidx: u16, pnidx: u32) {
    unsafe { ffi::GX_LoadPosMtxIdx(mtxidx, pnidx) }
}

/// Used to load a normal transform matrix into matrix memory at location `pnidx` from
/// the 4x3 matrix `mt`.
/// 
/// See [`ogc_sys::GX_LoadNrmMtxImm()`] for more.
pub fn load_nrm_mtx_imm(mut mt: Mtx34, pnidx: u32) {
    unsafe { ffi::GX_LoadNrmMtxImm(mt.as_mut_ptr(), pnidx) }
}

/// Loads a 3x3 normal matrix into matrix memory at location `pnidx` from a 3x3 matrix
/// located at index `mtxidx` from the array in main memory.
/// 
/// See [`ogc_sys::GX_LoadNrmMtxIdx3x3()`] for more.
pub fn load_nrm_mtx_idx_3x3(mtxidx: u16, pnidx: u32) {
    unsafe { ffi::GX_LoadNrmMtxIdx3x3(mtxidx, pnidx) }
}

/// Used to load a normal transform matrix into matrix memory at location `pnidx` from
/// the 3x3 matrix `mt`.
/// 
/// See [`ogc_sys::GX_LoadNrmMtxImm3x3()`] for more.
pub fn load_nrm_mtx_imm_3x3(mut mt: Mtx33, pnidx: u32) {
    unsafe { ffi::GX_LoadNrmMtxImm3x3(mt.as_mut_ptr(), pnidx) }
}

/// Loads a texture matrix `mt` into the matrix memory at location `texidx`.
/// 
/// See [`ogc_sys::GX_LoadTexMtxImm()`] for more.
pub fn load_tex_mtx_imm(mut mt: Mtx34, texidx: u32, r#type: MtxType) {
    debug_assert!([30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60].contains(&texidx));
    unsafe { ffi::GX_LoadTexMtxImm(mt.as_mut_ptr(), texidx, r#type as _) }
}

/// Loads a texture matrix at index `mtxidx` from the array in main memory.
/// 
/// See [`ogc_sys::GX_LoadTexMtxIdx()`] for more.
pub fn load_tex_mtx_idx(mtxidx: u16, texidx: u32, r#type: MtxType) {
    debug_assert!([30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60].contains(&texidx));
    unsafe { ffi::GX_LoadTexMtxIdx(mtxidx, texidx, r#type as _) }
}

/// Enables or disables dithering.
///
/// See [`ogc_sys::GX_SetDither()`] for more.
pub fn set_dither(dither: bool) {
    unsafe { ffi::GX_SetDither(dither as u8) }
}

/// Sends a DrawDone command to the GP and stalls until its subsequent execution.
///
/// See [`ogc_sys::GX_DrawDone()`] for more.
pub fn draw_done() {
    /*
    //This should work :shrug:
    BPReg::PE_DONE.load(2);

    //THIS DIRECTLY OVERRUNS WPAR for a flush.
    for _ in 0..8 {
        for bytes in 0u32.to_be_bytes() {
            GX_PIPE.write(bytes);
        }
    }
    */
    unsafe { ffi::GX_DrawDone(); }
}

/// Sets the Z-buffer compare mode.
/// 
/// See [`ogc_sys::GX_SetZMode()`] for more.
pub fn set_z_mode(enable: bool, func: CmpFn, update_enable: bool) {
    unsafe { ffi::GX_SetZMode(enable as u8, func as u8, update_enable as u8) }
}

/// Determines how the source image, generated by the graphics processor, is blended with
/// the Embedded Frame Buffer (EFB).
/// 
/// See [`ogc_sys::GX_SetBlendMode()`] for more.
pub fn set_blend_mode(
    b_type: BlendMode,
    src_fact: BlendCtrl,
    dst_fact: BlendCtrl,
    op: LogicOp,
) {
    unsafe { ffi::GX_SetBlendMode(b_type as u8, src_fact as u8, dst_fact as u8, op as u8) }
}

/// Enables or disables alpha-buffer updates of the Embedded Frame Buffer (EFB).
/// 
/// See [`ogc_sys::GX_SetAlphaUpdate()`] for more.
pub fn set_alpha_update(enable: bool) {
    unsafe { ffi::GX_SetAlphaUpdate(enable as u8) }
}

/// Enables or disables color-buffer updates when rendering into the Embedded Frame Buffer
/// (EFB).
/// 
/// See [`ogc_sys::GX_SetColorUpdate()`] for more.
pub fn set_color_update(enable: bool) {
    unsafe { ffi::GX_SetColorUpdate(enable as u8) }
}

/// Sets the array base pointer and stride for a single attribute.
/// 
/// See [`ogc_sys::GX_SetArray()`] for more.
pub fn set_array<T>(attr: VtxAttr, array: &[T]) {
    unsafe {
        cache::data_cache_flush(array);
        ffi::GX_SetArray(
            attr as _,
            array.as_ptr() as *mut c_void,
            core::mem::size_of::<T>().try_into().unwrap()
        )
    }
}

/// Begins drawing of a graphics primitive.
/// 
/// See [`ogc_sys::GX_Begin()`] for more.
pub fn begin(primitive: Primitive, vtxfmt: u8, vtxcnt: u16) {
    unsafe { ffi::GX_Begin(primitive as u8, vtxfmt, vtxcnt) }
}

/// Sets the parameters for the alpha compare function which uses the alpha output from
/// the last active TEV stage.
/// 
/// See [`ogc_sys::GX_SetAlphaCompare()`] for more.
pub fn set_alpha_compare(comp0: CmpFn, ref0: u8, aop: AlphaOp, comp1: CmpFn, ref1: u8) {
    unsafe { ffi::GX_SetAlphaCompare(comp0 as u8, ref0, aop as u8, comp1 as u8, ref1) }
}

/// Sets the parameters for the alpha compare function which uses the alpha output from
/// the last active TEV stage.
/// 
/// It's recommended to use [`enable_clip()`] and [`disable_clip()`] instead to be more
/// clear.
/// 
/// See [`ogc_sys::GX_SetClipMode()`] for more.
pub fn set_clip_mode(mode: u8) {
    unsafe { ffi::GX_SetClipMode(mode) }
}

/// Enables clip mode. See [`set_clip_mode()`].
pub fn enable_clip() {
    set_clip_mode(ffi::GX_CLIP_ENABLE as u8);
}

/// Disable clip mode. See [`set_clip_mode()`].
pub fn disable_clip() {
    set_clip_mode(ffi::GX_CLIP_DISABLE as u8);
}

/// Writes a 3D position encoded as three `f32`s.
#[inline]
pub fn pos3_f32(x: f32, y: f32, z: f32) {
    unsafe { ffi::GX_Position3f32(x, y, z);}
}

/// Writes a 3D position encoded as three `u16`s.
#[inline]
pub fn pos3_u16(x: u16, y: u16, z: u16) {
    unsafe { ffi::GX_Position3u16(x, y, z) }
}

/// Writes a 3D position encoded as three `i16`s.
#[inline]
pub fn pos3_i16(x: i16, y: i16, z: i16) {
    unsafe { ffi::GX_Position3s16(x, y, z) }
}

/// Writes a 3D position encoded as three `u8`s.
#[inline]
pub fn pos3_u8(x: u8, y: u8, z: u8) {
    unsafe { ffi::GX_Position3u8(x, y, z) }
}

/// Writes a 3D position encoded as three `i8`s.
#[inline]
pub fn pos3_i8(x: i8, y: i8, z: i8) {
    unsafe { ffi::GX_Position3s8(x, y, z) }
}

/// Writes a 2D position encoded as two `f32`s.
#[inline]
pub fn pos2_f32(x: f32, y: f32) {
    unsafe { ffi::GX_Position2f32(x, y);}
}

/// Writes a 2D position encoded as two `u16`s.
#[inline]
pub fn pos2_u16(x: u16, y: u16) {
    unsafe { ffi::GX_Position2u16(x, y) }
}

/// Writes a 2D position encoded as two `i16`s.
#[inline]
pub fn pos2_i16(x: i16, y: i16) {
    unsafe { ffi::GX_Position2s16(x, y) }
}

/// Writes a 2D position encoded as two `u8`s.
#[inline]
pub fn pos2_u8(x: u8, y: u8) {
    unsafe { ffi::GX_Position2u8(x, y) }
}

/// Writes a 2D position encoded as two `i8`s.
#[inline]
pub fn pos2_i8(x: i8, y: i8) {
    unsafe { ffi::GX_Position2s8(x, y) }
}

/// Writes a `u8` index to a position.
#[inline]
pub fn pos1_x8(index: u8) {
    unsafe { ffi::GX_Position1x8(index) }
}

/// Writes a `u16` index to a position.
#[inline]
pub fn pos1_x16(index: u16) {
    unsafe { ffi::GX_Position1x16(index) }
}

/// Writes a normal encoded as three `f32`s.
#[inline]
pub fn normal3_f32(x: f32, y: f32, z: f32) {
    unsafe { ffi::GX_Normal3f32(x, y, z) }
}

/// Writes a normal encoded as three `i16`s.
#[inline]
pub fn normal3_i16(x: i16, y: i16, z: i16) {
    unsafe { ffi::GX_Normal3s16(x, y, z) }
}

/// Writes a normal encoded as three `i8`s.
#[inline]
pub fn normal3_i8(x: i8, y: i8, z: i8) {
    unsafe { ffi::GX_Normal3s8(x, y, z) }
}

/// Writes a `u8` index to a normal.
#[inline]
pub fn normal1_x8(index: u8) {
    unsafe { ffi::GX_Normal1x8(index) }
}

/// Writes a `u16` index to a normal.
#[inline]
pub fn normal1_x16(index: u16) {
    unsafe { ffi::GX_Normal1x16(index) }
}

/// Writes an RGBA color encoded as four `u8`s.
#[inline]
pub fn color4_u8(r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::GX_Color4u8(r, g, b, a) }
}

/// Writes an RGB color encoded as three `u8`s.
#[inline]
pub fn color3_u8(r: u8, g: u8, b: u8) {
    unsafe { ffi::GX_Color3u8(r, g, b) }
}

/// Writes an RGB color encoded as three `f32`s between 0 and 1.
#[inline]
pub fn color3_f32(r: f32, g: f32, b: f32) {
    unsafe { ffi::GX_Color3f32(r, g, b) }
}

/// Writes a color encoded as a `u32`.
#[inline]
pub fn color1_u32(clr: u32) {
    unsafe { ffi::GX_Color1u32(clr) }
}

/// Writes a color encoded as a `u16`.
#[inline]
pub fn color1_u16(clr: u16) {
    unsafe { ffi::GX_Color1u16(clr) }
}

/// Writes a `u8` index to a color.
#[inline]
pub fn color1_x8(index: u8) {
    unsafe { ffi::GX_Color1x8(index) }
}

/// Writes a `u16` index to a color.
#[inline]
pub fn color1_x16(index: u16) {
    unsafe { ffi::GX_Color1x16(index) }
}

/// Writes an RGBA color given as a [`Color`] object.
/// 
/// This is a convenience function and not part of libogc itself.
#[inline]
pub fn color1_rgba(clr: Color) {
    color4_u8(clr.0.r, clr.0.g, clr.0.b, clr.0.a);
}

/// Writes an RGB color given as a [`Color`] object.
/// 
/// This is a convenience function and not part of libogc itself.
#[inline]
pub fn color1_rgb(clr: Color) {
    color3_u8(clr.0.r, clr.0.g, clr.0.b);
}

/// Writes a 2D texture coordinate encoded as two `f32`s.
#[inline]
pub fn texcoord2_f32(s: f32, t: f32) {
    unsafe { ffi::GX_TexCoord2f32(s, t) }
}

/// Writes a 2D texture coordinate encoded as two `u16`s.
#[inline]
pub fn texcoord2_u16(s: u16, t: u16) {
    unsafe { ffi::GX_TexCoord2u16(s, t) }
}

/// Writes a 2D texture coordinate encoded as two `i16`s.
#[inline]
pub fn texcoord2_i16(s: i16, t: i16) {
    unsafe { ffi::GX_TexCoord2s16(s, t) }
}

/// Writes a 2D texture coordinate encoded as two `u8`s.
#[inline]
pub fn texcoord2_u8(s: u8, t: u8) {
    unsafe { ffi::GX_TexCoord2u8(s, t) }
}

/// Writes a 2D texture coordinate encoded as two `i8`s.
#[inline]
pub fn texcoord2_i8(s: i8, t: i8) {
    unsafe { ffi::GX_TexCoord2s8(s, t) }
}

/// Writes a 1D texture coordinate encoded as a `f32`.
#[inline]
pub fn texcoord1_f32(s: f32) {
    unsafe { ffi::GX_TexCoord1f32(s) }
}

/// Writes a 1D texture coordinate encoded as a `u16`.
#[inline]
pub fn texcoord1_u16(s: u16) {
    unsafe { ffi::GX_TexCoord1u16(s) }
}

/// Writes a 1D texture coordinate encoded as a `i16`.
#[inline]
pub fn texcoord1_i16(s: i16) {
    unsafe { ffi::GX_TexCoord1s16(s) }
}

/// Writes a 1D texture coordinate encoded as a `u8`.
#[inline]
pub fn texcoord1_u8(s: u8) {
    unsafe { ffi::GX_TexCoord1u8(s) }
}

/// Writes a 1D texture coordinate encoded as a `i8`.
#[inline]
pub fn texcoord1_i8(s: i8) {
    unsafe { ffi::GX_TexCoord1s8(s) }
}

/// Writes a `u8` index to a texture coordinate.
#[inline]
pub fn texcoord1_x8(index: u8) {
    unsafe { ffi::GX_TexCoord1x8(index) }
}

/// Writes a `u16` index to a texture coordinate.
#[inline]
pub fn texcoord1_x16(index: u16) {
    unsafe { ffi::GX_TexCoord1x16(index) }
}

/// Writes a `u8` index to a matrix.
#[inline]
pub fn matrix1_x8(index: u8) {
    unsafe { ffi::GX_MatrixIndex1x8(index) }
}

/// Flushes all commands to the GP.
/// 
/// See [`ogc_sys::GX_Flush()`] for more.
pub fn flush() {
    unsafe { ffi::GX_Flush() }
}

#[inline]
pub fn end() {
    unsafe { ffi::GX_End() }
}

/// Loads a given texture from DRAM into the texture memory.
///
/// See [`ogc_sys::GX_PreloadEntireTexture()`] for more.
pub fn preload_entire_texture(obj: &RawTexture, region: &mut RawTexRegion) {
    unsafe { ffi::GX_PreloadEntireTexture(&obj.inner as *const _ as *mut _, &mut region.0) }
}

/// Copies a Texture Look-Up Table (TLUT) from main memory to Texture Memory (TMEM).
///
/// See [`ogc_sys::GX_LoadTlut()`] for more.
pub fn load_tlut(obj: &Tlut, tlut_name: u32) {
    unsafe { ffi::GX_LoadTlut(obj as *const _ as *mut _, tlut_name) }
}

/// This function sends a token into the command stream.
///
/// See [`ogc_sys::GX_SetDrawSync()`] for more.
pub fn set_draw_sync(token: u16) {
    unsafe { ffi::GX_SetDrawSync(token); }
}

/// Returns the value of the token register, which is written using the
/// [`set_draw_sync()`] function.
pub fn get_draw_sync() -> u16 {
    unsafe { ffi::GX_GetDrawSync() }
}

/// Sets two performance metrics to measure in the GP.
///
/// When in display list mode, this is a no-op.
///
/// See [`ogc_sys::GX_SetGPMetric()`] for more.
pub fn set_gp_metric(perf0: Perf0, perf1: Perf1) {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        unsafe { ffi::GX_SetGPMetric(perf0 as _, perf1 as _) }
    }
}

/// Returns the count of the previously set performance metrics.
///
/// When in display list mode, this returns `None`.
///
/// See [`ogc_sys::GX_ReadGPMetric()`] for more.
pub fn read_gp_metric() -> Option<(u32, u32)> {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        let mut counts = (0, 0);
        unsafe { ffi::GX_ReadGPMetric(&mut counts.0, &mut counts.1) }
        Some(counts)
    } else {
        None
    }
}

/// Sets the metric the Vertex Cache performance counter will measure.
///
/// See [`ogc_sys::GX_SetVCacheMetric()`] for more.
pub fn set_vcache_metric(attr: VCacheAttr) {
    unsafe { ffi::GX_SetVCacheMetric(attr as _) }
}

/// Returns vertex cache performance counters.
///
/// When in display list mode, this returns `None`.
///
/// See [`ogc_sys::GX_ReadVCacheMetric()`] for more.
pub fn read_vcache_metric() -> Option<(u32, u32, u32)> {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        let (mut check, mut miss, mut stall) = (0, 0, 0);
        unsafe { ffi::GX_ReadVCacheMetric(&mut check, &mut miss, &mut stall); }
        Some((check, miss, stall))
    } else {
        None
    }
}

/// Copies the embedded framebuffer (EFB) to the texture image buffer `dest` in main
/// memory.
/// 
/// Since `dest` should be 32-byte aligned, this function takes a `Buf32` to meet that
/// requirement. Make sure it's big enough to hold the EFB!
///
/// See [`ogc_sys::GX_CopyTex()`] for more.
pub fn copy_tex(mut dest: Buf32, clear: bool) {
    unsafe { ffi::GX_CopyTex(dest.as_mut_ptr() as *mut _, clear as _) }
}

/// Sets the width of line primitives.
///
/// See [`ogc_sys::GX_SetLineWidth()`] for more.
pub fn set_line_width(width: u8, fmt: TexOffset) {
    unsafe { ffi::GX_SetLineWidth(width, fmt as _); }
}

/// Sets the size of point primitives.
///
/// See [`ogc_sys::GX_SetPointSize()`] for more.
pub fn set_point_size(width: u8, fmt: TexOffset) {
    unsafe { ffi::GX_SetPointSize(width, fmt as _); }
}

/// Sets the fog color.
///
/// See [`ogc_sys::GX_SetFogColor()`] for more.
pub fn set_fog_color(color: Color) {
    unsafe { ffi::GX_SetFogColor(color.0); }
}

/// Enables fog.
///
/// See [`ogc_sys::GX_SetFog()`] for more.
pub fn set_fog(ft: FogType, startz: f32, endz: f32, nearz: f32, farz: f32, col: Color) {
    unsafe {
        ffi::GX_SetFog(ft as _, startz, endz, nearz, farz, col.0);
    }
}

/// Enables or disables horizontal fog-range adjustment.
/// 
/// See [`ogc_sys::GX_SetFogRangeAdj()`] for more.
pub fn set_fog_range_adj(enable: bool, center: u16, mut table: FogAdjTable) {
    unsafe { ffi::GX_SetFogRangeAdj(enable as u8, center, &mut table.0) }
}

/// Begins a display list and disables writes to the FIFO currently attached to the CPU.
///
/// When already in a display list, this is a no-op.
///
/// See [`ogc_sys::GX_BeginDispList()`] for more.
pub fn begin_display_list(list: &mut Buf32) {
    if ! IN_DISPLAY_LIST.swap(true, Ordering::AcqRel) {
        // TODO: we should not be allowing write access from the application to the Buf32
        // when libogc will also be writing to it.
        unsafe { ffi::GX_BeginDispList(list.as_mut_ptr() as *mut _, list.len() as u32) }
    }
}

/// Ends a display list and resumes writing graphics commands to the CPU FIFO.
///
/// When not in a display list, this returns `None`. Otherwise, it returns the final size
/// of the display list, which is passed to libogc when calling the display list with
/// [`call_display_list()`].
///
/// See [`ogc_sys::GX_EndDispList()`] for more.
pub fn end_display_list() -> Option<u32> {
    if IN_DISPLAY_LIST.swap(false, Ordering::AcqRel) {
        Some(unsafe { ffi::GX_EndDispList() })
    } else {
        None
    }
}

/// Causes the GP to execute graphics commands from the display list instead of from the GP FIFO.
///
/// When already in a display list, this is a no-op.
///
/// See [`ogc_sys::GX_CallDispList()`] for more.
pub fn call_display_list(list: &Buf32, size: u32) {
    if ! IN_DISPLAY_LIST.load(Ordering::Acquire) {
        unsafe { ffi::GX_CallDispList(list.as_ptr() as *const _ as *mut _, size) }
    }
}

/// Enables a special texture offset feature for points and lines.
///
/// For `coord`, libogc will use the lowest 3 bits for the texture slot, so its value
/// should be between 0 and 7 inclusive.
///
/// See [`ogc_sys::GX_EnableTexOffsets()`] for more.
pub fn enable_tex_offsets(coord: u8, line_enable: bool, point_enable: bool) {
    unsafe { ffi::GX_EnableTexOffsets(coord, line_enable as u8, point_enable as u8) }
}

pub fn get_overflow_count() -> u32 {
    unsafe { ffi::GX_GetOverflowCount() }
}

/// Loads the state describing a preloaded texture into one of eight hardware register sets.
///
/// See [`ogc_sys::GX_LoadTexObjPreloaded()`] for more.
pub fn load_texture_preloaded(obj: &mut RawTexture, region: &mut RawTexRegion, mapid: u8) {
    unsafe { ffi::GX_LoadTexObjPreloaded(&mut obj.inner, &mut region.0, mapid) }
}

/// Invalidates the vertex cache.
///
/// See [`ogc_sys::GX_InvVtxCache()`] for more.
pub fn invalidate_vtx_cache() {
    unsafe { ffi::GX_InvVtxCache() }
}

/// Allows the CPU to read a color value directly from the Embedded Frame Buffer (EFB) at position (x, y).
///
/// See [`ogc_sys::GX_PeekARGB()`] for more.
pub fn peek_argb(x: u16, y: u16) -> Color {
    let mut color = MaybeUninit::uninit();
    unsafe {
        ffi::GX_PeekARGB(x, y, color.as_mut_ptr());
        Color(color.assume_init())
    }
}

/// Allows the CPU to read a z value directly from the Embedded Frame Buffer (EFB) at position (x, y).
///
/// See [`ogc_sys::GX_PeekZ()`] for more.
pub fn peek_z(x: u16, y: u16) -> u32 {
    let mut z = MaybeUninit::uninit();
    unsafe {
        ffi::GX_PeekZ(x, y, z.as_mut_ptr());
        z.assume_init()
    }
}

/// Sets a threshold which is compared to the alpha of pixels written to the Embedded Frame
/// Buffer (EFB) using the `gx::poke_*()` functions.
///
/// See [`ogc_sys::GX_PokeAlphaMode()`] for more.
pub fn poke_alpha_mode(func: CmpFn, threshold: u8) {
    unsafe { ffi::GX_PokeAlphaMode(func as u8, threshold) }
}

/// Determines value of alpha read from a frame buffer with no alpha channel
#[repr(u32)]
pub enum AlphaReadMode {
    /// Always read 0x00.
    Zero = ffi::GX_READ_00,
    /// Always read 0xFF.
    Max = ffi::GX_READ_FF,
    /// Always read the real alpha value.
    None = ffi::GX_READ_NONE,
}

/// Determines what value of alpha will be read from the Embedded Frame Buffer (EFB).
///
/// See [`ogc_sys::GX_PokeAlphaRead()`] for more.
pub fn poke_alpha_read(mode: AlphaReadMode) {
    unsafe { ffi::GX_PokeAlphaRead(mode as u8) }
}

/// Enables or disables alpha-buffer updates for `gx::poke_*()` functions.
///
/// See [`ogc_sys::GX_PokeAlphaUpdate()`] for more.
pub fn poke_alpha_update(enable: bool) {
    unsafe { ffi::GX_PokeAlphaUpdate(enable as u8) }
}

/// Allows the CPU to write color directly to the Embedded Frame Buffer (EFB) at position (x, y).
///
/// See [`ogc_sys::GX_PokeARGB()`] for more.
pub fn poke_argb(x: u16, y: u16, color: Color) {
    debug_assert!(x <= 640, "x must be less than 640, currently {x}");
    debug_assert!(y <= 528, "y must be less than 528, currently {y}");
    unsafe { ffi::GX_PokeARGB(x, y, color.0) }
}

/// Determines how the source image, is blended with the current Embedded Frame Buffer (EFB).
///
/// See [`ogc_sys::GX_PokeBlendMode()`] for more.
pub fn poke_blend_mode(mode: BlendMode, src_fact: BlendCtrl, dst_fact: BlendCtrl, op: LogicOp) {
    unsafe { ffi::GX_PokeBlendMode(mode as u8, src_fact as u8, dst_fact as u8, op as u8) }
}

/// Enables or disables color-buffer updates when writing the Embedded Frame Buffer (EFB) using the
/// `gx::poke_*()` functions.
///
/// See [`ogc_sys::GX_PokeColorUpdate()`] for more.
pub fn poke_color_update(enable: bool) {
    unsafe { ffi::GX_PokeColorUpdate(enable as u8) }
}

/// Enables dithering when writing the Embedded Frame Buffer (EFB) using `gx::poke_*()` functions.
///
/// See [`ogc_sys::GX_PokeDither()`] for more.
pub fn poke_dither(enable: bool) {
    unsafe { ffi::GX_PokeDither(enable as u8) }
}

/// Sets a constant alpha value for writing to the Embedded Frame Buffer (EFB).
///
/// See [`ogc_sys::GX_PokeDstAlpha()`] for more.
pub fn poke_dst_alpha(enable: bool, a: u8) {
    unsafe { ffi::GX_PokeDstAlpha(enable as u8, a) }
}

/// Allows the CPU to write a z value directly to the Embedded Frame Buffer (EFB) at position (x, y).
///
/// See [`ogc_sys::GX_PokeZ()`] for more.
pub fn poke_z(x: u16, y: u16, z: u32) {
    debug_assert!(x <= 640, "x must be less than 640, currently {x}");
    debug_assert!(y <= 528, "y must be less than 528, currently {y}");
    unsafe { ffi::GX_PokeZ(x, y, z) }
}

/// Sets the Z-buffer compare mode when writing the Embedded Frame Buffer (EFB).
///
/// See [`ogc_sys::GX_PokeZMode()`] for more.
pub fn poke_z_mode(enable_cmp: bool, cmp: CmpFn, enable_update: bool) {
    unsafe { ffi::GX_PokeZMode(enable_cmp as u8, cmp as u8, enable_update as u8) }
}

/// Returns the bounding box of pixel coordinates that are drawn in the Embedded Framebuffer (EFB).
///
/// Returns a tuple `(top, bottom, left, right)` corresponding to each line.
///
/// See [`ogc_sys::GX_ReadBoundingBox()`] for more.
pub fn read_bounding_box() -> (u16, u16, u16, u16) {
    let (mut top, mut bottom, mut left, mut right) = (0, 0, 0, 0);
    unsafe {
        ffi::GX_ReadBoundingBox(&mut top, &mut bottom, &mut left, &mut right);
    }
    (top, bottom, left, right)
}

pub fn read_clks_per_vtx() -> u32 {
    unsafe { ffi::GX_ReadClksPerVtx() }
}

/// Read performance metric values from the XF and RAS units.
///
/// See [`ogc_sys::GX_ReadXfRasMetric()`] for more.
pub fn read_xf_ras_metric() -> (u32, u32, u32, u32) {
    let (mut xfwaitin, mut xfwaitout, mut rasbusy, mut clks) = (0, 0, 0, 0);
    unsafe { ffi::GX_ReadXfRasMetric(&mut xfwaitin, &mut xfwaitout, &mut rasbusy, &mut clks); }
    (xfwaitin, xfwaitout, rasbusy, clks)
}

pub fn reset_overflow_count() -> u32 {
    unsafe { ffi::GX_ResetOverflowCount() }
}

/// Sets the ambient color register for the color channel.
///
/// See [`ogc_sys::GX_SetChanAmbColor()`] for more.
pub fn set_chan_amb_color(channel: ColorChannel, color: Color) {
    unsafe { ffi::GX_SetChanAmbColor(channel as i32, color.0) }
}

/// Sets the material color register for the color channel.
///
/// See [`ogc_sys::GX_SetChanMatColor()`] for more.
pub fn set_chan_mat_color(channel: ColorChannel, color: Color) {
    unsafe { ffi::GX_SetChanMatColor(channel as i32, color.0) }
}

/// Enables or disables coplanar triangle processing.
///
/// See [`ogc_sys::GX_SetCoPlanar()`] for more.
pub fn set_coplanar(enable: bool) {
    unsafe { ffi::GX_SetCoPlanar(enable as u8) }
}

bitflags::bitflags! {
    /// XFB clamp mode flags. Use `ClampMode::empty()` for no clamping.
    pub struct ClampMode: u32 {
        //const NONE = ffi::GX_CLAMP_NONE; // use ClampMode::empty() instead
        const TOP = ffi::GX_CLAMP_TOP;
        const BOTTOM = ffi::GX_CLAMP_BOTTOM;
    }
}

/// Sets the vertical clamping mode to use during the EFB to XFB or texture copy.
///
/// See [`ogc_sys::GX_SetCopyClamp()`] for more.
pub fn set_copy_clamp(clamp: ClampMode) {
    unsafe { ffi::GX_SetCopyClamp(clamp.bits() as u8) }
}

/// Selects a specific matrix to use for transformations.
///
/// See [`ogc_sys::GX_SetCurrentMtx()`] for more.
pub fn set_current_mtx(mtx: u32) {
    unsafe { ffi::GX_SetCurrentMtx((mtx * 3).min(ffi::GX_PNMTX9)) }
}

/// Controls whether all lines, only even lines, or only odd lines are copied from the EFB.
#[repr(u32)]
pub enum CopyMode {
    Progressive = ffi::GX_COPY_PROGRESSIVE,
    InterlacedEven = ffi::GX_COPY_INTLC_EVEN,
    InterlacedOdd = ffi::GX_COPY_INTLC_ODD,
}

/// Determines which lines are read from the Embedded Frame Buffer (EFB) when using [`gx::copy_disp()`].
///
/// See [`ogc_sys::GX_SetDispCopyFrame2Field()`] for more.
pub fn set_disp_copy_frame_to_field(mode: CopyMode) {
    unsafe { ffi::GX_SetDispCopyFrame2Field(mode as u8) }
}

/// Sets a constant alpha value for writing to the Embedded Frame Buffer (EFB).
///
/// See [`ogc_sys::GX_SetDstAlpha()`] for more.
pub fn set_dst_alpha(enable: bool, a: u8) {
    unsafe { ffi::GX_SetDstAlpha(enable as u8, a) }
}

/// Selectively enables and disables interlacing of the frame buffer image.
///
/// See [`ogc_sys::GX_SetFieldMask()`] for more.
pub fn set_field_mask(even_mask: bool, odd_mask: bool) {
    unsafe { ffi::GX_SetFieldMask(even_mask as u8, odd_mask as u8) }
}

#[repr(u32)]
#[non_exhaustive]
pub enum MiscToken {
    /// External framebuffer flush?
    XfFlush = ffi::GX_MT_XF_FLUSH,
    /// Display list save context?
    DlSaveCtx = ffi::GX_MT_DL_SAVE_CTX,
}

/// Sets miscellanous settings in the GP.
///
/// See [`ogc_sys::GX_SetMisc()`] for more.
pub fn set_misc(token: MiscToken, value: u32) {
    unsafe { ffi::GX_SetMisc(token as u32, value) }
}

/// Used to set how many indirect lookups will take place.
///
/// See [`ogc_sys::GX_SetNumIndStages()`] for more.
pub fn set_num_ind_stages(nstages: u8) {
    debug_assert!(nstages <= 4, "There are only 4 indirect stages; got {nstages}");
    unsafe { ffi::GX_SetNumIndStages(nstages) }
}

/// Enables a consecutive number of TEV stages.
///
/// See [`ogc_sys::GX_SetNumTevStages()`] for more.
pub fn set_num_tev_stages(num: u8) {
    debug_assert!(num <= 16, "There are only 16 TEV stages; got {num}");
    unsafe { ffi::GX_SetNumTevStages(num) }
}

/// Repositions the scissorbox rectangle within the Embedded Frame Buffer (EFB) memory space.
///
/// See [`ogc_sys::GX_SetScissorBoxOffset()`] for more.
pub fn set_scissor_box_offset(xoffset: i32, yoffset: i32) {
    debug_assert!(
        (-342..=382).contains(&xoffset),
        "x offset should be between -342 and 382 inclusive; got {xoffset}"
    );
    debug_assert!(
        (-342..=494).contains(&yoffset),
        "y offset should be between -342 and 382 inclusive; got {yoffset}"
    );
    unsafe { ffi::GX_SetScissorBoxOffset(xoffset, yoffset) }
}

/// Sets the viewport and adjusts the viewport's line offset for interlaced field rendering.
///
/// See [`ogc_sys::GX_SetViewportJitter()`] for more.
pub fn set_viewport_jitter(x_orig: f32, y_orig: f32, wd: f32, ht: f32, near_z: f32, far_z: f32, odd: bool) {
    unsafe { ffi::GX_SetViewportJitter(x_orig, y_orig, wd, ht, near_z, far_z, odd as u32) }
}

/// Z texture operation
#[repr(u32)]
pub enum ZTexOp {
    Disable = ffi::GX_ZT_DISABLE,
    /// Add a Z texel to reference Z
    Add = ffi::GX_ZT_ADD,
    /// Replace reference Z with Z texel
    Replace = ffi::GX_ZT_REPLACE,
}

/// Controls Z texture operations.
///
/// `fmt` should be one of the Z texture variants.
///
/// See [`ogc_sys::GX_SetZTexture()`] for more.
pub fn set_z_texture(op: ZTexOp, fmt: TexFormat, bias: u32) {
    unsafe { ffi::GX_SetZTexture(op as u8, fmt as u8, bias) }
}

//All the following data is found from
// http://hitmen.c02.at/files/yagcd/yagcd/chap5.html#sec5.3

/*
/// `display_list` slice should be 32-byte aligned and padded to the next
/// 32-byte boundary. May want to use [`Buf32`] for this.
fn call_display_list(display_list: &[u8]) {
    let ptr = display_list.as_ptr().map_addr(mem::to_physical);

    GX_PIPE.write(GPCommand::CallDisplayList as u8);

    for byte in ptr.addr().to_be_bytes() {
        GX_PIPE.write(byte);
    }

    for byte in (display_list.len() as u32).to_be_bytes() {
        GX_PIPE.write(byte);
    }
}

//Currently doesnt check dirty state
fn draw_begin(command: GPDrawCommand, vertex_format: u8, vertex_count: u16) {
    assert!(vertex_format <= 7, "Incorrect vertex format");
    let gp_cmd = (command as u8) | (vertex_format & 7);

    GX_PIPE.write(gp_cmd);
    for byte in vertex_count.to_be_bytes() {
        GX_PIPE.write(byte);
    }
}
*/

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum GPCommand {
    Nop = 0x00,
    LoadCPReg = 0x08,
    LoadXFReg = 0x10,
    LoadPosIndexed = 0x20,
    LoadNormalIndexed = 0x28,
    LoadTexureIndexed = 0x30,
    LoadLightIndexed = 0x38,
    CallDisplayList = 0x40,
    InvalidateVertexCache = 0x48,
    LoadBPReg = 0x61,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum GPDrawCommand {
    DrawQuads = 0x80,
    DrawTriangles = 0x90,
    DrawTriangleStrip = 0x98,
    DrawTriangleFan = 0xA0,
    DrawLines = 0xA8,
    DrawLineStrip = 0xB0,
    DrawPoints = 0xBB,
}

/// The stub used by ogc-rs to invoke Rust functions for the breakpoint callback.
unsafe extern "C" fn breakpt_callback_stub() {
    let cb_ptr = BREAKPT_CB.load(Ordering::Acquire);
    if ! cb_ptr.is_null() {
        unsafe { (*cb_ptr)(); }
    }
}

/// Registers `cb` as a function to be invoked when a break point is encountered.
///
/// See [`ogc_sys::GX_SetBreakPtCallback()`] for more.
///
/// # Safety
/// The callback is marked unsafe due to running when interrupts are disabled. It should
/// return as soon as possible.
pub fn set_breakpt_callback(cb: Option<unsafe fn()>) -> Option<unsafe fn()> {
    let cb_ptr = match cb {
        None => {
            unsafe { ffi::GX_SetBreakPtCallback(None); }
            ptr::null_mut()
        }
        Some(mut f) => {
            unsafe { ffi::GX_SetBreakPtCallback(Some(breakpt_callback_stub)); }
            (&mut f) as *mut unsafe fn()
        }
    };

    let prev_cb_ptr = BREAKPT_CB.swap(cb_ptr, Ordering::AcqRel);

    if prev_cb_ptr.is_null() {
        None
    } else {
        // SAFETY:
        // - pointer already checked to be non-null
        // - we always set a valid rust function
        Some(unsafe { *prev_cb_ptr })
    }
}


/// The stub used by ogc-rs to invoke Rust functions for the DrawDone callback.
/// 
/// This stub is automatically set when calling [`gx::init()`].
unsafe extern "C" fn draw_done_callback_stub() {
    let cb_ptr = DRAW_DONE_CB.load(Ordering::Acquire);
    if ! cb_ptr.is_null() {
        // SAFETY:
        // - pointer already checked to be non-null
        // - we always set a valid rust function
        unsafe { (*cb_ptr)(); }
    }
}

/// Sets a Rust callback that gets invoked when the graphics pipeline encounters a
/// DrawDone command.
/// 
/// The DrawDone command is sent by [`gx::set_draw_done()`].
/// 
/// Note that by the time the callback is invoked, the GP will already have resumed
/// reading from the FIFO if there are any commands in it.
pub fn set_draw_done_callback(cb: Option<fn()>) -> Option<fn()> {
    let cb_ptr = match cb {
        None => {
            unsafe { ffi::GX_SetDrawDoneCallback(None); }
            ptr::null_mut()
        }
        Some(mut f) => {
            unsafe { ffi::GX_SetDrawDoneCallback(Some(draw_done_callback_stub)); }
            (&mut f) as *mut fn()
        }
    };

    let prev_cb_ptr = DRAW_DONE_CB.swap(cb_ptr, Ordering::AcqRel);

    if prev_cb_ptr.is_null() {
        None
    } else {
        // SAFETY:
        // - pointer already checked to be non-null
        // - we always set a valid rust function
        Some(unsafe { *prev_cb_ptr })
    }
}


/// The stub used by ogc-rs to invoke Rust functions for the DrawSync callback.
/// 
/// This stub is automatically set when calling [`gx::init()`].
unsafe extern "C" fn draw_sync_callback_stub(token: u16) {
    let cb_ptr = DRAW_SYNC_CB.load(Ordering::Acquire);
    if ! cb_ptr.is_null() {
        // SAFETY:
        // - pointer already checked to be non-null
        // - we always set a valid rust function
        unsafe { (*cb_ptr)(token); }
    }
}

/// Sets a Rust callback that gets invoked when the graphics pipeline encounters a
/// `DrawSync` token.
/// 
/// The argument to the callback is the value of the most recently encountered token. Your
/// code must be able to deduce if any tokens have been missed, since graphics processing
/// doesn't stop while the callback is running.
pub fn set_draw_sync_callback(cb: Option<fn(u16)>) -> Option<fn(u16)> {
    let cb_ptr = match cb {
        None => {
            unsafe { ffi::GX_SetDrawSyncCallback(None); }
            ptr::null_mut()
        }
        Some(mut f) => {
            unsafe { ffi::GX_SetDrawSyncCallback(Some(draw_sync_callback_stub)); }
            (&mut f) as *mut fn(u16)
        }
    };

    let prev_cb_ptr = DRAW_SYNC_CB.swap(cb_ptr, Ordering::AcqRel);

    if prev_cb_ptr.is_null() {
        None
    } else {
        // SAFETY:
        // - pointer already checked to be non-null
        // - we always set a valid rust function
        Some(unsafe { *prev_cb_ptr })
    }
}

/// Specifies the pixel format of the embedded frame buffer.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum PixelFormat {
    Rgb8Z24 = ffi::GX_PF_RGB8_Z24,
    Rgba6Z24 = ffi::GX_PF_RGBA6_Z24,
    Rgb565Z16 = ffi::GX_PF_RGB565_Z16,
    Z24 = ffi::GX_PF_Z24,
    Y8 = ffi::GX_PF_Y8,
    U8 = ffi::GX_PF_U8,
    V8 = ffi::GX_PF_V8,
    Yuv420 = ffi::GX_PF_YUV420,
}

/// Vertex data input type
#[repr(u32)]
pub enum VtxAttrIn {
    None = ffi::GX_NONE,
    Direct = ffi::GX_DIRECT,
    Index8 = ffi::GX_INDEX8,
    Index16 = ffi::GX_INDEX16,
}

/// Gamma correction value to apply.
#[repr(u32)]
pub enum Gamma {
    Mode10 = ffi::GX_GM_1_0,
    Mode17 = ffi::GX_GM_1_7,
    Mode22 = ffi::GX_GM_2_2,
}


/// Allows the sharing of a texcoord between an indirect stage and a regular TEV stage.
/// 
/// Scale arguments take a value between 0 and 8 inclusive, which corresponds to the power
/// of two used for the indirect scaling. For example, to get the same behavior as using
/// `GX_ITS_32`, use a scale value of 5.
/// 
/// See [`ogc_sys::GX_SetIndTexCoordScale()`] for more.
pub fn set_ind_tex_coord_scale(indtexid: u8, scale_s: u8, scale_t: u8) {
    debug_assert!(indtexid <= 3);
    debug_assert!(scale_s <= 8);
    debug_assert!(scale_t <= 8);
    unsafe { ffi::GX_SetIndTexCoordScale(indtexid, scale_s, scale_t) }
}

/// Indirect texture matrix
#[repr(u32)]
pub enum IndTexMat {
    /// Specifies a matrix of all zeroes.
    Off = ffi::GX_ITM_OFF,
    /// Specifies indirect matrix 0, indirect scale 0.
    Ind0 = ffi::GX_ITM_0,
    /// Specifies indirect matrix 1, indirect scale 1.
    Ind1 = ffi::GX_ITM_1,
    /// Specifies indirect matrix 2, indirect scale 2.
    Ind2 = ffi::GX_ITM_2,
    /// Specifies dynamic S-type matrix, indirect scale 0.
    S0 = ffi::GX_ITM_S0,
    /// Specifies dynamic S-type matrix, indirect scale 1.
    S1 = ffi::GX_ITM_S1,
    /// Specifies dynamic S-type matrix, indirect scale 2.
    S2 = ffi::GX_ITM_S2,
    /// Specifies dynamic T-type matrix, indirect scale 0.
    T0 = ffi::GX_ITM_T0,
    /// Specifies dynamic T-type matrix, indirect scale 1.
    T1 = ffi::GX_ITM_T1,
    /// Specifies dynamic T-type matrix, indirect scale 2.
    T2 = ffi::GX_ITM_T2,
}

/// Sets one of the three static indirect matrices and the associated scale factor.
/// 
/// See [`ogc_sys::GX_SetIndTexMatrix()`] for more.
pub fn set_ind_tex_matrix(indtexmtx: IndTexMat, offset_mtx: [[f32; 3]; 2], scale_exp: i8) {
    unsafe {
        ffi::GX_SetIndTexMatrix(indtexmtx as u8, offset_mtx.as_ptr().cast_mut(), scale_exp)
    }
}

/// Sets one of the three static indirect matrices and the associated scale factor.
/// 
/// See [`ogc_sys::GX_SetIndTexMatrix()`] for more.
pub fn set_ind_tex_order(indtexstage: u8, texcoord: Option<u8>, texmap: Option<u8>) {
    debug_assert!(indtexstage <= 3);
    debug_assert!(if let Some(x) = texcoord { x <= 7 } else { true });
    debug_assert!(if let Some(x) = texmap { x <= 7 } else { true });
    unsafe {
        ffi::GX_SetIndTexOrder(
            indtexstage,
            texcoord.unwrap_or(ffi::GX_TEXCOORDNULL as u8),
            texmap.unwrap_or(ffi::GX_TEXMAP_NULL as u8)
        )
    }
}

/// TEV alpha combiner input
#[repr(u32)]
pub enum TevAlphaArg {
    /// Use the alpha value from previous TEV stage
    APrev = ffi::GX_CA_APREV,
    /// Use the alpha value from the color/output register 0
    A0 = ffi::GX_CA_A0,
    /// Use the alpha value from the color/output register 1
    A1 = ffi::GX_CA_A1,
    /// Use the alpha value from the color/output register 2
    A2 = ffi::GX_CA_A2,
    /// Use the alpha value from texture
    TexA = ffi::GX_CA_TEXA,
    /// Use the alpha value from rasterizer
    RasA = ffi::GX_CA_RASA,
    Konst = ffi::GX_CA_KONST,
    /// Use to pass zero value
    Zero = ffi::GX_CA_ZERO,
}

/// Sets the alpha input sources for one tevstage of the Texture Environment (TEV) alpha
/// combiner.
/// 
/// See [`ogc_sys::GX_SetTevAlphaIn()`] for more.
pub fn set_tev_alpha_in(
    tevstage: u8,
    a: TevAlphaArg,
    b: TevAlphaArg,
    c: TevAlphaArg,
    d: TevAlphaArg
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevAlphaIn(tevstage, a as u8, b as u8, c as u8, d as u8); }
}

/// TEV combiner operator
#[repr(u32)]
pub enum TevOp {
    Add = ffi::GX_TEV_ADD,
    Sub = ffi::GX_TEV_SUB,
    CompR8Gt = ffi::GX_TEV_COMP_R8_GT,
    CompR8Eq = ffi::GX_TEV_COMP_R8_EQ,
    CompGR16Gt = ffi::GX_TEV_COMP_GR16_GT,
    CompGR16Eq = ffi::GX_TEV_COMP_GR16_EQ,
    CompBGR24Gt = ffi::GX_TEV_COMP_BGR24_GT,
    CompBGR24Eq = ffi::GX_TEV_COMP_BGR24_EQ,
    /// Can also be used to compare alpha values
    CompRGB8Gt = ffi::GX_TEV_COMP_RGB8_GT,
    /// Can also be used to compare alpha values
    CompRGB8Eq = ffi::GX_TEV_COMP_RGB8_EQ,
}

/// TEV bias value
#[repr(u32)]
pub enum TevBias {
    Zero = ffi::GX_TB_ZERO,
    AddHalf = ffi::GX_TB_ADDHALF,
    SubHalf = ffi::GX_TB_SUBHALF,
}

/// TEV scale value
#[repr(u32)]
pub enum TevScale {
    Scale1 = ffi::GX_CS_SCALE_1,
    Scale2 = ffi::GX_CS_SCALE_2,
    Scale4 = ffi::GX_CS_SCALE_4,
    Divide2 = ffi::GX_CS_DIVIDE_2,
}

/// TEV color/output register
#[repr(u32)]
pub enum TevColOutReg {
    /// Default register for passing results from one stage to another.
    Prev = ffi::GX_TEVPREV,
    Reg0 = ffi::GX_TEVREG0,
    Reg1 = ffi::GX_TEVREG1,
    Reg2 = ffi::GX_TEVREG2,
}

/// Sets the tevop, tevbias, tevscale and clamp-mode operation for the alpha combiner for
/// this tevstage of the TEV unit.
/// 
/// See [`ogc_sys::GX_SetTevAlphaOp()`] for more.
pub fn set_tev_alpha_op(
    tevstage: u8,
    tevop: TevOp,
    tevbias: TevBias,
    tevscale: TevScale,
    clamp: bool,
    tevregid: TevColOutReg
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe {
        ffi::GX_SetTevAlphaOp(
            tevstage,
            tevop as u8,
            tevbias as u8,
            tevscale as u8,
            clamp as u8,
            tevregid as u8
        )
    }
}

/// Used to set one of the primary color registers in the TEV unit.
/// 
/// See [`ogc_sys::GX_SetTevColor()`] for more.
pub fn set_tev_color(tev_regid: TevColOutReg, color: Color) {
    debug_assert!(
        matches!(tev_regid, TevColOutReg::Reg0 | TevColOutReg::Reg1 | TevColOutReg::Reg2)
    );
    unsafe { ffi::GX_SetTevColor(tev_regid as u8, color.0); }
}

/// TEV color combiner input
#[repr(u32)]
pub enum TevColorArg {
    /// Use the color value from previous TEV stage
    PrevColor = ffi::GX_CC_CPREV,
    /// Use the alpha value from previous TEV stage
    PrevAlpha = ffi::GX_CC_APREV,
    /// Use the color value from the color/output register 0
    Reg0Color = ffi::GX_CC_C0,
    /// Use the alpha value from the color/output register 0 
    Reg0Alpha = ffi::GX_CC_A0,
    /// Use the color value from the color/output register 1
    Reg1Color = ffi::GX_CC_C1,
    /// Use the alpha value from the color/output register 1
    Reg1Alpha = ffi::GX_CC_A1,
    /// Use the color value from the color/output register 2
    Reg2Color = ffi::GX_CC_C2,
    /// Use the alpha value from the color/output register 2
    Reg2Alpha = ffi::GX_CC_A2,
    /// Use the color value from texture
    TexColor = ffi::GX_CC_TEXC,
    /// Use the alpha value from texture
    TexAlpha = ffi::GX_CC_TEXA,
    /// Use the color value from rasterizer
    RasColor = ffi::GX_CC_RASC,
    /// Use the alpha value from rasterizer
    RasAlpha = ffi::GX_CC_RASA,
    One = ffi::GX_CC_ONE,
    Half = ffi::GX_CC_HALF,
    Konst = ffi::GX_CC_KONST,
    /// Use to pass zero value
    Zero = ffi::GX_CC_ZERO,
}

/// Sets the color input sources for one tevstage of the Texture Environment (TEV) color
/// combiner.
/// 
/// See [`ogc_sys::GX_SetTevColorIn()`] for more.
pub fn set_tev_color_in(
    tevstage: u8,
    a: TevColorArg,
    b: TevColorArg,
    c: TevColorArg,
    d: TevColorArg
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevColorIn(tevstage, a as u8, b as u8, c as u8, d as u8); }
}

/// Sets the tevop, tevbias, tevscale and clamp-mode operation for the color combiner for
/// this tevstage of the TEV unit.
/// 
/// See [`ogc_sys::GX_SetTevColorOp()`] for more.
pub fn set_tev_color_op(
    tevstage: u8,
    tevop: TevOp,
    tevbias: TevBias,
    tevscale: TevScale,
    clamp: bool,
    tevregid: TevColOutReg
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe {
        ffi::GX_SetTevColorOp(
            tevstage,
            tevop as u8,
            tevbias as u8,
            tevscale as u8,
            clamp as u8,
            tevregid as u8
        );
    }
}

/// Used to set one of the constant color registers in the TEV unit.
/// 
/// See [`ogc_sys::GX_SetTevColorS10()`] for more.
pub fn set_tev_color_s10(tev_regid: TevColOutReg, color: ColorS10) {
    debug_assert!(
        matches!(tev_regid, TevColOutReg::Reg0 | TevColOutReg::Reg1 | TevColOutReg::Reg2)
    );
    unsafe { ffi::GX_SetTevColorS10(tev_regid as u8, color.0); }
}

/// Used to turn off all indirect texture processing for the specified regular TEV stage.
/// 
/// See [`ogc_sys::GX_SetTevDirect()`] for more.
pub fn set_tev_direct(tevstage: u8) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevDirect(tevstage); }
}

/// Sets up an environment-mapped bump-mapped indirect lookup.
/// 
/// See [`ogc_sys::GX_SetTevIndBumpST()`] for more.
pub fn set_tev_ind_bump_st(tevstage: u8, indtexstage: u8, mtx_sel: IndTexMat) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(indtexstage < ffi::GX_MAX_INDTEXSTAGE as u8);
    unsafe { ffi::GX_SetTevIndBumpST(tevstage, indtexstage, mtx_sel as u8); }
}

/// Sets up an environment-mapped bump-mapped indirect lookup.
/// 
/// See [`ogc_sys::GX_SetTevIndBumpXYZ()`] for more.
pub fn set_tev_ind_bump_xyz(tevstage: u8, indtexstage: u8, mtx_sel: IndTexMat) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(indtexstage < ffi::GX_MAX_INDTEXSTAGE as u8);
    unsafe { ffi::GX_SetTevIndBumpXYZ(tevstage, indtexstage, mtx_sel as u8); }
}

/// Indirect texture format
/// 
/// Bits for the indirect offsets are extracted from the high end of each component byte.
/// Bits for the bump alpha are extraced off the low end of the byte. For
/// `IndTexFmt::EightBits`, the byte is duplicated for the offset and the bump alpha.
#[repr(u32)]
pub enum IndTexFmt {
    EightBits = ffi::GX_ITF_8,
    FiveBits = ffi::GX_ITF_5,
    FourBits = ffi::GX_ITF_4,
    ThreeBits = ffi::GX_ITF_3,
}

/// Indirect texture bias select
/// 
/// The bias is fixed at -128 for `GX_ITF_8` (TODO: what?) and +1 for the other formats.
/// The bias happens prior to the indirect matrix multiply.
#[repr(u32)]
pub enum IndTexBias {
    None = ffi::GX_ITB_NONE,
    S = ffi::GX_ITB_S,
    T = ffi::GX_ITB_T,
    ST = ffi::GX_ITB_ST,
    U = ffi::GX_ITB_U,
    SU = ffi::GX_ITB_SU,
    TU = ffi::GX_ITB_TU,
    STU = ffi::GX_ITB_STU,
}

/// Indirect texture wrap value
/// 
/// Indicates whether the regular texture coordinate should be wrapped before being added
/// to the offset.
#[repr(u32)]
pub enum IndTexWrap {
    Off = ffi::GX_ITW_OFF,
    Mod256 = ffi::GX_ITW_256,
    Mod128 = ffi::GX_ITW_128,
    Mod64 = ffi::GX_ITW_64,
    Mod32 = ffi::GX_ITW_32,
    Mod16 = ffi::GX_ITW_16,
    Zero = ffi::GX_ITW_0,
}

/// Indirect texture bump alpha select
/// 
/// Indicates which offset component should provide the "bump" alpha output for the given
/// TEV stage. Note that bump alpha is not available for TEV stage 0.
#[repr(u32)]
pub enum IndTexAlphaSel {
    Off = ffi::GX_ITBA_OFF,
    S = ffi::GX_ITBA_S,
    T = ffi::GX_ITBA_T,
    U = ffi::GX_ITBA_U,
}

/// Controls how the results from an indirect lookup will be used to modify a given
/// regular TEV stage lookup.
/// 
/// See [`ogc_sys::GX_SetTevIndirect()`] for more.
pub fn set_tev_indirect(
    tevstage: u8,
    indtexid: u8,
    format: IndTexFmt,
    bias: IndTexBias,
    mtxid: IndTexMat,
    wrap_s: IndTexWrap,
    wrap_t: IndTexWrap,
    addprev: bool,
    utclod: bool,
    a: IndTexAlphaSel
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(indtexid < ffi::GX_MAX_INDTEXSTAGE as u8);
    unsafe {
        ffi::GX_SetTevIndirect(
            tevstage,
            indtexid,
            format as u8,
            bias as u8,
            mtxid as u8,
            wrap_s as u8,
            wrap_t as u8,
            addprev as u8,
            utclod as u8,
            a as u8
        )
    }
}

/// Set a given TEV stage to use the same texture coordinates as were computed in the
/// previous stage.
/// 
/// See [`ogc_sys::GX_SetTevIndRepeat()`] for more.
pub fn set_tev_ind_repeat(tevstage: u8) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevIndRepeat(tevstage); }
}

/// Used to implement tiled texturing using indirect textures.
/// 
/// See [`ogc_sys::GX_SetTevIndTile()`] for more.
pub fn set_tev_ind_tile(
    tevstage: u8,
    indtexid: u8,
    tilesize_x: u16,
    tilesize_y: u16,
    tilespacing_x: u16,
    tilespacing_y: u16,
    indtexfmt: IndTexFmt,
    indtexmtx: IndTexMat,
    bias_sel: IndTexBias,
    alpha_sel: IndTexAlphaSel
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(indtexid < ffi::GX_MAX_INDTEXSTAGE as u8);
    unsafe {
        ffi::GX_SetTevIndTile(
            tevstage,
            indtexid,
            tilesize_x,
            tilesize_y,
            tilespacing_x,
            tilespacing_y,
            indtexfmt as u8,
            indtexmtx as u8,
            bias_sel as u8,
            alpha_sel as u8
        )
    }
}

/// Used to warp a normal texture lookup using an indirect texture map with 8-bit offsets.
/// 
/// See [`ogc_sys::GX_SetTevIndWarp()`] for more.
pub fn set_tev_ind_warp(
    tevstage: u8,
    indtexid: u8,
    bias_flag: bool,
    replace_tex: bool,
    mtxid: IndTexMat
) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(indtexid < ffi::GX_MAX_INDTEXSTAGE as u8);
    unsafe {
        ffi::GX_SetTevIndWarp(
            tevstage,
            indtexid,
            bias_flag as u8,
            replace_tex as u8,
            mtxid as u8
        )
    }
}

/// TEV constant alpha selection
#[repr(u32)]
pub enum TevKAlphaSel {
    One = ffi::GX_TEV_KASEL_1,
    Half = ffi::GX_TEV_KASEL_1_2,
    Fourth = ffi::GX_TEV_KASEL_1_4,
    Eighth = ffi::GX_TEV_KASEL_1_8,
    ThreeFourths = ffi::GX_TEV_KASEL_3_4,
    ThreeEighths = ffi::GX_TEV_KASEL_3_8,
    FiveEighths = ffi::GX_TEV_KASEL_5_8,
    SevenEighths = ffi::GX_TEV_KASEL_7_8,
    K0Alpha = ffi::GX_TEV_KASEL_K0_A,
    K0Blue = ffi::GX_TEV_KASEL_K0_B,
    K0Green = ffi::GX_TEV_KASEL_K0_G,
    K0Red = ffi::GX_TEV_KASEL_K0_R,
    K1Alpha = ffi::GX_TEV_KASEL_K1_A,
    K1Blue = ffi::GX_TEV_KASEL_K1_B,
    K1Green = ffi::GX_TEV_KASEL_K1_G,
    K1Red = ffi::GX_TEV_KASEL_K1_R,
    K2Alpha = ffi::GX_TEV_KASEL_K2_A,
    K2Blue = ffi::GX_TEV_KASEL_K2_B,
    K2Green = ffi::GX_TEV_KASEL_K2_G,
    K2Red = ffi::GX_TEV_KASEL_K2_R,
    K3Alpha = ffi::GX_TEV_KASEL_K3_A,
    K3Blue = ffi::GX_TEV_KASEL_K3_B,
    K3Green = ffi::GX_TEV_KASEL_K3_G,
    K3Red = ffi::GX_TEV_KASEL_K3_R,
}

/// Selects a "konstant" alpha input to be used in a given TEV stage.
/// 
/// See [`ogc_sys::GX_SetTevKAlphaSel()`] for more.
pub fn set_tev_k_alpha_sel(tevstage: u8, sel: TevKAlphaSel) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevKAlphaSel(tevstage, sel as u8); }
}

/// Sets one of the "konstant" color registers in the TEV unit.
/// 
/// These registers are available to all TEV stages. They are constant in the sense that
/// they cannot be written to the TEV itself.
/// 
/// See [`ogc_sys::GX_SetTevKColor()`] for more.
pub fn set_tev_k_color(sel: u8, col: Color) {
    debug_assert!(sel < ffi::GX_KCOLOR_MAX as u8);
    unsafe { ffi::GX_SetTevKColor(sel, col.0); }
}

/// Used to set one of the constant color registers in the Texture Environment (TEV) unit.
/// 
/// See [`ogc_sys::GX_SetTevKColorS10()`] for more.
pub fn set_tev_k_color_s10(sel: u8, col: ColorS10) {
    debug_assert!(sel < ffi::GX_KCOLOR_MAX as u8);
    unsafe { ffi::GX_SetTevKColorS10(sel, col.0); }
}

/// TEV constant alpha selection
#[repr(u32)]
pub enum TevKColorSel {
    One = ffi::GX_TEV_KCSEL_1,
    Half = ffi::GX_TEV_KCSEL_1_2,
    Fourth = ffi::GX_TEV_KCSEL_1_4,
    Eighth = ffi::GX_TEV_KCSEL_1_8,
    ThreeFourths = ffi::GX_TEV_KCSEL_3_4,
    ThreeEighths = ffi::GX_TEV_KCSEL_3_8,
    FiveEighths = ffi::GX_TEV_KCSEL_5_8,
    SevenEighths = ffi::GX_TEV_KCSEL_7_8,
    K0 = ffi::GX_TEV_KCSEL_K0,
    K0Alpha = ffi::GX_TEV_KCSEL_K0_A,
    K0Blue = ffi::GX_TEV_KCSEL_K0_B,
    K0Green = ffi::GX_TEV_KCSEL_K0_G,
    K0Red = ffi::GX_TEV_KCSEL_K0_R,
    K1 = ffi::GX_TEV_KCSEL_K1,
    K1Alpha = ffi::GX_TEV_KCSEL_K1_A,
    K1Blue = ffi::GX_TEV_KCSEL_K1_B,
    K1Green = ffi::GX_TEV_KCSEL_K1_G,
    K1Red = ffi::GX_TEV_KCSEL_K1_R,
    K2 = ffi::GX_TEV_KCSEL_K2,
    K2Alpha = ffi::GX_TEV_KCSEL_K2_A,
    K2Blue = ffi::GX_TEV_KCSEL_K2_B,
    K2Green = ffi::GX_TEV_KCSEL_K2_G,
    K2Red = ffi::GX_TEV_KCSEL_K2_R,
    K3 = ffi::GX_TEV_KCSEL_K3,
    K3Alpha = ffi::GX_TEV_KCSEL_K3_A,
    K3Blue = ffi::GX_TEV_KCSEL_K3_B,
    K3Green = ffi::GX_TEV_KCSEL_K3_G,
    K3Red = ffi::GX_TEV_KCSEL_K3_R,
}

/// Selects a "konstant" color input to be used in a given TEV stage.
/// 
/// See [`ogc_sys::GX_SetTevKColorSel()`] for more.
pub fn set_tev_k_color_sel(tevstage: u8, sel: TevKColorSel) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    unsafe { ffi::GX_SetTevKColorSel(tevstage, sel as u8); }
}

/// Selects a set of swap modes for the rasterized color and texture color for a given TEV
/// stage.
/// 
/// `ras_sel` and `tex_sel` are numbers between 0 and 4 inclusive, representing the swap
/// table entry to be used.
/// 
/// See [`ogc_sys::GX_SetTevSwapMode()`] for more.
pub fn set_tev_swap_mode(tevstage: u8, ras_sel: u8, tex_sel: u8) {
    debug_assert!(tevstage < ffi::GX_MAX_TEVSTAGE as u8);
    debug_assert!(ras_sel < ffi::GX_MAX_TEVSWAP as u8);
    debug_assert!(tex_sel < ffi::GX_MAX_TEVSWAP as u8);
    unsafe { ffi::GX_SetTevSwapMode(tevstage, ras_sel, tex_sel); }
}

/// Sets up the TEV color swap table.
/// 
/// See [`ogc_sys::GX_SetTevSwapModeTable()`] for more.
pub fn set_tev_swap_mode_table(swapid: u8, r: u8, g: u8, b: u8, a: u8) {
    debug_assert!(swapid < ffi::GX_MAX_TEVSWAP as u8);
    debug_assert!(r < 4 as u8);
    debug_assert!(g < 4 as u8);
    debug_assert!(b < 4 as u8);
    debug_assert!(a < 4 as u8);
    unsafe { ffi::GX_SetTevSwapModeTable(swapid, r, g, b, a); }
}

/// Sets the texture coordinate bias of a particular texture.
/// 
/// See [`ogc_sys::GX_SetTexCoordBias()`] for more.
pub fn set_tex_coord_bias(texcoord: u8, s_enable: bool, t_enable: bool) {
    debug_assert!(texcoord < ffi::GX_MAXCOORD as u8);
    unsafe { ffi::GX_SetTexCoordBias(texcoord, s_enable as u8, t_enable as u8) }
}

pub fn set_tex_coord_cyl_wrap(texcoord: u8, s_enable: bool, t_enable: bool) {
    debug_assert!(texcoord < ffi::GX_MAXCOORD as u8);
    unsafe { ffi::GX_SetTexCoordCylWrap(texcoord, s_enable as u8, t_enable as u8) }
}

/// Texture coordinate generation type
#[repr(u32)]
pub enum TexGenType {
    /// 3x4 matrix multiply on the input attribute and generate S,T,Q coordinates; S,T are
    /// then divided by Q to produce the actual 2D texture coordinates.
    Mtx3x4 = ffi::GX_TG_MTX3x4,
    /// 2x4 matrix multiply on the input attribute and generate S,T texture coordinates.
    Mtx2x4 = ffi::GX_TG_MTX2x4,
    /// Use light 0 in the bump map calculation.
    Bump0 = ffi::GX_TG_BUMP0,
    /// Use light 1 in the bump map calculation.
    Bump1 = ffi::GX_TG_BUMP1,
    /// Use light 2 in the bump map calculation.
    Bump2 = ffi::GX_TG_BUMP2,
    /// Use light 3 in the bump map calculation.
    Bump3 = ffi::GX_TG_BUMP3,
    /// Use light 4 in the bump map calculation.
    Bump4 = ffi::GX_TG_BUMP4,
    /// Use light 5 in the bump map calculation.
    Bump5 = ffi::GX_TG_BUMP5,
    /// Use light 6 in the bump map calculation.
    Bump6 = ffi::GX_TG_BUMP6,
    /// Use light 7 in the bump map calculation.
    Bump7 = ffi::GX_TG_BUMP7,
    /// Coordinates generated from vertex lighting results; one of the color channel results
    /// is converted into texture coordinates.
    Srtg = ffi::GX_TG_SRTG,
}

/// Texture coordinate source
#[repr(u32)]
pub enum TexGenSrc {
    Pos = ffi::GX_TG_POS,
    Normal = ffi::GX_TG_NRM,
    Binorm = ffi::GX_TG_BINRM,
    Tangent = ffi::GX_TG_TANGENT,
    Tex0 = ffi::GX_TG_TEX0,
    Tex1 = ffi::GX_TG_TEX1,
    Tex2 = ffi::GX_TG_TEX2,
    Tex3 = ffi::GX_TG_TEX3,
    Tex4 = ffi::GX_TG_TEX4,
    Tex5 = ffi::GX_TG_TEX5,
    Tex6 = ffi::GX_TG_TEX6,
    Tex7 = ffi::GX_TG_TEX7,
    TexCoord0 = ffi::GX_TG_TEXCOORD0,
    TexCoord1 = ffi::GX_TG_TEXCOORD1,
    TexCoord2 = ffi::GX_TG_TEXCOORD2,
    TexCoord3 = ffi::GX_TG_TEXCOORD3,
    TexCoord4 = ffi::GX_TG_TEXCOORD4,
    TexCoord5 = ffi::GX_TG_TEXCOORD5,
    TexCoord6 = ffi::GX_TG_TEXCOORD6,
    Color0 = ffi::GX_TG_COLOR0,
    Color1 = ffi::GX_TG_COLOR1,
}

/// An extension of `set_tex_coord_gen()`. It allows one to specify additional
/// texgen options.
/// 
/// See [`ogc_sys::GX_SetTexCoordGen2()`] for more.
pub fn set_tex_coord_gen_2(
    texcoord: u8,
    tgen_type: TexGenType,
    tgen_src: TexGenSrc,
    mtxsrc: u32,
    normalize: bool,
    postmtx: u32
) {
    debug_assert!(texcoord < ffi::GX_MAXCOORD as u8);
    debug_assert!([30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60].contains(&mtxsrc));
    unsafe {
        ffi::GX_SetTexCoordGen2(
            texcoord as u16,
            tgen_type as u32,
            tgen_src as u32,
            mtxsrc,
            normalize as u32,
            postmtx
        )
    }
}

/// Overrides the automatic texture coordinate scaling (based upon the associated map
/// size) and lets one manually assign the scale values that are used for a given
/// texcoord.
/// 
/// See [`ogc_sys::GX_SetTexCoordScaleManually()`] for more.
pub fn set_tex_coord_scale_manually(texcoord: u8, enable: bool, ss: u16, ts: u16) {
    debug_assert!(texcoord < ffi::GX_MAXCOORD as u8);
    unsafe { ffi::GX_SetTexCoordScaleManually(texcoord, enable as u8, ss, ts) }
}

/// This function sets the width and height of the destination texture buffer in texels.
/// 
/// See [`ogc_sys::GX_SetTexCopyDst()`] for more.
pub fn set_tex_copy_dst(wd: u16, ht: u16, fmt: TexFormat, mipmap: bool) {
    unsafe { ffi::GX_SetTexCopyDst(wd, ht, fmt as u32, mipmap as u8)}
}

/// Sets the source parameters for the Embedded Frame Buffer (EFB) to texture image copy.
/// 
/// See [`ogc_sys::GX_SetTexCopySrc()`] for more.
pub fn set_tex_copy_src(left: u16, top: u16, wd: u16, ht: u16) {
    debug_assert!(left.is_multiple_of(2));
    debug_assert!(top.is_multiple_of(2));
    debug_assert!(wd.is_multiple_of(2));
    debug_assert!(ht.is_multiple_of(2));
    unsafe { ffi::GX_SetTexCopySrc(left, top, wd, ht) }
}
