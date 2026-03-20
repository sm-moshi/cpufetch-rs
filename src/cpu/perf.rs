//! Peak CPU performance (GFLOP/s) calculation.
//!
//! Computes theoretical peak floating-point throughput based on core count,
//! clock frequency, SIMD width, and FMA support.

/// Calculate theoretical peak double-precision GFLOP/s.
///
/// Formula: `cores × freq_GHz × simd_width_dp × fma_factor × vpu_count`
///
/// Where:
/// - `simd_width_dp` = doubles per cycle per VPU (AVX-512→8, AVX/AVX2→4, SSE→2, scalar→1)
/// - `fma_factor` = 2 if FMA3 is supported (multiply + add in one cycle), else 1
/// - `vpu_count` = vector processing units per core (1 for most desktop/server CPUs)
///
/// Returns `None` if frequency is unknown or zero.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[must_use]
pub fn calculate_peak_flops(
    physical_cores: u32,
    freq_max_mhz: Option<f64>,
    freq_base_mhz: Option<f64>,
    features: crate::cpu::X86Features,
) -> Option<f64> {
    use crate::cpu::X86Features;

    let mhz = freq_max_mhz.or(freq_base_mhz)?;
    if mhz <= 0.0 {
        return None;
    }

    let clock_ghz = mhz / 1000.0;
    let cores = f64::from(physical_cores);

    // Double-precision SIMD width: doubles per SIMD register
    let simd_width_dp = if features.contains(X86Features::AVX512F) {
        8.0 // 512 bits / 64 bits per double
    } else if features.contains(X86Features::AVX2) || features.contains(X86Features::AVX) {
        4.0 // 256 bits / 64 bits per double
    } else {
        2.0 // SSE: 128 bits / 64 bits per double
    };

    // FMA doubles effective throughput (fused multiply-add counts as 2 FLOP/cycle)
    let fma_factor = if features.contains(X86Features::FMA) { 2.0 } else { 1.0 };

    // One VPU per core for mainstream CPUs; some Skylake-X Xeons have 2
    let vpu_count = 1.0_f64;

    let gflops = cores * clock_ghz * simd_width_dp * fma_factor * vpu_count;
    Some(gflops)
}

#[cfg(test)]
mod tests {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    use super::*;

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_flops_with_avx2_fma() {
        use crate::cpu::X86Features;

        let features = X86Features::AVX2 | X86Features::FMA;
        // 4 cores × 4.0 GHz × 4 doubles × 2 (FMA) × 1 VPU = 128 GFLOP/s
        let result = calculate_peak_flops(4, Some(4000.0), None, features);
        assert!(result.is_some());
        let flops = result.unwrap();
        assert!((flops - 128.0).abs() < 0.01, "Expected 128.0, got {flops}");
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_flops_no_frequency() {
        use crate::cpu::X86Features;

        let features = X86Features::AVX2 | X86Features::FMA;
        let result = calculate_peak_flops(4, None, None, features);
        assert!(result.is_none());
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_flops_avx512() {
        use crate::cpu::X86Features;

        let features = X86Features::AVX512F | X86Features::FMA;
        // 1 core × 1.0 GHz × 8 doubles × 2 (FMA) × 1 VPU = 16 GFLOP/s
        let result = calculate_peak_flops(1, Some(1000.0), None, features);
        assert!(result.is_some());
        let flops = result.unwrap();
        assert!((flops - 16.0).abs() < 0.01, "Expected 16.0, got {flops}");
    }
}
