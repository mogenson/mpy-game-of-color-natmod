use core::ffi::{c_char, c_void};

pub type MpObjT = usize;
pub type MpIntT = isize;

#[repr(C)]
pub struct MpPrintT {
    pub env: *mut c_void,
    pub print_strn: unsafe extern "C" fn(env: *mut c_void, str: *const c_char, len: usize),
}

extern "C" {
    pub fn mpy_get_plat_print() -> *const MpPrintT;
    pub fn mpy_obj_get_int(obj: MpObjT) -> MpIntT;
    pub fn mpy_obj_new_int(val: MpIntT) -> MpObjT;
}
