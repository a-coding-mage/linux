// SPDX-License-Identifier: GPL-2.0-only
//! Sort kernel exception, ORC unwind, and mcount tables in executable ELF files.
//!
//! All references are checked before edits are applied. The target's ELF class,
//! byte order, and architecture determine table layouts independently of the host.
// Copyright 2011-2012 Cavium, Inc.
// Copyright 1999-2019 Alibaba Group Holding Limited, Shile Zhang.

#[path = "elf-parse.rs"]
mod elf;

use elf::{ElfFile, Section, Symbol};
use std::env;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::process::ExitCode;

struct Patch {
    offset: u64,
    bytes: Vec<u8>,
}

struct Function {
    address: u64,
    size: u64,
}

fn read_integer(bytes: &[u8], little: bool) -> u64 {
    if little {
        bytes
            .iter()
            .rev()
            .fold(0, |value, &byte| (value << 8) | u64::from(byte))
    } else {
        bytes
            .iter()
            .fold(0, |value, &byte| (value << 8) | u64::from(byte))
    }
}

fn write_integer(bytes: &mut [u8], value: u64, little: bool) {
    let width = bytes.len();
    for (index, byte) in bytes.iter_mut().enumerate() {
        let shift = if little { index } else { width - index - 1 };
        *byte = (value >> (shift * 8)) as u8;
    }
}

fn section_location(section: &Section, address: u64, size: u64) -> Result<u64, String> {
    let relative = address
        .checked_sub(section.address)
        .ok_or("address precedes ELF section:")?;
    let end = relative.checked_add(size).ok_or("ELF address overflow:")?;
    if section.kind == 8 || end > section.size {
        return Err("address exceeds ELF section:".into());
    }
    section
        .offset
        .checked_add(relative)
        .ok_or_else(|| "ELF offset overflow:".into())
}

#[derive(Clone, Copy)]
enum ExceptionLayout {
    Absolute,
    Relative,
    RelativeWithData,
}

fn exception_layout(machine: u16) -> Result<ExceptionLayout, String> {
    match machine {
        183 | 243 | 3 | 258 | 22 | 62 => Ok(ExceptionLayout::RelativeWithData),
        15 | 20 | 21 => Ok(ExceptionLayout::Relative),
        93 | 195 | 40 | 189 | 8 | 94 => Ok(ExceptionLayout::Absolute),
        _ => Err(format!("unrecognized e_machine {machine}")),
    }
}

fn sort_exceptions(
    elf: &ElfFile<'_>,
    section: &Section,
    layout: ExceptionLayout,
) -> Result<Patch, String> {
    let width = match layout {
        ExceptionLayout::Absolute => elf.word_size() * 2,
        ExceptionLayout::Relative => 8,
        ExceptionLayout::RelativeWithData => 12,
    };
    let source = elf.section_data(section)?;
    if source.len() as u64 != section.size || source.len() % width != 0 {
        return Err("invalid __ex_table entry size:".into());
    }
    let little = elf.little_endian();
    let mut entries: Vec<Vec<u8>> = source.chunks_exact(width).map(<[u8]>::to_vec).collect();
    if matches!(layout, ExceptionLayout::Absolute) {
        entries.sort_by_key(|entry| read_integer(&entry[..elf.word_size()], little));
    } else {
        // Both relative addresses refer to their own field, not the entry start.
        for (index, entry) in entries.iter_mut().enumerate() {
            for field in [0, 4] {
                let address = read_integer(&entry[field..field + 4], little) as u32;
                let normalized = address.wrapping_add((index * width + field) as u32);
                write_integer(&mut entry[field..field + 4], u64::from(normalized), little);
            }
        }
        entries.sort_by_key(|entry| read_integer(&entry[..4], little) as i32);
        for (index, entry) in entries.iter_mut().enumerate() {
            for field in [0, 4] {
                let address = read_integer(&entry[field..field + 4], little) as u32;
                let relative = address.wrapping_sub((index * width + field) as u32);
                write_integer(&mut entry[field..field + 4], u64::from(relative), little);
            }
        }
    }
    Ok(Patch {
        offset: section.offset,
        bytes: entries.into_iter().flatten().collect(),
    })
}

