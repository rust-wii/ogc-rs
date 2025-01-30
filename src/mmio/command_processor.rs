#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use voladdress::{Safe, VolAddress};

pub use types::{
    AlignedPhysPtr, AlignedPhysPtrHigh, AlignedPhysPtrLow, BoundingBox, Clear, Control,
    PerformanceSelect, Status, Token,
};

const BASE: usize = 0xCC00_0000;

const STATUS_REGISTER: VolAddress<Status, Safe, Safe> = unsafe { VolAddress::new(BASE) };

const CONTROL_REGISTER: VolAddress<Control, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x2) };

const CLEAR_REGISTER: VolAddress<Clear, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x4) };

const PERFORMANCE_SELECT: VolAddress<PerformanceSelect, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x6) };

const TOKEN: VolAddress<Token, Safe, Safe> = unsafe { VolAddress::new(BASE + 0xE) };

const BOUNDING_BOX_LEFT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x10) };

const BOUNDING_BOX_RIGHT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x12) };

const BOUNDING_BOX_TOP: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x14) };

const BOUNDING_BOX_BOTTOM: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x16) };

const FIFO_BASE_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x20) };

const FIFO_BASE_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x22) };

const FIFO_END_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x24) };

const FIFO_END_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x26) };

const FIFO_HIGH_WATERMARK_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x28) };

const FIFO_HIGH_WATERMARK_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2A) };

const FIFO_LOW_WATERMARK_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2C) };

const FIFO_LOW_WATERMARK_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2E) };

const FIFO_READ_WRITE_DISTANCE_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x30) };

const FIFO_READ_WRITE_DISTANCE_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x32) };

const FIFO_WRITE_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x34) };

const FIFO_WRITE_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x36) };

const FIFO_READ_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x38) };

const FIFO_READ_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3A) };

const FIFO_BREAKPOINT_ADDRESS_LOW: VolAddress<AlignedPhysPtrLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3C) };

const FIFO_BREAKPOINT_ADDRESS_HIGH: VolAddress<AlignedPhysPtrHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3E) };

const TRANSFORM_RASTER_BUSY_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x40) };

const TRANSFORM_RASTER_BUSY_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x42) };

const TRANSFORM_CLOCK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x44) };

const TRANSFORM_CLOCK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x46) };

const TRANSFORM_WAIT_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x48) };

const TRANSFORM_WAIT_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4A) };

const TRANSFORM_WAIT_OUT_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4C) };

const TRANSFORM_WAIT_OUT_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4E) };

const VERTEX_CACHE_CHECK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x50) };

const VERTEX_CACHE_CHECK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x52) };

const VERTEX_CACHE_MISS_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x54) };

const VERTEX_CACHE_MISS_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x56) };

const VERTEX_CACHE_STALL_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x58) };

const VERTEX_CACHE_STALL_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x5A) };

const CLOCKS_PER_VERTEX_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x60) };

const CLOCKS_PER_VERTEX_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x62) };

const CLOCKS_PER_VERTEX_OUT_COUNT: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x64) };

/// Write `ptr` to the fifo base mmio registers.
pub unsafe fn write_fifo_base(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_BASE_ADDRESS_LOW.write(low);
    FIFO_BASE_ADDRESS_HIGH.write(high);
}

/// Read fifo base mmio registers, returning the physical pointer
pub unsafe fn read_fifo_base() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(FIFO_BASE_ADDRESS_LOW.read(), FIFO_BASE_ADDRESS_HIGH.read())
}

/// Write `ptr` to the fifo end mmio registers.
pub unsafe fn write_fifo_end(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_END_ADDRESS_LOW.write(low);
    FIFO_END_ADDRESS_HIGH.write(high);
}

/// Read fifo end mmio registers, returning the physical pointer.
pub unsafe fn read_fifo_end() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(FIFO_END_ADDRESS_LOW.read(), FIFO_END_ADDRESS_HIGH.read())
}

/// Write `ptr` to the fifo low watermark mmio registers.
pub unsafe fn write_fifo_low_watermark(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_LOW_WATERMARK_ADDRESS_LOW.write(low);
    FIFO_LOW_WATERMARK_ADDRESS_HIGH.write(high);
}

/// Read fifo low watermark mmio registers, returning the physical pointer
pub unsafe fn read_fifo_low_watermark() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(
        FIFO_LOW_WATERMARK_ADDRESS_LOW.read(),
        FIFO_LOW_WATERMARK_ADDRESS_HIGH.read(),
    )
}

/// Write `ptr` to the fifo high watermark mmio registers.
pub unsafe fn write_fifo_high_watermark(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_HIGH_WATERMARK_ADDRESS_LOW.write(low);
    FIFO_HIGH_WATERMARK_ADDRESS_HIGH.write(high);
}

