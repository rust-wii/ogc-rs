//! The ``gx`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the graphics functions found in ``gx.h``.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use alloc::vec::Vec;
use bit_field::BitField;
use ffi::GXTexObj;
use voladdress::{Safe, VolAddress};

use num_traits::Float;

use crate::ffi::{self, Mtx as Mtx34, Mtx44};
use crate::gx::regs::BPReg;
use crate::lwp;
use crate::utils::mem;

use self::regs::XFReg;
use self::types::{Gamma, PixelEngineControl, PixelFormat, VtxDest, ZFormat};

pub const GX_PIPE: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0xCC00_8000) };

mod regs;
pub mod types;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Color(ffi::GXColor);

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::with_alpha(r, g, b, 255)
    }

    pub const fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(ffi::GXColor { r, g, b, a })
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

/// Specifies whether the input source color for a color channel comes from a register or a vertex.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Source {
    Register = ffi::GX_SRC_REG as _,
    Vertex = ffi::GX_SRC_VTX as _,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DiffFn {
    None = ffi::GX_DF_NONE as _,
    Signed = ffi::GX_DF_SIGNED as _,
    Clamp = ffi::GX_DF_CLAMP as _,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum AttnFn {
    /// No attenuation
    None = ffi::GX_AF_NONE as _,
    /// Specular computation
    Spec = ffi::GX_AF_SPEC as _,
    /// Spot light attenuation
    Spot = ffi::GX_AF_SPOT as _,
}

/// Object describing a graphics FIFO.
///
/// The Graphics FIFO is the mechanism used to communicate graphics commands from the CPU to the
/// Graphics Processor (GP). The FIFO base pointer is 32-byte aligned and the size must be a
/// multiple of 32 bytes.
///
/// The CPU's write-gather pipe is used to write data to the FIFO. Therefore, the FIFO memory
/// area must be forced out of the CPU cache prior to being used. `DCInvalidateRange()` may be
/// used for this purpose. Due to the mechanics of flushing the write-gather pipe, the FIFO
/// memory area should be at least 32 bytes larger than the maximum expected amount of data
/// stored. Up to 32 NOPs may be written at the end during flushing.
#[derive(Debug)]
#[repr(transparent)]
pub struct Fifo(ffi::GXFifoObj);

impl Default for Fifo {
    fn default() -> Self {
        Fifo::new()
    }
}

impl Fifo {
    /// The minimum allowed size for a FIFO.
    pub const MIN_SIZE: usize = ffi::GX_FIFO_MINSIZE as usize;

    /// Constructs a new `Fifo` with the minimum size.
    pub fn new() -> Self {
        Self::with_size(Self::MIN_SIZE)
    }

    /// Constructs a new `Fifo` with the given size.
    ///
    /// If the given size is less than the minimum, the minimum size is used.
    pub fn with_size(mut size: usize) -> Self {
        let mut fifo = core::mem::MaybeUninit::zeroed();

        if size < Fifo::MIN_SIZE {
            size = Fifo::MIN_SIZE;
        }

        let mut buf = crate::utils::Buf32::new(size);

        // SAFETY:
        // + original libogc source suggests that available init functions don't
        //   completely initialize the fifo, so it's been zeroed() above just in
        //   case.
        // + all other safety ensured by Buf32.
        unsafe {
            ffi::GX_InitFifoBase(
                fifo.as_mut_ptr(),
                buf.as_mut_ptr().map_addr(mem::to_uncached) as *mut _,
                buf.len() as u32,
            );
            Fifo(fifo.assume_init())
        }
    }

    /// Sets the high and low water mark for the fifo.
    ///
    /// The high and low water marks are used in *immediate-mode*, i.e. when the fifo is attached
    /// to both the CPU and Graphics Processor (GP) (see `Gx::set_cpu_fifo()` and
    /// `Gx::set_gp_fifo()`).
    ///
    /// The hardware keeps track of the number of bytes between the read and write pointers. This
    /// number represents how full the FIFO is, and when it is greater than or equal to the
    /// *hiwatermark*, the hardware issues an interrupt. The GX API will suspend sending graphics
    /// to the Graphics FIFO until it has emptied to a certain point. The *lowatermark* is used to
    /// set the point at which the FIFO is empty enough to resume sending graphics commands to the
    /// FIFO. Both *hiwatermark* and *lowatermark* should be in multiples of 32B. The count for
    /// *lowatermark* should be less than *hiwatermark*. Of course, *hiwatermark* and *lowatermark*
    /// must be less than the size of the FIFO.
    ///
    /// # Note
    /// When the FIFO is only attached to the CPU or only attached to the GP, the high and low
    /// watermark interrupts are disabled.
    pub fn set_limits(&mut self, hiwatermark: u32, lowatermark: u32) {
        assert_eq!(0, hiwatermark % 32);
        assert_eq!(0, lowatermark % 32);
        // assert!(hiwatermark < self.len());
        // assert!(lowatermark < self.len());
        unsafe { ffi::GX_InitFifoLimits(&mut self.0, hiwatermark, lowatermark) }
    }

    /// Get the base address for a given *fifo*.
    pub fn base(&self) -> *mut u8 {
        unsafe { ffi::GX_GetFifoBase(self as *const _ as *mut _) as *mut _ }
    }

    /// Returns number of cache lines in the FIFO.
    ///
    /// # Note
    /// The count is incorrect if an overflow has occurred (i.e. you have written more data than
    /// the size of the fifo), as the hardware cannot detect an overflow in general.
    pub fn cache_lines(&self) -> usize {
        // TODO: remove conversions when upstream changes pass.
        unsafe { ffi::GX_GetFifoCount(self as *const _ as *mut _) as usize }
    }

    /// Get the size of a given FIFO.
    pub fn len(&self) -> usize {
        // TODO: remove conversions when upstream changes pass.
        unsafe { ffi::GX_GetFifoSize(self as *const _ as *mut _) as usize }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns a non-zero value if the write pointer has passed the TOP of the FIFO.
    ///
    /// Returns true only if the FIFO is attached to the CPU and the FIFO write pointer has passed
    /// the top of the FIFO. Use the return value to detect whether or not an overflow has occured
    /// by initializing the FIFO's write pointer to the base of the FIFO before sending any
    /// commands to the FIFO.
    ///
    /// # Note
    /// If the FIFO write pointer is not explicitly set to the base of the FIFO, you cannot rely on
    /// this function to detect overflows.
    pub fn get_wrap(&self) -> u8 {
        unsafe { ffi::GX_GetFifoWrap(self as *const _ as *mut _) }
    }

    /// Returns the current value of the Graphics FIFO read and write pointers.
    ///
    /// # Note
    /// See `Gx::enable_breakpoint()` for an example of why you would do this.
    pub fn pointers(&self) -> (*const u8, *mut u8) {
        let mut rd_ptr = core::ptr::null_mut();
        let mut wt_ptr = core::ptr::null_mut();
        unsafe {
            ffi::GX_GetFifoPtrs(self as *const _ as *mut _, &mut rd_ptr, &mut wt_ptr);
        }
        (rd_ptr as *const _, wt_ptr as *mut _)
    }

    /// Sets the *fifo* read and write pointers.
    ///
    /// # Note
    /// This is normally done only during initialization of the FIFO. After that, the graphics
    /// hardware manages the FIFO pointers.
    pub fn set_pointers(&mut self, rd_ptr: *const u8, wt_ptr: *mut u8) {
        unsafe { ffi::GX_InitFifoPtrs(&mut self.0, rd_ptr as *mut _, wt_ptr as *mut _) }
    }
}

/// Object containing information on a light.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Light(ffi::GXLightObj);

/// Type of the brightness decreasing function by distance.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DistFn {
    Off = ffi::GX_DA_OFF as _,
    Gentle = ffi::GX_DA_GENTLE as _,
    Medium = ffi::GX_DA_MEDIUM as _,
    Steep = ffi::GX_DA_STEEP as _,
}

/// Spot illumination distribution function
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum SpotFn {
    Off = ffi::GX_SP_OFF as _,
    Flat = ffi::GX_SP_FLAT as _,
    Cos = ffi::GX_SP_COS as _,
    Cos2 = ffi::GX_SP_COS2 as _,
    Sharp = ffi::GX_SP_SHARP as _,
    Ring1 = ffi::GX_SP_RING1 as _,
    Ring2 = ffi::GX_SP_RING2 as _,
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
    /// The parameters `a0`, `a1`, and `a2` are used for angular (spotlight) attenuation. The
    /// coefficients `k0`, `k1`, and `k2` are used for distance attenuation. The attenuation
    /// function is:
    ///
    /// `atten = clamp0(a2^2 * aattn^2 + a1 * aattn + a0) / (k2 * d^2 + k1 * d + k0)`
    ///
    /// where `aattn` is the cosine of the angle between the light direction and the vector from
    /// the light position to the vertex, and `d` is the distance from the light position to the
    /// vertex when the channel attenuation function is `AttnFn::Spot`. The light color will be
    /// multiplied by the `atten` factor when the attenuation function for the color channel
    /// referencing this light is set to `AttnFn::Spot` (see [`Gx::set_channel_controls()`]).
    ///
    /// # Note
    /// The convenience function [`Light::spot_attn()`] can be used to set the angle
    /// attenuation coefficents based on several spot light types. The convenience function
    /// [`Light::dist_attn()`] can be used to set the distance attenuation coefficients
    /// using one of several common attenuation functions.
    ///
    /// The convenience macro [`Light::shininess()`] can be used to set the attenuation
    /// parameters for specular lights.
    ///
    /// When the channel attenuation function is set to `AttnFn::Spec`, the `aattn` and `d`
    /// parameter are equal to the dot product of the eye-space vertex normal and the half-angle
    /// vector set by [`Light::specular_dir()`].
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
    /// `AttnFn::Spec` using [`Gx::set_channel_controls()`].
    pub fn shininess(&mut self, shininess: f32) -> &mut Self {
        self.attn(0.0, 0.0, 1.0, shininess / 2.0, 0.0, 1.0 - shininess / 2.0)
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
    /// vertex. The `k0-2` coefficients can be set using [`Light::attn_k()`]. You can set
    /// both the `a0-2` and `k0-2` coefficients can be set using [`Light::attn()`]. The
    /// light color will be multiplied by the atten factor when the attenuation function for the
    /// color channel referencing this light is set to `AttnFn::Spot` (see
    /// [`Gx::set_channel_controls()`]).
    ///
    /// # Note
    /// The convenience function [`Light::spot_attn()`] can be used to set the angle
    /// attenuation coefficents based on several spot light types. The convenience function
    /// [`Light::dist_attn()`] can be used to set the distance attenuation coefficients
    /// using one of several common attenuation functions.
    pub fn attn_a(&mut self, a0: f32, a1: f32, a2: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightAttnA(&mut self.0, a0, a1, a2);
        }
        self
    }

    /// Sets coefficients used in the lighting distance attenuation calculation in a given light
    /// object.
    ///
    /// The coefficients `k0`, `k1`, and `k2` are used for distance attenuation. The attenuation
    /// function is:
    ///
    /// `atten = clamp0(a2^2 * cos(theta)^2 + a1 * cos(theta) + a0) / (k2 * d^2 + k1 * d + k0)`
    ///
    /// where `cos(theta)` is the cosine of the angle between the light normal and the vector from
    /// the light position to the vertex, and `d` is the distance from the light position to the
    /// vertex. The `a0-2` coefficients can be set using [`Light::attn_a()`]. You can set
    /// both the `a0-2` and `k0-2` coefficients can be set using [`Light::attn()`]. The
    /// light color will be multiplied by the atten factor when the attenuation function for the
    /// color channel referencing this light is set to `AttnFn::Spot` (see
    /// [`Gx::set_channel_controls()`]).
    ///
    /// # Note
    /// The convenience function [`Light::spot_attn()`] can be used to set the angle attenuation
    /// coefficents based on several spot light types. The convenience function
    /// [`Light::dist_attn()`] can be used to set the distance attenuation coefficients using one
    /// of several common attenuation functions.
    pub fn attn_k(&mut self, k0: f32, k1: f32, k2: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightAttnK(&mut self.0, k0, k1, k2);
        }
        self
    }

    /// Sets the color of the light in the light object.
    pub fn color(&mut self, color: Color) -> &mut Self {
        unsafe {
            ffi::GX_InitLightColor(&mut self.0, color.0);
        }
        self
    }

    /// Sets the direction of a light in the light object.
    ///
    /// This direction is used when the light object is used as spotlight or a specular light (see
    /// the `attn_fn` parameter of [`Gx::set_channel_controls()`]).
    ///
    /// # Note
    /// The coordinate space of the light normal should be consistent with a vertex normal
    /// transformed by a normal matrix; i.e., it should be transformed to view space.
    ///
    /// This function does not set the direction of parallel directional diffuse lights. If you
    /// want parallel diffuse lights, you may put the light position very far from every objects to
    /// be lit. (See [`Light::pos()`] and [`Gx::set_channel_controls()`])
    pub fn dir(&mut self, nx: f32, ny: f32, nz: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightDir(&mut self.0, nx, ny, nz);
        }
        self
    }

    /// Sets coefficients for distance attenuation in a light object.
    ///
    /// This function uses three easy-to-control parameters instead of `k0`, `k1`, and `k2` in
    /// [`Light::attn()`].
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
    /// If you want more flexible control, it is better to use [`Light::attn()`] and calculate
    /// appropriate coefficients.
    ///
    /// This function sets parameters only for distance attenuation. Parameters for angular
    /// attenuation should be set by using [`Light::spot_attn()`] or [`Light::attn_a()`].
    pub fn dist_attn(&mut self, ref_dist: f32, ref_brite: f32, dist_fn: DistFn) -> &mut Self {
        unsafe {
            ffi::GX_InitLightDistAttn(&mut self.0, ref_dist, ref_brite, dist_fn as u8);
        }
        self
    }

    /// Sets the position of the light in the light object.
    ///
    /// The GameCube graphics hardware supports local diffuse lights. The position of the light
    /// should be in the same space as a transformed vertex position (i.e., view space).
    ///
    /// # Note
    /// Although the hardware doesn't support parallel directional diffuse lights, it is possible
    /// to get "almost parallel" lights by setting sufficient large values to position parameters
    /// (x, y and z) which makes the light position very far away from objects to be lit and all
    /// rays considered almost parallel.
    pub fn pos(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitLightPos(&mut self.0, x, y, z);
        }
        self
    }

    /// Sets coefficients for angular (spotlight) attenuation in light object.
    ///
    /// This function uses two easy-to-control parameters instead of `a0`, `a1`, and `a2` on
    /// [`Light::attn()`].
    ///
    /// The parameter `cut_off` specifies cutoff angle of the spotlight by degree. The spotlight
    /// works while the angle between the ray for a vertex and the light direction given by
    /// [`Light::dir()`] is smaller than this cutoff angle. The value for `cut_off` should be
    /// within `0 < cut_off <= 90.0`, otherwise given light object doesn't become a spotlight.
    ///
    /// The parameter `spotfn` defines type of the illumination distribution within cutoff angle.
    /// The value `SpotFn::Off` turns spotlight feature off even if color channel setting is using
    /// `AttnFn::Spot` (see [`Gx::set_channel_controls()`]).
    ///
    /// # Note
    /// This function can generate only some kind of simple spotlights. If you want more flexible
    /// control, it is better to use [`Light::attn()`] and calculate appropriate coefficients.
    ///
    /// This function sets parameters only for angular attenuation. Parameters for distance
    /// attenuation should be set by using [`Light::dist_attn()`] or [`Light::attn_k()`].
    pub fn spot_attn(&mut self, cut_off: f32, spotfn: SpotFn) -> &mut Self {
        unsafe {
            ffi::GX_InitLightSpot(&mut self.0, cut_off, spotfn as u8);
        }
        self
    }

    /// Sets the direction of a specular light in the light object.
    ///
    /// This direction is used when the light object is used only as specular light. The coordinate
    /// space of the light normal should be consistent with a vertex normal transformed by a normal
    /// matrix; i.e., it should be transformed to view space.
    ///
    /// # Note
    /// This function should be used if and only if the light object is used as specular light. One
    /// specifies a specular light in [`Gx::set_channel_controls()`] by setting the [attenuation
    /// function](`AttnFn`) to `AttnFn::Spec`. Furthermore, one must not use [`Light::dir()`] or
    /// [`Light::pos()`] to set up a light object which will be used as a specular light since
    /// these functions will destroy the information set by [`Light::specular_dir()`]. In contrast
    /// to diffuse lights (including spotlights) that are considered local lights, a specular light
    /// is a parallel light (i.e. the specular light is infinitely far away such that all the rays
    /// of the light are parallel), and thus one can only specify directional information.
    pub fn specular_dir(&mut self, nx: f32, ny: f32, nz: f32) -> &mut Self {
        unsafe {
            ffi::GX_InitSpecularDir(&mut self.0, nx, ny, nz);
        }
        self
    }

    /// Sets the direction and half-angle vector of a specular light in the light object.
    ///
    /// These vectors are used when the light object is used only as specular light. In contrast to
    /// [`Light::specular_dir()`], which caclulates half-angle vector automatically by
    /// assuming the view vector as (0, 0, 1), this function allows users to specify half-angle
    /// vector directly as input arguments. It is useful to do detailed control for orientation of
    /// highlights.
    ///
    /// See also [`Light::specular_dir()`].
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
#[repr(u8)]
pub enum TexFilter {
    /// Point sampling, no mipmap
    Near = ffi::GX_NEAR as _,
    /// Point sampling, linear mipmap
    NearMipLin = ffi::GX_NEAR_MIP_LIN as _,
    /// Point sampling, discrete mipmap
    NearMipNear = ffi::GX_NEAR_MIP_NEAR as _,
    /// Trilinear filtering
    LinMipLin = ffi::GX_LIN_MIP_LIN as _,
    /// Bilinear filtering, discrete mipmap
    LinMipNear = ffi::GX_LIN_MIP_NEAR as _,
    /// Bilinear filtering, no mipmap
    Linear = ffi::GX_LINEAR as _,
}

