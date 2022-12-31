use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfClass {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "class32")]
    Class32,
    #[serde(rename = "class64")]
    Class64,
    Num(u8),
}

impl From<u8> for ElfClass {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Class32,
            2 => Self::Class64,
            _ => Self::Num(v),
        }
    }
}

impl Into<u8> for ElfClass {
    fn into(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Class32 => 1,
            Self::Class64 => 2,
            Self::Num(v) => v,
        }
    }
}

impl std::fmt::Display for ElfClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Num(v) => {
                return write!(f, "unknown({:x})", v);
            }

            Self::None => "none",
            Self::Class32 => "class32",
            Self::Class64 => "class64",
        };

        write!(f, "{}", s)
    }
}
