use crate::types;

/// A segment flag that means the segment is executable
pub const PF_X: types::Elf64Word = 0b0000_0001;
/// A segment flag that means the segment is writable
pub const PF_W: types::Elf64Word = 0b0000_0010;
/// A segment flag that means the segment is readable
pub const PF_R: types::Elf64Word = 0b0000_0100;
