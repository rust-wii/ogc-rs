#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub use types::{
    CommuicationStatus, ExternalClockLock, InputBufferHigh, InputBufferLow, OutputBuffer,
    PollingRegister, Status,
};
use voladdress::{Safe, VolAddress, VolBlock};

const BASE: usize = 0xCD00_6400; //or 0xCC00_6400 if you are using gamecube

const CHANNEL_0_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> = unsafe { VolAddress::new(BASE) };

const CHANNEL_0_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4) };

const CHANNEL_0_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x8) };

const CHANNEL_1_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0xC) };

const CHANNEL_1_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x10) };

const CHANNEL_1_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x14) };

const CHANNEL_2_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x18) };

const CHANNEL_2_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x1C) };

const CHANNEL_2_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x20) };

const CHANNEL_3_OUT_BUFFER: VolAddress<OutputBuffer, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x24) };

const CHANNEL_3_IN_BUFFER_HIGH: VolAddress<InputBufferHigh, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x28) };

const CHANNEL_3_IN_BUFFER_LOW: VolAddress<InputBufferLow, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2C) };

const POLLING_REGISTER: VolAddress<PollingRegister, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x30) };

const COMMUNICATION_STATUS_REGISTER: VolAddress<CommuicationStatus, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x34) };

const STATUS_REGISTER: VolAddress<Status, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x38) };

const EXTERNAL_CLOCK_LOCK: VolAddress<ExternalClockLock, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3C) };

/// General Purpose Input Output Buffer for Serial Devices.
///
/// This is used to supply and recieve data based on the [`CommuicationStatus`] register.
pub const INPUT_OUTPUT_BUFFER: VolBlock<u32, Safe, Safe, 32> =
    unsafe { VolBlock::new(BASE + 0x80) };

pub(crate) mod types {
    use bit_field::BitField;

    use super::{
        CHANNEL_0_IN_BUFFER_HIGH, CHANNEL_0_IN_BUFFER_LOW, CHANNEL_0_OUT_BUFFER,
        CHANNEL_1_IN_BUFFER_HIGH, CHANNEL_1_IN_BUFFER_LOW, CHANNEL_1_OUT_BUFFER,
        CHANNEL_2_IN_BUFFER_HIGH, CHANNEL_2_IN_BUFFER_LOW, CHANNEL_2_OUT_BUFFER,
        CHANNEL_3_IN_BUFFER_HIGH, CHANNEL_3_IN_BUFFER_LOW, CHANNEL_3_OUT_BUFFER,
        COMMUNICATION_STATUS_REGISTER, EXTERNAL_CLOCK_LOCK, POLLING_REGISTER, STATUS_REGISTER,
    };

    /// Output Buffer Register
    ///
    /// This is also known as the  Command Buffer Register. This isn't often used in general
    /// Gamecube Controller use, though might be more interesting in other contexts such as
    /// GBA/GBASP linking.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct OutputBuffer(u32);

    impl Default for OutputBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl OutputBuffer {
        /// Returns a new empty instance of [`OutputBuffer`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Read from channel 0's associated [`OutputBuffer`] register
        #[must_use]
        pub fn read_zero() -> Self {
            CHANNEL_0_OUT_BUFFER.read()
        }

        /// Read from channel 1's associated [`OutputBuffer`] register
        #[must_use]
        pub fn read_one() -> Self {
            CHANNEL_1_OUT_BUFFER.read()
        }

        /// Read from channel 2's associated [`OutputBuffer`] register
        #[must_use]
        pub fn read_two() -> Self {
            CHANNEL_2_OUT_BUFFER.read()
        }

        /// Read from channel 3's associated [`OutputBuffer`] register
        #[must_use]
        pub fn read_three() -> Self {
            CHANNEL_3_OUT_BUFFER.read()
        }

        /// Write into channel 0's [`OutputBuffer`] Register
        pub fn write_zero(self) {
            CHANNEL_0_OUT_BUFFER.write(self);
        }

        /// Write into channel 1's [`OutputBuffer`] Register
        pub fn write_one(self) {
            CHANNEL_1_OUT_BUFFER.write(self);
        }

        /// Write into channel 2's [`OutputBuffer`] Register
        pub fn write_two(self) {
            CHANNEL_2_OUT_BUFFER.write(self);
        }

