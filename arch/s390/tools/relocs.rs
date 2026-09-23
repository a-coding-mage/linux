// SPDX-License-Identifier: GPL-2.0
//! Extract sorted absolute s390 kernel relocations into assembler input.

#[path = "../../../scripts/elf-parse.rs"]
mod elf;

use elf::{ElfFile, Section, Symbol};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Vec<u8>>;

fn errno(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .into()
}

fn checked<T>(result: std::result::Result<T, String>) -> Result<T> {
    result.map_err(|message| format!("Invalid ELF: {message}\n").into_bytes())
}

// All callers use a header or entry whose complete extent was checked first.
fn number(bytes: &[u8], offset: usize, width: usize) -> u64 {
    bytes[offset..offset + width]
        .iter()
        .fold(0, |n, &b| n << 8 | u64::from(b))
}

fn seek(offset: u64) -> Result<()> {
    if offset > i64::MAX as u64 {
        return Err(format!("Seek to {offset} failed: Invalid argument\n").into_bytes());
    }
    Ok(())
}

fn extent(data: &[u8], offset: u64, size: u64, message: &str) -> Result<()> {
    seek(offset)?;
    // C still seeks for a zero-length table, but fread then succeeds even
    // beyond EOF. Do not inspect unrelated section contents.
    if size != 0 && (offset > data.len() as u64 || size > data.len() as u64 - offset) {
        return Err(format!("{message}: Success\n").into_bytes());
    }
    Ok(())
}

fn read_elf(data: &[u8]) -> Result<ElfFile<'_>> {
    if data.len() < 64 {
        return Err(b"Cannot read ELF header: Success\n".to_vec());
    }
    for (bad, message) in [
        (&data[..4] != b"\x7fELF", "No ELF magic\n"),
        (data[4] != 2, "Not a 64 bit executable\n"),
        (data[5] != 2, "ELF endian mismatch\n"),
        (data[6] != 1, "Unknown ELF version\n"),
        (
            !matches!(number(data, 16, 2), 2 | 3),
            "Unsupported ELF header type\n",
        ),
        (number(data, 18, 2) != 22, "Not for IBM S/390\n"),
        (number(data, 20, 4) != 1, "Unknown ELF version\n"),
        (number(data, 52, 2) != 64, "Bad Elf header size\n"),
        (number(data, 54, 2) != 56, "Bad program header entry\n"),
        (number(data, 58, 2) != 64, "Bad section header entry\n"),
    ] {
        if bad {
            return Err(message.as_bytes().to_vec());
        }
    }
    let offset = number(data, 40, 8);
    let mut count = number(data, 60, 2);
    let mut strings = number(data, 62, 2);
    if count == 0 || strings == 0xffff {
        extent(data, offset, 64, "Cannot read initial ELF section header")?;
        let first = &data[offset as usize..offset as usize + 64];
        if count == 0 {
            count = number(first, 32, 8);
        }
        if strings == 0xffff {
            strings = number(first, 40, 4);
        }
    }
    if strings >= count {
        return Err(b"String table index out of bounds\n".to_vec());
    }
    seek(offset)?;
    let available = (data.len() as u64).saturating_sub(offset) / 64;
    if count > available {
        return Err(
            format!("Cannot read ELF section headers {available}/{count}: Success\n").into_bytes(),
        );
    }
    checked(ElfFile::parse(data, (1 << 2) | (1 << 3)))
}

fn symbol_name<'a>(
    elf: &ElfFile<'a>,
    sections: &[Section],
    table: &Section,
    index: usize,
    symbol: &Symbol,
) -> Result<&'a [u8]> {
    if symbol.name != 0 {
        let strings = checked(elf.section(table.link as usize))?;
        if strings.kind != 3 {
            return Err(b"Invalid ELF: symbol table does not link to a string table\n".to_vec());
        }
        return checked(elf.string(&strings, symbol.name));
    }
    let section = if symbol.section == 0xffff {
        checked(elf.symbol_section_index(table, index, symbol))?
    } else {
        u32::from(symbol.section)
    };
    if let Some(section) = sections.get(section as usize) {
        checked(elf.section_name(section))
    } else {
        Ok(match section {
            0xfff1 => b"ABSOLUTE",
            0xfff2 => b"COMMON",
            _ => b"<noname>",
        })
    }
}

