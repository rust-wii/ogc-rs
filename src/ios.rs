use core::ffi::CStr;

#[repr(u32)]
pub enum Mode {
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

impl From<Mode> for u32 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::None => 0,
            Mode::Read => 1,
            Mode::Write => 2,
            Mode::ReadWrite => 3,
        }
    }
}

#[repr(i32)]
pub enum Error {
    Invalid = -4,
    NoHeap = -5,
    NoEntry = -6,
    QueueFull = -8,
    NoMemory = -22,
    FilePathLengthTooLong,
}

impl TryFrom<i32> for Error {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -4 => Ok(Self::Invalid),
            -8 => Ok(Self::QueueFull),
            -22 => Ok(Self::NoMemory),
            _ => Err(()),
        }
    }
}

pub struct FileDescriptor(i32);

pub fn open(file_path: &CStr, file_mode: Mode) -> Result<FileDescriptor, Error> {
    if file_path.to_bytes_with_nul().len() > 64 {
        return Err(Error::FilePathLengthTooLong);
    }

    match unsafe { ogc_sys::IOS_Open(file_path.as_ptr().cast(), file_mode.into()) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val => Ok(FileDescriptor(val)),
    }
}

pub fn close(fd: FileDescriptor) -> Result<(), Error> {
    match unsafe { ogc_sys::IOS_Close(fd.0) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        _ => Ok(()),
    }
}

pub fn read(fd: FileDescriptor, buf: &mut [u8]) -> Result<usize, Error> {
    let (ptr, len) = (buf.as_mut_ptr(), buf.len());
    match unsafe {
        ogc_sys::IOS_Read(
            fd.0,
            ptr.cast(),
            len.try_into().expect("buffer length exceeds i32::MAX"),
        )
    } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val => Ok(usize::try_from(val).expect("val did not fit in a `usize`")),
    }
}

pub fn write(fd: FileDescriptor, buf: &[u8]) -> Result<usize, Error> {
    let (ptr, len) = (buf.as_ptr(), buf.len());
    match unsafe {
        ogc_sys::IOS_Write(
            fd.0,
            ptr.cast(),
            len.try_into().expect("buffer length exceeds i32::MAX"),
        )
    } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val => Ok(usize::try_from(val).expect("val did not fit in a `usize`")),
    }
}

#[repr(i32)]
pub enum SeekMode {
    Set = 0,
    Current = 1,
    End = 2,
}

impl From<SeekMode> for i32 {
    fn from(val: SeekMode) -> i32 {
        match val {
            SeekMode::Set => 0,
            SeekMode::Current => 1,
            SeekMode::End => 2,
        }
    }
}

pub fn seek(fd: FileDescriptor, offset: i32, mode: SeekMode) -> Result<(), Error> {
    match unsafe { ogc_sys::IOS_Seek(fd.0, offset, mode.into()) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        _ => Ok(()),
    }
}

pub fn ioctl<IOCTL: Into<i32>>(
    fd: FileDescriptor,
    ioctl: IOCTL,
    buf_in: &[u8],
    buf_out: &mut [u8],
) -> Result<(), Error> {
    let io_s32 = ioctl.into();
    let (in_ptr, in_len) = (buf_in.as_ptr(), buf_in.len());
    let (out_ptr, out_len) = (buf_out.as_mut_ptr(), buf_out.len());
    // SAFETY: I promise in_buf does not get modified
    match unsafe {
        ogc_sys::IOS_Ioctl(
            fd.0,
            io_s32,
            in_ptr.cast_mut().cast(),
            in_len
                .try_into()
                .expect("In buffer's length exceeds i32::MAX"),
            out_ptr.cast(),
            out_len
                .try_into()
                .expect("Out buffer length exceeds i32::MAX"),
        )
    } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        _ => Ok(()),
    }
}

pub fn ioctlv<const COUNT_IN: usize, const COUNT_OUT: usize, IOCTL: Into<i32>>(
    fd: FileDescriptor,
    ioctl: IOCTL,
    buf_ins: &[&[u8]],
    buf_outs: &mut [&mut [u8]],
) -> Result<(), Error>
where
    [(); COUNT_IN + COUNT_OUT]:,
{
    debug_assert!(buf_ins.len() == COUNT_IN);
    debug_assert!(buf_outs.len() == COUNT_OUT);

    let mut ioctls = [ogc_sys::_ioctlv {
        data: core::ptr::null_mut(),
        len: 0,
    }; COUNT_IN + COUNT_OUT];

    //SAFETY: I promise that i don't modify the contents of in buffers up to COUNT_IN
    for (i, buf_in) in buf_ins.iter().enumerate() {
        ioctls[i] = ogc_sys::_ioctlv {
            data: buf_in.as_ptr().cast_mut().cast(),
            len: buf_in
                .len()
                .try_into()
                .expect("In buffer length exceeds u32::MAX"),
        }
    }

    for (i, buf_out) in buf_outs.iter_mut().enumerate() {
        ioctls[COUNT_IN + i] = ogc_sys::_ioctlv {
            data: buf_out.as_mut_ptr().cast(),
            len: buf_out
                .len()
                .try_into()
                .expect("Out buffer length excceds u32::MAX"),
        }
    }

    match unsafe {
        ogc_sys::IOS_Ioctlv(
            fd.0,
            ioctl.into(),
            COUNT_IN.try_into().expect("count in exceeds i32::MAX"),
            COUNT_OUT.try_into().expect("count out exceeds i32::MAX"),
            ioctls.as_ptr().cast_mut(),
        )
    } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        _ => Ok(()),
    }
}
