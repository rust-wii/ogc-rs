//! Utility Functions to convert between types.

use core::alloc::{Allocator, Layout};
use core::ptr::NonNull;

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


/// A heap-allocated buffer guaranteed to be aligned to a 32-byte boundary.
///
/// This buffer does not grow or reallocate. It's meant as a simple way to
/// handle the alignment requirements everpresent throughout libogc functions
/// that take buffers as parameters.
#[derive(Debug)]
pub struct Buf32(NonNull<[u8]>);

impl Buf32 {
	/// Allocates a new buffer at least `min_len` bytes long. Rounds up the size
	/// to the next multiple of 32.
	///
	/// # Panics
	/// Panics if rounding up `min_len` to the next multiple of 32 would
	/// overflow.
	pub fn new(min_len: usize) -> Self {
		// round len to lowest multiple of 32
		let padding = (32 - min_len % 32) % 32;
		let len = min_len.checked_add(padding).expect("length overflow");

		// SAFETY:
		// * align is non-zero and a power of two.
		// * `len` is checked above to not overflow `usize::MAX`.
		let layout = unsafe { Layout::from_size_align_unchecked(len, 32) };

		let block = match alloc::alloc::Global.allocate_zeroed(layout) {
			Ok(block) => block,
			Err(_) => alloc::alloc::handle_alloc_error(layout),
		};

		Buf32(block)
	}

	/// Returns the number of bytes in the buffer.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns an unsafe mutable pointer to the buffer.
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.0.as_mut_ptr()
	}

	/// Extracts a slice of the entire buffer.
	pub fn as_slice(&self) -> &[u8] {
		// SAFETY: `self.0` is aligned, dereferenceable, initialized, and
		//         enforces aliasing rules by binding the reference's lifetime
		//         to that of `&self`.
		unsafe { self.0.as_ref() }
	}

	/// Extracts a mutable slice of the entire buffer.
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		// SAFETY: `self.0` is aligned, dereferenceable, initialized, and
		//         enforces aliasing rules by binding the reference's lifetime
		//         to that of `&mut self`.
		unsafe { self.0.as_mut() }
	}
}

impl Clone for Buf32 {
	fn clone(&self) -> Self {
		let mut new_buf = Self::new(self.len());
		new_buf.clone_from(self);
		new_buf
	}
	
	fn clone_from(&mut self, source: &Self) {
		self.as_mut_slice().copy_from_slice(source.as_slice());
	}
}

impl Drop for Buf32 {
	fn drop(&mut self) {
		// SAFETY:
		// * from_size_align_unchecked():
		//   * align is non-zero and a power of two.
		//   * `len` is already known to not overflow `usize::MAX`.
		// * deallocate():
		//   * `self.0` is currently allocated.
		//   * `layout` fits the block, using the size given to us.
		unsafe {
			let layout = Layout::from_size_align_unchecked(self.len(), 32);
			alloc::alloc::Global.deallocate(self.0.as_non_null_ptr(), layout);
		}
	}
}
