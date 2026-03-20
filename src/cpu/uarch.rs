//! CPU microarchitecture detection.
//!
//! Lookup tables mapping (vendor, family, model) → microarchitecture name and process node.

use crate::cpu::info::Vendor;
use serde::{Deserialize, Serialize};
use std::fmt;

/// CPU microarchitecture identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Microarch {
    // Intel (NetBurst)
    Willamette,
    Northwood,
    Prescott,
    // Intel (Core)
    Nehalem,
    Westmere,
    SandyBridge,
    IvyBridge,
    Haswell,
    Broadwell,
    Skylake,
    KabyLake,
    CometLake,
    CannonLake,
    IceLake,
    TigerLake,
    AlderLake,
    RaptorLake,
    MeteorLake,
    SapphireRapids,
    GraniteRapids,
    // AMD
    K8,
    K10,
    Bobcat,
    Bulldozer,
    Piledriver,
    Steamroller,
    Excavator,
    Jaguar,
    Zen,
    ZenPlus,
    Zen2,
    Hygon,
    Zen3,
    Zen3Plus,
    Zen4,
    Zen5,
    // Apple Silicon (macOS aarch64)
    AppleM1,
    AppleM2,
    AppleM3,
    AppleM4,
}

impl Microarch {
    /// Process technology in nm, if known
    pub fn process_nm(&self) -> Option<u32> {
        match self {
            // Intel
            Microarch::Willamette => Some(180),
            Microarch::Northwood => Some(130),
            Microarch::Prescott => Some(90),
            Microarch::Nehalem => Some(45),
            Microarch::Westmere => Some(32),
            Microarch::SandyBridge => Some(32),
            Microarch::IvyBridge => Some(22),
            Microarch::Haswell => Some(22),
            Microarch::Broadwell => Some(14),
            Microarch::Skylake => Some(14),
            Microarch::KabyLake => Some(14),
            Microarch::CometLake => Some(14),
            Microarch::CannonLake => Some(10),
            Microarch::IceLake => Some(10),
            Microarch::TigerLake => Some(10),
            Microarch::AlderLake => Some(10),
            Microarch::RaptorLake => Some(10),
            Microarch::MeteorLake => Some(4),
            Microarch::SapphireRapids => Some(10),
            Microarch::GraniteRapids => Some(3),
            // AMD
            Microarch::K8 => Some(130),
            Microarch::K10 => Some(65),
            Microarch::Bobcat => Some(40),
            Microarch::Bulldozer => Some(32),
            Microarch::Piledriver => Some(32),
            Microarch::Steamroller => Some(28),
            Microarch::Excavator => Some(28),
            Microarch::Jaguar => Some(28),
            Microarch::Zen => Some(14),
            Microarch::ZenPlus => Some(12),
            Microarch::Zen2 => Some(7),
            Microarch::Hygon => Some(14),
            Microarch::Zen3 => Some(7),
            Microarch::Zen3Plus => Some(6),
            Microarch::Zen4 => Some(5),
            Microarch::Zen5 => Some(4),
            // Apple Silicon (TSMC process nodes)
            Microarch::AppleM1 => Some(5), // TSMC N5
            Microarch::AppleM2 => Some(5), // TSMC N4P (enhanced 5 nm)
            Microarch::AppleM3 => Some(3), // TSMC N3E
            Microarch::AppleM4 => Some(3), // TSMC N3E
        }
    }

