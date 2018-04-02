//! Abstracts architecture details.
//!
//! The job of this module is to have submodules for each architecture and to
//! provide interfaces to them.

#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

