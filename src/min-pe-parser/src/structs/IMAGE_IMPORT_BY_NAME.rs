use simple_endian::LittleEndian;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct IMAGE_IMPORT_BY_NAME {
    /// Hint
    pub hint: LittleEndian<u16>,
    /// Name bytes
    pub name_bytes: [u8; 1],
}

impl IMAGE_IMPORT_BY_NAME {
    /// Returns the hint.
    pub fn hint(&self) -> u16 {
        self.hint.into()
    }

    /// Sets the hint.
    pub fn set_hint(&mut self, value: u16) {
        self.hint = value.into();
    }
}
