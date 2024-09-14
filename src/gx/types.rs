use bit_field::BitField;

use super::CmpFn;
pub struct PixelFormat(u8);

impl PixelFormat {
    pub const RGB8_Z24: Self = Self(0);
    pub const RGBA6_Z24: Self = Self(1);
    pub const RGB565_Z16: Self = Self(2);
    pub const Z24: Self = Self(3);
    pub const Y8: Self = Self(4);
    pub const U8: Self = Self(5);
    pub const V8: Self = Self(6);
    pub const YUV420: Self = Self(7);
}

pub struct ZFormat(u8);

impl ZFormat {
    pub const LINEAR: Self = Self(0);
    pub const NEAR: Self = Self(1);
    pub const MID: Self = Self(2);
    pub const FAR: Self = Self(3);
}

pub struct ZCompareLocation(bool);

impl ZCompareLocation {
    pub const AFTER_TEXTURE: Self = Self(false);
    pub const BEFORE_TEXTURE: Self = Self(true);
}

pub struct Gamma(pub(crate) u8);

impl Gamma {
    pub const ONE_ZERO: Self = Self(0);
    pub const ONE_SEVEN: Self = Self(1);
    pub const TWO_TWO: Self = Self(2);
}

/* #[derive(Copy, Clone, PartialEq)]
pub struct VtxDest(pub(crate) u8);



impl VtxDest {
    pub const NONE: Self = Self(0);
    pub const DIRECT: Self = Self(1);
    pub const INDEX8: Self = Self(2);
    pub const INDEX16: Self = Self(3);
}
 */
pub struct PixelEngineControl {
    pixel_format: PixelFormat,
    z_format: ZFormat,
    z_comp_loc: ZCompareLocation,
}

impl PixelEngineControl {
    pub fn new() -> Self {
        Self {
            pixel_format: PixelFormat::RGBA6_Z24,
            z_format: ZFormat::LINEAR,
            z_comp_loc: ZCompareLocation::BEFORE_TEXTURE,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut pe_ctrl = 0u32;

        pe_ctrl.set_bits(0..=2, self.pixel_format.0.into());
        pe_ctrl.set_bits(3..=5, self.z_format.0.into());
        pe_ctrl.set_bit(6, self.z_comp_loc.0);

        pe_ctrl
    }

    #[must_use]
    pub fn pixel_format(mut self, format: PixelFormat) -> Self {
        self.pixel_format = format;
        self
    }

    #[must_use]
    pub fn z_format(mut self, format: ZFormat) -> Self {
        self.z_format = format;
        self
    }

    #[must_use]
    pub fn z_comp_loc(mut self, z_comp_loc: ZCompareLocation) -> Self {
        self.z_comp_loc = z_comp_loc;
        self
    }
}

impl From<u32> for PixelEngineControl {
    fn from(pe_ctrl: u32) -> Self {
        let pix_fmt = pe_ctrl.get_bits(0..=2);
        let z_fmt = pe_ctrl.get_bits(3..=5);
        let z_comp_loc = pe_ctrl.get_bit(6);
        Self {
            pixel_format: PixelFormat(pix_fmt.try_into().unwrap()),
            z_format: ZFormat(z_fmt.try_into().unwrap()),
            z_comp_loc: ZCompareLocation(z_comp_loc),
        }
    }
}

impl Default for PixelEngineControl {
    fn default() -> Self {
        Self::new()
    }
}

pub enum TextureFormat {
    Intensity4 = 0,
    Intensity8 = 1,
    IntensityAlpha4 = 2,
    IntensityAlpha8 = 3,
    Rgb565 = 4,
    Rgb5a3 = 5,
    Rgba8 = 6,
    ColorIndexed4 = 8,
    ColorIndexed8 = 9,
    ColorIndexed14 = 10,
    Compressed = 14,
}

impl TextureFormat {
    pub const fn into_u8(self) -> u8 {
        match self {
            TextureFormat::Intensity4 => 0,
            TextureFormat::Intensity8 => 1,
            TextureFormat::IntensityAlpha4 => 2,
            TextureFormat::IntensityAlpha8 => 3,
            TextureFormat::Rgb565 => 4,
            TextureFormat::Rgb5a3 => 5,
            TextureFormat::Rgba8 => 6,
            TextureFormat::ColorIndexed4 => 8,
            TextureFormat::ColorIndexed8 => 9,
            TextureFormat::ColorIndexed14 => 10,
            TextureFormat::Compressed => 14,
        }
    }
}
//
// #[derive(PartialEq)]
// pub struct ComponentType(u32);
//
// impl ComponentType {
//     pub const POSITION_XY: ComponentType = ComponentType(0);
//     pub const POSITION_XYZ: ComponentType = ComponentType(1);
//     pub const NORMAL_XYZ: ComponentType = ComponentType(0);
//     pub const NORMAL_NBT: ComponentType = ComponentType(1);
//     pub const NORMAL_NBT3: ComponentType = ComponentType(2);
//     pub const COLOR_RGB8: ComponentType = ComponentType(0);
//     pub const COLOR_RGBA: ComponentType = ComponentType(1);
//     pub const TEXTURE_S: ComponentType = ComponentType(0);
//     pub const TEXTURE_ST: ComponentType = ComponentType(1);
//
//     pub const fn into_u32(self) -> u32 {
//         self.0
//     }
// }

// #[derive(PartialEq)]
// pub struct ComponentSize(u32);
//
// impl ComponentSize {
//     pub const U8: ComponentSize = ComponentSize(0);
//     pub const I8: ComponentSize = ComponentSize(1);
//     pub const U16: ComponentSize = ComponentSize(2);
//     pub const I16: ComponentSize = ComponentSize(3);
//     pub const F32: ComponentSize = ComponentSize(4);
//
//     pub const COLOR_RGB565: ComponentSize = ComponentSize(0);
//     pub const COLOR_RGB8: ComponentSize = ComponentSize(1);
//     pub const COLOR_RGBX8: ComponentSize = ComponentSize(2);
//     pub const COLOR_RGBA4: ComponentSize = ComponentSize(3);
//     pub const COLOR_RGBA6: ComponentSize = ComponentSize(4);
//     pub const COLOR_RGBA8: ComponentSize = ComponentSize(5);
//
//     pub const fn into_u32(self) -> u32 {
//         self.0
//     }
// }

pub type TexCoordSlot = Slot;
pub type TexMapSlot = Slot;

pub enum Slot {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    None = 0xFF,
}

impl Slot {
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::None => 0xFF,
        }
    }
}

