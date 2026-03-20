//! CPU detection and information module.
//!
//! This module provides functionality for detecting and querying CPU information,
//! including feature detection, frequency measurement, and vendor identification.

pub mod cpuid;
pub mod flags;
pub mod info;
pub mod perf;
pub mod uarch;

// Conditionally include the frequency module based on feature flag
#[cfg(feature = "frequency")]
pub mod frequency;

// Re-export commonly used types and functions
pub use cpuid::{CacheInfo, CacheType, CpuidError, CpuidWrapper};
pub use flags::{ArmFeatures, FeatureError, X86Features, detect_features};
pub use info::{CpuError, CpuInfo, Vendor, Version};
pub use uarch::{Microarch, detect_uarch};

// Conditionally re-export the frequency module
#[cfg(feature = "frequency")]
pub use frequency::{Frequency, detect_frequency};
#[cfg(not(feature = "frequency"))]
pub use info::Frequency;