fn sort_orc(elf: &ElfFile<'_>, machine: u16) -> Result<Vec<Patch>, String> {
    let ip = elf
        .find_section(b".orc_unwind_ip")?
        .ok_or("incomplete ORC unwind tables in file:")?;
    let table = elf
        .find_section(b".orc_unwind")?
        .ok_or("incomplete ORC unwind tables in file:")?;
    // LoongArch has an additional saved return-address offset and register.
    let (entry_size, type_byte, type_shift) = match machine {
        258 => (8, 7, if elf.little_endian() { 4 } else { 1 }),
        62 | 3 => (6, 5, if elf.little_endian() { 0 } else { 5 }),
        _ => return Err("unsupported ORC architecture in file:".into()),
    };
    let ips = elf.section_data(&ip)?;
    let orcs = elf.section_data(&table)?;
    if ips.len() as u64 != ip.size
        || orcs.len() as u64 != table.size
        || ips.len() % 4 != 0
        || orcs.len() % entry_size != 0
        || ips.len() / 4 != orcs.len() / entry_size
    {
        return Err("inconsistent ORC unwind table entries in file:".into());
    }
    let little = elf.little_endian();
    let mut indices: Vec<_> = (0..ips.len() / 4).collect();
    let address = |index: usize| {
        i64::from(read_integer(&ips[index * 4..index * 4 + 4], little) as i32) + index as i64 * 4
    };
    let undefined = |index: usize| (orcs[index * entry_size + type_byte] >> type_shift) & 7 == 0;
    indices.sort_by(|&left, &right| {
        address(left)
            .cmp(&address(right))
            .then_with(|| undefined(right).cmp(&undefined(left)))
            .then_with(|| {
                // The C mergesort places equal non-terminators in reverse input
                // order. Spell out that tie-break without an invalid comparator.
                if undefined(left) {
                    left.cmp(&right)
                } else {
                    right.cmp(&left)
                }
            })
    });
    let mut sorted_ips = Vec::with_capacity(ips.len());
    let mut sorted_orcs = Vec::with_capacity(orcs.len());
    for (index, &original) in indices.iter().enumerate() {
        let relative = (address(original) as u32).wrapping_sub(index as u32 * 4);
        let mut bytes = [0; 4];
        write_integer(&mut bytes, u64::from(relative), little);
        sorted_ips.extend_from_slice(&bytes);
        sorted_orcs.extend_from_slice(&orcs[original * entry_size..(original + 1) * entry_size]);
    }
    Ok(vec![
        Patch {
            offset: ip.offset,
            bytes: sorted_ips,
        },
        Patch {
            offset: table.offset,
            bytes: sorted_orcs,
        },
    ])
}

fn function_contains(functions: &[Function], key: u64, before: u64) -> bool {
    // Match the function-boundary binary search, including overlapping aliases.
    let mut low = 0;
    let mut high = functions.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let function = &functions[middle];
        if key.wrapping_add(before) < function.address {
            high = middle;
        } else if key >= function.address.wrapping_add(function.size) {
            low = middle + 1;
        } else {
            return true;
        }
    }
    false
}

fn sort_mcount(
    elf: &ElfFile<'_>,
    section: &Section,
    start: u64,
    stop: u64,
    machine: u16,
    functions: &[Function],
) -> Result<Vec<Patch>, String> {
    let size = stop
        .checked_sub(start)
        .ok_or("invalid mcount location range")?;
    let width = elf.word_size();
    if size % width as u64 != 0 {
        return Err("invalid mcount entry size".into());
    }
    let offset = section_location(section, start, size)?;
    // Validate the complete backing section, even when RELA holds the values.
    elf.section_data(section)?;
    let mut values = Vec::new();
    let mut relocations = Vec::new();
    if machine == 183 {
        for section in elf.sections()? {
            if section.kind != 4 {
                continue;
            }
            if section.entry_size < (width * 3) as u64 || section.size % section.entry_size != 0 {
                return Err("invalid ELF relocation entry size".into());
            }
            elf.section_data(&section)?;
            for index in 0..section.size / section.entry_size {
                let entry = section.offset + index * section.entry_size;
                let location = elf.read_integer(entry, width)?;
                if !(start..stop).contains(&location) {
                    continue;
                }
                if values.len() as u64 == size / width as u64 {
                    return Err("Too many relocations".into());
                }
                let kind = elf.read_integer(entry + width as u64, width)?;
                if kind != 0x403 {
                    return Err(format!("rela has type {kind:x} but expected 403\n"));
                }
                let addend = entry + (width * 2) as u64;
                values.push(elf.read_integer(addend, width)?);
                relocations.push(addend);
            }
        }
    }
    if relocations.is_empty() {
        for index in 0..size / width as u64 {
            values.push(elf.read_integer(offset + index * width as u64, width)?);
        }
    }
    if values.len() as u64 != size / width as u64 {
        return Err(format!(
            "Expected {} mcount elements but found {}\n",
            size / width as u64,
            values.len()
        ));
    }
    if !functions.is_empty() {
        let before = if matches!(machine, 183 | 243) { 8 } else { 0 };
        for value in &mut values {
            if !function_contains(functions, *value, before) {
                *value = 0;
            }
        }
    }
    values.sort_unstable();
    if relocations.is_empty() {
        let mut bytes =
            vec![0; usize::try_from(size).map_err(|_| "mcount table exceeds host address space")?];
        for (entry, value) in bytes.chunks_exact_mut(width).zip(values) {
            write_integer(entry, value, elf.little_endian());
        }
        Ok(vec![Patch { offset, bytes }])
    } else {
        Ok(relocations
            .into_iter()
            .zip(values)
            .map(|(offset, value)| {
                let mut bytes = vec![0; width];
                write_integer(&mut bytes, value, elf.little_endian());
                Patch { offset, bytes }
            })
            .collect())
    }
}

