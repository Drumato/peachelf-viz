use crate::{header, section};

pub struct Elf64 {
    pub header: header::Header64,
    pub sections: section::SectionSet64,
}
