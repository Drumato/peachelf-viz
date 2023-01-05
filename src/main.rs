use std::io::Read;

use clap::Parser;
use serde::Serialize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long, default_value = "json")]
    output_mode: String,

    #[arg(long)]
    disable_header: bool,
    #[arg(long)]
    disable_sections: bool,
    #[arg(long)]
    disable_program_headers: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut f = std::fs::File::open(&args.filepath)?;
    let mut elf = Vec::with_capacity(4096);
    let _ = f.read_to_end(&mut elf)?;

    let elf64 = peachelf::parser::parse_elf64(&elf)?;
    let mut elf64 = Elf64Display::from(elf64);
    if args.disable_header {
        elf64.header = None;
    }
    if args.disable_sections {
        elf64.sections = None;
    }
    if args.disable_program_headers {
        elf64.program_headers = None;
    }

    println!("{}", serde_json::to_string(&elf64)?);

    Ok(())
}

#[derive(Serialize)]
pub struct Elf64Display {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<peachelf::header::Header64>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<peachelf::section::SectionSet64>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_headers: Option<peachelf::program_header::ProgramHeaders64>,
}

impl From<peachelf::file::Elf64> for Elf64Display {
    fn from(f: peachelf::file::Elf64) -> Self {
        Self {
            header: Some(f.header),
            sections: Some(f.sections),
            program_headers: Some(f.program_headers),
        }
    }
}
