use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=build");
    println!("cargo:rustc-link-lib=static=calculator");

    println!("cargo:rerun-if-changed=c_lib");

    let bindings = bindgen::Builder::default()
        .header("c_lib/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
