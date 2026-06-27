#![no_std]
mod mpy_print;
mod mpy_types;
mod universe;

use crate::mpy_types::*;
use crate::universe::Universe;

use core::cell::UnsafeCell;

struct UniverseCell(UnsafeCell<Option<Universe>>);
unsafe impl Sync for UniverseCell {}

static UNIVERSE: UniverseCell = UniverseCell(UnsafeCell::new(None));

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init(seed_obj: MpObjT) -> MpObjT {
    let seed = unsafe { mpy_obj_get_int(seed_obj) } as u64;
    let mut universe = Universe::new(seed);
    universe.populate();
    unsafe {
        *UNIVERSE.0.get() = Some(universe);
        mpy_const_none()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn step(draw_fn: MpObjT) -> MpObjT {
    unsafe {
        if let Some(universe) = (*UNIVERSE.0.get()).as_mut() {
            universe.step(|x, y, r, g, b| {
                let args_obj = [x, y, r, g, b].map(|v| mpy_obj_new_int(v));
                mpy_call_function_n_kw(draw_fn, 5, 0, args_obj.as_ptr());
            });
        }
        mpy_const_none()
    }
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
