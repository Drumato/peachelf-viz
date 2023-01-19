use serde::{Deserialize, Serialize};

use crate::types;

/// section header type
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
pub enum SectionType {
    /// unused section header table entry
    Null,
    /// program data
    ProgBits,
    /// symbol table
    SymTab,
    /// string table
    StrTab,
    /// relocation entries with addends
    Rela,
    /// symbol hash table
    Hash,
    /// dynamic linking  information
    Dynamic,
    /// Notes
    Note,
    /// Program space with no data (bss)
    NoBits,
    /// Relocation entries without addends
    Rel,
    /// reserved
    ShLib,
    /// dynamic linker symbol table
    DynSym,
    /// array of constructors
    InitArray,
    /// array of destructors
    FiniArray,
    /// array of pre-constructors
    PreInitArray,
    /// section group
    Group,
    /// extended section indices
    SymTabShNdx,
    /// number of defined types
    Num,
    /// any value
    Unknown { value: types::Elf64Word },
}

impl From<types::Elf64Word> for SectionType {
    fn from(v: types::Elf64Word) -> Self {
        match v {
            0 => Self::Null,
            1 => Self::ProgBits,
            2 => Self::SymTab,
            3 => Self::StrTab,
            4 => Self::Rela,
            5 => Self::Hash,
            6 => Self::Dynamic,
            7 => Self::Note,
            8 => Self::NoBits,
            9 => Self::Rel,
            10 => Self::ShLib,
            11 => Self::DynSym,
            14 => Self::InitArray,
            15 => Self::FiniArray,
            16 => Self::PreInitArray,
            17 => Self::Group,
            18 => Self::SymTabShNdx,
            19 => Self::Num,
            _ => Self::Unknown { value: v },
        }
    }
}
