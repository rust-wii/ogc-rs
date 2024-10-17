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
    DiscordSetClient,   //TODO: Unimplemented Currently
    DiscordSetPresence, //TODO: Unimplemented Currently
    DiscordReset,       //TODO: Unimplemented Currently,
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

//Get system time since UNIX_EPOCH
pub fn get_system_time() -> Result<u64, ios::Error> {
    let dolphin = ios::open(c"/dev/dolphin", Mode::ReadWrite)?;

    let mut buf: [u8; 8] = [0u8; 8];
    ios::ioctlv::<0, 1, 1>(dolphin, Ioctl::GetSystemTime, &[], &mut [&mut buf])?;

    let _ = ios::close(dolphin);
    Ok(u64::from_be_bytes(buf))
}
