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
//#[repr(C, align(32))]
/// SDIO request
pub struct Request {
    command: u32,
    command_type: u32,
    response_type: u32,
    arg: u32,
    block_count: u32,
    block_size: u32,
    dma_addr: *mut u8,
    is_dma: u32,
    pad0: u32,
}

impl Request {
    const SDIO_CMD_GO_IDLE: u32 = 0;
    const SDIO_RESPONSE_TYPE_R6: u32 = 7;
    const SDIO_CMD_SEND_IF_COND: u32 = 8;

    const SDIO_CMD_APPCMD: u32 = 55;
    const SDIO_CMD_SELECT: u32 = 7;
    const SDIO_CMD_SEND_CID: u32 = 2;
    const SDIO_CMD_SEND_RCA: u32 = 3;
    const SDIO_APPCMD_SENDOPCOND: u32 = 41;
    const SDIO_APPCMD_SET_BUS_WIDTH: u32 = 6;
    const SDIO_CMD_SET_BLOCK_LENGTH: u32 = 16;

    const SDIO_CMD_TYPE_AC: u32 = 3;

    const SDIO_RESPONSE_TYPE_R1: u32 = 1;
    const SDIO_RESPONSE_TYPE_R2: u32 = 3;
    const SDIO_RESPONSE_TYPE_R5: u32 = 6;
    const SDIO_RESPONSE_TYPE_R1B: u32 = 2;
    const SDIO_RESPONSE_TYPE_R3: u32 = 4;

    /// SDIO_CMD_GO_IDLE
    pub const GO_IDLE: Request =
        Request::new(Self::SDIO_CMD_GO_IDLE, 0, 0, 0, 0, 0, core::ptr::null_mut());

    /// SDIO_CMD_SEND_IF_COND
    pub const SEND_IF_COND: Request = Request::new(
        Self::SDIO_CMD_SEND_IF_COND,
        0,
        Self::SDIO_RESPONSE_TYPE_R6,
        0x1AA,
        0,
        0,
        core::ptr::null_mut(),
    );

    /// SDIO_CMD_APPCMD
    pub const APP_CMD: Request = Request::new(
        Self::SDIO_CMD_APPCMD,
        Self::SDIO_CMD_TYPE_AC,
        Self::SDIO_RESPONSE_TYPE_R1,
        0,
        0,
        0,
        core::ptr::null_mut(),
    );

    /// SDIO_CMD_APPCMD_SEND_OP_COND
    pub const SEND_OP_COND: Request = Request::new(
        Self::SDIO_APPCMD_SENDOPCOND,
        0,
        Self::SDIO_RESPONSE_TYPE_R3,
        0x40300000,
        0,
        0,
        core::ptr::null_mut(),
    );

    /// SDIO_CMD_DESELECT
    pub const DE_SELECT: Request = Request::new(
        Self::SDIO_CMD_SELECT,
        Self::SDIO_CMD_TYPE_AC,
        Self::SDIO_RESPONSE_TYPE_R1B,
        0,
        0,
        0,
        core::ptr::null_mut(),
    );

    /// SDIO_CMD_SENDRCA
    pub const SEND_RCA: Request = Request::new(
        Self::SDIO_CMD_SEND_RCA,
        0,
        Self::SDIO_RESPONSE_TYPE_R5,
        0,
        0,
        0,
        core::ptr::null_mut(),
    );

    /// SDIO_CMD_APPCMD_SET_BUS_WIDTH
    pub const fn set_bus_width(width: u32) -> Request {
        Request::new(
            Self::SDIO_APPCMD_SET_BUS_WIDTH,
            Self::SDIO_CMD_TYPE_AC,
            Self::SDIO_RESPONSE_TYPE_R1,
            width,
            0,
            0,
            core::ptr::null_mut(),
        )
    }

    /// SDIO_CMD_SEND_CID
    pub const fn send_cid(rca: u32) -> Request {
        Request::new(
            Self::SDIO_CMD_SEND_CID,
            0,
            Self::SDIO_RESPONSE_TYPE_R2,
            rca,
            0,
            0,
            core::ptr::null_mut(),
        )
    }

