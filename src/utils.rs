//! Utility Functions to convert between types.

use alloc::{string::String, vec::Vec};
use core::slice;

/// Converts a raw *mut u8 into a String.
pub fn raw_to_string(raw: *mut u8) -> String {
    let slice = unsafe { slice::from_raw_parts(raw, 1) };
    String::from_utf8(slice.to_vec()).unwrap()
}

/// Converts a raw *mut *mut u8 into a String vector.
pub fn raw_to_strings(raw: *mut *mut u8) -> Vec<String> {
    let slice = unsafe { slice::from_raw_parts(raw, 2) };
    slice
        .iter()
        .map(|x: &*mut u8| {
            let r = unsafe { slice::from_raw_parts(*x, 1) };
            String::from_utf8(r.to_vec()).unwrap()
        })
        .collect()
}

/// OS memory casting macros.
mod memory_casting {
    /// Cast a cached address to a uncached address.
    /// Example: 0x8xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_uncached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + (ogc_sys::SYS_BASE_UNCACHED - ogc_sys::SYS_BASE_CACHED)) as *mut c_void
        }};
    }

    /// Cast a cached address to a physical address.
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - ogc_sys::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a uncached address to a cached address.
    /// Example: 0xCxxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_cached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - (ogc_sys::SYS_BASE_UNCACHED - ogc_sys::SYS_BASE_CACHED)) as *mut c_void
        }};
    }

    /// Cast a uncached address to a physical address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) - ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a cached address.
    /// Example: 0x0xxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_cached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + ogc_sys::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a uncached address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_uncached {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) + ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a virtual address to a physical address.  
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_virtual_to_physical {
        ( $x:expr ) => {{
            use core::ffi::c_void;

            (($x as u32) & !ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
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