    /// Human-readable name for display
    pub fn name(&self) -> &'static str {
        match self {
            // Intel
            Microarch::Willamette => "Willamette",
            Microarch::Northwood => "Northwood",
            Microarch::Prescott => "Prescott",
            Microarch::Nehalem => "Nehalem",
            Microarch::Westmere => "Westmere",
            Microarch::SandyBridge => "Sandy Bridge",
            Microarch::IvyBridge => "Ivy Bridge",
            Microarch::Haswell => "Haswell",
            Microarch::Broadwell => "Broadwell",
            Microarch::Skylake => "Skylake",
            Microarch::KabyLake => "Kaby Lake",
            Microarch::CometLake => "Comet Lake",
            Microarch::CannonLake => "Cannon Lake",
            Microarch::IceLake => "Ice Lake",
            Microarch::TigerLake => "Tiger Lake",
            Microarch::AlderLake => "Alder Lake",
            Microarch::RaptorLake => "Raptor Lake",
            Microarch::MeteorLake => "Meteor Lake",
            Microarch::SapphireRapids => "Sapphire Rapids",
            Microarch::GraniteRapids => "Granite Rapids",
            // AMD
            Microarch::K8 => "K8",
            Microarch::K10 => "K10",
            Microarch::Bobcat => "Bobcat",
            Microarch::Bulldozer => "Bulldozer",
            Microarch::Piledriver => "Piledriver",
            Microarch::Steamroller => "Steamroller",
            Microarch::Excavator => "Excavator",
            Microarch::Jaguar => "Jaguar",
            Microarch::Zen => "Zen",
            Microarch::ZenPlus => "Zen+",
            Microarch::Zen2 => "Zen 2",
            Microarch::Hygon => "Hygon",
            Microarch::Zen3 => "Zen 3",
            Microarch::Zen3Plus => "Zen 3+",
            Microarch::Zen4 => "Zen 4",
            Microarch::Zen5 => "Zen 5",
            // Apple Silicon
            Microarch::AppleM1 => "Apple M1",
            Microarch::AppleM2 => "Apple M2",
            Microarch::AppleM3 => "Apple M3",
            Microarch::AppleM4 => "Apple M4",
        }
    }
}

impl fmt::Display for Microarch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Detect microarchitecture from vendor, family, and model IDs.
///
/// The `family` value passed here should already include the extended family
/// (as computed in `x86_64.rs`).
pub fn detect_uarch(vendor: &Vendor, family: u8, model: u8) -> Option<Microarch> {
    match vendor {
        Vendor::Intel => detect_intel_uarch(family, model),
        Vendor::AMD => detect_amd_uarch(family, model),
        _ => None,
    }
}

fn detect_intel_uarch(family: u8, model: u8) -> Option<Microarch> {
    match family {
        // Pentium 4 / NetBurst (family 15)
        15 => match model {
            0 | 1 => Some(Microarch::Willamette),
            2 => Some(Microarch::Northwood),
            3 | 4 | 6 => Some(Microarch::Prescott),
            _ => None,
        },
        // Core and later (family 6 — the vast majority of modern Intel)
        6 => match model {
            // Nehalem
            0x1A | 0x1E | 0x1F | 0x2E => Some(Microarch::Nehalem),
            // Westmere
            0x25 | 0x2C | 0x2F => Some(Microarch::Westmere),
            // Sandy Bridge
            0x2A | 0x2D => Some(Microarch::SandyBridge),
            // Ivy Bridge
            0x3A | 0x3E => Some(Microarch::IvyBridge),
            // Haswell
            0x3C | 0x3F | 0x45 | 0x46 => Some(Microarch::Haswell),
            // Broadwell
            0x3D | 0x47 | 0x4F | 0x56 => Some(Microarch::Broadwell),
            // Skylake / Cascade Lake / Cooper Lake / Skylake-SP
            0x4E | 0x5E | 0x55 => Some(Microarch::Skylake),
            // Kaby Lake / Whiskey Lake / Amber Lake / Coffee Lake
            0x8E | 0x9E => Some(Microarch::KabyLake),
            // Comet Lake
            0xA5 | 0xA6 => Some(Microarch::CometLake),
            // Cannon Lake
            0x66 => Some(Microarch::CannonLake),
            // Ice Lake (client + server)
            0x7D | 0x7E | 0x6A | 0x6C => Some(Microarch::IceLake),
            // Tiger Lake
            0x8C | 0x8D => Some(Microarch::TigerLake),
            // Alder Lake
            0x97 | 0x9A | 0xBE => Some(Microarch::AlderLake),
            // Raptor Lake (v1.06: 0xB7, 0xBF, 0xBA added)
            0xB7 | 0xBF | 0xBA => Some(Microarch::RaptorLake),
            // Meteor Lake
            0xAA | 0xAC => Some(Microarch::MeteorLake),
            // Sapphire Rapids (v1.06 addition)
            0xCF | 0x8F => Some(Microarch::SapphireRapids),
            // Granite Rapids
            0xAD | 0xAE => Some(Microarch::GraniteRapids),
            _ => None,
        },
        _ => None,
    }
}

