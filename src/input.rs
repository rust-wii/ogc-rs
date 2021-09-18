use alloc::boxed::Box;
use bitflags::bitflags;
use num_enum::IntoPrimitive;

#[derive(Copy, Clone)]
pub enum ControllerType {
    Gamecube,
    Wii,
}

#[derive(IntoPrimitive, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ControllerPort {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

#[derive(IntoPrimitive, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataFmt {
    Buttons = 0,
    ButtonsAccel = 1,
    ButtonsAccelIR = 2,
}

pub type Stick = (u32, u32);

#[derive(Clone)]
pub enum RawData {
    Gamecube,
    Wii(Option<ogc_sys::WPADData>),
    Unknown,
}

#[derive(Copy, Clone)]
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
    Menu,
    Home,
    Start,
}
#[derive(Copy, Clone)]
pub enum ButtonState {
    ButtonUp,
    ButtonDown,
    ButtonHeld,
}

pub struct Input {
    controller_type: ControllerType,
    controller_port: ControllerPort,
    buttons_up: u32,
    buttons_down: u32,
    buttons_held: u32,
    right_stick: Option<Stick>,
    left_stick: Option<Stick>,
    raw: RawData,
}

impl Input {
    pub fn init(ctrl_type: ControllerType) {
        match ctrl_type {
            ControllerType::Gamecube => Gamepad::init(),
            ControllerType::Wii => Wiipad::init(),
        }
    }

    pub fn new(ctrl_type: ControllerType, ctrl_port: ControllerPort) -> Self {
        Self {
            controller_type: ctrl_type,
            controller_port: ctrl_port,
            buttons_up: 0,
            buttons_down: 0,
            buttons_held: 0,
            right_stick: None,
            left_stick: None,
            raw: RawData::Unknown,
        }
    }

    pub fn update(&mut self) {
        match self.controller_type {
            ControllerType::Gamecube => Gamepad::update(),
            ControllerType::Wii => Wiipad::update(),
        }
    }

    pub fn buttons_up(&mut self) -> u32 {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.buttons_up = Gamepad::buttons_up(self.controller_port);
                self.buttons_up
            }
            ControllerType::Wii => {
                self.buttons_up = Wiipad::buttons_up(self.controller_port);
                self.buttons_up
            }
        }
    }
    pub fn buttons_down(&mut self) -> u32 {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.buttons_down = Gamepad::buttons_down(self.controller_port);
                self.buttons_down
            }
            ControllerType::Wii => {
                self.buttons_down = Wiipad::buttons_down(self.controller_port);
                self.buttons_down
            }
        }
    }

    pub fn buttons_held(&mut self) -> u32 {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.buttons_held = Gamepad::buttons_held(self.controller_port);
                self.buttons_held
            }
            ControllerType::Wii => {
                self.buttons_held = Wiipad::buttons_held(self.controller_port);
                self.buttons_held
            }
        }
    }

    pub fn is_button_in_state(&mut self, button: Button, button_state: ButtonState) -> bool {
        match self.controller_type {
            ControllerType::Gamecube => {
                Gamepad::is_button_in_state(self.controller_port, button, button_state)
            }
            ControllerType::Wii => {
                Wiipad::is_button_in_state(self.controller_port, button, button_state)
            }
        }
    }

    pub fn is_button_up(&mut self, button: Button) -> bool {
        match self.controller_type {
            ControllerType::Gamecube => {
                Gamepad::is_button_in_state(self.controller_port, button, ButtonState::ButtonUp)
            }
            ControllerType::Wii => {
                Wiipad::is_button_in_state(self.controller_port, button, ButtonState::ButtonUp)
            }
        }
    }

    pub fn is_button_down(&mut self, button: Button) -> bool {
        match self.controller_type {
            ControllerType::Gamecube => {
                Gamepad::is_button_in_state(self.controller_port, button, ButtonState::ButtonDown)
            }
            ControllerType::Wii => {
                Wiipad::is_button_in_state(self.controller_port, button, ButtonState::ButtonDown)
            }
        }
    }

    pub fn is_button_held(&mut self, button: Button) -> bool {
        match self.controller_type {
            ControllerType::Gamecube => {
                Gamepad::is_button_in_state(self.controller_port, button, ButtonState::ButtonHeld)
            }
            ControllerType::Wii => {
                Wiipad::is_button_in_state(self.controller_port, button, ButtonState::ButtonHeld)
            }
        }
    }

    pub fn get_trigger_data(&mut self, button: Button) -> f32 {
        match self.controller_type {
            ControllerType::Gamecube => {
                Gamepad::get_trigger_data(self.controller_port, button) as f32
            }
            ControllerType::Wii => Wiipad::get_trigger_data(self.controller_port, button),
        }
    }

    pub fn get_left_stick(&mut self) -> Option<Stick> {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.left_stick = Gamepad::get_left_stick(self.controller_port);
                self.left_stick
            }
            ControllerType::Wii => {
                self.right_stick = Wiipad::get_left_stick(self.controller_port);
                self.right_stick
            }
        }
    }

    pub fn get_right_stick(&mut self) -> Option<Stick> {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.right_stick = Gamepad::get_right_stick(self.controller_port);
                self.right_stick
            }
            ControllerType::Wii => {
                self.right_stick = Wiipad::get_right_stick(self.controller_port);
                self.right_stick
            }
        }
    }

    pub fn set_data_fmt(&self, format: DataFmt) {
        match self.controller_type {
            ControllerType::Wii => Wiipad::set_data_fmt(self.controller_port, format),
            ControllerType::Gamecube => (),
        }
    }

    pub fn get_raw(&mut self) -> RawData {
        match self.controller_type {
            ControllerType::Gamecube => {
                self.raw = RawData::Gamecube;
                self.raw.clone()
            }
            ControllerType::Wii => {
                self.raw = RawData::Wii(Wiipad::get_raw(self.controller_port));
                self.raw.clone()
            }
        }
    }

    pub fn ir(&mut self) -> (f32, f32) {
        match self.controller_type {
            ControllerType::Wii => {
                    let data = match self.get_raw() {
                        RawData::Wii(raw) => raw,
                        _ => None,
                    };
                    
                    if let Some(data) = data {
                        (data.ir.x, data.ir.y)
                    } else {
                        (0., 0.) 
                    }

                }
            _ => (0., 0.),
        }
    }   
}

