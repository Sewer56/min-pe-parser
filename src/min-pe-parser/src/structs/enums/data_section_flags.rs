use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct DataSectionFlags: u32 {
        /// Reserved for future use.
        const TYPE_REG = 0x00000000;
        /// Reserved for future use.
        const TYPE_DSECT = 0x00000001;
        /// Reserved for future use.
        const TYPE_NO_LOAD = 0x00000002;
        /// Reserved for future use.
        const TYPE_GROUP = 0x00000004;
        /// The section should not be padded to the next boundary. This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES. This is valid only for object files.
        const TYPE_NO_PADDED = 0x00000008;
        /// Reserved for future use.
        const TYPE_COPY = 0x00000010;
        /// The section contains executable code.
        const CONTENT_CODE = 0x00000020;
        /// The section contains initialized data.
        const CONTENT_INITIALIZED_DATA = 0x00000040;
        /// The section contains uninitialized data.
        const CONTENT_UNINITIALIZED_DATA = 0x00000080;
        /// Reserved for future use.
        const LINK_OTHER = 0x00000100;
        /// The section contains comments or other information. The .drectve section has this type. This is valid for object files only.
        const LINK_INFO = 0x00000200;
        /// Reserved for future use.
        const TYPE_OVER = 0x00000400;
        /// The section will not become part of the image. This is valid only for object files.
        const LINK_REMOVE = 0x00000800;
        /// The section contains COMDAT data. For more information, see section 5.5.6, COMDAT Sections (Object Only). This is valid only for object files.
        const LINK_COM_DAT = 0x00001000;
        /// Reset speculative exceptions handling bits in the TLB entries for this section.
        const NO_DEFER_SPEC_EXCEPTIONS = 0x00004000;
        /// The section contains data referenced through the global pointer (GP).
        const RELATIVE_GP = 0x00008000;
        /// Reserved for future use.
        const MEM_PURGEABLE = 0x00020000;
        /// Reserved for future use.
        const MEMORY16_BIT = 0x00020000;
        /// Reserved for future use.
        const MEMORY_LOCKED = 0x00040000;
        /// Reserved for future use.
        const MEMORY_PRELOAD = 0x00080000;
        /// Align data on a 1-byte boundary. Valid only for object files.
        const ALIGN_1_BYTES = 0x00100000;
        /// Align data on a 2-byte boundary. Valid only for object files.
        const ALIGN_2_BYTES = 0x00200000;
        /// Align data on a 4-byte boundary. Valid only for object files.
        const ALIGN_4_BYTES = 0x00300000;
        /// Align data on an 8-byte boundary. Valid only for object files.
        const ALIGN_8_BYTES = 0x00400000;
        /// Align data on a 16-byte boundary. Valid only for object files.
        const ALIGN_16_BYTES = 0x00500000;
        /// Align data on a 32-byte boundary. Valid only for object files.
        const ALIGN_32_BYTES = 0x00600000;
        /// Align data on a 64-byte boundary. Valid only for object files.
        const ALIGN_64_BYTES = 0x00700000;
        /// Align data on a 128-byte boundary. Valid only for object files.
        const ALIGN_128_BYTES = 0x00800000;
        /// Align data on a 256-byte boundary. Valid only for object files.
        const ALIGN_256_BYTES = 0x00900000;
        /// Align data on a 512-byte boundary. Valid only for object files.
        const ALIGN_512_BYTES = 0x00A00000;
        /// Align data on a 1024-byte boundary. Valid only for object files.
        const ALIGN_1024_BYTES = 0x00B00000;
        /// Align data on a 2048-byte boundary. Valid only for object files.
        const ALIGN_2048_BYTES = 0x00C00000;
        /// Align data on a 4096-byte boundary. Valid only for object files.
        const ALIGN_4096_BYTES = 0x00D00000;
        /// Align data on an 8192-byte boundary. Valid only for object files.
        const ALIGN_8192_BYTES = 0x00E00000;
        /// The section contains extended relocations.
        const LINK_EXTENDED_RELOCATION_OVERFLOW = 0x01000000;
        /// The section can be discarded as needed.
        const MEMORY_DISCARDABLE = 0x02000000;
        /// The section cannot be cached.
        const MEMORY_NOT_CACHED = 0x04000000;
        /// The section is not pageable.
        const MEMORY_NOT_PAGED = 0x08000000;
        /// The section can be shared in memory.
        const MEMORY_SHARED = 0x10000000;
        /// The section can be executed as code.
        const MEMORY_EXECUTE = 0x20000000;
        /// The section can be read.
        const MEMORY_READ = 0x40000000;
        /// The section can be written to.
        const MEMORY_WRITE = 0x80000000;
    }
}
