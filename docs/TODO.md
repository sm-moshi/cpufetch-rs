# ✅ TODO: cpufetch-rs Porting Checklist (Detailed)

This document tracks the complete set of tasks to rewrite the original C `cpufetch` in idiomatic Rust (non-async). The goal is to achieve 100% feature parity with the original while adhering to modern Rust practices, modular design, and high testability.

---

## 🧱 Project Scaffold & Environment

- [x] **Initialize Project Structure**
  - Run `cargo new cpufetch-rs --bin` to create the basic Cargo structure.
  - Create additional directories:
    - `src/cpu`
    - `src/arch`
    - `src/printer`
    - `src/cli`
    - `src/utils`
    - `tests/fixtures`
    - `logos`
    - `.github/workflows`
- [~] **Configuration & Tooling**
  - Setup `.gitignore` to ignore `target/` and temporary files.
  - Configure Cargo with a `.cargo/config.toml` (if needed for cross-compilation or custom settings).
  - Add GitHub Actions workflows for CI (build, cargo fmt, clippy, test).
- [x] **Core Files Setup**
  - Create stub files: `src/main.rs`, `src/lib.rs`, `src/error.rs`, `src/config.rs`.

---

## 🧠 CPU Information Module (`src/cpu/`)

- [x] **Define Data Structures**
  - In `info.rs`, define a `CpuInfo` struct with fields for:
    - Vendor (as an enum or string)
    - Brand/model string
    - Family, model, stepping (parsed from CPUID)
    - Core counts: physical cores, logical threads
    - Frequency information (base, max)
    - Feature flags (as a vector or bitflags)
- [x] **CPUID Parsing Implementation**
  - In `cpuid.rs`:
    - Use safe Rust wrappers or call an existing crate (e.g., `raw-cpuid`) to gather raw CPUID values.
    - Implement functions to extract vendor ID, max CPUID levels, and raw register outputs.
    - Support cross-platform implementations with proper architecture-specific gates
    - Handle conditional compilation for different architectures
- [x] **Feature Flag Extraction**
  - In `flags.rs`:
    - Define a set of CPU features (SSE, SSE2, AVX, AVX2, FMA, etc.) using an enum or bitflags.
    - Implement a function to map raw CPUID data to these flags.
    - Add ARM-specific features for cross-platform support
- [ ] **Frequency Measurement**
  - In `frequency.rs`:
    - Design functions to parse CPU frequency information.
    - Support parsing from OS-specific sources (e.g., `/proc/cpuinfo` on Linux) or use CPUID data if available.
    - Consider separate functions for base frequency and turbo/boost frequencies.
    - Add platform-specific implementations for different architectures
    - Implement proper cross-platform support with appropriate error handling

---

## 🧬 Architecture Detection Module (`src/arch/`)

- [x] **Enum Definition**
  - In a central file (e.g., `mod.rs` or `common.rs`), define an `Architecture` enum listing supported architectures (e.g., `X86_64`, `AArch64`, etc.).
- [x] **Platform-Specific Detection**
  - In `x86_64.rs`:
    - Implement functions to check for CPUID support and other x86-specific logic.
  - In `aarch64.rs`:
    - Implement detection based on system registers or OS files (e.g., `/proc/cpuinfo` for MIDR parsing).
- [x] **Unified Detection API**
  - In `common.rs`:
    - Create a function like `detect_architecture()` that determines the architecture and returns the appropriate enum variant.
    - Optionally log or warn if the architecture is unsupported.

---

## 🎨 ASCII Art & Layout Engine (`src/printer/`)

- [ ] **Logo & ASCII Art Management**
  - In `ascii.rs`:
    - Define constant strings for each vendor logo (Intel, AMD, Apple, etc.) or load from files in the `logos/` directory.
  - In `logo.rs`:
    - Implement functions to select the proper logo based on CPU vendor.
    - Support options for `--logo-only`, `--no-logo`, or different logo sizes.
- [ ] **Text Layout & Formatting**
  - In `layout.rs`:
    - Build functions to align labels and values in columns.
    - Handle edge cases when terminal width is limited.
    - Support both colored and plain-text outputs.
- [ ] **Printer Integration**
  - In `mod.rs`:
    - Re-export functions to render the final output.
    - Create a central function that receives a `CpuInfo` and prints the complete, formatted output.

---

## 💬 CLI Interface Module (`src/cli/`)

- [ ] **CLI Options with Clap**
  - In `args.rs`:
    - Define the CLI arguments using the `clap` crate.
    - Support flags:
      - `--logo-only`: Print only the logo.
      - `--no-logo`: Omit the logo.
      - `--ascii`: Force ASCII output (no color/unicode).
      - `--json`: Output in JSON format.
      - Standard `--help` and `--version`.
  - In `mod.rs`:
    - Create functions to parse the CLI options and return a configuration struct.
- [ ] **Integrate CLI with Main**
  - Ensure `main.rs` calls the CLI parser, and passes flags to the printer and CPU modules accordingly.

---

## ⚙️ Utility Functions (`src/utils/`)

- [ ] **Formatting Helpers**
  - In `formatting.rs`:
    - Write helper functions for string padding, trimming, and alignment.
- [ ] **Platform-Specific Utilities**
  - In `platform.rs`:
    - Implement functions to fetch system details (cache sizes, thread counts, etc.) that might differ per platform.
- [ ] **Module Exports**
  - In `mod.rs`:
    - Re-export common utilities for easier imports in other modules.

---

## 🧪 Testing

- [x] **Unit Testing**
  - Write unit tests for:
    - CPUID parsing and data extraction (simulate input data).
    - Feature flag extraction.
    - Architecture detection logic.
    - Formatting and layout functions.
- [x] **Integration Testing**
  - In `tests/integration.rs`:
    - Write tests that run the complete CPU detection
    - Test architecture-specific implementations
    - Verify proper handling of cross-platform code
- [x] **Cross-Platform Testing**
  - Implemented cross-platform tests for both ARM and x86_64 architectures
  - Added proper error handling for unsupported features

---

## 📝 Manpage Creation

- [ ] **Manpage Conversion**
  - Convert the original `cpufetch.1` (roff format) into a Markdown version or update it for the Rust version.
  - Instruct Cargo or CI to include the manpage in release artifacts.

---

## 📦 Final Integration & Validation

- [ ] **Main Integration in `main.rs`**
  - Combine CLI parsing, CPU info gathering, architecture detection, and printer output.
  - Ensure error handling is robust; all errors should be printed to stderr.
- [ ] **Feature Parity Verification**
  - Run the tool on various hardware (or simulated data) to ensure output matches the original `cpufetch` (including fields like vendor, cores, frequency, flags, cache sizes, etc.).
- [ ] **Documentation & Examples**
  - Update `README.md` with usage examples.
  - Ensure the repository includes detailed documentation on building, testing, and contributing.

---

## 💡 Optional Enhancements (Post-100% Parity)

- [ ] Refactor code for further modularity or performance optimizations.
- [ ] Add additional output formats (e.g., YAML) if desired.
- [ ] Package binaries for multiple platforms.

---
