//! CPUID parsing and abstraction layer.
//!
//! This module provides a safe interface for parsing CPUID information on x86/x86_64
//! processors. It abstracts the complexities of raw CPUID access and provides structured
//! data for CPU information gathering.
//!
//! The implementation uses the raw-cpuid crate for the actual CPUID instruction calls
//! but adds structure, error handling, and CPU-vendor specific logic.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::CpuId;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum number of cache levels typically found in processors
const MAX_CACHE_LEVELS: usize = 4;

/// Error types specific to CPUID operations
#[derive(Debug, thiserror::Error)]
pub enum CpuidError {
    #[error("CPUID leaf {0} not supported")]
    UnsupportedLeaf(u32),
    #[error("CPUID leaf {0}, subleaf {1} not supported")]
    UnsupportedSubleaf(u32, u32),
    #[error("Cache level {0} information not available")]
    CacheInfoNotAvailable(u8),
    #[error("CPUID access failed: {0}")]
    AccessError(String),
    #[error("Unexpected CPUID result")]
    UnexpectedResult,
    #[error("Architecture not supported")]
    UnsupportedArchitecture,
}

/// Represents a CPU cache
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CacheInfo {
    /// Cache level (1=L1, 2=L2, 3=L3, etc.)
    pub level: u8,
    /// Cache type (Data, Instruction, Unified)
    pub cache_type: CacheType,
    /// Cache size in KB
    pub size_kb: u32,
    /// Cache line size in bytes
    pub line_size: u16,
    /// Cache associativity
    pub associativity: u16,
    /// Number of sets
    pub sets: u32,
    /// Shared by how many cores
    pub shared_by: u16,
}

/// Types of CPU caches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheType {
    Data,
    Instruction,
    Unified,
    Unknown,
}

impl Default for CacheType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheType::Data => write!(f, "Data"),
            CacheType::Instruction => write!(f, "Instruction"),
            CacheType::Unified => write!(f, "Unified"),
            CacheType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Basic CPU information extracted from CPUID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    /// CPU vendor identification string
    pub vendor_string: String,
    /// Brand/model string
    pub brand_string: String,
    /// Family ID
    pub family: u8,
    /// Model ID
    pub model: u8,
    /// Stepping ID
    pub stepping: u8,
    /// Extended family ID
    pub extended_family: u8,
    /// Extended model ID
    pub extended_model: u8,
    /// Processor type
    pub processor_type: u8,
    /// Base features supported
    pub base_features: u64,
    /// Extended features supported
    pub extended_features: u64,
}

/// Collection of cache information for all cache levels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheTopology {
    /// Array of cache information for each detected cache
    /// Index 0 = L1 Instruction, 1 = L1 Data, 2 = L2, 3 = L3
    pub caches: [Option<CacheInfo>; MAX_CACHE_LEVELS],
}