fn relocations(data: &[u8]) -> Result<Vec<u32>> {
    let elf = read_elf(data)?;
    let sections = checked(elf.sections())?;
    // Match the original read order and errors, even for unreferenced tables.
    for kinds in [&[3][..], &[2, 18][..], &[4][..]] {
        for section in sections.iter().filter(|s| kinds.contains(&s.kind)) {
            extent(
                data,
                section.offset,
                section.size,
                if section.kind == 18 {
                    "Cannot read extended symbol table"
                } else {
                    "Cannot read symbol table"
                },
            )?;
        }
    }
    let mut symbols = vec![None; sections.len()];
    let mut result = Vec::new();
    for section in sections.iter().filter(|s| s.kind == 4) {
        let applies = checked(elf.section(section.info as usize))?;
        if applies.flags & 2 == 0 || section.size < 24 {
            continue;
        }
        let mut table = checked(elf.section(section.link as usize))?;
        if table.kind != 2 {
            return Err(b"Invalid ELF: relocation does not link to a symbol table\n".to_vec());
        }
        if symbols[table.index].is_none() {
            // The C reader uses sizeof(Elf64_Sym/Rela), not sh_entsize,
            // and ignores incomplete trailing entries after reading them.
            table.entry_size = 24;
            table.size -= table.size % 24;
            symbols[table.index] = Some(checked(elf.symbols(&table))?);
        }
        let table_symbols = symbols[table.index].as_ref().expect("loaded symbol table");
        let mut reloc_table = *section;
        reloc_table.entry_size = 24;
        reloc_table.size -= reloc_table.size % 24;
        for relocation in checked(elf.relocations(&reloc_table))? {
            let index = relocation.symbol as usize;
            let symbol = table_symbols
                .get(index)
                .ok_or_else(|| b"Invalid ELF: relocation symbol index out of bounds\n".to_vec())?;
            // C resolves names before classifying even ignored relocations.
            let name = symbol_name(&elf, &sections, &table, index, symbol)?;
            match relocation.kind {
                0 | 5 | 17 | 19 | 20 | 21 | 23 | 26 | 28 => {}
                4 if symbol.section == 0xfff1 => {
                    if !name.starts_with(b"__kcfi_typeid_") {
                        let mut error = b"Invalid absolute R_390_32 relocation: ".to_vec();
                        error.extend_from_slice(name);
                        error.push(b'\n');
                        return Err(error);
                    }
                }
                22 => result.push(relocation.offset as u32),
                kind => {
                    return Err(
                        format!("Unsupported relocation type: {}\n", kind as i32).into_bytes()
                    )
                }
            }
        }
    }
    result.sort_unstable();
    Ok(result)
}

fn path_error(prefix: &[u8], path: &Path, error: &str, newline: bool) -> Vec<u8> {
    let mut message = prefix.to_vec();
    message.extend_from_slice(path.as_os_str().as_encoded_bytes());
    message.extend_from_slice(b": ");
    message.extend_from_slice(error.as_bytes());
    if newline {
        message.push(b'\n');
    }
    message
}

fn run() -> Result<()> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() != 2 {
        return Err(b"relocs vmlinux\n".to_vec());
    }
    let path = Path::new(&arguments[1]);
    let mut file = File::open(path)
        .map_err(|error| path_error(b"Cannot open ", path, &errno(&error), true))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|error| path_error(b"Cannot read ", path, &errno(&error), false))?;
    if data.len() < 16 {
        return Err(path_error(b"Cannot read ", path, "Success", false));
    }
    let relocations = relocations(&data)?;
    let output = || -> io::Result<()> {
        let mut output = io::BufWriter::new(io::stdout().lock());
        writeln!(output, ".section \".vmlinux.relocs_64\",\"a\"")?;
        for offset in relocations {
            writeln!(output, "\t.long 0x{offset:08x}")?;
        }
        output.flush()
    };
    output().map_err(|error| format!("Cannot write relocations: {}\n", errno(&error)).into_bytes())
}

fn main() {
    if let Err(message) = run() {
        let _ = io::stderr().lock().write_all(&message);
        std::process::exit(1);
    }
}
