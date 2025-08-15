/// E-Ticket System Supported Ioctls
pub enum Ioctl {
    /// Add Ticket to NAND
    AddTicket,
    /// Add Title Start
    AddTitleStart,
    /// Add Title Content Start
    AddContentStart,
    /// Add Title Content Data
    AddContentData,
    /// Add Title Content Finish
    AddContentFinish,
    /// Add Title Finish
    AddTitleFinish,
    /// Get Device ID
    GetDeviceId,
    /// Launch Title
    Launch,
    /// Open Active Title Content
    OpenActiveTitleContent,
    /// Read Content
    ReadContent,
    /// Close Content File Descriptor
    CloseContent,
    /// Get Owned Title Count
    GetOwnedTitleCount,
    /// Get Owned Title IDs
    GetOwnedTitles,
    /// Get Title Count
    GetTitleCount,
    /// Get Title IDs
    GetTitles,
    /// Get Title Content Count
    GetTitleContentsCount,
    /// Get Title Content IDs
    GetTitleContents,
    /// Get Title Ticket View Count
    GetTicketViewCount,
    /// Get Title Ticket Views
    GetTicketViews,
    /// Get Title Metadata View Size
    GetTitleMetadataViewSize,
    /// Get Title Metadata View
    GetTitleMetadataView,
    /// Get current `ES` context consumption
    GetConsumption,
    /// Delete Title
    DeleteTitle,
    /// Delete Ticket
    DeleteTicket,
    /// Get Title Metadata View Size
    DiskInterfaceGetTitleMetadataViewSize,
    /// Get Title Metadata View
    DiskInterfaceGetTitleMetadataView,
    /// Get Ticket View
    DiskInterfaceGetTicketView,
    /// Verify Title
    DiskInterfaceVerify,
    /// Get Data Directory
    GetTitleDir,
    /// Get Device Certificate
    GetDeviceCertificate,
    /// Import Boot 2
    ImportBoot,
    /// Get Current Title ID,
    GetTitleId,
    /// Set `ES` context User ID
    SetUid,
    /// Delete Title Content
    DeleteTitleContent,
    /// Seek Title Content
    SeekContent,
    /// Open Title Content
    OpenContent,
    /// Launch Backwards Compatibility (Gamecube Mode)
    LauchBackwardsCompatibility,
    /// Export Title Initialize
    ExportTitleInitalize,
    /// Export Content Begin
    ExportContentBegin,
    /// Export Content Data
    ExportContentData,
    /// Export Content End
    ExportContentEnd,
    /// Export  Title Done / Export Title End
    ExportTitleDone,
    /// Add Title Metadata
    AddTitleMetadata,
    /// Encrypt
    Encrypt,
    /// Decrpyt
    Decrypt,
    /// Get Boot 2 Version
    GetBoot2Version,
    /// Add Title Cancel
    AddTitleCancel,
    /// Sign
    Sign,
    /// Verify Sign
    VerifySign,
    /// Get Stored Contents Count
    GetStoredContentCount,
    /// Get Stored Contents
    GetStoredContents,
    /// Get Stored Title Metadata Size
    GetStoredTitleMetadataSize,
    /// Get Stored Title Metadata
    GetStoredTitleMetadata,
    /// Get Shared Contents Count
    GetSharedContentCount,
    /// Get Shared Contents
    GetSharedContents,
    /// Delete Shared Contents
    DeleteSharedContents,
    /// Disk Interface Get Title Metadata Size
    DiskInterfaceGetTitleMetadataSize,

    /// Disk Interface Get Title Metadata
    DiskInterfaceGetTitleMetadata,
    /// Disk Interface Verify With View
    DiskInterfaceVerifyWithView,
    /// Setup Stream Key
    SetupStreamKey,
    /// Delete Stream Key
    DeleteStreamKey,
    /// Delete Content
    DeleteContent,
    // Invalid3F
    /// Get Version 0 Ticket From View
    GetVersion0TicketFromView,
    // Unknown41,
    // Unknown42,
    /// Get Ticket Size From Ticket View
    GetTicketSizeFromView,
    /// Get Ticket from Ticket View
    GetTicketFromView,
    /// Check Korea Region
    CheckKoreaRegion,
}

