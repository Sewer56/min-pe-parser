use super::common::*;
use crate::{prelude::*, types::*};
use alloc::{string::String, vec::Vec};
use core::{ffi::c_void, mem::size_of, ptr, ptr::write};

/// Get the names of the DLLs that are imported by the PE file.
///
/// # Arguments
/// - `pe_start` - A pointer to the start of the PE file in memory.
/// - `is_mapped` - A boolean indicating whether the PE file is mapped into memory.
/// - `force_pe64` - Force PE64 format. Saves a few branches in code.
/// - `force_pe32` - Force PE32 format. Saves a few branches in code.
///
/// # Remarks
/// This function assumes that the PE file is valid and that the pointer is not null.
/// Using `force_pe64` and `force_pe32` together is undefined behavior. These exist
/// to turn some branches into constants at runtime, for those craving absolute perf.
///
/// # Safety
/// We dajiobu if pe_start is a valid pointer and the PE file is valid.
///
/// # Returns
/// Names of all DLLs in the import table.
#[cfg_attr(feature = "size_opt", optimize(size))]
pub unsafe fn get_import_dll_names(
    pe_start: *const c_void,
    is_mapped: bool,
    force_pe64: bool,
    force_pe32: bool,
) -> Vec<String> {
    assert_pe_is_aligned(pe_start);

    // Validate PE file signature and setup initial pointer.
    let pe_start = pe_start as PIMAGE_DOS_HEADER;

    if (*pe_start).e_magic() != 0x00005A4D {
        return Vec::new(); // MZ signature
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
    let import_table_directory = &*data_directories_ptr.offset(DataDirectoryType::ImportTable as isize);

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

    // Translate the RVA of the import descriptor to an absolute offset in memory.
    let import_descriptor_abs_offset =
        rva_to_absolute_offset(import_table_directory.virtual_address(), section_headers, is_mapped);

    // Early exit if the absolute offset of the import descriptors could not be calculated.
    if import_descriptor_abs_offset.is_none() {
        return Vec::new();
    }

    let import_descriptor_abs_offset = import_descriptor_abs_offset.unwrap_unchecked() as usize;

    // Determine the start and end of the import descriptors array.
    let import_descriptors_start = (pe_start as pu8).add(import_descriptor_abs_offset) as PIMAGE_IMPORT_DESCRIPTOR;
    let num_import_descriptors = import_table_directory.size() / size_of::<IMAGE_IMPORT_DESCRIPTOR>() as u32;
    let import_descriptors_end = import_descriptors_start.add(num_import_descriptors as usize);

    // Iterate through the import descriptors to collect the names of imported DLLs.
    let mut dll_names = Vec::with_capacity(num_import_descriptors as usize);
    let mut current_descriptor = import_descriptors_start;
    let orig_insert_ptr = dll_names.as_ptr();
    let mut insert_ptr = dll_names.as_mut_ptr();

    while current_descriptor < import_descriptors_end {
        let dll_name_rva = (*current_descriptor).name();

        // Convert the DLL name's RVA to an absolute offset and retrieve the name as a string.
        if let Some(dll_name_absolute_address) = rva_to_absolute_offset(dll_name_rva, section_headers, is_mapped) {
            let dll_name_ptr = (pe_start as pu8).add(dll_name_absolute_address as usize);
            let dll_name = get_null_terminated_utf8_string(dll_name_ptr);
            write(insert_ptr, dll_name);
            insert_ptr = insert_ptr.add(1);
        }

        current_descriptor = current_descriptor.add(1);
    }

    dll_names.set_len((insert_ptr.offset_from(orig_insert_ptr)) as usize);
    dll_names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::{RELOADED_BOOTSTRAPPER_DLL_X64, RELOADED_BOOTSTRAPPER_DLL_X86};
    use alloc::vec;

    #[test]
    fn test_reloaded_bootstrapper_dll_imports_x64() {
        let pe_start = RELOADED_BOOTSTRAPPER_DLL_X64.as_ptr() as *const c_void;
        let is_mapped = false;

        let imports = unsafe { get_import_dll_names(pe_start, is_mapped, false, false) };

        let expected_imports = vec![
            "KERNEL32.dll",
            "USER32.dll",
            "SHELL32.dll",
            "MSVCP140.dll",
            "VCRUNTIME140_1.dll",
            "VCRUNTIME140.dll",
            "api-ms-win-crt-runtime-l1-1-0.dll",
            "api-ms-win-crt-heap-l1-1-0.dll",
            "api-ms-win-crt-stdio-l1-1-0.dll",
            "api-ms-win-crt-convert-l1-1-0.dll",
            "api-ms-win-crt-locale-l1-1-0.dll",
            "api-ms-win-crt-filesystem-l1-1-0.dll",
            "api-ms-win-crt-string-l1-1-0.dll",
            "api-ms-win-crt-time-l1-1-0.dll",
            "ADVAPI32.dll",
            "api-ms-win-crt-math-l1-1-0.dll",
        ]
        .iter()
        .map(|&s| s.into())
        .collect::<Vec<String>>();

        assert_eq!(imports, expected_imports);
    }

    #[test]
    fn test_reloaded_bootstrapper_dll_imports_x86() {
        let pe_start = RELOADED_BOOTSTRAPPER_DLL_X86.as_ptr() as *const c_void;
        let is_mapped = false;
        let imports = unsafe { get_import_dll_names(pe_start, is_mapped, false, false) };

        let expected_imports = vec![
            "KERNEL32.dll",
            "USER32.dll",
            "SHELL32.dll",
            "MSVCP140.dll",
            "VCRUNTIME140.dll",
            "api-ms-win-crt-runtime-l1-1-0.dll",
            "api-ms-win-crt-heap-l1-1-0.dll",
            "api-ms-win-crt-stdio-l1-1-0.dll",
            "api-ms-win-crt-convert-l1-1-0.dll",
            "api-ms-win-crt-locale-l1-1-0.dll",
            "api-ms-win-crt-filesystem-l1-1-0.dll",
            "api-ms-win-crt-string-l1-1-0.dll",
            "api-ms-win-crt-time-l1-1-0.dll",
            "ADVAPI32.dll",
            "api-ms-win-crt-math-l1-1-0.dll",
        ]
        .iter()
        .map(|&s| s.into())
        .collect::<Vec<String>>();

        assert_eq!(imports, expected_imports);
    }
}
