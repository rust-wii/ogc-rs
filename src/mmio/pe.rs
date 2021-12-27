use voladdress::{VolAddress, Safe, Unsafe};

#[repr(transparent)]
pub struct ZConfigControl(u16);
pub const Z_CONFIG: VolAddress<ZConfigControl, Safe, Safe> = unsafe { VolAddress::new(0xCC00_1000) };

#[repr(transparent)]
pub struct AlphaConfigControl(u16); 
pub const ALPHA_CONFIG: VolAddress<AlphaConfigControl, Safe, Safe> = unsafe { VolAddress::new(0xCC00_1002) };

#[repr(transparent)]
pub struct DestAlpha(u16);
pub const DEST_ALPHA: VolAddress<DestAlpha, Safe, Safe> = unsafe { VolAddress::new(0xCC00_1004) };

#[repr(transparent)]
pub struct AlphaMode(u16);
pub const ALPHA_MODE: VolAddress<AlphaMode, Safe, Safe> = unsafe { VolAddress::new(0xCC00_1006) };

#[repr(transparent)]
pub struct AlphaRead(u16);
pub const ALPHA_READ: VolAddress<AlphaRead, Safe, Safe> = unsafe { VolAddress::new(0xCC00_1008) };

#[repr(transparent)]
pub struct InterruptStatusControl(u16);
pub const INTERRUPT_STATUS_REGISTER: VolAddress<InterruptStatusControl, Unsafe, Unsafe> = unsafe { VolAddress::new(0xCC00_100A) };

pub const PE_TOKEN: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_100E) };
