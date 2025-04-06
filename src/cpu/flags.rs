//! CPU feature flag detection and representation.
//!
//! This module provides functionality for detecting and representing CPU feature flags
//! across different architectures. It uses dynamic feature detection where available
//! and falls back to static detection where necessary.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Error types specific to CPU feature detection
#[derive(Debug, thiserror::Error)]
pub enum FeatureError {
    #[error("Feature detection not supported on this architecture")]
    UnsupportedArch,
    #[error("Failed to detect feature {0}")]
    DetectionFailed(String),
}

bitflags! {
    /// CPU features for x86/x86_64 architectures
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct X86Features: u64 {
        const SSE = 1 << 0;
        const SSE2 = 1 << 1;
        const SSE3 = 1 << 2;
        const SSSE3 = 1 << 3;
        const SSE4_1 = 1 << 4;
        const SSE4_2 = 1 << 5;
        const AVX = 1 << 6;
        const AVX2 = 1 << 7;
        const FMA = 1 << 8;
        const BMI1 = 1 << 9;
        const BMI2 = 1 << 10;
        const F16C = 1 << 11;
        const POPCNT = 1 << 12;
        const AES = 1 << 13;
        const AVX512F = 1 << 14;
        const AVX512BW = 1 << 15;
        const AVX512CD = 1 << 16;
        const AVX512DQ = 1 << 17;
        const AVX512VL = 1 << 18;
    }
}

bitflags! {
    /// CPU features for ARM architectures
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ArmFeatures: u64 {
        const NEON = 1 << 0;
        const AES = 1 << 1;
        const PMULL = 1 << 2;
        const SHA1 = 1 << 3;
        const SHA2 = 1 << 4;
        const CRC32 = 1 << 5;
        const ATOMICS = 1 << 6;
        const FP = 1 << 7;
        const ASIMD = 1 << 8;
        const FPHP = 1 << 9;
        const ASIMDHP = 1 << 10;
        const ASIMDDP = 1 << 11;
        const ASIMDFHM = 1 << 12;
    }
}

/// Detect CPU features for the current architecture
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn detect_features() -> Result<X86Features, FeatureError> {
    let mut features = X86Features::empty();

    // Using is_x86_feature_detected! for runtime detection
    if cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") {
        if std::is_x86_feature_detected!("sse") {
            features |= X86Features::SSE;
        }
        if std::is_x86_feature_detected!("sse2") {
            features |= X86Features::SSE2;
        }
        if std::is_x86_feature_detected!("sse3") {
            features |= X86Features::SSE3;
        }
        if std::is_x86_feature_detected!("ssse3") {
            features |= X86Features::SSSE3;
        }
        if std::is_x86_feature_detected!("sse4.1") {
            features |= X86Features::SSE4_1;
        }
        if std::is_x86_feature_detected!("sse4.2") {
            features |= X86Features::SSE4_2;
        }
        if std::is_x86_feature_detected!("avx") {
            features |= X86Features::AVX;
        }
        if std::is_x86_feature_detected!("avx2") {
            features |= X86Features::AVX2;
        }
        if std::is_x86_feature_detected!("fma") {
            features |= X86Features::FMA;
        }
        if std::is_x86_feature_detected!("bmi1") {
            features |= X86Features::BMI1;
        }
        if std::is_x86_feature_detected!("bmi2") {
            features |= X86Features::BMI2;
        }
        if std::is_x86_feature_detected!("f16c") {
            features |= X86Features::F16C;
        }
        if std::is_x86_feature_detected!("popcnt") {
            features |= X86Features::POPCNT;
        }
        if std::is_x86_feature_detected!("aes") {
            features |= X86Features::AES;
        }
        if std::is_x86_feature_detected!("avx512f") {
            features |= X86Features::AVX512F;
        }
        if std::is_x86_feature_detected!("avx512bw") {
            features |= X86Features::AVX512BW;
        }
        if std::is_x86_feature_detected!("avx512cd") {
            features |= X86Features::AVX512CD;
        }
        if std::is_x86_feature_detected!("avx512dq") {
            features |= X86Features::AVX512DQ;
        }
        if std::is_x86_feature_detected!("avx512vl") {
            features |= X86Features::AVX512VL;
        }
    }

    Ok(features)
}

/// Detect CPU features for ARM architectures
#[cfg(target_arch = "aarch64")]
pub fn detect_features() -> Result<ArmFeatures, FeatureError> {
    let mut features = ArmFeatures::empty();

    // Using target_feature detection for ARM
    if cfg!(target_arch = "aarch64") {
        if std::arch::is_aarch64_feature_detected!("neon") {
            features |= ArmFeatures::NEON;
        }
        if std::arch::is_aarch64_feature_detected!("aes") {
            features |= ArmFeatures::AES;
        }
        if std::arch::is_aarch64_feature_detected!("pmull") {
            features |= ArmFeatures::PMULL;
        }
        if std::arch::is_aarch64_feature_detected!("sha2") {
            features |= ArmFeatures::SHA2;
        }
        if std::arch::is_aarch64_feature_detected!("crc") {
            features |= ArmFeatures::CRC32;
        }
        if std::arch::is_aarch64_feature_detected!("lse") {
            features |= ArmFeatures::ATOMICS;
        }
        if std::arch::is_aarch64_feature_detected!("fp") {
            features |= ArmFeatures::FP;
        }
        if std::arch::is_aarch64_feature_detected!("asimd") {
            features |= ArmFeatures::ASIMD;
        }
        // Note: Some features might not be available for detection in all environments
    }

    Ok(features)
}

/// Detect CPU features for unsupported architectures
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
pub fn detect_features() -> Result<(), FeatureError> {
    Err(FeatureError::UnsupportedArch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_features_flags() {
        let features = X86Features::SSE | X86Features::SSE2;
        assert!(features.contains(X86Features::SSE));
        assert!(features.contains(X86Features::SSE2));
        assert!(!features.contains(X86Features::AVX));
    }

    #[test]
    fn test_arm_features_flags() {
        let features = ArmFeatures::NEON | ArmFeatures::AES;
        assert!(features.contains(ArmFeatures::NEON));
        assert!(features.contains(ArmFeatures::AES));
        assert!(!features.contains(ArmFeatures::SHA2));
    }

    // Note: We can't reliably test actual feature detection in unit tests
    // as it depends on the CPU capabilities of the test machine
}
