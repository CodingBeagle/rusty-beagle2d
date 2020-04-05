extern crate bindgen;

use std::path::Path;
use std::path::PathBuf;
use std::env;

fn main() {
    // Add linking search directoris
    let project_directory = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-lib=dylib=freetype-64");
    println!("cargo:rustc-link-search=native={}", Path::new(&project_directory).join("libs").display());


    let bindings = bindgen::Builder::default()
        // Input headers we would like to generate bindings for.
        .header("headers/wrapper.h")
        .clang_arg("-ID:\\repos\\rusty-beagle2d\\rusty-beagle2d-freetype\\headers")
        .clang_arg("-v")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the result and panic on failure
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("freetype-bindings.rs"))
        .expect("Couldn't write bindings!");
}