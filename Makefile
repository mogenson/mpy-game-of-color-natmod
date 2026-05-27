MPY_DIR  ?= $(abspath ../micropython)
MOD      := factorial
ARCH     := armv6m

SRC := glue.c

RUST_LIB := $(abspath target/thumbv6m-none-eabi/release/lib$(MOD).a)

MPY_LD_FLAGS += -l $(RUST_LIB)

include $(MPY_DIR)/py/dynruntime.mk

build/glue.o: $(RUST_LIB)

$(RUST_LIB): $(wildcard src/*.rs) Cargo.toml
	cargo build --release

upload: build/glue.o
	mpremote fs cp build/$(MOD).native.mpy :/lib/$(MOD).mpy

repl: build/glue.o
	mpremote repl
