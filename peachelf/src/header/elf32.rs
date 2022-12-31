use super::super::types;
use super::*;

#[repr(C)]
/// the raw representation of 32bit elf header.
pub struct RawHeader32 {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: types::Elf32Half,
    pub e_machine: types::Elf32Half,
    pub e_version: types::Elf32Word,
    pub e_entry: types::Elf32Addr,
    pub e_phoff: types::Elf32Off,
    pub e_shoff: types::Elf32Off,
    pub e_flags: types::Elf32Word,
    pub e_ehsize: types::Elf32Half,
    pub e_phentsize: types::Elf32Half,
    pub e_phnum: types::Elf32Half,
    pub e_shentsize: types::Elf32Half,
    pub e_shnum: types::Elf32Half,
    pub e_shstrndx: types::Elf32Half,
}

impl RawHeader32 {
    pub const SIZE: usize = 52;
}

mod tests {
    #[test]
    fn test_raw_header32_size() {
        assert_eq!(
            super::RawHeader32::SIZE,
            std::mem::size_of::<super::RawHeader32>()
        );
    }
}
