use crate::ios;

/// SDIO supported Ioctls
pub enum Ioctl {
    /// Write to a SD host controller register
    WriteHostControllerRegister,
    /// Read from a SD host controller register
    ReadHostControllerRegister,
    /// Reset SD card
    ResetSDCard,
    /// Set SD card clock
    SetClock,
    /// Send SDIO command
    SendCommand,
    /// Get SD card status
    GetStatus,
    /// Get operating conditions register
    GetOperatingConditionsRegister,
}

impl From<Ioctl> for i32 {
    fn from(value: Ioctl) -> Self {
        match value {
            Ioctl::WriteHostControllerRegister => 1,
            Ioctl::ReadHostControllerRegister => 2,
            Ioctl::ResetSDCard => 4,
            Ioctl::SetClock => 6,
            Ioctl::SendCommand => 7,
            Ioctl::GetStatus => 11,
            Ioctl::GetOperatingConditionsRegister => 12,
        }
    }
}
/// Try from Ioctl Error
/// This happens when you don't provide a proper i32 to map to an [`Ioctl`]
pub struct TryFromIoctlError;
impl TryFrom<i32> for Ioctl {
    type Error = TryFromIoctlError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WriteHostControllerRegister),
            2 => Ok(Self::ReadHostControllerRegister),
            4 => Ok(Self::ResetSDCard),
            6 => Ok(Self::SetClock),
            7 => Ok(Self::SendCommand),
            11 => Ok(Self::GetStatus),
            12 => Ok(Self::GetOperatingConditionsRegister),
            _ => Err(TryFromIoctlError),
        }
    }
}

/// Write to a SD host controller register
pub fn write_to_host_controller_register(
    register: u8,
    size: u8,
    data: u32,
) -> Result<(), ios::Error> {
    let sdio = ios::open(c"/dev/sdio/slot0", ios::Mode::None)?;

    let mut buffer = [0u8; 24];
    buffer[0..4].copy_from_slice(&u32::from(register).to_be_bytes());
    buffer[12..16].copy_from_slice(&u32::from(size).to_be_bytes());
    buffer[16..20].copy_from_slice(&data.to_be_bytes());

    ios::ioctl(sdio, Ioctl::WriteHostControllerRegister, &buffer, &mut [])?;

    let _ = ios::close(sdio);
    Ok(())
}

/// Read from a SD host controller register
pub fn read_from_host_controller_register(register: u8, size: u8) -> Result<u32, ios::Error> {
    let sdio = ios::open(c"/dev/sdio/slot0", ios::Mode::None)?;

    let mut value = [0u8; 4];
    let mut query = [0u8; 24];
    query[0..4].copy_from_slice(&u32::from(register).to_be_bytes());
    query[12..16].copy_from_slice(&u32::from(size).to_be_bytes());

    ios::ioctl(sdio, Ioctl::ReadHostControllerRegister, &query, &mut value)?;

    let _ = ios::close(sdio);
    Ok(u32::from_be_bytes(value))
}
/// Read SD card status returning the relative card address
pub fn get_sdcard_status() -> Result<u16, ios::Error> {
    let sdio = ios::open(c"/dev/sdio/slot0", ios::Mode::None)?;

    let mut buffer = [0u8; 4];
    ios::ioctl(sdio, Ioctl::GetStatus, &[], &mut buffer)?;

    let _ = ios::close(sdio);
    Ok(u16::from_be_bytes(
        buffer[0..2].try_into().map_err(|_| ios::Error::Invalid)?,
    ))
}

// /// Set SD card clock
//pub fn set_clock
