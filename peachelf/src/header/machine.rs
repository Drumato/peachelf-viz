use crate::types;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfMachine {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "aarch64")]
    Aarch64,
    #[serde(rename = "amd64")]
    X8664,
    Unknown(types::Elf64Half),
}

impl From<types::Elf64Half> for ElfMachine {
    fn from(v: u16) -> Self {
        match v {
            0 => Self::None,
            62 => Self::X8664,
            183 => Self::Aarch64,
            _ => Self::Unknown(v),
        }
    }
}

impl Into<u16> for ElfMachine {
    fn into(self) -> u16 {
        match self {
            Self::None => 0,
            Self::X8664 => 62,
            Self::Aarch64 => 183,
            Self::Unknown(v) => v,
        }
    }
}

impl std::fmt::Display for ElfMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown(v) => {
                return write!(f, "UNKNOWN({:x})", v);
            }

            Self::None => "NONE",
            Self::X8664 => "Advanced Micro Devices X86-64",
            Self::Aarch64 => "AArch64",
        };

        write!(f, "{}", s)
    }
}
