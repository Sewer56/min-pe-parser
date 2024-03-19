use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_DATA_DIRECTORY {
    /// Virtual address
    pub virtual_address: LittleEndian<u32>,
    /// Size
    pub size: LittleEndian<u32>,
}

impl IMAGE_DATA_DIRECTORY {
    /// Returns the virtual address.
    pub fn virtual_address(&self) -> u32 {
        self.virtual_address.into()
    }

    /// Sets the virtual address.
    pub fn set_virtual_address(&mut self, value: u32) {
        self.virtual_address = value.into();
    }

    /// Returns the size.
    pub fn size(&self) -> u32 {
        self.size.into()
    }

    /// Sets the size.
    pub fn set_size(&mut self, value: u32) {
        self.size = value.into();
    }
}
