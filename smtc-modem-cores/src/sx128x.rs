use core::ffi;
pub use smtc_modem_cores_sys::{
    sx128x_chip_modes_e, sx128x_chip_status_t, sx128x_cmd_status_e, sx128x_hal_status_e,
    sx128x_lna_settings_e, sx128x_lora_cad_params_t, sx128x_lora_cad_symbs_e,
    sx128x_lora_ranging_bw_e, sx128x_lora_ranging_cr_e, sx128x_lora_ranging_pkt_len_modes_e,
    sx128x_lora_ranging_preamble_len_t, sx128x_lora_ranging_sf_e, sx128x_mod_params_lora_t,
    sx128x_pkt_params_lora_t, sx128x_pkt_status_lora_t, sx128x_pkt_type_e, sx128x_ramp_time_e,
    sx128x_reg_mod_e, sx128x_rx_buffer_status_t, sx128x_standby_cfg_e, sx128x_status_e,
    sx128x_tick_size_e,
};
use smtc_modem_cores_sys::{
    sx128x_clear_irq_status, sx128x_get_and_clear_irq_status, sx128x_get_irq_status,
    sx128x_get_lora_pkt_status, sx128x_get_pkt_type, sx128x_get_rssi_inst,
    sx128x_get_rx_buffer_status, sx128x_get_status, sx128x_read_buffer, sx128x_read_register,
    sx128x_reset, sx128x_save_context, sx128x_set_auto_fs, sx128x_set_buffer_base_address,
    sx128x_set_cad, sx128x_set_cad_params, sx128x_set_dio_irq_params, sx128x_set_fs,
    sx128x_set_lna_settings, sx128x_set_long_preamble, sx128x_set_lora_mod_params,
    sx128x_set_lora_pkt_params, sx128x_set_lora_sync_word, sx128x_set_pkt_type,
    sx128x_set_reg_mode, sx128x_set_rf_freq, sx128x_set_rx, sx128x_set_rx_duty_cycle,
    sx128x_set_sleep, sx128x_set_standby, sx128x_set_tx, sx128x_set_tx_cw, sx128x_set_tx_params,
    sx128x_wakeup, sx128x_write_buffer, sx128x_write_register,
};

use embedded_hal::spi::{Operation, SpiDevice};

/// Monomorphized HAL entry points for one concrete SPI type. C hands the
/// context pointer back on every HAL call, so dispatch goes through this
/// table rather than through a user-provided `#[no_mangle]` symbol.
#[repr(C)]
struct HalVtable {
    write: unsafe fn(*mut ffi::c_void, command: &[u8], data: &[u8]) -> sx128x_hal_status_e,
    read: unsafe fn(*mut ffi::c_void, command: &[u8], data: &mut [u8]) -> sx128x_hal_status_e,
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
    UnSupportedFeature,
    UnknownValue,
    Error,
}

impl From<sx128x_status_e> for Status {
    fn from(value: sx128x_status_e) -> Status {
        match value {
            sx128x_status_e::SX128X_STATUS_OK => Status::Ok,
            sx128x_status_e::SX128X_STATUS_UNSUPPORTED_FEATURE => Status::UnSupportedFeature,
            sx128x_status_e::SX128X_STATUS_UNKNOWN_VALUE => Status::UnknownValue,
            sx128x_status_e::SX128X_STATUS_ERROR => Status::Error,
        }
    }
}

impl From<Status> for sx128x_status_e {
    fn from(status: Status) -> sx128x_status_e {
        match status {
            Status::Ok => sx128x_status_e::SX128X_STATUS_OK,
            Status::UnSupportedFeature => sx128x_status_e::SX128X_STATUS_UNSUPPORTED_FEATURE,
            Status::UnknownValue => sx128x_status_e::SX128X_STATUS_UNKNOWN_VALUE,
            Status::Error => sx128x_status_e::SX128X_STATUS_ERROR,
        }
    }
}