impl From<Ioctl> for i32 {
    fn from(value: Ioctl) -> Self {
        match value {
            Ioctl::AddTicket => 1,
            Ioctl::AddTitleStart => 2,
            Ioctl::AddContentStart => 3,
            Ioctl::AddContentData => 4,
            Ioctl::AddContentFinish => 5,
            Ioctl::AddTitleFinish => 6,
            Ioctl::GetDeviceId => 7,
            Ioctl::Launch => 8,
            Ioctl::OpenActiveTitleContent => 9,
            Ioctl::ReadContent => 10,
            Ioctl::CloseContent => 11,
            Ioctl::GetOwnedTitleCount => 12,
            Ioctl::GetOwnedTitles => 13,
            Ioctl::GetTitleCount => 14,
            Ioctl::GetTitles => 15,
            Ioctl::GetTitleContentsCount => 16,
            Ioctl::GetTitleContents => 17,
            Ioctl::GetTicketViewCount => 18,
            Ioctl::GetTicketViews => 19,
            Ioctl::GetTitleMetadataViewSize => 20,
            Ioctl::GetTitleMetadataView => 21,
            Ioctl::GetConsumption => 22,
            Ioctl::DeleteTitle => 23,
            Ioctl::DeleteTicket => 24,
            Ioctl::DiskInterfaceGetTitleMetadataViewSize => 25,
            Ioctl::DiskInterfaceGetTitleMetadataView => 26,
            Ioctl::DiskInterfaceGetTicketView => 27,
            Ioctl::DiskInterfaceVerify => 28,
            Ioctl::GetTitleDir => 29,
            Ioctl::GetDeviceCertificate => 30,
            Ioctl::ImportBoot => 31,
            Ioctl::GetTitleId => 32,
            Ioctl::SetUid => 33,
            Ioctl::DeleteTitleContent => 34,
            Ioctl::SeekContent => 35,
            Ioctl::OpenContent => 36,
            Ioctl::LauchBackwardsCompatibility => 37,
            Ioctl::ExportTitleInitalize => 38,
            Ioctl::ExportContentBegin => 39,
            Ioctl::ExportContentData => 40,
            Ioctl::ExportContentEnd => 41,
            Ioctl::ExportTitleDone => 42,
            Ioctl::AddTitleMetadata => 43,
            Ioctl::Encrypt => 44,
            Ioctl::Decrypt => 45,
            Ioctl::GetBoot2Version => 46,
            Ioctl::AddTitleCancel => 47,
            Ioctl::Sign => 48,
            Ioctl::VerifySign => 49,
            Ioctl::GetStoredContentCount => 50,
            Ioctl::GetStoredContents => 51,
            Ioctl::GetStoredTitleMetadataSize => 52,
            Ioctl::GetStoredTitleMetadata => 53,
            Ioctl::GetSharedContentCount => 54,
            Ioctl::GetSharedContents => 55,
            Ioctl::DeleteSharedContents => 56,
            Ioctl::DiskInterfaceGetTitleMetadataSize => 57,
            Ioctl::DiskInterfaceGetTitleMetadata => 58,
            Ioctl::DiskInterfaceVerifyWithView => 59,
            Ioctl::SetupStreamKey => 60,
            Ioctl::DeleteStreamKey => 61,
            Ioctl::DeleteContent => 62,
            Ioctl::GetVersion0TicketFromView => 64,
            Ioctl::GetTicketSizeFromView => 67,
            Ioctl::GetTicketFromView => 68,
            Ioctl::CheckKoreaRegion => 69,
        }
    }
}

static DEV_ES: &CStr = c"/dev/es";

use core::ffi::CStr;

use alloc::{ffi::CString, vec::Vec};

use crate::ios::{self, FileDescriptor};

