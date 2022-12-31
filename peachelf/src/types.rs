// 32bit Unsigned program address.
pub type Elf32Addr = u32;
// 64bit Unsigned program address.
pub type Elf64Addr = u64;
// 32bit Unsigned medium integer.
pub type Elf32Half = u16;
// 64bit Unsigned medium integer.
pub type Elf64Half = u16;
// 32bit Unsigned file offset.
pub type Elf32Off = u32;
// 64bit Unsigned file offset.
pub type Elf64Off = u64;
// 64bit Unsigned large integer.
pub type Elf32Word = u32;
// 64bit Signed large integer.
pub type Elf32SWord = i32;
// 64bit Unsigned large integer.
pub type Elf64Word = u32;
// 64bit Signed large integer.
pub type Elf64SWord = i32;
// 64bit Unsigned large integer.
pub type Elf64Xword = u64;

/// Current Version
pub const EV_CURRENT: u8 = 1;
