//! CPU frequency detection module
//!
//! This module provides functionality for detecting CPU frequency information
//! across different platforms.

use crate::Error;
use std::fmt;

/// CPU frequency information
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Frequency {
    /// Base frequency in MHz
    pub base: Option<f64>,
    /// Current frequency in MHz
    pub current: Option<f64>,
    /// Maximum frequency in MHz (Turbo/Boost)
    pub max: Option<f64>,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = self
            .base
            .map_or_else(|| "Unknown".to_string(), |v| format!("{:.2} MHz", v));
        let current = self
            .current
            .map_or_else(|| "Unknown".to_string(), |v| format!("{:.2} MHz", v));
        let max = self
            .max
            .map_or_else(|| "Unknown".to_string(), |v| format!("{:.2} MHz", v));

        write!(f, "Base: {}, Current: {}, Max: {}", base, current, max)
    }
}

/// Detects CPU frequency using platform-specific methods
pub fn detect_frequency() -> Result<Frequency, Error> {
    #[cfg(feature = "frequency")]
    {
        // Platform-specific implementations
        #[cfg(target_os = "linux")]
        return detect_frequency_linux();

        #[cfg(target_os = "windows")]
        return detect_frequency_windows();

        #[cfg(target_os = "macos")]
        return detect_frequency_macos();

        // Generic fallback using sysinfo
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return detect_frequency_generic();
    }

    // Default fallback for when the frequency feature is disabled
    #[cfg(not(feature = "frequency"))]
    {
        Ok(Frequency::default())
    }
}

// Platform-specific implementations
#[cfg(all(feature = "frequency", target_os = "linux"))]
fn detect_frequency_linux() -> Result<Frequency, Error> {
    use procfs::CpuInfo;
    use std::fs::read_to_string;

    let mut frequency = Frequency::default();

    // Try to get base frequency from cpuinfo
    if let Ok(cpu_info) = CpuInfo::new() {
        if let Some(cpu) = cpu_info.cpus.first() {
            if let Some(cpu_mhz) = &cpu.get("cpu MHz") {
                if let Ok(mhz) = cpu_mhz.parse::<f64>() {
                    frequency.current = Some(mhz);
                }
            }

            if let Some(max_mhz) = &cpu.get("cpu max MHz") {
                if let Ok(mhz) = max_mhz.parse::<f64>() {
                    frequency.max = Some(mhz);
                }
            }

            if let Some(min_mhz) = &cpu.get("cpu min MHz") {
                if let Ok(mhz) = min_mhz.parse::<f64>() {
                    frequency.base = Some(mhz);
                }
            }
        }
    }

    // Try scaling_cur_freq if current frequency is still unknown
    if frequency.current.is_none() {
        if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            if let Ok(khz) = content.trim().parse::<f64>() {
                frequency.current = Some(khz / 1000.0);
            }
        }
    }

    // Try scaling_max_freq if max frequency is still unknown
    if frequency.max.is_none() {
        if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq") {
            if let Ok(khz) = content.trim().parse::<f64>() {
                frequency.max = Some(khz / 1000.0);
            }
        }
    }

    // If we still don't have base frequency, try cpufreq/base_frequency
    if frequency.base.is_none() {
        if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/base_frequency") {
            if let Ok(khz) = content.trim().parse::<f64>() {
                frequency.base = Some(khz / 1000.0);
            }
        }
    }

    // Fallback to sysinfo if we couldn't get frequencies
    if frequency.current.is_none() && frequency.max.is_none() && frequency.base.is_none() {
        return detect_frequency_generic();
    }

    Ok(frequency)
}

#[cfg(all(feature = "frequency", target_os = "windows"))]
fn detect_frequency_windows() -> Result<Frequency, Error> {
    use sysinfo::{CpuRefreshKind, System};
    use wmi::{COMLibrary, WMIConnection};
    use serde::Deserialize;

    // Define a structure that matches Win32_Processor WMI class
    #[derive(Deserialize, Debug)]
    struct Win32_Processor {
        CurrentClockSpeed: Option<u32>,
        MaxClockSpeed: Option<u32>,
    }

    let mut frequency = Frequency::default();

    // Try WMI access first for most accurate data
    match COMLibrary::new() {
        Ok(com_lib) => {
            if let Ok(wmi_con) = WMIConnection::new(com_lib) {
                // Query WMI for processor information
                if let Ok(processors) = wmi_con.query::<Win32_Processor>() {
                    if let Some(processor) = processors.first() {
                        // Current frequency
                        if let Some(current_speed) = processor.CurrentClockSpeed {
                            frequency.current = Some(current_speed as f64);
                        }

                        // Max frequency
                        if let Some(max_speed) = processor.MaxClockSpeed {
                            frequency.max = Some(max_speed as f64);

                            // If max is available but base isn't, estimate base as 80% of max
                            // This is a common rule of thumb for modern processors
                            if frequency.base.is_none() {
                                frequency.base = Some(max_speed as f64 * 0.8);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            // Log the error but continue with fallback
            eprintln!("Failed to initialize COM library for WMI: {}", e);
        }
    }

    // Use sysinfo as a fallback if WMI failed to provide frequency information
    if frequency.current.is_none() {
        let mut system = System::new();
        system.refresh_cpu_specifics(CpuRefreshKind::everything());

        if let Some(cpu) = system.cpus().first() {
            frequency.current = Some(cpu.frequency() as f64);
        }
    }

    // Fallback to generic method if we couldn't get any frequencies
    if frequency.current.is_none() && frequency.max.is_none() && frequency.base.is_none() {
        return detect_frequency_generic();
    }

    Ok(frequency)
}

#[cfg(all(feature = "frequency", target_os = "macos"))]
fn detect_frequency_macos() -> Result<Frequency, Error> {
    use sysctl::{CtlValue, Sysctl};

    let mut frequency = Frequency::default();

    // Try sysctl for frequency information
    if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency") {
        if let Ok(CtlValue::S64(freq)) = ctl.value() {
            frequency.current = Some((freq as f64) / 1_000_000.0);
        }
    }

    if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency_max") {
        if let Ok(CtlValue::S64(freq)) = ctl.value() {
            frequency.max = Some((freq as f64) / 1_000_000.0);
        }
    }

    if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency_min") {
        if let Ok(CtlValue::S64(freq)) = ctl.value() {
            frequency.base = Some((freq as f64) / 1_000_000.0);
        }
    }

    // Fallback to sysinfo if we couldn't get frequencies
    if frequency.current.is_none() && frequency.max.is_none() && frequency.base.is_none() {
        return detect_frequency_generic();
    }

    Ok(frequency)
}

#[cfg(feature = "frequency")]
fn detect_frequency_generic() -> Result<Frequency, Error> {
    use sysinfo::{CpuRefreshKind, System};

    let mut frequency = Frequency::default();
    let mut system = System::new();
    system.refresh_cpu_specifics(CpuRefreshKind::everything());

    if let Some(cpu) = system.cpus().first() {
        frequency.current = Some(cpu.frequency() as f64);
    }

    // Try to estimate base/max if we have current frequency
    if let Some(current) = frequency.current {
        // A very simplistic estimation - not accurate but provides something
        frequency.base = Some(current * 0.9);
        frequency.max = Some(current * 1.1);
    }

    Ok(frequency)
}
