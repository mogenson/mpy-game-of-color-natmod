#![allow(unused)]

use core::ffi::{c_char, c_void};

pub type MpObjT = usize; // mp_obj_t
pub type MpIntT = isize; // mp_int_t

#[repr(C)]
pub struct MpPrintT {
    pub env: *mut c_void,
    pub print_strn: unsafe extern "C" fn(env: *mut c_void, str: *const c_char, len: usize),
}

unsafe extern "C" {
    pub fn mpy_call_function_n_kw(
        fn_obj: MpObjT,
        n_args: usize,
        n_kw: usize,
        args: *const MpObjT,
    ) -> MpObjT;
    pub fn mpy_get_plat_print() -> *const MpPrintT;
    pub fn mpy_obj_get_int(obj: MpObjT) -> MpIntT;
    pub fn mpy_obj_new_int(val: MpIntT) -> MpObjT;
    pub fn mpy_obj_new_str(data: *const u8, len: usize) -> MpObjT;
    pub fn mpy_const_none() -> MpObjT;
}
