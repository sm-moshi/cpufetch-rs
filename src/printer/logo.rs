//! CPU vendor logo ASCII art.
//!
//! This module provides ASCII art representations of CPU vendor logos.

use crate::cpu::Vendor;
use colored::Colorize;

/// Get the ASCII art logo for a given CPU vendor
#[cfg(feature = "display")]
pub fn get_logo(vendor: &Vendor) -> String {
    match vendor {
        Vendor::Intel => format_logo(INTEL_LOGO, "blue"),
        Vendor::AMD => format_logo(AMD_LOGO, "red"),
        Vendor::ARM => format_logo(ARM_LOGO, "green"),
        Vendor::Apple => format_logo(APPLE_LOGO, "white"),
        Vendor::Unknown => format_logo(GENERIC_LOGO, "yellow"),
    }
}

/// Format the logo with the appropriate color
#[cfg(feature = "display")]
fn format_logo(logo: &str, color: &str) -> String {
    match color {
        "blue" => logo.blue().bold().to_string(),
        "red" => logo.red().bold().to_string(),
        "green" => logo.green().bold().to_string(),
        "yellow" => logo.yellow().bold().to_string(),
        _ => logo.white().bold().to_string(),
    }
}

/// Intel CPU logo ASCII art
#[cfg(feature = "display")]
const INTEL_LOGO: &str = r#"
    ///////////////
    /////INTEL/////
    ///////////////
"#;

/// AMD CPU logo ASCII art
#[cfg(feature = "display")]
const AMD_LOGO: &str = r#"
   ///////////
  //   AMD  //
 ///////////
"#;

/// ARM CPU logo ASCII art
#[cfg(feature = "display")]
const ARM_LOGO: &str = r#"
  /=======\
 // ARM   ||
 \\      //
  \=====/
"#;

/// Apple CPU logo ASCII art
#[cfg(feature = "display")]
const APPLE_LOGO: &str = r#"
     .
    / \
   /   \
  /     \
 /  APPLE \
/___________\
"#;

/// Generic CPU logo ASCII art
#[cfg(feature = "display")]
const GENERIC_LOGO: &str = r#"
  /---------\
 |   CPU    |
  \---------/
"#;
