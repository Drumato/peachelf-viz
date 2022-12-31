use crate::{file, header, types};
const ELF_MAGIC1: u8 = 0x7f;
const ELF_MAGIC2: u8 = 'E' as u8;
const ELF_MAGIC3: u8 = 'L' as u8;
const ELF_MAGIC4: u8 = 'F' as u8;

use byteorder::ReadBytesExt;
use std::io::Cursor;

pub fn parse_elf64(b: &[u8]) -> anyhow::Result<file::Elf64> {
    let mut cursor = std::io::Cursor::new(b);

    let elf64_header = parse_elf64_header(&mut cursor)?;
    Ok(file::Elf64 {
        header: elf64_header,
    })
}

fn parse_elf64_header(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<header::Header64> {
    parse_raw_elf64_header(cursor).map(|h| h.into())
}

fn parse_raw_elf64_header(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<header::RawHeader64> {
    let _ = parse_elf_magic(cursor)?;

    let mut hdr = header::RawHeader64::default();

    hdr.e_ident = parse_rest_elf_identification(cursor)?;
    hdr.e_type = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_machine = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_version = parse_elf64_word(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_entry = parse_elf64_addr(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_phoff = parse_elf64_offset(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_shoff = parse_elf64_offset(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_flags = parse_elf64_word(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_ehsize = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_phentsize = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_phnum = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_shentsize = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_shnum = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;
    hdr.e_shstrndx = parse_elf64_half(cursor, hdr.e_ident[header::EI_DATA])?;

    Ok(hdr)
}

fn parse_elf_magic(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
    let magic1 = cursor.read_u8()?;
    let magic2 = cursor.read_u8()?;
    let magic3 = cursor.read_u8()?;
    let magic4 = cursor.read_u8()?;

    if magic1 != ELF_MAGIC1 || magic2 != ELF_MAGIC2 || magic3 != ELF_MAGIC3 || magic4 != ELF_MAGIC4
    {
        return Err(anyhow::anyhow!(
            "the elf magic-number must be [0x7f, 'E', 'L', 'F'], but got invalid value"
        ));
    }

    Ok(())
}

fn parse_rest_elf_identification(
    cursor: &mut Cursor<&[u8]>,
) -> anyhow::Result<[u8; header::EI_NIDENT]> {
    const ELF_IDENTIFICATION_PADDING_LENGTH: usize = 7;

    let elf_class = cursor.read_u8()?;
    let elf_data = cursor.read_u8()?;
    let elf_version = cursor.read_u8()?;
    if elf_version != types::EV_CURRENT {
        return Err(anyhow::anyhow!(
            "the version field in e_identification must be EV_CURRENT(1)"
        ));
    }

    let elf_osabi = cursor.read_u8()?;
    let elf_osabi_version = cursor.read_u8()?;

    for _ in 0..ELF_IDENTIFICATION_PADDING_LENGTH {
        let _ = cursor.read_u8()?;
    }

    Ok([
        ELF_MAGIC1,
        ELF_MAGIC2,
        ELF_MAGIC3,
        ELF_MAGIC4,
        elf_class,
        elf_data,
        elf_version,
        elf_osabi,
        elf_osabi_version,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ])
}

fn parse_elf64_half(cursor: &mut Cursor<&[u8]>, elf_data: u8) -> anyhow::Result<types::Elf64Half> {
    if elf_data == header::ELFDATA_LSB {
        cursor
            .read_u16::<byteorder::LittleEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    } else {
        cursor
            .read_u16::<byteorder::BigEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

fn parse_elf64_word(cursor: &mut Cursor<&[u8]>, elf_data: u8) -> anyhow::Result<types::Elf64Word> {
    if elf_data == header::ELFDATA_LSB {
        cursor
            .read_u32::<byteorder::LittleEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    } else {
        cursor
            .read_u32::<byteorder::BigEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

fn parse_elf64_addr(cursor: &mut Cursor<&[u8]>, elf_data: u8) -> anyhow::Result<types::Elf64Addr> {
    if elf_data == header::ELFDATA_LSB {
        cursor
            .read_u64::<byteorder::LittleEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    } else {
        cursor
            .read_u64::<byteorder::BigEndian>()
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

fn parse_elf64_offset(cursor: &mut Cursor<&[u8]>, elf_data: u8) -> anyhow::Result<types::Elf64Off> {
    parse_elf64_addr(cursor, elf_data)
}
