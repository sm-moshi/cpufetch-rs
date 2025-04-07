//! Error types for cpufetch
//!
//! This module provides the error types used throughout the crate.

use thiserror::Error;

/// Main error type for cpufetch operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("CPU detection error: {0}")]
    CpuDetection(String),

    #[error("CPU error: {0}")]
    Cpu(#[from] crate::cpu::info::CpuError),

    #[error("Feature detection error: {0}")]
    Feature(#[from] crate::cpu::flags::FeatureError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported architecture")]
    UnsupportedArchitecture,

    // Feature-specific errors
    #[cfg(feature = "frequency")]
    #[error("Frequency detection error: {0}")]
    Frequency(String),

    #[cfg(feature = "display")]
    #[error("Display error: {0}")]
    Display(String),

    #[cfg(feature = "json")]
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "config")]
    #[error("Configuration error: {0}")]
    Config(String),

    #[cfg(feature = "cli")]
    #[error("CLI error: {0}")]
    Cli(String),

    #[error("Unknown error: {0}")]
    Other(String),
}

/// Create a basic implementation for conversion from string errors
impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Other(err)
    }
}

/// Create a basic implementation for conversion from static string errors
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Other(err.to_string())
    }
}
