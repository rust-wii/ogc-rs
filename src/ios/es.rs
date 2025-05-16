/// E-Ticket System Supported Ioctls
pub enum Ioctl {
    AddTicket,
    AddTitleStart,
    AddContentStart,
    AddContentData,
    AddContentFinish,
    AddTitleFinish,
    GetDeviceId,
    Launch,
    OpenActiveTitleContent,
    ReadContent,
    CloseContent,
    GetOwnedTitleCount,
    GetOwnedTitles,
    GetTitleCount,
    GetTitles,
    GetTitleContentsCount,
    GetTitleContents,
    GetViewCount,
    GetViews,
    GetTicketViewCount,
    GetTicketViews,
    GetConsumption,
    DeleteTitle,
    DeleteTicket,
    DiskInterfaceGetTitleMetadataViewSize,
    DiskInterfaceGetTitleMetadataView,
    DiskInterfaceGetTicketView,
    DiskInterfaceVerify,
    GetTitleDir,
    GetDeviceCertificate,
    ImportBoot,
    GetTitleId,
    SetUid,
    DeleteTitleContent,
    SeekContent,
    OpenContent,
    LauchBackwardsCompatibility,
    ExportTitleInitalize,
    ExportContentBegin,
    ExportContentData,
    ExportContentEnd,
    ExportTitleDone,
    AddTitleMetadata,
    Encrypt,
    Decrypt,
    GetBoot2Version,
    AddTitleCancel,
    Sign,
    VerifySign,
    GetStoredContentCount,
    GetStoredContents,
    GetStoredTitleMetadataSize,
    GetStoredTitleMetadata,
    GetSharedContentCount,
    GetSharedContents,
    DeleteSharedContents,
    DiskInterfaceGetTitleMetadataSize,
    DiskInterfaceGetTitleMetadata,
    DiskInterfaceVerifyWithView,
    SetupStreamKey,
    DeleteStreamKey,
    DeleteContent,
    // Invalid3F
    GetVersion0TicketFromView,
    // Unknown41,
    // Unknown42,
    GetTicketSizeFromView,
    GetTicketFromView,
    CheckKoreaRegion,
}

impl From<Ioctl> for i32 {
    fn from(value: Ioctl) -> Self {
        match value {
            Ioctl::AddTicket => todo!(),
            Ioctl::AddTitleStart => todo!(),
            Ioctl::AddContentStart => todo!(),
            Ioctl::AddContentData => todo!(),
            Ioctl::AddContentFinish => todo!(),
            Ioctl::AddTitleFinish => todo!(),
            Ioctl::GetDeviceId => 7,
            Ioctl::Launch => todo!(),
            Ioctl::OpenActiveTitleContent => todo!(),
            Ioctl::ReadContent => todo!(),
            Ioctl::CloseContent => todo!(),
            Ioctl::GetOwnedTitleCount => 12,
            Ioctl::GetOwnedTitles => 13,
            Ioctl::GetTitleCount => 14,
            Ioctl::GetTitles => 15,
            Ioctl::GetTitleContentsCount => 16,
            Ioctl::GetTitleContents => 17,
            Ioctl::GetViewCount => todo!(),
            Ioctl::GetViews => todo!(),
            Ioctl::GetTicketViewCount => todo!(),
            Ioctl::GetTicketViews => todo!(),
            Ioctl::GetConsumption => todo!(),
            Ioctl::DeleteTitle => todo!(),
            Ioctl::DeleteTicket => todo!(),
            Ioctl::DiskInterfaceGetTitleMetadataViewSize => todo!(),
            Ioctl::DiskInterfaceGetTitleMetadataView => todo!(),
            Ioctl::DiskInterfaceGetTicketView => todo!(),
            Ioctl::DiskInterfaceVerify => todo!(),
            Ioctl::GetTitleDir => todo!(),
            Ioctl::GetDeviceCertificate => todo!(),
            Ioctl::ImportBoot => todo!(),
            Ioctl::GetTitleId => todo!(),
            Ioctl::SetUid => todo!(),
            Ioctl::DeleteTitleContent => todo!(),
            Ioctl::SeekContent => todo!(),
            Ioctl::OpenContent => todo!(),
            Ioctl::LauchBackwardsCompatibility => todo!(),
            Ioctl::ExportTitleInitalize => todo!(),
            Ioctl::ExportContentBegin => todo!(),
            Ioctl::ExportContentData => todo!(),
            Ioctl::ExportContentEnd => todo!(),
            Ioctl::ExportTitleDone => todo!(),
            Ioctl::AddTitleMetadata => todo!(),
            Ioctl::Encrypt => todo!(),
            Ioctl::Decrypt => todo!(),
            Ioctl::GetBoot2Version => todo!(),
            Ioctl::AddTitleCancel => todo!(),
            Ioctl::Sign => todo!(),
            Ioctl::VerifySign => todo!(),
            Ioctl::GetStoredContentCount => todo!(),
            Ioctl::GetStoredContents => todo!(),
            Ioctl::GetStoredTitleMetadataSize => todo!(),
            Ioctl::GetStoredTitleMetadata => todo!(),
            Ioctl::GetSharedContentCount => todo!(),
            Ioctl::GetSharedContents => todo!(),
            Ioctl::DeleteSharedContents => todo!(),
            Ioctl::DiskInterfaceGetTitleMetadataSize => todo!(),
            Ioctl::DiskInterfaceGetTitleMetadata => todo!(),
            Ioctl::DiskInterfaceVerifyWithView => todo!(),
            Ioctl::SetupStreamKey => todo!(),
            Ioctl::DeleteStreamKey => todo!(),
            Ioctl::DeleteContent => todo!(),
            Ioctl::GetVersion0TicketFromView => todo!(),
            Ioctl::GetTicketSizeFromView => todo!(),
            Ioctl::GetTicketFromView => todo!(),
            Ioctl::CheckKoreaRegion => todo!(),
        }
    }
}

static DEV_ES: &CStr = c"/dev/es";

use core::ffi::CStr;

use alloc::vec::Vec;

use crate::ios;

pub fn get_title_count() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetTitleCount, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

pub fn get_titles(title_count: u32) -> Result<Vec<u64>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    // TODO: Avoid allocation
    let mut out_buf = alloc::vec![0u8; title_count as usize * core::mem::size_of::<u64>()];

    let count: [u8; 4] = title_count.to_be_bytes();
    ios::ioctlv::<1, 1, 2>(es, Ioctl::GetTitles, &[&count], &mut [&mut out_buf[..]])?;

    let _ = ios::close(es);

    // TODO: Avoid allocation
    Ok(out_buf
        .chunks_exact(core::mem::size_of::<u64>())
        .map(|bytes| u64::from_be_bytes(bytes.try_into().expect("should fit")))
        .collect())
}

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

    Ok(u32::from_be_bytes(
        out_buf.try_into().map_err(|_| ios::Error::Invalid)?,
    ))
}

// TODO: Proper enuming since there are different signature types and differing sizes for them
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
