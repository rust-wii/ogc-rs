use core::ffi::CStr;

use alloc::{borrow::ToOwned, vec::Vec};

use crate::ios::{self, Mode};

/// Filesystem Supported Ioctls
pub enum Ioctl {
    /// Format the NAND
    Format,
    /// Get Nand Stats
    GetStats,
    /// Create a Directory
    CreateDirectory,
    /// Read a Directory
    ReadDirectory,
    /// Set File or Directory Attributes
    SetAttributes,
    /// Get File or Directory Attributes
    GetAttributes,
    /// Delete a File or Directory
    Delete,
    /// Rename a File or Directory
    Rename,
    /// Create a File
    CreateFile,
    /// UNKNOWN
    SetFileVersionControl,
    /// Get File Stats
    GetFileStats,
    /// Get NAND usage
    GetUsage,
    /// Shutdown `/dev/fs` IOS device
    Shutdown,
}

impl From<Ioctl> for i32 {
    fn from(value: Ioctl) -> Self {
        match value {
            Ioctl::Format => 1,
            Ioctl::GetStats => 2,
            Ioctl::CreateDirectory => 3,
            Ioctl::ReadDirectory => 4,
            Ioctl::SetAttributes => 5,
            Ioctl::GetAttributes => 6,
            Ioctl::Delete => 7,
            Ioctl::Rename => 8,
            Ioctl::CreateFile => 9,
            Ioctl::SetFileVersionControl => 10,
            Ioctl::GetFileStats => 11,
            Ioctl::GetUsage => 12,
            Ioctl::Shutdown => 13,
        }
    }
}

pub enum Error {
    Invalid,
    AccessDenied,
    SuperblockWriteFailed,
    SuperblockInitFailed,
    AlreadyExists,
    NotFound,
    FstFull,
    NoFreeSpace,
    NoFreeHandle,
    TooManyPathComponents,
    InUse,
    BadBlock,
    EccError,
    CriticalEccError,
    FileNotEmpty,
    CheckFailed,
    UnknownError,
    ShortRead,
}

/// Format the NAND.
///
/// You must be `uid` 0 for this to complete.
/// **YOU SHOULD PROBABLY NOT DO THIS**
/// # Errors
/// See [`ios::Error`]
pub fn format() -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    ios::ioctl(filesystem, Ioctl::Format, &[], &mut [])?;

    let _ = ios::close(filesystem);
    Ok(())
}

/// Current NAND Statistics
pub struct NandStats {
    cluster_size: u32,
    free_clusters: u32,
    used_clusters: u32,
    bad_clusters: u32,
    reversed_clusters: u32,
    free_inodes: u32,
    used_inodes: u32,
}

/// Get current Nand Statistics
/// # Errors
/// See [`ios::Error`]
pub fn get_nand_stats() -> Result<NandStats, ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut buf = [0u8; 28];
    ios::ioctl(filesystem, Ioctl::GetStats, &[], &mut buf)?;

    let _ = ios::close(filesystem);

    Ok(NandStats {
        cluster_size: u32::from_be_bytes(buf[0..4].try_into().map_err(|_| ios::Error::Invalid)?),
        free_clusters: u32::from_be_bytes(buf[4..8].try_into().map_err(|_| ios::Error::Invalid)?),
        used_clusters: u32::from_be_bytes(buf[8..12].try_into().map_err(|_| ios::Error::Invalid)?),
        bad_clusters: u32::from_be_bytes(buf[12..16].try_into().map_err(|_| ios::Error::Invalid)?),
        reversed_clusters: u32::from_be_bytes(
            buf[16..20].try_into().map_err(|_| ios::Error::Invalid)?,
        ),
        free_inodes: u32::from_be_bytes(buf[20..24].try_into().map_err(|_| ios::Error::Invalid)?),
        used_inodes: u32::from_be_bytes(buf[24..28].try_into().map_err(|_| ios::Error::Invalid)?),
    })
}

/// Filesystem Attributes
///
/// These are the same on both directories and files.
pub struct Attributes {
    uid: u32,
    gid: u16,
    path: [u8; 64],
    owner_mode: Mode,
    group_mode: Mode,
    other_mode: Mode,
    attribute: u8,
}

