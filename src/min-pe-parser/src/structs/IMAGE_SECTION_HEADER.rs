use crate::prelude::DataSectionFlags;
use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_SECTION_HEADER {
    /// Section name
    pub name: [u8; 8],
    /// Virtual size of section
    pub virtual_size: LittleEndian<u32>,
    /// Virtual address of section
    pub virtual_address: LittleEndian<u32>,
    /// Size of raw data in section
    pub size_of_raw_data: LittleEndian<u32>,
    /// Pointer to raw data in section
    pub pointer_to_raw_data: LittleEndian<u32>,
    /// Pointer to relocations in section
    pub pointer_to_relocations: LittleEndian<u32>,
    /// Pointer to line numbers in section
    pub pointer_to_linenumbers: LittleEndian<u32>,
    /// Number of relocations in section
    pub number_of_relocations: LittleEndian<u16>,
    /// Number of line numbers in section
    pub number_of_linenumbers: LittleEndian<u16>,
    /// Characteristics of section
    pub characteristics: DataSectionFlags,
}

impl IMAGE_SECTION_HEADER {
    /// Returns the virtual size of the section.
    pub fn virtual_size(&self) -> u32 {
        self.virtual_size.into()
    }

    /// Sets the virtual size of the section.
    pub fn set_virtual_size(&mut self, value: u32) {
        self.virtual_size = value.into();
    }

    /// Returns the virtual address of the section.
    pub fn virtual_address(&self) -> u32 {
        self.virtual_address.into()
    }

    /// Sets the virtual address of the section.
    pub fn set_virtual_address(&mut self, value: u32) {
        self.virtual_address = value.into();
    }

    /// Returns the size of raw data in the section.
    pub fn size_of_raw_data(&self) -> u32 {
        self.size_of_raw_data.into()
    }

    /// Sets the size of raw data in the section.
    pub fn set_size_of_raw_data(&mut self, value: u32) {
        self.size_of_raw_data = value.into();
    }

    /// Returns the pointer to raw data in the section.
    pub fn pointer_to_raw_data(&self) -> u32 {
        self.pointer_to_raw_data.into()
    }

    /// Sets the pointer to raw data in the section.
    pub fn set_pointer_to_raw_data(&mut self, value: u32) {
        self.pointer_to_raw_data = value.into();
    }

    /// Returns the pointer to relocations in the section.
    pub fn pointer_to_relocations(&self) -> u32 {
        self.pointer_to_relocations.into()
    }

    /// Sets the pointer to relocations in the section.
    pub fn set_pointer_to_relocations(&mut self, value: u32) {
        self.pointer_to_relocations = value.into();
    }

    /// Returns the pointer to line numbers in the section.
    pub fn pointer_to_linenumbers(&self) -> u32 {
        self.pointer_to_linenumbers.into()
    }

    /// Sets the pointer to line numbers in the section.
    pub fn set_pointer_to_linenumbers(&mut self, value: u32) {
        self.pointer_to_linenumbers = value.into();
    }

    /// Returns the number of relocations in the section.
    pub fn number_of_relocations(&self) -> u16 {
        self.number_of_relocations.into()
    }

    /// Sets the number of relocations in the section.
    pub fn set_number_of_relocations(&mut self, value: u16) {
        self.number_of_relocations = value.into();
    }

    /// Returns the number of line numbers in the section.
    pub fn number_of_linenumbers(&self) -> u16 {
        self.number_of_linenumbers.into()
    }

    /// Sets the number of line numbers in the section.
    pub fn set_number_of_linenumbers(&mut self, value: u16) {
        self.number_of_linenumbers = value.into();
    }

    /// Returns the characteristics of the section.
    pub fn characteristics(&self) -> DataSectionFlags {
        self.characteristics
    }

    /// Sets the characteristics of the section.
    pub fn set_characteristics(&mut self, value: DataSectionFlags) {
        self.characteristics = value;
    }
}
