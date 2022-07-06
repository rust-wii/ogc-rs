use super::ControllerPort;
use crate::ffi;
use alloc::boxed::Box;
use bitflags::bitflags;

pub struct WPad {
	id: ControllerPort,
}

bitflags! {
	pub struct WPadButton: u32 {
		const TWO = 0x0001;
		const ONE = 0x0002;
		const B = 0x0004;
		const A = 0x0008;
		const MINUS = 0x0010;
		const HOME = 0x0080;
		const LEFT = 0x0100;
		const RIGHT = 0x0200;
		const UP = 0x0400;
		const DOWN  = 0x0800;
		const PLUS = 0x1000;

		// Nunchunk
		const NUNCHUNK_Z = (0x0001<<16);
		const NUNCHUNK_C = (0x0002<<16);

		// Classic Controller
		const CLASSIC_UP = (0x0001u32<<16);
		const CLASSIC_LEFT = (0x0002u32<<16);
		const CLASSIC_ZR = (0x0004u32<<16);
		const CLASSIC_X = (0x0008u32<<16);
		const CLASSIC_A = (0x0010u32<<16);
		const CLASSIC_Y = (0x0020u32<<16);
		const CLASSIC_B = (0x0040u32<<16);
		const CLASSIC_ZL = (0x0080u32<<16);
		const CLASSIC_FULL_R = (0x0200u32<<16);
		const CLASSIC_PLUS = (0x0400u32<<16);
		const CLASSIC_HOME = (0x0800u32<<16);
		const CLASSIC_MINUS = (0x1000u32<<16);
		const CLASSIC_FULL_L = (0x2000u32<<16);
		const CLASSIC_DOWN = (0x4000u32<<16);
		const CLASIC_RIGHT = (0x8000u32<<16);

		// GH3 Controller????
		const GH3_STRUM_UP = (0x0001<<16);
		const GH3_YELLOW = (0x0008<<16);
		const GH3_GREEN = (0x0010<<16);
		const GH3_BLUE = (0x0020<<16);
		const GH3_RED = (0x0040<<16);
		const GH3_ORANGE = (0x0080<<16);
		const GH3_PLUS = (0x0400<<16);
		const GH3_MINUS = (0x1000<<16);
		const GH3_STRUM_DOWN = (0x4000<<16);
	}
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum WPadDataFormat {
	Buttons = ffi::WPAD_FMT_BTNS,
	ButtonsAccel = ffi::WPAD_FMT_BTNS_ACC,
	ButtonsAccelIR = ffi::WPAD_FMT_BTNS_ACC_IR,
}

impl WPad {
	pub fn new(id: ControllerPort) -> Self {
		WPad { id }
	}

	pub fn buttons_up(&self) -> WPadButton {
		unsafe {
			WPadButton::from_bits_truncate(ffi::WPAD_ButtonsUp(self.id as i32))
		}
	}

	pub fn is_button_up(&self, button: WPadButton) -> bool {
		let buttons = self.buttons_up();
		buttons.contains(button)
	}

	pub fn buttons_down(&self) -> WPadButton {
		unsafe {
			WPadButton::from_bits_truncate(ffi::WPAD_ButtonsDown(
				self.id as i32,
			))
		}
	}

	pub fn is_button_down(&self, button: WPadButton) -> bool {
		let buttons = self.buttons_down();
		buttons.contains(button)
	}

	pub fn buttons_held(&self) -> WPadButton {
		unsafe {
			WPadButton::from_bits_truncate(ffi::WPAD_ButtonsHeld(
				self.id as i32,
			))
		}
	}

	pub fn is_button_held(&self, button: WPadButton) -> bool {
		let buttons = self.buttons_held();
		buttons.contains(button)
	}

	pub fn raw(&self) -> Box<ffi::WPADData> {
		unsafe { Box::new(*ffi::WPAD_Data(self.id as i32)) }
	}

	pub fn ir(&self) -> (f32, f32) {
		let data = self.raw();
		(data.ir.x, data.ir.y)
	}

	pub fn gforce(self) -> (f32, f32, f32) {
		let data = self.raw();
		(data.gforce.x, data.gforce.y, data.gforce.z)
	}

	pub fn accel(&self) -> (u16, u16, u16) {
		let data = self.raw();
		(data.accel.x, data.accel.y, data.accel.z)
	}

	pub fn expansion(&self) -> ffi::expansion_t {
		let data = self.raw();
		data.exp
	}

	pub fn set_data_format(&self, data_format: WPadDataFormat) {
		unsafe { ffi::WPAD_SetDataFormat(self.id as i32, data_format as i32) };
	}

	pub fn set_motion_plus(&self, enable_motion_plus: bool) {
		unsafe {
			ffi::WPAD_SetMotionPlus(self.id as i32, enable_motion_plus as u8)
		};
	}

	pub fn init() {
		unsafe { ffi::WPAD_Init() };
	}

	pub fn update() {
		unsafe { ffi::WPAD_ScanPads() };
	}
}