/// Create a Directory using `params`
/// # Errors
/// See [`ios::Error`]
pub fn create_directory(params: Attributes) -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 74];
    in_buf[0..4].copy_from_slice(&params.uid.to_be_bytes());
    in_buf[4..6].copy_from_slice(&params.gid.to_be_bytes());
    in_buf[6..70].copy_from_slice(&params.path);
    in_buf[70] = params.owner_mode.into();
    in_buf[71] = params.group_mode.into();
    in_buf[72] = params.other_mode.into();
    in_buf[73] = params.attribute;

    ios::ioctl(filesystem, Ioctl::CreateDirectory, &in_buf, &mut [])?;

    let _ = ios::close(filesystem);

    Ok(())
}

/// [`read_directory`] return values
///
/// This contains a list of names and the directory file count
pub struct ReadDirectory {
    file_list_buf: Vec<u8>,
    file_count: u32,
}

//TODO: Find a way to avoid allocation
/// Read the directory specified by `directory_path` reading up to `MAX_FILE_COUNT` entries
/// # Errors
/// See [`ios::Error`]
pub fn read_directory<const MAX_FILE_COUNT: usize>(
    directory_path: &str,
) -> Result<ReadDirectory, ios::Error>
where
    [(); 13 * MAX_FILE_COUNT]:,
{
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut path = [0u8; 64];
    let path_len = directory_path.as_bytes().len();
    path[0..path_len].copy_from_slice(directory_path.as_bytes());

    let mut file_list_buf = [0u8; 13 * MAX_FILE_COUNT];
    let mut file_count_buf = [0u8; 4];

    ios::ioctlv::<2, 2, 4>(
        filesystem,
        Ioctl::ReadDirectory,
        &[&path, &MAX_FILE_COUNT.to_be_bytes()],
        &mut [&mut file_list_buf, &mut file_count_buf],
    )?;

    let _ = ios::close(filesystem);

    let actual_file_count = u32::from_be_bytes(file_count_buf) * 13;
    let file_list_buf = &file_list_buf[0..actual_file_count
        .try_into()
        .map_err(|_| ios::Error::Invalid)?];

    Ok(ReadDirectory {
        file_list_buf: file_list_buf.to_owned(),
        file_count: actual_file_count,
    })
}
/// Set Filesystem Attributes
/// # Errors
/// See [`ios::Error`]
pub fn set_attributes(attributes: Attributes) -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 74];
    in_buf[0..4].copy_from_slice(&attributes.uid.to_be_bytes());
    in_buf[4..6].copy_from_slice(&attributes.gid.to_be_bytes());
    in_buf[6..70].copy_from_slice(&attributes.path);
    in_buf[70] = attributes.owner_mode.into();
    in_buf[71] = attributes.group_mode.into();
    in_buf[72] = attributes.other_mode.into();
    in_buf[73] = attributes.attribute;

    ios::ioctl(filesystem, Ioctl::SetAttributes, &in_buf, &mut [])?;

    let _ = ios::close(filesystem);

    Ok(())
}

/// Get Filesystem Attributes
/// # Errors
/// See [`ios::Error`]
pub fn get_attributes(name: &str) -> Result<Attributes, ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 64];
    in_buf[0..name.as_bytes().len()].copy_from_slice(name.as_bytes());

    let mut out_buf = [0u8; 74];
    ios::ioctl(filesystem, Ioctl::GetAttributes, &in_buf, &mut out_buf)?;

    let _ = ios::close(filesystem);

    Ok(Attributes {
        uid: u32::from_be_bytes(out_buf[0..4].try_into().map_err(|_| ios::Error::Invalid)?),
        gid: u16::from_be_bytes(out_buf[4..6].try_into().map_err(|_| ios::Error::Invalid)?),
        path: out_buf[6..70].try_into().map_err(|_| ios::Error::Invalid)?,
        owner_mode: out_buf[70].try_into().map_err(|()| ios::Error::Invalid)?,
        group_mode: out_buf[71].try_into().map_err(|()| ios::Error::Invalid)?,
        other_mode: out_buf[72].try_into().map_err(|()| ios::Error::Invalid)?,
        attribute: out_buf[73],
    })
}