impl<I: SpiDevice> Context<I> {
    pub fn new(inner: I) -> Self {
        Self {
            vtable: HalVtable {
                write: write_shim::<I>,
                read: read_shim::<I>,
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

    pub fn set_sleep(
        &mut self,
        retain_data_buffer: bool,
        retain_radio_configuration: bool,
    ) -> Status {
        unsafe { sx128x_set_sleep(self.ptr(), retain_data_buffer, retain_radio_configuration) }
            .into()
    }

    pub fn set_standby(&mut self, cfg: sx128x_standby_cfg_e) -> Status {
        unsafe { sx128x_set_standby(self.ptr(), cfg) }.into()
    }

    pub fn wakeup(&mut self) -> Status {
        unsafe { sx128x_wakeup(self.ptr()) }.into()
    }

    pub fn reset(&mut self) -> Status {
        unsafe { sx128x_reset(self.ptr()) }.into()
    }

    pub fn set_fs(&mut self) -> Status {
        unsafe { sx128x_set_fs(self.ptr()) }.into()
    }

    pub fn set_auto_fs(&mut self, enable: bool) -> Status {
        unsafe { sx128x_set_auto_fs(self.ptr(), enable) }.into()
    }

    pub fn set_rf_freq(&mut self, freq_in_hz: u32) -> Status {
        unsafe { sx128x_set_rf_freq(self.ptr(), freq_in_hz) }.into()
    }

    pub fn set_pkt_type(&mut self, pkt_type: sx128x_pkt_type_e) -> Status {
        unsafe { sx128x_set_pkt_type(self.ptr(), pkt_type) }.into()
    }

    pub fn get_pkt_type(&mut self) -> (Status, sx128x_pkt_type_e) {
        let mut out = sx128x_pkt_type_e::SX128X_PKT_TYPE_GFSK;
        let status = unsafe { sx128x_get_pkt_type(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn set_lora_mod_params(&mut self, params: &sx128x_mod_params_lora_t) -> Status {
        unsafe { sx128x_set_lora_mod_params(self.ptr(), params) }.into()
    }

    pub fn set_lora_pkt_params(&mut self, params: &sx128x_pkt_params_lora_t) -> Status {
        unsafe { sx128x_set_lora_pkt_params(self.ptr(), params) }.into()
    }

    pub fn set_lora_sync_word(&mut self, sync_word: u8) -> Status {
        unsafe { sx128x_set_lora_sync_word(self.ptr(), sync_word) }.into()
    }

    pub fn set_buffer_base_address(&mut self, tx_base: u8, rx_base: u8) -> Status {
        unsafe { sx128x_set_buffer_base_address(self.ptr(), tx_base, rx_base) }.into()
    }

    pub fn write_buffer(&mut self, offset: u8, buffer: &[u8]) -> Status {
        unsafe { sx128x_write_buffer(self.ptr(), offset, buffer.as_ptr(), buffer.len() as u16) }
            .into()
    }

    pub fn read_buffer(&mut self, offset: u8, buffer: &mut [u8]) -> Status {
        unsafe { sx128x_read_buffer(self.ptr(), offset, buffer.as_mut_ptr(), buffer.len() as u16) }
            .into()
    }

    pub fn write_register(&mut self, address: u16, buffer: &[u8]) -> Status {
        unsafe { sx128x_write_register(self.ptr(), address, buffer.as_ptr(), buffer.len() as u16) }
            .into()
    }

    pub fn read_register(&mut self, address: u16, buffer: &mut [u8]) -> Status {
        unsafe {
            sx128x_read_register(
                self.ptr(),
                address,
                buffer.as_mut_ptr(),
                buffer.len() as u16,
            )
        }
        .into()
    }

    pub fn set_tx(&mut self, period_base: sx128x_tick_size_e, period_base_count: u16) -> Status {
        unsafe { sx128x_set_tx(self.ptr(), period_base, period_base_count) }.into()
    }

    pub fn set_rx(&mut self, period_base: sx128x_tick_size_e, period_base_count: u16) -> Status {
        unsafe { sx128x_set_rx(self.ptr(), period_base, period_base_count) }.into()
    }

    pub fn set_rx_duty_cycle(
        &mut self,
        period_base: sx128x_tick_size_e,
        rx_period_base_count: u16,
        sleep_period_base_count: u16,
    ) -> Status {
        unsafe {
            sx128x_set_rx_duty_cycle(
                self.ptr(),
                period_base,
                rx_period_base_count,
                sleep_period_base_count,
            )
        }
        .into()
    }

    pub fn set_tx_cw(&mut self) -> Status {
        unsafe { sx128x_set_tx_cw(self.ptr()) }.into()
    }

    pub fn set_tx_params(&mut self, pwr_in_dbm: i8, ramp_time: sx128x_ramp_time_e) -> Status {
        unsafe { sx128x_set_tx_params(self.ptr(), pwr_in_dbm, ramp_time) }.into()
    }

    pub fn set_dio_irq_params(
        &mut self,
        irq_mask: u16,
        dio1_mask: u16,
        dio2_mask: u16,
        dio3_mask: u16,
    ) -> Status {
        unsafe { sx128x_set_dio_irq_params(self.ptr(), irq_mask, dio1_mask, dio2_mask, dio3_mask) }
            .into()
    }

    pub fn get_irq_status(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { sx128x_get_irq_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn clear_irq_status(&mut self, irq_mask: u16) -> Status {
        unsafe { sx128x_clear_irq_status(self.ptr(), irq_mask) }.into()
    }

    pub fn get_and_clear_irq_status(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { sx128x_get_and_clear_irq_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn set_cad_params(&mut self, params: &sx128x_lora_cad_params_t) -> Status {
        unsafe { sx128x_set_cad_params(self.ptr(), params) }.into()
    }

    pub fn set_cad(&mut self) -> Status {
        unsafe { sx128x_set_cad(self.ptr()) }.into()
    }

    pub fn get_status(&mut self) -> (Status, sx128x_chip_status_t) {
        let mut out = sx128x_chip_status_t {
            cmd_status: sx128x_cmd_status_e::SX128X_CMD_STATUS_CMD_OK,
            chip_mode: sx128x_chip_modes_e::SX128X_CHIP_MODE_STBY_RC,
        };
        let status = unsafe { sx128x_get_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_rx_buffer_status(&mut self) -> (Status, sx128x_rx_buffer_status_t) {
        let mut out = sx128x_rx_buffer_status_t {
            pld_len_in_bytes: 0,
            buffer_start_pointer: 0,
        };
        let status = unsafe { sx128x_get_rx_buffer_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_lora_pkt_status(&mut self) -> (Status, sx128x_pkt_status_lora_t) {
        let mut out = sx128x_pkt_status_lora_t { rssi: 0, snr: 0 };
        let status = unsafe { sx128x_get_lora_pkt_status(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn get_rssi_inst(&mut self) -> (Status, i16) {
        let mut out = 0i16;
        let status = unsafe { sx128x_get_rssi_inst(self.ptr(), &mut out) }.into();
        (status, out)
    }

    pub fn set_long_preamble(&mut self, enable: bool) -> Status {
        unsafe { sx128x_set_long_preamble(self.ptr(), enable) }.into()
    }

    pub fn set_reg_mode(&mut self, mode: sx128x_reg_mod_e) -> Status {
        unsafe { sx128x_set_reg_mode(self.ptr(), mode) }.into()
    }

    pub fn set_lna_settings(&mut self, settings: sx128x_lna_settings_e) -> Status {
        unsafe { sx128x_set_lna_settings(self.ptr(), settings) }.into()
    }

    pub fn save_context(&mut self) -> Status {
        unsafe { sx128x_save_context(self.ptr()) }.into()
    }
}

/// # Safety
/// `ctx` must point at the `Context<I>` this shim was monomorphized for, and
/// the C driver must not call back into the same context reentrantly.
unsafe fn write_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    command: &[u8],
    data: &[u8],
) -> sx128x_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    // Command and data belong to a single chip-select assertion
    let mut ops = [Operation::Write(command), Operation::Write(data)];
    match context.inner.transaction(&mut ops) {
        Ok(()) => sx128x_hal_status_e::SX128X_HAL_STATUS_OK,
        Err(_) => sx128x_hal_status_e::SX128X_HAL_STATUS_ERROR,
    }
}

/// # Safety
/// Same contract as [`write_shim`].
unsafe fn read_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    command: &[u8],
    data: &mut [u8],
) -> sx128x_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    let mut ops = [Operation::Write(command), Operation::Read(data)];
    match context.inner.transaction(&mut ops) {
        Ok(()) => sx128x_hal_status_e::SX128X_HAL_STATUS_OK,
        Err(_) => sx128x_hal_status_e::SX128X_HAL_STATUS_ERROR,
    }
}

unsafe fn slice_or_empty<'a>(ptr: *const u8, len: u16) -> &'a [u8] {
    if len > 0 && !ptr.is_null() {
        unsafe { core::slice::from_raw_parts(ptr, len as usize) }
    } else {
        &[]
    }
}

/// The single HAL symbol the C driver links against, owned by this crate.
/// Dispatches through the vtable at the head of the context.
///
/// # Safety
/// Only the C driver should call this, with a `context` originating from
/// `Context::new` and command/data pointers valid for their given lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx128x_hal_write(
    context: *const ffi::c_void,
    command: *const u8,
    command_length: u16,
    data: *const u8,
    data_length: u16,
) -> sx128x_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let command = unsafe { slice_or_empty(command, command_length) };
    let data = unsafe { slice_or_empty(data, data_length) };
    unsafe { (vtable.write)(context as *mut ffi::c_void, command, data) }
}

/// # Safety
/// Same contract as [`sx128x_hal_write`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx128x_hal_read(
    context: *const ffi::c_void,
    command: *const u8,
    command_length: u16,
    data: *mut u8,
    data_length: u16,
) -> sx128x_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let command = unsafe { slice_or_empty(command, command_length) };
    let data = if data_length > 0 && !data.is_null() {
        unsafe { core::slice::from_raw_parts_mut(data, data_length as usize) }
    } else {
        &mut []
    };
    unsafe { (vtable.read)(context as *mut ffi::c_void, command, data) }
}

/// GPIO-level operation in a real BSP (reset pin toggle); nothing crosses the
/// SPI bus, so there is no byte stream to model. No-op success.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx128x_hal_reset(_context: *const ffi::c_void) -> sx128x_hal_status_e {
    sx128x_hal_status_e::SX128X_HAL_STATUS_OK
}

/// NSS-toggle wakeup in a real BSP; no bytes are clocked. No-op success.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx128x_hal_wakeup(_context: *const ffi::c_void) -> sx128x_hal_status_e {
    sx128x_hal_status_e::SX128X_HAL_STATUS_OK
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

    macro_rules! recording_spi {
        ($name:ident) => {
            struct $name {
                data: Vec<u8>,
            }
            impl embedded_hal::spi::ErrorType for $name {
                type Error = SpiError;
            }
            impl SpiDevice for $name {
                fn transaction(
                    &mut self,
                    operations: &mut [Operation<'_, u8>],
                ) -> Result<(), Self::Error> {
                    for op in operations {
                        match op {
                            Operation::Write(buf) => self.data.extend_from_slice(buf),
                            _ => todo!(),
                        }
                    }
                    Ok(())
                }
            }
        };
    }

    // Two distinct SPI types in one binary: impossible with the old
    // user-defined #[no_mangle] design (duplicate symbol)
    recording_spi!(TestSpiA);
    recording_spi!(TestSpiB);

    #[test]
    fn sleep_retention_flags() {
        let mut context = Context::new(TestSpiA { data: vec![] });
        context.set_sleep(false, false);

        let mut other = Context::new(TestSpiB { data: vec![] });
        other.set_sleep(true, true);

        // SetSleep opcode 0x84; cfg byte is data-buffer retention (bit 1)
        // plus radio-configuration retention (bit 0)
        assert_eq!(context.inner.data, vec![0x84, 0x00]);
        assert_eq!(other.inner.data, vec![0x84, 0x03]);
    }

    #[test]
    fn standby_and_rf_freq() {
        let mut context = Context::new(TestSpiA { data: vec![] });
        context.set_standby(sx128x_standby_cfg_e::SX128X_STANDBY_CFG_XOSC);
        // SetStandby opcode 0x80
        assert_eq!(context.inner.data, vec![0x80, 0x01]);

        context.inner.data.clear();
        context.set_rf_freq(2_425_000_000);
        // SetRfFrequency opcode 0x86, freq in PLL steps of Fxtal(52 MHz)/2^18
        // ~= 198.364 Hz: 2425 MHz -> round(2425e6/198.364) = 0xBA89D9
        assert_eq!(context.inner.data, vec![0x86, 0xBA, 0x89, 0xD9]);
    }
}
