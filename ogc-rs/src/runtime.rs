//! The ``runtime`` module of ``ogc-rs``.
//!
//! This module implements runtime functions and allocators
//! required for ``no_std`` on the Wii.
//!
//! Most of the functions defined here are modified functions from
//! the helpful project [water](https://github.com/lemarcuspoilus/water).

use crate::{print, println};
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};

/// Uses the system's memory allocation and de-allocation functions.
///
/// # Remarks
/// This allocator _will panic_ if more than 16MB of memory is being allocated.
/// Since the Wii has approximately 84MB of total memory, such an allocation is probably an error.
pub struct OGCAllocator;

unsafe impl GlobalAlloc for OGCAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > 1024 * 1024 * 16 {
            panic!(
                "Attempted to allocate >16MB (asked for {} bytes)",
                layout.size()
            );
        } else {
            ogc_sys::malloc(layout.size() as u32) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ogc_sys::free(ptr as *mut _);
    }
}

/// Panic Handler for the Wii.
///
/// **Note**: The panic handler uses the ``println`` macro for output.
/// In order for this to work ``Console`` and a minimal ``Video`` setup is required!
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    println!("#######################################");
    println!("# <[ PANIC ]> {} ", panic_info);
    println!("#######################################");

    loop {}
}

/// Allocation Error Handler for the Wii.
///
/// **Note**: The allocation error handler uses the ``println`` macro for output.
/// In order for this to work ``Console`` and a minimal ``Video`` setup is required!
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    println!("#######################################");
    println!("# <[ ALLOC ]> Allocation Error!");
    println!(
        "# <[ ALLOC ]> Size: {} - Alignment: {}",
        layout.size(),
        layout.align()
    );
    println!("#######################################");

    loop {}
}