/// [`Ioctl::AddTicket`]
///
/// Add ticket, certificates and certificate revoke list to system
/// # Errors
/// See [`ios::Error`]
pub fn add_ticket(
    signed_ticket: &[u8],
    signed_certs: &[u8],
    signed_crl: &[u8],
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<3, 0, 3>(
        es,
        Ioctl::AddTicket,
        &[signed_ticket, signed_certs, signed_crl],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::AddTitleStart`]
///
/// Add title metadata, certificates and certifacte revoke list to system
/// Needs to be canceled with the same file descriptor that this function is called with
/// # Errors
/// See [`ios::Error`]
pub fn add_title_start(
    es: FileDescriptor,
    signed_title_meta: &[u8],
    signed_certs: &[u8],
    signed_crl: &[u8],
) -> Result<(), ios::Error> {
    ios::ioctlv::<4, 0, 4>(
        es,
        Ioctl::AddTitleStart,
        &[
            signed_title_meta,
            signed_certs,
            signed_crl,
            &1u32.to_be_bytes(),
        ],
        &mut [],
    )?;

    Ok(())
}

/// [`Ioctl::AddContentStart`]
///
/// Return content file descriptor for `title_id` and `content_id`
/// Needs to be finished with the same file descriptor that this function is called with
/// # Errors
/// See [`ios::Error`]
pub fn add_content_start(
    es: FileDescriptor,
    title_id: u64,
    content_id: u32,
) -> Result<i32, ios::Error> {
    let fd = ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::AddContentStart,
        &[&title_id.to_be_bytes(), &content_id.to_be_bytes()],
        &mut [],
    )?;

    Ok(fd)
}

/// [`Ioctl::AddContentData`]
///
/// Add data to content file descriptor
/// # Errors
/// See [`ios::Error`]
pub fn add_content_data(
    es: FileDescriptor,
    content_fd: i32,
    data: &[u8],
) -> Result<(), ios::Error> {
    ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::AddContentData,
        &[&content_fd.to_be_bytes(), data],
        &mut [],
    )?;

    Ok(())
}

/// [`Ioctl::AddContentFinish`]
///
/// Finish adding content data to content file descriptor
/// # Errors
/// See [`ios::Error`]
pub fn add_content_finish(es: FileDescriptor, content_fd: u32) -> Result<(), ios::Error> {
    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::AddContentFinish,
        &[&content_fd.to_be_bytes()],
        &mut [],
    )?;

    Ok(())
}

/// [`Ioctl::AddTitleFinish`]
///
/// Finish adding title to system
/// # Errors
/// See [`ios::Error`]
pub fn add_title_finish(es: FileDescriptor) -> Result<(), ios::Error> {
    ios::ioctlv::<0, 0, 0>(es, Ioctl::AddTitleFinish, &[], &mut [])?;

    Ok(())
}

/// [`Ioctl::GetDeviceId`]
///
/// Get device id
/// # Errors
/// See [`ios::Error`]
pub fn get_device_id() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetDeviceId, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

/// [`Ioctl::Launch`]
///
/// Launch system title
/// # Errors
/// See [`ios::Error`]
pub fn launch_title(title_id: u64, ticket_view: &[u8]) -> Result<!, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv_reboot::<2, 0, 2>(
        es,
        Ioctl::Launch,
        &[&title_id.to_be_bytes(), ticket_view],
        &mut [],
    )?;

    loop {}
}

/// [`Ioctl::OpenActiveTitleContent`]
///
/// Open content from current title
/// # Errors
/// See [`ios::Error`]
pub fn open_active_title_content(es: FileDescriptor, content_idx: u32) -> Result<i32, ios::Error> {
    let fd = ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::OpenActiveTitleContent,
        &[&content_idx.to_be_bytes()],
        &mut [],
    )?;

    Ok(fd)
}

/// [`Ioctl::ReadContent`]
///
/// Read data provided from `content_file_descriptor` into `out_buf`
/// # Errors
/// See [`ios::Error`]
pub fn read_content(
    es: FileDescriptor,
    content_file_descriptor: i32,
    out_buf: &mut [u8],
) -> Result<(), ios::Error> {
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::ReadContent,
        &[&content_file_descriptor.to_be_bytes()],
        &mut [out_buf],
    )?;

    Ok(())
}
/// [`Ioctl::CloseContent`]
///
/// Close content
/// # Errors
/// See [`ios::Error`]
pub fn close_content(es: FileDescriptor, content_file_descriptor: i32) -> Result<(), ios::Error> {
    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::CloseContent,
        &[&content_file_descriptor.to_be_bytes()],
        &mut [],
    )?;

    Ok(())
}

/// [`Ioctl::GetOwnedTitleCount`]
///
/// Get owned title count
/// # Errors
/// See [`ios::Error`]
pub fn get_owned_title_count() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetOwnedTitleCount, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}
/// [`Ioctl::GetOwnedTitles`]
///
/// Get ids for owned titles
/// # Errors
/// See [`ios::Error`]
pub fn get_owned_titles(title_count: u32) -> Result<Vec<u64>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    //TODO: Avoid allocation
    let mut out_buf = alloc::vec![0u8; core::mem::size_of::<u64>() * title_count as usize];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetOwnedTitles,
        &[&title_count.to_be_bytes()],
        &mut [out_buf.as_mut_slice()],
    )?;

    // TODO: Avoid allocation
    out_buf
        .chunks_exact(core::mem::size_of::<u64>())
        .map(|bytes| {
            if let Ok(bytes) = bytes.try_into() {
                Ok(u64::from_be_bytes(bytes))
            } else {
                Err(ios::Error::Invalid)
            }
        })
        .collect::<Result<Vec<_>, ios::Error>>()
}
/// [`Ioctl::GetTitleCount`]
///
/// Get title count
/// # Errors
/// See [`ios::Error`]
pub fn get_title_count() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetTitleCount, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

/// [`Ioctl::GetTitles`]
///
/// Get ids for all titles
/// # Errors
/// See [`ios::Error`]
pub fn get_titles(title_count: u32) -> Result<Vec<u64>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    // TODO: Avoid allocation
    let mut out_buf = alloc::vec![0u8; title_count as usize * core::mem::size_of::<u64>()];

    let count: [u8; 4] = title_count.to_be_bytes();
    ios::ioctlv::<1, 1, 2>(es, Ioctl::GetTitles, &[&count], &mut [&mut out_buf[..]])?;

    let _ = ios::close(es);

    // TODO: Avoid allocation
    out_buf
        .chunks_exact(core::mem::size_of::<u64>())
        .map(|bytes| {
            if let Ok(bytes) = bytes.try_into() {
                Ok(u64::from_be_bytes(bytes))
            } else {
                Err(ios::Error::Invalid)
            }
        })
        .collect::<Result<Vec<_>, ios::Error>>()
}

/// [`Ioctl::GetTitleContentsCount`]
///
/// Get title contents count for `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_title_contents_count(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetTitleContentsCount,
        &[&title_id.to_be_bytes()],
        &mut [&mut out_buf],
    )?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}
/// [`Ioctl::GetTitleContents`]
///
/// Get content ids of title with `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_title_contents(title_id: u64, content_count: u32) -> Result<Vec<u32>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    //TODO: avoid allocation
    let mut out_buf = alloc::vec![0u8; core::mem::size_of::<u32>() * content_count as usize];
    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::GetTitleContents,
        &[&title_id.to_be_bytes(), &content_count.to_be_bytes()],
        &mut [out_buf.as_mut_slice()],
    )?;

    let _ = ios::close(es);

    //TODO: avoid allocation
    out_buf
        .chunks_exact(core::mem::size_of::<u32>())
        .map(|bytes| {
            if let Ok(bytes) = bytes.try_into() {
                Ok(u32::from_be_bytes(bytes))
            } else {
                Err(ios::Error::Invalid)
            }
        })
        .collect::<Result<Vec<_>, ios::Error>>()
}

