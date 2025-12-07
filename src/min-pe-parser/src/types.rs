#![allow(non_camel_case_types)]
#![allow(unused)]
use crate::prelude::*;
use simple_endian::LittleEndian;

// Aliases to make the parsing code more readable.
pub(crate) type pu8 = *const u8;
pub(crate) type pu16 = *const u16;
pub(crate) type pu32 = *const u32;

pub(crate) type pu8_le = *const LittleEndian<u8>;
pub(crate) type pu16_le = *const LittleEndian<u16>;
pub(crate) type pu32_le = *const LittleEndian<u32>;

pub(crate) type PIMAGE_DATA_DIRECTORY = *const IMAGE_DATA_DIRECTORY;
pub(crate) type PIMAGE_DOS_HEADER = *const IMAGE_DOS_HEADER;
pub(crate) type PIMAGE_FILE_HEADER = *const IMAGE_FILE_HEADER;
pub(crate) type PIMAGE_IMPORT_BY_NAME = *const IMAGE_IMPORT_BY_NAME;
pub(crate) type PIMAGE_IMPORT_DESCRIPTOR = *const IMAGE_IMPORT_DESCRIPTOR;
pub(crate) type PIMAGE_OPTIONAL_HEADER32 = *const IMAGE_OPTIONAL_HEADER32;
pub(crate) type PIMAGE_OPTIONAL_HEADER64 = *const IMAGE_OPTIONAL_HEADER64;
pub(crate) type PIMAGE_SECTION_HEADER = *const IMAGE_SECTION_HEADER;
pub(crate) type PIMAGE_THUNK_DATA32 = *const IMAGE_THUNK_DATA32;
pub(crate) type PIMAGE_THUNK_DATA64 = *const IMAGE_THUNK_DATA64;
pub(crate) type PIMAGE_EXPORT_DIRECTORY = *const IMAGE_EXPORT_DIRECTORY;
