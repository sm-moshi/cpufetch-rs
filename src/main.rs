//! cpufetch: A command-line tool for displaying CPU information
//!
//! This binary provides a terminal interface to the cpufetch-rs library,
//! displaying detailed CPU information in a visually appealing format.

use anyhow::Result;
use cpufetch_rs::Error;

/// Main entry point for the cpufetch command-line tool
///
/// This displays CPU information in a nice format
fn main() -> Result<()> {
    // Try to detect CPU information
    let result = run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Main application logic
fn run() -> Result<()> {
    use cpufetch_rs::CpuInfo;

    // Get CPU information
    let cpu_info = CpuInfo::new().map_err(Error::from)?;

    // Basic output when no display features are enabled
    #[cfg(not(feature = "cli"))]
    {
        println!("CPU Information:");
        println!("Vendor: {}", cpu_info.vendor);
        println!("Model: {}", cpu_info.brand_string);
        println!(
            "Cores: {} physical, {} logical",
            cpu_info.physical_cores, cpu_info.logical_cores
        );

        if let Some(l1i) = cpu_info.cache_sizes[0] {
            println!("L1i Cache: {} KB", l1i);
        }
        if let Some(l1d) = cpu_info.cache_sizes[1] {
            println!("L1d Cache: {} KB", l1d);
        }
        if let Some(l2) = cpu_info.cache_sizes[2] {
            println!("L2 Cache: {} KB", l2);
        }
        if let Some(l3) = cpu_info.cache_sizes[3] {
            println!("L3 Cache: {} KB", l3);
        }

        // Display frequency information if available
        if cpu_info.frequency.base.is_some() || cpu_info.frequency.current.is_some() || cpu_info.frequency.max.is_some()
        {
            println!("Frequency: {}", cpu_info.frequency);
        }

        return Ok(());
    }

    // Enhanced CLI with arguments when cli/display features are enabled
    #[cfg(all(feature = "cli", feature = "display"))]
    {
        // Parse command-line arguments
        use cpufetch_rs::cli::Args;

        let args = <Args as clap::Parser>::parse();

        // Display CPU information based on the selected format
        if args.json {
            #[cfg(feature = "json")]
            {
                print_json_output(&cpu_info)?;
            }
            #[cfg(not(feature = "json"))]
            {
                eprintln!("Error: JSON output was requested but the 'json' feature is not enabled");
                eprintln!("Recompile with --feature=json to enable JSON output");
                return Err(anyhow::anyhow!("JSON feature not enabled"));
            }
        } else {
            print_cpu_info_detailed(&cpu_info, &args)?;
        }
    }

    // CLI feature enabled but display feature disabled
    #[cfg(all(feature = "cli", not(feature = "display")))]
    {
        use cpufetch_rs::cli::Args;

        let args = <Args as clap::Parser>::parse();

        // Simple output for CLI without fancy display
        println!("CPU Information:");
        println!("Vendor: {}", cpu_info.vendor);
        println!("Model: {}", cpu_info.brand_string);
        println!(
            "Cores: {} physical, {} logical",
            cpu_info.physical_cores, cpu_info.logical_cores
        );

        // Display cache information if requested
        if args.cache {
            if let Some(l1i) = cpu_info.cache_sizes[0] {
                println!("L1i Cache: {} KB", l1i);
            }
            if let Some(l1d) = cpu_info.cache_sizes[1] {
                println!("L1d Cache: {} KB", l1d);
            }
            if let Some(l2) = cpu_info.cache_sizes[2] {
                println!("L2 Cache: {} KB", l2);
            }
            if let Some(l3) = cpu_info.cache_sizes[3] {
                println!("L3 Cache: {} KB", l3);
            }
        }

        // Display frequency information if available and requested
        if args.frequency {
            if cpu_info.frequency.base.is_some()
                || cpu_info.frequency.current.is_some()
                || cpu_info.frequency.max.is_some()
            {
                println!("Frequency: {}", cpu_info.frequency);
            }
        }

        // Display features if requested
        if args.features {
            println!("CPU Features: {:?}", cpu_info.features);
        }

        // JSON output if requested
        if args.json {
            #[cfg(feature = "json")]
            {
                print_json_output(&cpu_info)?;
            }
            #[cfg(not(feature = "json"))]
            {
                eprintln!("Error: JSON output was requested but the 'json' feature is not enabled");
                eprintln!("Recompile with --feature=json to enable JSON output");
                return Err(anyhow::anyhow!("JSON feature not enabled"));
            }
        }
    }

    Ok(())
}

/// Temporary function to print CPU information in a detailed format
/// This will be moved to the printer module in the future
#[cfg(all(feature = "cli", feature = "display"))]
fn print_cpu_info_detailed(cpu_info: &cpufetch_rs::CpuInfo, args: &cpufetch_rs::cli::Args) -> Result<()> {
    println!("=============== CPU Information ===============");
    println!("Vendor: {}", cpu_info.vendor);
    println!("Model:  {}", cpu_info.brand_string);
    println!("Cores:  {} physical, {} logical", cpu_info.physical_cores, cpu_info.logical_cores);

    // Display cache information if requested
    if args.cache {
        println!("\n=============== Cache Information ===============");
        if let Some(l1i) = cpu_info.cache_sizes[0] {
            println!("L1i Cache: {} KB", l1i);
        }
        if let Some(l1d) = cpu_info.cache_sizes[1] {
            println!("L1d Cache: {} KB", l1d);
        }
        if let Some(l2) = cpu_info.cache_sizes[2] {
            println!("L2 Cache: {} KB", l2);
        }
        if let Some(l3) = cpu_info.cache_sizes[3] {
            println!("L3 Cache: {} KB", l3);
        }
    }

    // Display frequency information if requested
    if args.frequency {
        println!("\n=============== Frequency Information ===============");
        if let Some(base) = cpu_info.frequency.base {
            println!("Base Frequency:    {} MHz", base);
        }
        if let Some(current) = cpu_info.frequency.current {
            println!("Current Frequency: {} MHz", current);
        }
        if let Some(max) = cpu_info.frequency.max {
            println!("Max Frequency:     {} MHz", max);
        }
    }

    // Display features if requested
    if args.features {
        println!("\n=============== CPU Features ===============");

        // Handle x86/x86_64 features
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use cpufetch_rs::cpu::X86Features;

            if cpu_info.features.contains(X86Features::SSE) { println!("- SSE"); }
            if cpu_info.features.contains(X86Features::SSE2) { println!("- SSE2"); }
            if cpu_info.features.contains(X86Features::SSE3) { println!("- SSE3"); }
            if cpu_info.features.contains(X86Features::SSSE3) { println!("- SSSE3"); }
            if cpu_info.features.contains(X86Features::SSE4_1) { println!("- SSE4.1"); }
            if cpu_info.features.contains(X86Features::SSE4_2) { println!("- SSE4.2"); }
            if cpu_info.features.contains(X86Features::AVX) { println!("- AVX"); }
            if cpu_info.features.contains(X86Features::AVX2) { println!("- AVX2"); }
            if cpu_info.features.contains(X86Features::FMA) { println!("- FMA"); }
            if cpu_info.features.contains(X86Features::BMI1) { println!("- BMI1"); }
            if cpu_info.features.contains(X86Features::BMI2) { println!("- BMI2"); }
            if cpu_info.features.contains(X86Features::F16C) { println!("- F16C"); }
            if cpu_info.features.contains(X86Features::POPCNT) { println!("- POPCNT"); }
            if cpu_info.features.contains(X86Features::AES) { println!("- AES"); }
            if cpu_info.features.contains(X86Features::AVX512F) { println!("- AVX512F"); }
            if cpu_info.features.contains(X86Features::AVX512BW) { println!("- AVX512BW"); }
            if cpu_info.features.contains(X86Features::AVX512CD) { println!("- AVX512CD"); }
            if cpu_info.features.contains(X86Features::AVX512DQ) { println!("- AVX512DQ"); }
            if cpu_info.features.contains(X86Features::AVX512VL) { println!("- AVX512VL"); }
        }

        // Handle ARM/aarch64 features
        #[cfg(target_arch = "aarch64")]
        {
            use cpufetch_rs::cpu::ArmFeatures;

            if cpu_info.features.contains(ArmFeatures::NEON) { println!("- NEON"); }
            if cpu_info.features.contains(ArmFeatures::AES) { println!("- AES"); }
            if cpu_info.features.contains(ArmFeatures::PMULL) { println!("- PMULL"); }
            if cpu_info.features.contains(ArmFeatures::SHA1) { println!("- SHA1"); }
            if cpu_info.features.contains(ArmFeatures::SHA2) { println!("- SHA2"); }
            if cpu_info.features.contains(ArmFeatures::CRC32) { println!("- CRC32"); }
            if cpu_info.features.contains(ArmFeatures::ATOMICS) { println!("- ATOMICS"); }
            if cpu_info.features.contains(ArmFeatures::FP) { println!("- FP"); }
            if cpu_info.features.contains(ArmFeatures::ASIMD) { println!("- ASIMD"); }
            if cpu_info.features.contains(ArmFeatures::FPHP) { println!("- FPHP"); }
            if cpu_info.features.contains(ArmFeatures::ASIMDHP) { println!("- ASIMDHP"); }
            if cpu_info.features.contains(ArmFeatures::ASIMDDP) { println!("- ASIMDDP"); }
            if cpu_info.features.contains(ArmFeatures::ASIMDFHM) { println!("- ASIMDFHM"); }
        }
    }

    Ok(())
}

/// Temporary function to print CPU information in JSON format
/// This will be moved to the printer module in the future
#[cfg(feature = "json")]
fn print_json_output(cpu_info: &cpufetch_rs::CpuInfo) -> Result<()> {
    use serde_json::json;

    let json_output = json!({
        "vendor": cpu_info.vendor,
        "model": cpu_info.brand_string,
        "cores": {
            "physical": cpu_info.physical_cores,
            "logical": cpu_info.logical_cores
        },
        "cache": {
            "l1i": cpu_info.cache_sizes[0],
            "l1d": cpu_info.cache_sizes[1],
            "l2": cpu_info.cache_sizes[2],
            "l3": cpu_info.cache_sizes[3]
        },
        "frequency": {
            "base": cpu_info.frequency.base,
            "current": cpu_info.frequency.current,
            "max": cpu_info.frequency.max
        },
        "features": cpu_info.features
    });

    println!("{}", serde_json::to_string_pretty(&json_output)?);

    Ok(())
}
