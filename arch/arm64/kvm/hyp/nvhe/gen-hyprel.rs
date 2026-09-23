// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2020 - Google LLC
// Author: David Brazdil <dbrazdil@google.com>
//! Generate relative pointers to absolute addresses in the nVHE hypervisor.

#[path = "../../../../../scripts/elf-parse.rs"]
mod elf;

use elf::ElfFile;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

const TARGET_LITTLE: bool = !cfg!(CONFIG_CPU_BIG_ENDIAN);

fn errno_text(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .into()
}

fn checked<T>(result: Result<T, String>) -> Result<T, Vec<u8>> {
    result.map_err(String::into_bytes)
}

fn output(result: io::Result<()>) -> Result<(), Vec<u8>> {
    result.map_err(|error| format!("Could not write output: {}", errno_text(&error)).into_bytes())
}

// Retain the C assertions' decoded values and original source locations. Use
// the portable expressions instead of host-libc-specific macro expansions.
// File-controlled extents, names and indices additionally use the checked ELF
// reader rather than reproducing unchecked pointer arithmetic.
fn assertion(expression: &str, lhs: String, rhs: String, line: u32) -> Vec<u8> {
    format!("assertion {expression} failed (lhs={lhs}, rhs={rhs}, line={line})").into_bytes()
}

fn inspect(data: &[u8]) -> Result<ElfFile<'_>, Vec<u8>> {
    if data.len() < 64 {
        return Err(assertion(
            "stat.st_size >= sizeof(*elf.ehdr)",
            data.len().to_string(),
            "64".into(),
            279,
        ));
    }
    for (index, expected, constant) in [
        (0, 0x7f, "ELFMAG0"),
        (1, b'E', "ELFMAG1"),
        (2, b'L', "ELFMAG2"),
        (3, b'F', "ELFMAG3"),
    ] {
        if data[index] != expected {
            return Err(assertion(
                &format!("elf.ehdr->e_ident[EI_MAG{index}] == {constant}"),
                format!("0x{:x}", data[index]),
                format!("0x{expected:x}"),
                283 + index as u32,
            ));
        }
    }
    for (index, expected, expression, line) in [
        (4, 2, "elf.ehdr->e_ident[EI_CLASS] == ELFCLASS64", 289),
        (
            5,
            if TARGET_LITTLE { 1 } else { 2 },
            "elf.ehdr->e_ident[EI_DATA] == ELFENDIAN",
            290,
        ),
    ] {
        if data[index] != expected {
            return Err(assertion(
                expression,
                data[index].to_string(),
                expected.to_string(),
                line,
            ));
        }
    }
    let half = |offset| {
        let bytes = [data[offset], data[offset + 1]];
        if TARGET_LITTLE {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        }
    };
    for (offset, expected, expression, line) in [
        (16, 1, "elf16toh(elf.ehdr->e_type) == ET_REL", 291),
        (18, 183, "elf16toh(elf.ehdr->e_machine) == EM_AARCH64", 292),
    ] {
        if half(offset) != expected {
            return Err(assertion(
                expression,
                half(offset).to_string(),
                expected.to_string(),
                line,
            ));
        }
    }
    if data[40..48] == [0; 8] {
        return Err(assertion("off != 0UL", "0".into(), "0".into(), 233));
    }
    if half(62) == 0 {
        return Err(assertion("idx != SHN_UNDEF", "0".into(), "0".into(), 240));
    }
    checked(ElfFile::parse(data, 1 << 1))
}

fn emit(image: &ElfFile<'_>, out: &mut impl Write) -> Result<(), Vec<u8>> {
    output(out.write_all(b".data\n.pushsection .hyp.reloc, \"a\"\n"))?;
    let mut reloc_offset = 0u64;
    for section in checked(image.sections())? {
        if section.kind == 9 {
            let mut error = b"Unexpected SHT_REL section \"".to_vec();
            error.extend_from_slice(checked(image.section_name(&section))?);
            error.push(b'"');
            return Err(error);
        }
        if section.kind != 4 {
            continue;
        }
        let target = checked(image.section(section.info as usize))?;
        let name = checked(image.section_name(&target))?;
        if !name.starts_with(b".hyp") {
            continue;
        }
        output(out.write_all(b".global __hyp_section_"))?;
        output(out.write_all(name))?;
        output(out.write_all(b"\n"))?;
        // C walks fixed-size Elf64_Rela records, independent of sh_entsize.
        let records = checked(image.section_data(&section))?;
        if records.len() % 24 != 0 {
            return Err(b"Truncated ELF64 RELA record".to_vec());
        }
        for (index, _) in records.chunks_exact(24).enumerate() {
            let position = section.offset + index as u64 * 24;
            let offset = checked(image.read_integer(position, 8))?;
            let kind = checked(image.read_integer(position + 8, 8))? as u32;
            if offset >= target.size {
                return Err(assertion(
                    "elf64toh(rela->r_offset) < elf64toh(sh_orig->sh_size)",
                    format!("0x{offset:x}"),
                    format!("0x{:x}", target.size),
                    379,
                ));
            }
            match kind {
                257 => {
                    output(write!(
                        out,
                        ".word 0\n.reloc {reloc_offset}, R_AARCH64_PREL32, __hyp_section_"
                    ))?;
                    output(out.write_all(name))?;
                    output(writeln!(out, " + 0x{offset:x}"))?;
                    reloc_offset = reloc_offset
                        .checked_add(4)
                        .ok_or_else(|| b"Relocation offset overflow".to_vec())?;
                }
                // ABS32 is a kCFI hash; the remaining accepted types are
                // relative data, PC-relative addressing and branch fixups.
                258 | 260..=262 | 273..=280 | 282..=293 | 299 | 314 => {}
                _ => return Err(format!("Unexpected RELA type {kind}").into_bytes()),
            }
        }
    }
    output(out.write_all(b".popsection\n"))
}

fn run(path: &Path, out: &mut impl Write) -> Result<(), Vec<u8>> {
    let mut file = File::open(path)
        .map_err(|error| format!("Could not open ELF file: {}", errno_text(&error)).into_bytes())?;
    let metadata = file.metadata().map_err(|error| {
        format!("Could not get status of ELF file: {}", errno_text(&error)).into_bytes()
    })?;
    if metadata.len() == 0 {
        return Err(b"Could not mmap ELF file: Invalid argument".to_vec());
    }
    if metadata.is_dir() {
        return Err(b"Could not mmap ELF file: No such device".to_vec());
    }
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|error| format!("Could not read ELF file: {}", errno_text(&error)).into_bytes())?;
    emit(&inspect(&data)?, out)
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 {
        let mut error = b"Usage: ".to_vec();
        error.extend_from_slice(args[0].as_encoded_bytes());
        error.extend_from_slice(b" <elf_input>\n");
        let _ = io::stderr().write_all(&error);
        std::process::exit(1);
    }
    let mut out = BufWriter::new(io::stdout().lock());
    let result = run(Path::new(&args[1]), &mut out);
    // Flush even after a semantic error to retain the reference's partial
    // assembly output; unlike printf, report delayed output failures.
    let flushed = output(out.flush());
    if let Err(message) = result.and(flushed) {
        let mut error = b"error: ".to_vec();
        error.extend_from_slice(args[1].as_encoded_bytes());
        error.extend_from_slice(b": ");
        error.extend_from_slice(&message);
        error.push(b'\n');
        let _ = io::stderr().write_all(&error);
        std::process::exit(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
