# bindgen-testing

A demonstration project showing how to build a C library with CMake and call it from Rust using bindgen.

This implementation demonstrates linking to a **pre-built** C library, without using the `cc` crate to compile C code from Rust.

## Project Structure

- `c_lib/` - Simple C library with basic calculator functions
  - `calculator.h` - Header file with function declarations
  - `calculator.c` - Implementation of calculator functions
- `CMakeLists.txt` - CMake build configuration for the C library
- `build.rs` - Rust build script that uses bindgen to generate Rust bindings
- `src/main.rs` - Rust application that calls the C functions

## Prerequisites

- CMake 3.10 or higher
- GCC or compatible C compiler
- Rust toolchain (cargo, rustc)
- libclang (required by bindgen)

## Building

This project requires a **two-step build process**:

### Step 1: Build the C Library with CMake

First, build the C library using CMake:

```bash
mkdir -p build
cd build
cmake ..
make
cd ..
```

This will create `libcalculator.a` in the `build/` directory.

### Step 2: Build and Run the Rust Application

After the C library is built, you can build and run the Rust application:

```bash
cargo build
cargo run
```

The build.rs script will:
1. Generate Rust bindings from the C header using `bindgen`
2. Link the Rust application to the pre-built C library in the `build/` directory

**Note**: The Rust build expects to find `libcalculator.a` in the `build/` directory. Make sure to complete Step 1 before running `cargo build`.

## How It Works

1. **C Library**: The `c_lib/` directory contains a simple calculator library with three functions: `add`, `subtract`, and `multiply`.

2. **CMake Build**: The `CMakeLists.txt` file defines how to build the C library as a static library using CMake. This is done **before** building the Rust application.

3. **Bindgen Integration**: The `build.rs` file is a Rust build script that:
   - Uses `bindgen` to automatically generate Rust FFI bindings from the C header file
   - Tells the Rust linker where to find the pre-built C library (`build/libcalculator.a`)
   - Does **not** compile the C code (no `cc` crate dependency)

4. **Rust Application**: The `src/main.rs` file includes the generated bindings and calls the C functions from Rust code.

## Advantages of This Approach

- **Separation of concerns**: C and Rust builds are completely separate
- **No `cc` crate dependency**: The Rust build doesn't need to know how to compile C code
- **Flexible C build**: You can use any build system for the C library (CMake, Make, etc.)
- **Pre-built libraries**: Works with any pre-built C library, not just source code

## Example Output

```
=== Rust Bindgen Example ===
Calling C functions from Rust using bindgen

add(10, 5) = 15
subtract(10, 5) = 5
multiply(10, 5) = 50

Successfully called C functions from Rust!
```