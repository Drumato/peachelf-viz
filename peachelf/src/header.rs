mod elf64;
pub use elf64::*;

mod elf32;
pub use elf32::*;

mod class;
pub use class::*;

mod data;
pub use data::*;

mod osabi;
pub use osabi::*;

mod elf_type;
pub use elf_type::*;

mod machine;
pub use machine::*;

/// the first byte of the ELF magic number
pub const ELF_MAGIC1: u8 = 0x7f;
/// the second byte of the ELF magic number
pub const ELF_MAGIC2: u8 = 'E' as u8;
/// the third byte of the ELF magic number
pub const ELF_MAGIC3: u8 = 'L' as u8;
/// the fourth byte of the ELF magic number
pub const ELF_MAGIC4: u8 = 'F' as u8;

/// the length of the e_ident bytes-field.
pub const EI_NIDENT: usize = 16;

/// the ELFCLASS's index in the elf identification.
pub const EI_CLASS: usize = 4;

/// the ELFDATA's index in the elf identification.
pub const EI_DATA: usize = 5;

/// the ELFOSABI's index in the elf identification.
pub const EI_OSABI: usize = 7;

/// the ELFOSABIVERSION's index in the elf identification.
pub const EI_OSABI_VERSION: usize = 8;
