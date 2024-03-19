//! # Some Cool Reloaded Library
//! Here's the crate documentation.
#![feature(const_refs_to_static)]
#![feature(optimize_attribute)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod utils {
    pub(crate) mod common;
    pub mod get_export_rva;
    pub mod get_import_dll_names;
    pub mod get_section_names;
    #[cfg(test)]
    #[cfg_attr(tarpaulin, ignore)]
    pub mod test_utils;
}

#[cfg(not(tarpaulin_include))]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub mod structs {
    pub mod IMAGE_DATA_DIRECTORY;
    pub mod IMAGE_DOS_HEADER;
    pub mod IMAGE_EXPORT_DIRECTORY;
    pub mod IMAGE_FILE_HEADER;
    pub mod IMAGE_IMPORT_BY_NAME;
    pub mod IMAGE_IMPORT_DESCRIPTOR;
    pub mod IMAGE_OPTIONAL_HEADER32;
    pub mod IMAGE_OPTIONAL_HEADER64;
    pub mod IMAGE_SECTION_HEADER;
    pub mod IMAGE_THUNK_DATA32;
    pub mod IMAGE_THUNK_DATA64;

    pub mod enums {
        pub mod data_directory_type;
        pub mod data_section_flags;
        pub mod pe_magic;
    }
}

pub mod prelude {
    pub use crate::structs::{
        enums::{data_directory_type::*, data_section_flags::*, pe_magic::*},
        IMAGE_DATA_DIRECTORY::*,
        IMAGE_DOS_HEADER::*,
        IMAGE_EXPORT_DIRECTORY::*,
        IMAGE_FILE_HEADER::*,
        IMAGE_IMPORT_BY_NAME::*,
        IMAGE_IMPORT_DESCRIPTOR::*,
        IMAGE_OPTIONAL_HEADER32::*,
        IMAGE_OPTIONAL_HEADER64::*,
        IMAGE_SECTION_HEADER::*,
        IMAGE_THUNK_DATA32::*,
        IMAGE_THUNK_DATA64::*,
    };
}

#[cfg(not(tarpaulin_include))]
pub(crate) mod types;
