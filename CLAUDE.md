# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Rust rewrite of cpufetch — a cross-platform CLI tool and library for fetching detailed CPU information (vendor, model, cores, cache, frequency, feature flags). Supports x86_64 and aarch64 architectures with platform-specific backends for Linux, macOS, and Windows.

- **Binary**: `cpufetch` (via `src/main.rs`)
- **Library crate**: `cpufetch_rs` (via `src/lib.rs`)
- **Edition**: 2024
- **Status**: Early development (v0.0.1)

## Commands

```bash
cargo build                          # build (default features = full)
cargo build --no-default-features    # minimal build (core CPU detection only)
cargo build --features cli,json      # selective features
cargo test --all-features            # run all tests
cargo test --test cpu_detect         # single test file
cargo +nightly fmt --all --check     # format check (CI uses nightly rustfmt)
cargo +nightly fmt --all             # auto-format
cargo +beta clippy --all-targets --all-features -- -D warnings  # lint (CI uses beta clippy)
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features   # build docs
```

## Feature Flags

Dependencies are extensively feature-gated. `default = ["full"]` enables everything.

| Feature | What it enables | Key deps |
|---------|----------------|----------|
| `cli` | Command-line argument parsing | clap, anyhow |
| `display` | Coloured terminal output, ASCII art | colored, crossterm, textwrap, regex |
| `frequency` | CPU frequency detection | sysinfo, sys-info, platforms |
| `json` | JSON output | serde_json |
| `config` | TOML config file | toml |
| `linux` / `windows` / `macos` | Platform-specific backends | procfs / windows+wmi / sysctl |

When adding new dependencies, make them `optional = true` and gate behind a feature unless they're core CPU detection.

## Architecture

The crate is split into a library (`lib.rs`) and a binary (`main.rs`). The binary has three `#[cfg]`-gated code paths depending on which features are enabled (no-cli, cli+display, cli-only).

**Core flow**: `arch::*::detect()` → `CpuInfo` struct → `printer::*` output

- `cpu/info.rs` — Central `CpuInfo`, `Vendor`, `Frequency`, `Version` types (always compiled)
- `cpu/flags.rs` — CPU feature flag detection via `bitflags`
- `cpu/frequency.rs` — Frequency detection (feature-gated)
- `arch/x86_64/` — CPUID-based detection via `raw-cpuid`
- `arch/aarch64/` — ARM detection via platform-specific syscalls
- `printer/` — ASCII art rendering, layout, JSON output (feature-gated)
- `cli/` — Clap argument parsing (feature-gated)
- `error.rs` — `thiserror`-based error enum with feature-gated variants

## Code Conventions

- **Synchronous only** — no async, no concurrency
- **No unsafe** unless documented with `// SAFETY:` comment
- **Error handling**: `thiserror` for library errors, `anyhow` for CLI (optional)
- **Clippy**: pedantic level enabled globally via `.cargo/config.toml` (`clippy::all` + `clippy::pedantic`). Allowed: `module_name_repetitions`, `too_many_lines`
- **Formatting**: rustfmt with edition 2024, 120 char max width, Unix newlines
- **Documentation language**: British English
- **Doc comments**: required on public items, prefer "why-first" style

## Git Workflow

Uses **Gitflow**: `main` (production) + `develop` (integration). Feature branches from `develop`, releases via `release/*`, hotfixes via `hotfix/*`.

- Commit format: `🦀 type(scope): description` (emoji prefix required)
- Never commit directly to `main` or `develop`

## CI

GitHub Actions on push to `main`/`develop` and PRs:
- **test**: `cargo +beta test --all-features`
- **rustfmt**: `cargo +nightly fmt --all --check`
- **clippy**: `cargo +beta clippy --all-targets --all-features -- -D warnings`
- **docs**: `cargo doc` with `-D warnings`

CD pipeline triggers on version tags (`v*`), builds multi-platform binaries (macOS x86_64/arm64, Linux x86_64/arm64/i686, Windows x86_64) using `cross`, publishes to GitHub Releases and crates.io.
