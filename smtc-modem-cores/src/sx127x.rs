use smtc_modem_cores_sys::{
    sx127x_cal_img, sx127x_clear_irq_status, sx127x_get_and_clear_irq_status,
    sx127x_get_gfsk_pkt_status, sx127x_get_irq_status, sx127x_get_lora_pkt_status,
    sx127x_get_rssi_inst, sx127x_get_rx_buffer_status, sx127x_init, sx127x_read_buffer,
    sx127x_read_register, sx127x_set_cad, sx127x_set_gfsk_mod_params, sx127x_set_gfsk_pkt_params,
    sx127x_set_gfsk_sync_word, sx127x_set_irq_mask, sx127x_set_lora_mod_params,
    sx127x_set_lora_pkt_params, sx127x_set_lora_sync_timeout, sx127x_set_lora_sync_word,
    sx127x_set_ocp_value, sx127x_set_pa_cfg, sx127x_set_pkt_type, sx127x_set_rf_freq,
    sx127x_set_rx, sx127x_set_sleep, sx127x_set_standby, sx127x_set_tx, sx127x_set_tx_cw,
    sx127x_set_tx_params, sx127x_write_buffer, sx127x_write_register,
};
pub use smtc_modem_cores_sys::{
    sx127x_gfsk_mod_params_t, sx127x_gfsk_pkt_params_t, sx127x_gfsk_pkt_status_t,
    sx127x_hal_status_e, sx127x_lora_mod_params_t, sx127x_lora_pkt_params_t,
    sx127x_lora_pkt_status_t, sx127x_pa_cfg_params_t, sx127x_radio_id_e, sx127x_rx_buffer_status_t,
    sx127x_status_e, sx127x_t,
};
use std::ffi;

use embedded_hal::spi::{Operation, SpiDevice};

/// Monomorphized HAL entry points for one concrete SPI type.
///
/// The sx127x HAL differs from the command-based chips: the C driver hands a
/// `sx127x_t*` (not an opaque context pointer) to every HAL call and reaches
/// its `hal_context` field, which we point at this header. The wire format is
/// register-based: writes clock `[address | 0x80]` then data, reads clock
/// `[address & 0x7F]` then read the data while MOSI idles at NOP.
#[repr(C)]
struct HalVtable {
    write: unsafe fn(*mut ffi::c_void, address: u8, data: &[u8]) -> sx127x_hal_status_e,
    read: unsafe fn(*mut ffi::c_void, address: u8, data: &mut [u8]) -> sx127x_hal_status_e,
}

/// The part of the context the exported HAL symbols can see without knowing
/// the SPI type. Must stay the first field of `Inner`.
#[repr(C)]
struct HalHeader {
    vtable: HalVtable,
    radio_id: sx127x_radio_id_e,
}

#[repr(C)]
struct Inner<I> {
    // Must stay the first field: the exported HAL symbols read the header
    // through `sx127x_t::hal_context` without knowing `I`.
    header: HalHeader,
    spi: I,
}

/// Owns the C driver's `sx127x_t` state block plus the SPI device. The inner
/// part is boxed so `hal_context` stays valid when the `Context` moves.
pub struct Context<I> {
    inner: Box<Inner<I>>,
    radio: sx127x_t,
}

pub enum Status {
    Ok,
    UnSupportedFeature,
    UnknownValue,
    Error,
    HalContextUninitialized,
}

impl From<sx127x_status_e> for Status {
    fn from(value: sx127x_status_e) -> Status {
        match value {
            sx127x_status_e::SX127X_STATUS_OK => Status::Ok,
            sx127x_status_e::SX127X_STATUS_UNSUPPORTED_FEATURE => Status::UnSupportedFeature,
            sx127x_status_e::SX127X_STATUS_UNKNOWN_VALUE => Status::UnknownValue,
            sx127x_status_e::SX127X_STATUS_ERROR => Status::Error,
            sx127x_status_e::SX127X_STATUS_HAL_CONTEXT_UNINITIALIZED => {
                Status::HalContextUninitialized
            }
        }
    }
}

