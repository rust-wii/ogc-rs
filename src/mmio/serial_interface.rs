use types::{
    CommuicationStatus, ExternalClockLock, InputBufferHigh, InputBufferLow, OutputBuffer,
    PollingRegister, Status,
};
use voladdress::{Safe, VolAddress, VolBlock};

pub const BASE: usize = 0xCD00_6400; //or 0xCC00_6400 if you are using gamecube

pub const CHANNEL_0_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE) };

pub const CHANNEL_0_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4) };

pub const CHANNEL_0_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x8) };

pub const CHANNEL_1_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0xC) };

pub const CHANNEL_1_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x10) };

pub const CHANNEL_1_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x14) };

pub const CHANNEL_2_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x18) };

pub const CHANNEL_2_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x1C) };

pub const CHANNEL_2_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x20) };

pub const CHANNEL_3_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x24) };

pub const CHANNEL_3_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x28) };

pub const CHANNEL_3_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2C) };

pub const POLLING_REGISTER: VolAddress<PollingRegister, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x30) };

pub const COMMUNICATION_STATUS_REGISTER: VolAddress<CommuicationStatus, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x34) };

pub const STATUS_REGISTER: VolAddress<Status, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x38) };

pub const EXTERNAL_CLOCK_LOCK: VolAddress<ExternalClockLock, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3C) };

pub const INPUT_OUTPUT_BUFFER: VolBlock<u8, Safe, Safe, 128> =
    unsafe { VolBlock::new(BASE + 0x80) };

mod types {
    use bit_field::BitField;

    use super::{
        CHANNEL_0_IN_BUFFER_HIGH, CHANNEL_0_IN_BUFFER_LOW, CHANNEL_0_OUT_BUFFER,
        CHANNEL_1_IN_BUFFER_HIGH, CHANNEL_1_IN_BUFFER_LOW, CHANNEL_1_OUT_BUFFER,
        CHANNEL_2_IN_BUFFER_HIGH, CHANNEL_2_IN_BUFFER_LOW, CHANNEL_2_OUT_BUFFER,
        CHANNEL_3_IN_BUFFER_HIGH, CHANNEL_3_IN_BUFFER_LOW, CHANNEL_3_OUT_BUFFER,
        COMMUNICATION_STATUS_REGISTER, EXTERNAL_CLOCK_LOCK, POLLING_REGISTER, STATUS_REGISTER,
    };

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct OutputBuffer(u32);

    impl Default for OutputBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl OutputBuffer {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read_zero() -> Self {
            CHANNEL_0_OUT_BUFFER.read()
        }

        pub fn read_one() -> Self {
            CHANNEL_1_OUT_BUFFER.read()
        }

        pub fn read_two() -> Self {
            CHANNEL_2_OUT_BUFFER.read()
        }

        pub fn read_three() -> Self {
            CHANNEL_3_OUT_BUFFER.read()
        }

        pub fn write_zero(self) {
            CHANNEL_0_OUT_BUFFER.write(self);
        }

        pub fn write_one(self) {
            CHANNEL_1_OUT_BUFFER.write(self);
        }

        pub fn write_two(self) {
            CHANNEL_2_OUT_BUFFER.write(self);
        }

        pub fn write_three(self) {
            CHANNEL_3_OUT_BUFFER.write(self);
        }