/// Delete Directory or File
/// # Errors
/// See [`ios::Error`]
pub fn delete(name: &str) -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 64];
    in_buf[0..name.as_bytes().len()].copy_from_slice(name.as_bytes());

    ios::ioctl(filesystem, Ioctl::Delete, &in_buf, &mut [])?;

    let _ = ios::close(filesystem);

    Ok(())
}

/// Rename Directory or File
/// # Errors
/// See [`ios::Error`]
pub fn rename(source_name: &str, destination_name: &str) -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 128];
    in_buf[0..source_name.as_bytes().len()].copy_from_slice(source_name.as_bytes());
    in_buf[64..64 + destination_name.as_bytes().len()].copy_from_slice(destination_name.as_bytes());

    ios::ioctl(filesystem, Ioctl::Rename, &in_buf, &mut [])?;
    let _ = ios::close(filesystem);

    Ok(())
}

/// Create File
/// # Errors
/// See [`ios::Error`]
pub fn create_file(params: Attributes) -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 74];
    in_buf[0..4].copy_from_slice(&params.uid.to_be_bytes());
    in_buf[4..6].copy_from_slice(&params.gid.to_be_bytes());
    in_buf[6..70].copy_from_slice(&params.path);
    in_buf[70] = params.owner_mode.into();
    in_buf[71] = params.group_mode.into();
    in_buf[72] = params.other_mode.into();
    in_buf[73] = params.attribute;

    ios::ioctl(filesystem, Ioctl::CreateFile, &in_buf, &mut [])?;

    let _ = ios::close(filesystem);

    Ok(())
}

/// File Statistics
pub struct FileStats {
    file_size: u32,
    file_seek_position: u32,
}

/// Read file statistics of `file_name`
/// # Errors
/// See [`ios::Error`]
pub fn read_file_stats(file_name: &str) -> Result<FileStats, ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut file_buf = [0u8; 64];
    file_buf[0..file_name.as_bytes().len()].copy_from_slice(file_name.as_bytes());
    let file_name = CStr::from_bytes_with_nul(&file_buf[0..file_name.len() + 1])
        .map_err(|_| ios::Error::Invalid)?;

    let file = ios::open(file_name, Mode::ReadWrite)?;

    let mut out_buf = [0u8; 8];

    ios::ioctl(file, Ioctl::GetFileStats, &[], &mut out_buf)?;

    let _ = ios::close(file);
    let _ = ios::close(filesystem);

    Ok(FileStats {
        file_size: u32::from_be_bytes(out_buf[0..4].try_into().map_err(|_| ios::Error::Invalid)?),
        file_seek_position: u32::from_be_bytes(
            out_buf[4..8].try_into().map_err(|_| ios::Error::Invalid)?,
        ),
    })
}

/// NAND Usage
pub struct Usage {
    used_clusters: u32,
    used_inodes: u32,
}

/// Get current NAND usage
/// # Errors
/// See [`ios::Error`]
pub fn get_usage(name: &str) -> Result<Usage, ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    let mut in_buf = [0u8; 64];
    in_buf[0..name.as_bytes().len()].copy_from_slice(name.as_bytes());

    let mut used_clusters_buf = [0u8; 4];
    let mut used_inodes_buf = [0u8; 4];

    ios::ioctlv::<1, 2, 3>(
        filesystem,
        Ioctl::GetUsage,
        &[&in_buf],
        &mut [&mut used_clusters_buf, &mut used_inodes_buf],
    )?;

    let _ = ios::close(filesystem);

    Ok(Usage {
        used_clusters: u32::from_be_bytes(used_clusters_buf),
        used_inodes: u32::from_be_bytes(used_inodes_buf),
    })
}

/// Shutdown `/dev/fs` IOS device
/// # Errors
/// See [`ios::Error`]
pub fn shutdown() -> Result<(), ios::Error> {
    let filesystem = ios::open(c"/dev/fs", Mode::ReadWrite)?;

    ios::ioctl(filesystem, Ioctl::Shutdown, &[], &mut [])?;

    let _ = ios::close(filesystem);

    Ok(())
}
