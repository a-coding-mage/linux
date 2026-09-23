// SPDX-License-Identifier: GPL-2.0-only
//! Generate assembly embedding a compressed kernel and its size symbols.
// Copyright (C) 2009 Intel Corporation. All rights reserved.
// H. Peter Anvin <hpa@linux.intel.com>

use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

fn error_text(error: &io::Error) -> String {
    error
        .to_string()
        .split(" (os error ")
        .next()
        .unwrap_or_default()
        .into()
}

fn run() -> Result<(), String> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let name = args
        .next()
        .ok_or_else(|| format!("Usage: {} compressed_file", program.to_string_lossy()))?;
    let input_error =
        |error: io::Error| format!("{}: {}", name.to_string_lossy(), error_text(&error));
    let mut file = File::open(&name).map_err(input_error)?;
    file.seek(SeekFrom::End(-4)).map_err(input_error)?;
    let mut footer = [0; 4];
    file.read_exact(&mut footer).map_err(input_error)?;
    let input_len = file.stream_position().map_err(input_error)?;
    let output_len = u32::from_le_bytes(footer);

    let mut output = Vec::new();
    let _ = write!(output, ".section \".rodata..compressed\",\"a\",@progbits\n.globl z_input_len\nz_input_len = {input_len}\n.globl z_output_len\nz_output_len = {output_len}\n.globl input_data, input_data_end\ninput_data:\n.incbin \"");
    output.extend_from_slice(name.as_bytes());
    let _ = write!(output, "\"\ninput_data_end:\n.section \".rodata\",\"a\",@progbits\n.globl input_len\ninput_len:\n\t.long {input_len}\n.globl output_len\noutput_len:\n\t.long {output_len}\n");
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&output)
        .and_then(|()| stdout.flush())
        .map_err(|error| format!("stdout: {}", error_text(&error)))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
