use std::path::Path;
use std::env;

fn main() {
    // Add linking search directories
    let project_directory = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&project_directory).join("libs\\glfw").display());
}