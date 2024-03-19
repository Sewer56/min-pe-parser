use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone)]
pub union IMAGE_THUNK_DATA64 {
    /// Forwarder string
    forwarder_string: LittleEndian<u64>,
    /// Function address
    function: LittleEndian<u64>,
    /// Ordinal
    ordinal: LittleEndian<u64>,
    /// Address of data
    address_of_data: LittleEndian<u64>,
}