fn detect_amd_uarch(family: u8, model: u8) -> Option<Microarch> {
    match family {
        // K8 — Athlon 64, Opteron
        15 => Some(Microarch::K8),
        // K10 — Phenom, Barcelona, Shanghai, Thuban
        16 | 18 => Some(Microarch::K10),
        // Bobcat — Ontario, Zacate
        20 => Some(Microarch::Bobcat),
        // Bulldozer family (21)
        21 => match model {
            0x01 => Some(Microarch::Bulldozer),
            0x02 | 0x10..=0x1F => Some(Microarch::Piledriver),
            0x30..=0x3F => Some(Microarch::Steamroller),
            0x60..=0x7F => Some(Microarch::Excavator),
            _ => Some(Microarch::Bulldozer),
        },
        // Jaguar / Puma
        22 => Some(Microarch::Jaguar),
        // Zen, Zen+, Zen 2 (family 23 / 0x17)
        23 => match model {
            0x01 => Some(Microarch::Zen),     // Naples (EPYC 1000)
            0x08 => Some(Microarch::ZenPlus), // Pinnacle Ridge
            0x11 => Some(Microarch::Zen),     // Raven Ridge
            0x18 => Some(Microarch::ZenPlus), // Picasso
            0x20 => Some(Microarch::Zen2),    // Castle Peak (Threadripper 3000)
            0x31 => Some(Microarch::Zen2),    // Rome (EPYC 7002)
            0x47 => Some(Microarch::Zen2),    // Renoir
            0x60 => Some(Microarch::Zen2),    // Renoir
            0x68 => Some(Microarch::Zen2),    // Lucienne
            0x71 => Some(Microarch::Zen2),    // Matisse
            0x90 => Some(Microarch::Zen2),    // Van Gogh
            _ => Some(Microarch::Zen),
        },
        // Hygon — Chinese x86 clone (v1.05 addition, family 24 / 0x18)
        24 => Some(Microarch::Hygon),
        // Zen 3, Zen 3+, Zen 4 (family 25 / 0x19)
        25 => match model {
            0x01 => Some(Microarch::Zen3),     // Milan (EPYC 7003)
            0x08 => Some(Microarch::Zen3),     // Chagall (Threadripper Pro 5000WX)
            0x21 => Some(Microarch::Zen3),     // Vermeer (Ryzen 5000)
            0x40 => Some(Microarch::Zen3Plus), // Rembrandt (Ryzen 6000)
            0x44 => Some(Microarch::Zen3Plus), // Rembrandt-R
            0x50 => Some(Microarch::Zen3),     // Cezanne
            0x61 => Some(Microarch::Zen4),     // Raphael (Ryzen 7000)
            0x74 => Some(Microarch::Zen4),     // Phoenix (Ryzen 7040)
            0x78 => Some(Microarch::Zen4),     // Phoenix 2
            _ => Some(Microarch::Zen3),
        },
        // Zen 5 (v1.06 addition, family 26 / 0x1A)
        26 => Some(Microarch::Zen5),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_nehalem() {
        assert_eq!(detect_uarch(&Vendor::Intel, 6, 0x1A), Some(Microarch::Nehalem));
    }

    #[test]
    fn test_intel_raptor_lake() {
        assert_eq!(detect_uarch(&Vendor::Intel, 6, 0xB7), Some(Microarch::RaptorLake));
    }

    #[test]
    fn test_intel_sapphire_rapids() {
        assert_eq!(detect_uarch(&Vendor::Intel, 6, 0xCF), Some(Microarch::SapphireRapids));
    }

    #[test]
    fn test_amd_zen3() {
        assert_eq!(detect_uarch(&Vendor::AMD, 25, 0x21), Some(Microarch::Zen3));
    }

    #[test]
    fn test_amd_zen5() {
        assert_eq!(detect_uarch(&Vendor::AMD, 26, 0), Some(Microarch::Zen5));
    }

    #[test]
    fn test_amd_hygon() {
        assert_eq!(detect_uarch(&Vendor::AMD, 24, 0), Some(Microarch::Hygon));
    }

    #[test]
    fn test_microarch_display() {
        assert_eq!(Microarch::Zen3.to_string(), "Zen 3");
        assert_eq!(Microarch::SandyBridge.to_string(), "Sandy Bridge");
        assert_eq!(Microarch::ZenPlus.to_string(), "Zen+");
    }

    #[test]
    fn test_process_nm() {
        assert_eq!(Microarch::Zen3.process_nm(), Some(7));
        assert_eq!(Microarch::Zen5.process_nm(), Some(4));
        assert_eq!(Microarch::Haswell.process_nm(), Some(22));
        assert_eq!(Microarch::RaptorLake.process_nm(), Some(10));
    }

    #[test]
    fn test_unknown_vendor_returns_none() {
        assert_eq!(detect_uarch(&Vendor::ARM, 6, 0x97), None);
    }
}
