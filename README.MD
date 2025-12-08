# min-pe-parser

[![Crates.io](https://img.shields.io/crates/v/min-pe-parser.svg)](https://crates.io/crates/min-pe-parser)
[![Docs.rs](https://docs.rs/min-pe-parser/badge.svg)](https://docs.rs/min-pe-parser)
[![CI](https://github.com/Sewer56/min-pe-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/min-pe-parser/actions)
[![codecov](https://codecov.io/gh/Sewer56/min-pe-parser/graph/badge.svg)](https://codecov.io/gh/Sewer56/min-pe-parser)

Optimized routines for parsing certain parts of the PE header, optimized for use in the Reloaded3 libraries & runtime. Aimed at minimizing code size down to the absolute minimum.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
min-pe-parser = "0.1.0"
```

```rust
// Assuming `pe_bytes` is a byte slice containing your PE file data
let pe_start = pe_bytes.as_ptr() as *const c_void;
let is_mapped = false;

// Get imported DLL names
let imported_dlls = unsafe { get_import_dll_names(pe_start, is_mapped, false, false) };

// Get section names
let sections = unsafe { get_section_names(pe_start, false, false) };

// Get export RVA by name
let rva = unsafe { get_export_rva(pe_start, "MyExport", is_mapped, false, false) };
```

See the [min-pe-parser crate documentation](./src/min-pe-parser/README.MD) for detailed usage.

## Crates

- [min-pe-parser](./src/min-pe-parser/README.MD): Minimal PE parser library

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [GPL v3 (with Reloaded FAQ)](./LICENSE).
