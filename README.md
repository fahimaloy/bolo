# bolo

> **A tiny, fast, Rust implementation of the classic `echo` command.**

[![Crates.io](https://img.shields.io/crates/v/bolo.svg)](https://crates.io/crates/bolo)
![Downloads](https://img.shields.io/crates/d/bolo.svg)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)
![Version](https://img.shields.io/badge/version-0.1.0-orange)

---

## About `bolo`

`bolo` is a **zero-dependency**, **blazing fast** native binary written in **100% safe Rust**. It is designed to be a faithful and efficient clone of the standard POSIX `echo` utility.

The name "Bolo" means **"speak"** in Bengali — perfect for an `echo` clone!

---

## Features

* **Full POSIX `echo` behavior**
* Supports standard options: **`-n`** (suppress newline), **`-e`** (enable escapes), **`-E`** (disable escapes)
* Comprehensive **backslash escape sequence** handling when using `-e` (e.g., `\n`, `\t`, `\a`, `\b`, `\c`, `\0nnn`, `\xHH`)
* Includes **`--help`** and **`--version`** flags
* **Zero dependencies**
* **Blazing fast** native binary with a tiny footprint (approx. 200 KB)
* Built entirely with **safe Rust** for guaranteed memory safety.

---

## Installation

### From Crates.io (Recommended)

```bash
cargo install bolo
```

Now `bolo` is available globally!

### From Source

```bash
git clone https://github.com/fahimaloy/bolo.git
cd bolo
cargo install --path .
```

### Build & Run Locally

```bash
# Build the optimized release binary
cargo build --release
# Run the binary directly from the target directory
./target/release/bolo hello world
```

---

## Usage

```bash
bolo [OPTIONS] [STRING...]
```

### Examples

| Command | Output | Notes |
|---------|--------|-------|
| `bolo hello world` | `hello world` | Basic echo |
| `bolo -n "no newline"` | `no newline`(no newline at end) | Use `-n` to suppress the trailing newline |
| `bolo -e "Line1\nLine2\tTabbed"` | `Line1`<br>`Line2 Tabbed` | Use `-e` to interpret escape sequences |
| `bolo -e "Bell\a" && sleep 1` | (makes a beep sound) | `\a` (alert) produces an audible bell sound |
| `bolo -e "Stop here\c and ignore this"` | `Stop here` | `\c` stops the output immediately |
| `bolo --help` | Shows the help message | |
| `bolo --version` | Shows the version information | |

---

## Supported Escape Sequences (`-e`)

| Sequence | Meaning |
|----------|---------|
| `\\` | backslash character |
| `\a` | alert (bell) |
| `\b` | backspace |
| `\c` | stop output immediately |
| `\e` | escape character |
| `\f` | form feed |
| `\n` | newline |
| `\r` | carriage return |
| `\t` | horizontal tab |
| `\v` | vertical tab |
| `\0nnn` | octal value (1–3 digits) |
| `\xHH` | hexadecimal value (1–2 digits) |

---

## Project Structure

```
bolo/
├── Cargo.toml
├── src/
│   └── main.rs
├── README.md
├── LICENSE-MIT
└── LICENSE-APACHE
```

---

## Development

### Development Commands

```bash
# Run tests
cargo test

# Build the release binary
cargo build --release

# Run with arguments for quick testing
cargo run -- -e "Hello\nWorld"
```

---

## Why Rust?

Rust provides the perfect environment for small, high-performance CLI tools like `bolo`:

* **No Runtime/GC:** Results in a **tiny binary** (~200 KB) and minimal overhead.
* **Performance:** Compiles to native code, offering speed comparable to C/C++.
* **Safety:** Guarantees **memory safety** without sacrificing performance.
* **Distribution:** Produces a **single, easy-to-distribute binary**.

---

## Contributing

Contributions are always welcome! Feel free to:

* Open an **issue** to report bugs or suggest features.
* Submit a **Pull Request** with improvements.
* Areas for improvement include enhancing escape handling or adding optional color/formatting support.

Please follow the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

---

## License

This project is dual-licensed under the following, at your option:

* **MIT License**
* **Apache License 2.0**

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for full details.

---

## Author

**Fahim Ahmed**  
[fahimaloy@gmail.com](mailto:fahimaloy@gmail.com)

---
