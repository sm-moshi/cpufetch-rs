//! Layout handling for CPU information display.
//!
//! This module provides layout and formatting utilities for displaying
//! CPU information in the terminal.

use colored::control::set_override;

/// Set up the terminal display based on command line options
#[cfg(feature = "display")]
pub fn setup_display(no_color: bool) {
    // Disable colors if requested
    if no_color {
        set_override(false);
    }
}

/// Get the terminal width or default to 80 columns
#[cfg(feature = "display")]
pub fn get_terminal_width() -> usize {
    // Try to get the terminal width
    if let Ok((width, _)) = crossterm::terminal::size() {
        width as usize
    } else {
        // Default to 80 columns if we can't get the actual width
        80
    }
}

/// Pad a string to center it in the terminal
#[cfg(feature = "display")]
pub fn center_text(text: &str, width: usize) -> String {
    let padding = if text.len() < width {
        (width - text.len()) / 2
    } else {
        0
    };

    format!("{:width$}{}", "", text, width = padding)
}
