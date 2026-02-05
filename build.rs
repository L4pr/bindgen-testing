use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for the C library
    println!("cargo:rustc-link-search=native=build");
    println!("cargo:rustc-link-lib=static=calculator");

    // Tell cargo to invalidate the built crate whenever the C code changes
    println!("cargo:rerun-if-changed=c_lib/calculator.h");
    println!("cargo:rerun-if-changed=c_lib/calculator.c");

    // Compile the C library
    cc::Build::new()
        .file("c_lib/calculator.c")
        .compile("calculator");

    // Generate bindings
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
