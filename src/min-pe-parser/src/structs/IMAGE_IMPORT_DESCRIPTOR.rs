use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    /// Points to the first ImageImportByName struct
    pub original_first_thunk: LittleEndian<u32>,
    /// Time and date stamp
    pub time_date_stamp: LittleEndian<u32>,
    /// Forwarder Chain
    pub forwarder_chain: LittleEndian<u32>,
    /// RVA to the name of the DLL
    pub name: LittleEndian<u32>,
    /// Points to an ImageImportByName struct or to the address of the first function
    pub first_thunk: LittleEndian<u32>,
}

impl IMAGE_IMPORT_DESCRIPTOR {
    /// Returns the original first thunk.
    pub fn original_first_thunk(&self) -> u32 {
        self.original_first_thunk.into()
    }

    /// Sets the original first thunk.
    pub fn set_original_first_thunk(&mut self, value: u32) {
        self.original_first_thunk = value.into();
    }

    /// Returns the time and date stamp.
    pub fn time_date_stamp(&self) -> u32 {
        self.time_date_stamp.into()
    }

    /// Sets the time and date stamp.
    pub fn set_time_date_stamp(&mut self, value: u32) {
        self.time_date_stamp = value.into();
    }

    /// Returns the forwarder chain.
    pub fn forwarder_chain(&self) -> u32 {
        self.forwarder_chain.into()
    }

    /// Sets the forwarder chain.
    pub fn set_forwarder_chain(&mut self, value: u32) {
        self.forwarder_chain = value.into();
    }

    /// Returns the name.
    pub fn name(&self) -> u32 {
        self.name.into()
    }

    /// Sets the name.
    pub fn set_name(&mut self, value: u32) {
        self.name = value.into();
    }

    /// Returns the first thunk.
    pub fn first_thunk(&self) -> u32 {
        self.first_thunk.into()
    }

    /// Sets the first thunk.
    pub fn set_first_thunk(&mut self, value: u32) {
        self.first_thunk = value.into();
    }
}
