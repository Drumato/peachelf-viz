use crate::types;

pub const SHF_WRITE: types::Elf64Xword = 0b0000_0000_0000_0001;
pub const SHF_ALLOC: types::Elf64Xword = 0b0000_0000_0000_0010;
pub const SHF_EXECINSTR: types::Elf64Xword = 0b0000_0000_0000_0100;
pub const SHF_MERGE: types::Elf64Xword = 0b0000_0000_0001_0000;
pub const SHF_STRINGS: types::Elf64Xword = 0b0000_0000_0010_0000;
pub const SHF_INFO_LINK: types::Elf64Xword = 0b0000_0000_0100_0000;
pub const SHF_LINK_ORDER: types::Elf64Xword = 0b0000_0000_1000_0000;
pub const SHF_GROUP: types::Elf64Xword = 0b0000_0001_0000_0000;
pub const SHF_TLS: types::Elf64Xword = 0b0000_0010_0000_0000;
pub const SHF_COMPRESSED: types::Elf64Xword = 0b0000_0100_0000_0000;
