use super::common::*;
use crate::types::*;
use core::{ptr, ptr::write};
use std::os::raw::c_void;

/// Get the names of the sections in the PE file.
///
/// # Arguments
/// - `pe_start` - A pointer to the start of the PE file in memory.
/// - `force_pe64` - Force PE64 format. Saves a few branches in code.
/// - `force_pe32` - Force PE32 format. Saves a few branches in code.
///
/// # Safety
/// This function assumes that `pe_start` is a valid pointer to a PE file in memory.
pub unsafe fn get_section_names(pe_start: *const c_void, force_pe64: bool, force_pe32: bool) -> Vec<String> {
    assert_pe_is_aligned(pe_start);

    let pe_start = pe_start as PIMAGE_DOS_HEADER;

    if (*pe_start).e_magic() != 0x00005A4D {
        // MZ signature not found, invalid PE file.
        return Vec::new();
    }

    // Get the NT Header.
    let dos_header = pe_start;
    let nt_headers_ptr = (pe_start as pu8).add((*dos_header).e_lfanew() as usize) as pu32;

    // Access the IMAGE_FILE_HEADER to retrieve the number of sections and other metadata.
    let file_header = nt_headers_ptr.add(0x1) as PIMAGE_FILE_HEADER;
    let optional_header_ptr = file_header.add(1) as PIMAGE_OPTIONAL_HEADER32;

    // Determine the actual PE format (PE32 or PE64).
    let is_pe64 = force_pe64 || (*optional_header_ptr).magic().is_pe64();
    let is_pe32 = force_pe32 || (*optional_header_ptr).magic().is_pe32();

    // Calculate the number of data directories to access the import table directory.
    let num_rva_sizes = get_num_rva_and_sizes(optional_header_ptr, is_pe64, is_pe32) as isize;

    // Locate the data directories, specifically the import table directory, using the optional header.
    let data_directories_ptr = get_data_directories_ptr(optional_header_ptr as pu8, is_pe64, is_pe32);

    // Get the section headers to calculate the absolute offset of the import descriptor.
    let section_headers = (data_directories_ptr).offset(num_rva_sizes) as pu8;

    // WARNING !!
    //   Rust does not allow unaligned slices, to prevent unaligned reads.
    //   What we're doing here is undefined behaviour.
    //   However, because we can guarantee that the members of the struct are aligned, even if
    //   the struct itself isn't, we should be ok.
    let section_headers = &*ptr::slice_from_raw_parts(
        section_headers as PIMAGE_SECTION_HEADER,
        (*file_header).number_of_sections().into(),
    );

    // Extract section names
    let mut section_names = Vec::with_capacity(section_headers.len());
    let orig_insert_ptr = section_names.as_ptr();
    let mut insert_ptr = section_names.as_mut_ptr();

    for header in section_headers.iter() {
        let name = get_null_terminated_utf8_string_with_max_length(header.name.as_ptr(), 8);
        write(insert_ptr, name);
        insert_ptr = insert_ptr.add(1);
    }

    section_names.set_len((insert_ptr.offset_from(orig_insert_ptr)) as usize);
    section_names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_get_section_names_x64() {
        let section_names =
            unsafe { get_section_names(RELOADED_BOOTSTRAPPER_DLL_X64.as_ptr() as *const c_void, false, false) };

        let expected_sections = vec![".text", ".rdata", ".data", ".pdata", ".rsrc", ".reloc"];
        assert_eq!(section_names, expected_sections);
    }

    #[test]
    fn test_get_section_names_x86() {
        let section_names =
            unsafe { get_section_names(RELOADED_BOOTSTRAPPER_DLL_X86.as_ptr() as *const c_void, false, false) };

        let expected_sections = vec![".text", ".rdata", ".data", ".rsrc", ".reloc"];
        assert_eq!(section_names, expected_sections);
    }
}
