# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.x | Latest only |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it via
[GitHub Security Advisories](https://github.com/sm-moshi/cpufetch-rs/security/advisories/new).

Do not open a public issue for security vulnerabilities.

## Supply-Chain Security

This project uses [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) to enforce:

- No known vulnerabilities in dependencies (RustSec advisory database)
- No copyleft-licenced dependencies (MIT/Apache-2.0 dual-licence project)
- No wildcard version requirements
- No dependencies from unknown registries or git sources

These checks run on every PR and weekly via GitHub Actions.
