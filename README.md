# cpufetch‑rs 🦀

> A blazing‑fast, modern, and elegant Rust rewrite of [cpufetch](https://github.com/Dr-Noob/cpufetch)

[![CI](https://github.com/sm-moshi/cpufetch‑rs/actions/workflows/ci.yml/badge.svg)](https://github.com/sm-moshi/cpufetch‑rs/actions)
[![Crates.io](https://img.shields.io/crates/v/cpufetch‑rs.svg)](https://crates.io/crates/cpufetch‑rs)
[![Docs.rs](https://docs.rs/cpufetch‑rs/badge.svg)](https://docs.rs/cpufetch‑rs)
![License](https://img.shields.io/crates/l/cpufetch‑rs)
![Platform](https://img.shields.io/badge/platform-cross‑platform-green)
![Contributions](https://img.shields.io/badge/contributions-welcome-brightgreen)
![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange)

---

Note: This is an early development version - core CPU detection is functional but display formatting is still in progress.

---

## 📸 Demo

```bash
$ cpufetch‑rs
Vendor: AMD
Model: Ryzen 7 5800X
Cores: 8
Threads: 16
Frequency: 3.8GHz (base) / 4.7GHz (boost)
Flags: SSE, SSE2, AVX2, FMA, …
```

![Terminal Screenshot](docs/terminal_example.png)

---

## ✨ Features

- **Comprehensive CPU Data**
  Detects and displays CPU vendor, model name, microarchitecture, core/thread counts, and detailed frequency information.

- **Extensive Feature Flag Support**
  Accurately parses and lists supported instruction sets such as SSE, AVX, FMA, and more.

- **Cross‑Platform Architecture Support**
  Works on both ARM (Apple Silicon) and x86_64 architectures with graceful fallbacks.

- **Modern Rust Implementation**
  Leverages Rust's safety, performance, and modularity to ensure reliable, maintainable code.

- **Feature Detection**: Dynamic CPU feature detection for:
  - x86_64: SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, AVX, AVX2, FMA, BMI1, BMI2, etc.
  - ARM64: NEON, AES, PMULL, SHA1, SHA2, CRC32, ATOMICS, etc.

- **Cache Topology Detection**
  Identifies and displays detailed cache hierarchy information (L1i, L1d, L2, L3) when available.

---

## 🚀 Installation

### 📦 Via Cargo

Install cpufetch‑rs directly with Cargo:

```bash
cargo install cpufetch-rs
```

### 🛠️ From Source

Clone the repository and build it:

```bash
git clone https://github.com/sm-moshi/cpufetch‑rs.git
cd cpufetch‑rs
cargo build --release
./target/release/cpufetch
```

---

## 🧪 Testing

Run the full test suite with:

```bash
cargo test
```

To test on a specific architecture:

```bash
cargo test --target aarch64-apple-darwin  # For Apple Silicon
cargo test --target x86_64-unknown-linux-gnu  # For x86_64
```

Our tests include unit tests for CPUID parsing, architecture detection, feature flag support, and proper cross-platform behavior.

---

## 🗺️ Project Status

Current development progress:

- ✅ Core CPU information structures
- ✅ CPUID parsing module with architecture-specific gates
- ✅ Feature flag detection for both x86_64 and ARM
- ✅ Cache topology detection for x86_64 processors
- ✅ Cross-platform architecture detection
- ✅ ARM and x86_64 specific implementations
- 🔄 Frequency detection (in progress)
- 🔄 Display formatting (coming soon)
- 🔄 CLI interface (coming soon)

For a detailed roadmap, refer to our [ROADMAP.md](docs/ROADMAP.md) and [TODO.md](docs/TODO.md).

---

## 🤝 Contributing

We welcome contributions to cpufetch‑rs! Before submitting issues or pull requests, please review our guidelines:

- Follow the code style and formatting guidelines (use `cargo fmt` and `cargo clippy`).
- Write tests for any new features or bug fixes.
- Ensure CI passes on all supported platforms.
- Follow the Gitflow workflow described in our project docs.

Read our [CONTRIBUTING.md](docs/CONTRIBUTING.md) for full details.

---

## 🪪 License

cpufetch‑rs is dual‑licensed under the MIT License and the Apache 2.0 License – choose the one that best fits your needs.

- [MIT License](LICENSE-MIT)
- [Apache 2.0 License](LICENSE-APACHE)

Unless explicitly stated otherwise, any contribution you submit will be dual‑licensed.

---

## ❤️ Sponsors

Support the development of cpufetch‑rs on [GitHub Sponsors](https://github.com/sponsors/sm-moshi).

---

## 📚 References

- Original [cpufetch](https://github.com/Dr-Noob/cpufetch) repository.
- Rust ecosystem documentation: [Rust Lang](https://www.rust-lang.org/).
- [raw-cpuid crate](https://crates.io/crates/raw-cpuid) for CPUID parsing.

---

*cpufetch‑rs is a labour of love aimed at delivering the best possible terminal experience for CPU information in Rust. Thank you for your interest and support!*
