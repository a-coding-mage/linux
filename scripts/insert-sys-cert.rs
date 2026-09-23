// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) IBM Corporation, 2015
// Original author: Mehmet Kayaalp <mkayaalp@linux.vnet.ibm.com>

//! Replace the reserved extra-certificate area in a native-endian kernel ELF.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::os::unix::ffi::{OsStrExt, OsStringExt};

#[path = "elf-parse.rs"]
mod elf;
use elf::{ElfFile, Section};

const NAMES: [&str; 3] = [
    "system_extra_cert",
    "system_extra_cert_used",
    "system_certificate_list_size",
];

#[derive(Default)]
struct Messages {
    output: Vec<u8>,
    errors: Vec<u8>,
}

impl Messages {
    fn info(&mut self, message: &str) {
        self.errors.extend_from_slice(b"INFO:    ");
        self.errors.extend_from_slice(message.as_bytes());
        self.errors.push(b'\n');
    }

    fn warning(&mut self, message: &str) {
        self.output.extend_from_slice(b"WARNING: ");
        self.output.extend_from_slice(message.as_bytes());
        self.output.push(b'\n');
    }

    fn error(&mut self, message: &str) {
        self.errors.extend_from_slice(b"ERROR:   ");
        self.errors.extend_from_slice(message.as_bytes());
        self.errors.push(b'\n');
    }

    fn usage(&mut self, executable: &OsStr) {
        self.output.extend_from_slice(b"Usage ");
        self.output.extend_from_slice(executable.as_bytes());
        self.output
            .extend_from_slice(b" [-s <System.map>] -b <vmlinux> -c <certfile>\n");
    }
}

fn io_error(context: &OsStr, error: io::Error) -> String {
    let text = error.to_string();
    let text = text.split(" (os error ").next().unwrap_or(&text);
    format!("{}: {text}", context.to_string_lossy())
}

fn invalid(message: &str) -> String {
    format!("ERROR:   {message}")
}

#[derive(Clone, Copy)]
struct LocatedSymbol {
    address: u64,
    offset: u64,
    size: u64,
    available: u64,
}

fn in_section(section: &Section, address: u64, size: u64) -> Result<LocatedSymbol, String> {
    if section.kind == 8 {
        return Err("certificate symbol is in an uninitialized section".into());
    }
    let relative = address
        .checked_sub(section.address)
        .ok_or("certificate symbol precedes its section")?;
    let available = section
        .size
        .checked_sub(relative)
        .ok_or("certificate symbol exceeds its section")?;
    if size > available {
        return Err("certificate symbol exceeds its section".into());
    }
    Ok(LocatedSymbol {
        address,
        offset: section
            .offset
            .checked_add(relative)
            .ok_or("ELF offset overflow")?,
        size,
        available,
    })
}

fn from_table(
    elf: &ElfFile<'_>,
    table: &Section,
    name: &str,
) -> Result<Option<LocatedSymbol>, String> {
    let strings = elf.section(table.link as usize)?;
    for (index, symbol) in elf.symbols(table)?.iter().enumerate() {
        if elf.string(&strings, symbol.name)? == name.as_bytes() {
            if symbol.section == 0 {
                return Ok(None);
            }
            let section = elf.symbol_section(table, index, symbol)?;
            return in_section(&section, symbol.value, symbol.size).map(Some);
        }
    }
    Err(format!("Unable to find symbol: {name}"))
}

fn from_map(elf: &ElfFile<'_>, map: &[u8], name: &str) -> Result<Option<LocatedSymbol>, String> {
    for line in map.split_inclusive(|&byte| byte == b'\n') {
        if line.last() != Some(&b'\n') || line.len() >= 100 {
            return Err("Missing line ending.".into());
        }
        let fields: Vec<_> = line
            .split(|byte| byte.is_ascii_whitespace())
            .filter(|field| !field.is_empty())
            .collect();
        // Match the entire symbol, not a substring of system_extra_cert_used.
        if fields.get(2).copied() != Some(name.as_bytes()) {
            continue;
        }
        let address = std::str::from_utf8(fields[0])
            .ok()
            .and_then(|value| u64::from_str_radix(value.trim_start_matches("0x"), 16).ok())
            .ok_or("Invalid System.map address.")?;
        if address == 0 {
            return Ok(None);
        }
        for section in elf.sections()?.iter().skip(1) {
            if address >= section.address && address - section.address < section.size {
                return in_section(section, address, 0).map(Some);
            }
        }
        return Ok(None);
    }
    Err(format!("Unable to find symbol: {name}"))
}

