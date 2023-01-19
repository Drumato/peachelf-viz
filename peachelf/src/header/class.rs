use serde::{Deserialize, Serialize};

/// file class or capacity
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfClass {
    /// invalid class
    #[serde(rename = "none")]
    None,
    /// 32-bit objects
    #[serde(rename = "class32")]
    Class32,
    /// 64-bit objects
    #[serde(rename = "class64")]
    Class64,
    /// any value
    Unknown { value: u8 },
}

impl From<u8> for ElfClass {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Class32,
            2 => Self::Class64,
            _ => Self::Unknown { value: v },
        }
    }
}

impl Into<u8> for ElfClass {
    fn into(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Class32 => 1,
            Self::Class64 => 2,
            Self::Unknown { value: v } => v,
        }
    }
}

impl std::fmt::Display for ElfClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown { value } => {
                return write!(f, "unknown({:x})", value);
            }

            Self::None => "none",
            Self::Class32 => "class32",
            Self::Class64 => "class64",
        };

        write!(f, "{}", s)
    }
}
