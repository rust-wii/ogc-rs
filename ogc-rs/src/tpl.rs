pub struct Tpl;

pub enum Filetype {
    Memory,
    Disc,
}

pub struct Tplfile {
    file_type: Filetype,
    ntextures: Option<u32>,
    texdesc: Option<u32>,
    tpl_file: Option<u32>,
}

impl Tpl {
    pub fn open_tpl_from_memory(
        tdf: *mut ogc_sys::TPLFile,
        memory: *mut libc::c_void,
        len: u32,
    ) -> i32 {
        let file = ogc_sys::_tplfile {
            type_: 0,
            ntextures: 0,
            texdesc: 0 as *mut libc::c_void,
            tpl_file: 0 as *mut libc::c_void,
        };
        unsafe { ogc_sys::TPL_OpenTPLFromMemory(tdf, memory, len) }
    }
}
