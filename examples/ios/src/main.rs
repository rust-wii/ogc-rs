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
        let size = if ios::ioctl(fd, GET_FILE_STATS, &[], &mut out_buf).is_ok() {
            usize::try_from(u32::from_be_bytes(out_buf[0..4].try_into().unwrap())).unwrap()
        } else {
            0usize
        };

        // Try to seek to the start
        let _ = ios::seek(fd, 0, SeekMode::Start);

        let mut bytes = vec![0; size];
        if let Ok(bytes_read) = ios::read(fd, &mut bytes) {
            // SAFETY:  I read this much bytes
            unsafe { bytes.set_len(bytes_read) };
        };

        println!("{:?}", bytes);

        let _ = ios::close(fd);
    }

    loop {}
}