pub enum ColorSlot {
    Color0 = 0,
    Color1 = 1,
    Alpha0 = 2,
    Alpha1 = 3,
    Color0Alpha0 = 4,
    Color1Alpha1 = 5,
    Zero = 6,
    BumpAlpha = 7,
    BumpNormalAlpha = 8,
    None = 0xFF,
}

impl ColorSlot {
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Color0 => 0,
            Self::Color1 => 1,
            Self::Alpha0 => 2,
            Self::Alpha1 => 3,
            Self::Color0Alpha0 => 4,
            Self::Color1Alpha1 => 5,
            Self::Zero => 6,
            Self::BumpAlpha => 7,
            Self::BumpNormalAlpha => 8,
            Self::None => 0xFF,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextureEnviroment(u32);

pub enum ColorCombinerInput {
    PreviousColor = 0,
    PreviousAlpha = 1,
    Color0 = 2,
    Alpha0 = 3,
    Color1 = 4,
    Alpha1 = 5,
    Color2 = 6,
    Alpha2 = 7,
    TextureColor = 8,
    TextureAlpha = 9,
    RasterColor = 10,
    RasterAlpha = 11,
    One = 12,
    Half = 13,
    Konst = 14,
    Zero = 15,
}

pub enum AlphaCombinerInput {
    Previous = 0,
    Alpha0 = 1,
    Alpha1 = 2,
    Alpha2 = 3,
    TextureAlpha = 4,
    RasterAlpha = 5,
    Konst = 6,
    Zero = 7,
}
impl AlphaCombinerInput {
    pub const fn into_u8(self) -> u8 {
        match self {
            AlphaCombinerInput::Previous => 0,
            AlphaCombinerInput::Alpha0 => 1,
            AlphaCombinerInput::Alpha1 => 2,
            AlphaCombinerInput::Alpha2 => 3,
            AlphaCombinerInput::TextureAlpha => 4,
            AlphaCombinerInput::RasterAlpha => 5,
            AlphaCombinerInput::Konst => 6,
            AlphaCombinerInput::Zero => 7,
        }
    }
}

impl ColorCombinerInput {
    pub const fn into_u8(self) -> u8 {
        match self {
            ColorCombinerInput::PreviousColor => 0,
            ColorCombinerInput::PreviousAlpha => 1,
            ColorCombinerInput::Color0 => 2,
            ColorCombinerInput::Alpha0 => 3,
            ColorCombinerInput::Color1 => 4,
            ColorCombinerInput::Alpha1 => 5,
            ColorCombinerInput::Color2 => 6,
            ColorCombinerInput::Alpha2 => 7,
            ColorCombinerInput::TextureColor => 8,
            ColorCombinerInput::TextureAlpha => 9,
            ColorCombinerInput::RasterColor => 10,
            ColorCombinerInput::RasterAlpha => 11,
            ColorCombinerInput::One => 12,
            ColorCombinerInput::Half => 13,
            ColorCombinerInput::Konst => 14,
            ColorCombinerInput::Zero => 15,
        }
    }
}

pub enum TextureEnviromentBias {
    Zero = 0,
    AddHalf = 1,
    SubHalf = 2,
}

impl TextureEnviromentBias {
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::AddHalf => 1,
            Self::SubHalf => 2,
        }
    }
}