        /// Write into channel 3's [`OutputBuffer`] Register
        pub fn write_three(self) {
            CHANNEL_3_OUT_BUFFER.write(self);
        }

        /// Returns the command opcode
        #[must_use]
        pub fn command_opcode(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        /// Modify the command opcode
        ///
        /// This command is device specific. Sadly, there is little documentation on serial devices
        /// in general.
        #[must_use]
        pub fn with_command_opcode(mut self, opcode: u32) -> Self {
            self.0.set_bits(16..=23, opcode);
            self
        }

        /// Returns argument 1 of the command [`OutputBuffer`]
        #[must_use]
        pub fn output_zero(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        /// Modify argument 1 of the [`OutputBuffer`]   
        #[must_use]
        pub fn with_output_zero(mut self, output_zero: u32) -> Self {
            self.0.set_bits(8..=15, output_zero);
            self
        }

        /// Returns argument 2 of the command [`OutputBuffer`]
        #[must_use]
        pub fn output_one(self) -> u32 {
            self.0.get_bits(0..=7)
        }
        /// Modify argument 2 of the [`OutputBuffer`]   
        #[must_use]
        pub fn with_output_one(mut self, output_one: u32) -> Self {
            self.0.set_bits(0..=7, output_one);
            self
        }
    }

    /// Input Buffer High Register
    ///
    /// There are 4 MMIO values corresponding to each serial device port
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct InputBufferHigh(u32);

    impl Default for InputBufferHigh {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputBufferHigh {
        /// Returns a new empty instance of [`InputbufferHigh`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Read from channel 0's associated [`InputBufferHigh`] register
        #[must_use]
        pub fn read_zero() -> Self {
            CHANNEL_0_IN_BUFFER_HIGH.read()
        }

        /// Read from channel 1's associated [`InputBufferHigh`] register
        #[must_use]
        pub fn read_one() -> Self {
            CHANNEL_1_IN_BUFFER_HIGH.read()
        }

        /// Read from channel 2's associated [`InputBufferHigh`] register
        #[must_use]
        pub fn read_two() -> Self {
            CHANNEL_2_IN_BUFFER_HIGH.read()
        }

        /// Read from channel 3's associated [`InputBufferHigh`] register
        #[must_use]
        pub fn read_three() -> Self {
            CHANNEL_3_IN_BUFFER_HIGH.read()
        }

        /// Write into channel 0's associated [`InputBufferHigh`] register
        pub fn write_zero(self) {
            CHANNEL_0_IN_BUFFER_HIGH.write(self);
        }

        /// Write into channel 1's associated [`InputBufferHigh`] register
        pub fn write_one(self) {
            CHANNEL_1_IN_BUFFER_HIGH.write(self);
        }

        /// Write into channel 2's associated [`InputBufferHigh`] register
        pub fn write_two(self) {
            CHANNEL_2_IN_BUFFER_HIGH.write(self);
        }

        /// Write into channel 3's associated [`InputBufferHigh`] register
        pub fn write_three(self) {
            CHANNEL_3_IN_BUFFER_HIGH.write(self);
        }

        /// Returns whether an error occured while reading [`InputBufferHigh`]
        ///
        /// true = error occured (check if an error was latched with
        /// [`InputBufferHigh::error_latch()`] | false = no error has occurred
        #[must_use]
        pub fn error_status(self) -> bool {
            self.0.get_bit(31)
        }

        /// Modify whether an error occured while reading the [`InputBufferHigh`]
        ///
        /// true = error has occured | false = there is no error
        #[must_use]
        pub fn with_error_status(mut self, status: bool) -> Self {
            self.0.set_bit(31, status);
            self
        }

        /// Returns whether an error was latched into [`Status`]
        ///
        /// true = error has been latched into [`Status`] | false = no error latched into
        /// [`Status`]
        #[must_use]
        pub fn error_latch(self) -> bool {
            self.0.get_bit(30)
        }

        /// Modify the state of error latch
        ///
        /// true = error has been latched into [`Status`] | false = no error latched into
        /// [`Status`]
        #[must_use]
        pub fn with_error_latch(mut self, latch: bool) -> Self {
            self.0.set_bit(30, latch);
            self
        }

        /// Returns data byte 0
        #[must_use]
        pub fn input_zero(self) -> u32 {
            self.0.get_bits(24..=29)
        }

        /// Modify data byte 0
        #[must_use]
        pub fn with_input_zero(mut self, zero: u32) -> Self {
            self.0.set_bits(24..=29, zero);
            self
        }

        /// Returns data byte 1
        #[must_use]
        pub fn input_one(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        /// Modify data byte 1
        #[must_use]
        pub fn with_input_one(mut self, one: u32) -> Self {
            self.0.set_bits(16..=23, one);
            self
        }

        /// Returns data byte 2
        #[must_use]
        pub fn input_two(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        /// Modify data byte 2
        #[must_use]
        pub fn with_input_two(mut self, two: u32) -> Self {
            self.0.set_bits(8..=15, two);
            self
        }

        /// Returns data byte 3
        #[must_use]
        pub fn input_three(self) -> u32 {
            self.0.get_bits(0..=7)
        }

        /// Modify data byte 3
        #[must_use]
        pub fn with_input_three(mut self, three: u32) -> Self {
            self.0.set_bits(0..=7, three);
            self
        }
    }

    /// Input Buffer Low Register
    ///
    /// This has 4 different MMIO values for each available serial channel
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct InputBufferLow(u32);

    impl Default for InputBufferLow {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputBufferLow {
        /// Returns a new empty instance of [`InputBufferLow`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Read channel 0's associated [`InputBufferLow`] instance.
        #[must_use]
        pub fn read_zero() -> Self {
            CHANNEL_0_IN_BUFFER_LOW.read()
        }

        /// Read channel 1's associated [`InputBufferLow`] instance.
        #[must_use]
        pub fn read_one() -> Self {
            CHANNEL_1_IN_BUFFER_LOW.read()
        }

        /// Read channel 2's associated [`InputBufferLow`] instance.
        #[must_use]
        pub fn read_two() -> Self {
            CHANNEL_2_IN_BUFFER_LOW.read()
        }

        /// Read channel 3's associated [`InputBufferLow`] instance.
        #[must_use]
        pub fn read_three() -> Self {
            CHANNEL_3_IN_BUFFER_LOW.read()
        }

        /// Write into channel 0's associated [`InputBufferLow`] instance
        pub fn write_zero(self) {
            CHANNEL_0_IN_BUFFER_LOW.write(self);
        }

        /// Write into channel 1's associated [`InputBufferLow`] instance
        pub fn write_one(self) {
            CHANNEL_1_IN_BUFFER_LOW.write(self);
        }

        /// Write into channel 2's associated [`InputBufferLow`] instance
        pub fn write_two(self) {
            CHANNEL_2_IN_BUFFER_LOW.write(self);
        }

        /// Write into channel 3's associated [`InputBufferLow`] instance
        pub fn write_three(self) {
            CHANNEL_3_IN_BUFFER_LOW.write(self);
        }

        /// Returns data byte 4
        #[must_use]
        pub fn input_four(self) -> u32 {
            self.0.get_bits(24..=31)
        }

        /// Modify data byte 4
        #[must_use]
        pub fn with_input_four(mut self, four: u32) -> Self {
            self.0.set_bits(24..=31, four);
            self
        }

        ///Returns data byte 5
        #[must_use]
        pub fn input_five(self) -> u32 {
            self.0.get_bits(16..=23)
        }

        /// Modify data byte 5
        #[must_use]
        pub fn with_input_five(mut self, five: u32) -> Self {
            self.0.set_bits(16..=23, five);
            self
        }

        ///Returns data byte 6
        #[must_use]
        pub fn input_six(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        /// Modify data byte 6
        #[must_use]
        pub fn with_input_six(mut self, six: u32) -> Self {
            self.0.set_bits(8..=15, six);
            self
        }

        ///Returns data byte 7
        #[must_use]
        pub fn input_seven(self) -> u32 {
            self.0.get_bits(0..=7)
        }

        /// Modify data byte 7
        #[must_use]
        pub fn with_input_seven(mut self, seven: u32) -> Self {
            self.0.set_bits(0..=7, seven);
            self
        }
    }

    /// Serial Polling Register
    ///
    /// This controls whether to enable polling for a setgcerial devices
    /// This also provides when and how to poll data for the serial devices.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PollingRegister(u32);

    impl Default for PollingRegister {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PollingRegister {
        /// Returns an new empty instance of [`PollingRegister`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Reads the associated MMIO value
        #[must_use]
        pub fn read() -> Self {
            POLLING_REGISTER.read()
        }

        /// Writes to the associated MMIO value consuming self
        pub fn write(self) {
            POLLING_REGISTER.write(self);
        }

        /// Returns how many scanlines to wait before trying to poll enabled serial devices
        #[must_use]
        pub fn lines_per_poll(self) -> u32 {
            self.0.get_bits(16..=25)
        }

        /// Modify how many scanlines to wait before trying to poll the enabled serial devices
        #[must_use]
        pub fn with_lines_per_poll(mut self, lines: u32) -> Self {
            self.0.set_bits(16..=25, lines);
            self
        }

        /// Returns how many times the enabled serial devices are polled
        #[must_use]
        pub fn polls_per_frame(self) -> u32 {
            self.0.get_bits(8..=15)
        }

        /// Modify the amount of times to poll the enabled serial devices
        #[must_use]
        pub fn with_polls_per_frame(mut self, polls: u32) -> Self {
            self.0.set_bits(8..=15, polls);
            self
        }

        /// Returns whether the associate channel is enabled for polling
        #[must_use]
        pub fn channel_0_enable(self) -> bool {
            self.0.get_bit(7)
        }

        /// Modify associated channel's enabled state
        ///
        /// true = channel enable | false = channel disable
        #[must_use]
        pub fn with_channel_0_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(7, enable);
            self
        }

        /// Returns whether the associate channel is enabled for polling
        #[must_use]
        pub fn channel_1_enable(self) -> bool {
            self.0.get_bit(6)
        }

        /// Modify associated channel's enabled state
        ///
        /// true = channel enable | false = channel disable
        #[must_use]
        pub fn with_channel_1_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(6, enable);
            self
        }

        /// Returns whether the associate channel is enabled for polling
        #[must_use]
        pub fn channel_2_enable(self) -> bool {
            self.0.get_bit(5)
        }

        /// Modify associated channel's enabled state
        ///
        /// true = channel enable | false = channel disable
        #[must_use]
        pub fn with_channel_2_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(5, enable);
            self
        }

        /// Returns whether the associate channel is enabled for polling
        #[must_use]
        pub fn channel_3_enable(self) -> bool {
            self.0.get_bit(4)
        }

        /// Modify associated channel's enabled state
        ///
        /// true = channel enable | false = channel disable
        #[must_use]
        pub fn with_channel_3_enable(mut self, enable: bool) -> Self {
            self.0.set_bit(4, enable);
            self
        }

        /// Returns the associated channels copy on vblank state
        ///
        /// true = copy on Vblank, false = copy on write
        #[must_use]
        pub fn channel_0_copy_on_vblank(self) -> bool {
            self.0.get_bit(3)
        }

        /// Modify state of the associated channels copy on vblank status
        ///
        /// true = copy on vblank, false = copy on write
        #[must_use]
        pub fn with_channel_0_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(3, vblank);
            self
        }

        /// Returns the associated channels copy on vblank state
        ///
        /// true = copy on Vblank, false = copy on write
        #[must_use]
        pub fn channel_1_copy_on_vblank(self) -> bool {
            self.0.get_bit(2)
        }

        /// Modify state of the associated channels copy on vblank status
        ///
        /// true = copy on vblank, false = copy on write
        #[must_use]
        pub fn with_channel_1_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(2, vblank);
            self
        }

        /// Returns the associated channels copy on vblank state
        ///
        /// true = copy on Vblank, false = copy on write
        #[must_use]
        pub fn channel_2_copy_on_vblank(self) -> bool {
            self.0.get_bit(1)
        }

        /// Modify state of the associated channels copy on vblank status
        ///
        /// true = copy on vblank, false = copy on write
        #[must_use]
        pub fn with_channel_2_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(1, vblank);
            self
        }
        /// Returns the associated channels copy on vblank state
        ///
        /// true = copy on Vblank, false = copy on write
        #[must_use]
        pub fn channel_3_copy_on_vblank(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify state of the associated channels copy on vblank status
        ///
        /// true = copy on vblank, false = copy on write
        #[must_use]
        pub fn with_channel_3_copy_on_vblank(mut self, vblank: bool) -> Self {
            self.0.set_bit(0, vblank);
            self
        }
    }

    /// Serial Interface Communcation Status Register
    ///
    /// Controls communcation to serial devices. It also has state to check whether a communcation
    /// errored for any reason [`CommuicationStatus::communication_error`]
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct CommuicationStatus(u32);
    impl Default for CommuicationStatus {
        fn default() -> Self {
            Self::new()
        }
    }

    impl CommuicationStatus {
        /// Constructs a new empty empty instance of [`CommuicationStatus`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Returns the associate MMIO value
        #[must_use]
        pub fn read() -> Self {
            COMMUNICATION_STATUS_REGISTER.read()
        }

        /// Writes to the associated MMIO value consuming self
        pub fn write(self) {
            COMMUNICATION_STATUS_REGISTER.write(self);
        }

        /// Returns whether a communcation transfer is currently in progress
        #[must_use]
        pub fn transfer_start(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify the state of a communcation transfer
        ///
        /// true = start a communication transfer with  device specified by [`Self::channel()`] |
        /// false = dont change transfer start state
        #[must_use]
        pub fn with_transfer_start(mut self, start: bool) -> Self {
            self.0.set_bit(0, start);
            self
        }

        /// Returns which channel to start the communcation transfer on
        #[must_use]
        pub fn channel(self) -> u32 {
            self.0.get_bits(1..=2)
        }

        /// Modify the channel to start the transfer on
        ///
        /// channel must be 0..=3
        #[must_use]
        pub fn with_channel(mut self, channel: u32) -> Self {
            self.0.set_bits(1..=2, channel);
            self
        }

        /// Returns the amount of bytes expected receive from the serial device
        ///
        /// the returns value is 0..=127 where 0 = 128 bytes to recieve
        #[must_use]
        pub fn input_length(self) -> u32 {
            self.0.get_bits(8..=14)
        }

        /// Modify the amount data expected to read from the serial device
        ///
        /// length must be is 0..=127 where 0 = 128 bytes to recieve
        #[must_use]
        pub fn with_input_length(mut self, length: u32) -> Self {
            self.0.set_bits(8..=14, length);
            self
        }

        /// Returns the amount of bytes expected to send to the serial device
        ///
        /// the returns value is 0..=127 where 0 = 128 bytes to recieve
        #[must_use]
        pub fn output_length(self) -> u32 {
            self.0.get_bits(16..=22)
        }

        /// Modify the amount of data to send to the serial device.
        ///
        /// Safety:
        /// length must be 0..=127 where 0 = 128 bytes to recieve
        #[must_use]
        pub fn with_output_length(mut self, length: u32) -> Self {
            self.0.set_bits(16..=22, length);
            self
        }

        /// Returns whether a read status interrupt can occur
        ///
        /// true = read status interrupt can occur | false = read status interrupt can not occur
        #[must_use]
        pub fn read_status_interrupt_mask(self) -> bool {
            self.0.get_bit(27)
        }

        /// Modify the state of read status interrupt mask
        ///
        /// true = enable read status interrupt | false = disable read status interrupt
        #[must_use]
        pub fn with_read_status_interrupt_mask(mut self, enable: bool) -> Self {
            self.0.set_bit(27, enable);
            self
        }

        /// Returns whether a read status interrupt has occured
        ///
        /// true = read status interrupt has occured | false = read status interrupt hasn't occured
        /// yet
        #[must_use]
        pub fn read_status_interrupt(self) -> bool {
            self.0.get_bit(28)
        }

        /// Modify state of read status interrupt
        ///
        /// true = clear the read status interrupt | false = leave read status interrupt intact
        #[must_use]
        pub fn with_read_status_interrupt(mut self, enable: bool) -> Self {
            self.0.set_bit(28, enable);
            self
        }

        /// Returns whether a communcation error occured during a communcation transfer
        ///
        /// if true you will want to check out [`Status`] to find out what specific error occured
        #[must_use]
        pub fn communication_error(self) -> bool {
            self.0.get_bit(29)
        }

        /// Modify the state of communication error
        ///
        /// true = clears the communication error | false = leaves communication error intact
        #[must_use]
        pub fn with_communication_error(mut self, error: bool) -> Self {
            self.0.set_bit(29, error);
            self
        }

        /// Returns whether a transfer complete interrupt can occur
        ///
        /// true = transfer complete interrupts can occur | false = transfer complete interrupts
        /// are disabled
        #[must_use]
        pub fn transfer_complete_interrupt_mask(self) -> bool {
            self.0.get_bit(30)
        }
        /// Modify state of transfer complete interrupt mask
        ///
        /// true = enable transfer complete interrupt | false = disabled transfer complete
        /// interrupt
        #[must_use]
        pub fn with_transfer_complete_interrupt_mask(mut self, enable: bool) -> Self {
            self.0.set_bit(30, enable);
            self
        }

        /// Returns whether a transfer complete interrupt happened
        ///
        /// true = transfer complete interrupt happened | false = transfer complete interrupt has
        /// not happened
        #[must_use]
        pub fn transfer_complete_interrupt(self) -> bool {
            self.0.get_bit(31)
        }

        /// Modify state of transfer complete interrupt
        ///
        /// true = clears the interrupt. false = leave interrrupt intact
        #[must_use]
        pub fn with_transfer_complete_interrupt(mut self, enable: bool) -> Self {
            self.0.set_bit(31, enable);
            self
        }
    }

    /// Serial Interface Status Regsister
    ///
    /// This holds read and write status of channels and whether any channels have had errors
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Status(u32);

    impl Default for Status {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Status {
        /// Returns new empty instance of [`Status`]
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Reads the associated MMIO value returning a new instance of [`Status`]
        #[must_use]
        pub fn read() -> Self {
            STATUS_REGISTER.read()
        }
        /// Writes to associated MMIO value consuming self in the process.
        pub fn write(self) {
            STATUS_REGISTER.write(self);
        }

        /// Returns whether any output buffer need to be written
        ///
        /// true = need to write output buffers | false = output buffers on host and device are the
        /// same
        #[must_use]
        pub fn output_buffer_write(self) -> bool {
            self.0.get_bit(31)
        }

        ///Modify whether to write output buffers on [`Status::write`]
        ///
        ///true = write output buffer | false = leave output buffers be
        #[must_use]
        pub fn with_output_buffer_write(mut self, write: bool) -> Self {
            self.0.set_bit(31, write);
            self
        }

        /// Returns whether the associated channel's [`InputBufferHigh`] and [`InputBufferLow`] has already
        /// been read from the host
        ///
        /// true = we already read this data | false = we need to read data from the device
        #[must_use]
        pub fn channel_0_read_status(self) -> bool {
            self.0.get_bit(29)
        }

        /// Modify state of associated channel's read status
        #[must_use]
        pub fn with_channel_0_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(29, read_status);
            self
        }

        /// Returns whether the associated channel's [`OutputBuffer`] needs to be written to its corresponding
        /// device.
        ///
        /// true = we need to write the data to the device | false = host and device has same data
        #[must_use]
        pub fn channel_0_write_status(self) -> bool {
            self.0.get_bit(28)
        }

        /// Modify state of channel 0 write status
        #[must_use]
        pub fn with_channel_0_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(28, write_status);
            self
        }

        /// Returns whether channel 0 returned no response
        #[must_use]
        pub fn channel_0_no_response_error(self) -> bool {
            self.0.get_bit(27)
        }

        /// Modify state of channel 0 no response error
        #[must_use]
        pub fn with_channel_0_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(27, error);
            self
        }

        /// Returns whether a collision occured on channel 0
        #[must_use]
        pub fn channel_0_collision_error(self) -> bool {
            self.0.get_bit(26)
        }

        /// Modify state of channel 0 collision error
        #[must_use]
        pub fn with_channel_0_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(26, error);
            self
        }

        /// Returns whether a buffer overrun occured on channel 0
        #[must_use]
        pub fn channel_0_buffer_overrun_error(self) -> bool {
            self.0.get_bit(25)
        }

        /// Modify state of channel 0 buffer overrun error
        #[must_use]
        pub fn with_channel_0_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(25, error);
            self
        }

