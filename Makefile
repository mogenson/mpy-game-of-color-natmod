MPY_DIR  := $(abspath ../micropython)
MOD      := my_module
ARCH     := armv6m

SRC := glue.c

RUST_LIB := $(abspath target/thumbv6m-none-eabi/release/libmy_module.a)

MPY_LD_FLAGS += -l $(RUST_LIB)

include $(MPY_DIR)/py/dynruntime.mk

build/glue.o: $(RUST_LIB)

$(RUST_LIB): src/lib.rs Cargo.toml
	cargo build --release
