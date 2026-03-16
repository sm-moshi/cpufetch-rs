//! Printer module for displaying CPU information
//!
//! This module handles the formatting and display of CPU information in different formats.
//! It supports both terminal output with ASCII art and JSON serialization.

#[cfg(feature = "display")]
mod ascii;
#[cfg(feature = "display")]
mod layout;
#[cfg(feature = "display")]
mod logo;

#[cfg(feature = "display")]
use colored::Colorize;
#[cfg(feature = "display")]
use crate::cpu::CpuInfo;
#[cfg(feature = "display")]
use crate::cli::Args;

/// Print CPU information in a formatted display with optional ASCII art
#[cfg(feature = "display")]
pub fn print_cpu_info(cpu_info: &CpuInfo, args: &Args) -> anyhow::Result<()> {
    // Set up display (handle no-color option)
    layout::setup_display(args.no_color);

    if !args.no_logo {
        // Print logo based on CPU vendor
        println!("{}", logo::get_logo(&cpu_info.vendor));
    }

    // Print basic CPU information
    println!("{}", "CPU Information:".bold());
    println!("Vendor: {}", cpu_info.vendor.to_string().green());
    println!("Model:  {}", cpu_info.brand_string.green());
    println!(
        "Cores:  {} physical, {} logical",
        cpu_info.physical_cores.to_string().green(),
        cpu_info.logical_cores.to_string().green()
    );

    // Display cache information if requested
    if args.cache {
        println!("\n{}", "Cache Information:".bold());
        if let Some(l1i) = cpu_info.cache_sizes[0] {
            println!("L1i Cache: {} KB", l1i.to_string().green());
        }
        if let Some(l1d) = cpu_info.cache_sizes[1] {
            println!("L1d Cache: {} KB", l1d.to_string().green());
        }
        if let Some(l2) = cpu_info.cache_sizes[2] {
            println!("L2 Cache: {} KB", l2.to_string().green());
        }
        if let Some(l3) = cpu_info.cache_sizes[3] {
            println!("L3 Cache: {} KB", l3.to_string().green());
        }
    }

    // Display frequency information if requested
    if args.frequency {
        println!("\n{}", "Frequency Information:".bold());
        if let Some(base) = cpu_info.frequency.base {
            println!("Base Frequency:    {} MHz", base.to_string().green());
        }
        if let Some(current) = cpu_info.frequency.current {
            println!("Current Frequency: {} MHz", current.to_string().green());
        }
        if let Some(max) = cpu_info.frequency.max {
            println!("Max Frequency:     {} MHz", max.to_string().green());
        }
    }

    // Display features if requested
    if args.features {
        println!("\n{}", "CPU Features:".bold());

        // Handle x86/x86_64 features
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use crate::cpu::X86Features;

            if cpu_info.features.contains(X86Features::SSE) { println!("- {}", "SSE".green()); }
            if cpu_info.features.contains(X86Features::SSE2) { println!("- {}", "SSE2".green()); }
            if cpu_info.features.contains(X86Features::SSE3) { println!("- {}", "SSE3".green()); }
            if cpu_info.features.contains(X86Features::SSSE3) { println!("- {}", "SSSE3".green()); }
            if cpu_info.features.contains(X86Features::SSE4_1) { println!("- {}", "SSE4.1".green()); }
            if cpu_info.features.contains(X86Features::SSE4_2) { println!("- {}", "SSE4.2".green()); }
            if cpu_info.features.contains(X86Features::AVX) { println!("- {}", "AVX".green()); }
            if cpu_info.features.contains(X86Features::AVX2) { println!("- {}", "AVX2".green()); }
            if cpu_info.features.contains(X86Features::FMA) { println!("- {}", "FMA".green()); }
            if cpu_info.features.contains(X86Features::BMI1) { println!("- {}", "BMI1".green()); }
            if cpu_info.features.contains(X86Features::BMI2) { println!("- {}", "BMI2".green()); }
            if cpu_info.features.contains(X86Features::F16C) { println!("- {}", "F16C".green()); }
            if cpu_info.features.contains(X86Features::POPCNT) { println!("- {}", "POPCNT".green()); }
            if cpu_info.features.contains(X86Features::AES) { println!("- {}", "AES".green()); }
            if cpu_info.features.contains(X86Features::AVX512F) { println!("- {}", "AVX512F".green()); }
            if cpu_info.features.contains(X86Features::AVX512BW) { println!("- {}", "AVX512BW".green()); }
            if cpu_info.features.contains(X86Features::AVX512CD) { println!("- {}", "AVX512CD".green()); }
            if cpu_info.features.contains(X86Features::AVX512DQ) { println!("- {}", "AVX512DQ".green()); }
            if cpu_info.features.contains(X86Features::AVX512VL) { println!("- {}", "AVX512VL".green()); }
        }

        // Handle ARM/aarch64 features
        #[cfg(target_arch = "aarch64")]
        {
            use crate::cpu::ArmFeatures;

            if cpu_info.features.contains(ArmFeatures::NEON) { println!("- {}", "NEON".green()); }
            if cpu_info.features.contains(ArmFeatures::AES) { println!("- {}", "AES".green()); }
            if cpu_info.features.contains(ArmFeatures::PMULL) { println!("- {}", "PMULL".green()); }
            if cpu_info.features.contains(ArmFeatures::SHA1) { println!("- {}", "SHA1".green()); }
            if cpu_info.features.contains(ArmFeatures::SHA2) { println!("- {}", "SHA2".green()); }
            if cpu_info.features.contains(ArmFeatures::CRC32) { println!("- {}", "CRC32".green()); }
            if cpu_info.features.contains(ArmFeatures::ATOMICS) { println!("- {}", "ATOMICS".green()); }
            if cpu_info.features.contains(ArmFeatures::FP) { println!("- {}", "FP".green()); }
            if cpu_info.features.contains(ArmFeatures::ASIMD) { println!("- {}", "ASIMD".green()); }
            if cpu_info.features.contains(ArmFeatures::FPHP) { println!("- {}", "FPHP".green()); }
            if cpu_info.features.contains(ArmFeatures::ASIMDHP) { println!("- {}", "ASIMDHP".green()); }
            if cpu_info.features.contains(ArmFeatures::ASIMDDP) { println!("- {}", "ASIMDDP".green()); }
            if cpu_info.features.contains(ArmFeatures::ASIMDFHM) { println!("- {}", "ASIMDFHM".green()); }
        }
    }

    Ok(())
}

/// Print CPU information in JSON format
#[cfg(all(feature = "display", feature = "json"))]
pub fn print_json(cpu_info: &CpuInfo) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(cpu_info)?;
    println!("{}", json);
    Ok(())
}

/// Print CPU information in JSON format (no-op when json feature is disabled)
#[cfg(all(feature = "display", not(feature = "json")))]
pub fn print_json(_cpu_info: &CpuInfo) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("JSON feature not enabled"))
}
