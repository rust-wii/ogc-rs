use alloc::boxed::Box;
use core::mem;

/// Represents the pad service. No controllers can be read until an instance of
/// this struct is created. This service can only be created once!
pub struct Pad;

/// The controller to be read for the `pad` service.
#[derive(Copy, Clone)]
pub enum Controller {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

impl PartialEq<Controller> for u16 {
    fn eq(&self, other: &Controller) -> bool {
        *self == *other as u16
    }
}

/// The button to be checked for the `pad` service.
#[derive(Copy, Clone)]
pub enum Button {
    Left = 1,
    Right = 2,
    Down = 4,
    Up = 8,
    Z = 16,
    R = 32,
    L = 64,
    A = 256,
    B = 512,
    X = 1024,
    Y = 2048,
    Start = 4096,
}

impl PartialEq<Button> for u16 {
    fn eq(&self, other: &Button) -> bool {
        *self == *other as u16
    }
}

impl Pad {
    /// Initialization of the audio service.
    pub fn init() -> Self {
        unsafe {
            ogc_sys::PAD_Init();
            Pad
        }
    }

    /// Scan all pads. Must be called every time before checking buttons.
    pub fn scan_pads(&self) -> u32 {
        unsafe {
            ogc_sys::PAD_ScanPads()
        }
    }

    /// Returns button that is pressed.
    pub fn buttons_down(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsDown(controller as i32)
        }
    }

    /// Returns button that is held.
    pub fn buttons_held(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsHeld(controller as i32)
        }
    }

    /// Returns button that is released.
    pub fn buttons_up(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsUp(controller as i32)
        }
    }

    /// Returns analog x-value of grey joystick.
    pub fn stick_x(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_StickX(controller as i32)
        }
    }

    /// Returns analog y-value of grey joystick.
    pub fn stick_y(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_StickY(controller as i32)
        }
    }

    /// Returns analog x-value of yellow joystick.
    pub fn sub_stick_x(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_SubStickX(controller as i32)
        }
    }

    /// Returns analog y-value of yellow joystick.
    pub fn sub_stick_y(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_SubStickY(controller as i32)
        }
    }

    /// Returns analog value of left trigger.
    pub fn trigger_l(&self, controller: Controller) -> u8 {
        unsafe {
            ogc_sys::PAD_TriggerL(controller as i32)
        }
    }

    /// Returns analog value of right trigger.
    pub fn trigger_r(&self, controller: Controller) -> u8 {
        unsafe {
            ogc_sys::PAD_TriggerR(controller as i32)
        }
    }

    /// Register a user callback function for the `pad` interface.
    pub fn set_sampling_callback<F>(&self, callback: Box<F>)
    where
        F: Fn(u32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn() = mem::transmute(ptr);
            ogc_sys::PAD_SetSamplingCallback(Some(code));
        }
    }

    fn clamp(&self) { unimplemented!() }
    fn control_motor(&self) { unimplemented!() }
    fn read(&self) { unimplemented!() }
    fn recalibrate(&self) { unimplemented!() }
    fn reset(&self) { unimplemented!() }
    fn set_spec(&self) { unimplemented!() }
    fn sync(&self) { unimplemented!() }
}