pub enum TextureEnviromentScale {
    One = 0,
    Two = 1,
    Four = 2,
    Half = 3,
}

impl TextureEnviromentScale {
    pub const fn into_u8(self) -> u8 {
        match self {
            TextureEnviromentScale::One => 0,
            TextureEnviromentScale::Two => 1,
            TextureEnviromentScale::Four => 2,
            TextureEnviromentScale::Half => 3,
        }
    }
}

pub enum TextureEnviromentClamp {
    Linear = 0,
    GreaterEqual = 1,
    Equal = 2,
    LessEqual = 3,
}

impl TextureEnviromentClamp {
    pub const fn into_u8(self) -> u8 {
        match self {
            TextureEnviromentClamp::Linear => 0,
            TextureEnviromentClamp::GreaterEqual => 1,
            TextureEnviromentClamp::Equal => 2,
            TextureEnviromentClamp::LessEqual => 3,
        }
    }
}

pub enum ColorReg {
    Previous = 0,
    Zero = 1,
    One = 2,
    Two = 3,
}

impl ColorReg {
    pub fn into_u8(self) -> u8 {
        match self {
            ColorReg::Previous => 0,
            ColorReg::Zero => 1,
            ColorReg::One => 2,
            ColorReg::Two => 3,
        }
    }
}

pub enum Operation {
    Add = 0,
    Sub = 1,
    CompareRed8GreaterThan = 8,
    CompareRed8Equal = 9,
    CompareRedGreen16GreaterThan = 10,
    CompareRedGreen16Equal = 11,
    CompareBlueGreenRed24GreaterThan = 12,
    CompareBlueGreenRed24Equal = 13,
    CompareRGB8GreaterThan = 14,
    CompareRGB8Equal = 15,
}

impl Operation {
    pub fn into_u8(self) -> u8 {
        match self {
            Operation::Add => 0,
            Operation::Sub => 1,
            Operation::CompareRed8GreaterThan => 8,
            Operation::CompareRed8Equal => 9,
            Operation::CompareRedGreen16GreaterThan => 10,
            Operation::CompareRedGreen16Equal => 11,
            Operation::CompareBlueGreenRed24GreaterThan => 12,
            Operation::CompareBlueGreenRed24Equal => 13,
            Operation::CompareRGB8GreaterThan => 14,
            Operation::CompareRGB8Equal => 15,
        }
    }
}

impl Default for TextureEnviroment {
    fn default() -> Self {
        Self::new()
    }
}
impl TextureEnviroment {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn with_op(mut self, op: Operation) -> Self {
        self.0 = bitfrob::u32_with_value(18, 18, self.0, op.into_u8().into());
        self
    }

    pub fn with_output_register(mut self, output: ColorReg) -> Self {
        self.0 = bitfrob::u32_with_value(22, 23, self.0, output.into_u8().into());
        self
    }