/// Texture wrap modes
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum WrapMode {
    Clamp = ffi::GX_CLAMP as _,
    Repeat = ffi::GX_REPEAT as _,
    Mirror = ffi::GX_MIRROR as _,
}

#[repr(transparent)]
pub struct Texture<'img>(ffi::GXTexObj, PhantomData<&'img [u8]>);

impl Texture<'_> {
    /// Used to initialize or change a texture object for non-color index textures.
    pub fn new(
        img: &[u8],
        width: u16,
        height: u16,
        format: u8,
        wrap_s: WrapMode,
        wrap_t: WrapMode,
        mipmap: bool,
    ) -> Texture {
        let texture = core::mem::MaybeUninit::zeroed();
        assert_eq!(0, img.as_ptr().align_offset(32));
        assert!(width <= 1024, "max width for texture is 1024");
        assert!(height <= 1024, "max height for texture is 1024");
        unsafe {
            ffi::GX_InitTexObj(
                texture.as_ptr() as *mut _,
                img.as_ptr() as *mut _,
                width,
                height,
                format,
                wrap_s as u8,
                wrap_t as u8,
                mipmap as u8,
            );
            Texture(texture.assume_init(), PhantomData)
        }
    }

    /// Used to initialize or change a texture object when the texture is color index format.
    pub fn with_color_idx(
        img: &[u8],
        width: u16,
        height: u16,
        format: u8,
        wrap: (WrapMode, WrapMode),
        mipmap: bool,
        tlut_name: u32,
    ) -> Texture {
        let texture = core::mem::MaybeUninit::zeroed();
        assert_eq!(0, img.as_ptr().align_offset(32));
        assert!(width <= 1024, "max width for texture is 1024");
        assert!(height <= 1024, "max height for texture is 1024");
        unsafe {
            ffi::GX_InitTexObjCI(
                texture.as_ptr() as *mut _,
                img.as_ptr() as *mut _,
                width,
                height,
                format,
                wrap.0 as u8,
                wrap.1 as u8,
                mipmap as u8,
                tlut_name,
            );
            Texture(texture.assume_init(), PhantomData)
        }
    }

    /// Returns the texture height.
    pub fn height(&self) -> u16 {
        // TODO: remove conversions when upstream changes pass.
        unsafe { ffi::GX_GetTexObjHeight(self as *const _ as *mut _) }
    }

    /// Returns the texture width.
    pub fn width(&self) -> u16 {
        // TODO: remove conversions when upstream changes pass.
        unsafe { ffi::GX_GetTexObjWidth(self as *const _ as *mut _) }
    }

    /// Returns `true` if the texture's mipmap flag is enabled.
    pub fn is_mipmapped(&self) -> bool {
        // TODO: remove conversions when upstream changes pass.
        unsafe { ffi::GX_GetTexObjMipMap(self as *const _ as *mut _) != 0 }
    }

    /// Enables bias clamping for texture LOD.
    ///
    /// If set to `true`, the sum of LOD and `lodbias` (given in [`Texture::set_lod_bias()`])
    /// is clamped so that it is never less than the minimum extent of the pixel projected in
    /// texture space. This prevents over-biasing the LOD when the polygon is perpendicular to the
    /// view direction.
    pub fn set_bias_clamp(&mut self, enable: bool) {
        unsafe { ffi::GX_InitTexObjBiasClamp(&mut self.0, enable as u8) }
    }

    /// Changes LOD computing mode.
    ///
    /// When set to `true`, the LOD is computed using adjacent texels; when `false`, diagonal
    /// texels are used instead. This should be set to `true` if you use bias clamping (see
    /// [`Texture::set_bias_clamp()`]) or anisotropic filtering (`GX_ANISO_2` or `GX_ANISO_4` for
    /// [`Texture::set_max_aniso()`] argument).
    pub fn set_edge_lod(&mut self, enable: bool) {
        unsafe { ffi::GX_InitTexObjEdgeLOD(&mut self.0, enable as u8) }
    }

    /// Sets the filter mode for a texture.
    ///
    /// When the ratio of texels for this texture to pixels is not 1:1, the filter type for
    /// `minfilt` or `magfilt` is used. `minfilt` is used when the texel/pixel ratio is >= 1.0.
    /// `magfilt` is used when the texel/pixel ratio is < 1.0; needs to be `Near` or `Linear`.
    pub fn set_filter_mode(&mut self, minfilt: TexFilter, magfilt: TexFilter) {
        debug_assert!(
            matches!(magfilt, TexFilter::Near | TexFilter::Linear),
            "magfilt can only be `TexFilter::Near` or `TexFilter::Linear`"
        );
        unsafe { ffi::GX_InitTexObjFilterMode(&mut self.0, minfilt as u8, magfilt as u8) }
    }

    /// Sets texture Level Of Detail (LOD) controls explicitly for a texture object.
    ///
    /// It is the application's responsibility to provide memory for a texture object. When
    /// initializing a texture object using [`Texture::new()`] or [`Texture::with_color_idx()`],
    /// this information is set to default values based on the mipmap flag. This function allows
    /// the programmer to override those defaults.
    ///
    /// # Note
    /// This function should be called after [`Texture::new()`] or [`Texture::with_color_idx()`]
    /// for a particular texture object.
    ///
    /// Setting `biasclamp` prevents over-biasing the LOD when the polygon is perpendicular to the
    /// view direction.
    ///
    /// `edgelod` should be set if `biasclamp` is set or `maxaniso` is set to `GX_ANISO_2` or
    /// `GX_ANISO_4`.
    ///
    /// Theoretically, there is no performance difference amongst various
    /// magnification/minification filter settings except `GX_LIN_MIP_LIN` filter with
    /// `GX_TF_RGBA8` texture format which takes twice as much as other formats. However, this
    /// argument is assuming an environment where texture cache always hits. On real environments,
    /// you will see some performance differences by changing filter modes (especially minification
    /// filter) because cache-hit ratio changes according to which filter mode is being used.
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
                &mut self.0,
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
    pub fn set_lod_bias(&mut self, lodbias: f32) {
        unsafe { ffi::GX_InitTexObjLODBias(&mut self.0, lodbias) }
    }

    /// Sets the maximum anisotropic filter to use for a texture.
    pub fn set_max_aniso(&mut self, maxaniso: u8) {
        unsafe { ffi::GX_InitTexObjMaxAniso(&mut self.0, maxaniso) }
    }

    /// Sets the maximum LOD for a given texture.
    pub fn set_max_lod(&mut self, maxlod: f32) {
        debug_assert!(
            (0.0..=10.0).contains(&maxlod),
            "valid range for max LOD is 0.0 to 10.0"
        );
        unsafe { ffi::GX_InitTexObjMaxLOD(&mut self.0, maxlod) }
    }

    /// Sets the minimum LOD for a given texture.
    pub fn set_min_lod(&mut self, minlod: f32) {
        debug_assert!(
            (0.0..=10.0).contains(&minlod),
            "valid range for min LOD is 0.0 to 10.0"
        );
        unsafe { ffi::GX_InitTexObjMinLOD(&mut self.0, minlod) }
    }

    /// Allows one to modify the TLUT that is associated with an existing texture object.
    pub fn set_tlut(&mut self, tlut_name: u32) {
        unsafe { ffi::GX_InitTexObjTlut(&mut self.0, tlut_name) }
    }

    /// Allows one to modify the texture coordinate wrap modes for an existing texture object.
    pub fn set_wrap_mode(&mut self, wrap_s: WrapMode, wrap_t: WrapMode) {
        unsafe { ffi::GX_InitTexObjWrapMode(&mut self.0, wrap_s as u8, wrap_t as u8) }
    }

    pub fn gxtexobj(&mut self) -> &mut GXTexObj {
        &mut self.0
    }
}