        pub fn command_opcode(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        pub fn with_command_opcode(mut self, opcode: u32) -> Self {
            self.0.set_bits(16..=23, opcode);
            self
        }

        pub fn output_zero(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        pub fn with_output_zero(mut self, output_zero: u32) -> Self {
            self.0.set_bits(8..=15, output_zero);
            self
        }

        pub fn output_one(self) -> u32 {
            self.0.get_bits(0..=7)
        }

        pub fn with_output_one(mut self, output_one: u32) -> Self {
            self.0.set_bits(0..=7, output_one);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct InputBufferHigh(u32);

    impl Default for InputBufferHigh {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputBufferHigh {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read_zero() -> Self {
            CHANNEL_0_IN_BUFFER_HIGH.read()
        }

        pub fn read_one() -> Self {
            CHANNEL_1_IN_BUFFER_HIGH.read()
        }

        pub fn read_two() -> Self {
            CHANNEL_2_IN_BUFFER_HIGH.read()
        }

        pub fn read_three() -> Self {
            CHANNEL_3_IN_BUFFER_HIGH.read()
        }

        pub fn write_zero(self) {
            CHANNEL_0_IN_BUFFER_HIGH.write(self);
        }

        pub fn write_one(self) {
            CHANNEL_1_IN_BUFFER_HIGH.write(self);
        }

        pub fn write_two(self) {
            CHANNEL_2_IN_BUFFER_HIGH.write(self);
        }

        pub fn write_three(self) {
            CHANNEL_3_IN_BUFFER_HIGH.write(self);
        }

        pub fn error_status(self) -> bool {
            self.0.get_bit(31)
        }

        pub fn with_error_status(mut self, status: bool) -> Self {
            self.0.set_bit(31, status);
            self
        }

        pub fn error_latch(self) -> bool {
            self.0.get_bit(30)
        }

        pub fn with_error_latch(mut self, latch: bool) -> Self {
            self.0.set_bit(30, latch);
            self
        }

        pub fn input_zero(self) -> u32 {
            self.0.get_bits(24..=29)
        }

        pub fn with_input_zero(mut self, zero: u32) -> Self {
            self.0.set_bits(24..=29, zero);
            self
        }

        pub fn input_one(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        pub fn with_input_one(mut self, one: u32) -> Self {
            self.0.set_bits(16..=23, one);
            self
        }

        pub fn input_two(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        pub fn with_input_two(mut self, two: u32) -> Self {
            self.0.set_bits(8..=15, two);
            self
        }

        pub fn input_three(self) -> u32 {
            self.0.get_bits(0..=7)
        }

        pub fn with_input_three(mut self, three: u32) -> Self {
            self.0.set_bits(0..=7, three);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct InputBufferLow(u32);

    impl Default for InputBufferLow {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputBufferLow {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read_zero() -> Self {
            CHANNEL_0_IN_BUFFER_LOW.read()
        }

        pub fn read_one() -> Self {
            CHANNEL_1_IN_BUFFER_LOW.read()
        }

        pub fn read_two() -> Self {
            CHANNEL_2_IN_BUFFER_LOW.read()
        }

        pub fn read_three() -> Self {
            CHANNEL_3_IN_BUFFER_LOW.read()
        }

        pub fn write_zero(self) {
            CHANNEL_0_IN_BUFFER_LOW.write(self);
        }

        pub fn write_one(self) {
            CHANNEL_1_IN_BUFFER_LOW.write(self);
        }

        pub fn write_two(self) {
            CHANNEL_2_IN_BUFFER_LOW.write(self);
        }

        pub fn write_three(self) {
            CHANNEL_3_IN_BUFFER_LOW.write(self);
        }

        pub fn input_four(self) -> u32 {
            self.0.get_bits(24..=31)
        }

        pub fn with_input_four(mut self, four: u32) -> Self {
            self.0.set_bits(24..=31, four);
            self
        }

        pub fn input_five(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        pub fn with_input_five(mut self, five: u32) -> Self {
            self.0.set_bits(16..=23, five);
            self
        }

        pub fn input_six(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        pub fn with_input_six(mut self, six: u32) -> Self {
            self.0.set_bits(8..=15, six);
            self
        }

        pub fn input_seven(self) -> u32 {
            self.0.get_bits(0..=7)
        }

        pub fn with_input_seven(mut self, seven: u32) -> Self {
            self.0.set_bits(0..=7, seven);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PollingRegister(u32);

    impl Default for PollingRegister {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PollingRegister {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            POLLING_REGISTER.read()
        }

        pub fn write(self) {
            POLLING_REGISTER.write(self);
        }

        pub fn lines_per_poll(self) -> u32 {
            self.0.get_bits(16..=25)
        }

        pub fn with_lines_per_poll(mut self, lines: u32) -> Self {
            self.0.set_bits(16..=25, lines);
            self
        }

        pub fn polls_per_frame(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        pub fn with_polls_per_frame(mut self, polls: u32) -> Self {
            self.0.set_bits(8..=15, polls);
            self
        }

        pub fn channel_0_enable(self) -> bool {
            self.0.get_bit(7)
        }

        pub fn with_channel_0_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(7, enable);
            self
        }

        pub fn channel_1_enable(self) -> bool {
            self.0.get_bit(6)
        }

        pub fn with_channel_1_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(6, enable);
            self
        }

        pub fn channel_2_enable(self) -> bool {
            self.0.get_bit(5)
        }

        pub fn with_channel_2_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(5, enable);
            self
        }

        pub fn channel_3_enable(self) -> bool {
            self.0.get_bit(4)
        }

        pub fn with_channel_3_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(4, enable);
            self
        }

        pub fn channel_0_copy_on_vblank(self) -> bool {
            self.0.get_bit(3)
        }

        pub fn with_channel_0_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(3, vblank);
            self
        }

        pub fn channel_1_copy_on_vblank(self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_channel_1_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(2, vblank);
            self
        }

        pub fn channel_2_copy_on_vblank(self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_channel_2_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(1, vblank);
            self
        }

        pub fn channel_3_copy_on_vblank(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_channel_3_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(0, vblank);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct CommuicationStatus(u32);
    impl Default for CommuicationStatus {
        fn default() -> Self {
            Self::new()
        }
    }

    impl CommuicationStatus {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            COMMUNICATION_STATUS_REGISTER.read()
        }

        pub fn write(self) {
            COMMUNICATION_STATUS_REGISTER.write(self);
        }

        pub fn transfer_start(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_transfer_start(mut self, start: bool) -> Self {
            self.0.set_bit(0, start);
            self
        }

        pub fn channel(self) -> u32 {
            self.0.get_bits(1..=2)
        }

        pub fn with_channel(mut self, channel: u32) -> Self {
            self.0.set_bits(1..=2, channel);
            self
        }

        pub fn input_length(self) -> u32 {
            self.0.get_bits(8..=14)
        }

        pub fn with_input_length(mut self, length: u32) -> Self {
            self.0.set_bits(8..=14, length);
            self
        }

        pub fn output_length(self) -> u32 {
            self.0.get_bits(16..=22)
        }

        pub fn with_output_length(mut self, length: u32) -> Self {
            self.0.set_bits(16..=22, length);
            self
        }

        pub fn read_status_interrupt_mask(self) -> bool {
            self.0.get_bit(27)
        }

        pub fn with_read_status_interrupt_mask(mut self, enable: bool) -> Self {
            self.0.set_bit(27, enable);
            self
        }

        pub fn read_status_interrupt(self) -> bool {
            self.0.get_bit(28)
        }

        pub fn with_read_status_interrupt(mut self, enable: bool) -> Self {
            self.0.set_bit(28, enable);
            self
        }

        pub fn communication_error(self) -> bool {
            self.0.get_bit(29)
        }

        pub fn with_communication_error(mut self, error: bool) -> Self {
            self.0.set_bit(29, error);
            self
        }

        pub fn transfer_complete_interrupt_mask(self) -> bool {
            self.0.get_bit(30)
        }

        pub fn with_transfer_complete_interrupt_mask(mut self, enable: bool) -> Self {
            self.0.set_bit(30, enable);
            self
        }

        pub fn transfer_complete_interrupt(self) -> bool {
            self.0.get_bit(31)
        }

        pub fn with_transfer_complete_interrupt(mut self, enable: bool) -> Self {
            self.0.set_bit(31, enable);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Status(u32);

    impl Default for Status {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Status {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            STATUS_REGISTER.read()
        }

        pub fn write(self) {
            STATUS_REGISTER.write(self);
        }

        pub fn output_buffer_write(self) -> bool {
            self.0.get_bit(31)
        }

        pub fn with_output_buffer_write(mut self, write: bool) -> Self {
            self.0.set_bit(31, write);
            self
        }

        pub fn channel_0_read_status(self) -> bool {
            self.0.get_bit(29)
        }

        pub fn with_channel_0_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(29, read_status);
            self
        }

        pub fn channel_0_write_status(self) -> bool {
            self.0.get_bit(28)
        }

        pub fn with_channel_0_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(28, write_status);
            self
        }

        pub fn channel_0_no_response_error(self) -> bool {
            self.0.get_bit(27)
        }

        pub fn with_channel_0_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(27, error);
            self
        }

        pub fn channel_0_collision_error(self) -> bool {
            self.0.get_bit(26)
        }

        pub fn with_channel_0_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(26, error);
            self
        }

        pub fn channel_0_buffer_overrun_error(self) -> bool {
            self.0.get_bit(25)
        }

        pub fn with_channel_0_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(25, error);
            self
        }

        pub fn channel_0_buffer_underrun_error(self) -> bool {
            self.0.get_bit(24)
        }

        pub fn with_channel_0_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(24, error);
            self
        }

        pub fn channel_1_read_status(self) -> bool {
            self.0.get_bit(21)
        }

        pub fn with_channel_1_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(21, read_status);
            self
        }

        pub fn channel_1_write_status(self) -> bool {
            self.0.get_bit(20)
        }

        pub fn with_channel_1_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(20, write_status);
            self
        }

        pub fn channel_1_no_response_error(self) -> bool {
            self.0.get_bit(19)
        }

        pub fn with_channel_1_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(19, error);
            self
        }

        pub fn channel_1_collision_error(self) -> bool {
            self.0.get_bit(18)
        }

        pub fn with_channel_1_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(18, error);
            self
        }

        pub fn channel_1_buffer_overrun_error(self) -> bool {
            self.0.get_bit(17)
        }

        pub fn with_channel_1_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(17, error);
            self
        }

        pub fn channel_1_buffer_underrrun_error(self) -> bool {
            self.0.get_bit(16)
        }

        pub fn with_channel_1_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(16, error);
            self
        }

        pub fn channel_2_read_status(self) -> bool {
            self.0.get_bit(13)
        }

        pub fn with_channel_2_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(13, read_status);
            self
        }

        pub fn channel_2_write_status(self) -> bool {
            self.0.get_bit(12)
        }

        pub fn with_channel_2_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(12, write_status);
            self
        }

