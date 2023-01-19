use crate::types;

/// A raw representation of an 32-bit ELF section header like Elf32_Shdr.
#[repr(C)]
pub struct RawSectionHeader32 {
    /// section name (string table index)
    pub sh_name: types::Elf32Word,
    /// section type
    pub sh_type: types::Elf32Word,
    /// section flags
    pub sh_flags: types::Elf32Word,
    /// section virtual addr at execution
    pub sh_addr: types::Elf32Addr,
    /// section file offset
    pub sh_offset: types::Elf32Off,
    /// section size in bytes
    pub sh_size: types::Elf32Word,
    /// link to another section
    pub sh_link: types::Elf32Word,
    /// additional section information
    pub sh_info: types::Elf32Word,
    /// section alignment
    pub sh_addralign: types::Elf32Word,
    /// entry size if section data represents a table
    pub sh_entsize: types::Elf32Word,
}

impl RawSectionHeader32 {
    /// the size of the struct in memory.
    pub const SIZE: usize = 40;
}

mod tests {
    #[test]
    fn test_raw_section_header32_size() {
        assert_eq!(
            super::RawSectionHeader32::SIZE,
            std::mem::size_of::<super::RawSectionHeader32>()
        );
    }
}
