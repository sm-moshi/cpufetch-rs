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
                cpufetch_rs::printer::print_json(&cpu_info)?;
            }
            #[cfg(not(feature = "json"))]
            {
                eprintln!("Error: JSON output was requested but the 'json' feature is not enabled");
                eprintln!("Recompile with --feature=json to enable JSON output");
                return Err(anyhow::anyhow!("JSON feature not enabled"));
            }
        } else {
            cpufetch_rs::printer::print_cpu_info(&cpu_info, &args)?;
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
                cpufetch_rs::printer::print_json(&cpu_info)?;
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
