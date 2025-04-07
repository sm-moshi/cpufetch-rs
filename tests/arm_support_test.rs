//! Tests for ARM architecture support in cpufetch-rs
//!
//! These tests verify the ARM-specific functionality, particularly
//! cache detection and feature detection.

#[cfg(target_arch = "aarch64")]
mod arm_tests {
    use cpufetch_rs::cpu::{ArmFeatures, CpuInfo, Vendor, Version};

    #[cfg(feature = "cli")]
    use cpufetch_rs::cli::Args;

    #[cfg(feature = "display")]
    use cpufetch_rs::printer;

    /// Create a mock ARM CPU for testing
    fn create_mock_arm_cpu() -> CpuInfo {
        // Create default frequency with some values
        let frequency = cpufetch_rs::cpu::info::Frequency {
            base: Some(2500.0),
            current: Some(2700.0),
            max: Some(3200.0),
        };

        // Create default version - not as relevant for ARM
        let version = Version {
            family: 0,
            model: 0,
            stepping: 0,
        };

        // Create mock cache sizes [L1i, L1d, L2, L3]
        let cache_sizes = [Some(64), Some(64), Some(512), Some(4096)];

        // Create ARM features
        let mut features = ArmFeatures::empty();
        features.insert(ArmFeatures::NEON);
        features.insert(ArmFeatures::FP);
        features.insert(ArmFeatures::ASIMD);

        // Create mock CPU info with ARM values
        CpuInfo {
            vendor: Vendor::ARM,
            brand_string: "ARMv8 Processor @ 2.5GHz".to_string(),
            version,
            physical_cores: 4,
            logical_cores: 4, // ARM often has same physical/logical core count
            frequency,
            cache_sizes,
            features,
        }
    }

    /// Create a mock Apple Silicon CPU for testing
    fn create_mock_apple_silicon() -> CpuInfo {
        // Create default frequency with some values
        let frequency = cpufetch_rs::cpu::info::Frequency {
            base: Some(3200.0),
            current: Some(3200.0),
            max: Some(3200.0),
        };

        // Create default version - not as relevant for ARM
        let version = Version {
            family: 0,
            model: 0,
            stepping: 0,
        };

        // Create mock cache sizes [L1i, L1d, L2, L3]
        // Apple Silicon typically has larger caches
        let cache_sizes = [Some(192), Some(128), Some(4096), Some(12288)];

        // Create ARM features for Apple Silicon
        let mut features = ArmFeatures::empty();
        features.insert(ArmFeatures::NEON);
        features.insert(ArmFeatures::FP);
        features.insert(ArmFeatures::ASIMD);
        // Add more Apple-specific features as identified

        // Create mock CPU info with Apple values
        CpuInfo {
            vendor: Vendor::Apple,
            brand_string: "Apple M1 Pro".to_string(),
            version,
            physical_cores: 8,
            logical_cores: 8,
            frequency,
            cache_sizes,
            features,
        }
    }

    #[cfg(feature = "cli")]
    fn create_mock_args() -> Args {
        Args {
            frequency: true,
            cache: true,
            features: true,
            json: false,
            no_logo: false,
            no_color: false,
            debug: false,
        }
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_arm_cpu_display() {
        // Create mock ARM data
        let cpu_info = create_mock_arm_cpu();
        let args = create_mock_args();

        // Test ARM printing
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for ARM: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_apple_silicon_display() {
        // Create mock Apple Silicon data
        let cpu_info = create_mock_apple_silicon();
        let args = create_mock_args();

        // Test Apple Silicon printing
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for Apple Silicon: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli", feature = "json"))]
    fn test_arm_json_output() {
        // Create mock ARM data
        let cpu_info = create_mock_arm_cpu();

        // Test ARM JSON output
        let result = printer::print_json(&cpu_info);
        assert!(result.is_ok(), "print_json failed for ARM: {:?}", result.err());

        // Create mock Apple Silicon data
        let cpu_info = create_mock_apple_silicon();

        // Test Apple Silicon JSON output
        let result = printer::print_json(&cpu_info);
        assert!(result.is_ok(), "print_json failed for Apple Silicon: {:?}", result.err());
    }

    #[test]
    fn test_arm_feature_detection() {
        // Test that our feature detection for ARM works correctly
        // This is more of a unit test for the ARM features

        let features = ArmFeatures::empty();
        assert!(!features.contains(ArmFeatures::NEON), "Empty features should not contain NEON");

        let mut features = ArmFeatures::empty();
        features.insert(ArmFeatures::NEON);
        assert!(features.contains(ArmFeatures::NEON), "Features should contain NEON after insertion");
        assert!(!features.contains(ArmFeatures::ASIMD), "Features should not contain ASIMD");

        features.insert(ArmFeatures::ASIMD);
        assert!(features.contains(ArmFeatures::ASIMD), "Features should contain ASIMD after insertion");
    }

    // This test would require actual hardware or mocking of system calls
    // Just a placeholder for future implementation
    #[test]
    #[ignore]
    fn test_real_aarch64_detection() {
        // This test would run the actual detection code
        // It's marked as ignored since it requires real hardware
        // Implement this test when the detection code is more mature
    }
}

// Stub for non-aarch64 platforms to avoid test failures
#[cfg(not(target_arch = "aarch64"))]
mod arm_tests {
    #[test]
    fn test_arm_support_placeholder() {
        // This test exists just to avoid "no tests" warnings on non-ARM platforms
        println!("ARM tests are only available on aarch64 platforms");
    }
}
