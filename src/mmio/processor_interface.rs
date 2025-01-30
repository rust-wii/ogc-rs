#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use voladdress::{Safe, VolAddress};

// TODO: Move `AlignedPhysPtr` to utils
use super::command_processor::AlignedPhysPtr;

pub use types::{InterruptCause, InterruptMask};

const BASE: usize = 0xCC00_3000;

const INTERRUPT_CAUSE: VolAddress<InterruptCause, Safe, Safe> = unsafe { VolAddress::new(BASE) };

const INTERRUPT_MASK: VolAddress<InterruptMask, Safe, Safe> = unsafe { VolAddress::new(BASE + 4) };

const FIFO_BASE_ADDRESS: VolAddress<*mut u8, Safe, Safe> = unsafe { VolAddress::new(BASE + 0xC) };

const FIFO_END_ADDRESS: VolAddress<*mut u8, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x10) };

const FIFO_WRITE_ADDRESS: VolAddress<*mut u8, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x14) };

/// Write `ptr` to the fifo base mmio register.
pub unsafe fn write_fifo_base(ptr: AlignedPhysPtr<u8>) {
    FIFO_BASE_ADDRESS.write(ptr.into_ptr());
}

/// Read fifo base mmio register, returning the physical pointer
pub unsafe fn read_fifo_base() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_BASE_ADDRESS.read()).unwrap()
}

/// Write `ptr` to the fifo end mmio register.
pub unsafe fn write_fifo_end(ptr: AlignedPhysPtr<u8>) {
    FIFO_END_ADDRESS.write(ptr.into_ptr());
}

/// Read fifo end mmio register, returning the physical pointer
pub unsafe fn read_fifo_end() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_END_ADDRESS.read()).unwrap()
}

/// Write `ptr` to the fifo write ptr mmio register.
pub unsafe fn write_fifo_write_ptr(ptr: AlignedPhysPtr<u8>) {
    FIFO_WRITE_ADDRESS.write(ptr.into_ptr());
}

/// Read fifo write ptr mmio register, returning the physical pointer
pub unsafe fn read_fifo_write_ptr() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_WRITE_ADDRESS.read()).unwrap()
}

mod types {
    use bit_field::BitField;

    use super::{INTERRUPT_CAUSE, INTERRUPT_MASK};

    /// Interrupt Cause
    ///
    /// Shows all causes of external interrupts
    #[derive(Copy, Clone, Debug)]
    #[repr(transparent)]
    pub struct InterruptCause(u32);

    impl InterruptCause {
        /// Create an empty [`InterruptCause`]
        pub const fn new() -> Self {
            Self(0)
        }

        /// Read [`InterruptCause`] from the associated mmio register
        pub fn read() -> Self {
            INTERRUPT_CAUSE.read()
        }

        /// Write `self` to the associated mmio register
        pub fn write(self) {
            INTERRUPT_CAUSE.write(self);
        }

        /// Check whether a `Processor Interface` interrupt occurred
        pub fn processor_interface(self) -> bool {
            self.0.get_bit(0)
        }

        /// Check whether a reset switch interrupt occurred
        pub fn reset_switch(self) -> bool {
            self.0.get_bit(1)
        }

        /// Check whether a `Disk Interface` interrupt occurred
        pub fn disk_interface(self) -> bool {
            self.0.get_bit(2)
        }

        /// Check whether a `Serial Interface` interrupt occurred
        pub fn serial_interface(self) -> bool {
            self.0.get_bit(3)
        }

        /// Check whether a `External Interface` interrupt occurred
        pub fn external_interface(self) -> bool {
            self.0.get_bit(4)
        }

        /// Check whether a `Audio Interface` interrupt occurred
        pub fn audio_interface(self) -> bool {
            self.0.get_bit(5)
        }

        /// Check whether a `Digital Signal Processor` interrupt occurred
        pub fn digital_signal_processor(self) -> bool {
            self.0.get_bit(6)
        }

        /// Check whether a `Memory Interface` interrupt occurred
        pub fn memory_interface(self) -> bool {
            self.0.get_bit(7)
        }

        /// Check whether a `Video Interface` interrupt occurred
        pub fn video_interface(self) -> bool {
            self.0.get_bit(8)
        }

        /// Check whether a `Pixel Engine Token` interrupt occurred
        pub fn pixel_engine_token(self) -> bool {
            self.0.get_bit(9)
        }

        /// Check whether a `Pixel Engine Finish` interrupt occurred
        pub fn pixel_engine_finish(self) -> bool {
            self.0.get_bit(10)
        }

        /// Check whether a `Command Processor` interrupt occurred
        pub fn command_processor(self) -> bool {
            self.0.get_bit(11)
        }

        /// Check whether a `External Debugger` interrupt occurred
        pub fn external_debugger(self) -> bool {
            self.0.get_bit(12)
        }

        /// Check whether a `High Speed Port` interrupt occurred
        pub fn high_speed_port(self) -> bool {
            self.0.get_bit(13)
        }

        /// Check whether a `Interprocess Control` interrupt occurred
        pub fn interprocess_control(self) -> bool {
            self.0.get_bit(14)
        }

        /// Check whether the reset button is idle or pressed
        pub fn reset_switch_unpressed(self) -> bool {
            self.0.get_bit(16)
        }
    }

