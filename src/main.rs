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
    show_header: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut f = std::fs::File::open(&args.filepath)?;
    let mut elf = Vec::with_capacity(4096);
    let _ = f.read_to_end(&mut elf)?;

    let elf64 = peachelf::parser::parse_elf64(&elf)?;
    let mut elf64 = Elf64Display::from(elf64);
    if !args.show_header {
        elf64.header = None;
    }

    println!("{}", serde_json::to_string(&elf64)?);

    Ok(())
}

#[derive(Serialize)]
pub struct Elf64Display {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<peachelf::header::Header64>,
}

impl From<peachelf::file::Elf64> for Elf64Display {
    fn from(f: peachelf::file::Elf64) -> Self {
        Self {
            header: Some(f.header),
        }
    }
}
