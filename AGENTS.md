# AGENTS — Repository Enforcement Contract

This file is the **authoritative policy** for all automation touching this repo
(CI, bots, and AI coding agents). Violations are defects.

## 1) Scope

Repository type: **Public Rust CLI and library crate**.

- Binary: `cpufetch` — cross-platform CPU information tool
- Library: `cpufetch_rs` — reusable CPU detection library
- Upstream reference: [Dr-Noob/cpufetch](https://github.com/Dr-Noob/cpufetch)

Core principles:

- Idiomatic, safe Rust
- Feature-gated optional dependencies
- Cross-platform (x86_64 + aarch64, Linux/macOS/Windows/FreeBSD)
- Conventional commits for automated releases

## 2) Hard Rules

### 2.1 No unsafe code

`unsafe` is **forbidden** via `[lints.rust]` in `Cargo.toml`.

If a future change requires `unsafe`:

- Add a `// SAFETY:` comment documenting invariants
- Open a tracking issue explaining why safe alternatives are insufficient
- Require explicit human approval before merging

### 2.2 Feature-gated dependencies

New dependencies MUST be `optional = true` and gated behind a feature in `Cargo.toml`
unless they are core CPU detection (always compiled).

Do not add non-optional dependencies without explicit justification.

### 2.3 Lints live in Cargo.toml

All clippy and rustc lint configuration lives in the `[lints]` section of `Cargo.toml`.
Do not use `.cargo/config.toml` for lint flags.

### 2.4 No breaking public API changes without label

Changes that break the public API of `cpufetch_rs` (the library crate) must be tagged
with the `breaking` label and follow semver.

### 2.5 Conventional commits

Format: `type(scope): description`

Types: `feat`, `fix`, `refactor`, `docs`, `style`, `test`, `ci`, `chore`, `perf`

No emoji prefixes. release-plz parses these for automated version bumps.

### 2.6 British English

All prose — doc comments, commit messages, documentation — uses British English
(e.g. colour, behaviour, organisation).

### 2.7 Documentation

All public items must have doc comments. Prefer "why-first" style.

## 3) CI Contract

GitHub Actions enforces the following on every push to `main` and every PR:

| Check | Toolchain | Command | Failure = block |
|-------|-----------|---------|----------------|
| Format | nightly | `cargo fmt --all --check` | Yes |
| Clippy | stable | `cargo clippy --all-targets --all-features` | Yes |
| Test | stable | `cargo test --all-features` | Yes |
| Docs | stable | `cargo doc --no-deps --all-features` (`-D warnings`) | Yes |
| MSRV | 1.85.0 | `cargo check --all-features` | Yes |
| Audit | — | `cargo-audit` + `cargo-deny` | Yes |

## 4) Decision Authority

- `Cargo.toml` is the source of truth for dependencies, features, and lints
- `rust-toolchain.toml` pins the default toolchain channel
- `deny.toml` governs licence and advisory policy
- `release-plz.toml` controls release automation

## 5) Release Flow

1. Conventional commits land on `main`
2. `release-plz` (GHA) analyses commits, bumps `Cargo.toml` version, creates `v*` tag
3. Tag push triggers the Release workflow: cross-platform builds + GitHub Release + crates.io publish
4. Agents must not create tags manually

## 6) Agent Constraints

Agents MUST:

- Propose changes as diffs
- Run `cargo clippy` and `cargo test` before considering work done
- Verify assumptions before implementation
- Avoid speculative refactors

Agents MUST NOT:

- Silence failing checks or suppress clippy lints without justification
- Add `unsafe` code
- Create version tags or publish to crates.io
- Add non-optional dependencies without feature gating

If an agent cannot comply, it must fail loudly and request human intervention.
Silent policy bypass is forbidden.
