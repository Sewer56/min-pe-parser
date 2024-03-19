use super::common::*;
use crate::{prelude::*, types::*};
use core::ptr;
use std::os::raw::c_void;

/// Retrieves the offset of the given export in Virtual Memory,
/// relative to the start of the PE file. Add the returned value to the address
/// of the mapped PE file to get the absolute address of the export.
///
/// # Arguments
/// - `pe_start` - A pointer to the start of the PE file in memory.
/// - `export_name` - The name of the export to search.
/// - `is_mapped` - A boolean indicating whether the PE file is mapped into memory.
/// - `force_pe64` - Force PE64 format. Saves a few branches in code.
/// - `force_pe32` - Force PE32 format. Saves a few branches in code.
///
/// # Safety
/// We dajiobu if pe_start is a valid pointer and the PE file is valid.
///
/// # Returns
///
/// A value of [`usize::MAX`] indicates that the export was not found.
/// Otherwise, the return value is the absolute offset of the export in memory.
#[cfg_attr(feature = "size_opt", optimize(size))]
pub unsafe fn get_export_rva(
    pe_start: *const c_void,
    export_name: &str,
    is_mapped: bool,
    force_pe64: bool,
    force_pe32: bool,
) -> usize {
    assert_pe_is_aligned(pe_start);

    // Validate PE file signature and setup initial pointer.
    let pe_start = pe_start as PIMAGE_DOS_HEADER;

    if (*pe_start).e_magic() != 0x00005A4D {
        return usize::MAX;
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

    // Locate the data directories, specifically the export table directory, using the optional header.
    let data_directories_ptr = get_data_directories_ptr(optional_header_ptr as pu8, is_pe64, is_pe32);
    let export_table_directory = &*data_directories_ptr.offset(DataDirectoryType::ExportTable as isize);

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
    let export_descriptor_abs_offset =
        rva_to_absolute_offset(export_table_directory.virtual_address(), section_headers, is_mapped);

    // Early exit if the absolute offset of the export descriptors could not be calculated.
    if export_descriptor_abs_offset.is_none() {
        return usize::MAX;
    }

    let export_descriptor_abs_offset = export_descriptor_abs_offset.unwrap_unchecked() as usize;
    let export_descriptor = (pe_start as pu8).add(export_descriptor_abs_offset) as PIMAGE_EXPORT_DIRECTORY;

    // Get the ordinal 'base'
    let num_names = (*export_descriptor).number_of_names();

    // We now need to iterate through the export name pointer table.
    // Note: This could be optimized as the exports are lexically ordered,
    //       however for our purposes, we assume minimal/near zero exports,
    //       so saving on code size is more beneficial.

    // Get the address of the export name pointer table.
    let export_name_pointer_table_rva =
        rva_to_absolute_offset((*export_descriptor).address_of_names(), section_headers, is_mapped);

    // Early exit if the absolute offset of the export name pointer table could not be calculated.
    if export_name_pointer_table_rva.is_none() {
        return usize::MAX;
    }

    // Iterate through the export name pointer table to find the export.
    let export_name_pointer_table =
        (pe_start as pu8).add(export_name_pointer_table_rva.unwrap_unchecked() as usize) as pu32_le;

    for x in 0..num_names {
        let export_name_rva = *export_name_pointer_table.add(x as usize);
        let export_name_ptr = rva_to_absolute_offset(export_name_rva.into(), section_headers, is_mapped);

        // Ignore if the offset couldn't be calculated.
        if export_name_ptr.is_none() {
            continue;
        }

        // Get that export name.
        let export_name_ptr = (pe_start as pu8).add(export_name_ptr.unwrap_unchecked() as usize);

        // Compare the export name with the given export name.
        // Note: The exports are ASCII,
        if get_null_terminated_utf8_string_slice(export_name_ptr) == export_name {
            // Imitating what Microsoft specifies in docs:
            // https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#export-ordinal-table
            // i = Search_ExportNamePointerTable (name);
            // ordinal = ExportOrdinalTable [i];
            // rva = ExportAddressTable [ordinal];

            // Get the ordinal of the export.
            // The ordinal value is actually the index in the export address table.
            let ordinal_table_rva = rva_to_absolute_offset(
                (*export_descriptor).address_of_name_ordinals(),
                section_headers,
                is_mapped,
            );

            if ordinal_table_rva.is_none() {
                return usize::MAX;
            }

            let ordinal_table_rva = ordinal_table_rva.unwrap_unchecked() as usize;
            let ordinal_table = (pe_start as pu8).add(ordinal_table_rva) as pu16_le;

            // Look up the correct ordinal using the index.
            let ordinal: u16 = (*ordinal_table.add(x as usize)).into();

            // Get the address of the export address table.
            let export_address_table_rva =
                rva_to_absolute_offset((*export_descriptor).address_of_functions(), section_headers, is_mapped);
            if export_address_table_rva.is_none() {
                return usize::MAX;
            }
            let export_address_table =
                (pe_start as pu8).add(export_address_table_rva.unwrap_unchecked() as usize) as pu32_le;

            // Use the actual ordinal (adjusted by the base) to get the export address.
            let export_address_rva = *export_address_table.add(ordinal as usize);
            return u32::from(export_address_rva) as usize;
        }
    }

    // Couldn't find the export
    usize::MAX
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        get_export_rva::get_export_rva,
        test_utils::{RELOADED_BOOTSTRAPPER_DLL_X64, RELOADED_BOOTSTRAPPER_DLL_X86},
    };
    use std::os::raw::c_void;

    // Define a struct to hold test data for export names and their expected RVAs.
    struct ExportTestData {
        name: &'static str,
        expected_rva: usize,
    }

    impl ExportTestData {
        fn new(name: &'static str, expected_rva: usize) -> Self {
            Self { name, expected_rva }
        }
    }

    #[test]
    fn test_reloaded_bootstrapper_dll_exports_x64() {
        let pe_start = RELOADED_BOOTSTRAPPER_DLL_X64.as_ptr() as *const c_void;
        let exports = [
            ExportTestData::new("InitializeASI", 0x236c),
            ExportTestData::new("MainMemoryModInfo", 0x240c0),
            ExportTestData::new("ManiaModInfo", 0x240c0),
            ExportTestData::new("SA2ModInfo", 0x240c0),
            ExportTestData::new("SADXModInfo", 0x240c0),
            ExportTestData::new("SKCModInfo", 0x240c0),
            ExportTestData::new("SonicRModInfo", 0x240c0),
            ExportTestData::new("get_hostfxr_path", 0x10940),
        ];

        for export in exports.iter() {
            let offset = unsafe { get_export_rva(pe_start, export.name, false, true, false) };
            assert_eq!(offset, export.expected_rva, "Mismatch for export {}", export.name);
        }
    }

    #[test]
    fn test_reloaded_bootstrapper_dll_exports_x86() {
        let pe_start = RELOADED_BOOTSTRAPPER_DLL_X86.as_ptr() as *const c_void;
        let exports = [
            ExportTestData::new("InitializeASI", 0x1caa),
            ExportTestData::new("MainMemoryModInfo", 0x1c068),
            ExportTestData::new("ManiaModInfo", 0x1c068),
            ExportTestData::new("SA2ModInfo", 0x1c068),
            ExportTestData::new("SADXModInfo", 0x1c068),
            ExportTestData::new("SKCModInfo", 0x1c068),
            ExportTestData::new("SonicRModInfo", 0x1c068),
            ExportTestData::new("_get_hostfxr_path@12", 0xb1b0),
        ];

        for export in exports.iter() {
            let offset = unsafe { get_export_rva(pe_start, export.name, false, false, true) };
            assert_eq!(offset, export.expected_rva, "Mismatch for export {}", export.name);
        }
    }
}
