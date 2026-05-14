#![no_std]

pub type MpObjT = usize;
pub type MpIntT = isize;

extern "C" {
    fn mpy_obj_get_int(obj: MpObjT) -> MpIntT;
    fn mpy_obj_new_int(val: MpIntT) -> MpObjT;
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

/// Called directly by the C function object — no Rust-side struct needed.
#[no_mangle]
pub unsafe extern "C" fn py_factorial(x_obj: MpObjT) -> MpObjT {
    let x = mpy_obj_get_int(x_obj) as u32;
    mpy_obj_new_int(factorial(x) as MpIntT)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
