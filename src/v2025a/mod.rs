pub mod bindings;

pub const API_VERSION: &str = "v2025a";

#[macro_export]
macro_rules! ffi_try {
    ($expr:expr) => {
        ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| unsafe { $expr }))
            .map_err(|_| $crate::v2025a::SimulatorError::UnsafeOperation)
    };
}

#[cfg(not(webots_runtime_linked))]
mod bindings_stubs {
    use crate::v2025a::bindings::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_stubs.rs"));

    #[cfg(windows)]
    #[no_mangle]
    pub unsafe extern "C" fn wb_robot_init_msvc() -> ::std::os::raw::c_int {
        unsafe { ::std::mem::zeroed() }
    }
}

pub mod device;
pub mod supervisor;
mod webots;

pub use supervisor::Supervisor;
pub use webots::{Simulator, Webots};

use std::ffi::NulError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimulatorError {
    #[error("unknown device: {0}")]
    UnknownDevice(String),
    #[error("invalid name")]
    InvalidName(#[from] NulError),
    #[error("invalid utf8 string")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("unsafe call failed")]
    UnsafeOperation,
}