        /// Returns whether a buffer underrun occured on channel 0
        #[must_use]
        pub fn channel_0_buffer_underrun_error(self) -> bool {
            self.0.get_bit(24)
        }

        /// Modify state of channel 0 buffer underrrun error
        #[must_use]
        pub fn with_channel_0_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(24, error);
            self
        }

        /// Returns whether the associated channel's [`InputBufferHigh`] and [`InputBufferLow`] has already
        /// been read from the host
        ///
        /// true = we already read this data | false = we need to read data from the device
        #[must_use]
        pub fn channel_1_read_status(self) -> bool {
            self.0.get_bit(21)
        }

        /// Modify state of associated channel's read status
        #[must_use]
        pub fn with_channel_1_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(21, read_status);
            self
        }

        /// Returns whether the associated channel's [`OutputBuffer`] needs to be written to its corresponding
        /// device.
        ///
        /// true = we need to write the data to the device | false = host and device has same data
        #[must_use]
        pub fn channel_1_write_status(self) -> bool {
            self.0.get_bit(20)
        }

        /// Modify state of channel 1 write status
        #[must_use]
        pub fn with_channel_1_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(20, write_status);
            self
        }

        /// Returns whether channel 1 returned no response
        #[must_use]
        pub fn channel_1_no_response_error(self) -> bool {
            self.0.get_bit(19)
        }

        /// Modify state of channel 1 no response error
        #[must_use]
        pub fn with_channel_1_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(19, error);
            self
        }

        /// Returns whether a collision occured on channel 1
        #[must_use]
        pub fn channel_1_collision_error(self) -> bool {
            self.0.get_bit(18)
        }

        /// Modify state of channel 1 collision error
        #[must_use]
        pub fn with_channel_1_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(18, error);
            self
        }

        /// Returns whether a buffer overrun occured on channel 1
        #[must_use]
        pub fn channel_1_buffer_overrun_error(self) -> bool {
            self.0.get_bit(17)
        }

        /// Modify state of channel 1 buffer overrun error
        #[must_use]
        pub fn with_channel_1_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(17, error);
            self
        }

        /// Returns whether a buffer underrun occured on channel 1
        #[must_use]
        pub fn channel_1_buffer_underrrun_error(self) -> bool {
            self.0.get_bit(16)
        }

        /// Modify state of channel 1 buffer underrrun error
        #[must_use]
        pub fn with_channel_1_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(16, error);
            self
        }

        /// Returns whether the associated channel's [`InputBufferHigh`] and [`InputBufferLow`] has already
        /// been read from the host
        ///
        /// true = we already read this data | false = we need to read data from the device
        #[must_use]
        pub fn channel_2_read_status(self) -> bool {
            self.0.get_bit(13)
        }

        /// Modify state of associated channel's read status
        #[must_use]
        pub fn with_channel_2_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(13, read_status);
            self
        }

        /// Returns whether the associated channel's [`OutputBuffer`] needs to be written to its corresponding
        /// device.
        ///
        /// true = we need to write the data to the device | false = host and device has same data
        #[must_use]
        pub fn channel_2_write_status(self) -> bool {
            self.0.get_bit(12)
        }

        /// Modify state of channel 2 write status
        #[must_use]
        pub fn with_channel_2_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(12, write_status);
            self
        }

        /// Returns whether channel 2 returned no response
        #[must_use]
        pub fn channel_2_no_response_error(self) -> bool {
            self.0.get_bit(11)
        }

        /// Modify state of channel 2 no response error
        #[must_use]
        pub fn with_channel_2_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(11, error);
            self
        }

        /// Returns whether a collision occured on channel 2
        #[must_use]
        pub fn channel_2_collision_error(self) -> bool {
            self.0.get_bit(10)
        }

        /// Modify state of channel 2 collision error
        #[must_use]
        pub fn with_channel_2_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(10, error);
            self
        }

        /// Returns whether a buffer overrun occured on channel 2
        #[must_use]
        pub fn channel_2_buffer_overrun_error(self) -> bool {
            self.0.get_bit(9)
        }

        /// Modify state of channel 2 buffer overrun error
        #[must_use]
        pub fn with_channel_2_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(9, error);
            self
        }

        /// Returns whether a buffer underrun occured on channel 2
        #[must_use]
        pub fn channel_2_buffer_underrun_error(self) -> bool {
            self.0.get_bit(8)
        }

        /// Modify state of channel 2 buffer underrrun error
        #[must_use]
        pub fn with_channel_2_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(8, error);
            self
        }

        /// Returns whether the associated channel's [`InputBufferHigh`] and [`InputBufferLow`] has already
        /// been read from the host
        ///
        /// true = we already read this data | false = we need to read data from the device
        #[must_use]
        pub fn channel_3_read_status(self) -> bool {
            self.0.get_bit(5)
        }

        /// Modify state of associated channel's read status
        #[must_use]
        pub fn with_channel_3_read_status(mut self, read_status: bool) -> Self {
            self.0.set_bit(5, read_status);
            self
        }

        /// Returns whether the associated channel's [`OutputBuffer`] needs to be written to its corresponding
        /// device.
        ///
        /// true = we need to write the data to the device | false = host and device has same data
        #[must_use]
        pub fn channel_3_write_status(self) -> bool {
            self.0.get_bit(4)
        }

        /// Modify state of  channel 3 write status
        #[must_use]
        pub fn with_channel_3_write_status(mut self, write_status: bool) -> Self {
            self.0.set_bit(4, write_status);
            self
        }

        /// Returns whether channel 3 returned no response
        #[must_use]
        pub fn channel_3_no_response_error(self) -> bool {
            self.0.get_bit(3)
        }

        /// Modify state of channel 3 no response error
        #[must_use]
        pub fn with_channel_3_no_response_error(mut self, error: bool) -> Self {
            self.0.set_bit(3, error);
            self
        }

        /// Returns whether a collision occured on channel 3
        #[must_use]
        pub fn channel_3_collision_error(self) -> bool {
            self.0.get_bit(2)
        }

        /// Modify state of channel 3 collision error
        #[must_use]
        pub fn with_channel_3_collision_error(mut self, error: bool) -> Self {
            self.0.set_bit(2, error);
            self
        }

        /// Returns whether a buffer overrun occured on channel 3
        #[must_use]
        pub fn channel_3_buffer_overrun_error(self) -> bool {
            self.0.get_bit(1)
        }

        /// Modify state of channel 3 buffer overrun error
        #[must_use]
        pub fn with_channel_3_buffer_overrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(1, error);
            self
        }

        /// Returns whether a buffer underrun occured on channel 3
        #[must_use]
        pub fn channel_3_buffer_underrun_error(self) -> bool {
            self.0.get_bit(0)
        }

        /// Modify state of channel 3 buffer underrrun error
        #[must_use]
        pub fn with_channel_3_buffer_underrun_error(mut self, error: bool) -> Self {
            self.0.set_bit(0, error);
            self
        }
    }

    /// External Interface Clock Lock Register
    ///
    /// This provides a way to unlock 32Mhz `EXI` clock functionality
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct ExternalClockLock(u32);

    impl Default for ExternalClockLock {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ExternalClockLock {
        /// Returns a new empty instance of the register
        #[must_use]
        pub fn new() -> Self {
            Self(0)
        }

        /// Reads the associated MMIO value
        #[must_use]
        pub fn read() -> Self {
            EXTERNAL_CLOCK_LOCK.read()
        }

        /// Writes self into the associated mmio value
        pub fn write(self) {
            EXTERNAL_CLOCK_LOCK.write(self);
        }

        /// Returns whether the 32 Mhz clock options is not available
        ///
        /// false = can do 32 Mhz | true = can not do 32 Mhz
        #[must_use]
        pub fn thirty_two_mhz_clock_unavailable(self) -> bool {
            self.0.get_bit(31)
        }

        /// Modify wether the 32 Mhz clock option is not available
        ///
        /// false  = can do 32 Mhz | true = can not do 32 Mhz
        #[must_use]
        pub fn with_thirty_two_mhz_clock_unavailable(mut self, lock: bool) -> Self {
            self.0.set_bit(31, lock);
            self
        }
    }
}
