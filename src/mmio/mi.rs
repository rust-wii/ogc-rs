use voladdress::{Safe, Unsafe, VolAddress, VolSeries};

#[repr(transparent)]
pub struct PageAddress(u32);
pub const PROTECTED_REGION_ZERO: VolAddress<PageAddress, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_4000) };
pub const PROTECTED_REGION_ONE: VolAddress<PageAddress, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_4004) };
pub const PROTECTED_REGION_TWO: VolAddress<PageAddress, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_4008) };
pub const PROTECTED_REGION_THREE: VolAddress<PageAddress, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_400C) };

#[repr(transparent)]
pub struct ProtectionType(u16);
pub const PROJECTION_TYPE: VolAddress<ProtectionType, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_4010) };

#[repr(transparent)]
pub struct MemoryInterruptMask(u16);
pub const MI_INTERRUPT_MASK: VolAddress<MemoryInterruptMask, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_401C) };

#[repr(transparent)]
pub struct MemoryInterruptCause(u16);
pub const MI_INTERRUPT_CAUSE: VolAddress<MemoryInterruptCause, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_401E) };

#[repr(transparent)]
pub struct MemUnknown(u16);
pub const MEM_UNKNOWN: VolAddress<MemUnknown, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_4020) };

#[repr(transparent)]
pub struct MemAddrLo(u16);
pub const MEM_ADDR_LO: VolAddress<MemAddrLo, Safe, ()> =
	unsafe { VolAddress::new(0xCC00_4022) };

#[repr(transparent)]
pub struct MemAddrHi(u16);
pub const MEM_ADDR_HI: VolAddress<MemAddrHi, Safe, ()> =
	unsafe { VolAddress::new(0xCC00_4024) };

pub const TIMER_HI: VolSeries<u16, Safe, (), 10, 2> =
	unsafe { VolSeries::new(0xCC00_4032) };
pub const TIMER_LO: VolSeries<u16, Safe, (), 10, 2> =
	unsafe { VolSeries::new(0xCC00_4034) };
