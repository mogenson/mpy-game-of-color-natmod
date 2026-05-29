#![no_std]
mod mpy_print;
mod mpy_types;
mod universe;

use crate::mpy_types::{
    mpy_call_function_n_kw, mpy_obj_get_int, mpy_obj_new_int, MpIntT, MpObjT, NONE,
};
use crate::universe::Universe;

static mut DRAW_FN: Option<MpObjT> = None;
static mut UNIVERSE: Option<Universe> = None;

#[no_mangle]
pub unsafe extern "C" fn init(seed_obj: MpObjT, draw_fn_obj: MpObjT) -> MpObjT {
    let seed = mpy_obj_get_int(seed_obj) as u64;
    let mut universe = Universe::new(seed);
    universe.populate();
    UNIVERSE = Some(universe);
    DRAW_FN = Some(draw_fn_obj);
    NONE
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn step() -> MpObjT {
    if let (Some(universe), Some(draw_fn)) = (UNIVERSE.as_mut(), DRAW_FN) {
        universe.step(|x, y, r, g, b| {
            let args_obj = [x, y, r.into(), g.into(), b.into()];
            mpy_call_function_n_kw(draw_fn, 5, 0, args_obj.as_ptr() as *const MpObjT);
        });
    }
    NONE
}

#[no_mangle]
pub unsafe extern "C" fn factorial(x_obj: MpObjT) -> MpObjT {
    let x = mpy_obj_get_int(x_obj) as u32;
    mpy_println!("factorial({}):", x);

    let mut result: u32 = 1;
    for step in 1..=x {
        result *= step;
        mpy_println!("  step {}: {} * {} = {}", step, result / step, step, result);

        if let Some(draw_fn) = DRAW_FN {
            let result_obj = mpy_obj_new_int(result as MpIntT);
            mpy_call_function_n_kw(draw_fn, 1, 0, &result_obj as *const MpObjT);
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
