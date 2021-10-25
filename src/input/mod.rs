
pub mod controller;
pub mod pad;
pub mod wpad;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum ControllerPort {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3
}

pub use wpad::*;
pub use pad::*;
pub use controller::*;
