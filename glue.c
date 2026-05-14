#include "py/dynruntime.h"

// ── Rust function declarations ────────────────────────────────────────────────
extern mp_obj_t py_factorial(mp_obj_t x_obj);

// ── Function objects defined in C ────────────────────────────────────────────
// MP_DEFINE_CONST_FUN_OBJ_1 emits an R_ARM_ABS32 rodata relocation,
// which mpy_ld.py handles. Doing the equivalent in Rust produces
// R_ARM_THM_GOT_BREL (type 96) which mpy_ld.py does not handle.
static MP_DEFINE_CONST_FUN_OBJ_1(factorial_obj, py_factorial);

// ── Helpers callable from Rust ────────────────────────────────────────────────
mp_int_t mpy_obj_get_int(mp_obj_t obj) { return mp_obj_get_int(obj); }
mp_obj_t mpy_obj_new_int(mp_int_t val) { return mp_obj_new_int(val); }

// ── Module entry point ────────────────────────────────────────────────────────
mp_obj_t mpy_init(mp_obj_fun_bc_t *self, size_t n_args, size_t n_kw, mp_obj_t *args) {
    MP_DYNRUNTIME_INIT_ENTRY
    mp_store_global(MP_QSTR_factorial, MP_OBJ_FROM_PTR(&factorial_obj));
    MP_DYNRUNTIME_INIT_EXIT
}
