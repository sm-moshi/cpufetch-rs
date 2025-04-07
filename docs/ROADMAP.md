# 🗺️ cpufetch-rs Roadmap

This document outlines the detailed, step-by-step plan for rewriting the original C `cpufetch` as an idiomatic, synchronous Rust application named `cpufetch-rs`. The roadmap is organized into phases, with each phase addressing a group of related tasks and milestones. The ultimate goal is to reach 100% feature parity with the original while improving code structure, maintainability, and testability.

---

## Phase 1: Project Setup & Core Scaffold ✅

**Objective:** Establish a robust project foundation, configure development tools, and set up continuous integration.

- **Initialize Project Structure** ✅
  - Create the base Cargo project using `cargo new cpufetch-rs --bin`. This initializes the Cargo.toml, a default `src/main.rs`, and the basic project layout.
  - Manually add additional directories that will organize our modules:
    - `src/cpu` – for CPU-specific logic and CPUID parsing.
    - `src/arch` – for handling architecture and platform-specific detection.
    - `src/printer` – for output formatting, ASCII art, and terminal layout.
    - `src/cli` – for command-line parsing and configuration.
    - `src/utils` – for utility functions (e.g., string formatting, OS helpers).
    - `tests/fixtures` – to store test data and sample outputs.
    - `logos` – for static vendor logos.
    - `.github/workflows` – for GitHub Actions CI workflows.
- **Configuration & Tooling** ✅
  - Configure a `.gitignore` to exclude `target/`, build artifacts, and temporary files.
  - Optionally, create a `.cargo/config.toml` for target-specific settings or custom build configurations.
  - Set up GitHub Actions workflows to run:
    - Build checks.
    - Code formatting (using `cargo fmt`).
    - Linting (with `cargo clippy`).
    - Unit and integration tests.
- **Core File Initialization** ✅
  - Create and stub key files: `src/main.rs` (entry point), `src/lib.rs` (module exports), `src/error.rs` (custom error types), and `src/config.rs` (configuration parsing and defaults).

---

## Phase 2: CPU Information Module (`src/cpu/`) ⏳

**Objective:** Develop the module responsible for collecting and parsing CPU information.

- **Define Data Structures** ✅
  - In `info.rs`, design the `CpuInfo` struct to capture:
    - **Vendor:** Represented as a string or an enum.
    - **Brand/Model String:** Full name of the CPU.
    - **Family, Model, Stepping:** Numeric identifiers parsed from CPUID data.
    - **Core and Thread Counts:** Number of physical cores and logical threads.
    - **Frequency Data:** Base and maximum (boost) frequencies.
    - **Feature Flags:** A collection of supported CPU features (SSE, AVX, FMA, etc.).
- **Implement CPUID Parsing** ✅
  - In `cpuid.rs`, develop safe Rust functions (or wrap an existing crate like `raw-cpuid`) to:
    - Retrieve raw CPUID values.
    - Parse out key details such as vendor, maximum supported CPUID levels, and register values.
    - Extract additional information necessary for feature detection.
    - Implement proper cross-platform compatibility with architecture-specific gates
    - Handle conditional compilation for different CPU architectures
- **Feature Flag Extraction** ✅
  - In `flags.rs`, map the raw CPUID values to a set of defined feature flags.
    - Consider using the Rust `bitflags` crate to create a clear, type-safe set of flags.
    - Implement helper functions that check for the presence of specific features.
    - Support both x86_64 and ARM architectures with appropriate feature sets
- **Frequency Measurement** ⏳
  - In `frequency.rs`, implement methods to obtain CPU frequency information:
    - Use available CPUID leaves if supported.
    - Implement fallback methods (e.g., parsing `/proc/cpuinfo` on Linux) if necessary.
    - Separate functions for retrieving base frequency and turbo/boost frequency.
    - Add platform-specific implementations (Linux, macOS, Windows)
    - Support both x86_64 and ARM architectures through appropriate APIs

---

## Phase 3: Architecture Detection Module (`src/arch/`) ✅

**Objective:** Abstract platform- and architecture-specific differences in CPU detection.

- **Enum Definition** ✅
  - Define an `Architecture` enum (e.g., `X86_64`, `AArch64`) that represents supported architectures.
- **Platform-Specific Detection** ✅
  - In `x86_64.rs`:
    - Implement functions that utilize CPUID data for architecture-specific details on x86 systems.
    - Handle quirks or variations between different x86 CPUs (e.g., Intel vs. AMD).
  - In `aarch64.rs`:
    - Implement detection methods based on ARM-specific system registers or OS-exposed files (e.g., parsing MIDR from `/proc/cpuinfo`).
- **Unified Detection API** ✅
  - In `common.rs` (or a central module), provide a unified API function, for example, `detect_architecture()`, that automatically selects and returns the appropriate `Architecture` variant.
  - Include logging or error handling if the architecture is unsupported.

---

## Phase 4: ASCII Art & Output Printer Module (`src/printer/`) 🔄

**Objective:** Develop the rendering engine to display CPU information in an attractive, formatted manner.

- **ASCII Art Storage & Logo Selection**
  - In `ascii.rs`, store constant string representations of vendor logos (e.g., for Intel, AMD, Apple) or load them dynamically from the `logos` folder.
  - In `logo.rs`, implement logic to choose the correct logo based on the CPU vendor and possibly screen width.
- **Text Layout & Formatting**
  - In `layout.rs`, implement functions to:
    - Align field labels and values into a neat, column-based layout.
    - Adjust the layout based on terminal width, handling edge cases where space is limited.
    - Provide fallback modes that simplify output if the complete logo or formatting does not fit.
