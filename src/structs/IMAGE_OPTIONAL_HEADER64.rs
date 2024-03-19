use crate::prelude::PeMagic;
use simple_endian::LittleEndian;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    /// Magic number
    pub magic: PeMagic,
    /// Major linker version
    pub major_linker_version: u8,
    /// Minor linker version
    pub minor_linker_version: u8,
    /// Size of code section
    pub size_of_code: LittleEndian<u32>,
    /// Size of initialized data section
    pub size_of_initialized_data: LittleEndian<u32>,
    /// Size of uninitialized data section
    pub size_of_uninitialized_data: LittleEndian<u32>,
    /// Entry point address
    pub address_of_entry_point: LittleEndian<u32>,
    /// Base address of code section
    pub base_of_code: LittleEndian<u32>,
    /// Image base address
    pub image_base: LittleEndian<u64>,
    /// Section alignment
    pub section_alignment: LittleEndian<u32>,
    /// File alignment
    pub file_alignment: LittleEndian<u32>,
    /// Major operating system version
    pub major_operating_system_version: LittleEndian<u16>,
    /// Minor operating system version
    pub minor_operating_system_version: LittleEndian<u16>,
    /// Major image version
    pub major_image_version: LittleEndian<u16>,
    /// Minor image version
    pub minor_image_version: LittleEndian<u16>,
    /// Major subsystem version
    pub major_subsystem_version: LittleEndian<u16>,
    /// Minor subsystem version
    pub minor_subsystem_version: LittleEndian<u16>,
    /// Win32 version value
    pub win32_version_value: LittleEndian<u32>,
    /// Size of image
    pub size_of_image: LittleEndian<u32>,
    /// Size of headers
    pub size_of_headers: LittleEndian<u32>,
    /// Checksum
    pub check_sum: LittleEndian<u32>,
    /// Subsystem
    pub subsystem: LittleEndian<u16>,
    /// DLL characteristics
    pub dll_characteristics: LittleEndian<u16>,
    /// Size of stack reserve
    pub size_of_stack_reserve: LittleEndian<u64>,
    /// Size of stack commit
    pub size_of_stack_commit: LittleEndian<u64>,
    /// Size of heap reserve
    pub size_of_heap_reserve: LittleEndian<u64>,
    /// Size of heap commit
    pub size_of_heap_commit: LittleEndian<u64>,
    /// Loader flags
    pub loader_flags: LittleEndian<u32>,
    /// Number of RVA and sizes
    pub number_of_rva_and_sizes: LittleEndian<u32>,
}

impl IMAGE_OPTIONAL_HEADER64 {
    /// Returns the magic number.
    pub fn magic(&self) -> PeMagic {
        self.magic
    }

    /// Sets the magic number.
    pub fn set_magic(&mut self, value: u16) {
        self.magic = value.into();
    }

    /// Returns the major linker version.
    pub fn major_linker_version(&self) -> u8 {
        self.major_linker_version
    }

    /// Sets the major linker version.
    pub fn set_major_linker_version(&mut self, value: u8) {
        self.major_linker_version = value;
    }

    /// Returns the minor linker version.
    pub fn minor_linker_version(&self) -> u8 {
        self.minor_linker_version
    }

    /// Sets the minor linker version.
    pub fn set_minor_linker_version(&mut self, value: u8) {
        self.minor_linker_version = value;
    }

    /// Returns the size of the code section.
    pub fn size_of_code(&self) -> u32 {
        self.size_of_code.into()
    }

    /// Sets the size of the code section.
    pub fn set_size_of_code(&mut self, value: u32) {
        self.size_of_code = value.into();
    }

    /// Returns the size of initialized data section.
    pub fn size_of_initialized_data(&self) -> u32 {
        self.size_of_initialized_data.into()
    }

    /// Sets the size of initialized data section.
    pub fn set_size_of_initialized_data(&mut self, value: u32) {
        self.size_of_initialized_data = value.into();
    }

    /// Returns the size of uninitialized data section.
    pub fn size_of_uninitialized_data(&self) -> u32 {
        self.size_of_uninitialized_data.into()
    }

    /// Sets the size of uninitialized data section.
    pub fn set_size_of_uninitialized_data(&mut self, value: u32) {
        self.size_of_uninitialized_data = value.into();
    }

    /// Returns the entry point address.
    pub fn address_of_entry_point(&self) -> u32 {
        self.address_of_entry_point.into()
    }

    /// Sets the entry point address.
    pub fn set_address_of_entry_point(&mut self, value: u32) {
        self.address_of_entry_point = value.into();
    }

    /// Returns the base address of the code section.
    pub fn base_of_code(&self) -> u32 {
        self.base_of_code.into()
    }

    /// Sets the base address of the code section.
    pub fn set_base_of_code(&mut self, value: u32) {
        self.base_of_code = value.into();
    }

    /// Returns the image base address.
    pub fn image_base(&self) -> u64 {
        self.image_base.into()
    }

    /// Sets the image base address.
    pub fn set_image_base(&mut self, value: u64) {
        self.image_base = value.into();
    }

    /// Returns the section alignment.
    pub fn section_alignment(&self) -> u32 {
        self.section_alignment.into()
    }

    /// Sets the section alignment.
    pub fn set_section_alignment(&mut self, value: u32) {
        self.section_alignment = value.into();
    }

    /// Returns the file alignment.
    pub fn file_alignment(&self) -> u32 {
        self.file_alignment.into()
    }

