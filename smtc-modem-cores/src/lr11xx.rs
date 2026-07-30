use core::ffi;
pub use smtc_modem_cores_sys::{
    LR11XX_GNSS_SINGLE_ALMANAC_READ_SIZE, LR11XX_GNSS_SINGLE_ALMANAC_WRITE_SIZE,
};
pub use smtc_modem_cores_sys::{
    lr11xx_gnss_almanac_full_read_bytestream_t, lr11xx_gnss_constellation_mask_t,
    lr11xx_gnss_context_status_bytestream_t, lr11xx_gnss_context_status_t,
    lr11xx_gnss_cumulative_timing_t, lr11xx_gnss_demod_info_t, lr11xx_gnss_demod_status_t,
    lr11xx_gnss_detected_satellite_t, lr11xx_gnss_doppler_solver_result_t,
    lr11xx_gnss_fetch_time_option_t, lr11xx_gnss_freq_search_space_t,
    lr11xx_gnss_instantaneous_power_consumption_ua_t, lr11xx_gnss_read_almanac_status_t,
    lr11xx_gnss_solver_assistance_position_t, lr11xx_gnss_sv_type_t, lr11xx_gnss_time_t,
    lr11xx_gnss_version_t, lr11xx_gnss_visible_satellite_t,
    lr11xx_gnss_week_number_rollover_status_t, lr11xx_hal_status_e, lr11xx_lr_fhss_params_t,
    lr11xx_radio_cad_params_t, lr11xx_radio_mod_params_gfsk_t, lr11xx_radio_mod_params_lora_t,
    lr11xx_radio_pa_cfg_t, lr11xx_radio_pkt_params_gfsk_t, lr11xx_radio_pkt_params_lora_t,
    lr11xx_radio_pkt_status_gfsk_t, lr11xx_radio_pkt_status_lora_t,
    lr11xx_radio_rx_buffer_status_t, lr11xx_radio_stats_gfsk_t, lr11xx_radio_stats_lora_t,
    lr11xx_status_e, lr11xx_system_rfswitch_cfg_t, lr11xx_system_sleep_cfg_t,
    lr11xx_system_stat1_t, lr11xx_system_stat2_t, lr11xx_system_version_t,
};
use smtc_modem_cores_sys::{
    lr11xx_gnss_almanac_update, lr11xx_gnss_almanac_update_from_sat,
    lr11xx_gnss_compute_almanac_age, lr11xx_gnss_compute_power_consumption,
    lr11xx_gnss_config_almanac_update_period, lr11xx_gnss_config_delay_reset_assistance_position,
    lr11xx_gnss_fetch_time, lr11xx_gnss_get_almanac_age_for_satellite,
    lr11xx_gnss_get_context_status, lr11xx_gnss_get_detected_satellites,
    lr11xx_gnss_get_nb_detected_satellites, lr11xx_gnss_get_nb_visible_satellites,
    lr11xx_gnss_get_result_destination, lr11xx_gnss_get_result_size, lr11xx_gnss_get_sv_sync,
    lr11xx_gnss_get_visible_satellites, lr11xx_gnss_parse_context_status_buffer,
    lr11xx_gnss_push_dmc_msg, lr11xx_gnss_push_solver_msg, lr11xx_gnss_read_almanac,
    lr11xx_gnss_read_almanac_per_satellites, lr11xx_gnss_read_almanac_status,
    lr11xx_gnss_read_almanac_update, lr11xx_gnss_read_almanac_update_period,
    lr11xx_gnss_read_assistance_position, lr11xx_gnss_read_cumulative_timing,
    lr11xx_gnss_read_delay_reset_assistance_position, lr11xx_gnss_read_demod_status,
    lr11xx_gnss_read_doppler_solver_result, lr11xx_gnss_read_firmware_version,
    lr11xx_gnss_read_freq_search_space, lr11xx_gnss_read_gnss_rssi_test,
    lr11xx_gnss_read_keep_sync_status, lr11xx_gnss_read_last_scan_mode_launched,
    lr11xx_gnss_read_results, lr11xx_gnss_read_supported_constellations, lr11xx_gnss_read_time,
    lr11xx_gnss_read_used_constellations, lr11xx_gnss_read_week_number_rollover,
    lr11xx_gnss_reset_position, lr11xx_gnss_reset_time, lr11xx_gnss_scan,
    lr11xx_gnss_set_almanac_update, lr11xx_gnss_set_assistance_position,
    lr11xx_gnss_set_beidou_bit_mask_sat_activated, lr11xx_gnss_set_constellations_to_use,
    lr11xx_gnss_set_freq_search_space, lr11xx_gnss_set_gps_bit_mask_sat_activated,
    lr11xx_gnss_set_scan_mode, lr11xx_gnss_set_time, lr11xx_lr_fhss_build_frame,
    lr11xx_lr_fhss_init, lr11xx_radio_apply_high_acp_workaround, lr11xx_radio_auto_tx_rx,
    lr11xx_radio_cfg_rx_boosted, lr11xx_radio_get_gfsk_pkt_status, lr11xx_radio_get_gfsk_stats,
    lr11xx_radio_get_lora_pkt_status, lr11xx_radio_get_lora_stats, lr11xx_radio_get_rssi_inst,
    lr11xx_radio_get_rx_buffer_status, lr11xx_radio_reset_stats, lr11xx_radio_set_cad,
    lr11xx_radio_set_cad_params, lr11xx_radio_set_gfsk_crc_params,
    lr11xx_radio_set_gfsk_mod_params, lr11xx_radio_set_gfsk_pkt_params,
    lr11xx_radio_set_gfsk_sync_word, lr11xx_radio_set_gfsk_whitening_seed,
    lr11xx_radio_set_lora_mod_params, lr11xx_radio_set_lora_pkt_params,
    lr11xx_radio_set_lora_sync_timeout, lr11xx_radio_set_lora_sync_word,
    lr11xx_radio_set_lr_fhss_sync_word, lr11xx_radio_set_pa_cfg, lr11xx_radio_set_pkt_address,
    lr11xx_radio_set_pkt_type, lr11xx_radio_set_rf_freq, lr11xx_radio_set_rx,
    lr11xx_radio_set_rx_duty_cycle, lr11xx_radio_set_rx_tx_fallback_mode,
    lr11xx_radio_set_rx_with_timeout_in_rtc_step, lr11xx_radio_set_tx, lr11xx_radio_set_tx_cw,
    lr11xx_radio_set_tx_params, lr11xx_radio_set_tx_with_timeout_in_rtc_step,
    lr11xx_radio_stop_timeout_on_preamble, lr11xx_regmem_clear_rxbuffer,
    lr11xx_regmem_read_buffer8, lr11xx_regmem_read_mem8, lr11xx_regmem_read_regmem32,
    lr11xx_regmem_write_buffer8, lr11xx_regmem_write_mem8, lr11xx_regmem_write_regmem32,
    lr11xx_regmem_write_regmem32_mask, lr11xx_system_calibrate, lr11xx_system_calibrate_image,
    lr11xx_system_clear_errors, lr11xx_system_clear_irq_status,
    lr11xx_system_get_and_clear_irq_status, lr11xx_system_get_errors,
    lr11xx_system_get_random_number, lr11xx_system_get_status, lr11xx_system_get_temp,
    lr11xx_system_get_vbat, lr11xx_system_get_version, lr11xx_system_read_join_eui,
    lr11xx_system_read_uid, lr11xx_system_set_dio_as_rf_switch, lr11xx_system_set_dio_irq_params,
    lr11xx_system_set_reg_mode, lr11xx_system_set_sleep, lr11xx_system_set_standby,
    lr11xx_system_set_tcxo_mode,
};