fn checked_range(symbol: LocatedSymbol, size: u64, length: usize) -> Result<Range<usize>, String> {
    if size > symbol.available {
        return Err("certificate data exceeds its ELF section".into());
    }
    let start = usize::try_from(symbol.offset).map_err(|_| "ELF offset overflow")?;
    let size = usize::try_from(size).map_err(|_| "certificate size overflow")?;
    let end = start.checked_add(size).ok_or("ELF offset overflow")?;
    if start == 0 || end > length {
        return Err("certificate data exceeds the ELF file".into());
    }
    Ok(start..end)
}

struct Replacement {
    address: u64,
    changes: Vec<(usize, Vec<u8>)>,
}

fn replacement(
    image: &[u8],
    certificate: &[u8],
    map_path: Option<&OsStr>,
    executable: &OsStr,
    messages: &mut Messages,
) -> Result<Option<Replacement>, String> {
    let width = std::mem::size_of::<usize>();
    if image.len() < if width == 8 { 64 } else { 52 } {
        return Err(invalid("Invalid ELF file."));
    }
    if &image[..4] != b"\x7fELF" {
        return Err(invalid("Invalid ELF magic."));
    }
    if image[4] != if width == 8 { 2 } else { 1 } {
        return Err(invalid("ELF class mismatch."));
    }
    if image[5] != if cfg!(target_endian = "little") { 1 } else { 2 } {
        return Err(invalid("ELF endian mismatch."));
    }
    let shoff = if width == 8 {
        u64::from_ne_bytes(image[40..48].try_into().unwrap())
    } else {
        u64::from(u32::from_ne_bytes(image[32..36].try_into().unwrap()))
    };
    if shoff > image.len() as u64 {
        return Err(invalid("Could not find section header."));
    }
    let elf = ElfFile::parse(image, u32::MAX).map_err(|error| invalid(&error))?;
    let table = elf
        .sections()
        .map_err(|error| invalid(&error))?
        .into_iter()
        .skip(1)
        .find(|section| section.kind == 2);
    let map = if table.is_none() {
        messages.warning("Could not find the symbol table.");
        let Some(path) = map_path else {
            messages.usage(executable);
            return Err(invalid("Please provide a System.map file."));
        };
        fs::read(path).map_err(|error| io_error(path, error))?
    } else {
        messages.info("Symbol table found.");
        if map_path.is_some() {
            messages.warning("System.map is ignored.");
        }
        Vec::new()
    };
    let mut symbols = Vec::new();
    for name in NAMES {
        let result = if let Some(table) = &table {
            from_table(&elf, table, name)
        } else {
            from_map(&elf, &map, name)
        };
        match result {
            Ok(Some(symbol)) if symbol.offset != 0 => symbols.push(symbol),
            Ok(_) => (),
            Err(error) => messages.error(&error),
        }
    }
    if symbols.len() != NAMES.len() {
        return Err(String::new());
    }
    if table.is_none() {
        symbols[0].size = symbols[1]
            .address
            .checked_sub(symbols[0].address)
            .ok_or_else(|| invalid("Invalid certificate reserve size."))?;
    }
    for (name, symbol) in NAMES.iter().zip(&symbols) {
        messages.info(&format!("sym:    {name}"));
        messages.info(&format!("addr:   0x{:x}", symbol.address));
        messages.info(&format!("size:   {}", symbol.size));
        messages.info(&format!("offset: 0x{:x}", symbol.offset));
    }
    if certificate.len() as u64 > symbols[0].size {
        return Err(invalid("Certificate is larger than the reserved area!"));
    }
    let ranges = [
        checked_range(symbols[0], symbols[0].size, image.len()),
        checked_range(symbols[1], 4, image.len()),
        checked_range(symbols[2], width as u64, image.len()),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(|error| invalid(&error))?;
    for (index, range) in ranges.iter().enumerate() {
        for other in &ranges[..index] {
            if range.start < other.end && other.start < range.end {
                return Err(invalid("Overlapping certificate symbols."));
            }
        }
    }
    let used = i32::from_ne_bytes(image[ranges[1].clone()].try_into().unwrap());
    if used < 0 || used as u64 > symbols[0].size {
        return Err(invalid("Invalid existing certificate size."));
    }
    let list_size = elf
        .read_integer(symbols[2].offset, width)
        .map_err(|error| invalid(&error))?;
    let new_size = list_size
        .checked_sub(used as u64)
        .and_then(|size| size.checked_add(certificate.len() as u64))
        .filter(|&size| width == 8 || size <= u64::from(u32::MAX))
        .ok_or_else(|| invalid("Certificate list size overflow."))?;
    let cert_size =
        i32::try_from(certificate.len()).map_err(|_| invalid("Certificate size overflow."))?;
    // Certificates are binary: C's strncmp stopped at embedded NUL bytes.
    if used == cert_size
        && image[ranges[0].start..ranges[0].start + certificate.len()] == *certificate
    {
        messages.warning("Certificate was already inserted.");
        return Ok(None);
    }
    if used > 0 {
        messages.warning("Replacing previously inserted certificate.");
    }
    let mut reserved = vec![0; ranges[0].len()];
    reserved[..certificate.len()].copy_from_slice(certificate);
    let size_bytes = if width == 8 {
        new_size.to_ne_bytes().to_vec()
    } else {
        (new_size as u32).to_ne_bytes().to_vec()
    };
    Ok(Some(Replacement {
        address: symbols[0].address,
        changes: vec![
            (ranges[0].start, reserved),
            (ranges[1].start, cert_size.to_ne_bytes().to_vec()),
            (ranges[2].start, size_bytes),
        ],
    }))
}

fn run(arguments: &[OsString], messages: &mut Messages) -> Result<(), String> {
    let mut paths: [Option<OsString>; 3] = [None, None, None];
    let stop_at_operand = env::var_os("POSIXLY_CORRECT").is_some();
    let mut index = 1;
    while index < arguments.len() {
        let argument = arguments[index].as_bytes();
        index += 1;
        if argument == b"--" {
            break;
        }
        if !argument.starts_with(b"-") || argument == b"-" {
            if stop_at_operand {
                break;
            }
            continue;
        }
        let mut position = 1;
        while position < argument.len() {
            let option = argument[position];
            position += 1;
            let target = match option {
                b'b' => 0,
                b'c' => 1,
                b's' => 2,
                _ => {
                    messages.errors.extend_from_slice(arguments[0].as_bytes());
                    messages.errors.extend_from_slice(b": invalid option -- '");
                    messages.errors.push(option);
                    messages.errors.extend_from_slice(b"'\n");
                    continue;
                }
            };
            if position < argument.len() {
                paths[target] = Some(OsString::from_vec(argument[position..].to_vec()));
            } else if index < arguments.len() {
                paths[target] = Some(arguments[index].clone());
                index += 1;
            } else {
                messages.errors.extend_from_slice(arguments[0].as_bytes());
                messages
                    .errors
                    .extend_from_slice(b": option requires an argument -- '");
                messages.errors.push(option);
                messages.errors.extend_from_slice(b"'\n");
            }
            break;
        }
    }
    let (Some(image_path), Some(cert_path)) = (&paths[0], &paths[1]) else {
        messages.usage(&arguments[0]);
        return Err(String::new());
    };
    let certificate = fs::read(cert_path).map_err(|error| io_error(cert_path, error))?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(image_path)
        .map_err(|error| io_error(image_path, error))?;
    let mut image = Vec::new();
    file.read_to_end(&mut image)
        .map_err(|error| io_error(OsStr::new("File read failed"), error))?;
    let Some(replacement) = replacement(
        &image,
        &certificate,
        paths[2].as_deref(),
        &arguments[0],
        messages,
    )?
    else {
        return Ok(());
    };
    // Validate all ranges before writing. Preserve the inode, permissions,
    // hard links and unrelated bytes, just as MAP_SHARED did in the C tool.
    for (offset, bytes) in &replacement.changes {
        file.seek(SeekFrom::Start(*offset as u64))
            .and_then(|_| file.write_all(bytes))
            .map_err(|error| io_error(OsStr::new("File write failed"), error))?;
    }
    messages
        .errors
        .extend_from_slice(b"INFO:    Inserted the contents of ");
    messages.errors.extend_from_slice(cert_path.as_bytes());
    messages
        .errors
        .extend_from_slice(format!(" into {:x}.\n", replacement.address).as_bytes());
    messages.info(&format!(
        "Used {} bytes out of {} bytes reserved.",
        certificate.len(),
        replacement.changes[0].1.len()
    ));
    Ok(())
}

fn main() {
    let mut messages = Messages::default();
    let result = run(&env::args_os().collect::<Vec<_>>(), &mut messages);
    if let Err(error) = &result {
        if !error.is_empty() {
            messages.errors.extend_from_slice(error.as_bytes());
            messages.errors.push(b'\n');
        }
    }
    let output = io::stdout().lock().write_all(&messages.output);
    let errors = io::stderr().lock().write_all(&messages.errors);
    if result.is_err() || output.is_err() || errors.is_err() {
        std::process::exit(1);
    }
}