/// [`Ioctl::GetTicketViewCount`]
///
/// Get ticket view count of title with `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_ticket_view_count(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetTicketViewCount,
        &[&title_id.to_be_bytes()],
        &mut [&mut out_buf],
    )?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

// TODO: actually returns a Vec<TicketView> but I haven't made teh `TicketView` struct yet and
// don't want to do structs till the end of impling all these
//
/// [`Ioctl::GetTicketViews`]
///
/// Get ticket views for title with `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_ticket_views(title_id: u64, view_count: u32) -> Result<Vec<u8>, ios::Error> {
    const TICKET_VIEW_SIZE: usize = 216; // 0xD8

    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = alloc::vec![0u8; TICKET_VIEW_SIZE * view_count as usize];
    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::GetTicketViews,
        &[&title_id.to_be_bytes(), &view_count.to_be_bytes()],
        &mut [out_buf.as_mut_slice()],
    )?;

    let _ = ios::close(es);

    Ok(out_buf)
}
/// [`Ioctl::GetTitleMetadataViewSize`]
///
/// Get title metadata view size for  title with `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_title_metadata_view_size(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let title_id_in = title_id.to_be_bytes();
    let mut out_buf = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetTitleMetadataViewSize,
        &[&title_id_in],
        &mut [&mut out_buf],
    )?;

    let _ = ios::close(es);
    Ok(u32::from_be_bytes(out_buf))
}

//TODO: Return `TitleMetadataView` instead of owned allocation
//
/// [`Ioctl::GetTitleMetadataView`]
///
/// Get title metadata view  for title with `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_title_metadata_view(title_id: u64, size: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let title_id_in_buf = title_id.to_be_bytes();
    let size_in_buf = size.to_be_bytes();

    let size = usize::try_from(size).map_err(|_| ios::Error::Invalid)?;
    let mut out_buf = alloc::vec![0u8; size];

    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::GetTitleMetadataView,
        &[&title_id_in_buf, &size_in_buf],
        &mut [out_buf.as_mut_slice()],
    )?;

    let _ = ios::close(es);

    Ok(out_buf)
}