    /// SDIO_CMD_SET_BLOCK_LENGTH
    pub const fn set_block_length(length: u32) -> Request {
        Request::new(
            Self::SDIO_CMD_SET_BLOCK_LENGTH,
            Self::SDIO_CMD_TYPE_AC,
            Self::SDIO_RESPONSE_TYPE_R1,
            length,
            0,
            0,
            core::ptr::null_mut(),
        )
    }
    pub const fn select(rca: u32) -> Request {
        Request::new(
            Self::SDIO_CMD_SELECT,
            Self::SDIO_CMD_TYPE_AC,
            Self::SDIO_RESPONSE_TYPE_R1B,
            rca,
            0,
            0,
            core::ptr::null_mut(),
        )
    }

    pub const fn appcmd_with_rca(rca: u32) -> Request {
        Request::new(
            Self::SDIO_CMD_APPCMD,
            Self::SDIO_CMD_TYPE_AC,
            Self::SDIO_RESPONSE_TYPE_R1,
            rca,
            0,
            0,
            core::ptr::null_mut(),
        )
    }

    /// Create a new `Request` for the SDIO device
    pub const fn new(
        command: u32,
        command_type: u32,
        response_type: u32,
        arg: u32,
        block_count: u32,
        block_size: u32,
        dma_addr: *mut u8,
    ) -> Self {
        let is_dma = !dma_addr.is_null() as u32;
        Self {
            command,
            command_type,
            response_type,
            arg,
            block_count,
            block_size,
            dma_addr,
            is_dma,
            pad0: 0,
        }
    }
}

//#[repr(C, align(32))]
/// SDIO response
#[derive(Debug)]
pub struct Response {
    pub rsp_field0: u32,
    pub rsp_field1: u32,
    pub rsp_field2: u32,
    pub acmd12_response: u32,
}

pub use dev::Device;
mod dev {
    use core::mem::ManuallyDrop;

    use crate::ios::{
        self,
        sdio::{Ioctl, Request, Response},
        FileDescriptor,
    };

    type RawFd = core::ffi::c_int;

    struct ValidRawFd {
        fd: RawFd,
    }
    impl ValidRawFd {
        pub fn new(fd: RawFd) -> Option<Self> {
            if fd.is_positive() || fd == 0 {
                Some(ValidRawFd { fd })
            } else {
                None
            }
        }

        pub fn as_raw_fd(&self) -> RawFd {
            self.fd
        }

        pub fn into_raw_fd(self) -> RawFd {
            self.fd
        }
    }

    struct OwnedFd {
        fd: ValidRawFd,
    }

    impl OwnedFd {
        pub unsafe fn from_raw_fd(fd: RawFd) -> Option<Self> {
            ValidRawFd::new(fd).map(|fd| OwnedFd { fd })
        }

        pub fn into_raw_fd(self) -> RawFd {
            ManuallyDrop::new(self).fd.as_raw_fd()
        }

        pub fn as_file_descriptor(&self) -> FileDescriptor {
            FileDescriptor(self.fd.as_raw_fd())
        }
    }

    impl Drop for OwnedFd {
        fn drop(&mut self) {
            let _ = ios::close(ios::FileDescriptor(self.fd.as_raw_fd()));
        }
    }

    /// `/dev/sdio/slot0` Device
    pub struct Device {
        fd: OwnedFd,
    }

    impl Device {
        /// Try to open `/dev/sdio/slot0`
        pub fn open() -> Result<Self, ios::Error> {
            let sdio = ios::open(c"/dev/sdio/slot0", ios::Mode::Read)?;
            let fd = unsafe { OwnedFd::from_raw_fd(sdio.0) }.ok_or(ios::Error::Invalid)?;
            Ok(Self { fd })
        }

        /// Write to a SD host controller register
        pub fn write_to_host_controller_register(
            &mut self,
            register: u8,
            size: u8,
            data: u32,
        ) -> Result<(), ios::Error> {
            let mut buffer = [0u8; 24];
            buffer[0..4].copy_from_slice(&u32::from(register).to_be_bytes());
            buffer[12..16].copy_from_slice(&u32::from(size).to_be_bytes());
            buffer[16..20].copy_from_slice(&data.to_be_bytes());

            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::WriteHostControllerRegister,
                &buffer,
                &mut [],
            )?;

            Ok(())
        }

