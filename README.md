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

Note: This is an early WIP release to reserve the name and set up metadata. Not ready for production use yet.

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

- **Elegant ASCII Art & Layout**
  Presents information in a visually appealing, terminal‑friendly format with vendor‑specific logos.

- **Cross‑Platform Compatibility**
  Works on Linux, macOS, and Windows (where supported) without external C dependencies.

- **Modern Rust Implementation**
  Leverages Rust's safety, performance, and modularity to ensure reliable, maintainable code.

- **Feature Detection**: Dynamic CPU feature detection for:
  - x86_64: SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, AVX, AVX2, FMA, BMI1, BMI2, etc.
  - ARM64: NEON, AES, PMULL, SHA1, SHA2, CRC32, ATOMICS, etc.

---

## 🚀 Installation

### 📦 Via Cargo

Install cpufetch‑rs directly with Cargo:

```bash
cargo install cpufetch
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

Our tests include unit tests for CPUID parsing, architecture detection, layout formatting, and full integration tests that simulate complete CLI execution.

---

## 🗺️ Roadmap

The project is organized into several key phases to achieve full feature parity with the original cpufetch:

1. **Project Setup & Core Scaffold**
   Establish a robust Rust project structure with CI, core modules, and error handling.
2. **CPU Information Module**
   Develop modules to parse CPUID data, extract feature flags, and measure CPU frequencies.
3. **Architecture Detection**
   Implement platform‑specific detection (x86_64, AArch64) and unify it under a common API.
4. **ASCII Art & Printer Module**
   Design the terminal output engine, including dynamic logo selection and layout formatting.
5. **CLI Interface**
   Build a comprehensive CLI using clap to support options like `--logo-only`, `--no-logo`, `--ascii`, and `--json`.
6. **Utility Functions & Integration**
   Create shared helpers for formatting and system inquiries; integrate all modules in `main.rs`.
7. **Documentation & Testing**
   Finalize the manpage, README, and comprehensive tests to ensure full feature parity.
8. **Packaging & Distribution**
   Prepare binaries for multiple platforms and publish to Crates.io.

For a more detailed plan, refer to our [ROADMAP.md](docs/ROADMAP.md) and [TODO.md](docs/TODO.md).

---

## 🤝 Contributing

We welcome contributions to cpufetch‑rs! Before submitting issues or pull requests, please review our guidelines:

- Follow the code style and formatting guidelines (use `cargo fmt` and `cargo clippy`).
- Write tests for any new features or bug fixes.
- Ensure CI passes on all supported platforms.

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