use embedded_hal::spi::{Operation, SpiDevice};

/// Monomorphized HAL entry points for one concrete SPI type. C hands the
/// context pointer back on every HAL call, so dispatch goes through this
/// table rather than through a user-provided `#[no_mangle]` symbol.
///
/// Unlike the sx126x HAL, lr11xx reads are two chip-select assertions
/// (command write, then dummy-status-byte + data read), and there is an
/// additional command-less `direct_read`.
#[repr(C)]
struct HalVtable {
    write: unsafe fn(*mut ffi::c_void, command: &[u8], data: &[u8]) -> lr11xx_hal_status_e,
    read: unsafe fn(*mut ffi::c_void, command: &[u8], data: &mut [u8]) -> lr11xx_hal_status_e,
    direct_read: unsafe fn(*mut ffi::c_void, data: &mut [u8]) -> lr11xx_hal_status_e,
}

/// Wraps an SPI device for use as the C driver's `context` pointer.
#[repr(C)]
pub struct Context<I> {
    // Must stay the first field: the exported HAL symbols read the vtable
    // through the context pointer without knowing `I`.
    vtable: HalVtable,
    pub inner: I,
}

pub enum Status {
    Ok,
    Error,
}

impl From<lr11xx_status_e> for Status {
    fn from(value: lr11xx_status_e) -> Status {
        match value {
            lr11xx_status_e::LR11XX_STATUS_OK => Status::Ok,
            lr11xx_status_e::LR11XX_STATUS_ERROR => Status::Error,
        }
    }
}

impl<I: SpiDevice> Context<I> {
    pub fn new(inner: I) -> Self {
        Self {
            vtable: HalVtable {
                write: write_shim::<I>,
                read: read_shim::<I>,
                direct_read: direct_read_shim::<I>,
            },
            inner,
        }
    }
}

// &mut self on all of these: the C driver writes to `inner` through the
// context pointer
impl<I> Context<I> {
    fn ptr(&mut self) -> *const ffi::c_void {
        self as *mut Self as *const ffi::c_void
    }

    // --- system ---

    pub fn set_standby(&mut self, standby_cfg: u32) -> Status {
        unsafe { lr11xx_system_set_standby(self.ptr(), standby_cfg) }.into()
    }

    pub fn set_sleep(&mut self, sleep_cfg: lr11xx_system_sleep_cfg_t, sleep_time: u32) -> Status {
        unsafe { lr11xx_system_set_sleep(self.ptr(), sleep_cfg, sleep_time) }.into()
    }

    pub fn set_reg_mode(&mut self, reg_mode: u32) -> Status {
        unsafe { lr11xx_system_set_reg_mode(self.ptr(), reg_mode) }.into()
    }

    pub fn set_tcxo_mode(&mut self, tune: u32, timeout: u32) -> Status {
        unsafe { lr11xx_system_set_tcxo_mode(self.ptr(), tune, timeout) }.into()
    }

    pub fn calibrate(&mut self, calib_param: u8) -> Status {
        unsafe { lr11xx_system_calibrate(self.ptr(), calib_param) }.into()
    }

    pub fn calibrate_image(&mut self, freq1: u8, freq2: u8) -> Status {
        unsafe { lr11xx_system_calibrate_image(self.ptr(), freq1, freq2) }.into()
    }

    pub fn clear_errors(&mut self) -> Status {
        unsafe { lr11xx_system_clear_errors(self.ptr()) }.into()
    }

