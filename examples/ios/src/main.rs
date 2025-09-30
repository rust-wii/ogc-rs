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
use embedded_sdmmc::{
    blockdevice::{Block, BlockCount, BlockIdx},
    BlockDevice, TimeSource, Timestamp, VolumeIdx, VolumeManager,
};

pub struct SdCardDevice(Device);
pub struct DummyTimesource;
impl BlockDevice for SdCardDevice {
    type Error = ();

    fn read(&self, blocks: &mut [Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        if blocks.len() == 1 {
            let mut block = [0u8; 512];
            let res = self
                .0
                .read_sectors(core::slice::from_mut(&mut block), start_block_idx.0 as _)
                .map_err(|_| ());

            blocks[0].contents = block;
            res
        } else {
            todo!("read is not currently implemented")
        }
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        if blocks.len() == 1 {
            let block = blocks[0].contents;
            let res = self
                .0
                .write_sectors(core::slice::from_ref(&block), start_block_idx.0 as _)
                .map_err(|_| ());
            res
        } else {
            todo!("write is not currently implemented")
        }
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        todo!("num_blocks is not currently implemented")
    }
}
impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> Timestamp {
        todo!()
    }
}

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

    // let mut block = [0u8; 512];
    //
    // //read_sectors(blocks: &mut [[0u8; 512]], offset: usize]) -> Result<(), ios::Error>;
    // device
    //     .read_sectors(core::slice::from_mut(&mut block), 0)
    //     .unwrap();
    //
    // let bpb = BPB::from_bytes(block[0x00B..].try_into().unwrap());
    //
    // let mut info_block = [0u8; 512];
    // device
    //     .read_sectors(
    //         core::slice::from_mut(&mut info_block),
    //         bpb.fs_info_sector as usize,
    //     )
    //     .unwrap();
    //
    // let info = FSInfo::from_bytes(&info_block);
    //
    // println!("{:?}", bpb);
    // println!("{:?}", info);
    //
    let volmgr = VolumeManager::new(SdCardDevice(device), DummyTimesource);
    let volume = unsafe {
        volmgr
            .open_special(0x0C, BlockIdx(0), BlockCount(67108864))
            .unwrap()
    };
    let root_dir = volume.open_root_dir().unwrap();

    let func = |entry: &embedded_sdmmc::DirEntry| {
        println!("{:?}", entry.name);
    };

    root_dir.iterate_dir(func).unwrap();

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
        &self,
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

    fn write_sectors(
        &self,
        sectors: &[[u8; 512]],
        offset: usize,
    ) -> Result<(), ogc_rs::ios::Error> {
        let resp_rca = self.send_command(&Request::SEND_RCA)?;
        let rca = resp_rca.rsp_field0;

        self.send_command(&Request::select(rca))?;

        const SDIO_CMD_WRITEMULTIBLOCK: u32 = 0x19;
        const SDIO_CMD_TYPE_AC: u32 = 3;
        const SDIO_RESPONSE_TYPE_R1: u32 = 1;

        // SDIO requires 32 byte alignment
        // On hardware this probably needs to be in the IPC memory space :shrug:
        let mut aligned_buffer = ogc_rs::utils::alloc_aligned_buffer(sectors.as_flattened());

        self.send_command(&Request::new(
            SDIO_CMD_WRITEMULTIBLOCK,
            SDIO_CMD_TYPE_AC,
            SDIO_RESPONSE_TYPE_R1,
            offset as u32,
            sectors.len() as u32,
            512,
            aligned_buffer.as_mut_ptr(),
        ))?;

        self.send_command(&Request::DE_SELECT)?;

        // sectors
        //     .as_flattened_mut()
        //     .copy_from_slice(&mut aligned_buffer);
        //
        Ok(())
    }
}

trait DeviceExt {
    fn read_sectors(
        &self,
        sectors: &mut [[u8; 512]],
        offset: usize,
    ) -> Result<(), ogc_rs::ios::Error>;

    fn write_sectors(&self, sectors: &[[u8; 512]], offset: usize)
        -> Result<(), ogc_rs::ios::Error>;
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

