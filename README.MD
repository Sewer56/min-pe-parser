# min-pe-parser

[![Crates.io](https://img.shields.io/crates/v/min-pe-parser.svg)](https://crates.io/crates/min-pe-parser)
[![Docs.rs](https://docs.rs/min-pe-parser/badge.svg)](https://docs.rs/min-pe-parser)
[![CI](https://github.com/Sewer56/min-pe-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/min-pe-parser/actions)
[![codecov](https://codecov.io/gh/Sewer56/min-pe-parser/graph/badge.svg)](https://codecov.io/gh/Sewer56/min-pe-parser)

## About

Optimized routines for parsing certain parts of the PE header, optimized for use in the Reloaded3 libraries & runtime.

Aimed at minimizing code size down to the absolute minimum.
You can learn more about this project in the [dedicated documentation page][docs].

## Usage

This package provides the following utility functions:
- `get_import_dll_names` - Extracts the names of DLLs that a PE file imports.
- `get_section_names` - Retrieves the names of sections defined within the PE file.
- `get_export_rva` - Retrieves the Relative Virtual Address (RVA) of a specified export in the PE file.

### Extracting Imported DLL Names

To extract the names of DLLs that a PE file imports, use the `get_import_dll_names` function. 

This function requires a pointer to the start of the PE file in memory, flags to indicate whether 
the PE file is mapped into memory already and whether to force interpretation as PE32 or PE64 format.

```rust
// Assuming `pe_bytes` is a byte slice containing your PE file data
let pe_start = pe_bytes.as_ptr() as *const c_void;
let is_mapped = false; // Set to true if the PE file is already mapped into memory
let force_pe64 = false;// Force PE64 format
let force_pe32 = false;// Force PE32 format

let imported_dll_names = unsafe {
    get_import_dll_names(pe_start, is_mapped, force_pe64, force_pe32)
};
println!("Imported DLLs: {:?}", imported_dll_names);
```

### Retrieving Section Names

To get the names of sections within the PE file, use the `get_section_names` function. 
Similar to `get_import_dll_names`, this function requires a pointer to the start of the PE file 
and flags for PE format interpretation.

```rust
let section_names = unsafe {
    get_section_names(pe_start, force_pe64, force_pe32)
};
println!("Section Names: {:?}", section_names);
```

### Retrieving Export RVA

To get the Relative Virtual Address (RVA) of a specific export in the PE file, use the 
`get_export_rva` function.

This function requires a pointer to the start of the PE file, the name of the export,
and flags similar to the other functions.

```rust
let export_name = "SomeExportFunction";
let export_rva = unsafe {
    get_export_rva(pe_start, export_name, is_mapped, force_pe64, force_pe32)
};
if export_rva != usize::MAX {
    println!("RVA of {}: 0x{:X}", export_name, export_rva);
} else {
    println!("Export {} not found", export_name);
}
```

### Optimization

The `force_pe64` and `force_pe32` flags are used to force the parser to interpret the PE file as 
a specific format. This is a compiler hint that can be used to say 'I will only ever deal with PE32'
files, etc. Saves a few instructions.

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Releasing a New Version

Make a tag, aptly named after the current version of the project. For instance, if you are publishing version `0.1.0`, the tag should be `0.1.0`. This will create a GitHub release for you automatically.

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [GPL v3 (with Reloaded FAQ)](./LICENSE).

[docs]: https://min-pe-parser.github.io/min-pe-parser
