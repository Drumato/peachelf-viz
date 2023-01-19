use serde::{Deserialize, Serialize};

use crate::types;

use super::ProgramHeaderType;

/// A set of the 64-bit ELF program header.
#[derive(Serialize, Deserialize)]
pub struct ProgramHeaders64 {
    pub program_headers: Vec<ProgramHeader64>,
}

impl From<RawProgramHeader64> for ProgramHeader64 {
    fn from(raw_header: RawProgramHeader64) -> Self {
        Self {
            header_type: ProgramHeaderType::from(raw_header.p_type),
            offset: raw_header.p_offset,
            vaddr: raw_header.p_vaddr,
            paddr: raw_header.p_paddr,
            filesz: raw_header.p_filesz,
            memsz: raw_header.p_memsz,
            flags: raw_header.p_flags,
            align: raw_header.p_align,
        }
    }
}

/// A representation of an 64-bit ELF program header.
#[derive(Serialize, Deserialize)]
pub struct ProgramHeader64 {
    pub header_type: ProgramHeaderType,
    pub flags: types::Elf64Word,
    pub offset: types::Elf64Off,
    pub vaddr: types::Elf64Addr,
    pub paddr: types::Elf64Addr,
    pub filesz: types::Elf64Xword,
    pub memsz: types::Elf64Xword,
    pub align: types::Elf64Xword,
}

/// A raw representation of an 32-bit ELF program header like Elf32_Phdr.
#[repr(C)]
pub struct RawProgramHeader32 {
    pub p_type: types::Elf32Word,
    pub p_offset: types::Elf32Off,
    pub p_vaddr: types::Elf32Addr,
    pub p_paddr: types::Elf32Addr,
    pub p_filesz: types::Elf32Word,
    pub p_memsz: types::Elf32Word,
    pub p_flags: types::Elf32Word,
    pub p_align: types::Elf32Word,
}

impl RawProgramHeader32 {
    /// the size of the struct in memory.
    pub const SIZE: usize = 32;
}

/// A raw representation of an 64-bit ELF program header like Elf64_Phdr.
#[derive(Default)]
#[repr(C)]
pub struct RawProgramHeader64 {
    pub p_type: types::Elf64Word,
    pub p_flags: types::Elf64Word,
    pub p_offset: types::Elf64Off,
    pub p_vaddr: types::Elf64Addr,
    pub p_paddr: types::Elf64Addr,
    pub p_filesz: types::Elf64Xword,
    pub p_memsz: types::Elf64Xword,
    pub p_align: types::Elf64Xword,
}

impl RawProgramHeader64 {
    /// the size of the struct in memory.
    pub const SIZE: usize = 56;
}

mod tests {
    #[test]
    fn test_raw_program_header32_size() {
        assert_eq!(
            super::RawProgramHeader32::SIZE,
            std::mem::size_of::<super::RawProgramHeader32>()
        );
    }

    #[test]
    fn test_raw_program_header64_size() {
        assert_eq!(
            super::RawProgramHeader64::SIZE,
            std::mem::size_of::<super::RawProgramHeader64>()
        );
    }
}
