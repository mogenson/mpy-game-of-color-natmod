#include "py/dynruntime.h"

// ── Rust function declarations
extern mp_obj_t init(mp_obj_t seed);
extern mp_obj_t step(mp_obj_t draw_fn);

// ── Function objects defined in C ────────────────────────────────────────────
static MP_DEFINE_CONST_FUN_OBJ_1(step_obj, step);
static MP_DEFINE_CONST_FUN_OBJ_1(init_obj, init);

// ── Helpers callable from Rust
mp_int_t mpy_obj_get_int(mp_obj_t obj) { return mp_obj_get_int(obj); }

mp_obj_t mpy_obj_new_int(mp_int_t val) { return mp_obj_new_int(val); }

const mp_print_t *mpy_get_plat_print(void) { return &mp_plat_print; }

mp_obj_t mpy_call_function_n_kw(mp_obj_t fn, size_t n_args, size_t n_kw,
                                const mp_obj_t *args) {
  return mp_call_function_n_kw(fn, n_args, n_kw, args);
}

mp_obj_t mpy_obj_new_str(const char *data, size_t len) {
  return mp_obj_new_str(data, len);
}

mp_obj_t mpy_const_none(void) { return mp_const_none; }

// ── Module entry point
mp_obj_t mpy_init(mp_obj_fun_bc_t *self, size_t n_args, size_t n_kw,
                  mp_obj_t *args) {
  MP_DYNRUNTIME_INIT_ENTRY
  mp_store_global(MP_QSTR_init, MP_OBJ_FROM_PTR(&init_obj));
  mp_store_global(MP_QSTR_step, MP_OBJ_FROM_PTR(&step_obj));
  MP_DYNRUNTIME_INIT_EXIT
}
