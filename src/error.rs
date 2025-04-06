//! Error types for cpufetch
//!
//! This module provides the error types used throughout the crate.

use thiserror::Error;

/// Main error type for cpufetch operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("CPU error: {0}")]
    Cpu(#[from] crate::cpu::info::CpuError),
    #[error("Feature detection error: {0}")]
    Feature(#[from] crate::cpu::flags::FeatureError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unsupported architecture")]
    UnsupportedArchitecture,
    #[error("Unknown error: {0}")]
    Other(String),
}
