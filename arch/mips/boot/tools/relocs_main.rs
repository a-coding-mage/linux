// SPDX-License-Identifier: GPL-2.0
//! Byte-preserving command line and checked in-place ELF updates.
use crate::relocs_header::Options;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const USAGE: &[u8] = b"relocs [--reloc-info|--text|--bin|--keep] vmlinux\n";

pub(crate) fn errno_text(error: &io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error ").next().unwrap_or(&text).into()
}

pub(crate) fn output(result: io::Result<()>) -> Result<(), Vec<u8>> {
    result.map_err(|error| format!("Cannot write output: {}\n", errno_text(&error)).into_bytes())
}

fn path_error(action: &str, path: &std::path::Path, reason: &str, newline: bool) -> Vec<u8> {
    let mut error = format!("Cannot {action} ").into_bytes();
    error.extend_from_slice(path.as_os_str().as_encoded_bytes());
    error.extend_from_slice(b": ");
    error.extend_from_slice(reason.as_bytes());
    if newline {
        error.push(b'\n');
    }
    error
}

fn run() -> Result<(), Vec<u8>> {
    let mut options = Options::default();
    let mut path = None;
    for argument in std::env::args_os().skip(1) {
        match argument.as_encoded_bytes() {
            b"--reloc-info" => options.info = true,
            b"--text" => options.text = true,
            b"--bin" => options.binary = true,
            b"--keep" => options.keep = true,
            bytes if !bytes.starts_with(b"-") && path.is_none() => {
                path = Some(PathBuf::from(argument))
            }
            _ => return Err(USAGE.to_vec()),
        }
    }
    let path = path.ok_or_else(|| USAGE.to_vec())?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|error| path_error("open", &path, &errno_text(&error), true))?;
    let mut ident = [0; 16];
    file.read_exact(&mut ident).map_err(|error| {
        let reason = if error.kind() == io::ErrorKind::UnexpectedEof {
            "Success".into()
        } else {
            errno_text(&error)
        };
        path_error("read", &path, &reason, false)
    })?;
    file.rewind()
        .map_err(|error| path_error("seek", &path, &errno_text(&error), true))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|error| path_error("read", &path, &errno_text(&error), true))?;
    let mut out = BufWriter::new(io::stdout().lock());
    let processed = crate::process(&data, &options, &mut out);
    let flushed = output(out.flush());
    let edits = processed?;
    flushed?;
    for edit in edits {
        file.seek(SeekFrom::Start(edit.offset))
            .and_then(|_| file.write_all(&edit.bytes))
            .map_err(|error| path_error("write", &path, &errno_text(&error), true))?;
    }
    file.flush()
        .map_err(|error| path_error("write", &path, &errno_text(&error), true))
}

pub(crate) fn main() {
    if let Err(error) = run() {
        let _ = io::stderr().write_all(&error);
        std::process::exit(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
