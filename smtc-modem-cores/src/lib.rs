#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]

pub mod lr11xx;
pub mod sx126x;
pub mod sx127x;

/// Raw bindings, re-exported so downstream tests can reach the constified
/// enum values (most lr11xx enums bind as consts, not Rust enums)
pub use smtc_modem_cores_sys as sys;