fn make_plan(data: &[u8], functions: &[Function]) -> Result<Vec<Patch>, String> {
    let elf = ElfFile::parse(data, (1 << 2) | (1 << 3))?;
    let machine = elf.machine()?;
    let layout = exception_layout(machine)?;
    let mut patches = if cfg!(UNWINDER_ORC_ENABLED) {
        sort_orc(&elf, machine)?
    } else {
        Vec::new()
    };
    let exceptions = elf
        .find_section(b"__ex_table")?
        .ok_or("no __ex_table in file:")?;
    let symtab = elf.find_section(b".symtab")?.ok_or("no .symtab in file:")?;
    let strtab = elf.find_section(b".strtab")?.ok_or("no .strtab in file:")?;
    let symbols = elf.symbols(&symtab)?;
    let mut flag: Option<(usize, Symbol)> = None;
    let mut start = 0;
    let mut stop = 0;
    for (index, &symbol) in symbols.iter().enumerate() {
        match elf.string(&strtab, symbol.name)? {
            b"main_extable_sort_needed" if symbol.kind == 1 => flag = Some((index, symbol)),
            b"__start_mcount_loc" => start = symbol.value,
            b"__stop_mcount_loc" => stop = symbol.value,
            _ => (),
        }
    }
    if cfg!(MCOUNT_SORT_ENABLED) {
        let section = elf.find_section(b".init.data")?;
        if start == 0 || stop == 0 || section.is_none() {
            let prefix = if start == 0 {
                "get start_mcount_loc error!"
            } else if stop == 0 {
                "get stop_mcount_loc error!"
            } else {
                ""
            };
            return Err(format!("{prefix}incomplete mcount's sort in file:"));
        }
        patches.extend(
            sort_mcount(
                &elf,
                &section.expect("checked init data"),
                start,
                stop,
                machine,
                functions,
            )
            .map_err(|error| format!("failed to sort mcount '{error}':"))?,
        );
    }
    patches.push(sort_exceptions(&elf, &exceptions, layout)?);
    // Include the final symbol table entry; the C pointer loop skipped it.
    let (index, symbol) = flag.ok_or("no main_extable_sort_needed symbol in file:")?;
    let section = elf.symbol_section(&symtab, index, &symbol)?;
    elf.section_data(&section)?;
    let offset = section_location(&section, symbol.value, 4)?;
    patches.push(Patch {
        offset,
        bytes: vec![0; 4],
    });
    for patch in &patches {
        let end = patch
            .offset
            .checked_add(patch.bytes.len() as u64)
            .ok_or("ELF edit offset overflow:")?;
        if end > data.len() as u64 {
            return Err("ELF edit exceeds file:".into());
        }
    }
    Ok(patches)
}

fn hex_number(bytes: &[u8]) -> u64 {
    let mut pos = 0;
    let negative = bytes.first() == Some(&b'-');
    if negative || bytes.first() == Some(&b'+') {
        pos += 1;
    }
    if bytes
        .get(pos..pos + 2)
        .is_some_and(|s| s == b"0x" || s == b"0X")
    {
        pos += 2;
    }
    let mut value = 0u64;
    let mut overflow = false;
    while let Some(&byte) = bytes.get(pos) {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => break,
        };
        match value
            .checked_mul(16)
            .and_then(|n| n.checked_add(u64::from(digit)))
        {
            Some(next) => value = next,
            None => overflow = true,
        }
        pos += 1;
    }
    if overflow {
        u64::MAX
    } else if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