    pub fn with_scale(mut self, scale: TextureEnviromentScale) -> Self {
        self.0 = bitfrob::u32_with_value(20, 21, self.0, scale.into_u8().into());
        self
    }

    pub fn with_clamp(mut self, clamp: TextureEnviromentClamp) -> Self {
        self.0 = bitfrob::u32_with_value(19, 19, self.0, clamp.into_u8().into());
        self
    }

    pub fn with_bias(mut self, bias: TextureEnviromentBias) -> Self {
        self.0 = bitfrob::u32_with_value(16, 17, self.0, bias.into_u8().into());
        self
    }

    pub fn with_a(mut self, a: ColorCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(12, 15, self.0, a.into_u8().into());
        self
    }

    pub fn with_b(mut self, b: ColorCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(8, 11, self.0, b.into_u8().into());
        self
    }

    pub fn with_c(mut self, c: ColorCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(4, 7, self.0, c.into_u8().into());
        self
    }

    pub fn with_d(mut self, d: ColorCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(0, 3, self.0, d.into_u8().into());
        self
    }

    pub fn with_alpha_d(mut self, d: AlphaCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(4, 6, self.0, d.into_u8().into());
        self
    }

    pub fn with_alpha_c(mut self, c: AlphaCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(7, 9, self.0, c.into_u8().into());
        self
    }

    pub fn with_alpha_b(mut self, b: AlphaCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(10, 12, self.0, b.into_u8().into());
        self
    }

    pub fn with_alpha_a(mut self, a: AlphaCombinerInput) -> Self {
        self.0 = bitfrob::u32_with_value(13, 15, self.0, a.into_u8().into());
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub enum TevOp {
    PassColor,
    Replace,
    Blend,
    Modulate,
    Decal,
}

pub struct ZMode(u32);

impl Default for ZMode {
    fn default() -> Self {
        Self::new()
    }
}
impl ZMode {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_func(mut self, func: CmpFn) -> Self {
        self.0 = bitfrob::u32_with_value(1, 3, self.0, func.into_u8().into());
        self
    }

    pub fn with_enable(mut self, enable: bool) -> Self {
        self.0 = bitfrob::u32_with_value(0, 0, self.0, u8::from(enable).into());
        self
    }

    pub fn with_update(mut self, update_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_value(4, 4, self.0, u8::from(update_enable).into());
        self
    }

    pub fn into_u32(self) -> u32 {
        self.0
    }
}

pub enum LogicOp {
    Clear = 0,
    And = 1,
    ReverseAnd = 2,
    Copy = 3,
    InverseAnd = 4,
    NoOperation = 5,
    ExclusiveOr = 6,
    Or = 7,
    NotOr = 8,
    Equivalent = 9,
    Inverse = 10,
    ReverseOr = 11,
    InverseCopy = 12,
    InverseOr = 13,
    NotAnd = 14,
    Set = 15,
}

impl LogicOp {
    pub fn into_u8(self) -> u8 {
        match self {
            LogicOp::Clear => 0,
            LogicOp::And => 1,
            LogicOp::ReverseAnd => 2,
            LogicOp::Copy => 3,
            LogicOp::InverseAnd => 4,
            LogicOp::NoOperation => 5,
            LogicOp::ExclusiveOr => 6,
            LogicOp::Or => 7,
            LogicOp::NotOr => 8,
            LogicOp::Equivalent => 9,
            LogicOp::Inverse => 10,
            LogicOp::ReverseOr => 11,
            LogicOp::InverseCopy => 12,
            LogicOp::InverseOr => 13,
            LogicOp::NotAnd => 14,
            LogicOp::Set => 15,
        }
    }
}

pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    InverseSourceColor = 3,
    SourceAlpha = 4,
    InverseSourceAlpha = 5,
    DestinationAlpha = 6,
    InverseDestinationAlpha = 7,
}

impl BlendFactor {
    pub fn into_u8(self) -> u8 {
        match self {
            BlendFactor::Zero => 0,
            BlendFactor::One => 1,
            BlendFactor::SourceColor => 2,
            BlendFactor::InverseSourceColor => 3,
            BlendFactor::SourceAlpha => 4,
            BlendFactor::InverseSourceAlpha => 5,
            BlendFactor::DestinationAlpha => 6,
            BlendFactor::InverseDestinationAlpha => 7,
        }
    }
}

pub struct CMode0(u32);

impl Default for CMode0 {
    fn default() -> Self {
        Self::new()
    }
}

impl CMode0 {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_blend_enable(mut self, blend_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(0, self.0, blend_enable);
        self
    }

