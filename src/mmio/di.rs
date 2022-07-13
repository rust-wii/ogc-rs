use voladdress::{Safe, VolAddress};

#[repr(transparent)]
pub struct DiStatusControl(u32);
pub const DI_STATUS_REGISTER: VolAddress<DiStatusControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_6000) };

#[repr(transparent)]
pub struct DiCoverControl(u32);
pub const DI_COVER_REGISTER: VolAddress<DiCoverControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_6004) };

#[repr(transparent)]
pub struct DiCommandBufControl(u32);
pub const DI_COMMAND_BUFFER_ZERO: VolAddress<DiCommandBufControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_6008) };
pub const DI_COMMAND_BUFFER_ONE: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_600C) };
pub const DI_COMMAND_BUFFER_TWO: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_6010) };
