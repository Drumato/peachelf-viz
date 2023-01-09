use serde::{Deserialize, Serialize};

use crate::types;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(tag = "type")]
pub enum ProgramHeaderType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    ShLib,
    Phdr,
    Tls,
    Num,
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
            _ => Self::Unknown { value: v },
        }
    }
}
