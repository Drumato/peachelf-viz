use crate::types;

/// A section flag that means the section is writable
pub const SHF_WRITE: types::Elf64Xword = 0b0000_0000_0000_0001;
/// A section flag that means the section occupies memory during execution
pub const SHF_ALLOC: types::Elf64Xword = 0b0000_0000_0000_0010;
/// A section flag that means the section is executable
pub const SHF_EXECINSTR: types::Elf64Xword = 0b0000_0000_0000_0100;
/// A section flag that means the section might be merged
pub const SHF_MERGE: types::Elf64Xword = 0b0000_0000_0001_0000;
/// A section flag that means the section contains null-terminated strings
pub const SHF_STRINGS: types::Elf64Xword = 0b0000_0000_0010_0000;
/// A section flag that means the section's 'sh_info' contains section header table index
pub const SHF_INFO_LINK: types::Elf64Xword = 0b0000_0000_0100_0000;
/// A section flag that means the section preserves order after combining
pub const SHF_LINK_ORDER: types::Elf64Xword = 0b0000_0000_1000_0000;
/// A section flag that means the section is member of a group
pub const SHF_GROUP: types::Elf64Xword = 0b0000_0001_0000_0000;
/// A section flag that means the section holds thread-local data
pub const SHF_TLS: types::Elf64Xword = 0b0000_0010_0000_0000;
/// A section flag that means the section with compressed data
pub const SHF_COMPRESSED: types::Elf64Xword = 0b0000_0100_0000_0000;
