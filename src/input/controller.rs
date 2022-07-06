use super::pad::PadButton;
use super::wpad::WPadButton;
use super::{ControllerPort, Pad, WPad};

pub enum ControllerType {
	Gamecube,
	Wii,
}

pub enum Button {
	Left,
	Right,
	Up,
	Down,
	TrigL,
	TrigR,
	TrigZ,
	TrigZL,
	TrigZR,
	A,
	B,
	C,
	X,
	Y,
	Z,
	One,
	Two,
	Minus,
	Plus,
	Home,
	Start,
}

impl From<Button> for WPadButton {
	fn from(button: Button) -> Self {
		match button {
			Button::Left => WPadButton::LEFT,
			Button::Right => WPadButton::RIGHT,
			Button::Up => WPadButton::UP,
			Button::Down => WPadButton::DOWN,
			Button::A => WPadButton::A,
			Button::B => WPadButton::B,
			Button::One => WPadButton::ONE,
			Button::Two => WPadButton::TWO,
			Button::Minus => WPadButton::MINUS,
			Button::Plus => WPadButton::PLUS,
			Button::Home => WPadButton::HOME,
			_ => WPadButton::HOME,
		}
	}
}

impl From<Button> for PadButton {
	fn from(button: Button) -> Self {
		match button {
			Button::Left => PadButton::LEFT,
			Button::Right => PadButton::RIGHT,
			Button::Down => PadButton::DOWN,
			Button::Up => PadButton::UP,
			Button::A => PadButton::A,
			Button::B => PadButton::B,
			Button::X => PadButton::X,
			Button::Y => PadButton::Y,
			Button::TrigZ => PadButton::TRIGGER_Z,
			Button::TrigR => PadButton::TRIGGER_R,
			Button::TrigL => PadButton::TRIGGER_L,
			Button::Start => PadButton::START,
			_ => PadButton::START,
		}
	}
}

pub struct Input {
	id: ControllerPort,
	ctrl_type: ControllerType,
}

impl Input {
	pub fn new(ctrl_type: ControllerType, id: ControllerPort) -> Self {
		Self { id, ctrl_type }
	}

	pub fn as_pad(&self) -> Pad {
		Pad::new(self.id)
	}

	pub fn as_wpad(&self) -> WPad {
		WPad::new(self.id)
	}

	pub fn is_button_up(&self, button: Button) -> bool {
		match self.ctrl_type {
			ControllerType::Gamecube => {
				let buttons = self.as_pad().buttons_up();
				buttons.contains(button.into())
			}
			ControllerType::Wii => {
				let buttons = self.as_wpad().buttons_up();
				buttons.contains(button.into())
			}
		}
	}

	pub fn is_button_down(&self, button: Button) -> bool {
		match self.ctrl_type {
			ControllerType::Gamecube => {
				let buttons = self.as_pad().buttons_down();
				buttons.contains(button.into())
			}
			ControllerType::Wii => {
				let buttons = self.as_wpad().buttons_down();
				buttons.contains(button.into())
			}
		}
	}

	pub fn is_button_held(&self, button: Button) -> bool {
		match self.ctrl_type {
			ControllerType::Gamecube => {
				let buttons = self.as_pad().buttons_held();
				buttons.contains(button.into())
			}
			ControllerType::Wii => {
				let buttons = self.as_wpad().buttons_held();
				buttons.contains(button.into())
			}
		}
	}

	pub fn init(ctrl_type: ControllerType) {
		match ctrl_type {
			ControllerType::Gamecube => {
				Pad::init();
			}
			ControllerType::Wii => {
				WPad::init();
			}
		}
	}

	pub fn update(ctrl_type: ControllerType) {
		match ctrl_type {
			ControllerType::Gamecube => Pad::update(),
			ControllerType::Wii => WPad::update(),
		}
	}
}
