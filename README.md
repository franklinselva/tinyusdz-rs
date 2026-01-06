# tinyusdz-rs

[![Crates.io](https://img.shields.io/crates/v/tinyusdz-rs.svg)](https://crates.io/crates/tinyusdz-rs)
[![Documentation](https://docs.rs/tinyusdz-rs/badge.svg)](https://docs.rs/tinyusdz-rs)
[![License](https://img.shields.io/crates/l/tinyusdz-rs.svg)](LICENSE)

**Rust bindings for [tinyusdz](https://github.com/lighttransport/tinyusdz)** - A lightweight, portable USD/USDZ parser.

## Overview

`tinyusdz-rs` provides safe Rust bindings to the tinyusdz C API, enabling USD file parsing in Rust applications without requiring the full OpenUSD library or Python.

### Features

- Parse USD files (USDA, USDC, USDZ)
- Traverse scene hierarchy (prims, children)
- Query prim types and property names
- Zero external runtime dependencies
- Cross-platform (Linux, macOS, Windows)

### Current Status

The tinyusdz C API is under active development. Currently supported:

| Feature | Status |
|---------|--------|
| Stage loading from files | ✅ |
| Prim traversal | ✅ |
| Prim type/name queries | ✅ |
| Property name listing | ✅ |
| Property value extraction | ⏳ Pending C API |
| Mesh geometry extraction | ⏳ Pending C API |
| Material properties | ⏳ Pending C API |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tinyusdz-rs = "0.1"
```

### Build Requirements

- Rust 1.70+
- CMake 3.16+
- C++14 compiler (GCC 4.9+, Clang 3.4+, MSVC 2019+)

## Quick Start

```rust
use tinyusdz_rs::{Stage, Result};

fn main() -> Result<()> {
    // Load a USD file
    let stage = Stage::open("model.usdz")?;

    // Traverse all prims
    for prim in stage.traverse() {
        println!("{}: {}", prim.name(), prim.type_name());

        // List properties
        for prop in prim.property_names() {
            println!("  .{}", prop);
        }
    }

    Ok(())
}
```

## Examples

### Parse USD File

```bash
cargo run --example parse_usdz -- model.usdz
```

Output:
```
Scene Statistics:
  Total prims: 68
  Meshes: 10
  Transforms: 17
  Materials: 9
```

### Dump Scene Hierarchy

```bash
cargo run --example dump_hierarchy -- scene.usdz
```

Output:
```
/scene <Xform>  [2 children, 0 properties]
/Materials <Scope>  [9 children, 0 properties]
/Glass <Material>  [1 children, 0 properties]
/Meshes <Xform>  [1 children, 1 properties]
    .xformOp:scale
```

## Building from Source

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/franklinselva/tinyusdz-rs.git
cd tinyusdz-rs

# Build
cargo build

# Run tests
cargo test

# Build release
cargo build --release
```

## Project Structure

```
tinyusdz-rs/
├── Cargo.toml
├── tinyusdz-sys/          # Raw FFI bindings
│   ├── Cargo.toml
│   ├── build.rs           # CMake + bindgen
│   ├── wrapper.h
│   └── src/lib.rs
├── src/                   # Safe Rust API
│   ├── lib.rs
│   ├── error.rs
│   ├── stage.rs
│   ├── prim.rs
│   ├── value.rs
│   ├── attribute.rs
│   ├── mesh.rs
│   └── material.rs
├── examples/
│   ├── parse_usdz.rs
│   ├── dump_hierarchy.rs
│   └── usd_to_glb.rs
└── tinyusdz/              # Git submodule
```

## Limitations

This crate wraps the tinyusdz C API, which is still under development. Some limitations:

### C API Limitations
- **Property value extraction not implemented** - Functions like `c_tinyusd_prim_property_get` are declared but not yet implemented in tinyusdz
- **No mesh geometry access** - While mesh prims are detected and property names listed, vertex/face data cannot be read
- **No material value access** - Material properties (diffuse color, roughness, etc.) cannot be extracted

### Workarounds
- Use property names to understand scene structure
- For full geometry extraction, consider using tinyusdz C++ API directly or wait for C API updates

### Upstream Tracking
These limitations will be resolved when tinyusdz implements the remaining C API functions. Track progress at:
- [tinyusdz C API](https://github.com/lighttransport/tinyusdz/blob/dev/src/c-tinyusd.h)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development

```bash
# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

### Areas of Interest

- Additional USD prim type support
- Animation/timesamples support
- WASM compilation
- Performance optimization

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Dependency Licenses

- [tinyusdz](https://github.com/lighttransport/tinyusdz) - MIT License

## Acknowledgments

- [lighttransport](https://github.com/lighttransport) for tinyusdz
- [Pixar](https://graphics.pixar.com/usd/) for the USD specification
