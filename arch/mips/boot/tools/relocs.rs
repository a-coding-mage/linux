// SPDX-License-Identifier: GPL-2.0
//! Checked MIPS kernel relocation collection and in-place table generation.

#[path = "../../../../scripts/elf-parse.rs"]
mod elf;
mod relocs_32;
mod relocs_64;
mod relocs_header;
mod relocs_main;

use elf::{ElfFile, Relocation, Section, Symbol};
use relocs_header::{relocation_name, Edit, Format, Options};
use relocs_main::output;
use std::io::Write;

fn checked<T>(value: Result<T, String>) -> Result<T, Vec<u8>> {
    value.map_err(|error| format!("Invalid ELF data: {error}\n").into_bytes())
}

fn header<'a>(data: &'a [u8], format: &Format) -> Result<ElfFile<'a>, Vec<u8>> {
    if data.len() < format.header_size {
        return Err(b"Cannot read ELF header: Success\n".to_vec());
    }
    if &data[..4] != b"\x7fELF" {
        return Err(b"No ELF magic\n".to_vec());
    }
    if data[4] != format.class {
        return Err(format!("Not a {} bit executable\n", format.bits).into_bytes());
    }
    let little = match data[5] {
        1 => true,
        2 => false,
        _ => return Err(b"Unknown ELF Endianness\n".to_vec()),
    };
    if data[6] != 1 {
        return Err(b"Unknown ELF version\n".to_vec());
    }
    let read = |offset: usize, size: usize| {
        let bytes = &data[offset..offset + size];
        if little {
            bytes
                .iter()
                .rev()
                .fold(0u64, |n, byte| n << 8 | u64::from(*byte))
        } else {
            bytes.iter().fold(0u64, |n, byte| n << 8 | u64::from(*byte))
        }
    };
    if !matches!(read(16, 2), 2 | 3) {
        return Err(b"Unsupported ELF header type\n".to_vec());
    }
    if read(18, 2) != 8 {
        return Err(format!("Not for {}\n", format.machine_name).into_bytes());
    }
    if read(20, 4) != 1 {
        return Err(b"Unknown ELF version\n".to_vec());
    }
    let sizes = if format.bits == 64 { 52 } else { 40 };
    if read(sizes, 2) != format.header_size as u64 {
        return Err(b"Bad ELF header size\n".to_vec());
    }
    if read(sizes + 2, 2) != format.program_size {
        return Err(b"Bad program header entry\n".to_vec());
    }
    if read(sizes + 6, 2) != format.section_size {
        return Err(b"Bad section header entry\n".to_vec());
    }
    let count = read(sizes + 8, 2);
    let strings = read(sizes + 10, 2);
    if count != 0 && strings != 0xffff && strings >= count {
        return Err(b"String table index out of bounds\n".to_vec());
    }
    checked(ElfFile::parse(data, (1 << 2) | (1 << 3)))
}

struct Tables {
    symbols: Vec<Option<Vec<Symbol>>>,
    relocations: Vec<Option<Vec<Relocation>>>,
}

fn tables(image: &ElfFile<'_>, sections: &[Section], format: &Format) -> Result<Tables, Vec<u8>> {
    let mut symbols = vec![None; sections.len()];
    let mut relocations = vec![None; sections.len()];
    // The C reader visits all tables before processing individual records.
    for section in sections {
        if section.kind == 3 {
            checked(image.section_data(section))?;
        }
    }
    for section in sections {
        if section.kind == 2 {
            // The original uses sizeof(Elf_Sym), not sh_entsize.
            checked(image.section_data(section))?;
            let mut table = *section;
            table.entry_size = format.symbol_size;
            table.size -= table.size % format.symbol_size;
            symbols[section.index] = Some(checked(image.symbols(&table))?);
        }
    }
    for section in sections {
        if section.kind == format.relocation_kind {
            checked(image.section_data(section))?;
            let mut table = *section;
            table.entry_size = format.relocation_size;
            table.size -= table.size % format.relocation_size;
            relocations[section.index] = Some(checked(image.relocations(&table))?);
        }
    }
    Ok(Tables {
        symbols,
        relocations,
    })
}

fn section_name<'a>(
    image: &ElfFile<'a>,
    sections: &[Section],
    index: usize,
) -> Result<&'a [u8], Vec<u8>> {
    if let Some(section) = sections.get(index) {
        return checked(image.section_name(section));
    }
    Ok(match index {
        0xfff1 => b"ABSOLUTE",
        0xfff2 => b"COMMON",
        _ => b"<noname>",
    })
}

fn padded(out: &mut impl Write, bytes: &[u8], width: usize) -> Result<(), Vec<u8>> {
    for _ in bytes.len()..width {
        output(out.write_all(b" "))?;
    }
    output(out.write_all(bytes))
}

fn info_row(out: &mut impl Write, fields: [&[u8]; 5]) -> Result<(), Vec<u8>> {
    for (index, (field, width)) in fields.into_iter().zip([16, 10, 16, 40, 16]).enumerate() {
        if index != 0 {
            output(out.write_all(b"  "))?;
        }
        padded(out, field, width)?;
    }
    output(out.write_all(b"\n"))
}

