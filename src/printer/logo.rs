//! CPU vendor logo ASCII art.
//!
//! Provides ASCII art representations of CPU vendor logos with multi-colour
//! support. Logos are stored with `$C1`–`$C4` colour markers and `$CR` reset
//! markers; the rendering function replaces these with ANSI escape codes.
//!
//! Each vendor has a SHORT and optionally a LONG variant. Terminal width
//! detection picks the best fit automatically.

use crate::cpu::Vendor;
use colored::Color;

// ── Types ────────────────────────────────────────────────────────────────────

/// Logo size variant.
#[cfg(feature = "display")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoSize {
    Short,
    Long,
}

/// Colour palette for a logo (up to 4 colours).
#[cfg(feature = "display")]
pub struct LogoColors {
    pub colors: [Option<Color>; 4],
}

// ── Public API ───────────────────────────────────────────────────────────────

/// Return the raw ASCII art (with colour markers) for the given vendor and size.
///
/// Falls back to `Short` if no `Long` variant exists for the vendor.
#[cfg(feature = "display")]
pub fn get_raw_logo(vendor: &Vendor, size: LogoSize) -> &'static str {
    match (vendor, size) {
        (Vendor::Intel, LogoSize::Long) => INTEL_LOGO_LONG,
        (Vendor::Intel, LogoSize::Short) => INTEL_LOGO,
        (Vendor::AMD, LogoSize::Long) => AMD_LOGO_LONG,
        (Vendor::AMD, LogoSize::Short) => AMD_LOGO,
        (Vendor::ARM, LogoSize::Long) => ARM_LOGO_LONG,
        (Vendor::ARM, LogoSize::Short) => ARM_LOGO,
        (Vendor::Apple, _) => APPLE_LOGO,
        (Vendor::Unknown, _) => GENERIC_LOGO,
    }
}

/// Return the colour palette for the given vendor's logo.
#[cfg(feature = "display")]
pub fn get_logo_colors(vendor: &Vendor) -> LogoColors {
    match vendor {
        Vendor::Intel | Vendor::ARM => LogoColors {
            colors: [Some(Color::Cyan), None, None, None],
        },
        Vendor::AMD => LogoColors {
            colors: [Some(Color::White), Some(Color::Green), None, None],
        },
        Vendor::Apple => LogoColors {
            colors: [Some(Color::White), None, None, None],
        },
        Vendor::Unknown => LogoColors {
            colors: [Some(Color::Yellow), None, None, None],
        },
    }
}

/// Return the text highlight colour for the info block (used for values).
#[cfg(feature = "display")]
#[allow(dead_code)]
pub fn get_text_color(vendor: &Vendor) -> Color {
    match vendor {
        Vendor::AMD => Color::Green,
        Vendor::Intel | Vendor::ARM | Vendor::Apple => Color::Cyan,
        Vendor::Unknown => Color::Yellow,
    }
}

/// Colourize a single logo line by replacing `$C1`–`$C4` and `$CR` markers
/// with ANSI colour codes from the given palette.
#[cfg(feature = "display")]
pub fn colorize_logo_line(line: &str, colors: &LogoColors) -> String {
    let mut result = String::with_capacity(line.len() * 2);
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            // Peek at the next character to determine the marker type.
            match chars.peek() {
                Some('C') => {
                    chars.next(); // consume 'C'
                    match chars.peek() {
                        Some('R') => {
                            chars.next(); // consume 'R'
                            // Reset — we just stop colouring; the next $Cn will set a new colour.
                            // For simplicity, insert a reset escape.
                            result.push_str("\x1b[0m");
                        },
                        Some(n @ '1'..='4') => {
                            let idx = (*n as usize) - ('1' as usize);
                            chars.next(); // consume digit
                            if let Some(color) = colors.colors[idx] {
                                result.push_str(&color_to_ansi_bold(color));
                            }
                        },
                        _ => {
                            result.push('$');
                            result.push('C');
                        },
                    }
                },
                _ => result.push('$'),
            }
        } else {
            result.push(ch);
        }
    }

    // Reset at end of line so colours don't bleed into the info column.
    result.push_str("\x1b[0m");
    result
}