    /// Sets the file alignment.
    pub fn set_file_alignment(&mut self, value: u32) {
        self.file_alignment = value.into();
    }

    /// Returns the major operating system version.
    pub fn major_operating_system_version(&self) -> u16 {
        self.major_operating_system_version.into()
    }

    /// Sets the major operating system version.
    pub fn set_major_operating_system_version(&mut self, value: u16) {
        self.major_operating_system_version = value.into();
    }

    /// Returns the minor operating system version.
    pub fn minor_operating_system_version(&self) -> u16 {
        self.minor_operating_system_version.into()
    }

    /// Sets the minor operating system version.
    pub fn set_minor_operating_system_version(&mut self, value: u16) {
        self.minor_operating_system_version = value.into();
    }

    /// Returns the major image version.
    pub fn major_image_version(&self) -> u16 {
        self.major_image_version.into()
    }

    /// Sets the major image version.
    pub fn set_major_image_version(&mut self, value: u16) {
        self.major_image_version = value.into();
    }

    /// Returns the minor image version.
    pub fn minor_image_version(&self) -> u16 {
        self.minor_image_version.into()
    }

    /// Sets the minor image version.
    pub fn set_minor_image_version(&mut self, value: u16) {
        self.minor_image_version = value.into();
    }

    /// Returns the major subsystem version.
    pub fn major_subsystem_version(&self) -> u16 {
        self.major_subsystem_version.into()
    }

    /// Sets the major subsystem version.
    pub fn set_major_subsystem_version(&mut self, value: u16) {
        self.major_subsystem_version = value.into();
    }

    /// Returns the minor subsystem version.
    pub fn minor_subsystem_version(&self) -> u16 {
        self.minor_subsystem_version.into()
    }

    /// Sets the minor subsystem version.
    pub fn set_minor_subsystem_version(&mut self, value: u16) {
        self.minor_subsystem_version = value.into();
    }

    /// Returns the Win32 version value.
    pub fn win32_version_value(&self) -> u32 {
        self.win32_version_value.into()
    }

    /// Sets the Win32 version value.
    pub fn set_win32_version_value(&mut self, value: u32) {
        self.win32_version_value = value.into();
    }

    /// Returns the size of image.
    pub fn size_of_image(&self) -> u32 {
        self.size_of_image.into()
    }

    /// Sets the size of image.
    pub fn set_size_of_image(&mut self, value: u32) {
        self.size_of_image = value.into();
    }

    /// Returns the size of headers.
    pub fn size_of_headers(&self) -> u32 {
        self.size_of_headers.into()
    }

    /// Sets the size of headers.
    pub fn set_size_of_headers(&mut self, value: u32) {
        self.size_of_headers = value.into();
    }

    /// Returns the checksum.
    pub fn check_sum(&self) -> u32 {
        self.check_sum.into()
    }

    /// Sets the checksum.
    pub fn set_check_sum(&mut self, value: u32) {
        self.check_sum = value.into();
    }

    /// Returns the subsystem.
    pub fn subsystem(&self) -> u16 {
        self.subsystem.into()
    }

    /// Sets the subsystem.
    pub fn set_subsystem(&mut self, value: u16) {
        self.subsystem = value.into();
    }

    /// Returns the DLL characteristics.
    pub fn dll_characteristics(&self) -> u16 {
        self.dll_characteristics.into()
    }

    /// Sets the DLL characteristics.
    pub fn set_dll_characteristics(&mut self, value: u16) {
        self.dll_characteristics = value.into();
    }

    /// Returns the size of stack reserve.
    pub fn size_of_stack_reserve(&self) -> u64 {
        self.size_of_stack_reserve.into()
    }

    /// Sets the size of stack reserve.
    pub fn set_size_of_stack_reserve(&mut self, value: u64) {
        self.size_of_stack_reserve = value.into();
    }

    /// Returns the size of stack commit.
    pub fn size_of_stack_commit(&self) -> u64 {
        self.size_of_stack_commit.into()
    }

    /// Sets the size of stack commit.
    pub fn set_size_of_stack_commit(&mut self, value: u64) {
        self.size_of_stack_commit = value.into();
    }

    /// Returns the size of heap reserve.
    pub fn size_of_heap_reserve(&self) -> u64 {
        self.size_of_heap_reserve.into()
    }

    /// Sets the size of heap reserve.
    pub fn set_size_of_heap_reserve(&mut self, value: u64) {
        self.size_of_heap_reserve = value.into();
    }

    /// Returns the size of heap commit.
    pub fn size_of_heap_commit(&self) -> u64 {
        self.size_of_heap_commit.into()
    }

    /// Sets the size of heap commit.
    pub fn set_size_of_heap_commit(&mut self, value: u64) {
        self.size_of_heap_commit = value.into();
    }

    /// Returns the loader flags.
    pub fn loader_flags(&self) -> u32 {
        self.loader_flags.into()
    }

    /// Sets the loader flags.
    pub fn set_loader_flags(&mut self, value: u32) {
        self.loader_flags = value.into();
    }

    /// Returns the number of RVA and sizes.
    pub fn number_of_rva_and_sizes(&self) -> u32 {
        self.number_of_rva_and_sizes.into()
    }

    /// Sets the number of RVA and sizes.
    pub fn set_number_of_rva_and_sizes(&mut self, value: u32) {
        self.number_of_rva_and_sizes = value.into();
    }
}
