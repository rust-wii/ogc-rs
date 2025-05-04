#![no_std]
#![no_main]

use alloc::vec;
use ogc_rs::{
    ios::{self, fs::get_file_stats, Mode, SeekMode},
    print, println,
};
extern crate alloc;

#[no_mangle]
extern "C" fn main() {
    if let Ok(metadata) = get_file_stats("/shared2/sys/SYSCONF") {
        if let Ok(fd) = ios::open(c"/shared2/sys/SYSCONF", Mode::Read) {
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
    loop {
        core::hint::spin_loop();
    }
}
