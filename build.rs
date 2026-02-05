use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for the C library in the build directory
    // The library should be built with CMake before running cargo build
    println!("cargo:rustc-link-search=native=build");
    println!("cargo:rustc-link-lib=static=calculator");

    // Tell cargo to invalidate the built crate whenever the C header changes
    // Note: If C source changes, you must rebuild the library with CMake first
    println!("cargo:rerun-if-changed=c_lib/calculator.h");

    // Generate bindings from the C header file
    let bindings = bindgen::Builder::default()
        .header("c_lib/calculator.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
