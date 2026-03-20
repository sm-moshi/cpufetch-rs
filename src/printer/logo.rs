//! CPU vendor logo ASCII art.
//!
//! This module provides ASCII art representations of CPU vendor logos and
//! helpers for colouring them on a line-by-line basis (required for correct
//! side-by-side layout — applying ANSI codes to the whole multi-line string
//! produces inconsistent escape sequences when the string is split).

use crate::cpu::Vendor;
use colored::Colorize;

/// Return the raw (uncoloured) ASCII art for the given vendor.
#[cfg(feature = "display")]
pub fn get_raw_logo(vendor: &Vendor) -> &'static str {
    match vendor {
        Vendor::Intel => INTEL_LOGO,
        Vendor::AMD => AMD_LOGO,
        Vendor::ARM => ARM_LOGO,
        Vendor::Apple => APPLE_LOGO,
        Vendor::Unknown => GENERIC_LOGO,
    }
}

/// Return the ANSI colour name string for the given vendor.
#[cfg(feature = "display")]
pub fn get_logo_color_name(vendor: &Vendor) -> &'static str {
    match vendor {
        Vendor::Intel => "blue",
        Vendor::AMD => "red",
        Vendor::ARM => "green",
        Vendor::Apple => "white",
        Vendor::Unknown => "yellow",
    }
}

/// Colourize a single line of ASCII art using a named colour.
#[cfg(feature = "display")]
pub fn colorize_line(line: &str, color: &str) -> String {
    match color {
        "blue" => line.blue().bold().to_string(),
        "red" => line.red().bold().to_string(),
        "green" => line.green().bold().to_string(),
        "yellow" => line.yellow().bold().to_string(),
        _ => line.white().bold().to_string(),
    }
}

// ── Logo constants ──────────────────────────────────────────────────────────

/// Intel CPU logo ASCII art
#[cfg(feature = "display")]
const INTEL_LOGO: &str = "    ///////////////\n    /////INTEL/////\n    ///////////////";

/// AMD CPU logo ASCII art
#[cfg(feature = "display")]
const AMD_LOGO: &str = "   ///////////\n  //   AMD  //\n ///////////";

/// ARM CPU logo ASCII art
#[cfg(feature = "display")]
const ARM_LOGO: &str = "  /=======\\\n // ARM   ||\n \\\\      //\n  \\=====/";

/// Apple CPU logo ASCII art
#[cfg(feature = "display")]
const APPLE_LOGO: &str = "     .\n    / \\\n   /   \\\n  /     \\\n / APPLE \\\n/_________\\";

/// Generic CPU logo ASCII art
#[cfg(feature = "display")]
const GENERIC_LOGO: &str = "  /---------\\\n |   CPU    |\n  \\---------/";
