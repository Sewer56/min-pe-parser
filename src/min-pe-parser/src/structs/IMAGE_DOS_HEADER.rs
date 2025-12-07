use simple_endian::LittleEndian;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IMAGE_DOS_HEADER {
    /// Magic number
    pub e_magic: LittleEndian<u16>,
    /// Bytes on last page of file
    pub e_cblp: LittleEndian<u16>,
    /// Pages in file
    pub e_cp: LittleEndian<u16>,
    /// Relocations
    pub e_crlc: LittleEndian<u16>,
    /// Size of header in paragraphs
    pub e_cparhdr: LittleEndian<u16>,
    /// Minimum extra paragraphs needed
    pub e_minalloc: LittleEndian<u16>,
    /// Maximum extra paragraphs needed
    pub e_maxalloc: LittleEndian<u16>,
    /// Initial (relative) SS value
    pub e_ss: LittleEndian<u16>,
    /// Initial SP value
    pub e_sp: LittleEndian<u16>,
    /// Checksum
    pub e_csum: LittleEndian<u16>,
    /// Initial IP value
    pub e_ip: LittleEndian<u16>,
    /// Initial (relative) CS value
    pub e_cs: LittleEndian<u16>,
    /// File address of relocation table
    pub e_lfarlc: LittleEndian<u16>,
    /// Overlay number
    pub e_ovno: LittleEndian<u16>,
    /// Reserved words
    pub e_res_0: LittleEndian<u16>,
    /// Reserved words
    pub e_res_1: LittleEndian<u16>,
    /// Reserved words
    pub e_res_2: LittleEndian<u16>,
    /// Reserved words
    pub e_res_3: LittleEndian<u16>,
    /// OEM identifier (for e_oeminfo)
    pub e_oemid: LittleEndian<u16>,
    /// OEM information; e_oemid specific
    pub e_oeminfo: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_0: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_1: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_2: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_3: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_4: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_5: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_6: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_7: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_8: LittleEndian<u16>,
    /// Reserved words
    pub e_res2_9: LittleEndian<u16>,
    /// File address of new exe header
    pub e_lfanew: LittleEndian<u32>,
}

impl IMAGE_DOS_HEADER {
    /// Returns the magic number.
    pub fn e_magic(&self) -> u16 {
        self.e_magic.into()
    }

    /// Sets the magic number.
    pub fn set_e_magic(&mut self, value: u16) {
        self.e_magic = value.into();
    }

    /// Returns the bytes on the last page of the file.
    pub fn e_cblp(&self) -> u16 {
        self.e_cblp.into()
    }

    /// Sets the bytes on the last page of the file.
    pub fn set_e_cblp(&mut self, value: u16) {
        self.e_cblp = value.into();
    }

    /// Returns the pages in the file.
    pub fn e_cp(&self) -> u16 {
        self.e_cp.into()
    }

    /// Sets the pages in the file.
    pub fn set_e_cp(&mut self, value: u16) {
        self.e_cp = value.into();
    }

    /// Returns the relocations.
    pub fn e_crlc(&self) -> u16 {
        self.e_crlc.into()
    }

    /// Sets the relocations.
    pub fn set_e_crlc(&mut self, value: u16) {
        self.e_crlc = value.into();
    }

    /// Returns the size of the header in paragraphs.
    pub fn e_cparhdr(&self) -> u16 {
        self.e_cparhdr.into()
    }

    /// Sets the size of the header in paragraphs.
    pub fn set_e_cparhdr(&mut self, value: u16) {
        self.e_cparhdr = value.into();
    }

    /// Returns the minimum extra paragraphs needed.
    pub fn e_minalloc(&self) -> u16 {
        self.e_minalloc.into()
    }

    /// Sets the minimum extra paragraphs needed.
    pub fn set_e_minalloc(&mut self, value: u16) {
        self.e_minalloc = value.into();
    }

    /// Returns the maximum extra paragraphs needed.
    pub fn e_maxalloc(&self) -> u16 {
        self.e_maxalloc.into()
    }

    /// Sets the maximum extra paragraphs needed.
    pub fn set_e_maxalloc(&mut self, value: u16) {
        self.e_maxalloc = value.into();
    }

    /// Returns the initial (relative) SS value.
    pub fn e_ss(&self) -> u16 {
        self.e_ss.into()
    }

    /// Sets the initial (relative) SS value.
    pub fn set_e_ss(&mut self, value: u16) {
        self.e_ss = value.into();
    }

