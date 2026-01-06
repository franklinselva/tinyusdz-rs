use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tinyusdz_dir = manifest_dir.parent().unwrap().join("tinyusdz");
    let build_dir = out_dir.join("build");

    // Create build directory
    std::fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    // Configure with CMake
    let cmake_status = Command::new("cmake")
        .current_dir(&build_dir)
        .arg(&tinyusdz_dir)
        .arg("-DTINYUSDZ_WITH_C_API=ON")
        .arg("-DTINYUSDZ_WITH_TYDRA=ON")
        .arg("-DTINYUSDZ_BUILD_TESTS=OFF")
        .arg("-DTINYUSDZ_BUILD_EXAMPLES=OFF")
        .arg("-DTINYUSDZ_BUILD_SHARED_LIBS=OFF")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .status()
        .expect("Failed to run cmake");

    if !cmake_status.success() {
        panic!("CMake configuration failed");
    }

    // Build with CMake
    let build_status = Command::new("cmake")
        .current_dir(&build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release")
        .arg("--parallel")
        .status()
        .expect("Failed to run cmake build");

    if !build_status.success() {
        panic!("CMake build failed");
    }

    // Link the static libraries from the build directory
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    // Note: Libraries are named with _static suffix
    println!("cargo:rustc-link-lib=static=c-tinyusd_static");
    println!("cargo:rustc-link-lib=static=tinyusdz_static");

    // Link C++ standard library
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=msvcrt");

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("wrapper.h").to_str().unwrap())
        .clang_arg(format!("-I{}", tinyusdz_dir.join("src").display()))
        // Whitelist C API functions and types
        .allowlist_function("c_tinyusd_.*")
        .allowlist_type("CTinyUSD.*")
        .allowlist_type("c_tinyusd_.*")
        .allowlist_var("C_TINYUSD_.*")
        // Generate Rust enums from C enums
        .rustified_enum("CTinyUSDFormat")
        .rustified_enum("CTinyUSDAxis")
        .rustified_enum("CTinyUSDValueType")
        .rustified_enum("CTinyUSDPrimType")
        // Other options
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to OUT_DIR
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rebuild if these files change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
}