    pub fn with_logic_enable(mut self, logic_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(1, self.0, logic_enable);
        self
    }

    pub fn with_dither_enable(mut self, dither_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(2, self.0, dither_enable);
        self
    }

    pub fn with_color_update(mut self, color_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(3, self.0, color_enable);
        self
    }

    pub fn with_alpha_update(mut self, alpha_enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(4, self.0, alpha_enable);
        self
    }

    pub fn with_source_factor(mut self, factor: BlendFactor) -> Self {
        self.0 = bitfrob::u32_with_value(5, 7, self.0, factor.into_u8().into());
        self
    }

    pub fn with_destination_factor(mut self, factor: BlendFactor) -> Self {
        self.0 = bitfrob::u32_with_value(8, 10, self.0, factor.into_u8().into());
        self
    }

    pub fn should_blend(mut self, enable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(11, self.0, enable);
        self
    }

    pub fn with_logic_op(mut self, logic_op: LogicOp) -> Self {
        self.0 = bitfrob::u32_with_value(12, 23, self.0, logic_op.into_u8().into());
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub enum TextureOffset {
    Zero = 0,
    Sixteenth = 1,
    Eighth = 2,
    Fourth = 3,
    Half = 4,
    One = 5,
}

impl TextureOffset {
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Sixteenth => 1,
            Self::Eighth => 2,
            Self::Fourth => 3,
            Self::Half => 4,
            Self::One => 5,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct LinePointSize(u32);

impl Default for LinePointSize {
    fn default() -> Self {
        Self::new()
    }
}
impl LinePointSize {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_line_size(mut self, line_size: u8) -> Self {
        self.0 = bitfrob::u32_with_value(0, 7, self.0, line_size.into());
        self
    }

    pub fn with_point_size(mut self, point_size: u8) -> Self {
        self.0 = bitfrob::u32_with_value(8, 15, self.0, point_size.into());
        self
    }

    pub fn with_line_offset(mut self, offset: TextureOffset) -> Self {
        self.0 = bitfrob::u32_with_value(16, 18, self.0, offset.into_u8().into());
        self
    }

    pub fn with_point_offset(mut self, offset: TextureOffset) -> Self {
        self.0 = bitfrob::u32_with_value(19, 21, self.0, offset.into_u8().into());
        self
    }

    pub fn with_half_aspect_ratio(mut self, has_half_aspect_ratio: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(22, self.0, has_half_aspect_ratio);
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct MatrixIndexLow(u32);

impl Default for MatrixIndexLow {
    fn default() -> Self {
        Self::new()
    }
}
impl MatrixIndexLow {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_geometry_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(0, 5, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_0_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(6, 11, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_1_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(12, 17, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_2_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(18, 23, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_3_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(24, 29, self.0, matrix_index.into());
        self
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct MatrixIndexHigh(u32);

impl Default for MatrixIndexHigh {
    fn default() -> Self {
        Self::new()
    }
}
impl MatrixIndexHigh {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_texture_4_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(0, 5, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_5_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(6, 11, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_6_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(12, 17, self.0, matrix_index.into());
        self
    }

    pub fn with_texture_7_matrix_index(mut self, matrix_index: u8) -> Self {
        debug_assert!(matrix_index <= 63);
        self.0 = bitfrob::u32_with_value(18, 23, self.0, matrix_index.into());
        self
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct ClipMode(u32);

impl Default for ClipMode {
    fn default() -> Self {
        Self::new()
    }
}
impl ClipMode {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_disable(mut self, disable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(0, self.0, disable);
        self
    }

    pub fn with_trivial_rejection_disable(mut self, disable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(1, self.0, disable);
        self
    }

    pub fn with_clipping_acceleration_disable(mut self, disable: bool) -> Self {
        self.0 = bitfrob::u32_with_bit(1, self.0, disable);
        self
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct ScissorTopLeft(u32);

impl Default for ScissorTopLeft {
    fn default() -> Self {
        Self::new()
    }
}
impl ScissorTopLeft {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_y_origin(mut self, y_origin: u32) -> Self {
        self.0 = bitfrob::u32_with_value(0, 10, self.0, y_origin);
        self
    }

    pub fn with_x_origin(mut self, x_origin: u32) -> Self {
        self.0 = bitfrob::u32_with_value(12, 22, self.0, x_origin);
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

pub struct ScissorHeightWidth(u32);

impl Default for ScissorHeightWidth {
    fn default() -> Self {
        Self::new()
    }
}

impl ScissorHeightWidth {
    pub const fn new() -> Self {
        Self(0)
    }
    pub fn with_height(mut self, height: u32) -> Self {
        self.0 = bitfrob::u32_with_value(0, 10, self.0, height);
        self
    }
    pub fn with_width(mut self, width: u32) -> Self {
        self.0 = bitfrob::u32_with_value(12, 22, self.0, width);
        self
    }
    pub const fn into_u32(self) -> u32 {
        self.0
    }
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

pub struct ScissorBoxOffset(u32);

impl Default for ScissorBoxOffset {
    fn default() -> Self {
        Self::new()
    }
}

impl ScissorBoxOffset {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_x_offset(mut self, x_off: u32) -> Self {
        self.0 = bitfrob::u32_with_value(0, 9, self.0, x_off);
        self
    }
    pub fn with_y_offset(mut self, y_off: u32) -> Self {
        self.0 = bitfrob::u32_with_value(10, 19, self.0, y_off);
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

pub struct DisplayTopLeft(u32);

impl Default for DisplayTopLeft {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayTopLeft {
    pub const fn new() -> Self {
        Self(0)
    }
    pub fn with_x_origin(mut self, x_origin: u16) -> Self {
        self.0 = bitfrob::u32_with_value(0, 9, self.0, x_origin.into());
        self
    }
    pub fn with_y_origin(mut self, y_origin: u16) -> Self {
        self.0 = bitfrob::u32_with_value(10, 19, self.0, y_origin.into());
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

pub struct DisplayWidthHeight(u32);

impl Default for DisplayWidthHeight {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayWidthHeight {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_width(mut self, width: u16) -> Self {
        self.0 = bitfrob::u32_with_value(0, 9, self.0, width.into());
        self
    }
    pub fn with_height(mut self, height: u16) -> Self {
        self.0 = bitfrob::u32_with_value(10, 19, self.0, height.into());
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

pub struct DisplayStride(u32);

impl Default for DisplayStride {
    fn default() -> Self {
        Self::new()
    }
}
impl DisplayStride {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_stride(mut self, stride: u16) -> Self {
        self.0 = bitfrob::u32_with_value(0, 9, self.0, stride.into());
        self
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct DisplayYScale(u32);

impl Default for DisplayYScale {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayYScale {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn with_scale(mut self, y_scale: f32) -> Self {
        let y_scale_u32 = u32::from_be_bytes(y_scale.to_be_bytes());

        self.0 = bitfrob::u32_with_value(0, 8, self.0, y_scale_u32);
        self
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct DisplayFilter(u32);

impl Default for DisplayFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayFilter {
    pub const fn new() -> Self {
        Self(0)
    }

    /// # Safety
    ///  A proper bit pattern for this type is provided
    pub const unsafe fn from_u32(val: u32) -> Self {
        Self(val)
    }

    pub fn with_pattern_0(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(0, 3, self.0, pattern.into());
        self
    }

    pub fn with_pattern_1(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(4, 7, self.0, pattern.into());
        self
    }

    pub fn with_pattern_2(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(8, 11, self.0, pattern.into());
        self
    }

    pub fn with_pattern_3(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(12, 15, self.0, pattern.into());
        self
    }

    pub fn with_pattern_4(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(16, 19, self.0, pattern.into());
        self
    }

    pub fn with_pattern_5(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(20, 23, self.0, pattern.into());
        self
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct CopyFilter(u32);

impl Default for CopyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl CopyFilter {
    pub const fn new() -> Self {
        Self(0)
    }
    /// # Safety
    /// A proper bit pattern for this type is provided
    pub const unsafe fn from_u32(val: u32) -> Self {
        Self(val)
    }

    pub fn with_pattern_0(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(0, 5, self.0, pattern.into());
        self
    }

    pub fn with_pattern_1(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(6, 11, self.0, pattern.into());
        self
    }

    pub fn with_pattern_2(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(12, 17, self.0, pattern.into());
        self
    }

    pub fn with_pattern_3(mut self, pattern: u8) -> Self {
        self.0 = bitfrob::u32_with_value(18, 23, self.0, pattern.into());
        self
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub type VtxDest = VertexDestination;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VertexDestination {
    None,
    Direct,
    Indexed8bit,
    Indexed16bit,
}

pub enum Error {
    FromU32Error(u32),
}

impl core::fmt::Debug for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl VertexDestination {
    pub const NONE: Self = Self::None;
    pub const DIRECT: Self = Self::Direct;
    pub const INDEX8: Self = Self::Indexed8bit;
    pub const INDEX16: Self = Self::Indexed16bit;

    pub const fn into_u32(self) -> u32 {
        match self {
            VertexDestination::None => 0,
            VertexDestination::Direct => 1,
            VertexDestination::Indexed8bit => 2,
            VertexDestination::Indexed16bit => 3,
        }
    }

    pub const fn try_from_u32(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Direct),
            2 => Ok(Self::Indexed16bit),
            3 => Ok(Self::Indexed8bit),
            val => Err(Error::FromU32Error(val)),
        }
    }
}

impl From<VertexDestination> for u32 {
    fn from(value: VertexDestination) -> Self {
        value.into_u32()
    }
}

impl TryFrom<u32> for VertexDestination {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentType {
    PositionXy,
    PositionXyz,
    NormalXyz,
    NormalNbt,
    NormalNbt3,
    TexcoordS,
    TexcoordSt,
    ColorRgb,
    ColorRgba,
}

impl ComponentType {
    pub const POSITION_XY: Self = Self::PositionXy;
    pub const POSITION_XYZ: Self = Self::PositionXyz;
    pub const NORMAL_XYZ: Self = Self::NormalXyz;
    pub const NORMAL_NBT: Self = Self::NormalNbt;
    pub const NORMAL_NBT3: Self = Self::NormalNbt3;
    pub const COLOR_RGB8: Self = Self::ColorRgb;
    pub const COLOR_RGBA: Self = Self::ColorRgba;
    pub const TEXTURE_S: Self = Self::TexcoordS;
    pub const TEXTURE_ST: Self = Self::TexcoordSt;

    pub const fn into_u32(self) -> u32 {
        match self {
            ComponentType::PositionXy | Self::NormalXyz | Self::TexcoordS | Self::ColorRgb => 0,
            ComponentType::PositionXyz | Self::NormalNbt | Self::TexcoordSt | Self::ColorRgba => 1,
            Self::NormalNbt3 => 2,
        }
    }
}

impl From<ComponentType> for u32 {
    fn from(value: ComponentType) -> Self {
        value.into_u32()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentSize {
    U8,
    S8,
    U16,
    S16,
    F32,
    Rgb565,
    Rgb888,
    Rgb888X,
    Rgba4,
    Rgba6,
    Rgba8,
}

impl ComponentSize {
    pub const U8: Self = Self::U8;
    pub const I8: Self = Self::S8;
    pub const U16: Self = Self::U16;
    pub const I16: Self = Self::S16;
    pub const F32: Self = Self::F32;

    pub const COLOR_RGB565: Self = Self::Rgb565;
    pub const COLOR_RGB8: Self = Self::Rgb888;
    pub const COLOR_RGBX8: Self = Self::Rgb888X;
    pub const COLOR_RGBA4: Self = Self::Rgba4;
    pub const COLOR_RGBA6: Self = Self::Rgba6;
    pub const COLOR_RGBA8: Self = Self::Rgba8;

    pub const fn into_u32(self) -> u32 {
        match self {
            Self::U8 | Self::Rgb565 => 0,
            Self::S8 | Self::Rgb888 => 1,
            Self::U16 | Self::Rgb888X => 2,
            Self::S16 | Self::Rgba4 => 3,
            Self::F32 | Self::Rgba6 => 4,
            Self::Rgba8 => 5,
        }
    }
}

impl From<ComponentSize> for u32 {
    fn from(value: ComponentSize) -> Self {
        value.into_u32()
    }
}
