// SPDX-License-Identifier: GPL-2.0-only
//! Prepare checked x86 vDSO images and their kernel-side metadata.
// Copyright (c) 2014 Andy Lutomirski and others

#[path = "../../../scripts/elf-parse.rs"]
mod elf;
#[path = "vdso2c_header.rs"]
mod image;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::process::ExitCode;

fn io_error(operation: &str, path: &OsStr, error: io::Error) -> String {
    let message = error.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    format!("vdso2c: {operation}({}): {message}", path.to_string_lossy())
}

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 4 {
        println!("Usage: vdso2c RAW_INPUT STRIPPED_INPUT OUTPUT");
        return Err(String::new());
    }
    let raw = fs::read(&args[1]).map_err(|error| io_error("open", &args[1], error))?;
    let stripped = fs::read(&args[2]).map_err(|error| io_error("open", &args[2], error))?;
    let path = Path::new(&args[3]);
    let filename = path
        .file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .as_bytes();
    let image_name = if args[3].as_bytes().ends_with(b".so") {
        None
    } else {
        Some(
            filename
                .split(|&byte| byte == b'.')
                .next()
                .unwrap_or_default()
                .iter()
                .map(|&byte| if byte == b'-' { b'_' } else { byte })
                .collect::<Vec<_>>(),
        )
    };
    // Finish validation before opening the destination. Malformed inputs must
    // not remove a previous output or destroy an input aliased by that output.
    let data = image::generate(&raw, &stripped, image_name.as_deref())
        .map_err(|error| format!("Error: {error}"))?;
    let mut output = fs::File::create(path).map_err(|error| io_error("fopen", &args[3], error))?;
    output
        .write_all(&data)
        .and_then(|()| output.flush())
        .map_err(|error| io_error("write", &args[3], error))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if !error.is_empty() {
                eprintln!("{error}");
            }
            ExitCode::FAILURE
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
