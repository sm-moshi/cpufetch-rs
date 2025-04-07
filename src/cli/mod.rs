//! Command-line interface module
//!
//! This module provides the command-line argument parsing functionality.

#[cfg(feature = "cli")]
mod args;

#[cfg(feature = "cli")]
pub use args::Args;
