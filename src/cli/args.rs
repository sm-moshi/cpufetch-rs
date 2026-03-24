use clap::Parser;
/// cpufetch - A fast, modern CPU detection tool
#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Show CPU frequency information
    #[clap(short, long)]
    pub frequency: bool,

    /// Show CPU cache information
    #[clap(short, long)]
    pub cache: bool,

    /// Show CPU feature flags
    #[clap(short = 'F', long)]
    pub features: bool,

    /// Output in JSON format instead of ASCII art
    #[clap(short, long)]
    pub json: bool,

    /// Don't show the CPU logo
    #[clap(long)]
    pub no_logo: bool,

    /// Don't use colour in the output
    #[clap(long)]
    pub no_color: bool,

    /// Force the short (compact) logo variant
    #[clap(long, conflicts_with = "logo_long")]
    pub logo_short: bool,

    /// Force the long (detailed) logo variant
    #[clap(long, conflicts_with = "logo_short")]
    pub logo_long: bool,

    /// Output style: default (no frame), fancy (box border), retro (ASCII border)
    #[clap(short, long, value_parser = ["default", "fancy", "retro"])]
    pub style: Option<String>,

    /// Show debug information
    #[clap(long)]
    pub debug: bool,
}