/// Get tiklimit consumption count
/// # Errors
/// See [`ios::Error`]
pub fn get_consumption_count(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let title_id_in_buf = title_id.to_be_bytes();
    let mut out_buf = [0u8; 4];
    ios::ioctlv::<1, 2, 3>(
        es,
        Ioctl::GetConsumption,
        &[&title_id_in_buf],
        &mut [&mut [], &mut out_buf],
    )?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

/// [`Ioctl::GetConsumption`]
///
/// Get tiklimit consumption
/// # Errors
/// See [`ios::Error`]
pub fn get_consumption(title_id: u64, limit_count: u32) -> Result<Vec<u8>, ios::Error> {
    const TIKLIMIT_SIZE: usize = 8;

    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let limit_count = usize::try_from(limit_count).map_err(|_| ios::Error::Invalid)?;

    let title_id_in_buf = title_id.to_be_bytes();
    let mut limit_out_buf = alloc::vec![0u8; TIKLIMIT_SIZE * limit_count];
    ios::ioctlv::<1, 2, 3>(
        es,
        Ioctl::GetConsumption,
        &[&title_id_in_buf],
        &mut [limit_out_buf.as_mut_slice(), &mut []],
    )?;

    let _ = ios::close(es);

    Ok(limit_out_buf)
}

/// [`Ioctl::DeleteTitle`]
///
/// Delete title from system
/// # Errors
/// See [`ios::Error`]
pub fn delete_title(title_id: u64) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(es, Ioctl::DeleteTitle, &[&title_id.to_be_bytes()], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl:::DeleteTicket`]
///
/// Delete ticket from system
/// # Errors
/// See [`ios::Error`]
pub fn delete_ticket(ticket_view: &[u8]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(es, Ioctl::DeleteTicket, &[ticket_view], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::DiskInterfaceGetTitleMetadataViewSize`]
///
/// Get current disk's title metadata view size
/// # Errors
/// See [`ios::Error`]
pub fn disk_interface_get_title_metadata_view_size(
    signed_title_meta: &[u8],
) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut size_buf: [u8; 4] = [0u8; 4];

    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::DiskInterfaceGetTitleMetadataViewSize,
        &[signed_title_meta],
        &mut [&mut size_buf],
    )?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(size_buf))
}

/// [`Ioctl::DiskInterfaceGetTitleMetadataView`]
///
/// Get current disk's title metadata view
/// # Errors
/// See [`ios::Error`]
pub fn disk_interface_get_title_metadata_view(
    signed_title_meta: &[u8],
    tmd_view_size: u32,
) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = alloc::vec![0u8; tmd_view_size as usize];

    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::DiskInterfaceGetTitleMetadataView,
        &[signed_title_meta, &tmd_view_size.to_be_bytes()],
        &mut [out_buf.as_mut_slice()],
    )?;

    let _ = ios::close(es);

    Ok(out_buf)
}

const TICKET_VIEW_SIZE: usize = 216; // 0xD8
/// [`Ioctl::DiskInterfaceGetTicketView`]
///
/// Get current disk's ticket view
/// # Errors
/// See [`ios::Error`]
pub fn disk_interface_get_ticket_view(
    signed_ticket: &[u8],
) -> Result<[u8; TICKET_VIEW_SIZE], ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;
    let mut out_buf = [0u8; TICKET_VIEW_SIZE];

    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::DiskInterfaceGetTicketView,
        &[signed_ticket],
        &mut [out_buf.as_mut_slice()],
    )?;

    let _ = ios::close(es);

    Ok(out_buf)
}

// pub fn disk_interface_verify

/// [`Ioctl::GetTitleDir`]
///
/// Get title with `title_id`'s data directory
/// # Errors
/// See [`ios::Error`]
pub fn get_data_directory(title_id: u64) -> Result<CString, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 32];

    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetTitleDir,
        &[&title_id.to_be_bytes()],
        &mut [&mut out_buf],
    )?;

    CStr::from_bytes_until_nul(&out_buf)
        .map(CString::from)
        .map_err(|_| ios::Error::Invalid)
}

const DEVICE_CERT_SIZE: usize = 384;
/// [`Ioctl::GetDeviceCertificate`]
///
/// Get this device's certificate
/// # Errors
/// See [`ios::Error`]
pub fn get_device_certificate() -> Result<[u8; DEVICE_CERT_SIZE], ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; DEVICE_CERT_SIZE];

    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetDeviceCertificate, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(out_buf)
}

// pub fn import_boot
/// [`Ioctl::GetTitleId`]
///
/// Get currently running title's `title_id`
/// # Errors
/// See [`ios::Error`]
pub fn get_title_id() -> Result<u64, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut title_id = [0u8; 8];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetTitleId, &[], &mut [&mut title_id])?;

    let _ = ios::close(es);

    Ok(u64::from_be_bytes(title_id))
}

/// [`Ioctl::SetUid`]
///
/// Set E-Ticket system user id
/// # Errors
/// See [`ios::Error`]
pub fn set_uid(uid: u64) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(es, Ioctl::SetUid, &[&uid.to_be_bytes()], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::DeleteTitleContent`]
///
/// Delete title with `title_id` contents
/// # Errors
/// See [`ios::Error`]
pub fn delete_title_content(title_id: u64) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::DeleteTitleContent,
        &[&title_id.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::SeekContent`]
