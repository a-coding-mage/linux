// SPDX-License-Identifier: GPL-2.0
//! Read a MIPS kernel ELF entry address, canonically extending ELF32 addresses.

#[path = "../boot/host_tool.rs"]
mod host_tool;

use std::fs::File;
use std::io::{self, Read, Write};

fn run() -> Result<(), Vec<u8>> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() != 2 {
        return Err(b"Usage: elf-entry <elf-file>\n".to_vec());
    }
    let mut input = File::open(&arguments[1])
        .map_err(|error| host_tool::perror("Unable to open input file", error))?;
    // The original reads the ELF64-sized union even for an ELF32 input.
    let mut header = [0u8; 64];
    input.read_exact(&mut header).map_err(|error| {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            b"Unable to read input file: Success\n".to_vec()
        } else {
            host_tool::perror("Unable to read input file", error)
        }
    })?;
    if &header[..4] != b"\x7fELF" {
        return Err(b"Input is not an ELF\n".to_vec());
    }
    let width = match header[4] {
        1 => 4,
        2 => 8,
        _ => return Err(b"Invalid ELF class\n".to_vec()),
    };
    let mut entry = match header[5] {
        1 => header[24..24 + width]
            .iter()
            .rev()
            .fold(0u64, |value, &byte| value << 8 | u64::from(byte)),
        2 => header[24..24 + width]
            .iter()
            .fold(0u64, |value, &byte| value << 8 | u64::from(byte)),
        _ => return Err(b"Invalid ELF encoding\n".to_vec()),
    };
    if width == 4 {
        entry = entry as i32 as i64 as u64;
    }
    writeln!(io::stdout().lock(), "0x{entry:016x}")
        .map_err(|error| host_tool::perror("write", error))?;
    Ok(())
}

fn main() {
    host_tool::finish(run());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
