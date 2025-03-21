fn main() {
    use cmake::Config;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

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
    
    // Get LLVM library path
    let output = Command::new(llvm_config)
        .arg("--libdir")
        .output()
        .expect("Failed to execute llvm-config");
    
    let lib_path = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output from llvm-config")
        .trim()
        .to_string();

    // Add LLVM library path
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=clang-10");

    // Initialize clang-sys with the library path
    unsafe {
        env::set_var("LIBCLANG_PATH", &lib_path);
    }
    clang_sys::load().expect("Could not find libclang");

    let dst = Config::new("./")
        .define("BUILD_TESTING", "OFF")
        .define("CMAKE_C_COMPILER_WORKS", "1")
        .define("CMAKE_CXX_COMPILER_WORKS", "1")
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
