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
            .map_or_else(|| "Unknown".to_string(), |v| format!("{v:.2} MHz"));
        let current = self
            .current
            .map_or_else(|| "Unknown".to_string(), |v| format!("{v:.2} MHz"));
        let max = self
            .max
            .map_or_else(|| "Unknown".to_string(), |v| format!("{v:.2} MHz"));

        write!(f, "Base: {base}, Current: {current}, Max: {max}")
    }
}

/// Detects CPU frequency using platform-specific methods
///
/// # Errors
///
/// Returns an error if frequency detection fails on the current platform.
pub fn detect_frequency() -> Result<Frequency, Error> {
    #[cfg(feature = "frequency")]
    {
        // Platform-specific implementations
        #[cfg(target_os = "linux")]
        return Ok(detect_frequency_linux());

        #[cfg(target_os = "windows")]
        return detect_frequency_windows();

        #[cfg(target_os = "macos")]
        return detect_frequency_macos();

        // Generic fallback using sysinfo
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return Ok(detect_frequency_generic());
    }

    // Default fallback for when the frequency feature is disabled
    #[cfg(not(feature = "frequency"))]
    {
        Ok(Frequency::default())
    }
}

// Platform-specific implementations
#[cfg(all(feature = "frequency", target_os = "linux"))]
fn detect_frequency_linux() -> Frequency {
    use std::fs::read_to_string;

    let mut frequency = Frequency::default();

    // Read current frequency from cpufreq sysfs
    if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        && let Ok(khz) = content.trim().parse::<f64>()
    {
        frequency.current = Some(khz / 1000.0);
    }

    // Read max frequency from cpufreq sysfs
    if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq")
        && let Ok(khz) = content.trim().parse::<f64>()
    {
        frequency.max = Some(khz / 1000.0);
    }

    // Read base frequency from cpufreq sysfs (not always present)
    if let Ok(content) = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/base_frequency")
        && let Ok(khz) = content.trim().parse::<f64>()
    {
        frequency.base = Some(khz / 1000.0);
    }

    // Fallback to sysinfo if sysfs yielded nothing
    if frequency.current.is_none() && frequency.max.is_none() && frequency.base.is_none() {
        return detect_frequency_generic();
    }

    frequency
}

#[cfg(all(feature = "frequency", target_os = "windows"))]
fn detect_frequency_windows() -> Result<Frequency, Error> {
    use serde::Deserialize;
    use sysinfo::{CpuRefreshKind, System};
    use wmi::{COMLibrary, WMIConnection};

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
        },
        Err(e) => {
            // Log the error but continue with fallback
            eprintln!("Failed to initialize COM library for WMI: {}", e);
        },
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
        return Ok(detect_frequency_generic());
    }

    Ok(frequency)
}

// Result is needed for uniformity with other platform detect_ fns called from detect_frequency.
#[cfg(all(feature = "frequency", target_os = "macos"))]
#[allow(clippy::unnecessary_wraps)]
fn detect_frequency_macos() -> Result<Frequency, Error> {
    Ok(detect_frequency_macos_inner())
}

#[cfg(all(feature = "frequency", target_os = "macos"))]
fn detect_frequency_macos_inner() -> Frequency {
    use sysctl::{CtlValue, Sysctl};

    let mut frequency = Frequency::default();

    // Try sysctl for frequency information
    // Precision loss is acceptable: CPU frequencies in Hz fit in f64 mantissa at GHz scale.
    #[allow(clippy::cast_precision_loss)]
    {
        if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency")
            && let Ok(CtlValue::S64(freq)) = ctl.value()
        {
            frequency.current = Some((freq as f64) / 1_000_000.0);
        }

        if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency_max")
            && let Ok(CtlValue::S64(freq)) = ctl.value()
        {
            frequency.max = Some((freq as f64) / 1_000_000.0);
        }

        if let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency_min")
            && let Ok(CtlValue::S64(freq)) = ctl.value()
        {
            frequency.base = Some((freq as f64) / 1_000_000.0);
        }
    }

    // Fallback to sysinfo if we couldn't get frequencies
    if frequency.current.is_none() && frequency.max.is_none() && frequency.base.is_none() {
        return detect_frequency_generic();
    }

    frequency
}

#[cfg(feature = "frequency")]
fn detect_frequency_generic() -> Frequency {
    use sysinfo::{CpuRefreshKind, System};

    let mut frequency = Frequency::default();
    let mut system = System::new();
    system.refresh_cpu_specifics(CpuRefreshKind::everything());

    if let Some(cpu) = system.cpus().first() {
        #[allow(clippy::cast_precision_loss)]
        let freq = cpu.frequency() as f64;
        frequency.current = Some(freq);
    }

    // Try to estimate base/max if we have current frequency
    if let Some(current) = frequency.current {
        // A very simplistic estimation - not accurate but provides something
        frequency.base = Some(current * 0.9);
        frequency.max = Some(current * 1.1);
    }

    frequency
}
