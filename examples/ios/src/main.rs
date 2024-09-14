#![no_std]
#![feature(start)]

use alloc::vec;
use ogc_rs::{
    ios::{self, Mode, SeekMode},
    print, println,
};

extern crate alloc;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Try to open SYSCONF
    if let Ok(fd) = ios::open(c"/shared2/sys/SYSCONF", Mode::Read) {
        // Try to grab size or default to 0;
        const GET_FILE_STATS: i32 = 11;
        let mut out_buf = [0u8; 8];
        let (size, seek_pos) = if ios::ioctl(fd, GET_FILE_STATS, &[], &mut out_buf).is_ok() {
            (
                usize::try_from(u32::from_be_bytes(out_buf[0..4].try_into().unwrap())).unwrap(),
                usize::try_from(u32::from_be_bytes(out_buf[4..8].try_into().unwrap())).unwrap(),
            )
        } else {
            (0usize, 0usize)
        };
        println!("{:?}, {:?}", size, seek_pos);

        if seek_pos != 0 {
            // Try to seek to the start
            let _ = ios::seek(fd, 0, SeekMode::Start);
        }

        let mut bytes = vec![0; size];
        if let Ok(bytes_read) = ios::read(fd, &mut bytes) {
            // SAFETY:  I read this much bytes
            unsafe { bytes.set_len(bytes_read.try_into().unwrap()) };
        };

        println!("{:?}", bytes);
        let _ = ios::close(fd);
    }

    println!("Format: {:?}", get_video_format());
    loop {
        core::hint::spin_loop();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum VideoFormat {
    Ntsc,
    Pal,
    MPal,
}

fn get_video_format() -> Option<VideoFormat> {
    if let Ok(fd) = ios::open(c"/title/00000001/00000002/data/setting.txt", Mode::Read) {
        let mut bytes = [0u8; 256];
        if let Ok(bytes_read) = ios::read(fd, &mut bytes) {
            debug_assert!(bytes_read == 256);
        }

        let mut key: u32 = 0x73B5DBFA;
        for byte in &mut bytes {
            *byte ^= u8::try_from(key & 0xff).unwrap();
            key = (key << 1) | (key >> 31);
        }

        let text = if let Err(vld) = core::str::from_utf8(&bytes) {
            unsafe { core::str::from_utf8_unchecked(&bytes[..vld.valid_up_to()]) }
        } else {
            return None;
        };

        for line in text.lines() {
            if let Some(char) = line.find("VIDEO=") {
                return match line[char + 6..].trim() {
                    "NTSC" => Some(VideoFormat::Ntsc),
                    "PAL" => Some(VideoFormat::Pal),
                    "MPAL" => Some(VideoFormat::MPal),
                    _ => None,
                };
            }
        }
        let _ = ios::close(fd);
    }
    None
}
