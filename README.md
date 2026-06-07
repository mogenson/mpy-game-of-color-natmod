# MicroPython Game of Color Native Module

A MicroPython native module (`.mpy`) written in Rust that implements Conway's Game of Life with a color component ("Game of Color"). Designed to run on resource-constrained microcontrollers (specifically targeting ARM Cortex-M0/M0+ devices like the RP2040 using the `thumbv6m-none-eabi` target).

## Overview

This native module implements a colorful cellular automaton on a double-buffered 53x11 grid. Each cell is represented not just by a binary state, but by a combination of:
*   **Hue** ($0^{\circ}$ to $359^{\circ}$): Determines the color of the cell.
*   **Light** (0 to 100): Determines the brightness/intensity.

The logic is calculated natively in Rust for maximum performance, and updates are passed back to MicroPython via a callback function.

---

## How It Works

### Cell States
*   **Alive**: A cell with `light > 50` (spawned cells start at `light = 100`).
*   **Dead**: A cell with `light <= 50` (newly deceased cells start at `light = 50`).

### State Transition Rules
During each simulation step, every cell's next state is determined by its current state and the number of active neighbors (using a wrapping toroidal grid):

1.  **Underpopulation**: A live cell with less than 2 live neighbors dies (`light` becomes 50).
2.  **Survival**: A live cell with 2 or 3 live neighbors remains alive (no change).
3.  **Overpopulation**: A live cell with more than 3 live neighbors dies (`light` becomes 50).
4.  **Reproduction**: A dead cell with exactly 3 or 4 live neighbors becomes alive (`light` becomes 100).
    *   The new cell's **Hue** is computed as the average of its neighbors' hues with a small random perturbation to allow colors to drift and evolve over time (calculated via `hueverage`).
5.  **Fading Glow**: Dead cells that stay dead have their brightness/light level decreased by 2 each step until they reach 0, creating a fading trail effect.

### Stalling Detection
If the number of cells born is equal to the number of cells that died in a single step, the simulation detects a potential stall. If this condition persists for 5 consecutive steps, the universe automatically repopulates itself using a random seed and a 50% initial density.

---

## Codebase Structure

The project integrates Rust code with C glue logic for MicroPython's dynamic native modules:

*   [src/lib.rs](file:///Users/mike/Code/mpy-game-of-color-natmod/src/lib.rs): Defines the dynamic module API entry points:
    *   `init(seed)`: Initializes the global [Universe](file:///Users/mike/Code/mpy-game-of-color-natmod/src/universe.rs#L17-L24) with a given random seed and populates it.
    *   `step(draw_fn)`: Advances the simulation by one step and invokes the MicroPython callback `draw_fn(x, y, r, g, b)` for every cell on the grid.
    *   `panic(...)`: Implements a no-std panic handler that logs failure messages to the MicroPython terminal.
*   [src/universe.rs](file:///Users/mike/Code/mpy-game-of-color-natmod/src/universe.rs): Implements the core cellular automaton logic, containing:
    *   [Universe](file:///Users/mike/Code/mpy-game-of-color-natmod/src/universe.rs#L17-L24): The main simulation state.
    *   `Universe::step`: Advances the simulation, handles double-buffering page flips, checks for stalls, and converts HSL values to RGB.
    *   `hueverage`: Computes a circular average of cell hues using trigonometric functions (`libm::cosf` and `libm::sinf`) with random offsets.
    *   `hue_to_rgb`: Converts HSL (hue, saturation, light) values to standard RGB coordinates.
*   [src/mpy_types.rs](file:///Users/mike/Code/mpy-game-of-color-natmod/src/mpy_types.rs): Declares FFI type definitions (`MpObjT`, `MpIntT`) and binds MicroPython runtime functions (`mpy_call_function_n_kw`, `mpy_obj_get_int`, etc.).
*   [src/mpy_print.rs](file:///Users/mike/Code/mpy-game-of-color-natmod/src/mpy_print.rs): Exposes dynamic module logging macros (`mpy_print!` and `mpy_println!`) which write directly to the MicroPython system console using `ufmt`.
*   [glue.c](file:///Users/mike/Code/mpy-game-of-color-natmod/glue.c): Bridge C code that registers the MicroPython module (`mpy_init`), exposes global symbols (`init`, `step`), and redirects utility helper functions to Rust.

---

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/) with the `thumbv6m-none-eabi` target installed:
    ```bash
    rustup target add thumbv6m-none-eabi
    ```
*   [MicroPython source code](https://github.com/micropython/micropython) (referenced by `MPY_DIR` in the Makefile).
*   An ARM cross-compilation toolchain (`arm-none-eabi-gcc`).
*   `mpremote` for uploading the built native module to your board.

Alternatively, if you use [Nix](https://nixos.org/), a [shell.nix](file:///Users/mike/Code/mpy-game-of-color-natmod/shell.nix) file is provided which sets up the correct MicroPython version (v1.28.0), toolchains, and env variables automatically.

### Compilation

To build the module:

1.  Enter the Nix development shell (optional but recommended):
    ```bash
    nix-shell
    ```
2.  Run `make` to compile the Rust library and the `.mpy` native module wrapper:
    ```bash
    make
    ```

This generates `build/universe.native.mpy`.

### Deploying

To upload the compiled module to your MicroPython device:
```bash
make upload
```
This copies the module to `/lib/universe.mpy` on the device.

To enter a REPL session immediately after:
```bash
make repl
```

---

## License

This project is licensed under the [MIT License](file:///Users/mike/Code/mpy-game-of-color-natmod/LICENSE).
