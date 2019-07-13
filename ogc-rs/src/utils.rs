//! Utility Functions to convert between types.

/// Converts a raw *mut u8 into a String.
pub fn raw_to_string(raw: *mut u8) -> String {
    unsafe {
        let slice = std::slice::from_raw_parts(raw, 1);
        String::from_utf8(slice.to_vec()).unwrap()
    }
}

/// Converts a raw *mut *mut u8 into a String vector.
pub fn raw_to_strings(raw: *mut *mut u8) -> Vec<String> {
    unsafe {
        let slice = std::slice::from_raw_parts(raw, 2);
        slice
            .into_iter()
            .map(|x: &*mut u8| {
                let r = std::slice::from_raw_parts(*x, 1);
                String::from_utf8(r.to_vec()).unwrap()
            })
            .collect()
    }
}

/// OS memory casting macros.
mod memory_casting {
    /// Cast a cached address to a uncached address.
    /// Example: 0x8xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_uncached {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) + (ogc_sys::SYS_BASE_UNCACHED - ogc_sys::SYS_BASE_CACHED)) as *mut c_void
        }};
    }

    /// Cast a cached address to a physical address.
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_cached_to_physical {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) - ogc_sys::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a uncached address to a cached address.
    /// Example: 0xCxxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_cached {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) - (ogc_sys::SYS_BASE_UNCACHED - ogc_sys::SYS_BASE_CACHED)) as *mut c_void
        }};
    }

    /// Cast a uncached address to a physical address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_uncached_to_physical {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) - ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a cached address.
    /// Example: 0x0xxxxxxx -> 0x8xxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_cached {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) + ogc_sys::SYS_BASE_CACHED) as *mut c_void
        }};
    }

    /// Cast a physical address to a uncached address.
    /// Example: 0x0xxxxxxx -> 0xCxxxxxxx
    #[macro_export]
    macro_rules! mem_physical_to_uncached {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) + ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }

    /// Cast a virtual address to a physical address.  
    /// Example: 0x8xxxxxxx -> 0x0xxxxxxx
    #[macro_export]
    macro_rules! mem_virtual_to_physical {
        ( $x:expr ) => {{
            use std::ffi::c_void;

            (($x as u32) & !ogc_sys::SYS_BASE_UNCACHED) as *mut c_void
        }};
    }
}
