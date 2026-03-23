# cpufetch-rs

[![CI](https://github.com/sm-moshi/cpufetch-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/sm-moshi/cpufetch-rs/actions/workflows/ci.yml)
[![Security](https://github.com/sm-moshi/cpufetch-rs/actions/workflows/security.yml/badge.svg)](https://github.com/sm-moshi/cpufetch-rs/actions/workflows/security.yml)
[![crates.io](https://img.shields.io/crates/v/cpufetch-rs.svg)](https://crates.io/crates/cpufetch-rs)
[![docs.rs](https://docs.rs/cpufetch-rs/badge.svg)](https://docs.rs/cpufetch-rs)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](Cargo.toml)
[![License](https://img.shields.io/crates/l/cpufetch-rs.svg)](LICENSE-MIT)

A cross-platform Rust rewrite of [cpufetch](https://github.com/Dr-Noob/cpufetch) — fetches and displays detailed CPU information with colourful ASCII art.

> **Status**: Early development (v0.0.1). Core CPU detection works, display formatting is in progress.

## What it detects

- **Vendor and model** — Intel, AMD, Apple Silicon, ARM
- **Microarchitecture** — Raptor Lake, Zen 5, Firestorm, etc.
- **Core topology** — physical cores, logical threads
- **Frequencies** — base and boost clocks
- **ISA extensions** — SSE, AVX, AVX-512, FMA, NEON, AES, SHA, and more
- **Cache hierarchy** — L1i/L1d, L2, L3 sizes
- **Hypervisor detection** — KVM, VMware, Hyper-V, etc.
- **Peak performance** — theoretical GFLOP/s estimate

## Platform support

| Platform | x86_64 | aarch64 |
|----------|--------|---------|
| Linux | Full | Full |
| macOS | Full | Full (Apple Silicon) |
| Windows | Full | Planned |
| FreeBSD | Builds | Builds |

## Install

From [crates.io](https://crates.io/crates/cpufetch-rs):

```bash
cargo install cpufetch-rs
```

Or download a pre-built binary from [GitHub Releases](https://github.com/sm-moshi/cpufetch-rs/releases).

## Build from source

```bash
git clone https://github.com/sm-moshi/cpufetch-rs
cd cpufetch-rs
cargo build --release
./target/release/cpufetch
```

### Minimal build (core detection only)

```bash
cargo build --release --no-default-features
```

## Usage

```bash
cpufetch                  # Colourful output with vendor logo
cpufetch --json           # JSON output
cpufetch --no-logo        # Text only, no ASCII art
cpufetch --no-color       # Plain text, no colours
cpufetch --frequency      # Show frequency details
cpufetch --cache          # Show cache topology
cpufetch --features       # Show ISA feature flags
```

## Feature flags

All features are optional and gated behind Cargo features. `default = ["full"]` enables everything.

| Feature | What it enables |
|---------|----------------|
| `cli` | Command-line argument parsing (clap) |
| `display` | Coloured terminal output, ASCII art |
| `frequency` | CPU frequency detection |
| `json` | JSON output format |
| `config` | TOML configuration file |
| `linux` / `macos` / `windows` | Platform-specific backends |

## Development

```bash
mise install                        # Install dev tools
mise run test                       # Run tests
mise run lint                       # Clippy (pedantic)
mise run fmt                        # Format check
mise run audit                      # Security audit
```

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## Licence

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).
