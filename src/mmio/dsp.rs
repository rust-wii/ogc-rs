use voladdress::{Safe, Unsafe, VolAddress};

#[repr(transparent)]
pub struct MailBoxVal(u16);

pub const DSP_MAILBOX_HI: VolAddress<MailBoxVal, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_5000) };
pub const DSP_MAILBOX_LO: VolAddress<MailBoxVal, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_5002) };
pub const CPU_MAILBOX_HI: VolAddress<MailBoxVal, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_5004) };
pub const CPU_MAILBOX_LO: VolAddress<MailBoxVal, Safe, Safe> =
	unsafe { VolAddress::new(0xCC00_5006) };

#[repr(transparent)]
pub struct DSPControlStatus(u16);
pub const DSP_CONTROL_STATUS_REGISTER: VolAddress<
	DSPControlStatus,
	Unsafe,
	Safe,
> = unsafe { VolAddress::new(0xCC00_500a) };

pub const AR_SIZE: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5012) };
pub const AR_MODE: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5016) };
pub const AR_REFRESH: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_501A) };
pub const AR_MRAM_ADDR_HI: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5020) };
pub const AR_MRAM_ADDR_LO: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5022) };
pub const AR_ARAM_ADDR_HI: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5024) };
pub const AR_ARAM_ADDR_LO: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5026) };

#[repr(transparent)]
pub struct DmaCountHi(u16);
pub const AR_DMA_COUNT_HI: VolAddress<DmaCountHi, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_5028) };
pub const AR_DMA_COUNT_LO: VolAddress<u16, Unsafe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_502A) };

pub const AR_START_ADDR_HI: VolAddress<u16, Unsafe, Safe> =
	unsafe { VolAddress::new(0xCC00_5030) };
pub const AR_START_ADDR_LO: VolAddress<u16, Unsafe, Safe> =
	unsafe { VolAddress::new(0xCC00_5032) };

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DmaControl(u16);
pub const DMA_CONTROL: VolAddress<DmaControl, Unsafe, Safe> =
	unsafe { VolAddress::new(0xCC00_5036) };

pub const DMA_BYTES_LEFT: VolAddress<u16, Safe, Unsafe> =
	unsafe { VolAddress::new(0xCC00_503A) };
