// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) Paul Mackerras 1997.
//! Fill the XCOFF optional-header fields required by Open Firmware.

mod host_tool;

use host_tool::Failure;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn u16_be(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

fn run() -> Result<(), Failure> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() != 2 {
        return Err(Failure::new(1, "Usage: hack-coff coff-file\n"));
    }
    let path = Path::new(&arguments[1]);
    // The C program passes argv[2] (NULL) to perror(), so this diagnostic
    // intentionally has no pathname prefix.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|error| Failure::io(1, b"", error))?;
    let read_error = || Failure::path(1, "", path, ": read error or file too short\n");
    let mut header = [0u8; 20];
    file.read_exact(&mut header).map_err(|_| read_error())?;
    if !matches!(u16_be(&header, 0), 0o730 | 0o735 | 0o737) {
        return Err(Failure::path(1, "", path, ": not an xcoff file\n"));
    }
    let optional_size = usize::from(u16_be(&header, 16));
    let mut optional = [0u8; 72];
    if optional_size > optional.len() {
        return Err(Failure::path(1, "", path, ": optional header too large\n"));
    }
    file.read_exact(&mut optional[..optional_size])
        .map_err(|_| read_error())?;
    for index in 1..=u16_be(&header, 2) {
        let mut section = [0u8; 40];
        file.read_exact(&mut section).map_err(|_| read_error())?;
        let end = section[..8].iter().position(|&byte| byte == 0).unwrap_or(8);
        let fields: &[usize] = match &section[..end] {
            b".text" => &[32, 34],
            b".data" => &[36],
            b".bss" => &[42],
            _ => &[],
        };
        for &offset in fields {
            optional[offset..offset + 2].copy_from_slice(&index.to_be_bytes());
        }
    }
    optional[..2].copy_from_slice(&0x010bu16.to_be_bytes());
    file.seek(SeekFrom::Start(20))
        .and_then(|_| file.write_all(&optional[..optional_size]))
        .map_err(|_| Failure::path(1, "", path, ": write error\n"))?;
    Ok(())
}

fn main() {
    host_tool::finish(run());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
