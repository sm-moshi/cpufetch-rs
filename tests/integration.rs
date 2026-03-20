//! End-to-end integration tests for cpufetch-rs.
//!
//! These tests exercise the full detection → render pipeline without mocking
//! any platform abstractions.

use cpufetch_rs::cpu::CpuInfo;

// ── Full pipeline ────────────────────────────────────────────────────────────

/// Detect a real CPU, build the info struct, and render it without panicking.
#[test]
fn test_full_pipeline_no_panic() {
    let cpu_info = CpuInfo::new().expect("CpuInfo::new() should succeed on a real machine");

    // Basic sanity — every machine should have at least one core
    assert!(cpu_info.physical_cores > 0, "physical_cores must be > 0");
    assert!(cpu_info.logical_cores > 0, "logical_cores must be > 0");
    assert!(!cpu_info.brand_string.is_empty(), "brand_string must not be empty");

    // Render to string (display feature)
    #[cfg(feature = "display")]
    {
        use cpufetch_rs::printer::print_cpu_info;
        use cpufetch_rs::cli::Args;

        let args = Args {
            frequency: false,
            cache: true,
            features: false,
            json: false,
            no_logo: true, // avoid ASCII art in test output
            no_color: true,
            debug: false,
        };

        // Should not panic
        print_cpu_info(&cpu_info, &args).expect("print_cpu_info should not fail");
    }
}

/// Detect using the lazy-static singleton accessor.
#[test]
fn test_cpu_info_get_singleton() {
    let a = CpuInfo::get();
    let b = CpuInfo::get();
    // Must be the same allocation (pointer equality)
    assert!(std::ptr::eq(a, b), "CpuInfo::get() must return the same instance");
}

// ── JSON round-trip ──────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "json")]
fn test_json_roundtrip() {
    let cpu_info = CpuInfo::new().expect("CpuInfo::new() should succeed");
    let json = serde_json::to_string_pretty(&cpu_info).expect("serialisation must not fail");
    assert!(!json.is_empty());

    let restored: CpuInfo = serde_json::from_str(&json).expect("deserialisation must not fail");

    assert_eq!(cpu_info.vendor, restored.vendor);
    assert_eq!(cpu_info.brand_string, restored.brand_string);
    assert_eq!(cpu_info.physical_cores, restored.physical_cores);
    assert_eq!(cpu_info.logical_cores, restored.logical_cores);
    assert_eq!(cpu_info.cache_sizes, restored.cache_sizes);
}

// ── CLI integration ──────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "cli")]
fn test_cli_help_exits_zero() {
    use assert_cmd::Command;
    Command::cargo_bin("cpufetch")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_version_exits_zero() {
    use assert_cmd::Command;
    Command::cargo_bin("cpufetch")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_json_output_is_valid() {
    use assert_cmd::Command;
    use predicates::str::contains;

    Command::cargo_bin("cpufetch")
        .unwrap()
        .args(["--json", "--no-color"])
        .assert()
        .success()
        .stdout(contains("vendor"))
        .stdout(contains("brand_string"));
}

// ── Architecture-specific ────────────────────────────────────────────────────

/// On x86_64, after the frequency wire-up, at least one frequency field
/// should be populated on Linux and macOS (Windows WMI is environment-dependent).
#[test]
#[cfg(all(target_arch = "x86_64", any(target_os = "linux", target_os = "macos"), feature = "frequency"))]
fn test_frequency_populated_on_x86_64() {
    let cpu = CpuInfo::new().expect("CpuInfo::new() should succeed");
    assert!(
        cpu.frequency.base.is_some() || cpu.frequency.max.is_some() || cpu.frequency.current.is_some(),
        "At least one frequency field must be populated on Linux/macOS x86_64"
    );
}

/// On x86_64, microarchitecture should be detected for Intel and AMD CPUs.
#[test]
#[cfg(target_arch = "x86_64")]
fn test_microarch_detected_on_x86_64() {
    use cpufetch_rs::cpu::Vendor;
    let cpu = CpuInfo::new().expect("CpuInfo::new() should succeed");
    match cpu.vendor {
        Vendor::Intel | Vendor::AMD => {
            assert!(
                cpu.microarch.is_some(),
                "Microarchitecture should be detected for Intel/AMD CPUs (family={}, model={})",
                cpu.version.family,
                cpu.version.model
            );
        },
        _ => {
            // ARM, Apple, Unknown — microarch detection is not yet implemented
        },
    }
}

/// Peak FLOP/s should be positive when frequency is available on x86_64.
#[test]
#[cfg(all(target_arch = "x86_64", feature = "frequency"))]
fn test_peak_flops_positive_when_freq_available() {
    let cpu = CpuInfo::new().expect("CpuInfo::new() should succeed");
    let has_freq = cpu.frequency.max.is_some() || cpu.frequency.base.is_some();
    if has_freq {
        assert!(
            cpu.peak_flops.map(|f| f > 0.0).unwrap_or(false),
            "peak_flops should be positive when frequency is available"
        );
    }
}
