// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2023 - Google LLC
// Author: Ard Biesheuvel <ardb@google.com>
//! Check early ARM64 data relocations and convert opted-in ABS64 references.

#[path = "../../../../scripts/elf-parse.rs"]
mod elf;

use elf::ElfFile;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

struct Edits {
    words: Vec<(u64, [u8; 8])>,
    rejected: bool,
}

fn inspect(data: &[u8]) -> Result<Edits, String> {
    let elf = ElfFile::parse(data, u32::MAX)?;
    if elf.word_size() != 8 {
        return Err("expected ELF64 input".into());
    }
    let mut edits = Edits {
        words: Vec::new(),
        rejected: false,
    };
    for section in elf.sections()? {
        if section.kind != 4 {
            continue;
        }
        let target = elf.section(section.info as usize)?;
        if target.flags & (2 | 4) != 2 {
            continue;
        }
        let prel64 = elf
            .section_name(&target)?
            .windows(b".rodata.prel64".len())
            .any(|window| window == b".rodata.prel64");
        // The original walks native Elf64_Rela-sized records, independently
        // of sh_entsize, and leaves any incomplete final record untouched.
        let entries = elf.section_data(&section)?.len() / 24;
        for index in 0..entries {
            let offset = section.offset + (index as u64) * 24 + 8;
            let mut info = elf.read_integer(offset, 8)?;
            if info as u32 != 257 {
                continue;
            }
            if !prel64 {
                edits.rejected = true;
                return Ok(edits);
            }
            info ^= 257 ^ 260;
            edits.words.push((
                offset,
                if elf.little_endian() {
                    info.to_le_bytes()
                } else {
                    info.to_be_bytes()
                },
            ));
        }
    }
    Ok(edits)
}

fn path_message(prefix: &str, path: &Path) -> Vec<u8> {
    let mut message = prefix.as_bytes().to_vec();
    message.extend_from_slice(path.as_os_str().as_encoded_bytes());
    message.push(b'\n');
    message
}

fn run(input: &Path, original: &Path) -> Result<(), Vec<u8>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input)
        .map_err(|_| path_message("failed to open ", input))?;
    let metadata = file
        .metadata()
        .map_err(|_| path_message("failed to stat() ", input))?;
    if metadata.len() == 0 {
        return Err(path_message("failed to mmap() ", input));
    }
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|_| path_message("failed to mmap() ", input))?;
    let edits = inspect(&data).map_err(|error| {
        let mut message = path_message("invalid ELF input ", input);
        message.pop();
        message.extend_from_slice(b": ");
        message.extend_from_slice(error.as_bytes());
        message.push(b'\n');
        message
    })?;
    for (offset, bytes) in edits.words {
        file.seek(SeekFrom::Start(offset))
            .and_then(|_| file.write_all(&bytes))
            .map_err(|_| path_message("failed to write ", input))?;
    }
    if edits.rejected {
        // Keep the C behavior: earlier conversions remain visible through
        // other links to this inode, but the rejected output is unlinked.
        drop(file);
        let _ = fs::remove_file(input);
        return Err(path_message(
            "Unexpected absolute relocations detected in ",
            original,
        ));
    }
    Ok(())
}

fn main() {
    let arguments: Vec<_> = std::env::args_os().collect();
    let result = if arguments.len() < 3 {
        Err(b"file arguments missing\n".to_vec())
    } else {
        run(Path::new(&arguments[1]), Path::new(&arguments[2]))
    };
    if let Err(message) = result {
        let _ = io::stderr().lock().write_all(&message);
        std::process::exit(1);
    }
}
