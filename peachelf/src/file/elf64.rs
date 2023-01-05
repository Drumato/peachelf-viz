use crate::{header, program_header, section};

pub struct Elf64 {
    pub header: header::Header64,
    pub sections: section::SectionSet64,
    pub program_headers: program_header::ProgramHeaders64,
}
