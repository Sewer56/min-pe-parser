# min-pe-parser

[![Crates.io](https://img.shields.io/crates/v/min-pe-parser.svg)](https://crates.io/crates/min-pe-parser)
[![Docs.rs](https://docs.rs/min-pe-parser/badge.svg)](https://docs.rs/min-pe-parser)
[![CI](https://github.com/Sewer56/min-pe-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/min-pe-parser/actions)

Optimized routines for parsing certain parts of the PE header, optimized for use in the Reloaded3 libraries & runtime.

Aimed at minimizing code size down to the absolute minimum.

## Features

- `get_import_dll_names` - Extracts the names of DLLs that a PE file imports.
- `get_section_names` - Retrieves the names of sections defined within the PE file.
- `get_export_rva` - Retrieves the Relative Virtual Address (RVA) of a specified export in the PE file.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
min-pe-parser = "0.1.0"
```

### Feature Flags

| Feature    | Description                                          |
| ---------- | ---------------------------------------------------- |
| `std`      | Enable standard library support (enabled by default) |
| `size_opt` | Enable size optimizations                            |

## Usage

### Extracting Imported DLL Names

To extract the names of DLLs that a PE file imports, use the `get_import_dll_names` function. 

This function requires a pointer to the start of the PE file in memory, flags to indicate whether 
the PE file is mapped into memory already and whether to force interpretation as PE32 or PE64 format.

```rust,no_run
# use core::ffi::c_void;
# use min_pe_parser::utils::get_import_dll_names::get_import_dll_names;
# fn main() {
# let pe_bytes: &[u8] = &[];
// Assuming `pe_bytes` is a byte slice containing your PE file data
let pe_start = pe_bytes.as_ptr() as *const c_void;
let is_mapped = false; // Set to true if the PE file is already mapped into memory
let force_pe64 = false; // Force PE64 format
let force_pe32 = false; // Force PE32 format

let imported_dll_names = unsafe {
    get_import_dll_names(pe_start, is_mapped, force_pe64, force_pe32)
};
println!("Imported DLLs: {:?}", imported_dll_names);
# }
```

### Retrieving Section Names

To get the names of sections within the PE file, use the `get_section_names` function. 
Similar to `get_import_dll_names`, this function requires a pointer to the start of the PE file 
and flags for PE format interpretation.

```rust,no_run
# use core::ffi::c_void;
# use min_pe_parser::utils::get_section_names::get_section_names;
# fn main() {
# let pe_bytes: &[u8] = &[];
# let pe_start = pe_bytes.as_ptr() as *const c_void;
# let force_pe64 = false;
# let force_pe32 = false;
let section_names = unsafe {
    get_section_names(pe_start, force_pe64, force_pe32)
};
println!("Section Names: {:?}", section_names);
# }
```

### Retrieving Export RVA

To get the Relative Virtual Address (RVA) of a specific export in the PE file, use the 
`get_export_rva` function.

This function requires a pointer to the start of the PE file, the name of the export,
and flags similar to the other functions.

```rust,no_run
# use core::ffi::c_void;
# use min_pe_parser::utils::get_export_rva::get_export_rva;
# fn main() {
# let pe_bytes: &[u8] = &[];
# let pe_start = pe_bytes.as_ptr() as *const c_void;
# let is_mapped = false;
# let force_pe64 = false;
# let force_pe32 = false;
let export_name = "SomeExportFunction";
let export_rva = unsafe {
    get_export_rva(pe_start, export_name, is_mapped, force_pe64, force_pe32)
};
if export_rva != usize::MAX {
    println!("RVA of {}: 0x{:X}", export_name, export_rva);
} else {
    println!("Export {} not found", export_name);
}
# }
```

### Optimization

The `force_pe64` and `force_pe32` flags are used to force the parser to interpret the PE file as 
a specific format. This is a compiler hint that can be used to say 'I will only ever deal with PE32'
files, etc. Saves a few instructions.

## License

Licensed under [GPL v3 (with Reloaded FAQ)](../../LICENSE).