impl<I: SpiDevice> Context<I> {
    /// Build a context for one of the supported radio IDs. Does NOT touch the
    /// SPI bus — call [`Context::init`] for the driver's own initialization.
    pub fn new(spi: I, radio_id: sx127x_radio_id_e) -> Self {
        let inner = Box::new(Inner {
            header: HalHeader {
                vtable: HalVtable {
                    write: write_shim::<I>,
                    read: read_shim::<I>,
                },
                radio_id,
            },
            spi,
        });
        let radio = sx127x_t {
            hal_context: &inner.header as *const HalHeader as *const ffi::c_void,
            // sx127x_init() fetches this via the HAL; the driver's variant
            // dispatch reads the struct field, so set it here too for use
            // without init()
            radio_id,
            ..Default::default()
        };
        Self { inner, radio }
    }
}

// &mut self on all of these: the C driver reaches the SPI device through
// hal_context
impl<I> Context<I> {
    pub fn spi(&self) -> &I {
        &self.inner.spi
    }

    pub fn spi_mut(&mut self) -> &mut I {
        &mut self.inner.spi
    }

    /// Runs `sx127x_init`: fetches the radio id, attaches (no-op) IRQ
    /// handlers, and writes the driver's register defaults to the bus.
    pub fn init(&mut self) -> Status {
        unsafe { sx127x_init(&mut self.radio) }.into()
    }

    pub fn set_sleep(&mut self) -> Status {
        unsafe { sx127x_set_sleep(&mut self.radio) }.into()
    }

    pub fn set_standby(&mut self) -> Status {
        unsafe { sx127x_set_standby(&mut self.radio) }.into()
    }

    pub fn set_tx(&mut self) -> Status {
        unsafe { sx127x_set_tx(&mut self.radio) }.into()
    }

    pub fn set_rx(&mut self, timeout_ms: u32) -> Status {
        unsafe { sx127x_set_rx(&mut self.radio, timeout_ms) }.into()
    }

    pub fn set_cad(&mut self) -> Status {
        unsafe { sx127x_set_cad(&mut self.radio) }.into()
    }

    pub fn set_tx_cw(&mut self) -> Status {
        unsafe { sx127x_set_tx_cw(&mut self.radio) }.into()
    }

    pub fn cal_img(&mut self, freq_in_hz: u32) -> Status {
        unsafe { sx127x_cal_img(&mut self.radio, freq_in_hz) }.into()
    }

    pub fn set_pa_cfg(&mut self, params: &sx127x_pa_cfg_params_t) -> Status {
        unsafe { sx127x_set_pa_cfg(&mut self.radio, params) }.into()
    }

    pub fn set_ocp_value(&mut self, ocp_trim: u8) -> Status {
        unsafe { sx127x_set_ocp_value(&mut self.radio, ocp_trim) }.into()
    }

    pub fn write_register(&mut self, address: u16, data: &[u8]) -> Status {
        unsafe { sx127x_write_register(&mut self.radio, address, data.as_ptr(), data.len() as u8) }
            .into()
    }

    pub fn read_register(&mut self, address: u16, data: &mut [u8]) -> Status {
        unsafe {
            sx127x_read_register(
                &mut self.radio,
                address,
                data.as_mut_ptr(),
                data.len() as u8,
            )
        }
        .into()
    }

    pub fn write_buffer(&mut self, offset: u8, data: &[u8]) -> Status {
        unsafe { sx127x_write_buffer(&mut self.radio, offset, data.as_ptr(), data.len() as u8) }
            .into()
    }

    pub fn read_buffer(&mut self, offset: u8, data: &mut [u8]) -> Status {
        unsafe { sx127x_read_buffer(&mut self.radio, offset, data.as_mut_ptr(), data.len() as u8) }
            .into()
    }

    pub fn set_irq_mask(&mut self, irq_mask: u16) -> Status {
        unsafe { sx127x_set_irq_mask(&mut self.radio, irq_mask) }.into()
    }

    pub fn get_irq_status(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { sx127x_get_irq_status(&mut self.radio, &mut out) }.into();
        (status, out)
    }

    pub fn clear_irq_status(&mut self, irq_mask: u16) -> Status {
        unsafe { sx127x_clear_irq_status(&mut self.radio, irq_mask) }.into()
    }

