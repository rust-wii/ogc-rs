use alloc::boxed::Box;
use core::{mem, ptr};

pub struct Pad;

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
    pub fn init() -> Self {
        unsafe {
            ogc_sys::PAD_Init();
            Pad
        }
    }

    pub fn scan_pads(&self) -> u32 {
        unsafe {
            ogc_sys::PAD_ScanPads()
        }
    }

    pub fn buttons_down(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsDown(controller as i32)
        }
    }

    pub fn buttons_held(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsHeld(controller as i32)
        }
    }

    pub fn buttons_up(&self, controller: Controller) -> u16 {
        unsafe {
            ogc_sys::PAD_ButtonsUp(controller as i32)
        }
    }

    pub fn stick_x(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_StickX(controller as i32)
        }
    }

    pub fn stick_y(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_StickY(controller as i32)
        }
    }

    pub fn sub_stick_x(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_SubStickX(controller as i32)
        }
    }

    pub fn sub_stick_y(&self, controller: Controller) -> i8 {
        unsafe {
            ogc_sys::PAD_SubStickY(controller as i32)
        }
    }

    pub fn trigger_l(&self, controller: Controller) -> u8 {
        unsafe {
            ogc_sys::PAD_TriggerL(controller as i32)
        }
    }

    pub fn trigger_r(&self, controller: Controller) -> u8 {
        unsafe {
            ogc_sys::PAD_TriggerR(controller as i32)
        }
    }

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

    // pub fn clamp(&self) { unimplemented!() }
    // pub fn control_motor(&self) { unimplemented!() }
    // pub fn read(&self) { unimplemented!() }
    // pub fn recalibrate(&self) { unimplemented!() }
    // pub fn reset(&self) { unimplemented!() }
    // pub fn set_spec(&self) { unimplemented!() }
    // pub fn sync(&self) { unimplemented!() }
}
