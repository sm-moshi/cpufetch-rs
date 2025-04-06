//! Architecture-specific CPU detection implementations
//!
//! This module contains CPU detection implementations for different architectures.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
