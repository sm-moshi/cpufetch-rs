//! Printer module for displaying CPU information
//!
//! Handles the formatting and display of CPU information in different formats.
//! Supports both terminal output with ASCII art logo (side-by-side layout) and
//! JSON serialisation.

#[cfg(feature = "display")]
mod ascii;
#[cfg(feature = "display")]
mod layout;
#[cfg(feature = "display")]
mod logo;

#[cfg(feature = "display")]
use crate::cli::Args;
#[cfg(feature = "display")]
use crate::cpu::CpuInfo;
#[cfg(feature = "display")]
use colored::Colorize;

/// Width of the label column (including the trailing colon).
/// "Microarchitecture:" is 18 chars — use 20 for a comfortable margin.
#[cfg(feature = "display")]
const LABEL_WIDTH: usize = 20;

/// Spacing between the logo frame and the info block.
#[cfg(feature = "display")]
const LOGO_INFO_GAP: usize = 3;

/// Print CPU information with an optional ASCII art logo in a side-by-side layout.
#[cfg(feature = "display")]
pub fn print_cpu_info(cpu_info: &CpuInfo, args: &Args) -> anyhow::Result<()> {
    layout::setup_display(args.no_color);

    // ── Build info lines ────────────────────────────────────────────────────
    let mut info_lines: Vec<String> = Vec::new();

    // Always-visible core information
    info_lines.push(layout::format_kv(
        "Vendor",
        &cpu_info.vendor.to_string().green().bold().to_string(),
        LABEL_WIDTH,
    ));
    info_lines.push(layout::format_kv(
        "Model",
        &cpu_info.brand_string.green().to_string(),
        LABEL_WIDTH,
    ));

    if let Some(ref uarch) = cpu_info.microarch {
        info_lines.push(layout::format_kv(
            "Microarchitecture",
            &uarch.to_string().green().to_string(),
            LABEL_WIDTH,
        ));
        if let Some(nm) = uarch.process_nm() {
            info_lines.push(layout::format_kv(
                "Technology",
                &format!("{} nm", nm).green().to_string(),
                LABEL_WIDTH,
            ));
        }
    }

    if let Some(ref hv) = cpu_info.hypervisor {
        info_lines.push(layout::format_kv(
            "Hypervisor",
            &hv.yellow().to_string(),
            LABEL_WIDTH,
        ));
    }

    info_lines.push(layout::format_kv(
        "Cores",
        &format!("{} physical, {} logical", cpu_info.physical_cores, cpu_info.logical_cores)
            .green()
            .to_string(),
        LABEL_WIDTH,
    ));

    // ── Frequency ───────────────────────────────────────────────────────────
    let has_freq = cpu_info.frequency.base.is_some()
        || cpu_info.frequency.max.is_some()
        || cpu_info.frequency.current.is_some();

    if args.frequency || has_freq {
        if let Some(base) = cpu_info.frequency.base {
            info_lines.push(layout::format_kv(
                "Base Frequency",
                &format!("{:.0} MHz", base).green().to_string(),
                LABEL_WIDTH,
            ));
        }
        if let Some(max) = cpu_info.frequency.max {
            info_lines.push(layout::format_kv(
                "Max Frequency",
                &format!("{:.0} MHz", max).green().to_string(),
                LABEL_WIDTH,
            ));
        }
        if let Some(cur) = cpu_info.frequency.current {
            info_lines.push(layout::format_kv(
                "Current Frequency",
                &format!("{:.0} MHz", cur).green().to_string(),
                LABEL_WIDTH,
            ));
        }
    }

    // ── Cache ───────────────────────────────────────────────────────────────
    if args.cache {
        let cache_labels = ["L1i Cache", "L1d Cache", "L2 Cache", "L3 Cache"];
        for (label, size) in cache_labels.iter().zip(cpu_info.cache_sizes.iter()) {
            if let Some(kb) = size {
                info_lines.push(layout::format_kv(
                    label,
                    &format!("{} KB", kb).green().to_string(),
                    LABEL_WIDTH,
                ));
            }
        }
    }

    // ── Peak performance ────────────────────────────────────────────────────
    if let Some(flops) = cpu_info.peak_flops {
        if flops > 0.0 {
            info_lines.push(layout::format_kv(
                "Peak Performance",
                &format!("{:.2} GFLOP/s", flops).green().to_string(),
                LABEL_WIDTH,
            ));
        }
    }

    // ── CPU features ────────────────────────────────────────────────────────
    if args.features {
        info_lines.push(String::new()); // blank separator

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use crate::cpu::X86Features;
            let flag_names: &[(&str, X86Features)] = &[
                ("SSE", X86Features::SSE),
                ("SSE2", X86Features::SSE2),
                ("SSE3", X86Features::SSE3),
                ("SSSE3", X86Features::SSSE3),
                ("SSE4.1", X86Features::SSE4_1),
                ("SSE4.2", X86Features::SSE4_2),
                ("AVX", X86Features::AVX),
                ("AVX2", X86Features::AVX2),
                ("AVX-512F", X86Features::AVX512F),
                ("FMA", X86Features::FMA),
                ("AES", X86Features::AES),
                ("BMI1", X86Features::BMI1),
                ("BMI2", X86Features::BMI2),
                ("F16C", X86Features::F16C),
                ("POPCNT", X86Features::POPCNT),
            ];
            let active: Vec<&str> = flag_names
                .iter()
                .filter(|(_, flag)| cpu_info.features.contains(*flag))
                .map(|(name, _)| *name)
                .collect();
            if !active.is_empty() {
                info_lines.push(format!("  {}", active.join("  ").green()));
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            use crate::cpu::ArmFeatures;
            let flag_names: &[(&str, ArmFeatures)] = &[
                ("NEON", ArmFeatures::NEON),
                ("AES", ArmFeatures::AES),
                ("PMULL", ArmFeatures::PMULL),
                ("SHA1", ArmFeatures::SHA1),
                ("SHA2", ArmFeatures::SHA2),
                ("CRC32", ArmFeatures::CRC32),
                ("ATOMICS", ArmFeatures::ATOMICS),
                ("FP", ArmFeatures::FP),
                ("ASIMD", ArmFeatures::ASIMD),
            ];
            let active: Vec<&str> = flag_names
                .iter()
                .filter(|(_, flag)| cpu_info.features.contains(*flag))
                .map(|(name, _)| *name)
                .collect();
            if !active.is_empty() {
                info_lines.push(format!("  {}", active.join("  ").green()));
            }
        }
    }

    // ── Render ──────────────────────────────────────────────────────────────
    if args.no_logo {
        for line in &info_lines {
            println!("{}", line);
        }
        return Ok(());
    }

    // Build the framed logo (uncoloured) to determine visual width
    let raw_logo = logo::get_raw_logo(&cpu_info.vendor);
    let raw_framed = ascii::frame(raw_logo, 1);

    // Visual width = chars on the first frame line (all lines are equal-width)
    let logo_visual_width = raw_framed.lines().next().map(|l| l.chars().count()).unwrap_or(0);

    // Colourize each frame line individually to avoid ANSI code fragmentation
    let color_name = logo::get_logo_color_name(&cpu_info.vendor);
    let logo_lines: Vec<String> = raw_framed.lines().map(|l| logo::colorize_line(l, color_name)).collect();

    // Print side-by-side: logo on the left, info on the right
    let max_rows = logo_lines.len().max(info_lines.len());
    let blank_left = " ".repeat(logo_visual_width);

    for i in 0..max_rows {
        let right = info_lines.get(i).map(String::as_str).unwrap_or("");
        if let Some(left) = logo_lines.get(i) {
            println!("{}{}{}", left, " ".repeat(LOGO_INFO_GAP), right);
        } else {
            println!("{}{}{}", blank_left, " ".repeat(LOGO_INFO_GAP), right);
        }
    }

    Ok(())
}

/// Print CPU information in JSON format.
#[cfg(all(feature = "display", feature = "json"))]
pub fn print_json(cpu_info: &CpuInfo) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(cpu_info)?;
    println!("{}", json);
    Ok(())
}

/// No-op stub when the json feature is disabled.
#[cfg(all(feature = "display", not(feature = "json")))]
pub fn print_json(_cpu_info: &CpuInfo) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("JSON feature not enabled"))
}