impl<'a> From<GXTexObj> for Texture<'a> {
    fn from(obj: GXTexObj) -> Self {
        Self(obj, PhantomData)
    }
}

/// Vertex attribute array type
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum VtxAttr {
    Null = ffi::GX_VA_NULL as _,
    LightArray = ffi::GX_LIGHTARRAY as _,
    NrmMtxArray = ffi::GX_NRMMTXARRAY as _,
    PosMtxArray = ffi::GX_POSMTXARRAY as _,
    TexMtxArray = ffi::GX_TEXMTXARRAY as _,
    Color0 = ffi::GX_VA_CLR0 as _,
    Color1 = ffi::GX_VA_CLR1 as _,
    MaxAttr = ffi::GX_VA_MAXATTR as _,
    /// Normal, binormal, tangent
    Nbt = ffi::GX_VA_NBT as _,
    Nrm = ffi::GX_VA_NRM as _,
    Pos = ffi::GX_VA_POS as _,
    PtnMtxIdx = ffi::GX_VA_PTNMTXIDX as _,
    Tex0 = ffi::GX_VA_TEX0 as _,
    Tex0MtxIdx = ffi::GX_VA_TEX0MTXIDX as _,
    Tex1 = ffi::GX_VA_TEX1 as _,
    Tex1MtxIdx = ffi::GX_VA_TEX1MTXIDX as _,
    Tex2 = ffi::GX_VA_TEX2 as _,
    Tex2MtxIdx = ffi::GX_VA_TEX2MTXIDX as _,
    Tex3 = ffi::GX_VA_TEX3 as _,
    Tex3MtxIdx = ffi::GX_VA_TEX3MTXIDX as _,
    Tex4 = ffi::GX_VA_TEX4 as _,
    Tex4MtxIdx = ffi::GX_VA_TEX4MTXIDX as _,
    Tex5 = ffi::GX_VA_TEX5 as _,
    Tex5MtxIdx = ffi::GX_VA_TEX5MTXIDX as _,
    Tex6 = ffi::GX_VA_TEX6 as _,
    Tex6MtxIdx = ffi::GX_VA_TEX6MTXIDX as _,
    Tex7 = ffi::GX_VA_TEX7 as _,
    Tex7MtxIdx = ffi::GX_VA_TEX7MTXIDX as _,
}

