//! Layout handling for CPU information display.
//!
//! This module provides layout and formatting utilities for displaying
//! CPU information in the terminal.

use colored::control::set_override;

/// Set up the terminal display based on command-line options.
#[cfg(feature = "display")]
pub fn setup_display(no_color: bool) {
    if no_color {
        set_override(false);
    }
}

/// Format a key-value display line with aligned columns.
///
/// The label (with a trailing `:`) is left-padded to `label_width` characters,
/// followed by two spaces and then the (pre-coloured) value string.
///
/// Example output with `label_width = 20`:
/// ```text
/// Vendor:               AMD
/// Microarchitecture:    Zen 3
/// ```
#[cfg(feature = "display")]
pub fn format_kv(label: &str, value: &str, label_width: usize) -> String {
    let labelled = format!("{label}:");
    format!("{labelled:<label_width$}  {value}")
}