        pub fn channel_2_no_response_error(self) -> bool {
            self.0.get_bit(11)
        }

        pub fn with_channel_2_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(11, error);
            self
        }

        pub fn channel_2_collision_error(self) -> bool {
            self.0.get_bit(10)
        }

        pub fn with_channel_2_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(10, error);
            self
        }

        pub fn channel_2_buffer_overrun_error(self) -> bool {
            self.0.get_bit(9)
        }

        pub fn with_channel_2_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(9, error);
            self
        }

        pub fn channel_2_buffer_underrun_error(self) -> bool {
            self.0.get_bit(8)
        }

        pub fn with_channel_2_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(8, error);
            self
        }

        pub fn channel_3_read_status(self) -> bool {
            self.0.get_bit(5)
        }

        pub fn with_channel_3_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(5, read_status);
            self
        }

        pub fn channel_3_write_status(self) -> bool {
            self.0.get_bit(4)
        }

        pub fn with_channel_3_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(4, write_status);
            self
        }

        pub fn channel_3_no_response_error(self) -> bool {
            self.0.get_bit(3)
        }

        pub fn with_channel_3_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(3, error);
            self
        }

        pub fn channel_3_collision_error(self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_channel_3_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(2, error);
            self
        }

        pub fn channel_3_buffer_overrun_error(self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_channel_3_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(1, error);
            self
        }

        pub fn channel_3_buffer_underrun_error(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_channel_3_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(0, error);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct ExternalClockLock(u32);

    impl Default for ExternalClockLock {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ExternalClockLock {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            EXTERNAL_CLOCK_LOCK.read()
        }

        pub fn write(self) {
            EXTERNAL_CLOCK_LOCK.write(self);
        }

        pub fn thirty_two_mhz_clock_lock(self) -> bool {
            self.0.get_bit(31)
        }

        pub fn with_thirty_two_mhz_clock_lock(mut self, lock: bool) -> Self {
            self.0.set_bit(31, lock);
            self
        }
    }
}
