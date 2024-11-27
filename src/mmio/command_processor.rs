#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use voladdress::{Safe, VolAddress};

const BASE: usize = 0xCC00_0000;

const STATUS_REGISTER: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE) };

const CONTROL_REGISTER: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x2) };

const CLEAR_REGISTER: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x4) };

const PERFORMANCE_SELECT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x6) };

const TOKEN: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0xE) };

const BOUNDING_BOX_LEFT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x10) };

const BOUNDING_BOX_RIGHT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x12) };

const BOUNDING_BOX_TOP: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x14) };

const BOUNDING_BOX_BOTTOM: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x16) };

const FIFO_BASE_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x20) };

const FIFO_BASE_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x22) };

const FIFO_END_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x24) };

const FIFO_END_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x26) };

const FIFO_HIGH_WATERMARK_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x28) };

const FIFO_HIGH_WATERMARK_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2A) };

const FIFO_LOW_WATERMARK_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2C) };

const FIFO_LOW_WATERMARK_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2E) };

const FIFO_READ_WRITE_DISTANCE_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x30) };

const FIFO_READ_WRITE_DISTANCE_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x32) };

const FIFO_WRITE_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x34) };

const FIFO_WRITE_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x36) };

const FIFO_READ_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x38) };

const FIFO_READ_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x3A) };

const FIFO_BREAKPOINT_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3C) };

const FIFO_BREAKPOINT_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3E) };

const TRANSFORM_RASTER_BUSY_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x40) };

const TRANSFORM_RASTER_BUSY_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x42) };

const TRANSFORM_CLOCK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x44) };

const TRANSFORM_CLOCK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x46) };

const TRANSFORM_WAIT_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x48) };

const TRANSFORM_WAIT_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4A) };

const TRANSFORM_WAIT_OUT_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4C) };

const TRANSFORM_WAIT_OUT_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4E) };

const VERTEX_CACHE_CHECK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x50) };

const VERTEX_CACHE_CHECK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x52) };

const VERTEX_CACHE_MISS_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x54) };

const VERTEX_CACHE_MISS_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x56) };

const VERTEX_CACHE_STALL_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x58) };

const VERTEX_CACHE_STALL_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x5A) };

const CLOCKS_PER_VERTEX_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x60) };

const CLOCKS_PER_VERTEX_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x62) };

const CLOCKS_PER_VERTEX_OUT_COUNT: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x64) };
