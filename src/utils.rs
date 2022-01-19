//! Utility Functions to convert between types.

use core::alloc::Layout;

use alloc::vec::Vec;

/// OS memory casting macros.
mod memory_casting {
    /// Cast a cached address to a uncached address.
    /// Example: 0x8xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_uncached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + ($crate::ffi::SYS_BASE_UNCACHED - $crate::ffi::SYS_BASE_CACHED))
                as *mut c_void
        }};
    }

    /// Cast a cached address to a physical address.
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - $crate::ffi::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a uncached address to a cached address.
    /// Example: 0xCxxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_cached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - ($crate::ffi::SYS_BASE_UNCACHED - $crate::ffi::SYS_BASE_CACHED))
                as *mut c_void
        }};
    }

    /// Cast a uncached address to a physical address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - $crate::ffi::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a cached address.
    /// Example: 0x0xxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_cached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + $crate::ffi::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a uncached address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_uncached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + $crate::ffi::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a virtual address to a physical address.  
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_virtual_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) & !$crate::ffi::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }
}

/// Console printing macros.
mod console_printing {
    /// Prints to the console video output.
    ///
    /// Equivalent to the [`println!`] macro except that a newline is not printed at
    /// the end of the message.
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => {
            let s = ::alloc::fmt::format(format_args!($($arg)*));
            $crate::console::Console::print(&s);
        }
    }

    /// Prints to the standard output, with a newline.
    #[macro_export]
    macro_rules! println {
        () => (print!("\n"));
        ($fmt:expr) => (print!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
    }
}
pub fn alloc_aligned_buffer(buffer: &[u8]) -> Vec<u8> {
    let size = if buffer.len() % 32 == 0 {
        buffer.len()
    } else {
        ((buffer.len() + 31) / 32) * 32
    };

    let mut align_buf = unsafe {
        let ptr = alloc::alloc::alloc_zeroed(Layout::from_size_align(size, 32).unwrap()) as *mut u8;
        Vec::from_raw_parts(ptr, 0, size)
    };
    for byte in buffer {
        align_buf.push(*byte);
    }

    //Since AESND::play_voice uses Vec::len() to get the length of the buffer we make sure its
    //padded by setting the length.
    //
    // SAFETY: Capacity have already been allocated and zeroed out. all bytes have been moved
    // to the new buffer.
    //
    unsafe { align_buf.set_len(size) }

    align_buf
}
