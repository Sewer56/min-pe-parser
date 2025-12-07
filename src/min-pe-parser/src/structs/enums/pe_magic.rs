use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PeMagic {
    value: u16,
}

impl PeMagic {
    pub const PE32: PeMagic = PeMagic { value: 0x10B };
    pub const PE64: PeMagic = PeMagic { value: 0x20B };
    pub const ROM: PeMagic = PeMagic { value: 0x107 };

    /// Creates a new `PeMagic` instance from a `u16`.
    pub fn new(value: u16) -> Self {
        PeMagic { value }
    }

    /// Returns the inner `u16` value.
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Checks if the magic is PE32.
    pub fn is_pe32(&self) -> bool {
        self.value == PeMagic::PE32.value
    }

    /// Checks if the magic is PE32+ (PE64).
    pub fn is_pe64(&self) -> bool {
        self.value == PeMagic::PE64.value
    }

    /// Checks if the magic is ROM.
    pub fn is_rom(&self) -> bool {
        self.value == PeMagic::ROM.value
    }
}

impl From<PeMagic> for LittleEndian<u16> {
    fn from(magic: PeMagic) -> Self {
        LittleEndian::from(magic.value())
    }
}

impl From<LittleEndian<u16>> for PeMagic {
    fn from(le: LittleEndian<u16>) -> Self {
        PeMagic::new(le.into())
    }
}

impl From<PeMagic> for u16 {
    fn from(magic: PeMagic) -> Self {
        u16::from_le(magic.value())
    }
}

impl From<u16> for PeMagic {
    fn from(value: u16) -> Self {
        PeMagic::new(u16::to_le(value))
    }
}
