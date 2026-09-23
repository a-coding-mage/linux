// SPDX-License-Identifier: GPL-2.0
//! Build the historical IBM evaluation-board tree-boot image and checksum.

mod host_tool;

use host_tool::Failure;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn address(text: &[u8]) -> u32 {
    let mut text = text;
    while text
        .first()
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'..=b'\r'))
    {
        text = &text[1..];
    }
    let negative = text.first() == Some(&b'-');
    if matches!(text.first(), Some(b'+' | b'-')) {
        text = &text[1..];
    }
    let base = if text.len() >= 3
        && text[0] == b'0'
        && matches!(text[1], b'x' | b'X')
        && text[2].is_ascii_hexdigit()
    {
        text = &text[2..];
        16
    } else if text.first() == Some(&b'0') {
        8
    } else {
        10
    };
    let mut value = 0usize;
    let mut overflow = false;
    for &byte in text {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => break,
        } as usize;
        if digit >= base {
            break;
        }
        match value.checked_mul(base).and_then(|n| n.checked_add(digit)) {
            Some(next) => value = next,
            None => overflow = true,
        }
    }
    if overflow {
        value = usize::MAX;
    } else if negative {
        value = value.wrapping_neg();
    }
    value as u32
}

fn checksum(bytes: &[u8]) -> u32 {
    bytes.chunks_exact(4).fold(0u32, |sum, chunk| {
        sum.wrapping_add(u32::from_ne_bytes(
            chunk.try_into().expect("four-byte checksum word"),
        ))
    })
}

fn run() -> Result<(), Failure> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() < 5 {
        return Err(Failure::path(
            1,
            "usage: ",
            Path::new(&arguments[0]),
            " <zImage-file> <boot-image> <load address> <entry point>\n",
        ));
    }
    let input_path = Path::new(&arguments[1]);
    let size = fs::metadata(input_path)
        .map_err(|error| Failure::io(2, b"stat", error))?
        .len();
    let mut blocks = ((size + 512) / 512) as i32;
    let mut header = [0u8; 32];
    for (offset, value) in [
        (0, 0x0052504f),
        (4, address(arguments[3].as_encoded_bytes())),
        (8, blocks as u32),
        (16, address(arguments[4].as_encoded_bytes())),
    ] {
        header[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
    let mut input =
        File::open(input_path).map_err(|error| Failure::io(3, b"zImage open", error))?;
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&arguments[2])
        .map_err(|error| Failure::io(3, b"bootfile open", error))?;
    let mut sum = checksum(&header);
    let mut block = [0u8; 512];
    if input.read(&mut block).ok() != Some(block.len()) {
        return Err(Failure::path(
            4,
            "",
            input_path,
            " is too small to be an ELF image\n",
        ));
    }
    if &block[..4] != b"\x7fELF" {
        return Err(Failure::path(4, "", input_path, " is not an ELF image\n"));
    }
    input
        .seek(SeekFrom::Start(65536))
        .map_err(|_| Failure::path(4, "", input_path, " failed to seek in ELF image\n"))?;
    blocks = blocks.wrapping_sub(128);
    output
        .write_all(&header)
        .map_err(|error| Failure::io(5, b"boot-image write", error))?;
    for _ in 0..blocks {
        // Preserve the original image format's retained tail bytes after a
        // short final read, and its extra block for sector-aligned inputs.
        input
            .read(&mut block)
            .map_err(|error| Failure::io(5, b"zImage read", error))?;
        sum = sum.wrapping_add(checksum(&block));
        output
            .write_all(&block)
            .map_err(|error| Failure::io(5, b"boot-image write", error))?;
    }
    header[20..24].copy_from_slice(&sum.to_be_bytes());
    output
        .seek(SeekFrom::Start(0))
        .map_err(|error| Failure::io(1, b"rewrite seek", error))?;
    output
        .write_all(&header)
        .map_err(|error| Failure::io(1, b"boot-image rewrite", error))?;
    Ok(())
}

fn main() {
    host_tool::finish(run());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