/// Convert a `colored::Color` to a bold ANSI escape sequence.
#[cfg(feature = "display")]
fn color_to_ansi_bold(color: Color) -> String {
    // Use the colored crate's Colorize trait on a dummy string to extract the code,
    // or just map directly to ANSI codes for efficiency.
    match color {
        Color::Black => "\x1b[30;1m".to_string(),
        Color::Red => "\x1b[31;1m".to_string(),
        Color::Green => "\x1b[32;1m".to_string(),
        Color::Yellow => "\x1b[33;1m".to_string(),
        Color::Blue => "\x1b[34;1m".to_string(),
        Color::Magenta => "\x1b[35;1m".to_string(),
        Color::Cyan => "\x1b[36;1m".to_string(),
        Color::White => "\x1b[37;1m".to_string(),
        Color::BrightBlack => "\x1b[90;1m".to_string(),
        Color::BrightRed => "\x1b[91;1m".to_string(),
        Color::BrightGreen => "\x1b[92;1m".to_string(),
        Color::BrightYellow => "\x1b[93;1m".to_string(),
        Color::BrightBlue => "\x1b[94;1m".to_string(),
        Color::BrightMagenta => "\x1b[95;1m".to_string(),
        Color::BrightCyan => "\x1b[96;1m".to_string(),
        Color::BrightWhite => "\x1b[97;1m".to_string(),
        Color::TrueColor { r, g, b } => format!("\x1b[38;2;{r};{g};{b};1m"),
        Color::AnsiColor(n) => format!("\x1b[38;5;{n};1m"),
    }
}

// ── SHORT logos ──────────────────────────────────────────────────────────────
// Ported from cpufetch-c/src/common/ascii.h.
// Colour markers: $C1–$C4 = logo colours, $CR = reset.

#[cfg(feature = "display")]
const AMD_LOGO: &str = "\
$C2          '###############          \n\
$C2             ,#############         \n\
$C2                      .####         \n\
$C2              #.      .####         \n\
$C2            :##.      .####         \n\
$C2           :###.      .####         \n\
$C2           #########.   :##         \n\
$C2           #######.       ;         \n\
$C1                                    \n\
$C1    ###     ###      ###   ####### \n\
$C1   ## ##    #####  #####   ##   ## \n\
$C1  ##   ##   ### #### ###   ##    ##\n\
$C1 #########  ###  ##  ###   ##    ##\n\
$C1##       ## ###      ###   ##   ## \n\
$C1##       ## ###      ###   ####### ";

#[cfg(feature = "display")]
const INTEL_LOGO: &str = "\
$C1                   .#################.       \n\
$C1              .####                   ####.  \n\
$C1          .##                             ###\n\
$C1       ##                          :##     ##\n\
$C1    #                ##            :##      #\n\
$C1  ##   ##  ######.   ####  ######  :##      #\n\
$C1 ##    ##  ##:  ##:  ##   ##   ### :##     ##\n\
$C1##     ##  ##:  ##:  ##  :######## :##    ## \n\
$C1##     ##  ##:  ##:  ##   ##.   .  :## ####  \n\
$C1##      #  ##:  ##:  ####  #####:   ##       \n\
$C1 ##                                          \n\
$C1  ###.                         ..o####.      \n\
$C1   ######oo...         ..oo#######           \n\
$C1          o###############o                  ";

#[cfg(feature = "display")]
const ARM_LOGO: &str = "\
$C1   #####  ##   # #####  ## ####  ###### \n\
$C1 ###    ####   ###      ####  ###   ### \n\
$C1###       ##   ###      ###    ##    ###\n\
$C1 ###    ####   ###      ###    ##    ###\n\
$C1  ######  ##   ###      ###    ##    ###";

// Inspired by the neofetch mac logo
#[cfg(feature = "display")]
const APPLE_LOGO: &str = "\
$C1                   .\"c.      \n\
$C1                 ,xNMM.      \n\
$C1                .lMM\"        \n\
$C1                MM*          \n\
$C1     .;loddo;:.   olloddol;. \n\
$C1   cKMMMMMMMMMMNWMMMMMMMMMMM0\n\
$C1 .KMMMMMMMMMMMMMMMMMMMMMMMW* \n\
$C1 XMMMMMMMMMMMMMMMMMMMMMMMX.  \n\
$C1;MMMMMMMMMMMMMMMMMMMMMMMM:   \n\
$C1:MMMMMMMMMMMMMMMMMMMMMMMM:   \n\
$C1.MMMMMMMMMMMMMMMMMMMMMMMMX.  \n\
$C1 kMMMMMMMMMMMMMMMMMMMMMMMMWd.\n\
$C1 'XMMMMMMMMMMMMMMMMMMMMMMMMM \n\
$C1  'XMMMMMMMMMMMMMMMMMMMMMMK. \n\
$C1    kMMMMMMMMMMMMMMMMMMMMd   \n\
$C1     'KMMMMMMMWXXWMMMMMk.    \n\
$C1       \"cooc\"*    \"*coo'\"    ";

#[cfg(feature = "display")]
const GENERIC_LOGO: &str = "\
$C1  /---------\\\n\
$C1 |   CPU    |\n\
$C1  \\---------/";

// ── LONG logos ───────────────────────────────────────────────────────────────