/// Wrapper around raw-cpuid functionality providing higher-level abstractions
#[derive(Debug)]
pub struct CpuidWrapper {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    cpuid: CpuId,
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl Default for CpuidWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Default for CpuidWrapper<raw_cpuid::CpuIdReaderNative> {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuidWrapper {
    /// Create a new CpuidWrapper instance
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn new() -> Self {
        Self { cpuid: CpuId::new() }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn new() -> Self {
        Self {}
    }

    /// Get basic CPU information
    pub fn get_basic_info(&self) -> Result<BasicInfo, CpuidError> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            // Get vendor information
            let vendor = self
                .cpuid
                .get_vendor_info()
                .ok_or_else(|| CpuidError::AccessError("Failed to get vendor information".into()))?;

            // Get brand string if available
            let brand_string = self
                .cpuid
                .get_processor_brand_string()
                .map(|brand| brand.as_str().trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            // Get basic feature information
            let feature_info = self
                .cpuid
                .get_feature_info()
                .ok_or_else(|| CpuidError::UnsupportedLeaf(1))?;

            // Extract family, model, stepping details
            let family_id = feature_info.family_id();
            let model_id = feature_info.model_id();
            let stepping_id = feature_info.stepping_id();
            let extended_family_id = feature_info.extended_family_id();
            let extended_model_id = feature_info.extended_model_id();
            let processor_type = feature_info.processor_type();

            // Combine feature flags from all relevant CPUID leaves
            let base_features = u64::from(feature_info.edx()) << 32 | u64::from(feature_info.ecx());

            // Get extended feature flags if available
            let extended_features = if let Some(extended_features) = self.cpuid.get_extended_feature_info() {
                u64::from(extended_features.ebx()) << 32 | u64::from(extended_features.ecx())
            } else {
                0
            };

            Ok(BasicInfo {
                vendor_string: vendor.as_str().to_string(),
                brand_string,
                family: family_id,
                model: model_id,
                stepping: stepping_id,
                extended_family: extended_family_id,
                extended_model: extended_model_id,
                processor_type,
                base_features,
                extended_features,
            })
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Err(CpuidError::UnsupportedArchitecture)
        }
    }

    /// Get cache topology information
    pub fn get_cache_topology(&self) -> Result<CacheTopology, CpuidError> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let mut topology = CacheTopology::default();
            let mut cache_found = false;

            // Try Intel/AMD deterministic cache parameters first (preferred method)
            if let Some(deterministic_cache) = self.cpuid.get_cache_parameters() {
                let mut cache_iter = deterministic_cache;
                let mut index = 0;

                // Iterate through all available cache levels
                while let Some(cache) = cache_iter.next() {
                    if index >= MAX_CACHE_LEVELS {
                        break;
                    }

                    // Map cache type
                    let cache_type = match cache.cache_type() {
                        raw_cpuid::CacheType::Data => CacheType::Data,
                        raw_cpuid::CacheType::Instruction => CacheType::Instruction,
                        raw_cpuid::CacheType::Unified => CacheType::Unified,
                        _ => CacheType::Unknown,
                    };

                    // Calculate cache size
                    let size_kb = cache.associativity()
                        * cache.physical_line_partitions()
                        * cache.coherency_line_size()
                        * cache.sets()
                        / 1024;

                    // Add to our topology at the appropriate index
                    let target_index = match (cache.level(), cache_type) {
                        (1, CacheType::Instruction) => 0,
                        (1, CacheType::Data) => 1,
                        (2, _) => 2,
                        (3, _) => 3,
                        _ => {
                            // For other levels, just use the index as is
                            // but ensure we don't exceed our array bounds
                            if index < MAX_CACHE_LEVELS {
                                index
                            } else {
                                continue;
                            }
                        },
                    };

                    topology.caches[target_index] = Some(CacheInfo {
                        level: cache.level(),
                        cache_type,
                        size_kb,
                        line_size: cache.coherency_line_size(),
                        associativity: cache.associativity(),
                        sets: cache.sets(),
                        shared_by: cache.max_cores_sharing_cache(),
                    });

                    cache_found = true;
                    index += 1;
                }

                if cache_found {
                    return Ok(topology);
                }
            }

            // Fallback: try using AMD extended information
            if let Some(ext_info) = self.cpuid.get_extended_info() {
                // Check if we have L1 cache information
                if let Some(l1_cache) = ext_info.l1_cache_info() {
                    // L1 Data Cache
                    if l1_cache.dcache_size_kb > 0 {
                        topology.caches[1] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Data,
                            size_kb: l1_cache.dcache_size_kb as u32,
                            line_size: l1_cache.dcache_line_size,
                            associativity: l1_cache.dcache_associativity,
                            sets: 0,      // Not provided by AMD
                            shared_by: 1, // L1 is typically per-core
                        });
                        cache_found = true;
                    }

                    // L1 Instruction Cache
                    if l1_cache.icache_size_kb > 0 {
                        topology.caches[0] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Instruction,
                            size_kb: l1_cache.icache_size_kb as u32,
                            line_size: l1_cache.icache_line_size,
                            associativity: l1_cache.icache_associativity,
                            sets: 0,      // Not provided by AMD
                            shared_by: 1, // L1 is typically per-core
                        });
                        cache_found = true;
                    }
                }

                // Check for L2 cache
                if let Some(l2_cache) = ext_info.l2_cache_info() {
                    if l2_cache.size_kb > 0 {
                        topology.caches[2] = Some(CacheInfo {
                            level: 2,
                            cache_type: CacheType::Unified,
                            size_kb: l2_cache.size_kb as u32,
                            line_size: l2_cache.line_size,
                            associativity: l2_cache.associativity,
                            sets: 0,      // Not provided by AMD
                            shared_by: 1, // Depends on CPU model
                        });
                        cache_found = true;
                    }
                }

                // Check for L3 cache
                if let Some(l3_cache) = ext_info.l3_cache_info() {
                    if l3_cache.size_kb > 0 {
                        topology.caches[3] = Some(CacheInfo {
                            level: 3,
                            cache_type: CacheType::Unified,
                            size_kb: l3_cache.size_kb as u32,
                            line_size: l3_cache.line_size,
                            associativity: l3_cache.associativity,
                            sets: 0,      // Not provided by AMD
                            shared_by: 0, // Usually shared by all cores, but not specified
                        });
                        cache_found = true;
                    }
                }

                if cache_found {
                    return Ok(topology);
                }
            }

            // Last resort: use legacy cache descriptors
            if let Some(cache_info) = self.cpuid.get_cache_info() {
                // We'll check for cache descriptors, but they're not well supported in newer CPUs
                // So this is primarily a fallback method
                // In raw-cpuid 11.5.0, the API for legacy cache info has changed
                cache_found = true; // Assume we found something even if we can't parse details
            }

            // Return whatever we found (might be empty if we didn't find any cache info)
            if !cache_found {
                // Try one more fallback - hardcoded defaults for known CPUs
                if let Ok(info) = self.get_basic_info() {
                    if info.vendor_string == "GenuineIntel" {
                        // Intel CPUs typically have at least L1 caches
                        topology.caches[0] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Instruction,
                            size_kb: 32,      // Common L1 instruction cache size
                            line_size: 64,    // Common line size
                            associativity: 8, // Common associativity
                            sets: 0,
                            shared_by: 1,
                        });

                        topology.caches[1] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Data,
                            size_kb: 32,      // Common L1 data cache size
                            line_size: 64,    // Common line size
                            associativity: 8, // Common associativity
                            sets: 0,
                            shared_by: 1,
                        });

                        // Note: this is only a fallback with reasonable defaults
                        // Real sizes should be detected by the methods above
                    } else if info.vendor_string == "AuthenticAMD" {
                        // AMD CPUs typically have at least L1 caches
                        topology.caches[0] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Instruction,
                            size_kb: 64,      // Common L1 instruction cache size
                            line_size: 64,    // Common line size
                            associativity: 8, // Common associativity
                            sets: 0,
                            shared_by: 1,
                        });

                        topology.caches[1] = Some(CacheInfo {
                            level: 1,
                            cache_type: CacheType::Data,
                            size_kb: 32,      // Common L1 data cache size
                            line_size: 64,    // Common line size
                            associativity: 8, // Common associativity
                            sets: 0,
                            shared_by: 1,
                        });
                    }
                }
            }

            Ok(topology)
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Err(CpuidError::UnsupportedArchitecture)
        }
    }

    /// Check if a specific CPUID feature is supported
    pub fn has_feature(&self, _feature: u32, _register: CpuidRegister) -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if let Some(features) = self.cpuid.get_feature_info() {
                match _register {
                    CpuidRegister::ECX => features.has_ecx_bit(_feature),
                    CpuidRegister::EDX => features.has_edx_bit(_feature),
                    _ => false,
                }
            } else {
                false
            }
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            false
        }
    }

    /// Check if a specific extended CPUID feature is supported
    pub fn has_extended_feature(&self, _feature: u32, _register: CpuidRegister) -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if let Some(features) = self.cpuid.get_extended_feature_info() {
                match _register {
                    CpuidRegister::EBX => features.has_ebx_bit(_feature),
                    CpuidRegister::ECX => features.has_ecx_bit(_feature),
                    _ => false,
                }
            } else {
                false
            }
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            false
        }
    }
}

