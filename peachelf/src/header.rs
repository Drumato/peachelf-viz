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
