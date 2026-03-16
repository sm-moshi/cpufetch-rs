//! Integration tests for CPU detection
//!
//! These tests verify that CPU detection works correctly across
//! supported architectures.

use cpufetch_rs::cpu::CpuInfo;

#[test]
fn test_cpu_info_new() {
    // This should work on all supported architectures
    let info = CpuInfo::new().expect("Failed to detect CPU");

    // Basic validations that should pass on any CPU
    assert!(!info.brand_string.is_empty());
    assert!(info.physical_cores > 0);
    assert!(info.logical_cores > 0);

    // Print CPU info for debugging
    println!("Detected CPU: {:?}", info);
}

#[test]
fn test_cpu_info_static() {
    // Get the static CPU info
    let info = CpuInfo::get();

    // Basic validations
    assert!(!info.brand_string.is_empty());
    assert!(info.physical_cores > 0);
    assert!(info.logical_cores > 0);

    // Print CPU info for debugging
    println!("Static CPU info: {:?}", info);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn test_x86_64_specific() {
    let info = CpuInfo::new().expect("Failed to detect CPU");

    // CPU should always have some basic features on x86_64
    assert!(!info.features.is_empty());

    // Either Intel or AMD (most likely), but could be other vendors too
    println!("x86_64 CPU vendor: {}", info.vendor);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_aarch64_specific() {
    let info = CpuInfo::new().expect("Failed to detect CPU");

    // CPU should have NEON on most modern ARM chips
    // This is not guaranteed on all ARM chips, so we don't assert it
    println!("ARM features: {:?}", info.features);
    println!("ARM CPU vendor: {}", info.vendor);
}
