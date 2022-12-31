use crate::types;

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
    pub const SIZE: usize = 32;
}

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
