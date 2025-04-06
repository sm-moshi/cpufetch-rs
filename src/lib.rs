//! cpufetch-rs: A CPU information fetching library
//!
//! This library provides functionality to fetch and display detailed CPU information
//! across different architectures and platforms.

pub mod arch;
pub mod cli;
pub mod cpu;
pub mod error;
pub mod printer;
pub mod utils;

// Re-export commonly used types
pub use cpu::info::{CpuError, CpuInfo, Frequency, Vendor, Version};
pub use error::Error;
