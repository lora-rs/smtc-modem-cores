fn main() {
    use cmake::Config;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed=SWL2001/lbm_lib/smtc_modem_core/radio_drivers/sx126x_driver/src/sx126x.h"
    );

    let llvm_config_names = [
        env::var("LLVM_CONFIG_PATH").ok(),
        Some("llvm-config-18".to_string()), // Try latest first
        Some("llvm-config".to_string()),    // Fallback
    ];

    let llvm_config = llvm_config_names
        .iter()
        .flatten()
        .find(|name| Command::new(name).arg("--version").output().is_ok())
        .expect("Could not find llvm-config. Please install llvm or set LLVM_CONFIG_PATH");

    let lib_path = String::from_utf8(
        Command::new(llvm_config)
            .arg("--libdir")
            .output()
            .expect("Failed to run llvm-config --libdir")
            .stdout,
    )
        .expect("Invalid UTF-8 in llvm-config --libdir")
        .trim()
        .to_string();

    let include_path = String::from_utf8(
        Command::new(llvm_config)
            .arg("--includedir")
            .output()
            .expect("Failed to run llvm-config --includedir")
            .stdout,
    )
        .expect("Invalid UTF-8 in llvm-config --includedir")
        .trim()
        .to_string();

    // Tell Cargo to pass these to dependent crates (like clang-sys)
    println!("cargo:rustc-env=LIBCLANG_PATH={}", lib_path);
    println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config);

    // Link libraries
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=dylib=clang");
    println!("cargo:rustc-link-lib=dylib=LLVM");

    // Build C library with cmake
    let dst = Config::new("./")
        .define("BUILD_TESTING", "OFF")
        .define("CMAKE_C_COMPILER_WORKS", "1")
        .define("CMAKE_CXX_COMPILER_WORKS", "1")
        .define("LLVM_DIR", format!("{}/lib/cmake/llvm", include_path))
        .pic(false)
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=smtc-modem-cores");

    let bindings = bindgen::Builder::default()
        .raw_line("use cty;")
        .use_core()
        .ctypes_prefix("cty")
        .detect_include_paths(true)
        .header("SWL2001/lbm_lib/smtc_modem_core/radio_drivers/sx126x_driver/src/sx126x.h")
        .clang_arg(format!("-I{}/include", dst.display()))
        .clang_arg(format!("-I{}", include_path))
        .trust_clang_mangling(false)
        .allowlist_type("sx126x_status_e")
        .rustified_enum("sx126x_status_e")
        .allowlist_type("sx126x_sleep_cfgs_e")
        .rustified_enum("sx126x_sleep_cfgs_e")
        .allowlist_function("sx126x_set_sleep")
        .generate()
        .expect("Failed to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("smtc-modem-cores.rs"))
        .expect("Couldn't write bindings!");
}

