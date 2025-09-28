#![no_std]
#![no_main]

use alloc::vec;
use ogc_rs::{
    ios::{
        self,
        sdio::{Device, Request},
        Mode, SeekMode,
    },
    print, println,
};
extern crate alloc;

#[no_mangle]
extern "C" fn main() {
    if let Ok(fd) = ios::open(c"/shared2/sys/SYSCONF", Mode::Read) {
        if let Ok(metadata) = ios::fs::get_file_stats_from_fd(fd) {
            if metadata.offset() != 0 {
                let _ = ios::seek(fd, 0, SeekMode::Start);
            }

            let mut bytes = vec![0; metadata.size()];
            if let Ok(bytes_read) = ios::read(fd, &mut bytes) {
                unsafe { bytes.set_len(bytes_read.try_into().unwrap()) };
            }

            println!("{:?}", bytes);

            let _ = ios::close(fd);
        }
    }

    let (is_sdhc, mut device) = try_init_sd();

    let mut block = [0u8; 512];

    //read_sectors(blocks: &mut [[0u8; 512]], offset: usize]) -> Result<(), ios::Error>;
    device
        .read_sectors(core::slice::from_mut(&mut block), 0)
        .unwrap();

    let bpb = BPB::from_bytes(block[0x00B..].try_into().unwrap());

    let mut info_block = [0u8; 512];
    device
        .read_sectors(
            core::slice::from_mut(&mut info_block),
            bpb.fs_info_sector as usize,
        )
        .unwrap();

    let info = FSInfo::from_bytes(&info_block);

    println!("{:?}", bpb);
    println!("{:?}", info);

    loop {
        core::hint::spin_loop();
    }
}

pub struct SDCard {
    rca: u32,
    is_sdhc: bool,
    device: Device,
}

impl SDCard {
    pub fn init() -> Option<Self> {
        let (is_sdhc, mut device) = try_init_sd();

        let resp_rca = device.send_command(&Request::SEND_RCA).ok()?;
        let rca = resp_rca.rsp_field0;

        Some(Self {
            rca,
            is_sdhc,
            device,
        })
    }
}

impl DeviceExt for Device {
    fn read_sectors(
        &mut self,
        sectors: &mut [[u8; 512]],
        offset: usize,
    ) -> Result<(), ogc_rs::ios::Error> {
        let resp_rca = self.send_command(&Request::SEND_RCA)?;
        let rca = resp_rca.rsp_field0;

        self.send_command(&Request::select(rca))?;

        const SDIO_CMD_READMULTIBLOCK: u32 = 0x12;
        const SDIO_CMD_TYPE_AC: u32 = 3;
        const SDIO_RESPONSE_TYPE_R1: u32 = 1;

        // SDIO requires 32 byte alignment
        // On hardware this probably needs to be in the IPC memory space :shrug:
        let mut aligned_buffer = ogc_rs::utils::alloc_aligned_buffer(sectors.as_flattened_mut());

        //let req = Request::read_multiblock(offset, sectors, &mut aligned_buffer);

        self.send_command(&Request::new(
            SDIO_CMD_READMULTIBLOCK,
            SDIO_CMD_TYPE_AC,
            SDIO_RESPONSE_TYPE_R1,
            offset as u32,
            sectors.len() as u32,
            512,
            aligned_buffer.as_mut_ptr(),
        ))?;

        self.send_command(&Request::DE_SELECT)?;

        sectors
            .as_flattened_mut()
            .copy_from_slice(&mut aligned_buffer);

        Ok(())
    }
}

trait DeviceExt {
    fn read_sectors(
        &mut self,
        sectors: &mut [[u8; 512]],
        offset: usize,
    ) -> Result<(), ogc_rs::ios::Error>;
}

