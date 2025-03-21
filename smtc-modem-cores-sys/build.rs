fn main() {
    use cmake::Config;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    // Tell cargo to rerun this script if any of these files change
    println!("cargo:rerun-if-changed=SWL2001/lbm_lib/smtc_modem_core/radio_drivers/sx126x_driver/src/sx126x.h");
    println!("cargo:rerun-if-changed=build.rs");

    // Try different possible llvm-config names
    let llvm_config_names = [
        env::var("LLVM_CONFIG_PATH").ok(),
        Some("llvm-config-10".to_string()),
        Some("llvm-config".to_string()),
    ];

    let llvm_config = llvm_config_names
        .iter()
        .flatten()
        .find(|name| Command::new(name).arg("--version").output().is_ok())
        .expect("Could not find llvm-config. Please install llvm-10 or set LLVM_CONFIG_PATH");
    
    // Get LLVM library path and include path
    let lib_path = String::from_utf8(
        Command::new(llvm_config)
            .arg("--libdir")
            .output()
            .expect("Failed to execute llvm-config")
            .stdout
    ).expect("Invalid UTF-8 output from llvm-config").trim().to_string();

    let include_path = String::from_utf8(
        Command::new(llvm_config)
            .arg("--includedir")
            .output()
            .expect("Failed to execute llvm-config")
            .stdout
    ).expect("Invalid UTF-8 output from llvm-config").trim().to_string();

    // Set environment variables for clang-sys
    unsafe {
        env::set_var("LIBCLANG_PATH", &lib_path);
        env::set_var("LLVM_CONFIG_PATH", llvm_config);
    }

    // Initialize clang-sys with the library path
    clang_sys::load().expect("Could not find libclang");

    // Add LLVM library path and include path for both build and test
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=dylib=clang-10");
    println!("cargo:rustc-link-lib=dylib=LLVM-10");
    println!("cargo:rustc-link-search=native={}/lib", include_path);

    let dst = Config::new("./")
        .define("BUILD_TESTING", "OFF")
        .define("CMAKE_C_COMPILER_WORKS", "1")
        .define("CMAKE_CXX_COMPILER_WORKS", "1")
        .define("LLVM_DIR", format!("{}/lib/cmake/llvm", include_path))
        .pic(false)
        .build();

    // Make library paths available for both build and test
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
        .expect("Failed to generate sx12xx bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("smtc-modem-cores.rs")).expect("Couldn't write bindings!");
}
