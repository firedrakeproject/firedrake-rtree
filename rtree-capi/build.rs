extern crate cbindgen;

use std::env;
use std::fs;
use std::path::Path;

fn generate_bindings() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::from_root_or_default(&crate_dir);

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/rtree-capi.h");

    let py_include = Path::new(&crate_dir).join("..").join("firedrake_rtree").join("include");
    fs::create_dir_all(&py_include).expect("Failed to create Python include directory");
    fs::copy(
        Path::new(&crate_dir).join("include").join("rtree-capi.h"),
        py_include.join("rtree-capi.h"),
    ).expect("Failed to copy header to Python package");
}

fn main() {
    generate_bindings();
}
