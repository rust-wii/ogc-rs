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
        slice.into_iter().map(|x: &*mut u8| {
            let r = std::slice::from_raw_parts(*x, 1);
            String::from_utf8(r.to_vec()).unwrap()
        }).collect()
    }
}