        /// Read from a SD host controller register
        pub fn read_from_host_controller_register(
            &mut self,
            register: u8,
            size: u8,
        ) -> Result<u32, ios::Error> {
            let mut value = [0u8; 4];
            let mut query = [0u8; 24];
            query[0..4].copy_from_slice(&u32::from(register).to_be_bytes());
            query[12..16].copy_from_slice(&u32::from(size).to_be_bytes());

            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::ReadHostControllerRegister,
                &query,
                &mut value,
            )?;

            Ok(u32::from_be_bytes(value))
        }
        /// Reset SD card
        pub fn reset(&mut self) -> Result<u32, ios::Error> {
            let mut buffer = [0u8; 4];
            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::ResetSDCard,
                &[],
                &mut buffer,
            )?;

            Ok(u32::from_be_bytes(buffer))
        }

        /// Enable SD card clock
        pub fn enable_clock(&mut self, enable: bool) -> Result<(), ios::Error> {
            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::SetClock,
                &u32::from(enable).to_be_bytes(),
                &mut [],
            )?;

            Ok(())
        }

        /// Send SDIO command
        pub fn send_command(&self, request: &Request) -> Result<Response, ios::Error> {
            let mut in_buf: [u8; _] = [0u8; core::mem::size_of::<Request>()];
            in_buf[0..4].copy_from_slice(&request.command.to_be_bytes());
            in_buf[4..8].copy_from_slice(&request.command_type.to_be_bytes());
            in_buf[8..12].copy_from_slice(&request.response_type.to_be_bytes());
            in_buf[12..16].copy_from_slice(&request.arg.to_be_bytes());
            in_buf[16..20].copy_from_slice(&request.block_count.to_be_bytes());
            in_buf[20..24].copy_from_slice(&request.block_size.to_be_bytes());
            in_buf[24..28].copy_from_slice(&request.dma_addr.expose_provenance().to_be_bytes());
            in_buf[28..32].copy_from_slice(&request.is_dma.to_be_bytes());
            //in_buf[32..36].copy_from_slice(&request.pad0.to_be_bytes());

            let mut out_buf: [u8; _] = [0u8; core::mem::size_of::<Response>()];

            if !request.dma_addr.is_null() && request.is_dma != 0 {
                let dma_bytes = unsafe {
                    core::slice::from_raw_parts(
                        request.dma_addr,
                        usize::try_from(request.block_count * request.block_size)
                            .map_err(|_| ios::Error::Invalid)?,
                    )
                };

                ios::ioctlv::<2, 1, 3>(
                    self.fd.as_file_descriptor(),
                    Ioctl::SendCommand,
                    &[&in_buf, dma_bytes],
                    &mut [&mut out_buf],
                )?;
            } else {
                ios::ioctl(
                    self.fd.as_file_descriptor(),
                    Ioctl::SendCommand,
                    &in_buf,
                    &mut out_buf,
                )?;
            }
            let resp = Response {
                rsp_field0: u32::from_be_bytes(
                    out_buf[0..4].try_into().map_err(|_| ios::Error::Invalid)?,
                ),
                rsp_field1: u32::from_be_bytes(
                    out_buf[4..8].try_into().map_err(|_| ios::Error::Invalid)?,
                ),
                rsp_field2: u32::from_be_bytes(
                    out_buf[8..12].try_into().map_err(|_| ios::Error::Invalid)?,
                ),
                acmd12_response: u32::from_be_bytes(
                    out_buf[12..16]
                        .try_into()
                        .map_err(|_| ios::Error::Invalid)?,
                ),
            };

            Ok(resp)
        }

        /// Read SD card status returning the relative card address
        pub fn get_status(&mut self) -> Result<u32, ios::Error> {
            let mut buffer = [0u8; 4];
            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::GetStatus,
                &[],
                &mut buffer,
            )?;

            Ok(u32::from_be_bytes(
                buffer[0..4].try_into().map_err(|_| ios::Error::Invalid)?,
            ))
        }

        /// Get operation conditions register
        pub fn get_operating_conditions_register(&mut self) -> Result<u32, ios::Error> {
            let mut buffer = [0u8; 4];
            ios::ioctl(
                self.fd.as_file_descriptor(),
                Ioctl::GetOperatingConditionsRegister,
                &[],
                &mut buffer,
            )?;

            Ok(u32::from_be_bytes(buffer))
        }
    }
}
