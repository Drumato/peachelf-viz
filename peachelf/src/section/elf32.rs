use crate::types;

#[repr(C)]
pub struct RawSectionHeader32 {
    pub sh_name: types::Elf32Word,
    pub sh_type: types::Elf32Word,
    pub sh_flags: types::Elf32Word,
    pub sh_addr: types::Elf32Addr,
    pub sh_offset: types::Elf32Off,
    pub sh_size: types::Elf32Word,
    pub sh_link: types::Elf32Word,
    pub sh_info: types::Elf32Word,
    pub sh_addralign: types::Elf32Word,
    pub sh_entsize: types::Elf32Word,
}

impl RawSectionHeader32 {
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
