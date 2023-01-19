use crate::types;
use serde::{Deserialize, Serialize};

use super::SectionType;

/// A set of 64-bit ELF sections.
#[derive(Serialize, Deserialize)]
pub struct SectionSet64 {
    /// the set of the sections
    pub sections: Vec<Section64>,
}

/// A representation of an 64-bit ELF section.
#[derive(Serialize, Deserialize)]
pub struct Section64 {
    /// the section name that is corrected from .shstrtab
    pub name: String,
    /// the section header
    pub header: SectionHeader64,
    /// the section data
    pub data: SectionData64,
}

/// A representation of an 64-bit ELF section header.
#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct SectionHeader64 {
    /// sh_name
    pub name: types::Elf64Word,
    /// sh_type
    pub section_type: SectionType,
    /// sh_flags
    pub flags: types::Elf64Xword,
    /// sh_addr
    pub addr: types::Elf64Addr,
    /// sh_offset
    pub offset: types::Elf64Off,
    /// sh_size
    pub size: types::Elf64Xword,
    /// sh_link
    pub link: types::Elf64Word,
    /// sh_info
    pub info: types::Elf64Word,
    /// sh_addralign
    pub addralign: types::Elf64Xword,
    /// sh_entsize
    pub entsize: types::Elf64Xword,
}

impl From<RawSectionHeader64> for SectionHeader64 {
    fn from(raw_header: RawSectionHeader64) -> Self {
        Self {
            name: raw_header.sh_name,
            section_type: SectionType::from(raw_header.sh_type),
            flags: raw_header.sh_flags,
            addr: raw_header.sh_addr,
            offset: raw_header.sh_offset,
            size: raw_header.sh_size,
            link: raw_header.sh_link,
            info: raw_header.sh_info,
            addralign: raw_header.sh_addralign,
            entsize: raw_header.sh_entsize,
        }
    }
}

/// the actual section data
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SectionData64 {
    /// the section has no data
    Null,
    /// the section data as raw bytes
    Raw {
        #[serde(skip_serializing)]
        bytes: Vec<u8>,
    },
}

/// A raw representation of an 64-bit ELF section header like Elf64_Shdr.
#[derive(Default)]
#[repr(C)]
pub struct RawSectionHeader64 {
    /// section name (string table index)
    pub sh_name: types::Elf64Word,
    /// section type
    pub sh_type: types::Elf64Word,
    /// section flags
    pub sh_flags: types::Elf64Xword,
    /// section virtual addr at execution
    pub sh_addr: types::Elf64Addr,
    /// section file offset
    pub sh_offset: types::Elf64Off,
    /// section size in bytes
    pub sh_size: types::Elf64Xword,
    /// link to another section
    pub sh_link: types::Elf64Word,
    /// additional section information
    pub sh_info: types::Elf64Word,
    /// section alignment
    pub sh_addralign: types::Elf64Xword,
    /// entry size if section data represents a table
    pub sh_entsize: types::Elf64Xword,
}

impl RawSectionHeader64 {
    /// the size of the struct in memory.
    pub const SIZE: usize = 64;
}

mod tests {
    #[test]
    fn test_raw_section_header64_size() {
        assert_eq!(
            super::RawSectionHeader64::SIZE,
            std::mem::size_of::<super::RawSectionHeader64>()
        );
    }
}
