//! ARM64 architecture-specific CPU detection.
//!
//! On macOS, Apple Silicon chips are identified via `hw.cpufamily` sysctl and
//! distinguished by P-core / E-core counts.  On Linux and other platforms a
//! generic ARM fallback is returned.

use crate::cpu::info::Frequency;
use crate::cpu::{ArmFeatures, CpuError, CpuInfo, Vendor, Version};

/// Detect CPU information for ARM64 systems.
pub fn detect_cpu() -> Result<CpuInfo, CpuError> {
    // On macOS, attempt Apple Silicon identification first.
    #[cfg(all(target_os = "macos", feature = "macos"))]
    if let Some(info) = apple_silicon::detect() {
        return Ok(info);
    }

    // Generic ARM fallback (Linux, bare-metal, etc.)
    Ok(CpuInfo {
        vendor: Vendor::ARM,
        brand_string: String::from("ARM Processor"),
        version: Version {
            family: 0,
            model: 0,
            stepping: 0,
        },
        physical_cores: num_cpus::get_physical() as u32,
        logical_cores: num_cpus::get() as u32,
        frequency: Frequency::default(),
        cache_sizes: [None; 4],
        features: detect_arm_features()?,
        microarch: None,
        hypervisor: None,
        peak_flops: None,
    })
}

// ── ARM feature detection ────────────────────────────────────────────────────

fn detect_arm_features() -> Result<ArmFeatures, CpuError> {
    let mut features = ArmFeatures::empty();

    #[cfg(target_arch = "aarch64")]
    {
        macro_rules! probe {
            ($feat:expr, $flag:expr) => {
                if std::arch::is_aarch64_feature_detected!($feat) {
                    features |= $flag;
                }
            };
        }

        probe!("neon", ArmFeatures::NEON);
        probe!("aes", ArmFeatures::AES);
        probe!("pmull", ArmFeatures::PMULL);
        probe!("sha2", ArmFeatures::SHA2);
        probe!("crc", ArmFeatures::CRC32);
        probe!("lse", ArmFeatures::ATOMICS);
        probe!("fp", ArmFeatures::FP);
        probe!("asimd", ArmFeatures::ASIMD);
    }

    Ok(features)
}

// ── Apple Silicon detection (macOS only) ─────────────────────────────────────

#[cfg(all(target_os = "macos", feature = "macos"))]
mod apple_silicon {
    use super::detect_arm_features;
    use crate::cpu::info::Frequency;
    use crate::cpu::uarch::Microarch;
    use crate::cpu::{CpuInfo, Vendor, Version};

    /// Read a sysctl key as a `u32`, reinterpreting signed bits correctly.
    ///
    /// Many macOS hw.* keys are CTLTYPE_INT (signed 32-bit) but contain values
    /// that look like large unsigned constants (e.g. CPU family IDs such as
    /// `0xDA33D83D`).  Casting `i32 as u32` preserves the bit pattern.
    fn sysctl_u32(name: &str) -> Option<u32> {
        use sysctl::{Ctl, CtlValue, Sysctl};
        let ctl = Ctl::new(name).ok()?;
        match ctl.value().ok()? {
            CtlValue::Int(i) => Some(i as u32),
            CtlValue::Uint(u) => Some(u),
            CtlValue::Long(l) => Some(l as u32),
            CtlValue::Ulong(u) => Some(u as u32),
            _ => None,
        }
    }

    /// Map `hw.cpufamily` → (generation label, Microarch).
    ///
    /// Values are constants from `<mach/machine.h>` (macOS 15 / Sequoia):
    /// ```text
    /// CPUFAMILY_ARM_FIRESTORM_ICESTORM  0x1b588bb3  (M1 / A14)
    /// CPUFAMILY_ARM_BLIZZARD_AVALANCHE  0xda33d83d  (M2 / A15)
    /// CPUFAMILY_ARM_EVEREST_SAWTOOTH    0x8765edea  (M3 / A16/A17 Pro)
    /// CPUFAMILY_ARM_COLL                0x17d5b93a  (M4 / A18 Pro)
    /// ```
    fn classify_family(family: u32) -> Option<(&'static str, Microarch)> {
        match family {
            0x1b588bb3 => Some(("M1", Microarch::AppleM1)),
            0xda33d83d => Some(("M2", Microarch::AppleM2)),
            0x8765edea => Some(("M3", Microarch::AppleM3)),
            0x17d5b93a => Some(("M4", Microarch::AppleM4)),
            _ => None,
        }
    }