/// Read fifo read watermark mmio registers, returning the physical pointer
pub unsafe fn read_fifo_high_watermark() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(
        FIFO_HIGH_WATERMARK_ADDRESS_LOW.read(),
        FIFO_HIGH_WATERMARK_ADDRESS_HIGH.read(),
    )
}

/// Write `ptr` to the fifo read pointer mmio registers.
pub unsafe fn write_fifo_read_ptr(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_READ_ADDRESS_LOW.write(low);
    FIFO_READ_ADDRESS_HIGH.write(high);
}

/// Read fifo read pointer mmio registers, returning the physical pointer
pub unsafe fn read_fifo_read_ptr() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(FIFO_READ_ADDRESS_LOW.read(), FIFO_READ_ADDRESS_HIGH.read())
}

/// Write `ptr` to the fifo write pointer mmio registers.
pub unsafe fn write_fifo_write_ptr(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_WRITE_ADDRESS_LOW.write(low);
    FIFO_WRITE_ADDRESS_HIGH.write(high);
}

/// Read fifo write pointer mmio registers, returning the physical pointer
pub unsafe fn read_fifo_write_ptr() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(
        FIFO_WRITE_ADDRESS_LOW.read(),
        FIFO_WRITE_ADDRESS_HIGH.read(),
    )
}

/// Write `ptr` to the fifo breakpoint pointer mmio registers.
pub unsafe fn write_fifo_breakpoint_ptr(ptr: AlignedPhysPtr<u8>) {
    let (low, high) = ptr.split();

    FIFO_BREAKPOINT_ADDRESS_LOW.write(low);
    FIFO_BREAKPOINT_ADDRESS_HIGH.write(high);
}

/// Read fifo breakpoint pointer mmio registers, returning the physical pointer
pub unsafe fn read_fifo_breakpoint_ptr() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::from_split(
        FIFO_BREAKPOINT_ADDRESS_LOW.read(),
        FIFO_BREAKPOINT_ADDRESS_HIGH.read(),
    )
}

pub(crate) mod types {
    use core::ptr::NonNull;

    use bit_field::BitField;

    use super::{
        BOUNDING_BOX_BOTTOM, BOUNDING_BOX_LEFT, BOUNDING_BOX_RIGHT, BOUNDING_BOX_TOP,
        CLEAR_REGISTER, CONTROL_REGISTER, PERFORMANCE_SELECT, STATUS_REGISTER, TOKEN,
    };

    /// Status Register
    ///
    /// Used to checkup on specifc parts of the fifo
    /// Cleared using the [`Clear`] register
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Status(u16);

    impl Status {
        /// Creates a empty [`Status`]
        pub const fn new() -> Self {
            Self(0)
        }

        /// Read [`Status`] from the associated mmio register
        pub fn read() -> Self {
            STATUS_REGISTER.read()
        }

        /// Write `self` to the associated mmio register
        pub fn write(self) {
            STATUS_REGISTER.write(self);
        }

        /// Check whether a fifo overflow occurred
        pub fn overflow(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify whether a fifo overflow occurred
        pub fn with_overflow(mut self, has_overflowed: bool) -> Self {
            self.0.set_bit(0, has_overflowed);
            self
        }

        /// Check whether a fifo underflow occurred
        pub fn underflow(self) -> bool {
            self.0.get_bit(1)
        }

        /// Modify whether a fifo underflow occurred
        pub fn with_underflow(mut self, has_underflowed: bool) -> Self {
            self.0.set_bit(1, has_underflowed);
            self
        }

        /// Check whether the Command Processor is done reading the fifo
        pub fn read_idle(self) -> bool {
            self.0.get_bit(2)
        }

        /// Modify whether the Command Processor is done reading the fifo
        pub fn with_read_idle(mut self, is_idle: bool) -> Self {
            self.0.set_bit(2, is_idle);
            self
        }

        /// Check whether the Command Processor is done writing to the fifo
        pub fn command_idle(self) -> bool {
            self.0.get_bit(3)
        }

        /// Modify whether the Command Processor is done writing to the fifo
        pub fn with_command_idle(mut self, is_idle: bool) -> Self {
            self.0.set_bit(3, is_idle);
            self
        }
        /// Check whether a fifo breakpoint has been hit
        pub fn breakpoint(self) -> bool {
            self.0.get_bit(4)
        }

        /// Modify whether a fifo breakpoint has been hit
        pub fn with_breakpoint(mut self, breakpoint_hit: bool) -> Self {
            self.0.set_bit(4, breakpoint_hit);
            self
        }
    }

