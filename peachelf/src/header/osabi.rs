use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElfOsAbi {
    SystemV,
    #[serde(rename = "HP-UX")]
    Hpux,
    #[serde(rename = "NetBSD")]
    NetBsd,
    /// the object file uses GNU ELF extensions.
    #[serde(rename = "GNU")]
    Gnu,
    Num(u8),
}

impl From<u8> for ElfOsAbi {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::SystemV,
            1 => Self::Hpux,
            2 => Self::NetBsd,
            3 => Self::Gnu,
            _ => Self::Num(v),
        }
    }
}

impl Into<u8> for ElfOsAbi {
    fn into(self) -> u8 {
        match self {
            Self::SystemV => 0,
            Self::Hpux => 1,
            Self::NetBsd => 2,
            Self::Gnu => 3,
            Self::Num(v) => v,
        }
    }
}

impl std::fmt::Display for ElfOsAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Num(v) => {
                return write!(f, "UNKNOWN({:x})", v);
            }

            Self::SystemV => "SystemV",
            Self::Hpux => "HP-UX",
            Self::NetBsd => "NetBSD",
            Self::Gnu => "GNU",
        };

        write!(f, "{}", s)
    }
}