    /// Interrupt Mask
    ///
    /// Setting the bit enables the interrupt in the **software** interrupt handler
    #[derive(Copy, Clone, Debug)]
    #[repr(transparent)]
    pub struct InterruptMask(u32);

    impl InterruptMask {
        /// Create an empty [`InterruptMask`]
        pub const fn new() -> Self {
            Self(0)
        }

        /// Read [`InterruptMask`] from the associated mmio register
        pub fn read() -> Self {
            INTERRUPT_MASK.read()
        }

        /// Write `self` to the associated mmio register
        pub fn write(self) {
            INTERRUPT_MASK.write(self);
        }

        /// Check whether `Processor Interface` interrupts are enabled
        pub fn processor_interface(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify whether `Processor Interface` interrupts are enabled
        pub fn with_processor_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(0, enable);
            self
        }

        /// Check whether `Reset Switch` interrupts are enabled
        pub fn reset_switch(self) -> bool {
            self.0.get_bit(1)
        }

        /// Modify whether `Reset Switch` interrupts are enabled
        pub fn with_reset_switch(mut self, enable: bool) -> Self {
            self.0.set_bit(1, enable);
            self
        }

        /// Check whether `Disk Interface` interrupts are enabled
        pub fn disk_interface(self) -> bool {
            self.0.get_bit(2)
        }

        /// Modify whether `Disk Interface` interrupts are enabled
        pub fn with_disk_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(2, enable);
            self
        }

        /// Check whether `Serial Interface` interrupts are enabled
        pub fn serial_interface(self) -> bool {
            self.0.get_bit(3)
        }

        /// Modify whether `Serial Interface` interrupts are enabled
        pub fn with_serial_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(3, enable);
            self
        }

        /// Check whether `External Interface` interrupts are enabled
        pub fn external_interface(self) -> bool {
            self.0.get_bit(4)
        }

        /// Modify whether `External Interface` interrupts are enabled
        pub fn with_external_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(4, enable);
            self
        }

        /// Check whether `Audio Interface` interrupts are enabled
        pub fn audio_interface(self) -> bool {
            self.0.get_bit(5)
        }

        /// Modify whether `Audio Interface` interrupts are enabled
        pub fn with_audio_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(5, enable);
            self
        }

        /// Check whether `Digital Signal Processor` interrupts are enabled
        pub fn digital_signal_processor(self) -> bool {
            self.0.get_bit(6)
        }

        /// Modify whether `Digital Signal Processor` interrupts are enabled
        pub fn with_digital_signal_processor(mut self, enable: bool) -> Self {
            self.0.set_bit(6, enable);
            self
        }

        /// Check whether `Memory Interface` interrupts are enabled
        pub fn memory_interface(self) -> bool {
            self.0.get_bit(7)
        }

        /// Modify whether `Memory Interface` interrupts are enabled
        pub fn with_memory_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(7, enable);
            self
        }

        /// Check whether `Video Interface` interrupts are enabled
        pub fn video_interface(self) -> bool {
            self.0.get_bit(8)
        }

        /// Modify whether `Video Interface` interrupts are enabled
        pub fn with_video_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(8, enable);
            self
        }

        /// Check whether `Pixel Engine Token` interrupts are enabled
        pub fn pixel_engine_token(self) -> bool {
            self.0.get_bit(9)
        }

        /// Modify whether `Pixel Engine Token` interrupts are enabled
        pub fn with_pixel_engine_token(mut self, enable: bool) -> Self {
            self.0.set_bit(9, enable);
            self
        }

        /// Check whether `Pixel Engine Finish` interrupts are enabled
        pub fn pixel_engine_finish(self) -> bool {
            self.0.get_bit(10)
        }

        /// Modify whether `Pixel Engine Finish` interrupts are enabled
        pub fn with_pixel_engine_finish(mut self, enable: bool) -> Self {
            self.0.set_bit(10, enable);
            self
        }

        /// Check whether `Command Processor` interrupts are enabled
        pub fn command_processor(self) -> bool {
            self.0.get_bit(11)
        }

        /// Modify whether `Command Processor` interrupts are enabled
        pub fn with_command_processor(mut self, enable: bool) -> Self {
            self.0.set_bit(11, enable);
            self
        }

        /// Check whether `External Debugger` interrupts are enabled
        pub fn external_debugger(self) -> bool {
            self.0.get_bit(12)
        }

        /// Modify whether `External Debugger` interrupts are enabled
        pub fn with_external_debugger(mut self, enable: bool) -> Self {
            self.0.set_bit(12, enable);
            self
        }

        /// Check whether `High Speed Port` interrupts are enabled
        pub fn high_speed_port(self) -> bool {
            self.0.get_bit(13)
        }

        /// Modify whether `High Speed Port` interrupts are enabled
        pub fn with_high_speed_port(mut self, enable: bool) -> Self {
            self.0.set_bit(13, enable);
            self
        }

        /// Check whether `Interprocess Control` interrupts are enabled
        pub fn interprocess_control(self) -> bool {
            self.0.get_bit(14)
        }

        /// Modify whether `Interprocess Control` interrupts are enabled
        pub fn with_interprocess_control(mut self, enable: bool) -> Self {
            self.0.set_bit(14, enable);
            self
        }
    }
}
