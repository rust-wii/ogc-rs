use voladdress::{VolAddress, Safe, Unsafe};

/// TODO: ACTUALLY CHECK WHATS SAFE AND WHATS UNSAFE INSTEAD OF ASSUMING SAFE LOL

#[repr(transparent)]
pub struct StatusRegisterControl(u16);
pub const STATUS_REGISTER: VolAddress<StatusRegisterControl, Unsafe, Unsafe> = unsafe { VolAddress::new(0xCC00_0000) };

#[repr(transparent)]
pub struct ControlRegisterControl(u16);
pub const CONTROL_REGISTER: VolAddress<ControlRegisterControl, Unsafe, Unsafe> = unsafe { VolAddress::new(0xCC00_0002) };

#[repr(transparent)]
pub struct ClearRegisterControl(u16);
pub const CLEAR_REGISTER: VolAddress<ClearRegisterControl, (), Unsafe> = unsafe { VolAddress::new(0xCC00_0004) };

pub const TOKEN_REGISTER: VolAddress<u16, Unsafe, Unsafe> = unsafe { VolAddress::new(0xCC00_000E) };

#[repr(transparent)]
pub struct BoundingBoxWidthBound(u16);
pub const BBOX_LEFT: VolAddress<BoundingBoxWidthBound, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0010) };
pub const BBOX_RIGHT: VolAddress<BoundingBoxWidthBound, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0012) };

#[repr(transparent)]
pub struct BoundingBoxHeightBound(u16);
pub const BBOX_TOP: VolAddress<BoundingBoxHeightBound, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0014) };
pub const BBOX_BOTTOM: VolAddress<BoundingBoxHeightBound, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0016) };

pub const FIFO_BASE_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0020) };
pub const FIFO_BASE_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0022) };

pub const FIFO_END_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0024) };
pub const FIFO_END_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0026) };

pub const FIFO_HIGH_WATERMARK_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0028) };
pub const FIFO_HIGH_WATERMARK_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_002a) };

pub const FIFO_LOW_WATERMARK_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_002c) };
pub const FIFO_LOW_WATERMARK_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_002e) };

pub const FIFO_RW_DISTANCE_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0030) };
pub const FIFO_RW_DISTANCE_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0032) };

pub const FIFO_WRITE_PTR_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0034) };
pub const FIFO_WRITE_PTR_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0036) };

pub const FIFO_READ_PTR_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_0038) };
pub const FIFO_READ_PTR_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_003a) };

pub const FIFO_BREAKPOINT_PTR_LO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_003c) };
pub const FIFO_BREAKPOINT_PTR_HI: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_003e) };
