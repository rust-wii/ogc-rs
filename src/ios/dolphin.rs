use core::ffi::CStr;

use alloc::{borrow::ToOwned, ffi::CString};

use crate::ios::{self, Mode};

pub enum Ioctl {
    GetElapsedTime,
    GetVersion,
    GetSpeedLimit,
    SetSpeedLimit,
    GetCpuSpeed,
    GetRealProductCode,
    DiscordSetClient,
    DiscordSetPresence,
    DiscordReset,
    GetSystemTime,
}

impl From<Ioctl> for i32 {
    fn from(value: Ioctl) -> Self {
        match value {
            Ioctl::GetElapsedTime => 1,
            Ioctl::GetVersion => 2,
            Ioctl::GetSpeedLimit => 3,
            Ioctl::SetSpeedLimit => 4,
            Ioctl::GetCpuSpeed => 5,
            Ioctl::GetRealProductCode => 6,
            Ioctl::DiscordSetClient => 7,
            Ioctl::DiscordSetPresence => 8,
            Ioctl::DiscordReset => 9,
            Ioctl::GetSystemTime => 10,
        }
    }
}

//Get elapsed time since the emulation started in milliseconds.
pub fn get_elapsed_time() -> Result<u32, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf: [u8; 4] = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetElapsedTime, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(u32::from_be_bytes(buf))
}

//TODO: Figure out a way to get out of the allocation since I know the max size it can be?
pub fn get_version() -> Result<CString, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf = [0u8; 64];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetVersion, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(CStr::from_bytes_until_nul(&buf).unwrap().to_owned())
}

pub fn get_speed_limit() -> Result<u32, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf: [u8; 4] = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetSpeedLimit, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(u32::from_be_bytes(buf))
}

pub fn set_speed_limit(speed_limit: u32) -> Result<(), ios::Error> {
    //NOTE: This is arbitrary I just don't think some one wants to set anything higher then this
    //value currently
    debug_assert!(
        (0..=500).contains(&speed_limit),
        "Speed limit must be in in the range 0 to 500 inclusive"
    );

    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let bytes = speed_limit.to_be_bytes();
    ios::ioctlv::<1, 0, 1>(dolphin, Ioctl::SetSpeedLimit, &[&bytes], &mut [])?;

    let _ = ios::close(dolphin);
    Ok(())
}

pub fn get_cpu_speed() -> Result<u32, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetCpuSpeed, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(u32::from_be_bytes(buf))
}
//TODO: Figure out a way to get out of the allocation since I know the max size it can be?
pub fn get_product_code() -> Result<CString, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf = [0u8; 64];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetRealProductCode, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(CStr::from_bytes_until_nul(&buf).unwrap().to_owned())
}

pub fn set_discord_client(client_id: &CStr) -> Result<(), ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    ios::ioctlv::<1, 0, 1>(
        dolphin,
        Ioctl::DiscordSetClient,
        &[client_id.to_bytes()],
        &mut [],
    )?;

    let _ = ios::close(dolphin);
    Ok(())
}

pub struct ImageDetails<'a> {
    image_key: &'a CStr,
    image_text: &'a CStr,
}

impl<'a> ImageDetails<'a> {
    pub fn new(key: &'a CStr, text: &'a CStr) -> Self {
        Self {
            image_key: key,
            image_text: text,
        }
    }
}

pub struct Timestamps {
    start: u64,
    end: u64,
}

impl Timestamps {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

pub fn set_discord_presence(
    details: &CStr,
    state: &CStr,
    large_image_details: ImageDetails,
    small_image_details: ImageDetails,
    start_end_timestamps: Timestamps,
    party_size: u32,
    max_party_size: u32,
) -> Result<(), ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    ios::ioctlv::<10, 0, 10>(
        dolphin,
        Ioctl::DiscordSetPresence,
        &[
            details.to_bytes_with_nul(),
            state.to_bytes_with_nul(),
            large_image_details.image_key.to_bytes_with_nul(),
            large_image_details.image_text.to_bytes_with_nul(),
            small_image_details.image_key.to_bytes_with_nul(),
            small_image_details.image_text.to_bytes_with_nul(),
            &start_end_timestamps.start.to_be_bytes(),
            &start_end_timestamps.end.to_be_bytes(),
            &party_size.to_be_bytes(),
            &max_party_size.to_be_bytes(),
        ],
        &mut [],
    )?;

    let _ = ios::close(dolphin);
    Ok(())
}

//Get system time since UNIX_EPOCH
pub fn get_system_time() -> Result<u64, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf: [u8; 8] = [0u8; 8];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetSystemTime, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(u64::from_be_bytes(buf))
}