    const SOFTWARE_RESET_REGISTER: u8 = 0x2F;
    bitflags::bitflags! {
        pub struct SoftwareResetRegister: u8 {
            const RESET_ALL = 0b1;
            const RESET_CMD = 0b10;
            const RESET_DATA = 0b100;
        }
    }

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
        let reset = SoftwareResetRegister::RESET_ALL
            | SoftwareResetRegister::RESET_CMD
            | SoftwareResetRegister::RESET_DATA;

        device
            .write_to_host_controller_register(
                SOFTWARE_RESET_REGISTER,
                core::mem::size_of::<u8>().try_into().unwrap(),
                reset.bits().into(),
            )
            .unwrap();

        // Wait until all bits are 0
        while device
            .read_from_host_controller_register(SOFTWARE_RESET_REGISTER, 1)
            .unwrap()
            != 0
        {
            core::hint::spin_loop();
        }

        bitflags::bitflags! {
            pub struct NormalInterruptStatusEnableRegister: u16 {
                const COMMAND_COMPLETE_STATUS_ENABLE =   0b1;
                const TRANSFER_COMPLETE_STATUS_ENABLE =  0b10;
                const BLOCK_GAP_EVENT_STATUS_ENABLE =    0b100;
                const DMA_INTERRUPT_STATUS_ENABLE =      0b1000;
                const BUFFER_WRITE_READY_STATUS_ENABLE = 0b10000;
                const BUFFER_READ_READY_STATUS_ENABLE =  0b100000;
                const CARD_INSERTION_STATUS_ENABLE =     0b1000000;
                const CARD_REMOVAL_STATUS_ENABLE =       0b10000000;
                const CARD_INTERRUPT_STATUS_ENABLE =     0b100000000;
                const INT_A_STATUS_ENABLE =              0b1000000000;
                const INT_B_STATUS_ENABLE =              0b10000000000;
                const INT_C_STATUS_ENABLE =              0b100000000000;
                const RETUNING_EVENT_STATUS_ENABLE =     0b1000000000000;
                const FX_EVENT_STATUS_ENABLE =           0b10000000000000;
            }
        }

        bitflags::bitflags! {
            pub struct ErrorInterruptStatusEnableRegister: u16 {
                const COMMAND_TIMEOUT_ERROR_STATUS_ENABLE = 0b1;
                const COMMAND_CRC_ERROR_STATUS_ENABLE =     0b10;
                const COMMAND_END_BIT_ERROR_STATUS_ENABLE = 0b100;
                const COMMAND_INDEX_ERROR_STATUS_ENABLE =   0b1000;
                const DATA_TIMEOUT_ERROR_STATUS_ENABLE =    0b10000;
                const DATA_CRC_ERROR_STATUS_ENABLE =        0b100000;
                const DATA_END_BIT_ERROR_STATUS_ENABLE =    0b1000000;
                const CURRENT_LIMIT_ERROR_STATUS_ENABLE =   0b10000000;
                const AUTO_CMD_ERROR_STATUS_ENABLE =        0b100000000;
                const ADMA_ERROR_STATUS_ENABLE =            0b1000000000;
                const TUNING_STATUS_ERROR_STATUS_ENABLE =   0b10000000000;
                const REPSONSE_ERROR_STATUS_ENABLE =        0b100000000000;
            }
        }

        let normal_interrupt_status =
            NormalInterruptStatusEnableRegister::COMMAND_COMPLETE_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::TRANSFER_COMPLETE_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::BLOCK_GAP_EVENT_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::DMA_INTERRUPT_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::BUFFER_WRITE_READY_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::BUFFER_READ_READY_STATUS_ENABLE
                | NormalInterruptStatusEnableRegister::CARD_INTERRUPT_STATUS_ENABLE;

        let error_interrupt_status =
            ErrorInterruptStatusEnableRegister::COMMAND_TIMEOUT_ERROR_STATUS_ENABLE
                | ErrorInterruptStatusEnableRegister::COMMAND_CRC_ERROR_STATUS_ENABLE
                | ErrorInterruptStatusEnableRegister::DATA_END_BIT_ERROR_STATUS_ENABLE
                | ErrorInterruptStatusEnableRegister::CURRENT_LIMIT_ERROR_STATUS_ENABLE;

