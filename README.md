<div align="center">
<samp>

# pim

## a lightweight converter for CSS units (e.g., "px", "em", "rem").

</samp>
</div>

![Rust](https://img.shields.io/badge/Rust-1.72.0-orange)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

---

## Installation

### Build from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/airRnot1106/pim.git
   cd pim
   ```

2. **Build the binary**:
   ```bash
   cargo build --release
   ```

3. **Run the tool**:
   ```bash
   ./target/release/pim --help
   ```

### Using `Nix`

If you have `Nix` installed:
```bash
nix run . -- --help
```

---

## Usage

### Basic Usage

To convert `24px` to other unit:
```bash
pim 24 px
```

**Output**:
```
px: 24px
em: 1.5em
rem: 1.5rem
```

### Custom Root Font Size

Specify a custom root font size using the `-r` option:
```bash
pim 24 px -r 10
```

**Output**:
```
px: 24px
em: 2.4em
rem: 2.4rem
```

## Tests

Run all tests:
```bash
cargo test
```

---

## License

`pim` is licensed under the [MIT License](https://github.com/airRnot1106/pim/blob/main/LICENSE).