    /// Returns the initial SP value.
    pub fn e_sp(&self) -> u16 {
        self.e_sp.into()
    }

    /// Sets the initial SP value.
    pub fn set_e_sp(&mut self, value: u16) {
        self.e_sp = value.into();
    }

    /// Returns the checksum.
    pub fn e_csum(&self) -> u16 {
        self.e_csum.into()
    }

    /// Sets the checksum.
    pub fn set_e_csum(&mut self, value: u16) {
        self.e_csum = value.into();
    }

    /// Returns the initial IP value.
    pub fn e_ip(&self) -> u16 {
        self.e_ip.into()
    }

    /// Sets the initial IP value.
    pub fn set_e_ip(&mut self, value: u16) {
        self.e_ip = value.into();
    }

    /// Returns the initial (relative) CS value.
    pub fn e_cs(&self) -> u16 {
        self.e_cs.into()
    }

    /// Sets the initial (relative) CS value.
    pub fn set_e_cs(&mut self, value: u16) {
        self.e_cs = value.into();
    }

    /// Returns the file address of the relocation table.
    pub fn e_lfarlc(&self) -> u16 {
        self.e_lfarlc.into()
    }

    /// Sets the file address of the relocation table.
    pub fn set_e_lfarlc(&mut self, value: u16) {
        self.e_lfarlc = value.into();
    }

    /// Returns the overlay number.
    pub fn e_ovno(&self) -> u16 {
        self.e_ovno.into()
    }

    /// Sets the overlay number.
    pub fn set_e_ovno(&mut self, value: u16) {
        self.e_ovno = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res_0(&self) -> u16 {
        self.e_res_0.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res_0(&mut self, value: u16) {
        self.e_res_0 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res_1(&self) -> u16 {
        self.e_res_1.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res_1(&mut self, value: u16) {
        self.e_res_1 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res_2(&self) -> u16 {
        self.e_res_2.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res_2(&mut self, value: u16) {
        self.e_res_2 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res_3(&self) -> u16 {
        self.e_res_3.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res_3(&mut self, value: u16) {
        self.e_res_3 = value.into();
    }

    /// Returns the OEM identifier (for e_oeminfo).
    pub fn e_oemid(&self) -> u16 {
        self.e_oemid.into()
    }

    /// Sets the OEM identifier (for e_oeminfo).
    pub fn set_e_oemid(&mut self, value: u16) {
        self.e_oemid = value.into();
    }

    /// Returns the OEM information; e_oemid specific.
    pub fn e_oeminfo(&self) -> u16 {
        self.e_oeminfo.into()
    }

    /// Sets the OEM information; e_oemid specific.
    pub fn set_e_oeminfo(&mut self, value: u16) {
        self.e_oeminfo = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_0(&self) -> u16 {
        self.e_res2_0.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_0(&mut self, value: u16) {
        self.e_res2_0 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_1(&self) -> u16 {
        self.e_res2_1.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_1(&mut self, value: u16) {
        self.e_res2_1 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_2(&self) -> u16 {
        self.e_res2_2.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_2(&mut self, value: u16) {
        self.e_res2_2 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_3(&self) -> u16 {
        self.e_res2_3.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_3(&mut self, value: u16) {
        self.e_res2_3 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_4(&self) -> u16 {
        self.e_res2_4.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_4(&mut self, value: u16) {
        self.e_res2_4 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_5(&self) -> u16 {
        self.e_res2_5.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_5(&mut self, value: u16) {
        self.e_res2_5 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_6(&self) -> u16 {
        self.e_res2_6.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_6(&mut self, value: u16) {
        self.e_res2_6 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_7(&self) -> u16 {
        self.e_res2_7.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_7(&mut self, value: u16) {
        self.e_res2_7 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_8(&self) -> u16 {
        self.e_res2_8.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_8(&mut self, value: u16) {
        self.e_res2_8 = value.into();
    }

    /// Returns the reserved word.
    pub fn e_res2_9(&self) -> u16 {
        self.e_res2_9.into()
    }

    /// Sets the reserved word.
    pub fn set_e_res2_9(&mut self, value: u16) {
        self.e_res2_9 = value.into();
    }

    /// Returns the file address of the new exe header.
    pub fn e_lfanew(&self) -> u32 {
        self.e_lfanew.into()
    }

    /// Sets the file address of the new exe header.
    pub fn set_e_lfanew(&mut self, value: u32) {
        self.e_lfanew = value.into();
    }
}
