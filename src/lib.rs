#![no_std]
mod mpy_print;
mod mpy_types;
mod universe;

use crate::mpy_types::*;
use crate::universe::Universe;

static mut UNIVERSE: Option<Universe> = None;

#[no_mangle]
pub unsafe extern "C" fn init(seed_obj: MpObjT) -> MpObjT {
    let seed = mpy_obj_get_int(seed_obj) as u64;
    let mut universe = Universe::new(seed);
    universe.populate();
    UNIVERSE = Some(universe);
    mpy_const_none()
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn step(draw_fn: MpObjT) -> MpObjT {
    if let Some(universe) = UNIVERSE.as_mut() {
        universe.step(|x, y, r, g, b| {
            let args_obj = [x, y, r as usize, g as usize, b as usize]
                .map(|v| unsafe { mpy_obj_new_int(v as isize) });
            mpy_call_function_n_kw(draw_fn, 5, 0, args_obj.as_ptr());
        });
    }
    mpy_const_none()
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
