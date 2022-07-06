use super::ControllerPort;
use crate::ffi;
use bitflags::bitflags;

pub struct Pad {
	id: ControllerPort,
}

bitflags! {
	pub struct PadButton: u16 {
	   const LEFT = 0x0001;
	   const RIGHT = 0x0002;
	   const DOWN = 0x0004;
	   const UP = 0x0008;
	   const TRIGGER_Z = 0x0010;
	   const TRIGGER_R = 0x0020;
	   const TRIGGER_L = 0x0040;
	   const A = 0x0100;
	   const B = 0x0200;
	   const X = 0x0400;
	   const Y = 0x0800;
	   const START = 0x1000;
	}
}

impl Pad {
	pub fn new(id: ControllerPort) -> Self {
		Pad { id }
	}

	pub fn buttons_up(&self) -> PadButton {
		unsafe {
			PadButton::from_bits_truncate(ffi::PAD_ButtonsUp(self.id as i32))
		}
	}

	pub fn is_button_up(&self, button: PadButton) -> bool {
		let buttons = self.buttons_up();
		buttons.contains(button)
	}

	pub fn buttons_down(&self) -> PadButton {
		unsafe {
			PadButton::from_bits_truncate(ffi::PAD_ButtonsDown(self.id as i32))
		}
	}

	pub fn is_button_down(&self, button: PadButton) -> bool {
		let buttons = self.buttons_down();
		buttons.contains(button)
	}

	pub fn buttons_held(&self) -> PadButton {
		unsafe {
			PadButton::from_bits_truncate(ffi::PAD_ButtonsHeld(self.id as i32))
		}
	}

	pub fn is_button_held(&self, button: PadButton) -> bool {
		let buttons = self.buttons_held();
		buttons.contains(button)
	}

	pub fn stick_x(&self) -> i8 {
		unsafe { ffi::PAD_StickY(self.id as i32) }
	}

	pub fn stick_y(&self) -> i8 {
		unsafe { ffi::PAD_StickY(self.id as i32) }
	}

	pub fn c_stick_x(&self) -> i8 {
		unsafe { ffi::PAD_SubStickX(self.id as i32) }
	}

	pub fn c_stick_y(&self) -> i8 {
		unsafe { ffi::PAD_SubStickY(self.id as i32) }
	}

	pub fn trigger_l(&self) -> u8 {
		unsafe { ffi::PAD_TriggerL(self.id as i32) }
	}

	pub fn trigger_r(&self) -> u8 {
		unsafe { ffi::PAD_TriggerR(self.id as i32) }
	}

	pub fn init() {
		unsafe { ffi::PAD_Init() };
	}

	pub fn update() {
		unsafe { ffi::PAD_ScanPads() };
	}
}
