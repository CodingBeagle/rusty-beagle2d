/*
    Build Scripts are code which can be executed BEFORE the actual crate is compiled and build.
*/
extern crate bindgen;

use std::path::Path;
use std::path::PathBuf;
use std::env;

fn main() {
    // Add linking search directories
    let project_directory = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=dylib=glfw3-x64-debug");
    println!("cargo:rustc-link-search=native={}", Path::new(&project_directory).join("libs\\glfw").display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for the resulting bindings
    let bindings = bindgen::Builder::default()
        // Input headers we would like to generate bindings for.
        .header("glfw3.h")
        .header("glfw3native.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // Included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the result and panic on failure.
        .expect("Unable to generate bindings");

    // write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}