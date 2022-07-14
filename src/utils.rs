//! Utility Functions to convert between types.

use core::alloc::{Allocator, Layout};
use core::{fmt, ops};
use core::ptr::NonNull;

use alloc::vec::Vec;

/// OS memory pointer casting.
/// 
/// For more information, refer to [Memory map](https://wiibrew.org/wiki/Memory_Map).
pub mod mem {
    use crate::ffi;

    pub const BASE_CACHED: usize = ffi::SYS_BASE_CACHED as _;
    pub const BASE_UNCACHED: usize = ffi::SYS_BASE_UNCACHED as _;

    /// Cast an address into an uncached address.
    /// Examples:
    /// * `0x8xxx_xxxx` -> `0xCxxx_xxxx`
    /// * `0x9xxx_xxxx` -> `0xDxxx_xxxx`
    #[inline]
    pub fn to_uncached(addr: usize) -> usize {
        to_physical(addr) + BASE_UNCACHED
    }

    /// Cast an address into a cached address.
    /// Examples:
    /// * `0xCxxx_xxxx` -> `0x8xxx_xxxx`
    /// * `0xDxxx_xxxx` -> `0x9xxx_xxxx`
    #[inline]
    pub fn to_cached(addr: usize) -> usize {
        to_physical(addr) + BASE_CACHED
    }

    /// Cast a virtual address (cached or uncached) into a physical address.
    /// Examples:
    /// * `0x8xxx_xxxx` -> `0x0xxx_xxxx`
    /// * `0x9xxx_xxxx` -> `0x1xxx_xxxx`
    /// * `0xCxxx_xxxx` -> `0x0xxx_xxxx`
    /// * `0xDxxx_xxxx` -> `0x1xxx_xxxx`
    #[inline]
    pub fn to_physical(addr: usize) -> usize {
        addr & !BASE_UNCACHED
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
#[derive(Eq)]
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
		min_len.checked_add(padding).expect("length overflow");

		// SAFETY:
		// * align is non-zero and a power of two.
		// * `min_len` is checked above to not overflow `usize::MAX` after rounding up
		//   for alignment.
		let layout = unsafe { Layout::from_size_align_unchecked(min_len, 32) };

		let block = match alloc::alloc::Global.allocate_zeroed(layout) {
			Ok(block) => block,
			Err(_) => alloc::alloc::handle_alloc_error(layout),
		};

		Buf32(block)
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

impl core::ops::Deref for Buf32 {
	type Target = [u8];
	
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl core::ops::DerefMut for Buf32 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl ops::Index<usize> for Buf32 {
	type Output = u8;
	
	fn index(&self, index: usize) -> &Self::Output {
		&self.as_slice()[index]
	}
}

impl ops::IndexMut<usize> for Buf32 {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.as_mut_slice()[index]
	}
}

macro_rules! impl_index_for_buf32 {
	($($idx:ty),*) => {
		$(
			impl ops::Index<$idx> for Buf32 {
				type Output = [u8];
				
				fn index(&self, index: $idx) -> &Self::Output {
					&self.as_slice()[index]
				}
			}
			
			impl ops::IndexMut<$idx> for Buf32 {
				fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
					&mut self.as_mut_slice()[index]
				}
			}
		)*
	}
}

impl_index_for_buf32! {
	ops::RangeFull,
	ops::RangeFrom<usize>,
	ops::RangeTo<usize>,
	ops::Range<usize>,
	ops::RangeInclusive<usize>,
	ops::RangeToInclusive<usize>
}

impl fmt::Debug for Buf32 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_slice().fmt(f)
	}
}

impl core::cmp::Ord for Buf32 {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		self.as_slice().cmp(other.as_slice())
	}
}

impl core::cmp::PartialOrd for Buf32 {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl core::cmp::PartialEq for Buf32 {
	fn eq(&self, other: &Self) -> bool {
		self.as_slice() == other.as_slice()
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
