//! Webots `v2025a` bindings and safe wrapper layer.
//!
//! Most applications construct [`Webots`] and then request typed device handles from it.

pub mod bindings;

/// The Webots API version implemented by this module tree.
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

/// Supervisor helpers for scene-tree and simulation management APIs.
pub use supervisor::Supervisor;
/// Primary controller entrypoint for the `v2025a` API surface.
pub use webots::{Simulator, Webots};

use std::ffi::NulError;
use std::str::Utf8Error;
use thiserror::Error;

/// Errors returned by safe wrapper operations.
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