    /// Control Register
    ///
    /// Uses to setup Command Processor and Processor Interface linking ,reading, and writing.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Control(u16);

    impl Control {
        /// Creates an empty [`Control`]
        pub const fn new() -> Self {
            Self(0)
        }

        /// Read the associated mmio register
        pub fn read() -> Self {
            CONTROL_REGISTER.read()
        }

        /// Writes `self` to the associated mmio register
        pub fn write(self) {
            CONTROL_REGISTER.write(self);
        }

        /// Checks if the Command Processor can read from the fifo
        pub fn read_enable(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify whether the Command Processor can read from the fifo
        pub fn with_read_enable(mut self, read_enable: bool) -> Self {
            self.0.set_bit(0, read_enable);
            self
        }

        /// Checks whether fifo breakpoints are enabled
        pub fn breakpoint_enable(self) -> bool {
            self.0.get_bit(1)
        }

        /// Modify the state of the breakpoints
        pub fn with_breakpoint_enable(mut self, breakpoint_enable: bool) -> Self {
            self.0.set_bit(1, breakpoint_enable);
            self
        }

        /// Checks whether the overflow interupt is enabled
        pub fn overflow_interrupt_enable(self) -> bool {
            self.0.get_bit(2)
        }

        /// Modify the state of the overflow interrupt
        pub fn with_overflow_interrupt_enable(mut self, overflow_interrupt_enable: bool) -> Self {
            self.0.set_bit(2, overflow_interrupt_enable);
            self
        }

        /// Check if the underflow interrupt is enabled
        pub fn underflow_interrupt_enable(self) -> bool {
            self.0.get_bit(3)
        }

        /// Modify the state of the underflow interrupt
        pub fn with_underflow_interrupt_enable(mut self, underflow_interrupt_enable: bool) -> Self {
            self.0.set_bit(3, underflow_interrupt_enable);
            self
        }

        /// Check whether the Command Processor and Processor Interface is linked
        pub fn link_enable(self) -> bool {
            self.0.get_bit(4)
        }

        /// Modify the state of Command Processor and Processor interface linked
        pub fn with_link_enable(mut self, link_enable: bool) -> Self {
            self.0.set_bit(4, link_enable);
            self
        }

        /// Checks whether the breakpoint interrupt is enabled
        pub fn breakpoint_interrupt_enable(self) -> bool {
            self.0.get_bit(5)
        }

        /// Modify the state of the breakpoint interrupt
        ///
        /// This really isn't used very much unless you want to debug the fifo
        pub fn with_breakpoint_interrupt_enable(
            mut self,
            breakpoint_interrupt_enable: bool,
        ) -> Self {
            self.0.set_bit(5, breakpoint_interrupt_enable);
            self
        }
    }

    /// Clear Register
    ///
    /// Used to clear the [`Status`] register
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone)]
    pub struct Clear(u16);

    impl Clear {
        /// Creates a new empty Clear register
        pub const fn new() -> Self {
            Self(0)
        }

        /// Reads from the associated mmio register
        pub fn read() -> Self {
            CLEAR_REGISTER.read()
        }

        /// Write `self` to the associated mmio register
        pub fn write(self) {
            CLEAR_REGISTER.write(self);
        }

        /// Returns a bool to check if the fifo overflow will get cleared
        pub fn clear_overflow(self) -> bool {
            self.0.get_bit(0)
        }

        /// Sets whether to clear a fifo overflow if it happened
        pub fn with_clear_overflow(mut self, clear_overflow: bool) -> Self {
            self.0.set_bit(0, clear_overflow);
            self
        }

        /// Returns a bool to check if the fifo underflow will get cleared
        pub fn clear_underflow(self) -> bool {
            self.0.get_bit(1)
        }

        /// Sets whether to clear the fifo underflow if it happened
        pub fn with_clear_underflow(mut self, clear_underflow: bool) -> Self {
            self.0.set_bit(1, clear_underflow);
            self
        }

        /// Returns a bool to see if the metrics are gonna get cleared
        pub fn clear_metrics(self) -> bool {
            self.0.get_bit(2)
        }
        /// Sets whether to clear the CP metrics and writes to `self`
        pub fn with_clear_metrics(mut self, clear_metrics: bool) -> Self {
            self.0.set_bit(2, clear_metrics);
            self
        }
    }

    /// Command Processor Performance Select
    ///
    /// WARNING: Here be dragons, alot of stuff is just completely undocumented.
    /// There is a small bit from `libogc` but otherwise everything else is either not finished or
    /// stubbed out.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PerformanceSelect(u16);

    impl PerformanceSelect {
        /// Returns a empty performance select
        pub const fn new() -> Self {
            Self(0)
        }

