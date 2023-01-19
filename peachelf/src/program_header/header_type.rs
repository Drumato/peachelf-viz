use serde::{Deserialize, Serialize};

use crate::types;

///
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(tag = "type")]
pub enum ProgramHeaderType {
    /// unused program header table entry
    Null,
    /// loadble program segment
    Load,
    /// dynamic linking information
    Dynamic,
    /// program interpreter
    Interp,
    /// auxiliary information
    Note,
    /// reserved
    ShLib,
    /// entry for header table itself
    Phdr,
    /// thread-local storage segment
    Tls,
    /// number of defined types
    Num,
    /// GCC .eh_frame_hdr segment
    GnuEhFrame,
    /// indicates stack executability
    GnuStack,
    /// read-only after relocation
    GnuRelRO,
    /// GNU property
    GnuProperty,
    /// any value
    Unknown { value: types::Elf64Word },
}

impl From<types::Elf64Word> for ProgramHeaderType {
    fn from(v: types::Elf64Word) -> Self {
        match v {
            0 => Self::Null,
            1 => Self::Load,
            2 => Self::Dynamic,
            3 => Self::Interp,
            4 => Self::Note,
            5 => Self::ShLib,
            6 => Self::Phdr,
            7 => Self::Tls,
            8 => Self::Num,
            0x6474e550 => Self::GnuEhFrame,
            0x6474e551 => Self::GnuStack,
            0x6474e552 => Self::GnuRelRO,
            0x6474e553 => Self::GnuProperty,
            _ => Self::Unknown { value: v },
        }
    }
}