- **Printer Integration**
  - In the printer module's `mod.rs`, re-export functions to produce the final formatted output.
  - Develop a central function that takes a `CpuInfo` instance and the CLI configuration, and prints the output with proper formatting, color (using the `colored` crate if enabled), and alignment.

---

## Phase 5: CLI Interface & Configuration Module (`src/cli/`) 🔄

**Objective:** Build a robust CLI interface to configure application behavior at runtime.

- **Argument Parsing with Clap**
  - In `args.rs`, set up a comprehensive argument parser using the `clap` crate.
  - Define the following options and flags:
    - `--logo-only`: Display only the ASCII logo.
    - `--no-logo`: Suppress the logo entirely.
    - `--ascii`: Force output to be in plain ASCII (disable any color or Unicode formatting).
    - `--json`: Output the CPU information in JSON format.
    - Standard options: `--help`, `--version`, etc.
- **Configuration Struct**
  - In `mod.rs` of the CLI module, create a configuration struct that aggregates all parsed CLI options.
  - Ensure that the configuration is passed to the main application and influences behavior across CPU detection and printer modules.
- **Integration with Main**
  - Ensure `main.rs` calls the CLI parser and correctly forwards the resulting configuration to the other modules.

---

## Phase 6: Utility Functions Module (`src/utils/`) 🔄

**Objective:** Create shared helper functions that aid in string manipulation, formatting, and platform-specific inquiries.

- **Formatting Helpers**
  - In `formatting.rs`, implement functions for:
    - Padding strings to a given width.
    - Trimming and aligning text.
    - General string manipulation required by the printer.
- **Platform-Specific Utilities**
  - In `platform.rs`, implement functions to gather additional system details that may not be available via CPUID (e.g., cache sizes, system thread counts) using OS-specific techniques.
- **Module Consolidation**
  - In `mod.rs`, re-export these utility functions so that they can be easily imported by other modules.

---

## Phase 7: Testing & Validation ⏳

**Objective:** Ensure correctness and consistency across all modules with thorough testing.

- **Unit Tests** ✅
  - Develop unit tests for each module:
    - **CPUID Parsing:** Simulate various CPUID outputs and validate that the correct values are parsed.
    - **Feature Flags:** Test mapping of raw data to feature flag enums/bitflags.
    - **Architecture Detection:** Verify that the correct `Architecture` variant is returned for simulated inputs.
    - **Layout Functions:** Test alignment, padding, and string manipulation functions.
    - **CLI Parsing:** Validate that various combinations of command-line arguments produce the correct configuration.
- **Integration Tests** ✅
  - In `tests/integration.rs`, write end-to-end tests that:
    - Run the complete CPU detection functionality
    - Test architecture-specific implementations
    - Verify proper cross-platform behavior
- **Cross-Platform Verification** ⏳
  - Run the tests across multiple environments (Linux, macOS, and if possible Windows) to ensure consistent behavior.
  - Implemented proper testing for ARM and x86_64 architectures
  - Need to extend testing to cover frequency detection

---

## Phase 8: Manpage & Documentation 🔄

**Objective:** Update and extend documentation for user guidance and contributor onboarding.

- **Manpage Creation**
  - Convert the original `cpufetch.1` manpage (roff format) into an updated version.
  - Alternatively, create a Markdown version that can be converted to roff during packaging.
  - Include the manpage in the final release artifacts.
- **Documentation Update**
  - Update the `README.md` with:
    - Installation instructions.
    - Usage examples for various CLI options.
    - Explanation of the output fields and their meanings.
  - Create or update `CONTRIBUTING.md` to detail coding guidelines, commit standards, and how to run tests.

---

## Phase 9: Final Integration & Feature Parity Verification 🔄

**Objective:** Tie all components together and ensure that cpufetch-rs replicates all features of the original cpufetch.

- **Main Application Integration**
  - In `main.rs`, integrate the CLI parsing, CPU info gathering, architecture detection, and output printing.
  - Ensure errors are handled gracefully, with clear messages to stderr.
- **Feature Parity Checks**
  - Compare the output of cpufetch-rs against the original cpufetch:
    - Verify that vendor, model, microarchitecture, core counts, frequency, feature flags, and cache information are correctly displayed.
    - Confirm that CLI options such as `--json`, `--logo-only`, and `--no-logo` behave as expected.
- **User Acceptance Testing**
  - Test the application on real hardware or with simulated environments.
  - Adjust layout, spacing, and color schemes to match or improve upon the original design.

---

## Phase 10: Optional Post-Parity Enhancements 🔄

**Objective:** Once full feature parity is reached, consider optional improvements and additional features.

- **Code Refactoring**
  - Review and refactor the code for improved clarity, modularity, and performance.
  - Document any complex logic with inline comments and update the overall documentation.
- **Additional Output Formats**
  - Explore support for other output formats (YAML, TOML) if beneficial.
- **Packaging & Distribution**
  - Set up build scripts to create pre-built binaries for multiple platforms.
  - Prepare the repository for publishing on Crates.io.
  - Develop release notes and update changelogs.

---

## Current Progress Summary (April 2024)

- **Completed:**
  - ✅ Project scaffolding and basic structure
  - ✅ Core CPU information data structures
  - ✅ CPUID parsing module with architecture-specific gates
  - ✅ Feature flag detection for both x86_64 and ARM
  - ✅ Architecture detection module with cross-platform support
  - ✅ Unit and integration testing for core components
  - ✅ Cross-platform support with proper error handling

- **In Progress:**
  - ⏳ Frequency detection implementation (next immediate task)
  - ⏳ Testing and validation, particularly for platform-specific components

- **Coming Soon:**
  - 🔄 ASCII art and printer module
  - 🔄 CLI interface and configuration
  - 🔄 Utility functions
  - 🔄 Documentation and manpage
  - 🔄 Final integration and feature parity verification

---
