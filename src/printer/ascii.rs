//! ASCII art utilities.
//!
//! This module provides utilities for working with ASCII art in the terminal.

/// Compute the visual (character) width of the widest line in a multi-line string.
///
/// Uses `.chars().count()` so that multi-byte UTF-8 characters (such as box-drawing
/// glyphs) are each counted as one column, matching terminal rendering behaviour.
#[cfg(feature = "display")]
#[allow(dead_code)]
pub fn max_width(ascii_art: &str) -> usize {
    ascii_art.lines().map(|line| line.chars().count()).max().unwrap_or(0)
}

/// Frame an ASCII art string with a Unicode box border.
///
/// Each content line is padded to the same width so that the right border
/// aligns consistently.  Returns a string where every line has the same
/// visual width: `content_max_width + 2 * padding + 2` (two border chars).
#[cfg(feature = "display")]
pub fn frame(ascii_art: &str, padding: usize) -> String {
    let lines: Vec<&str> = ascii_art.lines().collect();
    // Visual width of the widest content line (ASCII-only logos, so len == chars)
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let inner_width = max_w + padding * 2;

    let mut result = String::new();

    // Top border
    result.push_str(&format!("┌{}┐\n", "─".repeat(inner_width)));

    // Content with left and right padding to reach consistent width
    for line in &lines {
        let line_chars = line.chars().count();
        let pad_right = max_w - line_chars + padding;
        result.push_str(&format!("│{}{}{}│\n", " ".repeat(padding), line, " ".repeat(pad_right)));
    }

    // Bottom border
    result.push_str(&format!("└{}┘", "─".repeat(inner_width)));

    result
}

/// Combine two ASCII art strings side by side with a fixed horizontal gap.
///
/// When the left side has fewer lines than the right, shorter rows are padded
/// with `left_visual_width` spaces so that the right column stays aligned.
/// This is necessary when the left string contains ANSI colour codes (which
/// inflate `.len()` but have zero visual width in a terminal).
#[cfg(feature = "display")]
#[allow(dead_code)]
pub fn combine_horizontal(left: &str, right: &str, left_visual_width: usize, spacing: usize) -> String {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();

    let max_height = left_lines.len().max(right_lines.len());
    let blank_left = " ".repeat(left_visual_width);

    let mut result = String::new();

    for i in 0..max_height {
        let left_line = left_lines.get(i).copied().unwrap_or(&blank_left);
        let right_line = right_lines.get(i).copied().unwrap_or("");
        let gap = " ".repeat(spacing);

        if left_lines.get(i).is_some() {
            result.push_str(&format!("{}{}{}\n", left_line, gap, right_line));
        } else {
            result.push_str(&format!("{}{}{}\n", blank_left, gap, right_line));
        }
    }

    result
}
