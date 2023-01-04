use crate::types;
use serde::{Deserialize, Serialize};

use super::SectionType;

#[derive(Serialize, Deserialize)]
pub struct SectionSet64 {
    pub sections: Vec<Section64>,
}

#[derive(Serialize, Deserialize)]
pub struct Section64 {
    pub name: String,
    pub data: SectionData64,
}

#[repr(C)]
pub struct SectionHeader64 {
    pub name: types::Elf64Word,
    pub section_type: SectionType,
    pub flags: types::Elf64Xword,
    pub addr: types::Elf64Addr,
    pub offset: types::Elf64Off,
    pub size: types::Elf64Xword,
    pub link: types::Elf64Word,
    pub info: types::Elf64Word,
    pub addralign: types::Elf64Xword,
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SectionData64 {
    Null,
    Raw {
        #[serde(skip_serializing)]
        bytes: Vec<u8>,
    },
}

#[derive(Default)]
#[repr(C)]
pub struct RawSectionHeader64 {
    pub sh_name: types::Elf64Word,
    pub sh_type: types::Elf64Word,
    pub sh_flags: types::Elf64Xword,
    pub sh_addr: types::Elf64Addr,
    pub sh_offset: types::Elf64Off,
    pub sh_size: types::Elf64Xword,
    pub sh_link: types::Elf64Word,
    pub sh_info: types::Elf64Word,
    pub sh_addralign: types::Elf64Xword,
    pub sh_entsize: types::Elf64Xword,
}

impl RawSectionHeader64 {
    pub const SIZE: usize = 64;
}

pub struct RawSectionSet64 {
    pub sections: RawSection64,
}

pub struct RawSection64 {
    pub header: RawSectionHeader64,
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
