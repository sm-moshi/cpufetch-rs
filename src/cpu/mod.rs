//! CPU detection and information module.
//!
//! This module provides functionality for detecting and querying CPU information
//! across different architectures. It handles feature detection, frequency
//! measurement, and general CPU capabilities.

mod info;
mod cpuid;
mod flags;
mod frequency;

pub use info::{CpuError, CpuInfo, Frequency, Vendor, Version};

/// Re-export commonly used types and functions
pub mod prelude {
    pub use super::{CpuError, CpuInfo, Frequency, Vendor, Version};
}
