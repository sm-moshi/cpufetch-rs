//! ARM64 architecture-specific CPU detection
//!
//! This module provides functionality for detecting CPU information on ARM64 systems.

use crate::cpu::info::Frequency;
use crate::cpu::{ArmFeatures, CpuError, CpuInfo, Vendor, Version};

/// Detect CPU information for ARM64 systems
pub fn detect_cpu() -> Result<CpuInfo, CpuError> {
    // Basic implementation for now
    Ok(CpuInfo {
        vendor: Vendor::ARM,
        brand_string: String::from("ARM Processor"),
        version: Version {
            family: 0,
            model: 0,
            stepping: 0,
        },
        physical_cores: num_cpus::get_physical() as u32,
        logical_cores: num_cpus::get() as u32,
        frequency: Frequency {
            base: None,
            max: None,
            current: None,
        },
        cache_sizes: [None; 4],
        features: detect_arm_features().map_err(|e| CpuError::InfoRead(e.to_string()))?,
    })
}

/// Detect ARM CPU features
fn detect_arm_features() -> Result<ArmFeatures, CpuError> {
    let mut features = ArmFeatures::empty();

    // Basic feature detection using is_aarch64_feature_detected!
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            features |= ArmFeatures::NEON;
        }
        if std::arch::is_aarch64_feature_detected!("aes") {
            features |= ArmFeatures::AES;
        }
        if std::arch::is_aarch64_feature_detected!("pmull") {
            features |= ArmFeatures::PMULL;
        }
        if std::arch::is_aarch64_feature_detected!("sha2") {
            features |= ArmFeatures::SHA2;
        }
        if std::arch::is_aarch64_feature_detected!("crc") {
            features |= ArmFeatures::CRC32;
        }
        if std::arch::is_aarch64_feature_detected!("lse") {
            features |= ArmFeatures::ATOMICS;
        }
        if std::arch::is_aarch64_feature_detected!("fp") {
            features |= ArmFeatures::FP;
        }
        if std::arch::is_aarch64_feature_detected!("asimd") {
            features |= ArmFeatures::ASIMD;
        }
    }

    Ok(features)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cpu() {
        let info = detect_cpu().unwrap();
        assert_eq!(info.vendor, Vendor::ARM);
        assert!(!info.brand_string.is_empty());
        assert!(info.logical_cores > 0);
        assert!(info.physical_cores > 0);
    }
}
