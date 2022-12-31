use serde::{Deserialize, Serialize};

pub const ELFDATA_LSB: u8 = 1;
pub const ELFDATA_MSB: u8 = 2;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfData {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "2lsb")]
    LSB,
    #[serde(rename = "2msb")]
    MSB,
    Num(u8),
}

impl From<u8> for ElfData {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::None,
            ELFDATA_LSB => Self::LSB,
            ELFDATA_MSB => Self::MSB,
            _ => Self::Num(v),
        }
    }
}

impl Into<u8> for ElfData {
    fn into(self) -> u8 {
        match self {
            Self::None => 0,
            Self::LSB => ELFDATA_LSB,
            Self::MSB => ELFDATA_MSB,
            Self::Num(v) => v,
        }
    }
}

impl std::fmt::Display for ElfData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Num(v) => {
                return write!(f, "unknown({:x})", v);
            }

            Self::None => "none",
            Self::LSB => "2lsb",
            Self::MSB => "2msb",
        };

        write!(f, "{}", s)
    }
}
