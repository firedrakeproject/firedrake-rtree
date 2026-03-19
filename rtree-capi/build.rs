extern crate cbindgen;

use std::env;

fn generate_bindings() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config::from_root_or_default(&crate_dir);

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/rtree-capi.h");
}

fn main() {
    generate_bindings();
}