///
/// Seek contents to `offset` using `seek_mode`
/// # Errors
/// See [`ios::Error`]
pub fn seek_content(
    content_file_descriptor: i32,
    seek_mode: ios::SeekMode,
    offset: i32,
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<3, 0, 3>(
        es,
        Ioctl::SeekContent,
        &[
            &content_file_descriptor.to_be_bytes(),
            &offset.to_be_bytes(),
            &i32::from(seek_mode).to_be_bytes(),
        ],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::OpenContent`]
///
/// Open title with `title_id` contents at `content_idx` using `ticket_views`
/// # Errors
/// See [`ios::Error`]
pub fn open_title_content(
    title_id: u64,
    ticket_views: &[u8],
    content_idx: u32,
) -> Result<i32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let fd = ios::ioctlv::<3, 0, 3>(
        es,
        Ioctl::OpenContent,
        &[
            &title_id.to_be_bytes(),
            ticket_views,
            &content_idx.to_be_bytes(),
        ],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(fd)
}

//pub fn launch_backwards_compat() -> Result<!, ios::Error> {}

/// [`Ioctl::ExportTitleInitalize`]
///
/// Export title with `title_id` metadata into `export_tmd_buf`
/// # Errors
/// See [`ios::Error`]
pub fn export_title_init(title_id: u64, exported_tmd_buf: &mut [u8]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::ExportTitleInitalize,
        &[&title_id.to_be_bytes()],
        &mut [exported_tmd_buf],
    )?;

    let _ = ios::close(es);
    Ok(())
}

/// [`Ioctl::ExportContentBegin`]
///
/// Open title with `title_id`'s content with `content_id`
/// # Errors
/// See [`ios::Error`]
pub fn export_content_begin(title_id: u64, content_id: u32) -> Result<i32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let fd = ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::ExportContentBegin,
        &[&title_id.to_be_bytes(), &content_id.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(fd)
}

/// [`Ioctl::ExportContentData`]
///
/// Export content data into `data` using the `content_file_descriptor` provived by
/// [`Ioctl::ExportContentBegin`]
/// # Errors
/// See [`ios::Error`]
pub fn export_content_data(
    content_file_descriptor: i32,
    data: &mut [u8],
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::ExportContentData,
        &[&content_file_descriptor.to_be_bytes()],
        &mut [data],
    )?;

    let _ = ios::close(es);

    Ok(())
}
/// [`Ioctl::ExportContentEnd`]
///
/// Close `content_file_descriptor` provided by [`Ioctl::ExportContentBegin`]
/// # Errors
/// See [`ios::Error`]
pub fn export_content_end(content_file_descriptor: i32) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::ExportContentEnd,
        &[&content_file_descriptor.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::ExportTitleDone`]
/// # Errors
/// See [`ios::Error`]
pub fn export_title_done() -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<0, 0, 0>(es, Ioctl::ExportTitleDone, &[], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::AddTitleMetadata`]
///
/// Add title metadata to system
/// # Errors
/// See [`ios::Error`]
pub fn add_tmd(title_meta: &[u8]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(es, Ioctl::AddTitleMetadata, &[title_meta], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

#[repr(u32)]
/// Key used to encrypt and decrypt date
///
/// Usually called `keynum` in libogc
pub enum Key {
    /// NAND Key
    NandFs = 2,
    /// Common Key
    Common = 4,
    /// Backup Key
    Backup = 5,
    /// SD Card Contents Key
    SdCard = 6,
    /// Korean Key
    Korean = 11,
}

/// [`Ioctl::Encrypt`]
///
/// Encrypt `source` with `iv` and `key` outputing to `destination`
/// # Errors
/// See [`ios::Error`]
pub fn encrypt(
    keynum: Key,
    iv: &mut [u8; 16],
    source: &[u8],
    destination: &mut [u8],
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;
    let iv_copy = *iv;

    ios::ioctlv::<3, 2, 5>(
        es,
        Ioctl::Encrypt,
        &[&(keynum as u32).to_be_bytes(), &iv_copy, source],
        &mut [iv, destination],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::Decrypt`]
///
/// Decrypt `source` with `iv` and `key` outputing to `destination`
/// # Errors
/// See [`ios::Error`]
pub fn decrypt(
    keynum: Key,
    iv: &mut [u8; 16],
    source: &[u8],
    destination: &mut [u8],
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;
    let iv_copy = *iv;

    ios::ioctlv::<3, 2, 5>(
        es,
        Ioctl::Decrypt,
        &[&(keynum as u32).to_be_bytes(), &iv_copy, source],
        &mut [iv, destination],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::GetBoot2Version`]
///
/// Get boot 2 version
/// # Errors
/// See [`ios::Error`]
pub fn get_boot_2_version() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut boot_version = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetBoot2Version, &[], &mut [&mut boot_version])?;

    let _ = ios::close(es);

    let boot_version = u32::from_be_bytes(boot_version);
    Ok(boot_version)
}

/// [`Ioctl::AddTitleCancel`]
///
/// Cancel add title to nand
/// # Errors
/// See [`ios::Error`]
pub fn cancel_add_title() -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<0, 0, 0>(es, Ioctl::AddTitleCancel, &[], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::Sign`]
///
/// Sign provided `data` returning a signature and certificate
/// # Errors
/// See [`ios::Error`]
pub fn sign(data: &[u8]) -> Result<([u8; 60], [u8; 0x180]), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut cert = [0u8; 0x180];
    let mut signature = [0u8; 60];
    ios::ioctlv::<1, 2, 3>(es, Ioctl::Sign, &[data], &mut [&mut signature, &mut cert])?;

    let _ = ios::close(es);
    Ok((signature, cert))
}

/// [`Ioctl::VerifySign`]
///
/// Taking in `data_sha1`, `signature` and `certs` verify if properly signed
/// # Errors
/// See [`ios::Error`]
pub fn verify_sign(data_sha1: &[u8], signature: &[u8], certs: &[u8]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<3, 0, 3>(
        es,
        Ioctl::VerifySign,
        &[data_sha1, signature, certs],
        &mut [],
    )?;

    let _ = ios::close(es);
    Ok(())
}

/// [`Ioctl::GetStoredContentCount`]
///
/// Get count of contents stored on the NAND
/// # Errors
/// See [`ios::Error`]
pub fn get_stored_contents_count(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut title_count = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetStoredContentCount,
        &[&title_id.to_be_bytes()],
        &mut [&mut title_count],
    )?;

    let _ = ios::close(es);

    let title_count = u32::from_be_bytes(title_count);
    Ok(title_count)
}

