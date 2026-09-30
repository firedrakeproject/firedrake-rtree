extern crate cbindgen;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn set_soname() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "linux"
        || env::var_os("CARGO_FEATURE_PYTHON").is_none()
    {
        return;
    }

    println!("cargo:rerun-if-env-changed=PYO3_PYTHON");
    println!("cargo:rerun-if-env-changed=PYO3_ENVIRONMENT_SIGNATURE");
    let output = Command::new(env::var_os("PYO3_PYTHON").expect("Build with maturin"))
        .args([
            "-c",
            "import sysconfig; print(sysconfig.get_config_var('EXT_SUFFIX'))",
        ])
        .output()
        .expect("Failed to query Python");
    assert!(output.status.success(), "Failed to query Python");
    let suffix = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-link-arg-cdylib=-Wl,-soname,firedrake_rtree{}", suffix.trim());
}

fn generate_bindings() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::from_root_or_default(&crate_dir);

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/rtree-capi.h");

    let py_include = Path::new(&crate_dir)
        .join("..")
        .join("firedrake_rtree")
        .join("include");
    fs::create_dir_all(&py_include).expect("Failed to create Python include directory");
    fs::copy(
        Path::new(&crate_dir).join("include").join("rtree-capi.h"),
        py_include.join("rtree-capi.h"),
    )
    .expect("Failed to copy header to Python package");
}

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=Cargo.toml");
    set_soname();
    generate_bindings();
}
