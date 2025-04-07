//! x86_64 architecture-specific CPU detection
//!
//! This module provides functionality for detecting CPU information on x86_64 systems
//! using CPUID instructions through our CPUID wrapper.

use crate::cpu::info::Frequency;
use crate::cpu::{CacheInfo, CpuError, CpuInfo, CpuidError, CpuidWrapper, Vendor, Version, X86Features};

/// Detect CPU information for x86_64 systems
pub fn detect_cpu() -> Result<CpuInfo, CpuError> {
    // Create a CPUID wrapper
    let cpuid = CpuidWrapper::new();

    // Get basic CPU information
    let basic_info = cpuid
        .get_basic_info()
        .map_err(|e| CpuError::InfoRead(format!("Failed to get basic CPU info: {}", e)))?;

    // Determine the vendor
    let cpu_vendor = match basic_info.vendor_string.as_str() {
        "GenuineIntel" => Vendor::Intel,
        "AuthenticAMD" => Vendor::AMD,
        _ => Vendor::Unknown,
    };

    // Convert extended family/model info into version structure
    let version = Version {
        family: if basic_info.family == 0xF {
            ((basic_info.extended_family as u16) << 4) as u8 + basic_info.family
        } else {
            basic_info.family
        },
        model: if basic_info.family == 0xF || basic_info.family == 0x6 {
            ((basic_info.extended_model as u16) << 4) as u8 + basic_info.model
        } else {
            basic_info.model
        },
        stepping: basic_info.stepping,
    };

    // Get CPU features
    let features = crate::cpu::detect_features()
        .map_err(|e| CpuError::InfoRead(format!("Failed to detect CPU features: {}", e)))?;

    // Get core counts using num_cpus crate
    let logical_cores = num_cpus::get() as u32;
    let physical_cores = num_cpus::get_physical() as u32;

    // For now, use a default Frequency structure
    // TODO: Implement proper frequency detection using MSR or platform-specific APIs
    let frequency = Frequency {
        base: None,
        max: None,
        current: None,
    };

    // Get cache sizes from our CPUID cache topology
    let mut cache_sizes = [None; 4];

    if let Ok(topology) = cpuid.get_cache_topology() {
        // Map our cache topology to the simplified array format
        for (i, cache) in topology.caches.iter().enumerate() {
            if let Some(cache_info) = cache {
                let index = match (cache_info.level, cache_info.cache_type) {
                    // L1 Instruction Cache
                    (1, crate::cpu::CacheType::Instruction) => Some(0),
                    // L1 Data Cache
                    (1, crate::cpu::CacheType::Data) => Some(1),
                    // L2 Cache (Unified or Data)
                    (2, _) => Some(2),
                    // L3 Cache
                    (3, _) => Some(3),
                    // Other caches not represented in our simplified model
                    _ => None,
                };

                // If this is a cache we want to track, store its size
                if let Some(idx) = index {
                    if idx < cache_sizes.len() {
                        cache_sizes[idx] = Some(cache_info.size_kb);
                    }
                }
            }
        }
    }

    Ok(CpuInfo {
        vendor: cpu_vendor,
        brand_string: basic_info.brand_string,
        version,
        physical_cores,
        logical_cores,
        frequency,
        cache_sizes,
        features,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(target_arch = "x86_64"), ignore)]
    fn test_detect_cpu() {
        let info = detect_cpu().unwrap();
        assert!(!info.brand_string.is_empty());
        assert!(info.logical_cores > 0);
        assert!(info.physical_cores > 0);

        // Print info for debugging
        println!("Detected x86_64 CPU: {:?}", info);
        println!("Cache sizes: {:?}", info.cache_sizes);
    }
}
