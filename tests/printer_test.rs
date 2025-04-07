//! Tests for the printer module functionality
//!
//! These tests verify that the printer module correctly formats and displays
//! CPU information with different feature combinations.

#[cfg(feature = "display")]
mod printer_tests {
    use cpufetch_rs::cpu::{CpuInfo, Vendor, Version};

    #[cfg(feature = "cli")]
    use cpufetch_rs::cli::Args;

    #[cfg(feature = "display")]
    use cpufetch_rs::printer;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    use cpufetch_rs::cpu::X86Features;

    #[cfg(target_arch = "aarch64")]
    use cpufetch_rs::cpu::ArmFeatures;

    /// Create a mock CPU info structure for testing
    fn create_mock_cpu_info() -> CpuInfo {
        // Create default frequency with some values
        let frequency = cpufetch_rs::cpu::info::Frequency {
            base: Some(2800.0),
            current: Some(3200.0),
            max: Some(4000.0),
        };

        // Create default version
        let version = Version {
            family: 10,
            model: 5,
            stepping: 2,
        };

        // Create mock cache sizes [L1i, L1d, L2, L3]
        let cache_sizes = [Some(32), Some(32), Some(256), Some(8192)];

        // Create CPU features based on architecture
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let features = {
            let mut f = X86Features::empty();
            f.insert(X86Features::SSE);
            f.insert(X86Features::SSE2);
            f.insert(X86Features::AVX);
            f
        };

        #[cfg(target_arch = "aarch64")]
        let features = {
            let mut f = ArmFeatures::empty();
            f.insert(ArmFeatures::NEON);
            f.insert(ArmFeatures::FP);
            f
        };

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
        let features = ();

        // Create mock CPU info with test values
        CpuInfo {
            vendor: Vendor::Intel,
            brand_string: "Mock Intel CPU @ 2.8GHz".to_string(),
            version,
            physical_cores: 4,
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
    fn test_print_cpu_info_basic() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();
        let args = create_mock_args();

        // Test basic printing
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_print_cpu_info_no_logo() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();
        let mut args = create_mock_args();
        args.no_logo = true;

        // Test without logo
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info with no_logo failed: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_print_cpu_info_no_color() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();
        let mut args = create_mock_args();
        args.no_color = true;

        // Test without color
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info with no_color failed: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_print_cpu_info_selective_info() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();
        let mut args = create_mock_args();

        // Test with only frequency information
        args.frequency = true;
        args.cache = false;
        args.features = false;
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info with only frequency failed: {:?}", result.err());

        // Test with only cache information
        args.frequency = false;
        args.cache = true;
        args.features = false;
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info with only cache failed: {:?}", result.err());

        // Test with only feature information
        args.frequency = false;
        args.cache = false;
        args.features = true;
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info with only features failed: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli", feature = "json"))]
    fn test_print_json() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();

        // Test JSON printing
        let result = printer::print_json(&cpu_info);
        assert!(result.is_ok(), "print_json failed: {:?}", result.err());
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli", not(feature = "json")))]
    fn test_print_json_feature_disabled() {
        // Create mock data
        let cpu_info = create_mock_cpu_info();

        // Test JSON printing when the feature is disabled
        // This should return an error
        let result = printer::print_json(&cpu_info);
        assert!(result.is_err(), "print_json should have failed when json feature is disabled");
    }

    #[test]
    #[cfg(all(feature = "display", feature = "cli"))]
    fn test_different_cpu_vendors() {
        let mut cpu_info = create_mock_cpu_info();
        let args = create_mock_args();

        // Test Intel
        cpu_info.vendor = Vendor::Intel;
        cpu_info.brand_string = "Mock Intel CPU @ 3.6GHz".to_string();
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for Intel: {:?}", result.err());

        // Test AMD
        cpu_info.vendor = Vendor::AMD;
        cpu_info.brand_string = "Mock AMD CPU @ 3.4GHz".to_string();
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for AMD: {:?}", result.err());

        // Test ARM
        cpu_info.vendor = Vendor::ARM;
        cpu_info.brand_string = "Mock ARM CPU @ 2.0GHz".to_string();
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for ARM: {:?}", result.err());

        // Test Apple
        cpu_info.vendor = Vendor::Apple;
        cpu_info.brand_string = "Mock Apple M1 CPU @ 3.2GHz".to_string();
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for Apple: {:?}", result.err());

        // Test Unknown
        cpu_info.vendor = Vendor::Unknown;
        cpu_info.brand_string = "Mock Unknown CPU".to_string();
        let result = printer::print_cpu_info(&cpu_info, &args);
        assert!(result.is_ok(), "print_cpu_info failed for Unknown: {:?}", result.err());
    }
}

// Basic integration tests for the whole command
#[cfg(all(feature = "cli", feature = "display"))]
mod cli_integration_tests {
    use std::process::Command;

    // Helper to run the cpufetch command with arguments
    fn run_command(args: &[&str]) -> std::io::Result<std::process::Output> {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_cpufetch"));
        cmd.args(args);
        cmd.output()
    }

    #[test]
    fn test_basic_command() {
        let output = run_command(&[]).expect("Failed to run basic command");
        assert!(output.status.success(), "Command failed with status: {}", output.status);

        // Basic check that output contains expected CPU information
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("CPU Information"), "Output doesn't contain 'CPU Information'");
        assert!(stdout.contains("Vendor:"), "Output doesn't contain 'Vendor'");
        assert!(stdout.contains("Model:"), "Output doesn't contain 'Model'");
        assert!(stdout.contains("Cores:"), "Output doesn't contain 'Cores'");
    }

    #[test]
    fn test_no_logo_option() {
        let output = run_command(&["--no-logo"]).expect("Failed to run --no-logo command");
        assert!(output.status.success(), "Command failed with status: {}", output.status);

        // Check that output does not contain ASCII art (crude check)
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.contains("======="), "Output still contains logo elements");
    }

    #[test]
    fn test_no_color_option() {
        let output = run_command(&["--no-color"]).expect("Failed to run --no-color command");
        assert!(output.status.success(), "Command failed with status: {}", output.status);

        // Can't easily check for color codes automatically, but command should run
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("CPU Information"), "Output doesn't contain 'CPU Information'");
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_option() {
        let output = run_command(&["--json"]).expect("Failed to run --json command");
        assert!(output.status.success(), "Command failed with status: {}", output.status);

        // Check that output is valid JSON
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"), "Output doesn't contain JSON opening brace");
        assert!(stdout.contains("}"), "Output doesn't contain JSON closing brace");
        assert!(stdout.contains("\"vendor\":"), "Output doesn't contain vendor JSON field");
    }
}
