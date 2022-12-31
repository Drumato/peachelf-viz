use serde::{Deserialize, Serialize};

use super::super::types;
use super::*;

#[derive(Default, PartialEq, Debug)]
#[repr(C)]
/// the raw representation of 64bit elf header.
pub struct RawHeader64 {
    pub e_ident: [u8; super::EI_NIDENT],
    pub e_type: types::Elf64Half,
    pub e_machine: types::Elf64Half,
    pub e_version: types::Elf64Word,
    pub e_entry: types::Elf64Addr,
    pub e_phoff: types::Elf64Off,
    pub e_shoff: types::Elf64Off,
    pub e_flags: types::Elf64Word,
    pub e_ehsize: types::Elf64Half,
    pub e_phentsize: types::Elf64Half,
    pub e_phnum: types::Elf64Half,
    pub e_shentsize: types::Elf64Half,
    pub e_shnum: types::Elf64Half,
    pub e_shstrndx: types::Elf64Half,
}

impl RawHeader64 {
    /// the entire size of 64bit elf header.
    pub const SIZE: usize = 64;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Header64 {
    pub class: ElfClass,
    pub data: ElfData,
    pub osabi: ElfOsAbi,
    pub osabi_version: u8,
    pub elf_type: ElfType,
    pub machine: ElfMachine,
    pub version: types::Elf64Word,
    pub entry: types::Elf64Addr,
    pub phoff: types::Elf64Off,
    pub shoff: types::Elf64Off,
    pub flags: types::Elf64Word,
    pub ehsize: types::Elf64Half,
    pub phentsize: types::Elf64Half,
    pub phnum: types::Elf64Half,
    pub shentsize: types::Elf64Half,
    pub shnum: types::Elf64Half,
    pub shstrndx: types::Elf64Half,
}

impl From<RawHeader64> for Header64 {
    fn from(raw_header: RawHeader64) -> Self {
        Self {
            class: ElfClass::from(raw_header.e_ident[EI_CLASS]),
            data: ElfData::from(raw_header.e_ident[EI_DATA]),
            osabi: ElfOsAbi::from(raw_header.e_ident[EI_OSABI]),
            osabi_version: raw_header.e_ident[EI_OSABI_VERSION],
            elf_type: ElfType::from(raw_header.e_type),
            machine: ElfMachine::from(raw_header.e_machine),
            version: raw_header.e_version,
            entry: raw_header.e_entry,
            phoff: raw_header.e_phoff,
            shoff: raw_header.e_shoff,
            flags: raw_header.e_flags,
            ehsize: raw_header.e_ehsize,
            phentsize: raw_header.e_phentsize,
            phnum: raw_header.e_phnum,
            shentsize: raw_header.e_shentsize,
            shnum: raw_header.e_shnum,
            shstrndx: raw_header.e_shstrndx,
        }
    }
}

mod tests {
    #[test]
    fn test_raw_header64_size() {
        assert_eq!(
            super::RawHeader64::SIZE,
            std::mem::size_of::<super::RawHeader64>()
        );
    }
}