pub fn try_init_sd() -> (bool, Device) {
    const SD_STATUS_INSERTED: u32 = 0b1;
    const SD_STATUS_INITALIZED: u32 = 0b1_0000_0000_0000_0000;
    const SD_STATUS_SDHC: u32 = 0x100000;

    const HOST_CTRL_4BIT: u32 = 2;

    const HOST_CONTROLLER_REG_SOFT_RESET: u8 = 47;
    const HOST_CONTROLLER_REG_PWR_CTRL: u8 = 41;
    const HOST_CONTROLLER_REG_HOST_CTRL: u8 = 40;
    const HOST_CONTROLLER_REG_CLK_CTRL: u8 = 44;
    const HOST_CONTROLLER_REG_TIMEOUT_CTRL: u8 = 46;
    //
    // const SDIO_CMD_APPCMD: u32 = 55;
    // const SDIO_CMD_SELECT: u32 = 7;
    // const SDIO_CMD_SEND_CID: u32 = 2;
    // const SDIO_CMD_SEND_RCA: u32 = 3;
    // const SDIO_APPCMD_SENDOPCOND: u32 = 41;
    // const SDIO_APPCMD_SET_BUS_WIDTH: u32 = 6;
    // const SDIO_CMD_SET_BLOCK_LENGTH: u32 = 16;
    //
    // const SDIO_CMD_TYPE_AC: u32 = 3;
    //
    // const SDIO_RESPONSE_TYPE_R1: u32 = 1;
    // const SDIO_RESPONSE_TYPE_R2: u32 = 3;
    // const SDIO_RESPONSE_TYPE_R5: u32 = 6;
    // const SDIO_RESPONSE_TYPE_R1B: u32 = 2;
    // const SDIO_RESPONSE_TYPE_R3: u32 = 4;
    //
    let mut device = Device::open().unwrap();

    // Reset
    let mut rca = device.reset().unwrap();
    let status = device.get_status().unwrap();
    let is_sdhc = status & SD_STATUS_SDHC == SD_STATUS_SDHC;

    if status & SD_STATUS_INSERTED != SD_STATUS_INSERTED {
        println!("SD Card not found");
        panic!();
    }

    if status & SD_STATUS_INITALIZED != SD_STATUS_INITALIZED {
        // Drop and reopen the device
        drop(device);
        let mut device = Device::open().unwrap();

        // Reset host controller using device
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_SOFT_RESET, 1, 7)
            .unwrap();

        let software_reset = 7;
        // Wait until properly reset
        while software_reset
            == device
                .read_from_host_controller_register(HOST_CONTROLLER_REG_SOFT_RESET, 1)
                .unwrap()
        {
            core::hint::spin_loop();
        }

        let _ = device.write_to_host_controller_register(0x34, 4, 0x13f00c3);
        let _ = device.write_to_host_controller_register(0x38, 4, 0x13f00c3);

        // Set power
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_PWR_CTRL, 1, 14)
            .unwrap();
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_PWR_CTRL, 1, 15)
            .unwrap();

        // Clock
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2, 0)
            .unwrap();
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2, 0x101)
            .unwrap();

        let clk_ctrl = 0x101;

        while clk_ctrl
            == device
                .read_from_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2)
                .unwrap()
        {
            core::hint::spin_loop();
        }

        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2, 0x107)
            .unwrap();

        // Timeout
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_TIMEOUT_CTRL, 1, 14)
            .unwrap();
        let _ = device.send_command(&Request::GO_IDLE).unwrap();
        let resp = device.send_command(&Request::SEND_IF_COND).unwrap();

        if resp.rsp_field0 & 0xFF != 0xAA {
            println!("Response from IF_COND: {}", resp.rsp_field0);
        }

        let is_sdhc = loop {
            let _ = device.send_command(&Request::APP_CMD).unwrap();
            let resp = device.send_command(&Request::SEND_OP_COND).unwrap();
            if resp.rsp_field0 & 1 << 31 == 1 << 31 {
                break resp.rsp_field0 & 1 << 30 == 1 << 30;
            }
        };

        let resp_cid = device.send_command(&Request::send_cid(rca)).unwrap();
        println!("{:?}", resp_cid);

        let resp_rca = device.send_command(&Request::SEND_RCA).unwrap();
        println!("{:?}", resp_rca);
        rca = resp_rca.rsp_field0;

        let mut host_ctrl = device
            .read_from_host_controller_register(HOST_CONTROLLER_REG_HOST_CTRL, 1)
            .unwrap();
        host_ctrl &= 0xff;
        host_ctrl &= !HOST_CTRL_4BIT;
        host_ctrl |= HOST_CTRL_4BIT;
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_HOST_CTRL, 1, host_ctrl)
            .unwrap();

        device.enable_clock(true).unwrap();

        device.send_command(&Request::select(rca)).unwrap();
        {
            device
                .send_command(&Request::set_block_length(512))
                .unwrap();
            device.send_command(&Request::appcmd_with_rca(rca)).unwrap();
            device.send_command(&Request::set_bus_width(4)).unwrap();
        }
        device.send_command(&Request::DE_SELECT).unwrap();

        println!("END OF INIT");

        return (is_sdhc, device);
    }

    return (is_sdhc, device);
}

#[derive(Debug)]
pub struct BPB {
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sector_count: u16,
    fat_count: u8,
    fat16_max_root_dir_entry_count: u16,
    sector_count: u16,
    media_type: u8,
    sectors_per_fat_count: u16,
    sectors_per_track: u16,
    head_count: u16,
    hidden_sector_count: u32,
    sector_count_fat32: u32,
    sectors_per_fat: u32,
    drive_flags: u16,
    version: u16,
    cluster_root_dir_start: u32,
    fs_info_sector: u16,
    backup_sector: u16,
}

impl BPB {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes_per_sector: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            sectors_per_cluster: bytes[2],
            reserved_sector_count: u16::from_le_bytes(bytes[3..5].try_into().unwrap()),
            fat_count: bytes[5],
            fat16_max_root_dir_entry_count: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            sector_count: u16::from_le_bytes(bytes[8..10].try_into().unwrap()),
            media_type: bytes[10],
            sectors_per_fat_count: u16::from_le_bytes(bytes[11..13].try_into().unwrap()),
            sectors_per_track: u16::from_le_bytes(bytes[13..15].try_into().unwrap()),
            head_count: u16::from_le_bytes(bytes[15..17].try_into().unwrap()),
            hidden_sector_count: u32::from_le_bytes(bytes[17..21].try_into().unwrap()),
            sector_count_fat32: u32::from_le_bytes(bytes[21..25].try_into().unwrap()),
            sectors_per_fat: u32::from_le_bytes(bytes[25..29].try_into().unwrap()),
            drive_flags: u16::from_le_bytes(bytes[29..31].try_into().unwrap()),
            version: u16::from_le_bytes(bytes[31..33].try_into().unwrap()),
            cluster_root_dir_start: u32::from_le_bytes(bytes[33..37].try_into().unwrap()),
            fs_info_sector: u16::from_le_bytes(bytes[37..39].try_into().unwrap()),
            backup_sector: u16::from_le_bytes(bytes[39..41].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct FSInfo {
    free_cluster_count: u32,
    recent_cluster: u32,
}

impl FSInfo {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            free_cluster_count: u32::from_le_bytes(bytes[488..492].try_into().unwrap()),
            recent_cluster: u32::from_le_bytes(bytes[492..496].try_into().unwrap()),
        }
    }
}
