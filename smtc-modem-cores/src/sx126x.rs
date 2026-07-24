pub use smtc_modem_cores_sys::{sx126x_hal_status_e, sx126x_status_e};
use smtc_modem_cores_sys::{sx126x_set_sleep, sx126x_sleep_cfgs_e};
use std::ffi;

use embedded_hal::spi::{Operation, SpiDevice};

/// Monomorphized HAL entry points for one concrete SPI type. C hands the
/// context pointer back on every HAL call, so dispatch goes through this
/// table rather than through a user-provided `#[no_mangle]` symbol.
#[repr(C)]
struct HalVtable {
    write: unsafe fn(*mut ffi::c_void, command: &[u8], data: &[u8]) -> sx126x_hal_status_e,
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("smtc-modem-cores sx126x invalid status value: {0}")]
    InvalidStatusValue(u32),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl From<sx126x_status_e> for Status {
    fn from(value: sx126x_status_e) -> Status {
        match value {
            sx126x_status_e::SX126X_STATUS_OK => Status::Ok,
            sx126x_status_e::SX126X_STATUS_UNSUPPORTED_FEATURE => Status::UnSupportedFeature,
            sx126x_status_e::SX126X_STATUS_UNKNOWN_VALUE => Status::UnknownValue,
            sx126x_status_e::SX126X_STATUS_ERROR => Status::Error,
        }
    }
}

impl From<Status> for sx126x_status_e {
    fn from(status: Status) -> sx126x_status_e {
        match status {
            Status::Ok => sx126x_status_e::SX126X_STATUS_OK,
            Status::UnSupportedFeature => sx126x_status_e::SX126X_STATUS_UNSUPPORTED_FEATURE,
            Status::UnknownValue => sx126x_status_e::SX126X_STATUS_UNKNOWN_VALUE,
            Status::Error => sx126x_status_e::SX126X_STATUS_ERROR,
        }
    }
}

pub enum SleepCfg {
    ColdStart,
    WarmStart,
}

impl From<SleepCfg> for sx126x_sleep_cfgs_e {
    fn from(cfg: SleepCfg) -> sx126x_sleep_cfgs_e {
        match cfg {
            SleepCfg::ColdStart => sx126x_sleep_cfgs_e::SX126X_SLEEP_CFG_COLD_START,
            SleepCfg::WarmStart => sx126x_sleep_cfgs_e::SX126X_SLEEP_CFG_WARM_START,
        }
    }
}

impl<I: SpiDevice> Context<I> {
    pub fn new(inner: I) -> Self {
        Self {
            vtable: HalVtable {
                write: write_shim::<I>,
            },
            inner,
        }
    }
}

impl<I> Context<I> {
    // &mut self: the C driver writes to `inner` through the context pointer
    pub fn set_sleep(&mut self, cfg: SleepCfg) {
        let ptr = self as *mut Self as *const ffi::c_void;
        unsafe {
            sx126x_set_sleep(ptr, cfg.into());
        }
    }
}

/// # Safety
/// `ctx` must point at the `Context<I>` this shim was monomorphized for, and
/// the C driver must not call back into the same context reentrantly.
unsafe fn write_shim<I: SpiDevice>(
    ctx: *mut ffi::c_void,
    command: &[u8],
    data: &[u8],
) -> sx126x_hal_status_e {
    let context = unsafe { &mut *(ctx as *mut Context<I>) };
    // Command and data belong to a single chip-select assertion
    let mut ops = [Operation::Write(command), Operation::Write(data)];
    match context.inner.transaction(&mut ops) {
        Ok(()) => sx126x_hal_status_e::SX126X_HAL_STATUS_OK,
        Err(_) => sx126x_hal_status_e::SX126X_HAL_STATUS_ERROR,
    }
}

unsafe fn slice_or_empty<'a>(ptr: *const u8, len: u16) -> &'a [u8] {
    if len > 0 && !ptr.is_null() {
        unsafe { std::slice::from_raw_parts(ptr, len as usize) }
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
pub unsafe extern "C" fn sx126x_hal_write(
    context: *const ffi::c_void,
    command: *const u8,
    command_length: u16,
    data: *const u8,
    data_length: u16,
) -> sx126x_hal_status_e {
    let vtable = unsafe { &*(context as *const HalVtable) };
    let command = unsafe { slice_or_empty(command, command_length) };
    let data = unsafe { slice_or_empty(data, data_length) };
    unsafe { (vtable.write)(context as *mut ffi::c_void, command, data) }
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
    fn sleep_cold_and_warm() {
        let mut context = Context::new(TestSpiA { data: vec![] });
        context.set_sleep(SleepCfg::ColdStart);

        let mut other = Context::new(TestSpiB { data: vec![] });
        other.set_sleep(SleepCfg::WarmStart);

        // SetSleep opcode 0x84; sleep cfg byte is the enum value directly
        assert_eq!(context.inner.data, vec![0x84, 0x00]);
        assert_eq!(other.inner.data, vec![0x84, 0x04]);
    }
}
