use core::ffi::CStr;

#[repr(u32)]
/// Interprocess Control / IOS File Mode
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
#[derive(Copy, Clone, Debug)]
/// Interprocess Control / IOS Errors
pub enum Error {
    /// A invalid argument was provided.
    Invalid = -4,
    /// No IPC heap is currently allocated
    NoHeap = -5,
    /// No file entry was found.
    NoEntry = -6,
    /// The IPC queue is full. This is non-fatal.
    QueueFull = -8,
    /// Could not allocate an IPC request.
    NoMemory = -22,
    /// The provided file path was too long.
    FilePathLengthTooLong,
}

impl TryFrom<i32> for Error {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -4 => Ok(Self::Invalid),
            -5 => Ok(Self::NoHeap),
            -6 => Ok(Self::NoEntry),
            -8 => Ok(Self::QueueFull),
            -22 => Ok(Self::NoMemory),
            _ => Err(()),
        }
    }
}

/// Interprocess Control / IOS File Descriptor
///
/// Represents either a device in the case of something under the `/dev/` file path or an actual
/// file handle.
///
#[derive(Copy, Clone, Debug)]
pub struct FileDescriptor(i32);

/// Attempt to open a file from IOS
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
///
pub fn open(file_path: &CStr, file_mode: Mode) -> Result<FileDescriptor, Error> {
    if file_path.count_bytes() + 1 > 64 {
        return Err(Error::FilePathLengthTooLong);
    }

    match unsafe { ogc_sys::IOS_Open(file_path.as_ptr().cast(), file_mode.into()) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val if { val >= 0 } => Ok(FileDescriptor(val)),
        val => panic!("{val:?}: Unknown error code reached"),
    }
}
/// Attempts to close an open file descriptor
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
///
pub fn close(fd: FileDescriptor) -> Result<(), Error> {
    match unsafe { ogc_sys::IOS_Close(fd.0) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val if { val >= 0 } => Ok(()),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}

/// Attempts to read bytes from a file descriptor into a buffer.
///
/// Attempts to read up to `buf.len()` bytes into `buf` from `fd`. Returns the amount of bytes read.
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
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
        val if { val >= 0 } => Ok(usize::try_from(val).expect("val did not fit in a `usize`")),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}

/// Attempts to writes bytes into a file descriptor from a buffer.
///
/// Attempts to write up to `buf.len()` bytes into `fd` Returns the amount of bytes written.
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
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
        val if { val >= 0 } => Ok(usize::try_from(val).expect("val did not fit in a `usize`")),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}

/// Interprocess Control / IOS Seek Mode
#[repr(i32)]
pub enum SeekMode {
    /// Seek from the start
    Start = 0,
    /// Seek from current position
    Current = 1,
    /// Seek from the end
    End = 2,
}

impl From<SeekMode> for i32 {
    fn from(val: SeekMode) -> i32 {
        match val {
            SeekMode::Start => 0,
            SeekMode::Current => 1,
            SeekMode::End => 2,
        }
    }
}
/// Attempts to seek to a certain position within a file descriptor
///
/// Attempts to seek to `offset` from `mode` in `fd`
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
pub fn seek(fd: FileDescriptor, offset: i32, mode: SeekMode) -> Result<(), Error> {
    match unsafe { ogc_sys::IOS_Seek(fd.0, offset, mode.into()) } {
        val if { val == -4 || val == -5 || val == -6 || val == -8 || val == -22 } => {
            Err(Error::try_from(val).expect("Error does not contain a known error code"))
        }
        val if { val >= 0 } => Ok(()),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}

/// Attempts to call an ioctl using a file descriptor with an in buffer and out buffer
///
/// Attempts to call `ioctl` with `fd` using `buf_in` and `buf_out`
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is encountered.
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
        val if { val >= 0 } => Ok(()),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}

/// Attempts to call ioctl using a file descriptor with multiple input and output buffers
///
/// Attempts to call `ioctl` using `fd` with `bufs_in` and `bufs_out`
///
/// # Errors
/// See [`Error`]
///
/// # Panics
/// Panics if an unknown error code is reached.
pub fn ioctlv<
    const COUNT_IN: usize,
    const COUNT_OUT: usize,
    //Invariant: This must be COUNT_IN + COUNT_OUT (waiting for `generic_const_exprs` to be
    //stabilizied)
    const COUNT_IN_OUT: usize,
    IOCTL: Into<i32>,
>(
    fd: FileDescriptor,
    ioctl: IOCTL,
    buf_ins: &[&[u8]],
    buf_outs: &mut [&mut [u8]],
) -> Result<(), Error> {
    type Ioctlv = ogc_sys::_ioctlv;
    debug_assert!(buf_ins.len() == COUNT_IN);
    debug_assert!(buf_outs.len() == COUNT_OUT);
    debug_assert!(COUNT_IN + COUNT_OUT == COUNT_IN_OUT);

    let mut ioctls = [Ioctlv {
        data: core::ptr::null_mut(),
        len: 0,
    }; COUNT_IN_OUT];
    //SAFETY: I promise that i don't modify the contents of in buffers up to COUNT_IN
    for (i, buf_in) in buf_ins.iter().enumerate() {
        ioctls[i] = Ioctlv {
            data: buf_in.as_ptr().cast_mut().cast(),
            len: buf_in
                .len()
                .try_into()
                .expect("In buffer length exceeds u32::MAX"),
        }
    }

    for (i, buf_out) in buf_outs.iter_mut().enumerate() {
        ioctls[COUNT_IN + i] = Ioctlv {
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
        val if { val >= 0 } => Ok(()),
        val => panic!("{val:?} : Unknown error code reached"),
    }
}