fn walk(
    image: &ElfFile<'_>,
    sections: &[Section],
    tables: &Tables,
    format: &Format,
    base: u64,
    options: &Options,
    out: &mut impl Write,
) -> Result<Vec<u32>, Vec<u8>> {
    let extab = checked(image.find_section(b"__ex_table"))?.map(|section| section.index);
    let mut encoded = Vec::new();
    for section in sections {
        let Some(relocations) = &tables.relocations[section.index] else {
            continue;
        };
        if Some(section.info as usize) == extab {
            continue;
        }
        let applies = checked(image.section(section.info as usize))?;
        if applies.flags & 2 == 0 {
            continue;
        }
        let symbols = tables
            .symbols
            .get(section.link as usize)
            .and_then(Option::as_ref)
            .ok_or_else(|| b"Invalid ELF relocation symbol table\n".to_vec())?;
        let table = checked(image.section(section.link as usize))?;
        let strings = checked(image.section(table.link as usize))?;
        for relocation in relocations {
            let symbol = symbols
                .get(relocation.symbol as usize)
                .ok_or_else(|| b"Invalid ELF relocation symbol index\n".to_vec())?;
            let name = if symbol.name == 0 {
                section_name(image, sections, symbol.section as usize)?
            } else {
                checked(image.string(&strings, symbol.name))?
            };
            let relative = relocation.offset.wrapping_sub(base);
            let relative = if format.bits == 32 {
                u64::from(relative as u32)
            } else {
                relative
            };
            if options.info {
                info_row(
                    out,
                    [
                        checked(image.section_name(&applies))?,
                        format!("0x{:08x}", relative as u32).as_bytes(),
                        relocation_name(relocation.kind).as_bytes(),
                        name,
                        section_name(image, sections, symbol.section as usize)?,
                    ],
                )?;
                continue;
            }
            if (symbol.binding == 2 && symbol.value == 0) || name.starts_with(b"__crc_") {
                continue;
            }
            match relocation.kind {
                // PC-relative, unused and high/low parts unchanged by a
                // 64-KiB relocation inside the same 4-GiB segment.
                0 | 3 | 6 | 10 | 28 | 29 | 60 | 61 | 248 => {}
                2 | 4 | 5 | 18 => {
                    // add_reloc's original uint32_t argument truncates before
                    // encoding. Preserve its modulo-32-bit offset semantics.
                    let offset = (relative as u32) >> 2;
                    if offset > 0x00ff_ffff {
                        return Err(b"Kernel image exceeds maximum size for relocation!\n".to_vec());
                    }
                    encoded.push(offset | relocation.kind << 24);
                }
                kind => {
                    return Err(format!(
                        "Unsupported relocation type: {} ({kind})\n",
                        relocation_name(kind)
                    )
                    .into_bytes())
                }
            }
        }
    }
    Ok(encoded)
}

fn validate_edits(data: &[u8], edits: &[Edit]) -> Result<(), Vec<u8>> {
    let mut ranges = Vec::new();
    for edit in edits {
        let start: usize = edit
            .offset
            .try_into()
            .map_err(|_| b"ELF edit offset overflow\n".to_vec())?;
        let end = start
            .checked_add(edit.bytes.len())
            .ok_or_else(|| b"ELF edit size overflow\n".to_vec())?;
        if data.get(start..end).is_none() {
            return Err(b"ELF edit exceeds file bounds\n".to_vec());
        }
        if start != end {
            ranges.push((start, end));
        }
    }
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return Err(b"Overlapping ELF relocation edits\n".to_vec());
    }
    Ok(())
}

fn process(data: &[u8], options: &Options, out: &mut impl Write) -> Result<Vec<Edit>, Vec<u8>> {
    let format = if data.get(4) == Some(&2) {
        &relocs_64::FORMAT
    } else {
        &relocs_32::FORMAT
    };
    let image = header(data, format)?;
    let sections = checked(image.sections())?;
    let tables = tables(&image, &sections, format)?;
    let text = checked(image.find_section(b".text"))?
        .ok_or_else(|| b"Could not find .text section\n".to_vec())?;
    if options.info {
        info_row(
            out,
            [
                b"reloc section",
                b"offset",
                b"reloc type",
                b"symbol",
                b"symbol section",
            ],
        )?;
        walk(
            &image,
            &sections,
            &tables,
            format,
            text.address,
            options,
            out,
        )?;
        return Ok(Vec::new());
    }
    let reserved = checked(image.find_section(b".data.reloc"))?
        .ok_or_else(|| b"Could not find relocation section\n".to_vec())?;
    let mut encoded = walk(
        &image,
        &sections,
        &tables,
        format,
        text.address,
        options,
        out,
    )?;
    if !encoded.is_empty() {
        encoded.push(0);
    }
    let mut bytes = Vec::with_capacity(encoded.len() * 4);
    for value in &encoded {
        bytes.extend_from_slice(&if image.little_endian() {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        });
    }
    if options.text {
        output(out.write_all(b".section \".data.reloc\",\"a\"\n.balign 4\n"))?;
        for value in &encoded {
            output(writeln!(out, "\t.long 0x{value:08x}"))?;
        }
    } else if options.binary {
        output(out.write_all(&bytes))?;
    }
    if bytes.len() as u64 > reserved.size {
        let suggested = (bytes.len() as u64 + 0x1000) & !0xfff;
        return Err(format!("Relocations overflow available space!\nPlease adjust CONFIG_RELOCATION_TABLE_SIZE to at least 0x{suggested:08x}\n").into_bytes());
    }
    let mut edits = Vec::new();
    if !options.text && !options.binary && !bytes.is_empty() {
        edits.push(Edit {
            offset: reserved.offset,
            bytes,
        });
    }
    if !options.keep {
        let width = image.word_size();
        let table = checked(image.read_integer(if width == 8 { 40 } else { 32 }, width))?;
        for section in &sections {
            if section.kind == format.relocation_kind {
                edits.push(Edit {
                    offset: table
                        + section.index as u64 * format.section_size
                        + if width == 8 { 32 } else { 20 },
                    bytes: vec![0; width],
                });
            }
        }
    }
    validate_edits(data, &edits)?;
    Ok(edits)
}

fn main() {
    relocs_main::main();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
