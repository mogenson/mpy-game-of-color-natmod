#include "py/dynruntime.h"

// ── Rust function declarations
extern mp_obj_t factorial(mp_obj_t x_obj);
extern mp_obj_t set_plot_function(mp_obj_t fn_obj);

// ── Function objects defined in C ────────────────────────────────────────────
static MP_DEFINE_CONST_FUN_OBJ_1(factorial_obj, factorial);
static MP_DEFINE_CONST_FUN_OBJ_1(set_plot_function_obj, set_plot_function);

// ── Helpers callable from Rust
mp_int_t mpy_obj_get_int(mp_obj_t obj) { return mp_obj_get_int(obj); }

mp_obj_t mpy_obj_new_int(mp_int_t val) { return mp_obj_new_int(val); }

const mp_print_t *mpy_get_plat_print(void) { return &mp_plat_print; }

mp_obj_t mpy_call_function_n_kw(mp_obj_t fn, size_t n_args, size_t n_kw,
                                const mp_obj_t *args) {
  return mp_call_function_n_kw(fn, n_args, n_kw, args);
}

// ── Module entry point
mp_obj_t mpy_init(mp_obj_fun_bc_t *self, size_t n_args, size_t n_kw,
                  mp_obj_t *args) {
  MP_DYNRUNTIME_INIT_ENTRY
  mp_store_global(MP_QSTR_factorial, MP_OBJ_FROM_PTR(&factorial_obj));
  mp_store_global(MP_QSTR_set_plot_function,
                  MP_OBJ_FROM_PTR(&set_plot_function_obj));

  MP_DYNRUNTIME_INIT_EXIT
}
