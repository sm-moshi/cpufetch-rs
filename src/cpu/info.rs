//! CPU information structures and traits.
//!
//! This module provides the core data structures for representing CPU information
//! across different architectures. It aims to provide a unified interface for
//! accessing CPU details regardless of the underlying hardware.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Error types for CPU information gathering
#[derive(Debug, thiserror::Error)]
pub enum CpuError {
    #[error("Failed to detect CPU vendor: {0}")]
    VendorDetection(String),
    #[error("Failed to read CPU information: {0}")]
    InfoRead(String),
    #[error("Unsupported CPU architecture")]
    UnsupportedArch,
}

/// CPU vendor identification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vendor {
    Intel,
    AMD,
    ARM,
    Apple,
    Unknown,
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vendor::Intel => write!(f, "Intel"),
            Vendor::AMD => write!(f, "AMD"),
            Vendor::ARM => write!(f, "ARM"),
            Vendor::Apple => write!(f, "Apple"),
            Vendor::Unknown => write!(f, "Unknown"),
        }
    }
}

/// CPU frequency information in MHz
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Frequency {
    /// Base/nominal frequency
    pub base: Option<u32>,
    /// Maximum turbo frequency
    pub max: Option<u32>,
    /// Current operating frequency
    pub current: Option<u32>,
}

/// Represents version information for a CPU
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Version {
    /// CPU family identifier
    pub family: u8,
    /// CPU model identifier
    pub model: u8,
    /// CPU stepping identifier
    pub stepping: u8,
}

/// Core CPU information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// CPU vendor identification
    pub vendor: Vendor,
    /// Marketing name of the CPU
    pub brand_string: String,
    /// Version information (family/model/stepping)
    pub version: Version,
    /// Number of physical CPU cores
    pub physical_cores: u32,
    /// Number of logical CPU threads
    pub logical_cores: u32,
    /// Frequency information
    pub frequency: Frequency,
    /// Cache sizes in KB (L1i, L1d, L2, L3)
    pub cache_sizes: [Option<u32>; 4],
    /// CPU features
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub features: crate::cpu::X86Features,
    #[cfg(target_arch = "aarch64")]
    pub features: crate::cpu::ArmFeatures,
}

impl CpuInfo {
    /// Creates a new CpuInfo instance by detecting the current CPU
    pub fn new() -> Result<Self, CpuError> {
        #[cfg(target_arch = "x86_64")]
        {
            crate::arch::x86_64::detect_cpu()
        }
        #[cfg(target_arch = "aarch64")]
        {
            crate::arch::aarch64::detect_cpu()
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Err(CpuError::UnsupportedArch)
        }
    }

    /// Returns a reference to a statically detected CPU info
    ///
    /// This is useful when you want to avoid the overhead of detecting
    /// the CPU multiple times during program execution.
    pub fn get() -> &'static Self {
        static CPU_INFO: once_cell::sync::Lazy<CpuInfo> =
            once_cell::sync::Lazy::new(|| {
                CpuInfo::new().expect("Failed to detect CPU information")
            });
        &CPU_INFO
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            vendor: Vendor::Unknown,
            brand_string: String::new(),
            version: Version::default(),
            physical_cores: 0,
            logical_cores: 0,
            frequency: Frequency::default(),
            cache_sizes: [None; 4],
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            features: crate::cpu::X86Features::empty(),
            #[cfg(target_arch = "aarch64")]
            features: crate::cpu::ArmFeatures::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_display() {
        assert_eq!(Vendor::Intel.to_string(), "Intel");
        assert_eq!(Vendor::AMD.to_string(), "AMD");
        assert_eq!(Vendor::ARM.to_string(), "ARM");
        assert_eq!(Vendor::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_cpu_info_default() {
        let info = CpuInfo::default();
        assert_eq!(info.vendor, Vendor::Unknown);
        assert_eq!(info.physical_cores, 0);
        assert_eq!(info.logical_cores, 0);
        assert_eq!(info.cache_sizes, [None; 4]);
    }
}
