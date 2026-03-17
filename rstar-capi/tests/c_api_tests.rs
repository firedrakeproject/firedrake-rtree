use std::path::PathBuf;
use std::process::Command;

#[test]
fn c_api() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let lib_dir = manifest_dir.join("target").join("debug");
    let include_dir = manifest_dir.join("include");
    let test_src = manifest_dir.join("tests").join("test.c");
    let test_binary = lib_dir.join("test_c_api");

    let compile_status = Command::new("gcc")
        .args([
            "-Wall",
            "-Werror",
            "-O1",
            "-g",
            "-I",
            include_dir.to_str().unwrap(),
            "-L",
            lib_dir.to_str().unwrap(),
            "-lrstar_capi",
            "-o",
            test_binary.to_str().unwrap(),
            test_src.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to compile");

    assert!(
        compile_status.success(),
        "C compilation failed (exit {})",
        compile_status
    );

    let run_status = Command::new(&test_binary)
        .status()
        .expect("Failed to launch C test binary");

    assert!(
        run_status.success(),
        "C API tests failed (exit {})",
        run_status
    );
}