        let status: u32 = u32::from(normal_interrupt_status.bits()) << 16
            | u32::from(error_interrupt_status.bits());

        const NORMAL_INTERRUPT_STATUS_REGISTER: u8 = 0x34;
        //const ERROR_INTERRUPT_STATUS_REGISTER: u8 = 0x36;

        const NORMAL_INTERRUPT_SIGNAL_STATUS_REGISTER: u8 = 0x38;
        //const ERROR_INTERRUPT_SIGNAL_STATUS_REGISTER: u8 = 0x3A;

        // This writes to `NORMAL_INTERRUPT_STATUS_REGISTER` and `ERROR_INTERRUPT_STATUS_REGISTER` as
        // one call
        let _ =
            device.write_to_host_controller_register(NORMAL_INTERRUPT_STATUS_REGISTER, 4, status);

        // This writes to both `NORMAL_INTERRUPT_SIGNAL_STATUS_REGISTER` and
        // `ERROR_INTERRUPT_STATUS_REGISTER` as one call
        let _ = device.write_to_host_controller_register(
            NORMAL_INTERRUPT_SIGNAL_STATUS_REGISTER,
            4,
            status,
        );

        bitflags::bitflags! {
            pub struct PowerControlRegister: u8 {
                const SD_BUS_POWER_VDD1 = 0b1;
                const SD_BUS_VOLTAGE_SELECT_18V =  0b1010;
                const SD_BUS_VOLTAGE_SELECT_3V =   0b1100;
                const SD_BUS_VOLTAGE_SELECT_33V =  0b1110;
            }
        }

        let select = PowerControlRegister::SD_BUS_VOLTAGE_SELECT_33V;

        // Set power
        device
            .write_to_host_controller_register(
                HOST_CONTROLLER_REG_PWR_CTRL,
                1,
                select.bits().into(),
            )
            .unwrap();
        device
            .write_to_host_controller_register(
                HOST_CONTROLLER_REG_PWR_CTRL,
                1,
                (select | PowerControlRegister::SD_BUS_POWER_VDD1)
                    .bits()
                    .into(),
            )
            .unwrap();

        bitflags::bitflags! {
            pub struct ClockControlRegister: u16 {
                const INTERNAL_CLOCK_ENABLE = 0b1;
                const INTERNAL_CLOCK_STABLE = 0b10;
                const SD_CLOCK_ENABLE =       0b100;
                const PLL_ENABLE =            0b1000;
                const CLK_DIV_BY_2 =          0b100000000;
                const CLK_DIV_BY_4 =          0b1000000000;
                const CLK_DIV_BY_8 =          0b10000000000;
                const CLK_DIV_BY_16 =         0b100000000000;
                const CLK_DIV_BY_32 =         0b1000000000000;
                const CLK_DIV_BY_64 =         0b10000000000000;
                const CLK_DIV_BY_128 =        0b100000000000000;
                const CLK_DIV_BY_256 =        0b1000000000000000;
            }
        }

        // Clock
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2, 0)
            .unwrap();

        let clock =
            ClockControlRegister::INTERNAL_CLOCK_ENABLE | ClockControlRegister::CLK_DIV_BY_2;

        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2, clock.bits().into())
            .unwrap();

        while device
            .read_from_host_controller_register(HOST_CONTROLLER_REG_CLK_CTRL, 2)
            .unwrap()
            & u32::from(ClockControlRegister::INTERNAL_CLOCK_STABLE.bits())
            != u32::from(ClockControlRegister::INTERNAL_CLOCK_STABLE.bits())
        {
            core::hint::spin_loop();
        }

        device
            .write_to_host_controller_register(
                HOST_CONTROLLER_REG_CLK_CTRL,
                2,
                (clock
                    | ClockControlRegister::INTERNAL_CLOCK_STABLE
                    | ClockControlRegister::SD_CLOCK_ENABLE)
                    .bits()
                    .into(),
            )
            .unwrap();

        //max timeout for stand sd cards not sdxc
        //CLK x 2^27
        const TIMEOUT_CLOCK: u32 = 0b1110;

        // Timeout
        device
            .write_to_host_controller_register(HOST_CONTROLLER_REG_TIMEOUT_CTRL, 1, TIMEOUT_CLOCK)
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