        /// Reads [`PerformanceSelect`] from the associated mmio register
        pub fn read() -> Self {
            PERFORMANCE_SELECT.read()
        }

        /// Write `self` to the associated mmio register
        pub fn write(self) {
            PERFORMANCE_SELECT.write(self);
        }

        //TODO: Swap `u16` with an enum
        /// The value of the performance select
        pub fn value(self) -> u16 {
            self.0
        }

        //TODO: Swap `u16` with an enum
        /// Write the performance select value to `self`
        pub fn with_value(mut self, value: u16) -> Self {
            debug_assert!(
                (0..=5).contains(&value),
                "value must be between 0 and 5 inclusive"
            );
            self.0 = value;
            self
        }
    }

    /// Token Register
    ///
    /// This can be written to using fifo commands.
    /// Usually written too after the end of draws and end of finishes
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Token(u16);

    impl Token {
        /// Returns a empty token register
        pub const fn new() -> Self {
            Self(0)
        }
        /// Returns the token value of the associated mmio register
        pub fn read() -> Self {
            TOKEN.read()
        }

        /// Write the provided token to the associated mmio register
        pub fn write(self) {
            TOKEN.write(self);
        }

        /// Grabs the token value out of the token
        pub fn value(self) -> u16 {
            self.0
        }

        /// Sets a value for the token
        pub fn with_value(mut self, value: u16) -> Self {
            self.0 = value;
            self
        }
    }

    /// Screen bounding box
    #[derive(Copy, Clone, Debug)]
    pub struct BoundingBox {
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    }

    impl BoundingBox {
        /// Returns a 0 sized bounding box
        pub const fn new() -> Self {
            Self {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            }
        }
        /// Returns the current bounding box from the associated mmio registers
        pub fn read() -> Self {
            Self {
                left: BOUNDING_BOX_LEFT.read(),
                right: BOUNDING_BOX_RIGHT.read(),
                top: BOUNDING_BOX_TOP.read(),
                bottom: BOUNDING_BOX_BOTTOM.read(),
            }
        }
    }

    /// 32 byte aligned physical space pointer
    ///
    /// This is used pretty frequently when handling CP fifos and display lists.
    pub struct AlignedPhysPtr<T: ?Sized>(NonNull<T>);

    /// The low half of a [`AlignedPhysPtr`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct AlignedPhysPtrLow(u16);

    /// The high half of a [`AlignedPhysPtr`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct AlignedPhysPtrHigh(u16);

    impl<T> AlignedPhysPtr<T> {
        /// Takes `ptr` and makes sure it is 32 byte aligned. It then converts `ptr` to physical
        /// address space
        pub fn new(ptr: *mut T) -> Option<Self> {
            let phys_ptr = match ptr.addr() {
                0x0000_0000..=0x017F_FFFF | 0x1000_0000..=0x13FF_FFFF => ptr,
                0x8000_0000..=0x817F_FFFF => ptr.map_addr(|addr| addr - 0x8000_0000),
                0x9000_0000..=0x93FF_FFFF => ptr.map_addr(|addr| addr - 0x9000_0000),
                0xC000_0000..=0xC17F_FFFF => ptr.map_addr(|addr| addr - 0xC000_0000),
                0xD000_0000..=0xD3FF_FFFF => ptr.map_addr(|addr| addr - 0xD000_0000),
                _ => return None,
            };

            if phys_ptr.align_offset(32) != 0 {
                None
            } else {
                NonNull::new(phys_ptr).map(|val| Self(val))
            }
        }
        /// Consumes a [`AlignedPhysPtr`] to gives out its halves
        pub fn split(self) -> (AlignedPhysPtrLow, AlignedPhysPtrHigh) {
            let addr = self.0.as_ptr().expose_provenance();

            let low = u16::try_from(addr.get_bits(0..=15)).unwrap();
            let high = u16::try_from(addr.get_bits(16..=31)).unwrap();

            (AlignedPhysPtrLow(low), AlignedPhysPtrHigh(high))
        }
        /// Takes `low` and `high` and reconstitutes it back to its original physical pointer
        ///
        /// Safety: Only values previously gotten from `AlignedPhysPtr::split` should be passed
        /// into this function
        pub unsafe fn from_split(low: AlignedPhysPtrLow, high: AlignedPhysPtrHigh) -> Self {
            let mut addr = 0usize;
            addr = *addr.set_bits(0..=15, low.0.into());
            addr = *addr.set_bits(16..=31, high.0.into());

            NonNull::new(core::ptr::with_exposed_provenance_mut(addr))
                .map(|val| AlignedPhysPtr(val))
                .expect("expected a ptr that was not null")
        }
    }
}
