use crate::c_api::{xbuf::XBuf, ASCIICode, StrNumber};
use std::cell::RefCell;

const MAX_GLOB_STRS: usize = 10;
pub const GLOB_STR_SIZE: usize = 20000;

pub struct GlobalData {
    glb_bib_str_ptr: XBuf<StrNumber>,
    global_strs: XBuf<ASCIICode>,
    glb_str_end: XBuf<i32>,
    num_glb_strs: i32,
}

impl GlobalData {
    fn new() -> GlobalData {
        GlobalData {
            glb_bib_str_ptr: XBuf::new(MAX_GLOB_STRS),
            global_strs: XBuf::new((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS),
            glb_str_end: XBuf::new(MAX_GLOB_STRS),
            num_glb_strs: 0,
        }
    }

    fn grow(&mut self) {
        self.glb_bib_str_ptr.grow(MAX_GLOB_STRS);
        self.global_strs.grow((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS);
        self.glb_str_end.grow(MAX_GLOB_STRS);
    }
}

thread_local! {
    pub static GLOBALS: RefCell<GlobalData> = RefCell::new(GlobalData::new());
}

pub fn reset() {
    GLOBALS.with(|globals| *globals.borrow_mut() = GlobalData::new());
}

fn with_globals<T>(f: impl FnOnce(&GlobalData) -> T) -> T {
    GLOBALS.with(|globals| f(&globals.borrow()))
}

fn with_globals_mut<T>(f: impl FnOnce(&mut GlobalData) -> T) -> T {
    GLOBALS.with(|globals| f(&mut globals.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn num_glb_strs() -> i32 {
    with_globals(|globals| globals.num_glb_strs)
}

#[no_mangle]
pub extern "C" fn set_num_glb_strs(val: i32) {
    with_globals_mut(|globals| globals.num_glb_strs = val)
}

#[no_mangle]
pub extern "C" fn check_grow_global_strs() {
    with_globals_mut(|globals| {
        if globals.num_glb_strs as usize == globals.glb_bib_str_ptr.len() {
            globals.grow();
        }
    })
}

#[no_mangle]
pub extern "C" fn glb_bib_str_ptr(pos: usize) -> usize {
    with_globals(|globals| globals.glb_bib_str_ptr[pos])
}

#[no_mangle]
pub extern "C" fn set_glb_bib_str_ptr(pos: usize, val: usize) {
    with_globals_mut(|globals| globals.glb_bib_str_ptr[pos] = val)
}

#[no_mangle]
pub extern "C" fn global_strs(pos: usize) -> ASCIICode {
    with_globals(|globals| globals.global_strs[pos])
}

#[no_mangle]
pub extern "C" fn set_global_strs(pos: usize, val: ASCIICode) {
    with_globals_mut(|globals| globals.global_strs[pos] = val)
}

#[no_mangle]
pub extern "C" fn glb_str_end(pos: usize) -> i32 {
    with_globals(|globals| globals.glb_str_end[pos])
}

#[no_mangle]
pub extern "C" fn set_glb_str_end(pos: usize, val: i32) {
    with_globals_mut(|globals| globals.glb_str_end[pos] = val)
}
