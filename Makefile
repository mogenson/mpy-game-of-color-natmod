# ── Paths ─────────────────────────────────────────────────────────────────────
MPY_DIR  := ../micropython          # path to MicroPython repo root
MOD      := my_module
ARCH     := armv6m                  # RP2040 = Cortex-M0+ = armv6m

# Rust produces a static library; we extract the object file we need.
RUST_LIB := target/thumbv6m-none-eabi/release/libmy_module.a
RUST_OBJ := build/rust_lib.o

# ── Sources ───────────────────────────────────────────────────────────────────
SRC_C    := glue.c
# We add the Rust extracted object to the link step below.

# ── Bring in MicroPython build rules ─────────────────────────────────────────
include $(MPY_DIR)/py/dynruntime.mk

# ── Build Rust ────────────────────────────────────────────────────────────────
$(RUST_LIB): src/lib.rs Cargo.toml
	cargo build --release

# Extract a single relocatable .o from the Rust static library.
# mpy_ld.py works with .o files, not .a archives.
$(RUST_OBJ): $(RUST_LIB) | build
	$(CROSS)ld -r --whole-archive $< -o $@

build:
	mkdir -p build

# ── Override the link target to include Rust ──────────────────────────────────
# dynruntime.mk defines a rule that links all OBJ files.
# We append our Rust object to OBJ before that rule fires.
OBJ += $(RUST_OBJ)