/// [`Ioctl::GetStoredContents`]
///
/// Get contents stored on the NAND
/// # Errors
/// See [`ios::Error`]
pub fn get_stored_contents(title_id: u64, content_count: u32) -> Result<Vec<u32>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut content_ids = alloc::vec![0u8; content_count as usize * core::mem::size_of::<u32>()];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetStoredContents,
        &[&title_id.to_be_bytes()],
        &mut [&mut content_ids],
    )?;

    let _ = ios::close(es);

    content_ids
        .chunks_exact(4)
        .map(|bytes| {
            if let Ok(bytes) = bytes.try_into() {
                Ok(u32::from_be_bytes(bytes))
            } else {
                Err(ios::Error::Invalid)
            }
        })
        .collect::<Result<Vec<_>, ios::Error>>()
}

/// [`Ioctl::GetStoredTitleMetadataSize`]
///
/// Get stored title metadata size of `title_id` title
/// # Errors
/// See [`ios::Error`]
pub fn get_stored_title_metadata_size(title_id: u64) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let in_buf = title_id.to_be_bytes();
    let mut out_buf = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetStoredTitleMetadataSize,
        &[&in_buf],
        &mut [&mut out_buf],
    )?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

// TODO: Proper enuming since there are different signature types and differing sizes for them
//
/// [`Ioctl::GetStoredTitleMetadata`]
///
/// Get stored title metadata of `title_id` title
/// # Errors
/// See [`ios::Error`]
pub fn get_stored_title_metadata(title_id: u64, size: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let title_buf = title_id.to_be_bytes();
    let size_buf = size.to_be_bytes();
    let size_usize: usize = usize::try_from(size).map_err(|_| ios::Error::Invalid)?;
    // TODO: Avoid allocation
    let mut out_buf = alloc::vec![0u8; size_usize];
    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::GetStoredTitleMetadata,
        &[&title_buf, &size_buf],
        &mut [&mut out_buf[..]],
    )?;

    let _ = ios::close(es);

    Ok(out_buf)
}

/// [`Ioctl::GetSharedContentCount`]
///
/// Get shared contents count on NAND
/// # Errors
/// See [`ios::Error`]
pub fn get_shared_contents_count() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut shared_contents_count: [u8; 4] = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(
        es,
        Ioctl::GetSharedContentCount,
        &[],
        &mut [&mut shared_contents_count],
    )?;

    let _ = ios::close(es);

    let shared_contents_count = u32::from_be_bytes(shared_contents_count);
    Ok(shared_contents_count)
}