/// Structure describing how a single vertex attribute will be referenced.
///
/// An array of these structures can be used to describe all the attributes in a vertex. The
/// attribute `GX_VA_NULL` should be used to mark the end of the array.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct VtxDesc(ffi::GXVtxDesc);

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ProjectionType {
    Perspective = ffi::GX_PERSPECTIVE as _,
    Orthographic = ffi::GX_ORTHOGRAPHIC as _,
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

/// Represents the GX service.
pub struct Gx;

impl Gx {
    /// Initializes the graphics processor to its initial state.
    ///
    /// This function sets the default state of the graphics processor and should be called before
    /// any other GX functions. This function sets up an immediate-mode method of communicating
    /// graphics commands from the CPU to the Graphics Processor (GP). This function will
    /// initialize a FIFO and attach it to both the CPU and GP. The CPU will write commands to the
    /// FIFO and the GP will read the commands. This function returns a pointer to the initialized
    /// FIFO. The application must allocate the memory for the FIFO. The parameter `base` is a
    /// pointer to the allocated main memory and must be aligned to 32B. `size` is the size of the
    /// FIFO in bytes and must be a multiple of 32B. Refer to additional notes in [`Fifo`]
    /// concerning the FIFO memory.
    ///
    /// # Note
    /// It is also possible to override the default immediate-mode style and instead buffer the
    /// graphics for frame n+1 while the GP is reading the graphics for frame n. See
    /// [`Gx::set_cpu_fifo()`] and [`Gx::set_gp_fifo()`] for further information.
    ///
    /// This function also designates the calling thread as the default GX thread; i.e., it assumes
    /// the calling thread is the one responsible for generating graphics data. This thread will be
    /// the thread to be suspended when the FIFO gets too full. The current GX thread can be
    /// changed by calling [`Gx::set_current_gx_thread()`].
    pub fn init(mut size: usize) -> &'static mut Fifo {
        if size < Fifo::MIN_SIZE {
            size = Fifo::MIN_SIZE;
        }

        //TODO: keep buf around othersise can start overwritting memory
        let mut buf = ManuallyDrop::new(crate::utils::Buf32::new(size));

        // SAFETY: all safety is ensured by Buf32.
        unsafe {
            let fifo = ffi::GX_Init(
                buf.as_mut_ptr().map_addr(mem::to_uncached) as *mut _,
                buf.len() as u32,
            );
            &mut *(fifo as *mut Fifo)
        }
    }

    /// Attaches *fifo* to the GP.
    ///
    /// # Note
    /// If the FIFO is also attached to the CPU, the system is in immediate-mode, and the fifo acts
    /// like a true FIFO. In immediate-mode, graphics commands are fed directly from the CPU to the
    /// GP, and the FIFO's high and low water marks are active. The high and low water marks
    /// implement the flow-control mechanism between the CPU and GP. When the FIFO becomes more
    /// full than the high water mark, the CPU will stop writing graphics commands into the FIFO.
    /// When the FIFO empties to a point lower than the low water mark, the CPU will resume writing
    /// graphics commands into the FIFO. The high and low water marks are set with
    /// [`Fifo::set_limits()`].
    ///
    /// If the FIFO is only attached to the GP, the FIFO acts like a buffer. In this case, high and
    /// low water marks are disabled, and the GP reads the FIFO until it is empty. Before attaching
    /// a new FIFO to the GP, you should make sure the previous FIFO is empty, using the *cmdIdle*
    /// status returned by [`Gx::get_gp_status()`].
    ///
    /// The break point mechanism can be used to force the FIFO to stop reading commands at a
    /// certain point; see [`Gx::enable_breakpt()`].
    pub fn set_gp_fifo(fifo: Fifo) {
        unsafe { ffi::GX_SetGPFifo((&fifo) as *const _ as *mut _) }
    }

    /// Attaches a FIFO to the CPU.
    ///
    /// # Note
    /// If the FIFO being attached is one already attached to the GP, the FIFO can be considered to
    /// be in immediate mode. If not, the CPU can write commands, and the GP will execute them when
    /// the GP attaches to this FIFO (multi-buffered mode).
    pub fn set_cpu_fifo(fifo: Fifo) {
        unsafe { ffi::GX_SetCPUFifo((&fifo) as *const _ as *mut _) }
    }

    /// Returns the current GX thread.
    ///
    /// The current GX thread should be the thread that is currently responsible
    /// for generating graphics data. By default, the GX thread is the thread
    /// that invoked [`Gx::init()`]; however, it may be changed by calling
    /// [`Gx::set_current_gx_thread()`].
    ///
    /// # Note
    /// When graphics data is being generated in immediate mode (that is, the
    /// CPU FIFO = GP FIFO, and the GP is actively consuming data), the high
    /// watermark may be triggered. When this happens, the high watermark
    /// interrupt handler will suspend the GX thread, thus preventing any
    /// further graphics data from being generated. The low watermark interrupt
    /// handler will resume the thread.
    pub fn get_current_gx_thread() -> lwp::Thread {
        lwp::Thread::new(unsafe { ffi::GX_GetCurrentGXThread() })
    }

    /// Aborts the current frame.
    ///
    /// This command will reset the entire graphics pipeline, including any commands in the
    /// graphics FIFO.
    ///
    /// # Note
    /// Texture memory will not be reset, so currently loaded textures will still be valid;
    /// however, when loading texture using [`Gx::preload_entire_texture()`] or TLUTs using
    /// [`Gx::load_tlut()`], you must make sure the command completed. You can use the draw sync
    /// mechanism to do this; see [`Gx::set_draw_sync()`] and [`Gx::get_draw_sync()`].
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
    /// # Note
    /// The counter's function is set using [`Gx::set_gp_metric()`]; the counter's value is read
    /// using [`Gx::read_gp_metric()`]. Consult these for more details.
    ///
    /// # Safety
    /// This function resets CPU accessible counters, so it should **not** be used in a display list.
    pub unsafe fn clear_gp_metric() {
        ffi::GX_ClearGPMetric()
    }

    /// Clears the Vertex Cache performance counter.
    ///
    /// This function clears the performance counter by sending a special clear token via the
    /// Graphics FIFO.
    ///
    /// # Note
    /// To set the metric for the counter, call [`Gx::set_vcache_metric()`]; to read the counter
    /// value, call [`Gx::read_vcache_metric()`].
    pub fn clear_vcache_metric() {
        unsafe { ffi::GX_ClearVCacheMetric() }
    }

    /// Sets a breakpoint that causes the GP to halt when encountered.
    ///
    /// # Note
    /// The break point feature allows the application to have two frames of graphics in the FIFO
    /// at the same time, overlapping one frame's processing by the graphics processor with another
    /// frame's processing by the CPU. For example, suppose you finish writing the graphics
    /// commands for one frame and are ready to start on the next. First, execute a [`Gx::flush()`]
    /// command to make sure all the data in the CPU write gatherer is flushed into the FIFO. This
    /// will also align the FIFO write pointer to a 32B boundary. Next, read the value of the
    /// current write pointer using [`Fifo::pointers()`]. Write the value of the write pointer as
    /// the break point address using `Gx::enable_breakpt()`. When the FIFO read pointer reaches
    /// the break point address the hardware will disable reads from the FIFO. The status `brkpt`,
    /// returned by [`Gx::get_gp_status()`], can be polled to detect when the break point is
    /// reached. The application can then decide when to disable the break point, using
    /// [`Gx::disable_breakpt()`], which will allow the FIFO to resume reading graphics commands.
    ///
    /// FIFO reads will stall when the GP FIFO read pointer is equal to the break point address
    /// `break_pt`. To re-enable reads of the GP FIFO, use [`Gx::disable_breakpt()`].
    ///
    /// Use [`Gx::set_breakpt_callback()`] to set what function runs when the breakpoint is
    /// encountered.
    pub fn enable_breakpt(break_pt: *mut u8) {
        unsafe { ffi::GX_EnableBreakPt(break_pt as _) }
    }

    /// Registers `cb` as a function to be invoked when a break point is encountered.
    ///
    /// Passing `None` means no function will run. The return value is a pointer to the previously
    /// registered callback, if any.
    ///
    /// # Safety
    /// The callback will run with interrupts disabled, so it should terminate as quickly as
    /// possible.
    pub unsafe fn set_breakpt_callback(
        cb: Option<unsafe extern "C" fn()>,
    ) -> Option<unsafe extern "C" fn()> {
        ffi::GX_SetBreakPtCallback(cb)
    }

    /// Allows reads from the FIFO currently attached to the Graphics Processor (GP) to resume.
    ///
    /// See [`Gx::enable_breakpt()`] for an explanation of the FIFO break point feature.
    ///
    /// # Note
    /// The breakpoint applies to the FIFO currently attached to the Graphics Processor (GP) (see
    /// [`Gx::set_gp_fifo()`]).
    pub fn disable_breakpt() {
        unsafe { ffi::GX_DisableBreakPt() }
    }

    /// Initialize the transformation unit (XF) rasterizer unit (RAS) to take performance
    /// measurements.
    ///
    /// # Safety
    /// This function should be avoided; use the GP performance metric functions instead.
    pub unsafe fn init_xf_ras_metric() {
        ffi::GX_InitXfRasMetric()
    }

    /// Reads the current status of the GP.
    ///
    /// `overhi` and `underlow` will indicate whether or not the watermarks have been reached. If
    /// the CPU and GP FIFOs are the same, then `overhi` will indicate whether or not the current
    /// GX thread is suspended. The value of `brkpt` can be used to determine if a breakpoint is in
    /// progress (i.e. GP reads are suspended; they are resumed by a call to
    /// [`Gx::disable_breakpt()`]). A callback can also be used to notify your application that
    /// the break point has been reached. (see [`Gx::set_breakpt_callback()`])
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
    /// This function copies the light object data into the graphics FIFO through the CPU
    /// write-gather buffer mechanism. This guarantees that the light object is coherent with the
    /// CPU cache.
    ///
    /// # Note
    /// The light object must have been initialized first using the necessary `GX_InitLight*()`
    /// functions.
    ///
    /// Another way to load a light object is with `Gx::load_light_idx()`.
    pub fn load_light(lit_obj: &Light, lit_id: u8) {
        unsafe { ffi::GX_LoadLightObj(lit_obj as *const _ as *mut _, lit_id) }
    }

    /// Instructs the GP to fetch the light object at *litobjidx* from an array.
    ///
    /// The light object is retrieved from the array to which
    /// `Gx::set_array(GX_VA_LIGHTARRAY, ...)` points. Then it loads the object into the hardware
    /// register associated with Light ID.
    ///
    /// # Note
    /// Data flows directly from the array in DRAM to the GP; therefore, the light object data may
    /// not be coherent with the CPU's cache. The application is responsible for storing the light
    /// object data from the CPU cache (using `DCStoreRange()`) before calling
    /// `Gx::load_light_idx()`.
    pub fn load_light_idx(litobjidx: usize, litid: u8) {
        unsafe { ffi::GX_LoadLightObjIdx(litobjidx as u32, litid) }
    }

    /// Causes the GPU to wait for the pipe to flush.
    ///
    /// This function inserts a synchronization command into the graphics FIFO. When the GPU sees
    /// this command it will allow the rest of the pipe to flush before continuing. This command is
    /// useful in certain situation such as after using [`Gx::copy_tex()`] and before a primitive
    /// that uses the copied texture.
    ///
    /// # Note
    /// The command is actually implemented by writing the control register that determines the
    /// format of the embedded frame buffer (EFB). As a result, care should be used if this command
    /// is placed within a display list.
    pub fn pix_mode_sync() {
        unsafe { ffi::GX_PixModeSync() }
    }

    /// Restores the write-gather pipe.
    ///
    /// The CPU fifo that was attached at the time [`Gx::redirect_write_gather_pipe()`] was called
    /// will be re-attached. If there is data pending in the write gather pipe (e.g. if the amount
    /// of data written was not a multiple of 32 bytes), the data will be padded with zeroes and
    /// flushed out.
    ///
    /// # Safety
    /// This function must be called between successive calls to [`Gx::redirect_write_gather_pipe()`].
    pub unsafe fn restore_write_gather_pipe() {
        ffi::GX_RestoreWriteGatherPipe()
    }

    /// Sends a DrawDone command to the GP.
    ///
    /// When all previous commands have been processed and the pipeline is empty, a *DrawDone*
    /// status bit will be set, and an interrupt will occur. You can receive notification of this
    /// event by installing a callback on the interrupt with [`Gx::set_draw_done_callback()`], or
    /// you can poll the status bit with [`Gx::wait_draw_done()`]. This function also flushes the
    /// write-gather FIFO in the CPU to make sure that all commands are sent to the graphics
    /// processor.
    ///
    /// # Note
    /// This function is normally used in multibuffer mode (see [`Gx::set_cpu_fifo()`]). In
    /// immediate mode, the [`Gx::draw_done()`] command can be used, which both sends the command
    /// and stalls until the *DrawDone* status is true.
    pub fn set_draw_done() {
        unsafe { ffi::GX_SetDrawDone() }
    }

    /// Inserts a synchronization command into the graphics FIFO. When the Graphics Processor sees
    /// this command, it will allow the texture pipeline to flush before continuing.
    ///
    /// This command is necessary when changing the usage of regions of texture memory from
    /// preloaded or TLUT to cached areas. It makes sure that the texture pipeline is finished with
    /// that area of the texture memory prior to changing its usage. This function should be called
    /// prior to drawing any primitives that uses the texture memory region in its new mode. It is
    /// not necessary to call this command when changing texture memory regions from cached to
    /// preloaded (or TLUT), since the commands to load the regions with data will cause the
    /// necessary synchronization to happen automatically.
    pub fn tex_mode_sync() {
        unsafe { ffi::GX_TexModeSync() }
    }

    /// Stalls until DrawDone is encountered by the GP.
    ///
    /// It means all graphics commands sent before this *DrawDone* command have executed and the
    /// last pixel has been written to the frame buffer. You may want to execute some non-graphics
    /// operations between executing [`Gx::set_draw_done()`] and this function, but if you simply
    /// want to wait and have nothing to execute, you can use [`Gx::draw_done()`].
    ///
    /// # Note
    /// This function is normally used in immediate mode (see [`Gx::set_cpu_fifo()`]). In
    /// multibuffer mode, sending the 'done' command is separated from polling the 'done' status
    /// (see [`Gx::set_draw_done()`] and [`Gx::wait_draw_done()`]).
    pub fn wait_draw_done() {
        unsafe { ffi::GX_WaitDrawDone() }
    }

    /// Sets whether Z buffering happens before or after texturing.
    ///
    /// Normally, Z buffering should happen before texturing, as this enables better performance by
    /// not texturing pixels that are not visible; however, when alpha compare is used, Z buffering
    /// must be done after texturing (see [`Gx::set_alpha_compare()`]).
    pub fn set_zcomp_loc(before_tex: bool) {
        unsafe { ffi::GX_SetZCompLoc(before_tex as u8) }
    }

    /// Sets color and Z value to clear the EFB to during copy operations.
    /// See [GX_SetCopyClear](https://libogc.devkitpro.org/gx_8h.html#a17265aefd7e64820de53abd9113334bc) for more.
    pub fn set_copy_clear(background: Color, z_value: u32) {
        BPReg::PE_CLEAR_AR.load(u32::from_be_bytes([
            0u8,
            0u8,
            background.0.a,
            background.0.r,
        ]));

        BPReg::PE_CLEAR_GB.load(u32::from_be_bytes([
            0u8,
            0u8,
            background.0.b,
            background.0.g,
        ]));

        BPReg::PE_CLEAR_Z.load(z_value);
        //unsafe { ffi::GX_SetCopyClear(background.0, z_value) };
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
        //unsafe { ffi::GX_SetDispCopySrc(left, top, wd, hd) }

        let mut top_left = 0u32;
        top_left.set_bits(..10, left.into());
        top_left.set_bits(10.., top.into());

        let mut width_height = 0u32;
        width_height.set_bits(..10, (wd - 1).into());
        width_height.set_bits(10.., (hd - 1).into());

        BPReg::EFB_ADDR_TOP_LEFT.load(top_left);
        BPReg::EFB_ADDR_DIMENSIONS.load(width_height);
    }

    /// Sets the witth and height of the display buffer in pixels.
    /// See [GX_SetDispCopyDst](https://libogc.devkitpro.org/gx_8h.html#ab6f639059b750e57af4c593ba92982c5) for more.
    pub fn set_disp_copy_dst(width: u16, _height: u16) {
        assert!(width <= 0x3FF, "width isn't a valid value");

        BPReg::MIPMAP_STRIDE.load(width.into());

        //unsafe { ffi::GX_SetDispCopyDst(width, height) }
    }

    /// Sets the subpixel sample patterns and vertical filter coefficients used to filter subpixels into pixels.
    /// See [GX_SetCopyFilter](https://libogc.devkitpro.org/gx_8h.html#afd65b7e5f2040ddb3352649efde72faf) for more.
    pub fn set_copy_filter(
        aa: bool,
        sample_pattern: &mut [[u8; 2]; 12],
        vf: bool,
        v_filter: &mut [u8; 7],
    ) {
        let mut disp_copy_0 = 0x666666u32;
        let mut disp_copy_1 = 0x666666u32;
        let mut disp_copy_2 = 0x666666u32;
        let mut disp_copy_3 = 0x666666u32;

        let mut trgt_copy_0 = 0x595000u32;
        let mut trgt_copy_1 = 0x000015u32;

        if aa {
            disp_copy_0.set_bits(0..4, sample_pattern[0][0].into());
            disp_copy_0.set_bits(4..8, sample_pattern[0][1].into());
            disp_copy_0.set_bits(8..12, sample_pattern[1][0].into());
            disp_copy_0.set_bits(12..16, sample_pattern[1][1].into());
            disp_copy_0.set_bits(16..20, sample_pattern[2][0].into());
            disp_copy_0.set_bits(20..24, sample_pattern[2][1].into());

            disp_copy_1.set_bits(0..4, sample_pattern[3][0].into());
            disp_copy_1.set_bits(4..8, sample_pattern[3][1].into());
            disp_copy_1.set_bits(8..12, sample_pattern[4][0].into());
            disp_copy_1.set_bits(12..16, sample_pattern[4][1].into());
            disp_copy_1.set_bits(16..20, sample_pattern[5][0].into());
            disp_copy_1.set_bits(20..24, sample_pattern[5][1].into());

            disp_copy_2.set_bits(0..4, sample_pattern[6][0].into());
            disp_copy_2.set_bits(4..8, sample_pattern[6][1].into());
            disp_copy_2.set_bits(8..12, sample_pattern[7][0].into());
            disp_copy_2.set_bits(12..16, sample_pattern[7][1].into());
            disp_copy_2.set_bits(16..20, sample_pattern[8][0].into());
            disp_copy_2.set_bits(20..24, sample_pattern[8][1].into());

            disp_copy_3.set_bits(0..4, sample_pattern[9][0].into());
            disp_copy_3.set_bits(4..8, sample_pattern[9][1].into());
            disp_copy_3.set_bits(8..12, sample_pattern[10][0].into());
            disp_copy_3.set_bits(12..16, sample_pattern[10][1].into());
            disp_copy_3.set_bits(16..20, sample_pattern[11][0].into());
            disp_copy_3.set_bits(20..24, sample_pattern[11][1].into());
        }

        if vf {
            trgt_copy_0.set_bits(0..6, v_filter[0].into());
            trgt_copy_0.set_bits(6..12, v_filter[1].into());
            trgt_copy_0.set_bits(12..18, v_filter[2].into());
            trgt_copy_0.set_bits(18..24, v_filter[3].into());

            trgt_copy_1.set_bits(0..6, v_filter[4].into());
            trgt_copy_1.set_bits(6..12, v_filter[5].into());
            trgt_copy_1.set_bits(12..18, v_filter[6].into());
        }

        BPReg::DISP_COPY_FILT0.load(disp_copy_0);
        BPReg::DISP_COPY_FILT1.load(disp_copy_1);
        BPReg::DISP_COPY_FILT2.load(disp_copy_2);
        BPReg::DISP_COPY_FILT3.load(disp_copy_3);

        BPReg::TRGT_COPY_FILT0.load(trgt_copy_0);
        BPReg::TRGT_COPY_FILT1.load(trgt_copy_1);
    }

    /// Sets the lighting controls for a particular color channel.
    pub fn set_channel_controls(
        channel: i32,
        enable: bool,
        ambsrc: Source,
        matsrc: Source,
        litmask: u8,
        diff_fn: DiffFn,
        attn_fn: AttnFn,
    ) {
        unsafe {
            ffi::GX_SetChanCtrl(
                channel,
                enable as u8,
                ambsrc as u8,
                matsrc as u8,
                litmask,
                diff_fn as u8,
                attn_fn as u8,
            );
        }
    }

    /// Controls various rasterization and texturing parameters that relate to field-mode and double-strike rendering.
    /// See [GX_SetFieldMode](https://libogc.devkitpro.org/gx_8h.html#a13f0df0011d04c3d986135e800fbcd21) for more.
    pub fn set_field_mode(field_mode: bool, half_aspect_ratio: bool) {
        unsafe { ffi::GX_SetFieldMode(field_mode as u8, half_aspect_ratio as u8) }
    }

    /// Sets the format of pixels in the Embedded Frame Buffer (EFB).
    /// See [GX_SetPixelFmt](https://libogc.devkitpro.org/gx_8h.html#a018d9b0359f9689ac41f44f0b2374ffb) for more.
    pub fn set_pixel_fmt(pix_fmt: PixelFormat, z_fmt: ZFormat) {
        let pe_ctrl = PixelEngineControl::new()
            .pixel_format(pix_fmt)
            .z_format(z_fmt);

        BPReg::PE_CTRL.load(pe_ctrl.to_u32());
    }

    /// Enables or disables culling of geometry based on its orientation to the viewer.
    ///
    /// Primitives in which the vertex order is clockwise to the viewer are considered front-facing.
    ///
    /// See [GX_SetCullMode](https://libogc.devkitpro.org/gx_8h.html#adb4b17c39b24073c3e961458ecf02e87) for more.
    pub fn set_cull_mode(mode: CullMode) {
        unsafe { ffi::GX_SetCullMode(mode as u8) }
    }

    /// Sets the current GX thread to the calling thread.
    ///
    /// The new thread should be the thread that will be responsible for
    /// generating graphics data. By default, the GX thread is the thread that
    /// invoked [`Gx::init()`]; however, it may be changed by calling this
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
    pub fn set_current_gx_thread() -> lwp::Thread {
        lwp::Thread::new(unsafe { ffi::GX_SetCurrentGXThread() })
    }

    /// Copies the embedded framebuffer (EFB) to the external framebuffer(XFB) in main memory.
    ///
    /// # Safety
    ///
    /// The user must ensure the dest ptr is a valid pointer,
    ///
    /// See [GX_CopyDisp](https://libogc.devkitpro.org/gx_8h.html#a9ed0ae3f900abb6af2e930dff7a6bc28) for more.
    pub unsafe fn copy_disp(dest: *mut c_void, clear: bool) {
        ffi::GX_CopyDisp(dest, clear as u8)
    }

    /// Sets the gamma correction applied to pixels during EFB to XFB copy operation.
    /// See [GX_SetDispCopyGamma](https://libogc.devkitpro.org/gx_8h.html#aa8e5bc962cc786b2049345fa698d4efa) for more.
    pub fn set_disp_copy_gamma(gamma: Gamma) {
        unsafe { ffi::GX_SetDispCopyGamma(gamma.0) }
    }

    /// Sets the attribute format (vtxattr) for a single attribute in the Vertex Attribute Table (VAT).
    /// See [GX_SetVtxAttrFmt](https://libogc.devkitpro.org/gx_8h.html#a87437061debcc0457b6b6dc2eb021f23) for more.
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
    ///
    /// It takes about 512 GP clocks to invalidate all the texture caches.
    ///
    /// # Note
    /// Preloaded textures (see [`Gx::preload_entire_texture()`]) are not affected.
    pub fn invalidate_tex_all() {
        unsafe { ffi::GX_InvalidateTexAll() }
    }

    /// Loads the state describing a texture into one of eight hardware register sets.
    ///
    /// Before this happens, the texture object *obj* should be initialized using
    /// [`Texture::new()`] or [`Texture::with_color_idx()`]. The *mapid* parameter refers to the
    /// texture map slot that is set, and takes a value between 0 and 7 inclusive. Once loaded, the
    /// texture can be used in any Texture Environment (TEV) stage using [`Gx::set_tev_order()`].
    ///
    /// # Note
    /// This function will call the functions set by [`Gx::set_tex_region_callback()`] (and
    /// [`Gx::set_tlut_region_callback()`] if the texture is color-index format) to obtain the
    /// texture regions associated with this texture object. These callbacks are set to default
    /// functions by [`Gx::init()`].
    ///
    /// # Safety
    /// If the texture is a color-index texture, you **must** load the associated TLUT (using
    /// [`Gx::load_tlut()`]) before calling this function.
    pub fn load_texture(obj: &Texture, mapid: u8) {
        unsafe { ffi::GX_LoadTexObj((&obj.0) as *const _ as *mut _, mapid) }
    }

    /// Sets the projection matrix.
    /// See [GX_LoadProjectionMtx](https://libogc.devkitpro.org/gx_8h.html#a241a1301f006ed04b7895c051959f64e) for more.
    pub fn load_projection_mtx(matrix: &Mtx44, projection: ProjectionType) {
        let mut values: [f32; 6] = [0.0; 6];
        values[0] = matrix[0][0];
        values[2] = matrix[1][1];
        values[4] = matrix[2][2];
        values[5] = matrix[2][3];

        match projection {
            ProjectionType::Perspective => {
                values[1] = matrix[0][2];
                values[3] = matrix[1][2];
            }
            ProjectionType::Orthographic => {
                values[1] = matrix[0][3];
                values[3] = matrix[1][3];
            }
        }

        let mut vals = values
            .iter()
            .map(|val| val.to_be_bytes())
            .collect::<Vec<[u8; 4]>>();
        vals.push((projection as u32).to_be_bytes());
        XFReg::PROJ_PRM_A.load_multi(7, &vals)
    }

    ///Sets global material color 1 or 0 in gx regs.
    pub fn set_global_mat_color(color_channel: ColorChannel, color: Color) {
        match color_channel {
            ColorChannel::Color0 => XFReg::MATERIAL0.load(u32::from_be_bytes([
                color.0.a, color.0.b, color.0.g, color.0.r,
            ])),
            ColorChannel::Color1 => XFReg::MATERIAL1.load(u32::from_be_bytes([
                color.0.a, color.0.b, color.0.g, color.0.r,
            ])),
        }
        Gx::color_color(color);
    }

    ///Sets global ambient color 1 or 0 in gx regs.
    pub fn set_global_ambient_color(color_channel: ColorChannel, color: Color) {
        match color_channel {
            ColorChannel::Color0 => XFReg::AMBIENT0.load(u32::from_be_bytes([
                color.0.a, color.0.b, color.0.g, color.0.r,
            ])),
            ColorChannel::Color1 => XFReg::AMBIENT1.load(u32::from_be_bytes([
                color.0.a, color.0.b, color.0.g, color.0.r,
            ])),
        }
        Gx::color_color(color);
    }

    /// Invalidates the vertex cache.
    ///
    /// Specifically, this functions invalidates the vertex cache tags. This function should be
    /// used whenever you relocate or modify data that is read by, or may be cached by, the vertex
    /// cache. The invalidation is very fast, taking only two Graphics Processor (GP) clock cycles
    /// to complete.
    ///
    /// # Note
    /// The vertex cache is used to cache indexed attribute data. Any attribute that is set to
    /// `GX_INDEX8` or `GX_INDEX16` in the current vertex descriptor (see [`Gx::set_vtx_desc()`])
    /// is indexed. Direct data bypasses the vertex cache. Direct data is any attribute that is set
    /// to `GX_DIRECT` in the current vertex descriptor.
    pub fn inv_vtx_cache() {
        GX_PIPE.write(GPCommand::InvalidateVertexCache as u8);
    }

    /// Clears all vertex attributes of the current vertex descriptor to `GX_NONE`.
    ///
    /// # Note
    /// The same functionality can be obtained using [`Gx::set_vtx_descv()`], however using
    /// [`Gx::clear_vtx_desc()`] is much more efficient.
    pub fn clear_vtx_desc() {
        unsafe { ffi::GX_ClearVtxDesc() }
    }

    /// Sets the type of a single attribute (attr) in the current vertex descriptor.
    /// See [GX_SetVtxDesc](https://libogc.devkitpro.org/gx_8h.html#af41b45011ae731ae5697b26b2bf97e2f) for more.
    pub fn set_vtx_desc(attr: VtxAttr, v_type: VtxDest) {
        unsafe { ffi::GX_SetVtxDesc(attr as u8, v_type.0) }
    }

    /// Used to load a 3x4 modelview matrix mt into matrix memory at location pnidx.
    /// See [GX_LoadPosMtxImm](https://libogc.devkitpro.org/gx_8h.html#a90349e713128a1fa4fd6048dcab7b5e7) for more.
    pub fn load_pos_mtx_imm(mt: &mut Mtx34, pnidx: u32) {
        unsafe { ffi::GX_LoadPosMtxImm(mt as *mut _, pnidx) }
    }

    pub fn load_nrm_mtx_imm(mt: &mut Mtx34, pnidx: u32) {
        unsafe { ffi::GX_LoadNrmMtxImm(mt as *mut _, pnidx) }
    }

    pub fn load_tex_mtx_imm(mt: &mut Mtx34, pnidx: u32) {
        unsafe { ffi::GX_LoadTexMtxImm(mt as *mut _, pnidx, ffi::GX_MTX3x4 as _) }
    }

    /// Enables or disables dithering.
    ///
    /// A 4x4 Bayer matrix is used for dithering.
    ///
    /// # Note
    /// Only valid when the pixel format (see GX_SetPixelFmt()) is either `GX_PF_RGBA6_Z24` or
    /// `GX_PF_RGB565_Z16`.
    ///
    /// Dithering should probably be turned off if you are planning on using the result of
    /// rendering for comparisons (e.g. outline rendering algorithm that writes IDs to the alpha
    /// channel, copies the alpha channel to a texture, and later compares the texture in the TEV).
    pub fn set_dither(dither: bool) {
        unsafe { ffi::GX_SetDither(dither as u8) }
    }

    /// Sends a DrawDone command to the GP and stalls until its subsequent execution.
    ///
    /// # Note
    /// This function is equivalent to calling [`Gx::set_draw_done()`] then
    /// [`Gx::wait_draw_done()`].
    pub fn draw_done() {
        //This should work :shrug:
        BPReg::PE_DONE.load(2);

        //THIS DIRECTLY OVERRUNS WPAR for a flush.
        for _ in 0..8 {
            for bytes in 0u32.to_be_bytes() {
                GX_PIPE.write(bytes);
            }
        }
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
    pub fn set_array<T>(attr: u32, array: &[T], stride: u8) {
        unsafe { ffi::GX_SetArray(attr, array.as_ptr() as *mut c_void, stride) }
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
        unsafe {
            ffi::GX_PokeARGB(x, y, color.0);
        }
    }

    #[inline]
    pub fn position_3f32(x: f32, y: f32, z: f32) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();
        let z_bytes = z.to_be_bytes();
        for byte in x_bytes {
            GX_PIPE.write(byte);
        }
        for byte in y_bytes {
            GX_PIPE.write(byte);
        }

        for byte in z_bytes {
            GX_PIPE.write(byte);
        }
        /*
        unsafe {
            ffi::GX_Position3f32(x, y, z);
        }
        */
    }

    #[inline]
    pub fn position_3u16(x: u16, y: u16, z: u16) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();
        let z_bytes = z.to_be_bytes();
        for byte in x_bytes {
            GX_PIPE.write(byte);
        }
        for byte in y_bytes {
            GX_PIPE.write(byte);
        }

        for byte in z_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_3i16(x: i16, y: i16, z: i16) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();
        let z_bytes = z.to_be_bytes();
        for byte in x_bytes {
            GX_PIPE.write(byte);
        }
        for byte in y_bytes {
            GX_PIPE.write(byte);
        }

        for byte in z_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_3u8(x: u8, y: u8, z: u8) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();
        let z_bytes = z.to_be_bytes();
        for byte in x_bytes {
            GX_PIPE.write(byte);
        }
        for byte in y_bytes {
            GX_PIPE.write(byte);
        }

        for byte in z_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_3i8(x: i8, y: i8, z: i8) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();
        let z_bytes = z.to_be_bytes();
        for byte in x_bytes {
            GX_PIPE.write(byte);
        }
        for byte in y_bytes {
            GX_PIPE.write(byte);
        }

        for byte in z_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_2f32(x: f32, y: f32) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();

        for byte in x_bytes {
            GX_PIPE.write(byte);
        }

        for byte in y_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_2u16(x: u16, y: u16) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();

        for byte in x_bytes {
            GX_PIPE.write(byte);
        }

        for byte in y_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_2i16(x: i16, y: i16) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();

        for byte in x_bytes {
            GX_PIPE.write(byte);
        }

        for byte in y_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_2u8(x: u8, y: u8) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();

        for byte in x_bytes {
            GX_PIPE.write(byte);
        }

        for byte in y_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position_2i8(x: i8, y: i8) {
        let x_bytes = x.to_be_bytes();
        let y_bytes = y.to_be_bytes();

        for byte in x_bytes {
            GX_PIPE.write(byte);
        }

        for byte in y_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position1x8(index: u8) {
        let idx_bytes = index.to_be_bytes();
        for byte in idx_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn position1x16(index: u16) {
        let idx_bytes = index.to_be_bytes();
        for byte in idx_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color_4u8(r: u8, g: u8, b: u8, a: u8) {
        let r_bytes = r.to_be_bytes();
        let g_bytes = g.to_be_bytes();
        let b_bytes = b.to_be_bytes();
        let a_bytes = a.to_be_bytes();

        for byte in r_bytes {
            GX_PIPE.write(byte);
        }
        for byte in g_bytes {
            GX_PIPE.write(byte);
        }
        for byte in b_bytes {
            GX_PIPE.write(byte);
        }
        for byte in a_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color_3u8(r: u8, g: u8, b: u8) {
        let r_bytes = r.to_be_bytes();
        let g_bytes = g.to_be_bytes();
        let b_bytes = b.to_be_bytes();

        for byte in r_bytes {
            GX_PIPE.write(byte);
        }
        for byte in g_bytes {
            GX_PIPE.write(byte);
        }
        for byte in b_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color_3f32(r: f32, g: f32, b: f32) {
        assert!((0.0..=1.0).contains(&r));
        assert!((0.0..=1.0).contains(&g));
        assert!((0.0..=1.0).contains(&b));

        let r: u8 = (r * 255.0).round() as u8;
        let g: u8 = (g * 255.0).round() as u8;
        let b: u8 = (b * 255.0).round() as u8;

        GX_PIPE.write(r);
        GX_PIPE.write(g);
        GX_PIPE.write(b);
    }

    #[inline]
    pub fn color_4f32(r: f32, g: f32, b: f32, a: f32) {
        assert!((0.0..=1.0).contains(&a));

        let a = (a * 255.0).round() as u8;

        Gx::color_3f32(r, g, b);
        GX_PIPE.write(a);
    }

    #[inline]
    pub fn color_1u32(clr: u32) {
        let clr_bytes = clr.to_be_bytes();
        for byte in clr_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color_1u16(clr: u16) {
        let clr_bytes = clr.to_be_bytes();
        for byte in clr_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color1x8(index: u8) {
        let idx_bytes = index.to_be_bytes();
        for byte in idx_bytes {
            GX_PIPE.write(byte);
        }
    }

    #[inline]
    pub fn color1x16(index: u16) {
        let idx_bytes = index.to_be_bytes();
        for byte in idx_bytes {
            GX_PIPE.write(byte);
        }
    }

    ///Helper functions to just pass in a color object
    pub fn color_color(clr: Color) {
        Gx::color_4u8(clr.0.r, clr.0.g, clr.0.b, clr.0.a);
    }

    #[inline]
    pub fn tex_coord_2f32(s: f32, t: f32) {
        let s_bytes = s.to_be_bytes();
        let t_bytes = t.to_be_bytes();

        for byte in s_bytes {
            GX_PIPE.write(byte);
        }

        for byte in t_bytes {
            GX_PIPE.write(byte);
        }
    }

    pub fn flush() {
        unsafe { ffi::GX_Flush() }
    }

    #[inline]
    pub fn end() {
        unsafe { ffi::GX_End() }
    }
}

//All the following data is found from
// http://hitmen.c02.at/files/yagcd/yagcd/chap5.html#sec5.3

//THIS IS PROBABLY NOT CORRECT IF SOMEONE COULD correct it for me that would be amazing!
fn call_display_list(display_list: &[u8]) {
    let ptr = display_list.as_ptr().map_addr(mem::to_physical);

    assert!(
        display_list.as_ptr().align_offset(32) == 0,
        "The display list is not correctly 32 byte aligned."
    );
    assert!(
        display_list.len() % 32 == 0,
        "The display list is not correctly padded to 32 bytes. Please pad with GPCommand::Nop"
    );

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

#[repr(u32)]
pub enum ColorChannel {
    Color0 = ffi::GX_COLOR0,
    Color1 = ffi::GX_COLOR1,
}
