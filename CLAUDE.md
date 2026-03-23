# CLAUDE — Runtime Instructions

This is a Rust CLI and library crate for cross-platform CPU information.

**Authoritative policy lives in [AGENTS.md](AGENTS.md).**
If this file conflicts with `AGENTS.md`, follow `AGENTS.md`.

## Commands

```bash
# Development (via mise)
mise run test              # Run tests via cargo-nextest
mise run lint              # Clippy with pedantic warnings as errors
mise run fmt               # Format check (nightly rustfmt)
mise run fmt:fix           # Auto-format
mise run audit             # cargo-audit for known vulnerabilities
mise run deny              # cargo-deny for licence/advisory checks
mise run docs              # Build docs with -D warnings
mise run udeps             # Check for unused deps (nightly)

# Raw cargo equivalents
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt --all --check
cargo +nightly fmt --all
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
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

When adding new dependencies, make them `optional = true` and gate behind a feature
unless they are core CPU detection.

## Architecture

The crate is split into a library (`lib.rs`) and a binary (`main.rs`). The binary has
three `#[cfg]`-gated code paths depending on which features are enabled.

**Core flow**: `arch::*::detect()` → `CpuInfo` struct → `printer::*` output

- `cpu/info.rs` — Central `CpuInfo`, `Vendor`, `Frequency`, `Version` types (always compiled)
- `cpu/flags.rs` — CPU feature flag detection via `bitflags`
- `cpu/frequency.rs` — Frequency detection (feature-gated)
- `cpu/uarch.rs` — Microarchitecture lookup table
- `arch/x86_64.rs` — CPUID-based detection via `raw-cpuid`
- `arch/aarch64.rs` — ARM detection via platform-specific syscalls
- `printer/` — ASCII art rendering, layout, JSON output (feature-gated)
- `cli/` — Clap argument parsing (feature-gated)
- `error.rs` — `thiserror`-based error enum with feature-gated variants

## Code Conventions

- **Synchronous only** — no async, no concurrency
- **No unsafe** — forbidden via `[lints.rust]` in `Cargo.toml`
- **Error handling**: `thiserror` for library errors, `anyhow` for CLI
- **Lints**: pedantic clippy via `[lints.clippy]` in `Cargo.toml`
- **Formatting**: nightly rustfmt, edition 2024, 120 char max, Unix newlines
- **Documentation language**: British English
- **Doc comments**: required on public items, prefer "why-first" style

## Git Workflow

Trunk-based: `main` is the sole long-lived branch. Feature work via short-lived branches + PRs.

- Commit format: `type(scope): description` (conventional commits, no emoji)
- release-plz auto-bumps version and creates tags from conventional commits

## CI

**Woodpecker CI** (primary — GitHub forge):

- **CI** (`ci.yaml`): fmt (nightly), clippy (stable), test, docs, cargo-audit, cargo-deny
- **Release-plz** (`release-plz.yaml`): auto version bump + tag on main push
- **Release** (`release.yaml`): Linux x86_64/aarch64/i686 builds, crates.io publish, GitHub Release

**GitHub Actions** (supplementary — dormant until billing resolved):

- **CI** (`ci.yml`): same checks + macOS test matrix + MSRV check
- **Security** (`security.yml`): cargo-audit + cargo-deny (weekly cron)
- **Release** (`release.yml`): cross-platform builds (macOS, Windows, FreeBSD) on `v*` tags

## Dev Environment

```bash
# Install tools
mise install

# Pin stable Rust + components
# (handled by rust-toolchain.toml)
rustup toolchain install nightly --component rustfmt  # for fmt
```

## Style

- Use British English in all prose (e.g. colour, organisation, behaviour).
