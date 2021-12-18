//! The ``tpl`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the texture functions found in ``tpl.h``.

use crate::ffi;
use alloc::{vec::Vec, boxed::Box};

#[derive(Copy, Clone)]
enum FileType {
    Disc = 0,
    Memory = 1,
}

struct Tpl {
    type_: FileType,
    ntextures: u32,
    texdesc: Vec<u8>,
    tpl_file: ffi::FHANDLE,
}

impl Tpl {
    fn new() -> Self {
        Self {
            type_: FileType::Disc,
            ntextures: 0,
            texdesc: Vec::new(),
            tpl_file: 0 as _,
        }
    }

    fn open_tpl_from_memory(&mut self, memory: &mut [u8]) {
        unsafe { ffi::TPL_OpenTPLFromMemory(self.raw(), memory.as_mut_ptr() as *mut _, memory.len() as u32 )};
    }

    fn raw(&mut self) -> *mut ffi::TPLFile {
        Box::into_raw(Box::new(ffi::TPLFile {
            type_: self.type_ as _,
            ntextures: self.ntextures as _,
            texdesc: self.texdesc.as_mut_ptr() as *mut _,
            tpl_file: self.tpl_file,
        }))
    }
}