bitflags! {
    pub struct GCNButton: u32 {
        const LEFT = 0x0001;
        const RIGHT = 0x0002;
        const DOWN = 0x0004;
        const UP = 0x0008;
        const TRIG_Z = 0x010;
        const TRIG_R = 0x020;
        const TRIG_L = 0x40;
        const A = 0x0100;
        const B = 0x0200;
        const X = 0x0400;
        const Y = 0x0800;
        const START = 0x1000;
        const MENU = 0x1000;
}
}

pub struct Gamepad;

impl Gamepad {
    pub fn init() {
        unsafe {
            ogc_sys::PAD_Init();
        }
    }

    pub fn update() {
        unsafe {
            ogc_sys::PAD_ScanPads();
        }
    }

    pub fn buttons_up(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::PAD_ButtonsUp(ctrl_port as i32) as u32 }
    }

    pub fn buttons_down(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::PAD_ButtonsDown(ctrl_port as i32) as u32 }
    }

    pub fn buttons_held(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::PAD_ButtonsHeld(ctrl_port as i32) as u32 }
    }

    ///This does not do triggers.
    ///Please use the `get_trigger_data` function
    pub fn is_button_in_state(
        ctrl_port: ControllerPort,
        button: Button,
        button_state: ButtonState,
    ) -> bool {
        let buttons = match button_state {
            ButtonState::ButtonUp => Gamepad::buttons_up(ctrl_port),
            ButtonState::ButtonDown => Gamepad::buttons_down(ctrl_port),
            ButtonState::ButtonHeld => Gamepad::buttons_held(ctrl_port),
        };
        match button {
            Button::Up => buttons & GCNButton::UP.bits() > 0,
            Button::Down => buttons & GCNButton::DOWN.bits() > 0,
            Button::Right => buttons & GCNButton::RIGHT.bits() > 0,
            Button::Left => buttons & GCNButton::LEFT.bits() > 0,
            Button::A => buttons & GCNButton::A.bits() > 0,
            Button::B => buttons & GCNButton::B.bits() > 0,
            Button::X => buttons & GCNButton::X.bits() > 0,
            Button::Y => buttons & GCNButton::Y.bits() > 0,
            Button::TrigZ => buttons & GCNButton::TRIG_Z.bits() > 0,
            Button::Menu => buttons & GCNButton::MENU.bits() > 0,
            Button::Start => buttons & GCNButton::START.bits() > 0,
            _ => false,
        }
    }

    pub fn get_trigger_data(ctrl_port: ControllerPort, button: Button) -> u32 {
        match button {
            Button::TrigL => unsafe { ogc_sys::PAD_TriggerL(ctrl_port as i32) as u32 },
            Button::TrigR => unsafe { ogc_sys::PAD_TriggerR(ctrl_port as i32) as u32 },
            _ => 0, //Nothing else is a trigger on this.
        }
    }

    pub fn get_left_stick(ctrl_port: ControllerPort) -> Option<Stick> {
        unsafe {
            let x = ogc_sys::PAD_StickX(ctrl_port as i32);
            let y = ogc_sys::PAD_StickY(ctrl_port as i32);
            Some((x as u32, y as u32))
        }
    }

    pub fn get_right_stick(ctrl_port: ControllerPort) -> Option<Stick> {
        unsafe {
            let x = ogc_sys::PAD_SubStickX(ctrl_port as i32);
            let y = ogc_sys::PAD_SubStickY(ctrl_port as i32);
            Some((x as u32, y as u32))
        }
    }
}

