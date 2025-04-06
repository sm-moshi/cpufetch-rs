//! CPU detection and information module.
//!
//! This module provides functionality for detecting and querying CPU information,
//! including feature detection, frequency measurement, and vendor identification.

pub mod flags;
pub mod info;

// Re-export commonly used types and functions
pub use flags::{ArmFeatures, FeatureError, X86Features, detect_features};
pub use info::{CpuError, CpuInfo, Frequency, Vendor, Version};