    pub fn get_and_clear_irq_status(&mut self) -> (Status, u16) {
        let mut out = 0u16;
        let status = unsafe { sx127x_get_and_clear_irq_status(&mut self.radio, &mut out) }.into();
        (status, out)
    }

    pub fn set_rf_freq(&mut self, freq_in_hz: u32) -> Status {
        unsafe { sx127x_set_rf_freq(&mut self.radio, freq_in_hz) }.into()
    }

    pub fn set_pkt_type(&mut self, pkt_type: u32) -> Status {
        unsafe { sx127x_set_pkt_type(&mut self.radio, pkt_type) }.into()
    }

    pub fn set_tx_params(&mut self, pwr_in_dbm: i8, ramp_time: u32) -> Status {
        unsafe { sx127x_set_tx_params(&mut self.radio, pwr_in_dbm, ramp_time) }.into()
    }

    pub fn set_gfsk_mod_params(&mut self, params: &sx127x_gfsk_mod_params_t) -> Status {
        unsafe { sx127x_set_gfsk_mod_params(&mut self.radio, params) }.into()
    }

    pub fn set_lora_mod_params(&mut self, params: &sx127x_lora_mod_params_t) -> Status {
        unsafe { sx127x_set_lora_mod_params(&mut self.radio, params) }.into()
    }

    pub fn set_gfsk_pkt_params(&mut self, params: &sx127x_gfsk_pkt_params_t) -> Status {
        unsafe { sx127x_set_gfsk_pkt_params(&mut self.radio, params) }.into()
    }

    pub fn set_lora_pkt_params(&mut self, params: &sx127x_lora_pkt_params_t) -> Status {
        unsafe { sx127x_set_lora_pkt_params(&mut self.radio, params) }.into()
    }

    pub fn set_lora_sync_timeout(&mut self, nb_of_symbs: u16) -> Status {
        unsafe { sx127x_set_lora_sync_timeout(&mut self.radio, nb_of_symbs) }.into()
    }

    pub fn set_gfsk_sync_word(&mut self, sync_word: &[u8]) -> Status {
        unsafe {
            sx127x_set_gfsk_sync_word(&mut self.radio, sync_word.as_ptr(), sync_word.len() as u8)
        }
        .into()
    }

    pub fn set_lora_sync_word(&mut self, sync_word: u8) -> Status {
        unsafe { sx127x_set_lora_sync_word(&mut self.radio, sync_word) }.into()
    }

    pub fn get_rx_buffer_status(&mut self) -> (Status, sx127x_rx_buffer_status_t) {
        let mut out = sx127x_rx_buffer_status_t::default();
        let status = unsafe { sx127x_get_rx_buffer_status(&mut self.radio, &mut out) }.into();
        (status, out)
    }

    pub fn get_gfsk_pkt_status(&mut self) -> (Status, sx127x_gfsk_pkt_status_t) {
        let mut out = sx127x_gfsk_pkt_status_t::default();
        let status = unsafe { sx127x_get_gfsk_pkt_status(&mut self.radio, &mut out) }.into();
        (status, out)
    }

    pub fn get_lora_pkt_status(&mut self) -> (Status, sx127x_lora_pkt_status_t) {
        let mut out = sx127x_lora_pkt_status_t::default();
        let status = unsafe { sx127x_get_lora_pkt_status(&mut self.radio, &mut out) }.into();
        (status, out)
    }

    pub fn get_rssi_inst(&mut self) -> (Status, i16) {
        let mut out = 0i16;
        let status = unsafe { sx127x_get_rssi_inst(&mut self.radio, &mut out) }.into();
        (status, out)
    }
}

/// # Safety
/// `ctx` must point at the `Inner<I>` this shim was monomorphized for, and
/// the C driver must not call back into the same context reentrantly.
unsafe fn write_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    address: u8,
    data: &[u8],
) -> sx127x_hal_status_e {
    let inner = unsafe { &mut *(ctx as *mut Inner<I>) };
    // wnr bit set for write access
    let cmd = [address | 0x80];
    let mut ops = [Operation::Write(&cmd), Operation::Write(data)];
    match inner.spi.transaction(&mut ops) {
        Ok(()) => sx127x_hal_status_e::SX127X_HAL_STATUS_OK,
        Err(_) => sx127x_hal_status_e::SX127X_HAL_STATUS_ERROR,
    }
}