bitflags! {
    pub struct WIIButton: u32 {
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

        //Nunchunk
        const NUNCHUNK_Z = (0x0001<<16);
        const NUNCHUNK_C = (0x0002<<16);

        //Classic Controller
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

        //GH3 Controller????
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

pub struct Wiipad;

impl Wiipad {
    pub fn init() {
        unsafe {
            ogc_sys::WPAD_Init();
        }
    }

    pub fn update() {
        unsafe {
            ogc_sys::WPAD_ScanPads();
        }
    }

    pub fn buttons_up(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsUp(ctrl_port as i32) }
    }

    pub fn buttons_down(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsDown(ctrl_port as i32) }
    }

    pub fn buttons_held(ctrl_port: ControllerPort) -> u32 {
        unsafe { ogc_sys::WPAD_ButtonsHeld(ctrl_port as i32) }
    }

    ///THIS DOES NOT DO EXPANSIONS YET.  
    pub fn is_button_in_state(
        ctrl_port: ControllerPort,
        button: Button,
        button_state: ButtonState,
    ) -> bool {
        let buttons = match button_state {
            ButtonState::ButtonUp => Wiipad::buttons_up(ctrl_port),
            ButtonState::ButtonDown => Wiipad::buttons_down(ctrl_port),
            ButtonState::ButtonHeld => Wiipad::buttons_held(ctrl_port),
        };
        match button {
            Button::Up => buttons & WIIButton::UP.bits() > 0,
            Button::Down => buttons & WIIButton::DOWN.bits() > 0,
            Button::Left => buttons & WIIButton::LEFT.bits() > 0,
            Button::Right => buttons & WIIButton::RIGHT.bits() > 0,
            Button::A => buttons & WIIButton::A.bits() > 0,
            Button::B => buttons & WIIButton::B.bits() > 0,
            Button::One => buttons & WIIButton::ONE.bits() > 0,
            Button::Two => buttons & WIIButton::TWO.bits() > 0,
            Button::Minus => buttons & WIIButton::MINUS.bits() > 0,
            Button::Plus => buttons & WIIButton::PLUS.bits() > 0,
            Button::Home => buttons & WIIButton::HOME.bits() > 0,
            _ => false,
        }
    }

    pub fn get_trigger_data(ctrl_port: ControllerPort, button: Button) -> f32 {
        let raw = Wiipad::get_raw(ctrl_port);
        if let Some(raw) = raw { 
        let value = match raw.exp.type_ {
                2 => match button {
                    Button::TrigL => unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().l_shoulder },
                    Button::TrigR => unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().r_shoulder },
                    _ => 0.0,
                },
                _ => 0.0,
            };
        return value
        }
        return 0.;
    }

    pub fn get_left_stick(ctrl_port: ControllerPort) -> Option<Stick> {
        let raw = unsafe { *ogc_sys::WPAD_Data(ctrl_port as i32) };

        match raw.exp.type_ {
            1 => {
                let x = unsafe { raw.exp.__bindgen_anon_1.nunchuk.as_ref().js.pos.x };
                let y = unsafe { raw.exp.__bindgen_anon_1.nunchuk.as_ref().js.pos.y };

                Some((x.into(), y.into()))
            }
            2 => {
                let x = unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().ljs.pos.x };
                let y = unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().ljs.pos.y };

                Some((x.into(), y.into()))
            }
            _ => None,
        }
    }

    pub fn set_data_fmt(ctrl_port: ControllerPort, fmt: DataFmt) {
        unsafe {
            ogc_sys::WPAD_SetDataFormat(ctrl_port as i32, fmt as i32);
        }
    }

    pub fn get_right_stick(ctrl_port: ControllerPort) -> Option<Stick> {
        let raw = unsafe { *ogc_sys::WPAD_Data(ctrl_port as i32) };

        match raw.exp.type_ {
            2 => {
                let x = unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().rjs.pos.x };
                let y = unsafe { raw.exp.__bindgen_anon_1.classic.as_ref().rjs.pos.y };

                Some((x.into(), y.into()))
            }
            _ => None,
        }
    }

    pub fn get_raw(ctrl_port: ControllerPort) -> Option<ogc_sys::WPADData> {
        unsafe {
            let data = ogc_sys::WPAD_Data(ctrl_port as i32);
            if data.is_null() {
                None 
            } else {
                Some(*data)
            }

        }
    }
}
