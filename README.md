# bindgen-testing

A demonstration project showing how to build a C library with CMake and call it from Rust using bindgen.

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

### Building the C Library with CMake

```bash
mkdir -p build
cd build
cmake ..
make
```

This will create `libcalculator.a` in the `build/` directory.

### Building and Running the Rust Application

The Rust application automatically compiles the C library and generates bindings:

```bash
cargo build
cargo run
```

The build.rs script will:
1. Compile the C library using the `cc` crate
2. Generate Rust bindings using `bindgen`
3. Link the C library with the Rust application

## How It Works

1. **C Library**: The `c_lib/` directory contains a simple calculator library with three functions: `add`, `subtract`, and `multiply`.

2. **CMake Build**: The `CMakeLists.txt` file defines how to build the C library as a static library using CMake. This demonstrates that the C library can be built independently.

3. **Bindgen Integration**: The `build.rs` file is a Rust build script that:
   - Uses the `cc` crate to compile the C code
   - Uses `bindgen` to automatically generate Rust FFI bindings from the C header file
   - Links the compiled C library with the Rust binary

4. **Rust Application**: The `src/main.rs` file includes the generated bindings and calls the C functions from Rust code.

## Example Output

```
=== Rust Bindgen Example ===
Calling C functions from Rust using bindgen

add(10, 5) = 15
subtract(10, 5) = 5
multiply(10, 5) = 50

Successfully called C functions from Rust!
```