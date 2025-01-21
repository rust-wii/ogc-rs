use core::cell::Cell;

use bit_field::BitField;
use critical_section::Mutex;

use crate::mmio::serial_interface::{
    types::{
        CommuicationStatus, InputBufferHigh, InputBufferLow, OutputBuffer, PollingRegister, Status,
    },
    INPUT_OUTPUT_BUFFER,
};

#[derive(Debug, Copy, Clone)]
pub enum Channel {
    Zero,
    One,
    Two,
    Three,
}

impl From<Channel> for u32 {
    fn from(value: Channel) -> Self {
        match value {
            Channel::Zero => 0,
            Channel::One => 1,
            Channel::Two => 2,
            Channel::Three => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    NoResponse,
    Collision,
    BufferUnderrun,
    BufferOverrun,
    Unknown,
    NoAvailableChannel,
}

pub fn transfer(channel: Channel, input_buf: &[u8], output_buf: &mut [u8]) -> Result<(), Error> {
    // Output = Us
    // Input = Controller
    let command_transfer = CommuicationStatus::new()
        .with_channel(channel.into())
        .with_output_length(
            u32::try_from(input_buf.len()).expect("input buf length is larger then u32::MAX"),
        )
        .with_input_length(
            u32::try_from(output_buf.len()).expect("output buf length is larger then u32::MAX"),
        )
        .with_transfer_start(true);

    for (index, chunk) in input_buf.chunks(4).enumerate() {
        if let Some(addr) = INPUT_OUTPUT_BUFFER.get(index) {
            addr.write(u32::from_be_bytes([
                *chunk.first().unwrap_or(&0),
                *chunk.get(1).unwrap_or(&0),
                *chunk.get(2).unwrap_or(&0),
                *chunk.get(3).unwrap_or(&0),
            ]));
        }
    }

    command_transfer.write();
    while CommuicationStatus::read().transfer_start() {
        core::hint::spin_loop();
    }

    match CommuicationStatus::read().communication_error() {
        false => {
            for (index, chunk) in output_buf.chunks_mut(4).enumerate() {
                if let Some(addr) = INPUT_OUTPUT_BUFFER.get(index) {
                    let bytes = addr.read().to_be_bytes();

                    if let Some(val) = chunk.get_mut(0) {
                        *val = bytes[0]
                    }

                    if let Some(val) = chunk.get_mut(1) {
                        *val = bytes[1]
                    }

                    if let Some(val) = chunk.get_mut(2) {
                        *val = bytes[2]
                    }

                    if let Some(val) = chunk.get_mut(3) {
                        *val = bytes[3]
                    }
                }
            }

            Ok(())
        }
        true => status_error(channel),
    }
}

fn status_error(channel: Channel) -> Result<(), Error> {
    let status = Status::read();
    match channel {
        Channel::Zero => {
            if status.channel_0_no_response_error() {
                Err(Error::NoResponse)
            } else if status.channel_0_collision_error() {
                Err(Error::Collision)
            } else if status.channel_0_buffer_underrun_error() {
                Err(Error::BufferUnderrun)
            } else if status.channel_0_buffer_overrun_error() {
                Err(Error::BufferOverrun)
            } else {
                Err(Error::Unknown)
            }
        }
        Channel::One => {
            if status.channel_1_no_response_error() {
                Err(Error::NoResponse)
            } else if status.channel_1_collision_error() {
                Err(Error::Collision)
            } else if status.channel_1_buffer_underrrun_error() {
                Err(Error::BufferUnderrun)
            } else if status.channel_1_buffer_overrun_error() {
                Err(Error::BufferOverrun)
            } else {
                Err(Error::Unknown)
            }
        }
        Channel::Two => {
            if status.channel_2_no_response_error() {
                Err(Error::NoResponse)
            } else if status.channel_2_collision_error() {
                Err(Error::Collision)
            } else if status.channel_2_buffer_underrun_error() {
                Err(Error::BufferUnderrun)
            } else if status.channel_2_buffer_overrun_error() {
                Err(Error::BufferOverrun)
            } else {
                Err(Error::Unknown)
            }
        }
        Channel::Three => {
            if status.channel_3_no_response_error() {
                Err(Error::NoResponse)
            } else if status.channel_3_collision_error() {
                Err(Error::Collision)
            } else if status.channel_3_buffer_underrun_error() {
                Err(Error::BufferUnderrun)
            } else if status.channel_3_buffer_overrun_error() {
                Err(Error::BufferOverrun)
            } else {
                Err(Error::Unknown)
            }
        }
    }
}

pub fn get_type(channel: Channel) -> Result<[u8; 3], Error> {
    const CMD_TYPE_AND_STATUS: u8 = 0;
    let mut data = [0u8; 3];

    transfer(
        channel,
        core::slice::from_ref(&CMD_TYPE_AND_STATUS),
        &mut data,
    )
    .map(|()| Ok(data))?
}

pub fn get_origin(channel: Channel) -> Result<[u8; 10], Error> {
    const CMD_READ_ORIGIN: u8 = 0x41;
    let mut data = [0u8; 10];

    transfer(channel, core::slice::from_ref(&CMD_READ_ORIGIN), &mut data).map(|()| Ok(data))?
}

pub fn recalibrate(channel: Channel) -> Result<[u8; 10], Error> {
    const CMD_RECALIBRATE: u8 = 0x42;
    let mut data = [0u8; 10];

    transfer(channel, core::slice::from_ref(&CMD_RECALIBRATE), &mut data).map(|()| Ok(data))?
}

pub fn set_analog_mode(channel: Channel, mode: u8) -> Result<(), Error> {
    debug_assert!(mode < 8);
    let command = OutputBuffer::new()
        .with_command_opcode(0x40)
        .with_output_zero(mode.into());
    match channel {
        Channel::Zero => command.write_zero(),
        Channel::One => command.write_one(),
        Channel::Two => command.write_two(),
        Channel::Three => command.write_three(),
    }

    let status = Status::read();
    status.with_output_buffer_write(true).write();

    Ok(())
}

#[derive(Debug)]
pub struct Controller {
    channel: Channel,
}

#[derive(Debug)]
pub struct State {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub start: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,
    pub dpad_up: bool,
    pub z: bool,
    pub r: bool,
    pub l: bool,
    pub stick_x: i8,
    pub stick_y: i8,
    pub sub_stick_x: i8,
    pub sub_stick_y: i8,
    pub analog_l: u8,
    pub analog_r: u8,
}

static AVAILABLE_CHANNELS: Mutex<Cell<[Option<Channel>; 4]>> = Mutex::new(Cell::new([
    Some(Channel::Zero),
    Some(Channel::One),
    Some(Channel::Two),
    Some(Channel::Three),
]));

impl Controller {
    pub fn new() -> Result<Self, Error> {
        let mut si = None;
        critical_section::with(|cs| {
            let mut available_channels = AVAILABLE_CHANNELS.borrow(cs).get();

            available_channels.iter_mut().for_each(|channel| {
                if channel.is_some() && get_type(channel.unwrap()).is_ok() {
                    si = channel.take();
                }
            });

            AVAILABLE_CHANNELS.borrow(cs).set(available_channels);
        });

        if si.is_none() {
            return Err(Error::NoAvailableChannel);
        }

        let channel = si.unwrap();
        let _type = get_type(channel)?;
        set_analog_mode(channel, 3)?;

        //Enable polling for channel
        //Copy on Write instead of Copy on Vblank
        let poll = PollingRegister::read();
        match channel {
            Channel::Zero => {
                poll.with_channel_0_copy_on_vblank(false)
                    .with_channel_0_enable(true)
                    .write();
            }
            Channel::One => {
                poll.with_channel_1_copy_on_vblank(false)
                    .with_channel_1_enable(true)
                    .write();
            }
            Channel::Two => {
                poll.with_channel_2_copy_on_vblank(false)
                    .with_channel_2_enable(true)
                    .write();
            }
            Channel::Three => {
                poll.with_channel_3_copy_on_vblank(false)
                    .with_channel_3_enable(true)
                    .write();
            }
        }

        Ok(Self { channel })
    }

    pub fn raw(&self) -> [u8; 8] {
        let input_high = match self.channel {
            Channel::Zero => InputBufferHigh::read_zero(),
            Channel::One => InputBufferHigh::read_one(),
            Channel::Two => InputBufferHigh::read_two(),
            Channel::Three => InputBufferHigh::read_three(),
        };

        debug_assert!(
            !input_high.error_status() && !input_high.error_latch(),
            "Should not be able to get an error from just reading data in theory",
        );

        let input_low = match self.channel {
            Channel::Zero => InputBufferLow::read_zero(),
            Channel::One => InputBufferLow::read_one(),
            Channel::Two => InputBufferLow::read_two(),
            Channel::Three => InputBufferLow::read_three(),
        };

        [
            u8::try_from(input_high.input_zero()).unwrap(),
            u8::try_from(input_high.input_one()).unwrap(),
            u8::try_from(input_high.input_two()).unwrap(),
            u8::try_from(input_high.input_three()).unwrap(),
            u8::try_from(input_low.input_four()).unwrap(),
            u8::try_from(input_low.input_five()).unwrap(),
            u8::try_from(input_low.input_six()).unwrap(),
            u8::try_from(input_low.input_seven()).unwrap(),
        ]
    }

    pub fn state(&self) -> State {
        let raw = self.raw();
        Self::state_from_bytes(raw)
    }

    fn remap_stick_state(state: u8) -> i8 {
        i8::try_from(i16::from(state) - 128).unwrap()
    }

    fn state_from_bytes(raw: [u8; 8]) -> State {
        State {
            a: raw[0].get_bit(0),
            b: raw[0].get_bit(1),
            x: raw[0].get_bit(2),
            y: raw[0].get_bit(3),
            start: raw[0].get_bit(4),
            dpad_left: raw[1].get_bit(0),
            dpad_right: raw[1].get_bit(1),
            dpad_down: raw[1].get_bit(2),
            dpad_up: raw[1].get_bit(3),
            z: raw[1].get_bit(4),
            r: raw[1].get_bit(5),
            l: raw[1].get_bit(6),
            stick_x: Self::remap_stick_state(raw[2]),
            stick_y: Self::remap_stick_state(raw[3]),
            sub_stick_x: Self::remap_stick_state(raw[4]),
            sub_stick_y: Self::remap_stick_state(raw[5]),
            analog_l: raw[6],
            analog_r: raw[7],
        }
    }

    pub fn read(&self) -> Result<State, Error> {
        let status = Status::read();
        let has_new_data = match self.channel {
            Channel::Zero => status.channel_0_read_status(),
            Channel::One => status.channel_1_read_status(),
            Channel::Two => status.channel_2_read_status(),
            Channel::Three => status.channel_3_read_status(),
        };

        let input_high = match self.channel {
            Channel::Zero => InputBufferHigh::read_zero(),
            Channel::One => InputBufferHigh::read_one(),
            Channel::Two => InputBufferHigh::read_two(),
            Channel::Three => InputBufferHigh::read_three(),
        };
        match !has_new_data && !input_high.error_status() && !input_high.error_latch() {
            false => Err(status_error(self.channel).expect_err("Didn't get a known error")),
            true => {
                let input_low = match self.channel {
                    Channel::Zero => InputBufferLow::read_zero(),
                    Channel::One => InputBufferLow::read_one(),
                    Channel::Two => InputBufferLow::read_two(),
                    Channel::Three => InputBufferLow::read_three(),
                };

                Ok(Self::state_from_bytes([
                    u8::try_from(input_high.input_zero()).unwrap(),
                    u8::try_from(input_high.input_one()).unwrap(),
                    u8::try_from(input_high.input_two()).unwrap(),
                    u8::try_from(input_high.input_three()).unwrap(),
                    u8::try_from(input_low.input_four()).unwrap(),
                    u8::try_from(input_low.input_five()).unwrap(),
                    u8::try_from(input_low.input_six()).unwrap(),
                    u8::try_from(input_low.input_seven()).unwrap(),
                ]))
            }
        }
    }

    pub fn copy_on_vblank(&mut self) -> &mut Self {
        let poll = PollingRegister::read();

        match self.channel {
            Channel::Zero => {
                poll.with_channel_0_copy_on_vblank(true).write();
            }
            Channel::One => {
                poll.with_channel_1_copy_on_vblank(true).write();
            }
            Channel::Two => {
                poll.with_channel_2_copy_on_vblank(true).write();
            }
            Channel::Three => {
                poll.with_channel_3_copy_on_vblank(true).write();
            }
        }

        self
    }
}
