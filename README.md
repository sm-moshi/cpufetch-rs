# cpufetch-rs

Rust rewrite of [cpufetch](https://github.com/Dr-Noob/cpufetch).

Early development — core CPU detection works, display formatting is in progress.

## Install

```bash
cargo install cpufetch-rs
```

Or build from source:

```bash
cargo build --release
./target/release/cpufetch
```

## What it detects

- Vendor, model name, microarchitecture
- Core/thread counts, base and boost frequencies
- Instruction set extensions (SSE, AVX, FMA, NEON, etc.)
- Cache topology (L1i/d, L2, L3)

Supports x86_64 and ARM64 (including Apple Silicon).

## Development

```bash
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
```

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).
