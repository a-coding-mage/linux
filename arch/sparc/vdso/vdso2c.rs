// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2014 Andy Lutomirski and others
// Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
//! Prepare 32-bit and 64-bit SPARC vDSO images for embedding in the kernel.

#![forbid(unsafe_code)]

#[path = "vdso2c_header.rs"]
mod image;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

fn system_error(program: &[u8], operation: &[u8], error: io::Error) -> Vec<u8> {
    let message = error.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    [program, b": ", operation, b": ", message.as_bytes(), b"\n"].concat()
}

fn read_input(program: &[u8], name: &OsStr) -> Result<Vec<u8>, Vec<u8>> {
    let mut input = File::open(name).map_err(|err| system_error(program, name.as_bytes(), err))?;
    let length = input
        .seek(SeekFrom::End(0))
        .map_err(|err| system_error(program, b"lseek", err))?;
    if length == 0 {
        return Err(system_error(
            program,
            b"mmap",
            io::Error::from_raw_os_error(22),
        ));
    }
    // Read through the owned handle in place of mapping unchecked pointers.
    input
        .seek(SeekFrom::Start(0))
        .map_err(|err| system_error(program, b"lseek", err))?;
    let mut bytes = Vec::new();
    input
        .read_to_end(&mut bytes)
        .map_err(|err| system_error(program, b"mmap", err))?;
    Ok(bytes)
}

fn run() -> Result<(), Vec<u8>> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 4 {
        let _ = io::stdout().write_all(b"Usage: vdso2c RAW_INPUT STRIPPED_INPUT OUTPUT\n");
        return Err(Vec::new());
    }
    let program = args[0]
        .as_bytes()
        .rsplit(|byte| *byte == b'/')
        .next()
        .unwrap_or(b"vdso2c");
    let raw = read_input(program, &args[1])?;
    let stripped = read_input(program, &args[2])?;
    // Validate before truncating previous output. Aliased input/output paths
    // are also safe, unlike the original mmap-based implementation.
    image::validate(&raw, stripped.len()).map_err(|err| [b"Error: ", err.as_bytes()].concat())?;
    let output = args[3].as_bytes();
    let name = if output.ends_with(b".so") {
        None
    } else {
        Some(
            output
                .rsplit(|byte| *byte == b'/')
                .next()
                .unwrap_or_default()
                .split(|byte| *byte == b'.')
                .next()
                .unwrap_or_default()
                .iter()
                .map(|byte| if *byte == b'-' { b'_' } else { *byte })
                .collect::<Vec<_>>(),
        )
    };
    // Preserve the original fopen error's (surprising) stripped-input filename.
    let file =
        File::create(&args[3]).map_err(|err| system_error(program, args[2].as_bytes(), err))?;
    let mut writer = BufWriter::new(file);
    image::emit(&mut writer, &stripped, name.as_deref())
        .and_then(|()| writer.flush())
        .map_err(|err| system_error(program, output, err))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            let _ = io::stderr().write_all(&message);
            ExitCode::FAILURE
        }
    }
}