/// # Safety
/// Same contract as [`write_shim`].
unsafe fn read_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    address: u8,
    data: &mut [u8],
) -> sx127x_hal_status_e {
    let inner = unsafe { &mut *(ctx as *mut Inner<I>) };
    let cmd = [address & 0x7F];
    let mut ops = [Operation::Write(&cmd), Operation::Read(data)];
    match inner.spi.transaction(&mut ops) {
        Ok(()) => sx127x_hal_status_e::SX127X_HAL_STATUS_OK,
        Err(_) => sx127x_hal_status_e::SX127X_HAL_STATUS_ERROR,
    }
}

unsafe fn header<'a>(radio: *const sx127x_t) -> &'a HalHeader {
    unsafe { &*((*radio).hal_context as *const HalHeader) }
}

/// HAL symbol the C driver links against, owned by this crate. Recovers the
/// vtable through the radio's `hal_context` field.
///
/// # Safety
/// Only the C driver should call this, with a radio originating from
/// `Context::new` and a data pointer valid for `data_len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_write(
    radio: *const sx127x_t,
    address: u16,
    data: *const u8,
    data_len: u16,
) -> sx127x_hal_status_e {
    let hdr = unsafe { header(radio) };
    let data = if data_len > 0 && !data.is_null() {
        unsafe { std::slice::from_raw_parts(data, data_len as usize) }
    } else {
        &[]
    };
    unsafe {
        (hdr.vtable.write)(
            (*radio).hal_context as *mut ffi::c_void,
            address as u8,
            data,
        )
    }
}

/// # Safety
/// Same contract as [`sx127x_hal_write`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_read(
    radio: *const sx127x_t,
    address: u16,
    data: *mut u8,
    data_len: u16,
) -> sx127x_hal_status_e {
    let hdr = unsafe { header(radio) };
    let data = if data_len > 0 && !data.is_null() {
        unsafe { std::slice::from_raw_parts_mut(data, data_len as usize) }
    } else {
        &mut []
    };
    unsafe {
        (hdr.vtable.read)(
            (*radio).hal_context as *mut ffi::c_void,
            address as u8,
            data,
        )
    }
}

/// # Safety
/// Only the C driver should call this, with a radio from `Context::new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_get_radio_id(radio: *const sx127x_t) -> sx127x_radio_id_e {
    unsafe { header(radio) }.radio_id
}

/// DIO interrupt wiring is GPIO-level in a real BSP; nothing to model.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_dio_irq_attach(_radio: *const sx127x_t) {}

/// Reset-pin toggle in a real BSP; no bytes cross the SPI bus. No-op.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_reset(_radio: *const sx127x_t) {}

/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_get_dio_1_pin_state(_radio: *const sx127x_t) -> u32 {
    0
}

/// The driver uses a host timer for GFSK RX timeouts; irrelevant for byte
/// comparison. Reports success without arming anything.
///
/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_timer_start(
    _radio: *const sx127x_t,
    _time_in_ms: u32,
    _callback: Option<unsafe extern "C" fn(*mut ffi::c_void)>,
) -> sx127x_hal_status_e {
    sx127x_hal_status_e::SX127X_HAL_STATUS_OK
}

/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_timer_stop(_radio: *const sx127x_t) -> sx127x_hal_status_e {
    sx127x_hal_status_e::SX127X_HAL_STATUS_OK
}

/// # Safety
/// Only the C driver should call this.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sx127x_hal_timer_is_started(_radio: *const sx127x_t) -> bool {
    false
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
                    Operation::Read(buf) => self.data.extend_from_slice(&vec![0u8; buf.len()]),
                    _ => todo!(),
                }
            }
            Ok(())
        }
    }

    #[test]
    fn rf_freq_burst_write() {
        let mut context = Context::new(
            TestSpi { data: vec![] },
            sx127x_radio_id_e::SX127X_RADIO_ID_SX1276,
        );
        // 868.1 MHz -> pll steps 0xD90666; one burst write to Frf MSB (0x06)
        context.set_rf_freq(868_100_000);
        assert_eq!(context.spi().data, vec![0x06 | 0x80, 0xD9, 0x06, 0x66]);
    }
}
