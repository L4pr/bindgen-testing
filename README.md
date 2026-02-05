# bindgen-testing

A comprehensive demonstration project showing how to build a C library with CMake and seamlessly call it from Rust using bindgen for automatic FFI binding generation.

## What This Repository Does

This project demonstrates **Rust-C interoperability** by:
- Building a C library (calculator and analytics functions) as a static library using CMake
- Automatically generating Rust FFI bindings from C header files using `bindgen`
- Calling C functions from Rust with type-safe wrappers
- Managing memory allocated in C from Rust using RAII patterns
- Accessing C structs, constants, and global variables from Rust

This implementation demonstrates linking to a **pre-built** C library, without using the `cc` crate to compile C code from Rust. The C library is built separately with CMake, then the Rust build links against it.

## Quick Start Tutorial

Follow these three simple steps to build and run this project:

### Step 1: Build the C Library with CMake

First, build the C library using CMake:

```bash
mkdir -p build
cd build
cmake ..
make
cd ..
```

This creates `libcalculator.a` (static library) in the `build/` directory, containing all the compiled C functions.

### Step 2: Build the Rust Application

After the C library is built, build the Rust application:

```bash
cargo build
```

During this step:
- The `build.rs` script runs automatically
- `bindgen` generates Rust FFI bindings from C headers
- The Rust compiler links against the pre-built C library

### Step 3: Run the Rust Application

Finally, run the application:

```bash
cargo run
```

You should see output demonstrating all the C functions being called from Rust!

## Project Structure

```
bindgen-testing/
├── c_lib/              # C library source code
│   ├── calculator.c    # Basic math functions (add, subtract, multiply)
│   ├── calculator.h    # Calculator function declarations
│   ├── analytics.c     # Advanced functions (distance, heap management)
│   ├── analytics.h     # Analytics function declarations
│   ├── common_types.h  # Shared C types and constants
│   └── wrapper.h       # Main header including all components
├── src/
│   ├── main.rs         # Main Rust application demonstrating C calls
│   └── safe_wrapper.rs # Safe Rust wrappers around C functions
├── CMakeLists.txt      # CMake configuration for C library
├── build.rs            # Rust build script using bindgen
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## Prerequisites

Before building this project, ensure you have:

- **CMake** 3.10 or higher - For building the C library
- **C Compiler** (GCC, Clang, or MSVC) - For compiling C code
- **Rust Toolchain** (rustc, cargo) - For building the Rust application
  - Install from [rustup.rs](https://rustup.rs/)
- **libclang** - Required by bindgen for parsing C headers
  - Ubuntu/Debian: `sudo apt-get install libclang-dev`
  - macOS: `brew install llvm`
  - Windows: Install LLVM from [llvm.org](https://llvm.org/)

## Detailed Build Instructions

### Step 1: Build C Library with CMake

```bash
# Create build directory
mkdir -p build
cd build

# Run CMake to configure the build
cmake ..

# Build the static library
make

# Return to project root
cd ..
```

**What happens:** CMake compiles `calculator.c` and `analytics.c` into a static library `libcalculator.a` in the `build/` directory.

### Step 2: Build Rust Application

```bash
cargo build
```

**What happens:**
1. Cargo runs `build.rs` before compiling Rust code
2. `build.rs` invokes `bindgen` to generate Rust bindings from `c_lib/wrapper.h`
3. Generated bindings are saved to `$OUT_DIR/bindings.rs`
4. `build.rs` tells the linker where to find the C library (`build/`)
5. Cargo compiles the Rust code and links it with `libcalculator.a`

### Step 3: Run the Application

```bash
cargo run
```

Or run the compiled binary directly:
```bash
./target/debug/rust_bindgen_example
```

## Expected Output

```
=== Rust + C Complete Integration ===

Math Check:
  10 + 5 = 15
  10 - 5 = 5
  10 * 5 = 50

Global Check:
  System Limit (from C): 100

Struct Check (Stack):
  Calculated Distance: 5.00

Memory Check (Heap):
  [Scope Start] Creating SmartPoint...
  [Usage] SmartPoint X: 15.0
  [Usage] SmartPoint Y: 20.0
  [Scope End] heap_p is about to go out of scope...
  [Rust] Automatically freeing C memory by dropping...
  [After Scope] Execution continues safe and sound.
```

## How It Works

### 1. C Library (`c_lib/`)

The C library provides several functionalities:

- **Calculator functions**: Basic arithmetic operations (add, subtract, multiply)
- **Analytics functions**: Distance calculations, heap memory management
- **Data types**: Structs like `DataPoint` for coordinate data
- **Constants**: Global constants like `SYSTEM_LIMIT`

### 2. CMake Build (`CMakeLists.txt`)

The CMake configuration:
- Defines a static library target named `calculator`
- Compiles all C source files
- Configures include directories
- Produces `libcalculator.a` in the build directory

### 3. Bindgen Integration (`build.rs`)

The Rust build script:
- Runs during `cargo build` before compiling Rust code
- Uses `bindgen::Builder` to parse C headers
- Generates type-safe Rust FFI bindings automatically
- Emits linker directives (`cargo:rustc-link-search`, `cargo:rustc-link-lib`)
- Does **not** compile C code (no `cc` crate)

### 4. Safe Rust Wrappers (`src/safe_wrapper.rs`)

Provides safe, idiomatic Rust wrappers:
- Functions with Rust types instead of raw C types
- Structs that implement Rust traits
- RAII patterns for automatic C memory management
- Type conversions between Rust and C

### 5. Rust Application (`src/main.rs`)

Demonstrates various integration patterns:
- Calling C functions from Rust
- Passing data between Rust and C
- Using C structs in Rust
- Accessing C constants and globals
- Managing C heap memory safely

## Features Demonstrated

### ✅ Basic Function Calls
Simple C functions called from Rust with automatic type conversion.

### ✅ Struct Usage
C structs used in Rust with full type safety.

### ✅ Memory Management
Heap-allocated C memory managed safely from Rust using Drop trait.

### ✅ Constants and Globals
C preprocessor constants and global variables accessible from Rust.

### ✅ Safe Wrappers
Idiomatic Rust wrappers that hide unsafe FFI calls.

## Advantages of This Approach

- **Separation of Concerns**: C and Rust builds are completely independent
- **No `cc` Crate Dependency**: Rust build doesn't need to know how to compile C code
- **Flexible C Build System**: Use any build system (CMake, Make, Ninja, etc.)
- **Works with Pre-built Libraries**: Can link against any existing C library
- **Automatic Binding Generation**: `bindgen` handles tedious FFI declarations
- **Type Safety**: Generated bindings are type-safe and checked at compile time

## Troubleshooting

**Error: "cannot find -lcalculator"**
- Make sure you completed Step 1 (CMake build) before running `cargo build`
- Verify that `build/libcalculator.a` exists

**Error: "failed to run custom build command for rust_bindgen_example"**
- Ensure `libclang` is installed
- Check that C header files exist in `c_lib/`

**Error: "undefined reference to add/subtract/multiply"**
- The C library wasn't linked properly
- Rebuild the C library with `cd build && make clean && make && cd ..`

## Learning Resources

- [The Rustonomicon - FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/)
- [CMake Tutorial](https://cmake.org/cmake/help/latest/guide/tutorial/index.html)

## License

This is a demonstration project for educational purposes.