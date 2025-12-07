use simple_endian::LittleEndian;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_FILE_HEADER {
    /// Machine type
    pub machine: LittleEndian<u16>,
    /// Number of sections
    pub number_of_sections: LittleEndian<u16>,
    /// Time and date stamp
    pub time_date_stamp: LittleEndian<u32>,
    /// Pointer to symbol table
    pub pointer_to_symbol_table: LittleEndian<u32>,
    /// Number of symbols
    pub number_of_symbols: LittleEndian<u32>,
    /// Size of optional header
    pub size_of_optional_header: LittleEndian<u16>,
    /// Characteristics
    pub characteristics: LittleEndian<u16>,
}

impl IMAGE_FILE_HEADER {
    /// Returns the machine type.
    pub fn machine(&self) -> u16 {
        self.machine.into()
    }

    /// Sets the machine type.
    pub fn set_machine(&mut self, value: u16) {
        self.machine = value.into();
    }

    /// Returns the number of sections.
    pub fn number_of_sections(&self) -> u16 {
        self.number_of_sections.into()
    }

    /// Sets the number of sections.
    pub fn set_number_of_sections(&mut self, value: u16) {
        self.number_of_sections = value.into();
    }

    /// Returns the time and date stamp.
    pub fn time_date_stamp(&self) -> u32 {
        self.time_date_stamp.into()
    }

    /// Sets the time and date stamp.
    pub fn set_time_date_stamp(&mut self, value: u32) {
        self.time_date_stamp = value.into();
    }

    /// Returns the pointer to symbol table.
    pub fn pointer_to_symbol_table(&self) -> u32 {
        self.pointer_to_symbol_table.into()
    }

    /// Sets the pointer to symbol table.
    pub fn set_pointer_to_symbol_table(&mut self, value: u32) {
        self.pointer_to_symbol_table = value.into();
    }

    /// Returns the number of symbols.
    pub fn number_of_symbols(&self) -> u32 {
        self.number_of_symbols.into()
    }

    /// Sets the number of symbols.
    pub fn set_number_of_symbols(&mut self, value: u32) {
        self.number_of_symbols = value.into();
    }

    /// Returns the size of optional header.
    pub fn size_of_optional_header(&self) -> u16 {
        self.size_of_optional_header.into()
    }

    /// Sets the size of optional header.
    pub fn set_size_of_optional_header(&mut self, value: u16) {
        self.size_of_optional_header = value.into();
    }

    /// Returns the characteristics.
    pub fn characteristics(&self) -> u16 {
        self.characteristics.into()
    }

    /// Sets the characteristics.
    pub fn set_characteristics(&mut self, value: u16) {
        self.characteristics = value.into();
    }
}
