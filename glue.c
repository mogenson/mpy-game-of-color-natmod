// Include the MicroPython dynamic runtime header.
// This gives us mp_obj_t, mp_store_global, MP_QSTR_*, mp_obj_new_int, etc.
#include "py/dynruntime.h"

// ── Wrappers for macros that Rust cannot call directly ──────────────────────

// Called at the very start of mpy_init — sets up the module's globals dict.
void mpy_dynruntime_init_entry(mp_obj_fun_bc_t *self, size_t n_args,
                               size_t n_kw, mp_obj_t *args) {
    MP_DYNRUNTIME_INIT_ENTRY   // expands to several statements
}

// Called at the very end of mpy_init — restores the previous globals dict.
void mpy_dynruntime_init_exit(void) {
    MP_DYNRUNTIME_INIT_EXIT
}

// Register a callable object in the module namespace under the given qstr name.
void mpy_store_global(qstr name, mp_obj_t obj) {
    mp_store_global(name, obj);
}

// ── Helpers your Rust code needs ─────────────────────────────────────────────

mp_int_t mpy_obj_get_int(mp_obj_t obj) {
    return mp_obj_get_int(obj);
}

mp_obj_t mpy_obj_new_int(mp_int_t val) {
    return mp_obj_new_int(val);
}

qstr mpy_qstr_factorial(void) {
    return MP_QSTR_factorial;
}

// Add more wrappers here as your module grows (strings, floats, lists, …)
