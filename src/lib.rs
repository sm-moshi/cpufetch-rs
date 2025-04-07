//! cpufetch-rs: A CPU information fetching library
//!
//! This library provides functionality to fetch and display detailed CPU information
//! across different architectures and platforms.
//!
//! ## Features
//!
//! - **cpu**: CPU detection and feature flags (always included)
//! - **frequency**: Frequency detection (optional)
//! - **cli**: Command-line interface (optional)
//! - **display**: Terminal display and formatting (optional)
//! - **json**: JSON output support (optional)
//! - **config**: Configuration file support (optional)
//!
//! Platform-specific features: **linux**, **windows**, **macos**

// Core modules - always available
pub mod arch;
pub mod cpu;
pub mod error;

// Optional modules based on feature flags
#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "display")]
pub mod printer;

#[cfg(any(feature = "cli", feature = "frequency"))]
pub mod utils;

// Re-export commonly used types
pub use cpu::info::{CpuError, CpuInfo, Frequency, Vendor, Version};
pub use error::Error;

/// Crate version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
