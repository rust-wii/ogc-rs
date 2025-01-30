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

pub unsafe fn write_fifo_base(ptr: &AlignedPhysPtr<u8>) {
    FIFO_BASE_ADDRESS.write(ptr.as_ptr());
}

pub unsafe fn read_fifo_base() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_BASE_ADDRESS.read()).unwrap()
}

pub unsafe fn write_fifo_end(ptr: &AlignedPhysPtr<u8>) {
    FIFO_END_ADDRESS.write(ptr.as_ptr());
}

pub unsafe fn read_fifo_end() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_END_ADDRESS.read()).unwrap()
}

pub unsafe fn write_fifo_write_ptr(ptr: &AlignedPhysPtr<u8>) {
    FIFO_WRITE_ADDRESS.write(ptr.as_ptr());
}

pub unsafe fn read_fifo_write_ptr() -> AlignedPhysPtr<u8> {
    AlignedPhysPtr::new(FIFO_WRITE_ADDRESS.read()).unwrap()
}

mod types {
    use bit_field::BitField;

    use super::{INTERRUPT_CAUSE, INTERRUPT_MASK};

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

        pub fn processor_interface(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn reset_switch(self) -> bool {
            self.0.get_bit(1)
        }

        pub fn disk_interface(self) -> bool {
            self.0.get_bit(2)
        }

        pub fn serial_interface(self) -> bool {
            self.0.get_bit(3)
        }

        pub fn external_interface(self) -> bool {
            self.0.get_bit(4)
        }

        pub fn audio_interface(self) -> bool {
            self.0.get_bit(5)
        }

        pub fn digital_signal_processor(self) -> bool {
            self.0.get_bit(6)
        }

        pub fn memory_interface(self) -> bool {
            self.0.get_bit(7)
        }

        pub fn video_interface(self) -> bool {
            self.0.get_bit(8)
        }

        pub fn pixel_engine_token(self) -> bool {
            self.0.get_bit(9)
        }

        pub fn pixel_engine_finish(self) -> bool {
            self.0.get_bit(10)
        }

        pub fn command_processor(self) -> bool {
            self.0.get_bit(11)
        }

        pub fn external_debugger(self) -> bool {
            self.0.get_bit(12)
        }

        pub fn high_speed_port(self) -> bool {
            self.0.get_bit(13)
        }

        pub fn interprocess_control(self) -> bool {
            self.0.get_bit(14)
        }

        pub fn reset_switch_unpressed(self) -> bool {
            self.0.get_bit(16)
        }
    }

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

        pub fn processor_interface(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_processor_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(0, enable);
            self
        }

        pub fn reset_switch(self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_reset_switch(mut self, enable: bool) -> Self {
            self.0.set_bit(1, enable);
            self
        }

        pub fn disk_interface(self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_disk_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(2, enable);
            self
        }

        pub fn serial_interface(self) -> bool {
            self.0.get_bit(3)
        }

        pub fn with_serial_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(3, enable);
            self
        }

        pub fn external_interface(self) -> bool {
            self.0.get_bit(4)
        }

        pub fn with_external_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(4, enable);
            self
        }

        pub fn audio_interface(self) -> bool {
            self.0.get_bit(5)
        }

        pub fn with_audio_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(5, enable);
            self
        }

        pub fn digital_signal_processor(self) -> bool {
            self.0.get_bit(6)
        }

        pub fn with_digital_signal_processor(mut self, enable: bool) -> Self {
            self.0.set_bit(6, enable);
            self
        }

        pub fn memory_interface(self) -> bool {
            self.0.get_bit(7)
        }

        pub fn with_memory_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(7, enable);
            self
        }

        pub fn video_interface(self) -> bool {
            self.0.get_bit(8)
        }

        pub fn with_video_interface(mut self, enable: bool) -> Self {
            self.0.set_bit(8, enable);
            self
        }

        pub fn pixel_engine_token(self) -> bool {
            self.0.get_bit(9)
        }

        pub fn with_pixel_engine_token(mut self, enable: bool) -> Self {
            self.0.set_bit(9, enable);
            self
        }

        pub fn pixel_engine_finish(self) -> bool {
            self.0.get_bit(10)
        }

        pub fn with_pixel_engine_finish(mut self, enable: bool) -> Self {
            self.0.set_bit(10, enable);
            self
        }

        pub fn command_processor(self) -> bool {
            self.0.get_bit(11)
        }

        pub fn with_command_processor(mut self, enable: bool) -> Self {
            self.0.set_bit(11, enable);
            self
        }

        pub fn external_debugger(self) -> bool {
            self.0.get_bit(12)
        }

        pub fn with_external_debugger(mut self, enable: bool) -> Self {
            self.0.set_bit(12, enable);
            self
        }

        pub fn high_speed_port(self) -> bool {
            self.0.get_bit(13)
        }

        pub fn with_high_speed_port(mut self, enable: bool) -> Self {
            self.0.set_bit(13, enable);
            self
        }

        pub fn interprocess_control(self) -> bool {
            self.0.get_bit(14)
        }

        pub fn with_interprocess_control(mut self, enable: bool) -> Self {
            self.0.set_bit(14, enable);
            self
        }
    }
}
