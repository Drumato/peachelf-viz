use crate::types;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "rel")]
    Rel,
    #[serde(rename = "exec")]
    Exec,
    #[serde(rename = "dyn")]
    Dyn,
    #[serde(rename = "core")]
    Core,
    Num(types::Elf64Half),
}

impl From<types::Elf64Half> for ElfType {
    fn from(v: u16) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            4 => Self::Core,
            _ => Self::Num(v),
        }
    }
}

impl std::fmt::Display for ElfType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Num(v) => {
                return write!(f, "unknown({:x})", v);
            }

            Self::None => "none",
            Self::Rel => "rel",
            Self::Exec => "exec",
            Self::Dyn => "dyn",
            Self::Core => "core",
        };

        write!(f, "{}", s)
    }
}