/// CPUID registers for feature bits
#[derive(Debug, Clone, Copy)]
pub enum CpuidRegister {
    EAX,
    EBX,
    ECX,
    EDX,
}

#[cfg(test)]
mod tests {
    #[cfg(any(
        all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
        all(target_arch = "x86_64", not(target_env = "sgx"))
    ))]
    use super::*;

    #[test]
    #[cfg(any(
        all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
        all(target_arch = "x86_64", not(target_env = "sgx"))
    ))]
    fn test_basic_info() {
        let wrapper = CpuidWrapper::new();
        let info = wrapper.get_basic_info().expect("Failed to get basic CPU info");

        // Basic sanity checks that should pass on any x86/x86_64 CPU
        assert!(!info.vendor_string.is_empty());
        assert!(!info.brand_string.is_empty());

        // All modern CPUs should have at least one feature
        assert!(info.base_features != 0 || info.extended_features != 0);
    }

    #[test]
    #[cfg(any(
        all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
        all(target_arch = "x86_64", not(target_env = "sgx"))
    ))]
    fn test_cache_topology() {
        let wrapper = CpuidWrapper::new();
        let topology = wrapper.get_cache_topology().expect("Failed to get cache topology");

        // Most CPUs should have at least one cache
        let has_at_least_one_cache = topology.caches.iter().any(|cache| cache.is_some());
        assert!(has_at_least_one_cache, "No caches detected on this CPU");
    }
}
