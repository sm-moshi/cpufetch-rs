//! Command-line argument parsing
//!
//! This module defines the Args structure for parsing command-line arguments.

use clap::Parser;

/// cpufetch - A fast, modern CPU detection tool
#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Show CPU frequency information
    #[clap(short, long)]
    pub frequency: bool,

    /// Show CPU cache information
    #[clap(short, long)]
    pub cache: bool,

    /// Show CPU feature flags
    #[clap(short, long)]
    pub features: bool,

    /// Output in JSON format instead of ASCII art
    #[clap(short, long)]
    pub json: bool,

    /// Don't show the CPU logo
    #[clap(long)]
    pub no_logo: bool,

    /// Don't use color in the output
    #[clap(long)]
    pub no_color: bool,

    /// Show debug information
    #[clap(long)]
    pub debug: bool,
}