/// [`Ioctl::GetSharedContents`]
///
/// Get shared contents sha1 hashes on NAND
/// # Errors
/// See [`ios::Error`]
pub fn get_shared_contents(shared_contents_count: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut sha1_hashes = alloc::vec![0u8; 20 * shared_contents_count as usize];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetSharedContents,
        &[&shared_contents_count.to_be_bytes()],
        &mut [&mut sha1_hashes],
    )?;

    let _ = ios::close(es);

    Ok(sha1_hashes)
}

/// [`Ioctl::DeleteSharedContents`]
///
/// Delete shared content based on the provided `sha1_hash`
/// # Errors
/// See [`ios::Error`]
pub fn delete_shared_content(sha1_hash: &[u8; 20]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(es, Ioctl::DeleteSharedContents, &[sha1_hash], &mut [])?;
    Ok(())
}

/// [`Ioctl::DiskInterfaceGetTitleMetadataSize`]
///
/// Get disk title metadata size
/// # Errors
/// See [`ios::Error`]
pub fn disk_interface_get_title_metadata_size() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut tmd_size: [u8; 4] = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(
        es,
        Ioctl::DiskInterfaceGetTitleMetadataSize,
        &[],
        &mut [&mut tmd_size],
    )?;

    let _ = ios::close(es);

    let tmd_size = u32::from_be_bytes(tmd_size);
    Ok(tmd_size)
}

/// [`Ioctl::DiskInterfaceGetTitleMetadata`]
///
/// Get disk title metadata
/// # Errors
/// See [`ios::Error`]
pub fn disk_interface_get_title_metadata(size: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut tmd = alloc::vec![0u8; size as usize];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::DiskInterfaceGetTitleMetadata,
        &[&size.to_be_bytes()],
        &mut [&mut tmd],
    )?;

    let _ = ios::close(es);

    Ok(tmd)
}

// pub fn disk_interface_verify_with_view

/// [`Ioctl::SetupStreamKey`]
///
/// Setup stream key
/// # Errors
/// See [`ios::Error`]
pub fn setup_stream_key(tik_view: &[u8], tmd: &[u8]) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut handle = [0u8; 4];
    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::SetupStreamKey,
        &[tik_view, tmd],
        &mut [&mut handle],
    )?;

    let _ = ios::close(es);

    let handle = u32::from_be_bytes(handle);
    Ok(handle)
}

/// [`Ioctl::DeleteStreamKey`]
///
/// Delete stream key
/// # Errors
/// See [`ios::Error`]
pub fn delete_stream_key(handle: u32) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::DeleteStreamKey,
        &[&handle.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

/// [`Ioctl::DeleteContent`]
///
/// Delete `title_id` title's content using `content_id`
/// # Errors
/// See [`ios::Error`]
pub fn delete_content(title_id: u64, content_id: u32) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::DeleteContent,
        &[&title_id.to_be_bytes(), &content_id.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

const TICKET_SIZE: usize = 0x2A4;
/// [`Ioctl::GetVersion0TicketFromView`]
///
/// Get version ticket from provided `tik_view`
/// # Errors
/// See [`ios::Error`]
pub fn get_version_0_ticket_from_view(tik_view: &[u8]) -> Result<[u8; TICKET_SIZE], ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut ticket = [0u8; TICKET_SIZE];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetVersion0TicketFromView,
        &[tik_view],
        &mut [&mut ticket],
    )?;

    let _ = ios::close(es);

    Ok(ticket)
}

/// [`Ioctl::GetTicketFromView`]
///
/// Get ticket size from provided `tik_view`
/// # Errors
/// See [`ios::Error`]
pub fn get_ticket_size_from_view(tik_view: &[u8]) -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut size = [0u8; 4];
    ios::ioctlv::<1, 1, 2>(
        es,
        Ioctl::GetTicketSizeFromView,
        &[tik_view],
        &mut [&mut size],
    )?;

    let _ = ios::close(es);

    let size = u32::from_be_bytes(size);
    Ok(size)
}

/// [`Ioctl::GetTicketFromView`]
///
/// Get ticket from provided `tik_view`
/// # Errors
/// See [`ios::Error`]
pub fn get_ticket_from_view(tik_view: &[u8], size: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut ticket = alloc::vec![0u8; size as usize];
    ios::ioctlv::<2, 1, 3>(
        es,
        Ioctl::GetTicketFromView,
        &[tik_view, &size.to_be_bytes()],
        &mut [&mut ticket],
    )?;

    let _ = ios::close(es);

    Ok(ticket)
}

/// [`Ioctl::CheckKoreaRegion`]
///
/// Check if the console's region is Korea
/// # Errors
/// See [`ios::Error`]
pub fn check_korea_region() -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<0, 0, 0>(es, Ioctl::CheckKoreaRegion, &[], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}
