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
    GetTicketViewCount,
    GetTicketViews,
    GetTitleMetadataViewSize,
    GetTitleMetadataView,
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
            Ioctl::AddTicket => 1,
            Ioctl::AddTitleStart => 2,
            Ioctl::AddContentStart => 3,
            Ioctl::AddContentData => 4,
            Ioctl::AddContentFinish => 5,
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
            Ioctl::GetTicketViewCount => 17,
            Ioctl::GetTicketViews => 18,
            Ioctl::GetTitleMetadataViewSize => 20,
            Ioctl::GetTitleMetadataView => 21,
            Ioctl::GetConsumption => 22,
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
            Ioctl::GetTitleContents => todo!(),
        }
    }
}

static DEV_ES: &CStr = c"/dev/es";

use core::ffi::CStr;

use alloc::vec::Vec;

use crate::ios;

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

pub fn add_title_start(
    signed_title_meta: &[u8],
    signed_certs: &[u8],
    signed_crl: &[u8],
) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

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

    let _ = ios::close(es);

    Ok(())
}

pub fn add_content_start(title_id: u64, content_id: u32) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::AddContentStart,
        &[&title_id.to_be_bytes(), &content_id.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

pub fn add_content_data(content_fd: i32, data: &[u8]) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<2, 0, 2>(
        es,
        Ioctl::AddContentData,
        &[&content_fd.to_be_bytes(), data],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

pub fn add_content_finish(content_id: u32) -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<1, 0, 1>(
        es,
        Ioctl::AddContentFinish,
        &[&content_id.to_be_bytes()],
        &mut [],
    )?;

    let _ = ios::close(es);

    Ok(())
}

pub fn add_title_finish() -> Result<(), ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    ios::ioctlv::<0, 0, 0>(es, Ioctl::AddTitleFinish, &[], &mut [])?;

    let _ = ios::close(es);

    Ok(())
}

pub fn get_device_id() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetDeviceId, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

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

pub fn open_active_title_content(content_idx: u32) -> Result<(), ios::Error> {}

pub fn get_owned_title_count() -> Result<u32, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    let mut out_buf = [0u8; 4];
    ios::ioctlv::<0, 1, 1>(es, Ioctl::GetOwnedTitleCount, &[], &mut [&mut out_buf])?;

    let _ = ios::close(es);

    Ok(u32::from_be_bytes(out_buf))
}

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
    Ok(out_buf
        .chunks_exact(core::mem::size_of::<u64>())
        .map(|bytes| u64::from_be_bytes(bytes.try_into().expect("should fit")))
        .collect())
}

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

pub fn get_title_counts(title_id: u64, content_count: u32) -> Result<Vec<u32>, ios::Error> {
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
    Ok(out_buf
        .chunks_exact(core::mem::size_of::<u32>())
        .map(|bytes| u32::from_be_bytes(bytes.try_into().expect("should fit")))
        .collect())
}

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
pub fn get_ticket_views(title_id: u64, view_count: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    const TICKET_VIEW_SIZE: usize = 216; // 0xD8
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

pub fn get_consumption(title_id: u64, limit_count: u32) -> Result<Vec<u8>, ios::Error> {
    let es = ios::open(DEV_ES, ios::Mode::None)?;

    const TIKLIMIT_SIZE: usize = 8;
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
