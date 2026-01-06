use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The tinyusdz version/commit to use
const TINYUSDZ_VERSION: &str = "f85cdb6ad60ffb67aeab907e9da8a644f7bd8815";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // First check if tinyusdz exists as a sibling directory (for local development)
    let local_tinyusdz = manifest_dir.parent().unwrap().join("tinyusdz");
    let tinyusdz_dir = if local_tinyusdz.join("CMakeLists.txt").exists() {
        println!("cargo:warning=Using local tinyusdz source at {:?}", local_tinyusdz);
        local_tinyusdz
    } else {
        // Download tinyusdz source
        download_tinyusdz(&out_dir)
    };

    let build_dir = out_dir.join("build");

    // Create build directory
    fs::create_dir_all(&build_dir).expect("Failed to create build directory");

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

fn download_tinyusdz(out_dir: &PathBuf) -> PathBuf {
    let tinyusdz_dir = out_dir.join(format!("tinyusdz-{}", TINYUSDZ_VERSION));

    // Check if already downloaded
    if tinyusdz_dir.join("CMakeLists.txt").exists() {
        println!("cargo:warning=Using cached tinyusdz source at {:?}", tinyusdz_dir);
        return tinyusdz_dir;
    }

    println!("cargo:warning=Downloading tinyusdz source...");

    let archive_url = format!(
        "https://github.com/syoyo/tinyusdz/archive/{}.tar.gz",
        TINYUSDZ_VERSION
    );
    let archive_path = out_dir.join("tinyusdz.tar.gz");

    // Download using curl (available on most systems)
    let download_status = Command::new("curl")
        .args(["-L", "-o"])
        .arg(&archive_path)
        .arg(&archive_url)
        .status()
        .expect("Failed to run curl. Please install curl or provide tinyusdz source manually.");

    if !download_status.success() {
        panic!("Failed to download tinyusdz source from {}", archive_url);
    }

    // Extract using tar
    let extract_status = Command::new("tar")
        .args(["-xzf"])
        .arg(&archive_path)
        .arg("-C")
        .arg(out_dir)
        .status()
        .expect("Failed to run tar. Please install tar or provide tinyusdz source manually.");

    if !extract_status.success() {
        panic!("Failed to extract tinyusdz archive");
    }

    // Clean up archive
    let _ = fs::remove_file(&archive_path);

    println!("cargo:warning=Downloaded tinyusdz source to {:?}", tinyusdz_dir);
    tinyusdz_dir
}