    /// Determine the chip variant (Pro / Max / Ultra) from P-core and E-core counts.
    ///
    /// `hw.perflevel0.physicalcpu` = P-core count (performance, highest frequency)
    /// `hw.perflevel1.physicalcpu` = E-core count (efficiency, lower frequency)
    fn chip_variant(generation: &str, p_cores: u32, e_cores: u32) -> &'static str {
        let total = p_cores + e_cores;
        match generation {
            "M1" => match (p_cores, e_cores, total) {
                (4, 4, 8) => "",        // M1
                (6, 2, 8) => " Pro",    // M1 Pro (8-core config)
                (8, 2, 10) => " Pro",   // M1 Pro (10-core config)
                (8, 4, 12) => " Max",   // M1 Max
                (_, _, 20) => " Ultra", // M1 Ultra
                _ => "",
            },
            "M2" => match (p_cores, e_cores, total) {
                (4, 4, 8) => "",        // M2
                (6, 4, 10) => " Pro",   // M2 Pro (10-core config)
                (8, 4, 12) => " Pro",   // M2 Pro (12-core config)
                (_, _, 16) => " Max",   // M2 Max
                (_, _, 24) => " Ultra", // M2 Ultra
                _ => "",
            },
            "M3" => match (p_cores, e_cores, total) {
                (4, 4, 8) => "",        // M3
                (5, 6, 11) => " Pro",   // M3 Pro (11-core config)
                (6, 6, 12) => " Pro",   // M3 Pro (12-core config)
                (12, 4, 16) => " Max",  // M3 Max
                (_, _, 32) => " Ultra", // M3 Ultra
                _ => "",
            },
            "M4" => match (p_cores, e_cores, total) {
                (4, 6, 10) => "",                    // M4
                (10, 4, 14) => " Pro",               // M4 Pro 14-core
                (12, 4, 16) => " Max",               // M4 Max (16-core, if/when released)
                (_, _, 20) => " Max",                // M4 Max larger config
                (_, _, 28) | (_, _, 40) => " Ultra", // M4 Ultra (speculative)
                _ => "",
            },
            _ => "",
        }
    }

    /// Perform Apple Silicon detection and return a populated `CpuInfo`.
    ///
    /// Returns `None` if the CPU family is unrecognised (non-Apple ARM hardware).
    pub fn detect() -> Option<CpuInfo> {
        let family = sysctl_u32("hw.cpufamily")?;
        let (generation, microarch) = classify_family(family)?;

        // P-cores are perflevel 0 (fastest), E-cores are perflevel 1.
        let p_cores = sysctl_u32("hw.perflevel0.physicalcpu").unwrap_or(0);
        let e_cores = sysctl_u32("hw.perflevel1.physicalcpu").unwrap_or(0);

        let variant = chip_variant(generation, p_cores, e_cores);
        let brand_string = format!("Apple {}{}", generation, variant);

        let physical_cores = num_cpus::get_physical() as u32;
        let logical_cores = num_cpus::get() as u32;

        let features = detect_arm_features().unwrap_or_default();

        Some(CpuInfo {
            vendor: Vendor::Apple,
            brand_string,
            version: Version {
                family: 0,
                model: 0,
                stepping: 0,
            },
            physical_cores,
            logical_cores,
            frequency: Frequency::default(),
            cache_sizes: [None; 4],
            features,
            microarch: Some(microarch),
            hypervisor: None,
            peak_flops: None,
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cpu::uarch::Microarch;

        #[test]
        fn test_known_families() {
            // M1 — CPUFAMILY_ARM_FIRESTORM_ICESTORM
            assert!(matches!(classify_family(0x1b588bb3), Some(("M1", Microarch::AppleM1))));
            // M2 — CPUFAMILY_ARM_BLIZZARD_AVALANCHE
            assert!(matches!(classify_family(0xda33d83d), Some(("M2", Microarch::AppleM2))));
            // M3 — CPUFAMILY_ARM_EVEREST_SAWTOOTH
            assert!(matches!(classify_family(0x8765edea), Some(("M3", Microarch::AppleM3))));
            // M4 — CPUFAMILY_ARM_COLL (0x17D5B93A — the user's chip)
            assert!(matches!(classify_family(0x17d5b93a), Some(("M4", Microarch::AppleM4))));
            // Unknown family → None (no panic)
            assert!(classify_family(0xdeadbeef).is_none());
        }

        #[test]
        fn test_m4_pro_variant() {
            // M4 Pro 14-core: 10 P-cores + 4 E-cores
            assert_eq!(chip_variant("M4", 10, 4), " Pro");
            // M4 base: 4 P-cores + 6 E-cores
            assert_eq!(chip_variant("M4", 4, 6), "");
        }

        #[test]
        fn test_m1_variants() {
            assert_eq!(chip_variant("M1", 4, 4), ""); // M1
            assert_eq!(chip_variant("M1", 8, 2), " Pro"); // M1 Pro 10-core
            assert_eq!(chip_variant("M1", 8, 4), " Max"); // M1 Max
        }
    }
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cpu_runs() {
        let info = detect_cpu().unwrap();
        // On macOS Apple Silicon the vendor will be Apple; on other ARM it is ARM.
        assert!(
            matches!(info.vendor, Vendor::ARM | Vendor::Apple),
            "Expected ARM or Apple vendor, got {:?}",
            info.vendor
        );
        assert!(!info.brand_string.is_empty());
        assert!(info.logical_cores > 0);
        assert!(info.physical_cores > 0);
    }

    #[test]
    #[cfg(all(target_os = "macos", feature = "macos"))]
    fn test_apple_silicon_detected() {
        let info = detect_cpu().unwrap();
        assert_eq!(info.vendor, Vendor::Apple, "On macOS aarch64, vendor must be Apple");
        assert!(info.microarch.is_some(), "Apple Silicon microarch must be detected");
        println!(
            "Detected: {} ({:?}) — {}P + {}E cores",
            info.brand_string, info.microarch, info.physical_cores, info.logical_cores,
        );
    }
}