fn scan_word<'a>(bytes: &'a [u8], position: &mut usize, limit: usize) -> Option<&'a [u8]> {
    while bytes
        .get(*position)
        .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == 0x0b)
    {
        *position += 1;
    }
    let start = *position;
    while bytes
        .get(*position)
        .is_some_and(|byte| !byte.is_ascii_whitespace() && *byte != 0x0b)
        && *position - start < limit
    {
        *position += 1;
    }
    if *position == start {
        None
    } else {
        Some(&bytes[start..*position])
    }
}

fn parse_functions(path: &Path, functions: &mut Vec<Function>) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let mut position = 0;
    while let Some(address) = scan_word(&bytes, &mut position, 16) {
        let Some(size) = scan_word(&bytes, &mut position, 16) else {
            break;
        };
        let Some(kind) = scan_word(&bytes, &mut position, 1) else {
            break;
        };
        scan_word(&bytes, &mut position, usize::MAX);
        if matches!(kind[0], b't' | b'T' | b'W') {
            functions.push(Function {
                address: hex_number(address),
                size: hex_number(size),
            });
        }
    }
    functions.sort_by_key(|function| function.address);
    Ok(())
}

fn io_error(error: &io::Error) -> String {
    error
        .to_string()
        .split(" (os error ")
        .next()
        .unwrap_or_default()
        .to_owned()
}

fn process_file(path: &Path, functions: &[Function]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|error| format!("{}: {}", path.display(), io_error(&error)))?;
    if !file
        .metadata()
        .map_err(|error| format!("{}: {}", path.display(), io_error(&error)))?
        .is_file()
    {
        return Err(format!("not a regular file: {}", path.display()));
    }
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|error| format!("{}: {}", path.display(), io_error(&error)))?;
    let patches =
        make_plan(&data, functions).map_err(|error| format!("{error} {}", path.display()))?;
    for patch in patches {
        file.seek(SeekFrom::Start(patch.offset))
            .and_then(|_| file.write_all(&patch.bytes))
            .map_err(|error| format!("{}: {}", path.display(), io_error(&error)))?;
    }
    file.flush()
        .map_err(|error| format!("{}: {}", path.display(), io_error(&error)))
}

fn run() -> u8 {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let mut functions = Vec::new();
    let mut files = Vec::new();
    let mut scan = true;
    let posix = env::var_os("POSIXLY_CORRECT").is_some();
    while let Some(arg) = args.next() {
        let bytes = arg.as_encoded_bytes();
        if scan && bytes == b"--" {
            scan = false;
            continue;
        }
        if !scan || !bytes.starts_with(b"-") || bytes.len() == 1 {
            files.push(arg);
            if posix {
                scan = false;
            }
            continue;
        }
        if bytes[1] != b's' {
            eprintln!(
                "{}: invalid option -- '{}'",
                program.to_string_lossy(),
                char::from(bytes[1])
            );
            eprintln!("usage: sorttable [-s nm-file] vmlinux...");
            return 0;
        }
        let symbols = if bytes.len() > 2 {
            OsString::from_vec(bytes[2..].to_vec())
        } else if let Some(arg) = args.next() {
            arg
        } else {
            eprintln!(
                "{}: option requires an argument -- 's'",
                program.to_string_lossy()
            );
            eprintln!("usage: sorttable [-s nm-file] vmlinux...");
            return 0;
        };
        if cfg!(MCOUNT_SORT_ENABLED) {
            if let Err(error) = parse_functions(Path::new(&symbols), &mut functions) {
                eprintln!("{}: {}", symbols.to_string_lossy(), io_error(&error));
                eprintln!("Could not parse {}", symbols.to_string_lossy());
                return 255;
            }
        }
    }
    if files.is_empty() {
        eprintln!("usage: sorttable vmlinux...");
        return 0;
    }
    let mut status = 0;
    for file in files {
        if let Err(error) = process_file(Path::new(&file), &functions) {
            eprintln!("{error}");
            status = 1;
        }
    }
    status
}

fn main() -> ExitCode {
    ExitCode::from(run())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
