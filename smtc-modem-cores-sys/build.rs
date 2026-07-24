fn main() {
    use cmake::Config;
    use std::env;
    use std::path::PathBuf;

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
        .header("SWL2001/lbm_lib/smtc_modem_core/radio_drivers/sx126x_driver/src/sx126x_hal.h")
        .clang_arg(format!("-I{}/include", dst.display()))
        .trust_clang_mangling(false)
        .allowlist_type("sx126x_status_e")
        .rustified_enum("sx126x_status_e")
        .allowlist_type("sx126x_hal_status_e")
        .rustified_enum("sx126x_hal_status_e")
        .allowlist_type("sx126x_sleep_cfgs_e")
        .rustified_enum("sx126x_sleep_cfgs_e")
        .allowlist_type("sx126x_standby_cfgs_e")
        .rustified_enum("sx126x_standby_cfgs_e")
        .allowlist_type("sx126x_lora_sf_e")
        .rustified_enum("sx126x_lora_sf_e")
        .allowlist_type("sx126x_lora_bw_e")
        .rustified_enum("sx126x_lora_bw_e")
        .allowlist_type("sx126x_lora_cr_e")
        .rustified_enum("sx126x_lora_cr_e")
        .allowlist_type("sx126x_lora_pkt_len_modes_e")
        .rustified_enum("sx126x_lora_pkt_len_modes_e")
        .allowlist_type("sx126x_mod_params_lora_s")
        .allowlist_type("sx126x_pkt_params_lora_s")
        .allowlist_function("sx126x_set_sleep")
        .allowlist_function("sx126x_set_standby")
        .allowlist_function("sx126x_set_rf_freq")
        .allowlist_function("sx126x_set_lora_mod_params")
        .allowlist_function("sx126x_set_lora_pkt_params")
        .allowlist_function("sx126x_set_lora_sync_word")
        .allowlist_function("sx126x_set_buffer_base_address")
        .allowlist_function("sx126x_write_buffer")
        .allowlist_function("sx126x_set_tx")
        .allowlist_function("sx126x_set_dio_irq_params")
        .allowlist_function("sx126x_clear_irq_status")
        .allowlist_type("sx126x_cad_symbs_e")
        .rustified_enum("sx126x_cad_symbs_e")
        .allowlist_type("sx126x_cad_exit_modes_e")
        .rustified_enum("sx126x_cad_exit_modes_e")
        .allowlist_type("sx126x_cad_param_s")
        .allowlist_function("sx126x_stop_timer_on_preamble")
        .allowlist_function("sx126x_set_lora_symb_nb_timeout")
        .allowlist_function("sx126x_cfg_rx_boosted")
        .allowlist_function("sx126x_set_rx_with_timeout_in_rtc_step")
        .allowlist_function("sx126x_set_cad_params")
        .allowlist_function("sx126x_set_cad")
        .generate()
        .expect("Failed to generate sx12xx bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("smtc-modem-cores.rs"))
        .expect("Couldn't write bindings!");
}
