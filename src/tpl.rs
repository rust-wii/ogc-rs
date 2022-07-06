//! The ``tpl`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the texture functions found in
//! ``tpl.h``.

use crate::{ffi, gx};
use alloc::boxed::Box;

#[derive(Copy, Clone)]
enum FileType {
	Disc = 0,
	Memory = 1,
}

struct Tpl {
	type_: FileType,
	ntextures: u32,
	texdesc: *mut libc::c_void,
	tpl_file: ffi::FHANDLE,
}

impl Tpl {
	pub fn open_from_memory(memory: &mut [u8]) -> Self {
		let mut tpl = Self {
			type_: FileType::Memory,
			ntextures: 0,
			texdesc: 0 as _,
			tpl_file: 0 as _,
		};
		unsafe {
			ffi::TPL_OpenTPLFromMemory(
				tpl.raw(),
				memory.as_mut_ptr() as *mut _,
				memory.len() as _,
			)
		};
		tpl
	}

	// Loads texture by id into texture
	pub fn get_texture(&mut self, id: u32, texture: &mut gx::Texture) {
		unsafe { ffi::TPL_GetTexture(self.raw(), id as _, texture.gxtexobj()) };
	}

	fn raw(&mut self) -> *mut ffi::TPLFile {
		Box::into_raw(Box::new(ffi::TPLFile {
			type_: self.type_ as _,
			ntextures: self.ntextures as _,
			texdesc: self.texdesc,
			tpl_file: self.tpl_file,
		}))
	}
}

impl Drop for Tpl {
	fn drop(&mut self) {
		unsafe { ffi::TPL_CloseTPLFile(self.raw()) }
	}
}
