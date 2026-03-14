//! Webots simulator bindings and utilities.
//!
//! This crate provides versioned Rust wrappers around the Webots C API.

#![allow(clippy::macro_metavars_in_unsafe)]

#[cfg(feature = "v2025a")]
pub mod v2025a;

#[cfg(feature = "v2025a")]
pub use v2025a::*;

#[cfg(feature = "v2025a")]
pub const WEBOTS_API_VERSION: &str = v2025a::API_VERSION;
