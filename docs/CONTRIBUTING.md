# Contributing to cpufetch-rs

Thank you for considering contributing!

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/sm-moshi/cpufetch-rs/issues),
please check that it has not already been reported by searching for related keywords.

## Pull requests

Try to do one pull request per change.

### Commit messages

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for
automated version bumps and changelog generation.

Format: `type(scope): description`

Types: `feat`, `fix`, `refactor`, `docs`, `style`, `test`, `ci`, `chore`, `perf`

Examples:

- `feat(cli): add --verbose flag`
- `fix(aarch64): correct Apple M4 microarchitecture detection`
- `docs: update README platform support table`

## Setting up the development environment

```bash
git clone https://github.com/sm-moshi/cpufetch-rs
cd cpufetch-rs

# Install dev tools (cargo-nextest, cargo-audit, cargo-deny, etc.)
mise install

# Install nightly rustfmt (CI uses nightly for formatting)
rustup toolchain install nightly --component rustfmt
```

## Useful commands

```bash
# Run all tests
mise run test

# Clippy (pedantic, warnings as errors)
mise run lint

# Format check (nightly rustfmt)
mise run fmt

# Auto-format
mise run fmt:fix

# Security audit
mise run audit

# Build docs with warnings as errors
mise run docs
```

Or use raw `cargo` commands:

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt --all --check
cargo +nightly fmt --all
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## CI checks

Every PR is validated by GitHub Actions:

- **rustfmt** — nightly, `cargo fmt --all --check`
- **clippy** — stable, pedantic, warnings as errors
- **test** — Linux + macOS matrix
- **docs** — `cargo doc` with `-D warnings`
- **MSRV** — builds with Rust 1.85.0
- **security** — cargo-audit + cargo-deny

All checks must pass before merging.

## Code conventions

- **No unsafe code** — forbidden via `[lints.rust]` in `Cargo.toml`
- **Feature-gate new deps** — `optional = true` + feature flag
- **British English** in all prose and documentation
- **Doc comments** on all public items
- See [AGENTS.md](../AGENTS.md) for the full policy
