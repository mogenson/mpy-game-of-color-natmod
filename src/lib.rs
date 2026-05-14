#![no_std]
#![no_main] // mpy_init is our entry point, not main

use core::panic::PanicInfo;

// ── FFI types that mirror MicroPython's C types ──────────────────────────────
// mp_obj_t is a pointer-sized word on all MicroPython targets.
pub type MpObjT = usize;
pub type MpIntT = isize;
pub type Qstr = u16; // matches qstr in MicroPython source
pub type MpObjFunBcT = core::ffi::c_void; // opaque

// ── Extern declarations for our C glue ───────────────────────────────────────
extern "C" {
    fn mpy_dynruntime_init_entry(
        self_: *mut MpObjFunBcT,
        n_args: usize,
        n_kw: usize,
        args: *mut MpObjT,
    );
    fn mpy_dynruntime_init_exit();
    fn mpy_store_global(name: Qstr, obj: MpObjT);
    fn mpy_obj_get_int(obj: MpObjT) -> MpIntT;
    fn mpy_obj_new_int(val: MpIntT) -> MpObjT;
}

// ── MicroPython function object boilerplate ───────────────────────────────────
// MP_DEFINE_CONST_FUN_OBJ_1 in C expands to a static struct of this shape.
// We reproduce it in Rust so we can pass a pointer to mp_store_global.
//
// Layout must match mp_obj_fun_builtin_fixed_t in py/obj.h.
// All fields are pointer-sized; keep repr(C) to prevent reordering.
#[repr(C)]
struct MpFunObj {
    base_type: usize, // &mp_type_fun_builtin_1 — filled by init entry macro
    fun: usize,       // pointer to our Rust function, cast to usize
}

// These static objects live in BSS (zero-initialised at startup — fine for
// the function pointer; the base_type is set by the dynruntime init macro).
// BSS globals are supported; *initialised* data statics are NOT.
static mut FACTORIAL_OBJ: MpFunObj = MpFunObj {
    base_type: 0,
    fun: 0,
};

// ── Your actual logic ─────────────────────────────────────────────────────────

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

/// Python-callable wrapper: receives one mp_obj_t, returns one mp_obj_t.
///
/// Safety: called by MicroPython with a valid mp_obj_t.
#[no_mangle]
pub unsafe extern "C" fn py_factorial(x_obj: MpObjT) -> MpObjT {
    let x = mpy_obj_get_int(x_obj) as u32;
    let result = factorial(x) as MpIntT;
    mpy_obj_new_int(result)
}

// ── Module entry point ────────────────────────────────────────────────────────
/// Called by MicroPython when `import my_module` executes.
///
/// The signature must exactly match what dynruntime.mk expects.
#[no_mangle]
pub unsafe extern "C" fn mpy_init(
    self_: *mut MpObjFunBcT,
    n_args: usize,
    n_kw: usize,
    args: *mut MpObjT,
) -> MpObjT {
    // 1. Set up the module globals dict (must be first).
    mpy_dynruntime_init_entry(self_, n_args, n_kw, args);

    // 2. Point our function object at the Rust function.
    //    (base_type is filled in by the init entry macro above.)
    FACTORIAL_OBJ.fun = py_factorial as usize;

    // 3. Register the function in the module namespace.
    mpy_store_global(mpy_qstr_factorial(), (&raw const FACTORIAL_OBJ) as usize);

    // 4. Restore previous globals dict (must be last).
    mpy_dynruntime_init_exit();

    // Return value is ignored by MicroPython but must be a valid mp_obj_t.
    0 // mp_const_none
}

// ── Required for no_std ───────────────────────────────────────────────────────
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
