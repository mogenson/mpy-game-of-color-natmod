#![no_std]
mod mpy_print;
mod mpy_types;

use crate::mpy_types::{mpy_call_function_n_kw, mpy_obj_get_int, mpy_obj_new_int, MpIntT, MpObjT};

static mut PLOT_FN: MpObjT = 0;

#[no_mangle]
pub unsafe extern "C" fn set_plot_function(fn_obj: MpObjT) -> MpObjT {
    PLOT_FN = fn_obj;
    fn_obj // Return the stored callable so callers can confirm what was set.
}

#[no_mangle]
pub unsafe extern "C" fn factorial(x_obj: MpObjT) -> MpObjT {
    let x = mpy_obj_get_int(x_obj) as u32;
    mpy_println!("factorial({}):", x);

    let mut result: u32 = 1;
    for step in 1..=x {
        result *= step;
        mpy_println!("  step {}: {} * {} = {}", step, result / step, step, result);

        if PLOT_FN != 0 {
            let result_obj = mpy_obj_new_int(result as MpIntT);
            mpy_call_function_n_kw(PLOT_FN, 1, 0, &result_obj as *const MpObjT);
        }
    }

    mpy_println!("result: {}", result);
    mpy_obj_new_int(result as MpIntT)
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    if let Some(message) = panic_info.message().as_str() {
        mpy_println!("panicked: {}", message);
    }
    if let Some(location) = panic_info.location() {
        mpy_println!(
            "panicked in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    }

    loop {}
}
