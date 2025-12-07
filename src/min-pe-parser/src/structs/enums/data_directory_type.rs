use crate::prelude::*;
use core::mem::size_of;

pub const SIZE_OF_DATA_DIRECTORIES: usize =
    size_of::<IMAGE_DATA_DIRECTORY>() * (DataDirectoryType::Reserved as usize + 1);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataDirectoryType {
    /// Export Table
    ExportTable = 0,
    /// Import Table
    ImportTable,
    /// Resource Table
    ResourceTable,
    /// Exception Table
    ExceptionTable,
    /// Certificate Table
    CertificateTable,
    /// Base Relocation Table
    BaseRelocationTable,
    /// Debug
    Debug,
    /// Architecture
    Architecture,
    /// Global Pointer
    GlobalPtr,
    /// TLS Table
    TLSTable,
    /// Load Config Table
    LoadConfigTable,
    /// Bound Import
    BoundImport,
    /// Import Address Table
    IAT,
    /// Delay Import Descriptor
    DelayImportDescriptor,
    /// CLR Runtime Header
    CLRRuntimeHeader,
    /// Reserved
    Reserved,
}