#[cfg(feature = "display")]
const AMD_LOGO_LONG: &str = "\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1     @@@@      @@@       @@@   @@@@@@@@      $C2  ############\n\
$C1    @@@@@@     @@@@@   @@@@@   @@@    @@@    $C2    ##########\n\
$C1   @@@  @@@    @@@@@@@@@@@@@   @@@      @@   $C2   #     #####\n\
$C1  @@@    @@@   @@@  @@@  @@@   @@@      @@   $C2 ###     #####\n\
$C1 @@@@@@@@@@@@  @@@       @@@   @@@    @@@    $C2#########  ###\n\
$C1 @@@      @@@  @@@       @@@   @@@@@@@@@     $C2########    ##\n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           \n\
$C1                                                           ";

#[cfg(feature = "display")]
const INTEL_LOGO_LONG: &str = "\
$C1                               ###############@            \n\
$C1                       ######@                ######@      \n\
$C1                  ###@                              ###@   \n\
$C1              ##@                                     ###@ \n\
$C1         ##@                                             ##\n\
$C1         ##@                                             ##\n\
$C1      @                    ##@                ##@        ##\n\
$C1    #@   ##@   ########@   #####@   #####@    ##@        ##\n\
$C1   #@    ##@   ##@    ##@  ##@    ###@  ###@  ##@        ##\n\
$C1  #@     ##@   ##@    ##@  ##@    ##@    ##@  ##@       ## \n\
$C1 #@      ##@   ##@    ##@  ##@    #########@  ##@     ###  \n\
$C1 #@      ##@   ##@    ##@  ##@    ##@         ##@   ####   \n\
$C1 #@       #@   ##@    ##@   ####@  ########@   #@  ##     \n\
$C1 ##@                                                       \n\
$C1  ##@                                                      \n\
$C1  ###@                                        ###@         \n\
$C1    ####@                               #########@         \n\
$C1      #########@               ###############@            \n\
$C1          ##############################@                  ";

#[cfg(feature = "display")]
const ARM_LOGO_LONG: &str = "\
$C1     ############    ##########   ####  #######  ########\n\
$C1  ###############    #########    #######################\n\
$C1 ####        ####    ####         #####   ########   ####\n\
$C1####         ####    ####         ####     ######     ###\n\
$C1####         ####    ####         ####      ####      ###\n\
$C1 ####       #####    ####         ####      ####      ###\n\
$C1  ###############    ####         ####      ####      ###\n\
$C1   ########  ####    ####         ####      ####      ###";

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_logo_line_single_color() {
        let colors = LogoColors {
            colors: [Some(Color::Cyan), None, None, None],
        };
        let line = "$C1Hello World";
        let result = colorize_logo_line(line, &colors);
        assert!(result.contains("Hello World"));
        assert!(result.contains("\x1b[36;1m")); // cyan bold
        assert!(result.ends_with("\x1b[0m")); // reset at end
    }

    #[test]
    fn test_colorize_logo_line_two_colors() {
        let colors = LogoColors {
            colors: [Some(Color::White), Some(Color::Green), None, None],
        };
        let line = "$C1White part$C2Green part";
        let result = colorize_logo_line(line, &colors);
        assert!(result.contains("\x1b[37;1m")); // white bold
        assert!(result.contains("\x1b[32;1m")); // green bold
    }

    #[test]
    fn test_colorize_logo_line_with_reset() {
        let colors = LogoColors {
            colors: [Some(Color::Red), None, None, None],
        };
        let line = "$C1Red$CR Normal$C1Red again";
        let result = colorize_logo_line(line, &colors);
        // Should contain two red sequences with a reset between them
        assert_eq!(result.matches("\x1b[31;1m").count(), 2);
        assert!(result.contains("\x1b[0m"));
    }

    #[test]
    fn test_get_raw_logo_fallback() {
        // Apple has no LONG variant — should return SHORT
        let short = get_raw_logo(&Vendor::Apple, LogoSize::Short);
        let long = get_raw_logo(&Vendor::Apple, LogoSize::Long);
        assert_eq!(short, long);
    }

    #[test]
    fn test_logo_line_counts() {
        // Verify logos have expected line counts
        assert_eq!(get_raw_logo(&Vendor::AMD, LogoSize::Short).lines().count(), 15);
        assert_eq!(get_raw_logo(&Vendor::Intel, LogoSize::Short).lines().count(), 14);
        assert_eq!(get_raw_logo(&Vendor::ARM, LogoSize::Short).lines().count(), 5);
        assert_eq!(get_raw_logo(&Vendor::Apple, LogoSize::Short).lines().count(), 17);
        assert_eq!(get_raw_logo(&Vendor::AMD, LogoSize::Long).lines().count(), 19);
        assert_eq!(get_raw_logo(&Vendor::Intel, LogoSize::Long).lines().count(), 19);
        assert_eq!(get_raw_logo(&Vendor::ARM, LogoSize::Long).lines().count(), 8);
    }
}
