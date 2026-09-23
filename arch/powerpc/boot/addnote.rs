// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2000 Paul Mackerras.
// Adapted for 64-bit little-endian images by Andrew Tauferner.
//! Add CHRP and RPA Open Firmware notes to an ELF program-header table.

mod host_tool;

use host_tool::Failure;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn number(data: &[u8], offset: usize, width: usize, big: bool) -> u64 {
    let bytes = &data[offset..offset + width];
    if big {
        bytes
            .iter()
            .fold(0, |value, byte| value << 8 | u64::from(*byte))
    } else {
        bytes
            .iter()
            .rev()
            .fold(0, |value, byte| value << 8 | u64::from(*byte))
    }
}

fn put(data: &mut [u8], offset: usize, width: usize, value: u64, big: bool) {
    for index in 0..width {
        let shift = if big { width - index - 1 } else { index };
        data[offset + index] = (value >> (shift * 8)) as u8;
    }
}

fn run() -> Result<(), Failure> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() != 2 {
        return Err(Failure::path(
            1,
            "Usage: ",
            Path::new(&arguments[0]),
            " elf-file\n",
        ));
    }
    let path = Path::new(&arguments[1]);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|error| Failure::io(1, path.as_os_str().as_encoded_bytes(), error))?;
    let mut buffer = [0u8; 1024];
    let length = file
        .read(&mut buffer)
        .map_err(|error| Failure::io(1, b"read", error))?;
    let not_elf = || Failure::path(1, "", path, " does not appear to be an ELF file\n");
    let no_space = || {
        Failure::path(
            1,
            "sorry, I can't find space in ",
            path,
            " to put the note\n",
        )
    };
    if length < 6
        || &buffer[..4] != b"\x7fELF"
        || !matches!(buffer[4], 1 | 2)
        || !matches!(buffer[5], 1 | 2)
    {
        return Err(not_elf());
    }
    let is_64 = buffer[4] == 2;
    let big = buffer[5] == 2;
    let (header, phoff_at, ps_at, np_at, ph_size, offset_at, filesz_at, word) = if is_64 {
        (64, 32, 54, 56, 56, 8, 32, 8)
    } else {
        (52, 28, 42, 44, 32, 4, 16, 4)
    };
    if length < header {
        return Err(not_elf());
    }
    let ph = number(&buffer, phoff_at, word, big);
    let size = number(&buffer, ps_at, 2, big);
    let count = number(&buffer, np_at, 2, big);
    if ph < header as u64 || size < ph_size || count == 0 {
        return Err(not_elf());
    }
    // CHRP is 44 bytes; RPA is 68 bytes. All note and header edits must fit in
    // the first read, not merely within the ELF's total file length.
    let required = (count + 2)
        .checked_mul(size)
        .and_then(|n| ph.checked_add(n))
        .and_then(|n| n.checked_add(44 + 68))
        .ok_or_else(no_space)?;
    if required > length as u64 {
        return Err(no_space());
    }
    let (mut ph, size, count) = (ph as usize, size as usize, count as usize);
    for _ in 0..count {
        if number(&buffer, ph, 4, big) == 4 {
            Failure::path(0, "", path, " already has a note entry\n").print();
            return Ok(());
        }
        ph += size;
    }
    if buffer[ph..required as usize].iter().any(|&byte| byte != 0) {
        return Err(no_space());
    }
    let mut note = ph + size * 2;
    put(&mut buffer, ph, 4, 4, big);
    put(&mut buffer, ph + offset_at, word, note as u64, big);
    put(&mut buffer, ph + filesz_at, word, 44, big);
    put(&mut buffer, note, 4, 8, big);
    put(&mut buffer, note + 4, 4, 24, big);
    put(&mut buffer, note + 8, 4, 0x1275, big);
    buffer[note + 12..note + 20].copy_from_slice(b"PowerPC\0");
    note += 20;
    for value in [
        0xffffffff, 0x02000000, 0xffffffff, 0xffffffff, 0xffffffff, 0x4000,
    ] {
        put(&mut buffer, note, 4, value, true);
        note += 4;
    }
    ph += size;
    put(&mut buffer, ph, 4, 4, big);
    put(&mut buffer, ph + offset_at, word, note as u64, big);
    // Retain the existing ELF32 note-size convention exactly.
    put(
        &mut buffer,
        ph + filesz_at,
        word,
        if is_64 { 68 } else { 44 },
        big,
    );
    put(&mut buffer, note, 4, 22, big);
    put(&mut buffer, note + 4, 4, 32, big);
    put(&mut buffer, note + 8, 4, 0x12759999, big);
    buffer[note + 12..note + 34].copy_from_slice(b"IBM,RPA-Client-Config\0");
    note += 36;
    for value in [0, 64, 0, 40, 1, 0xffffffff, 0, 1] {
        put(&mut buffer, note, 4, value, true);
        note += 4;
    }
    put(&mut buffer, np_at, 2, (count + 2) as u64, big);
    file.seek(SeekFrom::Start(0))
        .map_err(|error| Failure::io(1, b"lseek", error))?;
    file.write_all(&buffer[..length])
        .map_err(|error| Failure::io(1, b"write", error))?;
    Ok(())
}

fn main() {
    host_tool::finish(run());
}
