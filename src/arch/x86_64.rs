//! x86_64 architecture-specific CPU detection
//!
//! This module provides functionality for detecting CPU information on x86_64 systems
//! using CPUID instructions through our CPUID wrapper.

use crate::cpu::info::Frequency;
use crate::cpu::uarch::detect_uarch;
use crate::cpu::{CpuError, CpuInfo, CpuidWrapper, Vendor, Version};

/// Detect CPU information for x86_64 systems
pub fn detect_cpu() -> Result<CpuInfo, CpuError> {
    let cpuid = CpuidWrapper::new();

    // Basic CPU information via CPUID
    let basic_info = cpuid
        .get_basic_info()
        .map_err(|e| CpuError::InfoRead(format!("Failed to get basic CPU info: {}", e)))?;

    // Vendor
    let cpu_vendor = match basic_info.vendor_string.as_str() {
        "GenuineIntel" => Vendor::Intel,
        "AuthenticAMD" | "HygonGenuine" => Vendor::AMD,
        _ => Vendor::Unknown,
    };

    // Family/model/stepping with extended IDs folded in (Intel SDM Vol. 2A §3.2)
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

    // ISA feature flags
    let features = crate::cpu::detect_features()
        .map_err(|e| CpuError::InfoRead(format!("Failed to detect CPU features: {}", e)))?;

    // Core counts
    let logical_cores = num_cpus::get() as u32;
    let physical_cores = num_cpus::get_physical() as u32;

    // Frequency — delegate to the platform-specific detection in `cpu::frequency`
    let frequency = detect_frequency_for_info();

    // Cache topology
    let cache_sizes = detect_cache_sizes(&cpuid);

    // Microarchitecture lookup
    let microarch = detect_uarch(&cpu_vendor, version.family, version.model);

    // Hypervisor detection (CPUID leaf 1 ECX bit 31)
    let hypervisor = cpuid.detect_hypervisor();

    // Theoretical peak double-precision GFLOP/s
    let peak_flops = crate::cpu::perf::calculate_peak_flops(physical_cores, frequency.max, frequency.base, features);

    Ok(CpuInfo {
        vendor: cpu_vendor,
        brand_string: basic_info.brand_string,
        version,
        physical_cores,
        logical_cores,
        frequency,
        cache_sizes,
        features,
        microarch,
        hypervisor,
        peak_flops,
    })
}

/// Resolve CPU frequency using the `frequency` feature when available,
/// falling back to all-`None` when the feature is compiled out.
fn detect_frequency_for_info() -> Frequency {
    #[cfg(feature = "frequency")]
    {
        match crate::cpu::frequency::detect_frequency() {
            Ok(f) => Frequency {
                base: f.base,
                current: f.current,
                max: f.max,
            },
            Err(_) => Frequency::default(),
        }
    }

    #[cfg(not(feature = "frequency"))]
    {
        Frequency::default()
    }
}

/// Extract a simplified [L1i, L1d, L2, L3] cache size array from CPUID topology.
fn detect_cache_sizes(cpuid: &CpuidWrapper) -> [Option<u32>; 4] {
    let mut cache_sizes = [None; 4];

    if let Ok(topology) = cpuid.get_cache_topology() {
        for cache in topology.caches.iter().flatten() {
            let index = match (cache.level, cache.cache_type) {
                (1, crate::cpu::CacheType::Instruction) => Some(0),
                (1, crate::cpu::CacheType::Data) => Some(1),
                (2, _) => Some(2),
                (3, _) => Some(3),
                _ => None,
            };

            if let Some(idx) = index {
                cache_sizes[idx] = Some(cache.size_kb);
            }
        }
    }

    cache_sizes
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
        println!("Detected x86_64 CPU: {:?}", info);
        println!("Cache sizes: {:?}", info.cache_sizes);
        println!("Microarch: {:?}", info.microarch);
        println!("Hypervisor: {:?}", info.hypervisor);
        println!("Peak GFLOP/s: {:?}", info.peak_flops);
    }

    #[test]
    #[cfg_attr(not(target_arch = "x86_64"), ignore)]
    fn test_frequency_populated() {
        let info = detect_cpu().unwrap();
        // On Linux and macOS, at least one frequency field should be populated
        // (Windows WMI may also provide it, but that's environment-dependent)
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        assert!(
            info.frequency.base.is_some() || info.frequency.max.is_some() || info.frequency.current.is_some(),
            "No frequency data detected — frequency feature may be disabled or unavailable"
        );
    }
}
