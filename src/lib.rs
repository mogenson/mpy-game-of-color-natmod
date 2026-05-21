#![no_std]
mod mpy_print;
mod mpy_types;

use crate::mpy_types::{mpy_obj_get_int, mpy_obj_new_int, MpIntT, MpObjT};

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

#[no_mangle]
pub unsafe extern "C" fn py_factorial(x_obj: MpObjT) -> MpObjT {
    let x = mpy_obj_get_int(x_obj) as u32;
    mpy_println!("factorial({}):", x);
    let result = factorial(x);
    mpy_println!("result: {}", result);
    mpy_obj_new_int(result as MpIntT)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
