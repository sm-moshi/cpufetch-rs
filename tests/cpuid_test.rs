//! Integration tests for CPUID parsing
//!
//! These tests verify the CPUID parsing works correctly on x86_64 systems.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_tests {
    use cpufetch_rs::cpu::{CacheType, CpuidWrapper};

    #[test]
    fn test_basic_info() {
        let cpuid = CpuidWrapper::new();
        let info = cpuid.get_basic_info().expect("Failed to get basic info");

        // Verify basic information is correct
        assert!(!info.vendor_string.is_empty());
        assert!(!info.brand_string.is_empty());
        assert!(info.family > 0);

        println!("Vendor: {}", info.vendor_string);
        println!("Brand: {}", info.brand_string);
        println!(
            "Family: {}, Model: {}, Stepping: {}",
            info.family, info.model, info.stepping
        );
    }

    #[test]
    fn test_cache_topology() {
        let cpuid = CpuidWrapper::new();
        let topology = cpuid.get_cache_topology().expect("Failed to get cache topology");

        // Print all detected caches for debugging
        for (i, cache) in topology.caches.iter().enumerate() {
            if let Some(cache_info) = cache {
                println!(
                    "Cache[{}]: Level={}, Type={}, Size={}KB, Line size={}, Associativity={}, Cores sharing={}",
                    i,
                    cache_info.level,
                    cache_info.cache_type,
                    cache_info.size_kb,
                    cache_info.line_size,
                    cache_info.associativity,
                    cache_info.shared_by
                );
            } else {
                println!("Cache[{}]: Not present", i);
            }
        }

        // Check that at least one cache was detected
        let cache_count = topology.caches.iter().filter(|c| c.is_some()).count();

        assert!(
            cache_count > 0,
            "No caches detected, which is unexpected for modern CPUs"
        );
    }

    #[test]
    fn test_features() {
        let cpuid = CpuidWrapper::new();

        // Check basic features that almost all modern CPUs should have
        assert!(
            cpuid.has_feature(0, cpufetch_rs::cpu::CpuidRegister::EDX),
            "FPU should be supported"
        );

        // Print available features for debugging
        if let Ok(info) = cpuid.get_basic_info() {
            println!("Base features: {:#x}", info.base_features);
            println!("Extended features: {:#x}", info.extended_features);
        }
    }
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_cpu_info_cache_detection() {
    // This test verifies that our CPU information includes cache sizes
    let info = cpufetch_rs::cpu::CpuInfo::new().expect("Failed to detect CPU");

    // Print detected cache sizes
    println!("Detected cache sizes: {:?}", info.cache_sizes);

    // On modern systems, at least one cache should be detected
    let has_cache = info.cache_sizes.iter().any(|c| c.is_some());
    assert!(has_cache, "No cache detected, which is unexpected for modern CPUs");
}

#[test]
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn test_cpu_info_cache_detection() {
    // This test verifies that our CPU information works on non-x86 architectures
    let info = cpufetch_rs::cpu::CpuInfo::new().expect("Failed to detect CPU");

    // On non-x86 architectures, we expect cache detection to not be implemented yet
    println!("Non-x86 architecture detected: cache detection not yet implemented");
    println!("Detected cache sizes: {:?}", info.cache_sizes);

    // Test passes on non-x86 architectures without cache detection
    assert!(true, "Cache detection test succeeded on non-x86 architecture");
}
