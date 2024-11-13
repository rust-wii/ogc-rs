use voladdress::{Safe, Unsafe, VolAddress};

#[repr(transparent)]
pub struct InterruptCause(u32);
pub const INTERRUPT_CAUSE: VolAddress<InterruptCause, Safe, ()> =
    unsafe { VolAddress::new(0xCC00_3000) };

#[repr(transparent)]
pub struct InterruptMask(u32);
pub const INTERRUPT_MASK: VolAddress<InterruptMask, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_3004) };

pub const CPU_FIFO_START: VolAddress<*const u8, Safe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_300C) };
pub const CPU_FIFO_END: VolAddress<*const u8, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_3010) };
pub const CPU_FIFO_WRITE_PTR: VolAddress<*mut u8, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_3014) };
pub const RESET: VolAddress<u32, (), Unsafe> = unsafe { VolAddress::new(0xCC00_3024) };

#[repr(transparent)]
pub struct HardwareDescription(u32);
pub const HW_DESCRIPTION: VolAddress<HardwareDescription, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_302C) };