    pub fn get_errors(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { lr11xx_system_get_errors(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn set_dio_as_rf_switch(&mut self, cfg: &lr11xx_system_rfswitch_cfg_t) -> Status {
        unsafe { lr11xx_system_set_dio_as_rf_switch(self.ptr(), cfg) }.into()
    }

    pub fn set_dio_irq_params(&mut self, irqs_dio1: u32, irqs_dio2: u32) -> Status {
        unsafe { lr11xx_system_set_dio_irq_params(self.ptr(), irqs_dio1, irqs_dio2) }.into()
    }

    pub fn clear_irq_status(&mut self, irqs_to_clear: u32) -> Status {
        unsafe { lr11xx_system_clear_irq_status(self.ptr(), irqs_to_clear) }.into()
    }

    pub fn get_and_clear_irq_status(&mut self) -> (Status, u32) {
        let mut out = 0u32;
        let status = unsafe { lr11xx_system_get_and_clear_irq_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_status(&mut self) -> (Status, lr11xx_system_stat1_t, lr11xx_system_stat2_t, u32) {
        let mut stat1 = lr11xx_system_stat1_t::default();
        let mut stat2 = lr11xx_system_stat2_t::default();
        let mut irq = 0u32;
        let status =
            unsafe { lr11xx_system_get_status(self.ptr(), &mut stat1, &mut stat2, &mut irq) }
                .into();
        (status, stat1, stat2, irq)
    }

    pub fn get_version(&mut self) -> (Status, lr11xx_system_version_t) {
        let mut out = lr11xx_system_version_t::default();
        let status = unsafe { lr11xx_system_get_version(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_temp(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { lr11xx_system_get_temp(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_vbat(&mut self) -> (Status, u8) {
        let mut out = 0u8;
        let status = unsafe { lr11xx_system_get_vbat(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_random_number(&mut self) -> (Status, u32) {
        let mut out = 0u32;
        let status = unsafe { lr11xx_system_get_random_number(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn read_uid(&mut self) -> (Status, [u8; 8]) {
        let mut out = [0u8; 8];
        let status = unsafe { lr11xx_system_read_uid(self.ptr(), out.as_mut_ptr()) }.into();
        (status, out)
    }

    pub fn read_join_eui(&mut self) -> (Status, [u8; 8]) {
        let mut out = [0u8; 8];
        let status = unsafe { lr11xx_system_read_join_eui(self.ptr(), out.as_mut_ptr()) }.into();
        (status, out)
    }

    // --- radio ---

    pub fn set_pkt_type(&mut self, pkt_type: u32) -> Status {
        unsafe { lr11xx_radio_set_pkt_type(self.ptr(), pkt_type) }.into()
    }

    pub fn set_lora_sync_word(&mut self, sync_word: u8) -> Status {
        unsafe { lr11xx_radio_set_lora_sync_word(self.ptr(), sync_word) }.into()
    }

    pub fn set_lora_mod_params(&mut self, params: &lr11xx_radio_mod_params_lora_t) -> Status {
        unsafe { lr11xx_radio_set_lora_mod_params(self.ptr(), params) }.into()
    }

    pub fn set_lora_pkt_params(&mut self, params: &lr11xx_radio_pkt_params_lora_t) -> Status {
        unsafe { lr11xx_radio_set_lora_pkt_params(self.ptr(), params) }.into()
    }

    pub fn set_rf_freq(&mut self, freq_in_hz: u32) -> Status {
        unsafe { lr11xx_radio_set_rf_freq(self.ptr(), freq_in_hz) }.into()
    }

    pub fn set_pa_cfg(&mut self, cfg: &lr11xx_radio_pa_cfg_t) -> Status {
        unsafe { lr11xx_radio_set_pa_cfg(self.ptr(), cfg) }.into()
    }

    pub fn set_tx_params(&mut self, pwr_in_dbm: i8, ramp_time: u32) -> Status {
        unsafe { lr11xx_radio_set_tx_params(self.ptr(), pwr_in_dbm, ramp_time) }.into()
    }

    pub fn set_tx(&mut self, timeout_in_ms: u32) -> Status {
        unsafe { lr11xx_radio_set_tx(self.ptr(), timeout_in_ms) }.into()
    }

    pub fn set_tx_with_timeout_in_rtc_step(&mut self, timeout: u32) -> Status {
        unsafe { lr11xx_radio_set_tx_with_timeout_in_rtc_step(self.ptr(), timeout) }.into()
    }

    pub fn set_rx(&mut self, timeout_in_ms: u32) -> Status {
        unsafe { lr11xx_radio_set_rx(self.ptr(), timeout_in_ms) }.into()
    }

    pub fn set_rx_with_timeout_in_rtc_step(&mut self, timeout: u32) -> Status {
        unsafe { lr11xx_radio_set_rx_with_timeout_in_rtc_step(self.ptr(), timeout) }.into()
    }

    pub fn stop_timeout_on_preamble(&mut self, stop: bool) -> Status {
        unsafe { lr11xx_radio_stop_timeout_on_preamble(self.ptr(), stop) }.into()
    }

    pub fn set_lora_sync_timeout(&mut self, nb_symbol: u16) -> Status {
        unsafe { lr11xx_radio_set_lora_sync_timeout(self.ptr(), nb_symbol) }.into()
    }

    pub fn cfg_rx_boosted(&mut self, enable: bool) -> Status {
        unsafe { lr11xx_radio_cfg_rx_boosted(self.ptr(), enable) }.into()
    }

    pub fn set_cad_params(&mut self, params: &lr11xx_radio_cad_params_t) -> Status {
        unsafe { lr11xx_radio_set_cad_params(self.ptr(), params) }.into()
    }

    pub fn set_cad(&mut self) -> Status {
        unsafe { lr11xx_radio_set_cad(self.ptr()) }.into()
    }

    pub fn get_rx_buffer_status(&mut self) -> (Status, lr11xx_radio_rx_buffer_status_t) {
        let mut out = lr11xx_radio_rx_buffer_status_t::default();
        let status = unsafe { lr11xx_radio_get_rx_buffer_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_lora_pkt_status(&mut self) -> (Status, lr11xx_radio_pkt_status_lora_t) {
        let mut out = lr11xx_radio_pkt_status_lora_t::default();
        let status = unsafe { lr11xx_radio_get_lora_pkt_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_gfsk_pkt_status(&mut self) -> (Status, lr11xx_radio_pkt_status_gfsk_t) {
        let mut out = lr11xx_radio_pkt_status_gfsk_t::default();
        let status = unsafe { lr11xx_radio_get_gfsk_pkt_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_rssi_inst(&mut self) -> (Status, i8) {
        let mut out = 0i8;
        let status = unsafe { lr11xx_radio_get_rssi_inst(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn set_tx_cw(&mut self) -> Status {
        unsafe { lr11xx_radio_set_tx_cw(self.ptr()) }.into()
    }

    pub fn set_rx_duty_cycle(
        &mut self,
        rx_period_in_ms: u32,
        sleep_period_in_ms: u32,
        mode: u32,
    ) -> Status {
        unsafe {
            lr11xx_radio_set_rx_duty_cycle(self.ptr(), rx_period_in_ms, sleep_period_in_ms, mode)
        }
        .into()
    }

    pub fn auto_tx_rx(&mut self, delay: u32, intermediary_mode: u32, timeout: u32) -> Status {
        unsafe { lr11xx_radio_auto_tx_rx(self.ptr(), delay, intermediary_mode, timeout) }.into()
    }

    pub fn set_rx_tx_fallback_mode(&mut self, fallback_mode: u32) -> Status {
        unsafe { lr11xx_radio_set_rx_tx_fallback_mode(self.ptr(), fallback_mode) }.into()
    }

    pub fn get_lora_stats(&mut self) -> (Status, lr11xx_radio_stats_lora_t) {
        let mut out = lr11xx_radio_stats_lora_t::default();
        let status = unsafe { lr11xx_radio_get_lora_stats(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_gfsk_stats(&mut self) -> (Status, lr11xx_radio_stats_gfsk_t) {
        let mut out = lr11xx_radio_stats_gfsk_t::default();
        let status = unsafe { lr11xx_radio_get_gfsk_stats(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn reset_stats(&mut self) -> Status {
        unsafe { lr11xx_radio_reset_stats(self.ptr()) }.into()
    }

    pub fn apply_high_acp_workaround(&mut self) -> Status {
        unsafe { lr11xx_radio_apply_high_acp_workaround(self.ptr()) }.into()
    }

    // --- radio: GFSK ---

    pub fn set_gfsk_mod_params(&mut self, params: &lr11xx_radio_mod_params_gfsk_t) -> Status {
        unsafe { lr11xx_radio_set_gfsk_mod_params(self.ptr(), params) }.into()
    }

    pub fn set_gfsk_pkt_params(&mut self, params: &lr11xx_radio_pkt_params_gfsk_t) -> Status {
        unsafe { lr11xx_radio_set_gfsk_pkt_params(self.ptr(), params) }.into()
    }

    /// `sync_word` is always 8 bytes on the wire
    pub fn set_gfsk_sync_word(&mut self, sync_word: &[u8; 8]) -> Status {
        unsafe { lr11xx_radio_set_gfsk_sync_word(self.ptr(), sync_word.as_ptr()) }.into()
    }

    pub fn set_gfsk_crc_params(&mut self, seed: u32, polynomial: u32) -> Status {
        unsafe { lr11xx_radio_set_gfsk_crc_params(self.ptr(), seed, polynomial) }.into()
    }

    pub fn set_gfsk_whitening_seed(&mut self, seed: u16) -> Status {
        unsafe { lr11xx_radio_set_gfsk_whitening_seed(self.ptr(), seed) }.into()
    }

    pub fn set_pkt_address(&mut self, node_address: u8, broadcast_address: u8) -> Status {
        unsafe { lr11xx_radio_set_pkt_address(self.ptr(), node_address, broadcast_address) }.into()
    }

    // --- regmem ---

    pub fn regmem_write_regmem32(&mut self, address: u32, buffer: &[u32]) -> Status {
        unsafe {
            lr11xx_regmem_write_regmem32(self.ptr(), address, buffer.as_ptr(), buffer.len() as u8)
        }
        .into()
    }

    pub fn regmem_read_regmem32(&mut self, address: u32, buffer: &mut [u32]) -> Status {
        unsafe {
            lr11xx_regmem_read_regmem32(
                self.ptr(),
                address,
                buffer.as_mut_ptr(),
                buffer.len() as u8,
            )
        }
        .into()
    }

    pub fn regmem_write_regmem32_mask(&mut self, address: u32, mask: u32, data: u32) -> Status {
        unsafe { lr11xx_regmem_write_regmem32_mask(self.ptr(), address, mask, data) }.into()
    }

    pub fn regmem_write_mem8(&mut self, address: u32, buffer: &[u8]) -> Status {
        unsafe {
            lr11xx_regmem_write_mem8(self.ptr(), address, buffer.as_ptr(), buffer.len() as u8)
        }
        .into()
    }

    pub fn regmem_read_mem8(&mut self, address: u32, buffer: &mut [u8]) -> Status {
        unsafe {
            lr11xx_regmem_read_mem8(self.ptr(), address, buffer.as_mut_ptr(), buffer.len() as u8)
        }
        .into()
    }

    pub fn regmem_write_buffer8(&mut self, buffer: &[u8]) -> Status {
        unsafe { lr11xx_regmem_write_buffer8(self.ptr(), buffer.as_ptr(), buffer.len() as u8) }
            .into()
    }

    pub fn regmem_read_buffer8(&mut self, offset: u8, buffer: &mut [u8]) -> Status {
        unsafe {
            lr11xx_regmem_read_buffer8(self.ptr(), buffer.as_mut_ptr(), offset, buffer.len() as u8)
        }
        .into()
    }

    pub fn regmem_clear_rxbuffer(&mut self) -> Status {
        unsafe { lr11xx_regmem_clear_rxbuffer(self.ptr()) }.into()
    }

    // --- LR-FHSS ---

    pub fn lr_fhss_init(&mut self) -> Status {
        unsafe { lr11xx_lr_fhss_init(self.ptr()) }.into()
    }

    pub fn set_lr_fhss_sync_word(&mut self, sync_word: &[u8; 4]) -> Status {
        unsafe { lr11xx_radio_set_lr_fhss_sync_word(self.ptr(), sync_word.as_ptr()) }.into()
    }

    pub fn lr_fhss_build_frame(
        &mut self,
        params: &lr11xx_lr_fhss_params_t,
        hop_sequence_id: u16,
        payload: &[u8],
    ) -> Status {
        unsafe {
            lr11xx_lr_fhss_build_frame(
                self.ptr(),
                params,
                hop_sequence_id,
                payload.as_ptr(),
                payload.len() as u8,
            )
        }
        .into()
    }

    // --- GNSS ---

    pub fn gnss_get_result_size(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { lr11xx_gnss_get_result_size(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_read_results(&mut self, result_buffer: &mut [u8]) -> Status {
        unsafe {
            lr11xx_gnss_read_results(
                self.ptr(),
                result_buffer.as_mut_ptr(),
                result_buffer.len() as u16,
            )
        }
        .into()
    }

    /// `blocks` is a concatenation of 20-byte
    /// ([`LR11XX_GNSS_SINGLE_ALMANAC_WRITE_SIZE`]) almanac blocks; the C
    /// driver chunks them into as many WriteAlmanacUpdate commands as fit
    /// the 256-byte command limit.
    pub fn gnss_almanac_update(&mut self, blocks: &[u8]) -> Status {
        let nb_of_blocks = (blocks.len() / LR11XX_GNSS_SINGLE_ALMANAC_WRITE_SIZE as usize) as u8;
        unsafe { lr11xx_gnss_almanac_update(self.ptr(), blocks.as_ptr(), nb_of_blocks) }.into()
    }

    /// Out-param instead of return value: the full almanac is 2820 bytes.
    pub fn gnss_read_almanac(
        &mut self,
        almanac: &mut lr11xx_gnss_almanac_full_read_bytestream_t,
    ) -> Status {
        unsafe { lr11xx_gnss_read_almanac(self.ptr(), almanac.as_mut_ptr()) }.into()
    }

    pub fn gnss_read_freq_search_space(&mut self) -> (Status, lr11xx_gnss_freq_search_space_t) {
        let mut out = lr11xx_gnss_freq_search_space_t::LR11XX_GNSS_FREQUENCY_SEARCH_SPACE_250_HZ;
        let status = unsafe { lr11xx_gnss_read_freq_search_space(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_set_freq_search_space(
        &mut self,
        freq_search_space: lr11xx_gnss_freq_search_space_t,
    ) -> Status {
        unsafe { lr11xx_gnss_set_freq_search_space(self.ptr(), freq_search_space) }.into()
    }

    pub fn gnss_get_almanac_age_for_satellite(&mut self, sv_id: u8) -> (Status, u16) {
        let mut out = 0u16;
        let status =
            unsafe { lr11xx_gnss_get_almanac_age_for_satellite(self.ptr(), sv_id, &mut out) }
                .into();
        (status, out)
    }

    pub fn gnss_push_solver_msg(&mut self, payload: &[u8]) -> Status {
        unsafe { lr11xx_gnss_push_solver_msg(self.ptr(), payload.as_ptr(), payload.len() as u16) }
            .into()
    }

    pub fn gnss_get_nb_visible_satellites(
        &mut self,
        date: u32,
        assistance_position: &lr11xx_gnss_solver_assistance_position_t,
        constellation: u32,
    ) -> (Status, u8) {
        let mut out = 0u8;
        let status = unsafe {
            lr11xx_gnss_get_nb_visible_satellites(
                self.ptr(),
                date,
                assistance_position,
                constellation,
                &mut out,
            )
        }
        .into();
        (status, out)
    }

    /// One entry is read per element of `visible_satellites`.
    pub fn gnss_get_visible_satellites(
        &mut self,
        visible_satellites: &mut [lr11xx_gnss_visible_satellite_t],
    ) -> Status {
        unsafe {
            lr11xx_gnss_get_visible_satellites(
                self.ptr(),
                visible_satellites.len() as u8,
                visible_satellites.as_mut_ptr(),
            )
        }
        .into()
    }

    pub fn gnss_set_constellations_to_use(&mut self, constellation_mask: u8) -> Status {
        unsafe { lr11xx_gnss_set_constellations_to_use(self.ptr(), constellation_mask) }.into()
    }

    pub fn gnss_read_used_constellations(&mut self) -> (Status, u8) {
        let mut out = 0u8;
        let status = unsafe { lr11xx_gnss_read_used_constellations(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_set_almanac_update(&mut self, constellations_to_update: u8) -> Status {
        unsafe { lr11xx_gnss_set_almanac_update(self.ptr(), constellations_to_update) }.into()
    }

    pub fn gnss_read_almanac_update(&mut self) -> (Status, u8) {
        let mut out = 0u8;
        let status = unsafe { lr11xx_gnss_read_almanac_update(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_read_firmware_version(&mut self) -> (Status, lr11xx_gnss_version_t) {
        let mut out = lr11xx_gnss_version_t::default();
        let status = unsafe { lr11xx_gnss_read_firmware_version(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_read_supported_constellations(&mut self) -> (Status, u8) {
        let mut out = 0u8;
        let status =
            unsafe { lr11xx_gnss_read_supported_constellations(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_set_scan_mode(&mut self, scan_mode: u32) -> Status {
        unsafe { lr11xx_gnss_set_scan_mode(self.ptr(), scan_mode) }.into()
    }

    pub fn gnss_scan(&mut self, effort_mode: u32, gnss_input_parameters: u8, nb_sat: u8) -> Status {
        unsafe { lr11xx_gnss_scan(self.ptr(), effort_mode, gnss_input_parameters, nb_sat) }.into()
    }

    pub fn gnss_set_assistance_position(
        &mut self,
        assistance_position: &lr11xx_gnss_solver_assistance_position_t,
    ) -> Status {
        unsafe { lr11xx_gnss_set_assistance_position(self.ptr(), assistance_position) }.into()
    }

    pub fn gnss_read_assistance_position(
        &mut self,
    ) -> (Status, lr11xx_gnss_solver_assistance_position_t) {
        let mut out = lr11xx_gnss_solver_assistance_position_t::default();
        let status = unsafe { lr11xx_gnss_read_assistance_position(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_push_dmc_msg(&mut self, dmc_msg: &[u8]) -> Status {
        // non-const in the C signature, but the driver only forwards it to
        // lr11xx_hal_write
        unsafe {
            lr11xx_gnss_push_dmc_msg(
                self.ptr(),
                dmc_msg.as_ptr().cast_mut(),
                dmc_msg.len() as u16,
            )
        }
        .into()
    }

    pub fn gnss_get_context_status(&mut self) -> (Status, lr11xx_gnss_context_status_bytestream_t) {
        let mut out = lr11xx_gnss_context_status_bytestream_t::default();
        let status = unsafe { lr11xx_gnss_get_context_status(self.ptr(), out.as_mut_ptr()) }.into();
        (status, out)
    }

    pub fn gnss_get_nb_detected_satellites(&mut self) -> (Status, u8) {
        let mut out = 0u8;
        let status = unsafe { lr11xx_gnss_get_nb_detected_satellites(self.ptr(), &mut out) }.into();
        (status, out)
    }

    /// One entry is read per element of `detected_satellites`.
    pub fn gnss_get_detected_satellites(
        &mut self,
        detected_satellites: &mut [lr11xx_gnss_detected_satellite_t],
    ) -> Status {
        unsafe {
            lr11xx_gnss_get_detected_satellites(
                self.ptr(),
                detected_satellites.len() as u8,
                detected_satellites.as_mut_ptr(),
            )
        }
        .into()
    }

    /// `almanacs` must hold 22 bytes
    /// ([`LR11XX_GNSS_SINGLE_ALMANAC_READ_SIZE`]) per satellite; the number
    /// of satellites to read is derived from its length.
    pub fn gnss_read_almanac_per_satellites(
        &mut self,
        sv_id_init: u8,
        almanacs: &mut [u8],
    ) -> Status {
        let n_sv = (almanacs.len() / LR11XX_GNSS_SINGLE_ALMANAC_READ_SIZE as usize) as u8;
        unsafe {
            lr11xx_gnss_read_almanac_per_satellites(
                self.ptr(),
                sv_id_init,
                n_sv,
                almanacs.as_mut_ptr(),
            )
        }
        .into()
    }

    pub fn gnss_read_gnss_rssi_test(&mut self) -> (Status, i8) {
        let mut out = 0i8;
        let status = unsafe { lr11xx_gnss_read_gnss_rssi_test(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_fetch_time(
        &mut self,
        effort_mode: u32,
        option: lr11xx_gnss_fetch_time_option_t,
    ) -> Status {
        unsafe { lr11xx_gnss_fetch_time(self.ptr(), effort_mode, option) }.into()
    }

    pub fn gnss_read_time(&mut self) -> (Status, lr11xx_gnss_time_t) {
        let mut out = lr11xx_gnss_time_t::default();
        let status = unsafe { lr11xx_gnss_read_time(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_reset_time(&mut self) -> Status {
        unsafe { lr11xx_gnss_reset_time(self.ptr()) }.into()
    }

    pub fn gnss_reset_position(&mut self) -> Status {
        unsafe { lr11xx_gnss_reset_position(self.ptr()) }.into()
    }

    pub fn gnss_read_week_number_rollover(
        &mut self,
    ) -> (Status, lr11xx_gnss_week_number_rollover_status_t, u8) {
        let mut wn_status =
            lr11xx_gnss_week_number_rollover_status_t::LR11XX_GNSS_WN_ROLLOVER_ROLLOVER_NEVER_SET;
        let mut wn_rollover = 0u8;
        let status = unsafe {
            lr11xx_gnss_read_week_number_rollover(self.ptr(), &mut wn_status, &mut wn_rollover)
        }
        .into();
        (status, wn_status, wn_rollover)
    }

    pub fn gnss_read_demod_status(
        &mut self,
    ) -> (Status, lr11xx_gnss_demod_status_t, lr11xx_gnss_demod_info_t) {
        let mut demod_status = lr11xx_gnss_demod_status_t::LR11XX_GNSS_WORD_SYNC_SEARCH_NOT_STARTED;
        let mut demod_info = lr11xx_gnss_demod_info_t::default();
        let status = unsafe {
            lr11xx_gnss_read_demod_status(self.ptr(), &mut demod_status, &mut demod_info)
        }
        .into();
        (status, demod_status, demod_info)
    }

    pub fn gnss_read_cumulative_timing(&mut self) -> (Status, lr11xx_gnss_cumulative_timing_t) {
        let mut out = lr11xx_gnss_cumulative_timing_t::default();
        let status = unsafe { lr11xx_gnss_read_cumulative_timing(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_set_time(&mut self, time: u32, time_accuracy: u16) -> Status {
        unsafe { lr11xx_gnss_set_time(self.ptr(), time, time_accuracy) }.into()
    }

    pub fn gnss_config_delay_reset_assistance_position(&mut self, delay: u32) -> Status {
        unsafe { lr11xx_gnss_config_delay_reset_assistance_position(self.ptr(), delay) }.into()
    }

    pub fn gnss_read_doppler_solver_result(
        &mut self,
    ) -> (Status, lr11xx_gnss_doppler_solver_result_t) {
        let mut out = lr11xx_gnss_doppler_solver_result_t::default();
        let status = unsafe { lr11xx_gnss_read_doppler_solver_result(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_read_delay_reset_assistance_position(&mut self) -> (Status, u32) {
        let mut out = 0u32;
        let status =
            unsafe { lr11xx_gnss_read_delay_reset_assistance_position(self.ptr(), &mut out) }
                .into();
        (status, out)
    }

    pub fn gnss_almanac_update_from_sat(
        &mut self,
        constellation_mask: u8,
        effort_mode: u32,
    ) -> Status {
        unsafe { lr11xx_gnss_almanac_update_from_sat(self.ptr(), constellation_mask, effort_mode) }
            .into()
    }

    /// Returns `(status, nb_visible_sat, time_elapsed)`.
    pub fn gnss_read_keep_sync_status(&mut self, constellation_mask: u8) -> (Status, u8, u32) {
        let mut nb_visible_sat = 0u8;
        let mut time_elapsed = 0u32;
        let status = unsafe {
            lr11xx_gnss_read_keep_sync_status(
                self.ptr(),
                constellation_mask,
                &mut nb_visible_sat,
                &mut time_elapsed,
            )
        }
        .into();
        (status, nb_visible_sat, time_elapsed)
    }

    pub fn gnss_read_almanac_status(&mut self) -> (Status, lr11xx_gnss_read_almanac_status_t) {
        let mut out = lr11xx_gnss_read_almanac_status_t::default();
        let status = unsafe { lr11xx_gnss_read_almanac_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn gnss_config_almanac_update_period(
        &mut self,
        constellation_mask: u8,
        sv_type: lr11xx_gnss_sv_type_t,
        period: u16,
    ) -> Status {
        unsafe {
            lr11xx_gnss_config_almanac_update_period(
                self.ptr(),
                constellation_mask,
                sv_type,
                period,
            )
        }
        .into()
    }

    pub fn gnss_read_almanac_update_period(
        &mut self,
        constellation_mask: u8,
        sv_type: lr11xx_gnss_sv_type_t,
    ) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe {
            lr11xx_gnss_read_almanac_update_period(
                self.ptr(),
                constellation_mask,
                sv_type,
                &mut out,
            )
        }
        .into();
        (status, out)
    }

    /// One byte is read per element of `sv_sync_list`.
    pub fn gnss_get_sv_sync(&mut self, constellation_mask: u8, sv_sync_list: &mut [u8]) -> Status {
        unsafe {
            lr11xx_gnss_get_sv_sync(
                self.ptr(),
                constellation_mask,
                sv_sync_list.len() as u8,
                sv_sync_list.as_mut_ptr(),
            )
        }
        .into()
    }

    pub fn gnss_set_gps_bit_mask_sat_activated(&mut self, gps_sat_activated_1_32: u32) -> Status {
        unsafe { lr11xx_gnss_set_gps_bit_mask_sat_activated(self.ptr(), gps_sat_activated_1_32) }
            .into()
    }

    pub fn gnss_set_beidou_bit_mask_sat_activated(
        &mut self,
        beidou_sat_activated_1_32: u32,
        beidou_sat_activated_33_63: u32,
    ) -> Status {
        unsafe {
            lr11xx_gnss_set_beidou_bit_mask_sat_activated(
                self.ptr(),
                beidou_sat_activated_1_32,
                beidou_sat_activated_33_63,
            )
        }
        .into()
    }

    pub fn gnss_read_last_scan_mode_launched(&mut self) -> (Status, u32) {
        let mut out = 0u32;
        let status =
            unsafe { lr11xx_gnss_read_last_scan_mode_launched(self.ptr(), &mut out) }.into();
        (status, out)
    }
}

// GNSS helpers that parse or compute host-side; nothing crosses the SPI bus,
// so they are free functions rather than `Context` methods.

pub fn gnss_parse_context_status_buffer(
    context_status_bytestream: &lr11xx_gnss_context_status_bytestream_t,
) -> (Status, lr11xx_gnss_context_status_t) {
    let mut out = lr11xx_gnss_context_status_t::default();
    let status = unsafe {
        lr11xx_gnss_parse_context_status_buffer(context_status_bytestream.as_ptr(), &mut out)
    }
    .into();
    (status, out)
}

/// Returns the destination (host, solver or DMC) encoded in the first byte
/// of a scan result buffer.
pub fn gnss_get_result_destination(result_buffer: &[u8]) -> (Status, u32) {
    let mut out = 0u32;
    let status = unsafe {
        lr11xx_gnss_get_result_destination(
            result_buffer.as_ptr(),
            result_buffer.len() as u16,
            &mut out,
        )
    }
    .into();
    (status, out)
}

pub fn gnss_compute_almanac_age(
    almanac_date: u16,
    nb_days_between_epoch_and_corresponding_gps_time_rollover: u16,
    nb_days_since_epoch: u16,
) -> u16 {
    unsafe {
        lr11xx_gnss_compute_almanac_age(
            almanac_date,
            nb_days_between_epoch_and_corresponding_gps_time_rollover,
            nb_days_since_epoch,
        )
    }
}

/// Returns `(power_consumption_nah, power_consumption_nwh)`.
pub fn gnss_compute_power_consumption(
    cumulative_timing: &lr11xx_gnss_cumulative_timing_t,
    instantaneous_power_consumption_ua: &lr11xx_gnss_instantaneous_power_consumption_ua_t,
) -> (u32, u32) {
    let mut nah = 0u32;
    let mut nwh = 0u32;
    unsafe {
        lr11xx_gnss_compute_power_consumption(
            cumulative_timing,
            instantaneous_power_consumption_ua,
            &mut nah,
            &mut nwh,
        )
    };
    (nah, nwh)
}

/// # Safety
/// `ctx` must point at the `Context<I>` this shim was monomorphized for, and
/// the C driver must not call back into the same context reentrantly.
unsafe fn write_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    command: &[u8],
    data: &[u8],
) -> lr11xx_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    // Command and data belong to a single chip-select assertion
    let mut ops = [Operation::Write(command), Operation::Write(data)];
    match context.inner.transaction(&mut ops) {
        Ok(()) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK,
        Err(_) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_ERROR,
    }
}

/// Two chip-select assertions per the lr11xx HAL spec: the command is
/// written and NSS released, then the response is clocked out in a second
/// assertion as one discarded status byte followed by `data`.
///
/// # Safety
/// Same contract as [`write_shim`].
unsafe fn read_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    command: &[u8],
    data: &mut [u8],
) -> lr11xx_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    if context.inner.write(command).is_err() {
        return lr11xx_hal_status_e::LR11XX_HAL_STATUS_ERROR;
    }
    let mut dummy = [0u8; 1];
    let mut ops = [Operation::Read(&mut dummy), Operation::Read(data)];
    match context.inner.transaction(&mut ops) {
        Ok(()) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK,
        Err(_) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_ERROR,
    }
}

/// # Safety
/// Same contract as [`write_shim`].
unsafe fn direct_read_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    data: &mut [u8],
) -> lr11xx_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    match context.inner.read(data) {
        Ok(()) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK,
        Err(_) => lr11xx_hal_status_e::LR11XX_HAL_STATUS_ERROR,
    }
}

unsafe fn slice_or_empty<'a>(ptr: *const u8, len: u16) -> &'a [u8] {
    if len > 0 && !ptr.is_null() {
        unsafe { core::slice::from_raw_parts(ptr, len as usize) }
    } else {
        &[]
    }
}

unsafe fn slice_or_empty_mut<'a>(ptr: *mut u8, len: u16) -> &'a mut [u8] {
    if len > 0 && !ptr.is_null() {
        unsafe { core::slice::from_raw_parts_mut(ptr, len as usize) }
    } else {
        &mut []
    }
}

/// HAL symbol the C driver links against, owned by this crate. Dispatches
/// through the vtable at the head of the context.
///
/// # Safety
/// Only the C driver should call this, with a `context` originating from
/// `Context::new` and command/data pointers valid for their given lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_write(
    context: *const ffi::c_void,
    command: *const u8,
    command_length: u16,
    data: *const u8,
    data_length: u16,
) -> lr11xx_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let command = unsafe { slice_or_empty(command, command_length) };
    let data = unsafe { slice_or_empty(data, data_length) };
    unsafe { (vtable.write)(context as *mut ffi::c_void, command, data) }
}

/// # Safety
/// Same contract as [`lr11xx_hal_write`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_read(
    context: *const ffi::c_void,
    command: *const u8,
    command_length: u16,
    data: *mut u8,
    data_length: u16,
) -> lr11xx_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let command = unsafe { slice_or_empty(command, command_length) };
    let data = unsafe { slice_or_empty_mut(data, data_length) };
    unsafe { (vtable.read)(context as *mut ffi::c_void, command, data) }
}

/// # Safety
/// Same contract as [`lr11xx_hal_write`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_direct_read(
    context: *const ffi::c_void,
    data: *mut u8,
    data_length: u16,
) -> lr11xx_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let data = unsafe { slice_or_empty_mut(data, data_length) };
    unsafe { (vtable.direct_read)(context as *mut ffi::c_void, data) }
}

/// GPIO-level operation in a real BSP (reset pin toggle); nothing crosses the
/// SPI bus, so there is no byte stream to model. No-op success.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_reset(_context: *const ffi::c_void) -> lr11xx_hal_status_e {
    lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK
}

/// NSS-toggle wakeup in a real BSP; no bytes are clocked. No-op success.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_wakeup(_context: *const ffi::c_void) -> lr11xx_hal_status_e {
    lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK
}

/// BUSY-pin abort sequence in a real BSP; no bytes are clocked. No-op success.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lr11xx_hal_abort_blocking_cmd(
    _context: *const ffi::c_void,
) -> lr11xx_hal_status_e {
    lr11xx_hal_status_e::LR11XX_HAL_STATUS_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::spi::ErrorKind;
    use std::result::Result;

    #[derive(Debug)]
    enum SpiError {}
    impl embedded_hal::spi::Error for SpiError {
        fn kind(&self) -> ErrorKind {
            todo!()
        }
    }

    struct TestSpi {
        data: Vec<u8>,
    }
    impl embedded_hal::spi::ErrorType for TestSpi {
        type Error = SpiError;
    }
    impl SpiDevice for TestSpi {
        fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
            for op in operations {
                match op {
                    Operation::Write(buf) => self.data.extend_from_slice(buf),
                    // reads clock NOPs onto the wire
                    Operation::Read(buf) => self.data.extend_from_slice(&vec![0u8; buf.len()]),
                    _ => todo!(),
                }
            }
            Ok(())
        }
    }

    #[test]
    fn standby_and_read_round_trip() {
        let mut context = Context::new(TestSpi { data: vec![] });
        // SetStandby opcode 0x011C, RC = 0x00
        context.set_standby(0);
        assert_eq!(context.inner.data, vec![0x01, 0x1C, 0x00]);

        // GetVersion opcode 0x0101: command write, then dummy + 4 response
        // bytes clocked as NOPs
        context.inner.data.clear();
        let (_, _version) = context.get_version();
        assert_eq!(context.inner.data, vec![0x01, 0x01, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn gnss_round_trip() {
        let mut context = Context::new(TestSpi { data: vec![] });
        // GnssSetConstellationToUse opcode 0x0400 + mask byte
        context.gnss_set_constellations_to_use(0x03);
        assert_eq!(context.inner.data, vec![0x04, 0x00, 0x03]);

        // GnssReadVersion opcode 0x0406: command write, then dummy + 2
        // response bytes clocked as NOPs
        context.inner.data.clear();
        let (_, _version) = context.gnss_read_firmware_version();
        assert_eq!(context.inner.data, vec![0x04, 0x06, 0, 0, 0]);

        // GnssScan opcode 0x040B + effort mode, result mask, nb sat
        context.inner.data.clear();
        context.gnss_scan(
            smtc_modem_cores_sys::lr11xx_gnss_search_mode_t_LR11XX_GNSS_OPTION_MID_EFFORT,
            0x3C,
            16,
        );
        assert_eq!(context.inner.data, vec![0x04, 0x0B, 0x01, 0x3C, 16]);
    }
}
