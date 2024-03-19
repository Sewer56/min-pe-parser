use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone)]
pub union IMAGE_THUNK_DATA32 {
    /// Forwarder string
    forwarder_string: LittleEndian<u32>,
    /// Function address
    function: LittleEndian<u32>,
    /// Ordinal
    ordinal: LittleEndian<u32>,
    /// Address of data
    address_of_data: LittleEndian<u32>,
}
