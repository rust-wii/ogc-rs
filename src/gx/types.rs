use bit_field::BitField;
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

pub struct VtxDest(pub(crate) u8);

impl VtxDest {
    pub const NONE: Self = Self(0);
    pub const DIRECT: Self = Self(1);
    pub const INDEX8: Self = Self(2);
    pub const INDEX16: Self = Self(3);
}

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

pub struct ComponentType(u32);

impl ComponentType {
    pub const POSITION_XY: ComponentType = ComponentType(0);
    pub const POSITION_XYZ: ComponentType = ComponentType(1);
    pub const NORMAL_XYZ: ComponentType = ComponentType(0);
    pub const NORMAL_NBT: ComponentType = ComponentType(1);
    pub const NORMAL_NBT3: ComponentType = ComponentType(2);
    pub const COLOR_RGB8: ComponentType = ComponentType(0);
    pub const COLOR_RGBA: ComponentType = ComponentType(1);
    pub const TEXTURE_S: ComponentType = ComponentType(0);
    pub const TEXTURE_ST: ComponentType = ComponentType(1);

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

pub struct ComponentSize(u32);

impl ComponentSize {
    pub const U8: ComponentSize = ComponentSize(0);
    pub const I8: ComponentSize = ComponentSize(1);
    pub const U16: ComponentSize = ComponentSize(2);
    pub const I16: ComponentSize = ComponentSize(3);
    pub const F32: ComponentSize = ComponentSize(4);

    pub const COLOR_RGB565: ComponentSize = ComponentSize(0);
    pub const COLOR_RGB8: ComponentSize = ComponentSize(1);
    pub const COLOR_RGBX8: ComponentSize = ComponentSize(2);
    pub const COLOR_RGBA4: ComponentSize = ComponentSize(3);
    pub const COLOR_RGBA6: ComponentSize = ComponentSize(4);
    pub const COLOR_RGBA8: ComponentSize = ComponentSize(5);

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

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
