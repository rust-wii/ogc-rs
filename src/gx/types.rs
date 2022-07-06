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
