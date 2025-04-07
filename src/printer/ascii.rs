//! ASCII art utilities.
//!
//! This module provides utilities for working with ASCII art in the terminal.

/// Calculate the max width of a multiline ASCII art string
#[cfg(feature = "display")]
pub fn max_width(ascii_art: &str) -> usize {
    ascii_art
        .lines()
        .map(|line| line.len())
        .max()
        .unwrap_or(0)
}

/// Frame an ASCII art string with a border
#[cfg(feature = "display")]
pub fn frame(ascii_art: &str, padding: usize) -> String {
    let lines: Vec<&str> = ascii_art.lines().collect();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut result = String::new();

    // Top border
    result.push_str(&format!("┌{}┐\n", "─".repeat(max_width + padding * 2)));

    // Content with padding
    for line in lines {
        let padding_right = max_width - line.len() + padding;
        result.push_str(&format!("│{}{}{}\n", " ".repeat(padding), line, " ".repeat(padding_right)));
    }

    // Bottom border
    result.push_str(&format!("└{}┘\n", "─".repeat(max_width + padding * 2)));

    result
}

/// Combine two ASCII art strings side by side
#[cfg(feature = "display")]
pub fn combine_horizontal(left: &str, right: &str, spacing: usize) -> String {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();

    let left_height = left_lines.len();
    let right_height = right_lines.len();
    let max_height = left_height.max(right_height);

    let mut result = String::new();

    for i in 0..max_height {
        let left_line = if i < left_height { left_lines[i] } else { "" };
        let right_line = if i < right_height { right_lines[i] } else { "" };

        result.push_str(&format!("{}{}{}\n", left_line, " ".repeat(spacing), right_line));
    }

    result
}
