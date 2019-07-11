//! Utility Functions to convert between types.

use std::ffi::c_void;

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

/// Converts uncached memory into cached memory (K0 type into K1 type).
pub fn mem_k0_to_k1(x: *mut c_void) -> *mut c_void {
    ((x as u32) + (ogc_sys::SYS_BASE_UNCACHED - ogc_sys::SYS_BASE_CACHED)) as *mut c_void
}
