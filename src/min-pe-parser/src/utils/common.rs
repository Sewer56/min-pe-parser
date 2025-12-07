use crate::{prelude::*, types::*};
use alloc::string::{String, ToString};
use core::{ffi::CStr, hint::unreachable_unchecked, mem::size_of, slice};

/// Converts a "Relative Virtual Address" (RVA) to an absolute offset.
///
/// # Arguments
/// * `rva` - The relative virtual address to convert.
/// * `section_headers` - A slice of IMAGE_SECTION_HEADER representing the section headers.
/// * `is_mapped` - A boolean indicating whether the PE file is mapped into memory.
///
/// # Returns
/// An option containing the absolute address if conversion is successful, or None.
#[inline]
pub(crate) unsafe fn rva_to_absolute_offset(
    rva: u32,
    section_headers: &[IMAGE_SECTION_HEADER],
    is_mapped: bool,
) -> Option<u32> {
    // If the RVA is 0, the section doesn't exist.
    if rva == 0 {
        return None;
    }

    // If it's already mapped into memory, then RVA == Absolute
    if is_mapped {
        return Some(rva);
    }

    // If it's not mapped, check what section it's in and adjust for pointer to raw data.
    for header in section_headers.iter() {
        let start_address = header.virtual_address();
        let end_address = start_address + header.virtual_size();

        if rva >= start_address && rva < end_address {
            let absolute_address = rva - start_address + header.pointer_to_raw_data();
            return Some(absolute_address);
        }
    }

    None
}

/// Retrieves the number of RVA and Sizes from the optional header.
/// In a way that makes the main code more readable.
#[inline]
pub(crate) unsafe fn get_num_rva_and_sizes(
    optional_header_ptr: PIMAGE_OPTIONAL_HEADER32,
    is_pe64: bool,
    is_pe32: bool,
) -> u32 {
    if is_pe64 {
        let optional_header_ptr = optional_header_ptr as PIMAGE_OPTIONAL_HEADER64;
        (*optional_header_ptr).number_of_rva_and_sizes()
    } else if is_pe32 {
        let optional_header_ptr = optional_header_ptr as PIMAGE_OPTIONAL_HEADER32;
        (*optional_header_ptr).number_of_rva_and_sizes()
    } else {
        unreachable_unchecked()
    }
}

/// Retrieves the pointer to the 'Data Directories' from the optional header.
/// In a way that makes the main code more readable.
#[inline]
pub(crate) unsafe fn get_data_directories_ptr(
    optional_header_ptr: *const u8,
    is_pe64: bool,
    is_pe32: bool,
) -> PIMAGE_DATA_DIRECTORY {
    optional_header_ptr.add(if is_pe64 {
        size_of::<IMAGE_OPTIONAL_HEADER64>()
    } else if is_pe32 {
        size_of::<IMAGE_OPTIONAL_HEADER32>()
    } else {
        unsafe { unreachable_unchecked() }
    }) as PIMAGE_DATA_DIRECTORY
}

#[inline]
pub(crate) unsafe fn get_null_terminated_utf8_string_slice<'a>(ptr: pu8) -> &'a str {
    // SAFETY: The caller guarantees that the string is valid UTF-8 and null-terminated.
    let c_str = CStr::from_ptr(ptr as *const i8);
    c_str.to_str().unwrap_unchecked() // Safe due to the UTF-8 guarantee
}

#[inline]
pub(crate) unsafe fn get_null_terminated_utf8_string(ptr: pu8) -> String {
    // SAFETY: The caller guarantees that the string is valid UTF-8 and null-terminated.
    let c_str = CStr::from_ptr(ptr as *const i8);
    let str_slice = c_str.to_str().unwrap_unchecked(); // Safe due to the UTF-8 guarantee
    str_slice.to_string() // Convert the &str to a String
}

#[inline]
pub(crate) unsafe fn get_null_terminated_utf8_string_with_max_length(ptr: pu8, max_len: usize) -> String {
    // SAFETY: The caller guarantees that the string is valid UTF-8 up to max_len and null-terminated.
    let mut end_ptr = ptr;
    let mut len = 0;

    // Iterate over the string to find its length or the max length
    while *end_ptr != 0 && len < max_len {
        end_ptr = end_ptr.add(1);
        len += 1;
    }

    // Create a slice from the raw parts, up to len
    let slice = slice::from_raw_parts(ptr, len);

    // SAFETY: The caller guarantees that the string is valid UTF-8.
    let str_slice = core::str::from_utf8_unchecked(slice);
    str_slice.to_string()
}

#[inline]
pub(crate) fn assert_pe_is_aligned(pe_start: *const core::ffi::c_void) {
    // Real PE files should be aligned to at least 512 bytes, i.e. matching 'FileAlignment'.
    // We only require 4, thankfully.
    debug_assert_eq!(
        (pe_start as usize) % 4,
        0,
        "pe_start must be aligned to at least 4 bytes"
    );
}
