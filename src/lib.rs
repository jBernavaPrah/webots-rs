#![doc = include_str!("../README.md")]
//! Additional crate docs:
//! - [`Webots`] is the primary controller entrypoint.
//! - [`Simulator`] is a type alias for [`Webots`].
//! - [`WEBOTS_API_VERSION`] exposes the selected bindings version at compile time.

#![allow(clippy::macro_metavars_in_unsafe)]

#[cfg(feature = "v2025a")]
pub mod v2025a;

#[cfg(feature = "v2025a")]
pub use v2025a::*;

/// The active Webots API namespace selected by Cargo features.
#[cfg(feature = "v2025a")]
pub const WEBOTS_API_VERSION: &str = v2025a::API_VERSION;

/// Whether this build linked against a real Webots controller runtime.
#[cfg(webots_runtime_linked)]
pub const WEBOTS_RUNTIME_LINKED: bool = true;

/// Whether this build linked against a real Webots controller runtime.
#[cfg(not(webots_runtime_linked))]
pub const WEBOTS_RUNTIME_LINKED: bool = false;
