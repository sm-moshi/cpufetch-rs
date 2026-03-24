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
///
/// # Errors
///
/// Returns an error if writing to stdout fails.
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
                &format!("{nm} nm").green().to_string(),
                LABEL_WIDTH,
            ));
        }
    }

    if let Some(ref hv) = cpu_info.hypervisor {
        info_lines.push(layout::format_kv("Hypervisor", &hv.yellow().to_string(), LABEL_WIDTH));
    }

    // ── Cores (with P/E breakdown for hybrid CPUs) ─────────────────────────
    let cores_str = match (cpu_info.p_cores, cpu_info.e_cores) {
        (Some(p), Some(e)) if p > 0 && e > 0 => {
            format!(
                "{}P + {}E ({} total), {} logical",
                p, e, cpu_info.physical_cores, cpu_info.logical_cores
            )
        },
        _ => format!(
            "{} physical, {} logical",
            cpu_info.physical_cores, cpu_info.logical_cores
        ),
    };
    info_lines.push(layout::format_kv("Cores", &cores_str.green().to_string(), LABEL_WIDTH));

    // ── Frequency (always shown when data is available) ─────────────────────
    if let Some(max) = cpu_info.frequency.max {
        let label = if cpu_info.frequency.base.is_some() {
            "Max Frequency"
        } else {
            "Frequency"
        };
        info_lines.push(layout::format_kv(
            label,
            &format!("{:.3} GHz", max / 1000.0).green().to_string(),
            LABEL_WIDTH,
        ));
    }
    if let Some(base) = cpu_info.frequency.base {
        info_lines.push(layout::format_kv(
            "Base Frequency",
            &format!("{:.3} GHz", base / 1000.0).green().to_string(),
            LABEL_WIDTH,
        ));
    }
    if args.frequency {
        if let Some(cur) = cpu_info.frequency.current {
            info_lines.push(layout::format_kv(
                "Current Frequency",
                &format!("{:.3} GHz", cur / 1000.0).green().to_string(),
                LABEL_WIDTH,
            ));
        }
    }

    // ── Cache (always shown when data is available) ──────────────────────
    let cache_labels = ["L1i Cache", "L1d Cache", "L2 Cache", "L3 Cache"];
    for (label, size) in cache_labels.iter().zip(cpu_info.cache_sizes.iter()) {
        if let Some(kb) = size {
            let display = if *kb >= 1024 {
                format!("{} MB", kb / 1024)
            } else {
                format!("{kb} KB")
            };
            info_lines.push(layout::format_kv(label, &display.green().to_string(), LABEL_WIDTH));
        }
    }

    // ── Peak performance ────────────────────────────────────────────────────
    if let Some(flops) = cpu_info.peak_flops
        && flops > 0.0
    {
        info_lines.push(layout::format_kv(
            "Peak Performance",
            &format!("{flops:.2} GFLOP/s").green().to_string(),
            LABEL_WIDTH,
        ));
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
            println!("{line}");
        }
        return Ok(());
    }

    // Determine logo size: auto-detect from terminal width, or use CLI override
    let logo_size = if args.logo_short {
        logo::LogoSize::Short
    } else if args.logo_long {
        logo::LogoSize::Long
    } else {
        // Auto-detect: try LONG first, fall back to SHORT if terminal is too narrow
        let term_width = crossterm::terminal::size().map_or(80, |(w, _)| u32::from(w));
        let long_logo = logo::get_raw_logo(&cpu_info.vendor, logo::LogoSize::Long);
        let long_width = long_logo.lines().map(visible_width).max().unwrap_or(0);
        #[allow(clippy::cast_possible_truncation)]
        let needed = (long_width + LOGO_INFO_GAP + LABEL_WIDTH + 40) as u32;
        if term_width >= needed {
            logo::LogoSize::Long
        } else {
            logo::LogoSize::Short
        }
    };

    let raw_logo = logo::get_raw_logo(&cpu_info.vendor, logo_size);
    let logo_colors = logo::get_logo_colors(&cpu_info.vendor);

    // Compute visual width from the raw logo (before adding colour codes)
    let logo_visual_width = raw_logo.lines().map(visible_width).max().unwrap_or(0);

    // Colourize each logo line and pad to uniform visual width
    let logo_lines: Vec<String> = raw_logo
        .lines()
        .map(|l| {
            let vis_w = visible_width(l);
            let pad = logo_visual_width.saturating_sub(vis_w);
            if args.no_color {
                format!("{}{}", strip_color_markers(l), " ".repeat(pad))
            } else {
                format!("{}{}", logo::colorize_logo_line(l, &logo_colors), " ".repeat(pad))
            }
        })
        .collect();

    // Print side-by-side: logo on the left, info on the right
    let max_rows = logo_lines.len().max(info_lines.len());
    let blank_left = " ".repeat(logo_visual_width);

    for i in 0..max_rows {
        let right = info_lines.get(i).map_or("", String::as_str);
        if let Some(left) = logo_lines.get(i) {
            println!("{left}{}{right}", " ".repeat(LOGO_INFO_GAP));
        } else {
            println!("{blank_left}{}{right}", " ".repeat(LOGO_INFO_GAP));
        }
    }

    Ok(())
}

/// Print CPU information in JSON format.
///
/// # Errors
///
/// Returns an error if serialisation or writing to stdout fails.
#[cfg(all(feature = "display", feature = "json"))]
pub fn print_json(cpu_info: &CpuInfo) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(cpu_info)?;
    println!("{json}");
    Ok(())
}

/// No-op stub when the json feature is disabled.
#[cfg(all(feature = "display", not(feature = "json")))]
pub fn print_json(_cpu_info: &CpuInfo) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("JSON feature not enabled"))
}

/// Compute the visible width of a logo line (excluding `$C1`–`$C4` and `$CR` markers).
#[cfg(feature = "display")]
fn visible_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('C') => {
                    chars.next(); // consume 'C'
                    if let Some('R' | '1'..='4') = chars.peek() {
                        chars.next();
                    } else {
                        width += 2; // '$' + 'C' are visible
                    }
                },
                _ => width += 1,
            }
        } else {
            width += 1;
        }
    }
    width
}

/// Strip colour markers from a logo line, returning plain text.
#[cfg(feature = "display")]
fn strip_color_markers(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('C') => {
                    chars.next();
                    if let Some('R' | '1'..='4') = chars.peek() {
                        chars.next();
                    } else {
                        result.push('$');
                        result.push('C');
                    }
                },
                _ => result.push('$'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}
