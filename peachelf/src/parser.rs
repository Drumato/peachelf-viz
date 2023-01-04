use crate::{file, header, section, types};
const ELF_MAGIC1: u8 = 0x7f;
const ELF_MAGIC2: u8 = 'E' as u8;
const ELF_MAGIC3: u8 = 'L' as u8;
const ELF_MAGIC4: u8 = 'F' as u8;

use byteorder::ReadBytesExt;
use std::io::Cursor;

pub fn parse_elf64(file_bytes: &[u8]) -> anyhow::Result<file::Elf64> {
    let elf64_header = {
        let mut cursor = std::io::Cursor::new(file_bytes);

        parse_elf64_header(&mut cursor)?
    };

    let elf64_section_headers = parse_elf64_section_headers(file_bytes, &elf64_header)?;
    let elf64_sections = parse_elf64_sections(file_bytes, elf64_section_headers, &elf64_header)?;

    Ok(file::Elf64 {
        header: elf64_header,
        sections: elf64_sections,
    })
}

fn parse_elf64_sections(
    file_bytes: &[u8],
    section_headers: Vec<section::SectionHeader64>,
    elf64_header: &header::Header64,
) -> anyhow::Result<section::SectionSet64> {
    // first, we should collect the section data as raw bytes.
    // because the sh_name field requires to index the shstrtab.
    let mut section_datas: Vec<Vec<u8>> = Vec::with_capacity(section_headers.len());

    for hdr in section_headers.iter() {
        let start_offset = hdr.offset as usize;
        let end_offset = hdr.offset as usize + hdr.size as usize;
        section_datas.push(file_bytes[start_offset..end_offset].to_vec());
    }

    let section_name_table = section_datas[elf64_header.shstrndx as usize].clone();

    let sections = section_headers
        .into_iter()
        .zip(section_datas.into_iter())
        .enumerate()
        .map(|(idx, (hdr, data))| {
            // ignore the null section
            if hdr.name == 0x00 {
                return section::Section64 {
                    name: String::new(),
                    data: section::SectionData64::Null,
                };
            }

            let name = get_elf_strtab_entry(&section_name_table, hdr.name as usize);

            let result = parse_elf64_section_data(&hdr, data);
            if result.is_err() {
                eprintln!("failed to parse the sections[{}] data", idx);
            }

            section::Section64 {
                name,
                data: result.unwrap_or(section::SectionData64::Null),
            }
        })
        .collect();

    Ok(section::SectionSet64 { sections })
}

fn parse_elf64_section_headers(
    file_bytes: &[u8],
    elf64_header: &header::Header64,
) -> anyhow::Result<Vec<section::SectionHeader64>> {
    parse_raw_elf64_section_headers(file_bytes, elf64_header).map(|sct_header| {
        sct_header
            .into_iter()
            .map(|raw_header| section::SectionHeader64::from(raw_header))
            .collect()
    })
}

fn parse_raw_elf64_section_headers(
    file_bytes: &[u8],
    elf64_header: &header::Header64,
) -> anyhow::Result<Vec<section::RawSectionHeader64>> {
    let mut sct_headers = Vec::with_capacity(elf64_header.shnum as usize);

    for sct_header_idx in 0..elf64_header.shnum as usize {
        let start_offset =
            elf64_header.shoff as usize + (section::RawSectionHeader64::SIZE * sct_header_idx);

        let mut cursor = std::io::Cursor::new(&file_bytes[start_offset..]);
        let sct_header = parse_raw_elf64_section_header(&mut cursor, elf64_header.data.into())?;
        sct_headers.push(sct_header);
    }

    Ok(sct_headers)
}

fn parse_raw_elf64_section_header(
    cursor: &mut Cursor<&[u8]>,
    elf_data: u8,
) -> anyhow::Result<section::RawSectionHeader64> {
    let mut hdr = section::RawSectionHeader64::default();

    hdr.sh_name = parse_elf64_word(cursor, elf_data)?;
    hdr.sh_type = parse_elf64_word(cursor, elf_data)?;
    hdr.sh_flags = parse_elf64_xword(cursor, elf_data)?;
    hdr.sh_addr = parse_elf64_addr(cursor, elf_data)?;
    hdr.sh_offset = parse_elf64_offset(cursor, elf_data)?;
    hdr.sh_size = parse_elf64_xword(cursor, elf_data)?;
    hdr.sh_link = parse_elf64_word(cursor, elf_data)?;
    hdr.sh_info = parse_elf64_word(cursor, elf_data)?;
    hdr.sh_addralign = parse_elf64_xword(cursor, elf_data)?;
    hdr.sh_entsize = parse_elf64_xword(cursor, elf_data)?;

    Ok(hdr)
}

fn parse_elf64_section_data(
    section_header: &section::SectionHeader64,
    data_bytes: Vec<u8>,
) -> anyhow::Result<section::SectionData64> {
    match &section_header.section_type {
        section::SectionType::Null => Ok(section::SectionData64::Null),
        _ => Ok(section::SectionData64::Raw { bytes: data_bytes }),
    }
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

fn get_elf_strtab_entry(table: &[u8], entry_index: usize) -> String {
    let mut name_end = entry_index;

    // find the '\0' termination
    loop {
        if name_end >= table.len() || table[name_end] == 0x00 {
            break;
        }

        name_end += 1;
    }

    unsafe { String::from_utf8_unchecked(table[entry_index..name_end].to_vec()) }
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

fn parse_elf64_xword(
    cursor: &mut Cursor<&[u8]>,
    elf_data: u8,
) -> anyhow::Result<types::Elf64Xword> {
    parse_elf64_addr(cursor, elf_data)
}
