use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_EXPORT_DIRECTORY {
    /// Reserved, must be 0.
    pub characteristics: LittleEndian<u32>,
    /// The time and date that the export data was created.
    pub time_date_stamp: LittleEndian<u32>,
    /// The major version number. The version number is specified by the user.
    pub major_version: LittleEndian<u16>,
    /// The minor version number. The version number is specified by the user.
    pub minor_version: LittleEndian<u16>,
    /// The address of the ASCII string that contains the name of the DLL.
    /// This address is relative to the image base.
    pub name: LittleEndian<u32>,
    /// The starting ordinal number for exports in this image.
    /// This field specifies the starting ordinal number for the export address table.
    /// It is usually set to 1.
    pub base: LittleEndian<u32>,
    /// The number of entries in the export address table.
    pub number_of_functions: LittleEndian<u32>,
    /// The number of entries in the name pointer table.
    /// This is also the number of entries in the ordinal table.
    pub number_of_names: LittleEndian<u32>,
    /// The address of the export address table, relative to the image base.
    /// The table size is given by the Number of Functions field.
    pub address_of_functions: LittleEndian<u32>,
    /// The address of the export name pointer table, relative to the image base.
    /// The table size is given by the Number of Names field.
    pub address_of_names: LittleEndian<u32>,
    /// The address of the ordinal table, relative to the image base.
    pub address_of_name_ordinals: LittleEndian<u32>,
}

impl IMAGE_EXPORT_DIRECTORY {
    /// Returns the characteristics. Reserved, must be 0.
    pub fn characteristics(&self) -> u32 {
        self.characteristics.into()
    }

    /// Sets the characteristics. Reserved, must be 0.
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = value.into();
    }

    /// Returns the creation time and date of the export data.
    pub fn time_date_stamp(&self) -> u32 {
        self.time_date_stamp.into()
    }

    /// Sets the creation time and date of the export data.
    pub fn set_time_date_stamp(&mut self, value: u32) {
        self.time_date_stamp = value.into();
    }

    /// Returns the major version number specified by the user.
    pub fn major_version(&self) -> u16 {
        self.major_version.into()
    }

    /// Sets the major version number specified by the user.
    pub fn set_major_version(&mut self, value: u16) {
        self.major_version = value.into();
    }

    /// Returns the minor version number specified by the user.
    pub fn minor_version(&self) -> u16 {
        self.minor_version.into()
    }

    /// Sets the minor version number specified by the user.
    pub fn set_minor_version(&mut self, value: u16) {
        self.minor_version = value.into();
    }

    /// Returns the address of the DLL name ASCII string, relative to the image base.
    pub fn name(&self) -> u32 {
        self.name.into()
    }

    /// Sets the address of the DLL name ASCII string, relative to the image base.
    pub fn set_name(&mut self, value: u32) {
        self.name = value.into();
    }

    /// Returns the starting ordinal number for exports in this image.
    pub fn base(&self) -> u32 {
        self.base.into()
    }

    /// Sets the starting ordinal number for exports in this image.
    pub fn set_base(&mut self, value: u32) {
        self.base = value.into();
    }

    /// Returns the number of entries in the export address table.
    pub fn number_of_functions(&self) -> u32 {
        self.number_of_functions.into()
    }

    /// Sets the number of entries in the export address table.
    pub fn set_number_of_functions(&mut self, value: u32) {
        self.number_of_functions = value.into();
    }

    /// Returns the number of entries in the export name pointer table.
    pub fn number_of_names(&self) -> u32 {
        self.number_of_names.into()
    }

    /// Sets the number of entries in the export name pointer table.
    pub fn set_number_of_names(&mut self, value: u32) {
        self.number_of_names = value.into();
    }

    /// Returns the address of the export address table, relative to the image base.
    pub fn address_of_functions(&self) -> u32 {
        self.address_of_functions.into()
    }

    /// Sets the address of the export address table, relative to the image base.
    pub fn set_address_of_functions(&mut self, value: u32) {
        self.address_of_functions = value.into();
    }

    /// Returns the address of the export name pointer table, relative to the image base.
    pub fn address_of_names(&self) -> u32 {
        self.address_of_names.into()
    }

    /// Sets the address of the export name pointer table, relative to the image base.
    pub fn set_address_of_names(&mut self, value: u32) {
        self.address_of_names = value.into();
    }

    /// Returns the address of the ordinal table, relative to the image base.
    pub fn address_of_name_ordinals(&self) -> u32 {
        self.address_of_name_ordinals.into()
    }

    /// Sets the address of the ordinal table, relative to the image base.
    pub fn set_address_of_name_ordinals(&mut self, value: u32) {
        self.address_of_name_ordinals = value.into();
    }
}
