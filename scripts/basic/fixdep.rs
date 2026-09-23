// SPDX-License-Identifier: GPL-2.0-only
//! Turn compiler dependency files into Kbuild dependencies and CONFIG symbol checks.
//!
//! Based on fixdep.c by Kai Germaschewski, copyright 2002.

#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

struct Failure {
    code: u8,
    message: Vec<u8>,
}

impl Failure {
    fn new(code: u8, message: &[u8]) -> Self {
        Self {
            code,
            message: message.to_vec(),
        }
    }

    fn io(prefix: &[u8], path: Option<&[u8]>, error: io::Error) -> Self {
        let mut message = prefix.to_vec();
        if let Some(path) = path {
            message.extend_from_slice(path);
        }
        message.extend_from_slice(b": ");
        // perror() does not append Rust's numeric errno annotation.
        let description = error.to_string();
        let suffix = error
            .raw_os_error()
            .map(|code| format!(" (os error {code})"));
        let description = suffix
            .as_deref()
            .and_then(|suffix| description.strip_suffix(suffix))
            .unwrap_or(&description);
        message.extend_from_slice(description.as_bytes());
        message.push(b'\n');
        Self { code: 2, message }
    }
}

/// Retain output errors until parsing completes, matching stdio's error flag.
struct Output<W> {
    writer: W,
    failed: bool,
}

impl<W: Write> Output<W> {
    fn emit(&mut self, parts: &[&[u8]]) {
        for part in parts {
            if self.writer.write_all(part).is_err() {
                self.failed = true;
            }
        }
    }

    fn flush(&mut self) {
        if self.writer.flush().is_err() {
            self.failed = true;
        }
    }
}

fn read_file(path: &[u8]) -> Result<Vec<u8>, Failure> {
    let mut file = File::open(OsStr::from_bytes(path))
        .map_err(|error| Failure::io(b"fixdep: error opening file: ", Some(path), error))?;
    let metadata = file
        .metadata()
        .map_err(|error| Failure::io(b"fixdep: error fstat'ing file: ", Some(path), error))?;
    // Like the C implementation, read the size reported by fstat, including
    // zero bytes for pseudo-files whose reported size is zero.
    let size = usize::try_from(metadata.len()).map_err(|_| Failure::new(1, b""))?;
    let mut contents = Vec::new();
    contents
        .try_reserve_exact(size)
        .map_err(|_| Failure::new(1, b""))?;
    contents.resize(size, 0);
    let count = file
        .read(&mut contents)
        .map_err(|error| Failure::io(b"fixdep: read", None, error))?;
    if count != size {
        return Err(Failure::new(2, b"fixdep: read: Success\n"));
    }
    // The original parser treats files as NUL-terminated byte strings. In
    // particular, CONFIG_ strings after a NUL in a binary input are ignored.
    if let Some(end) = contents.iter().position(|&byte| byte == 0) {
        contents.truncate(end);
    }
    Ok(contents)
}

fn is_symbol_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn parse_config_file<W: Write>(
    contents: &[u8],
    configs: &mut HashSet<Vec<u8>>,
    output: &mut Output<W>,
) {
    let mut position = 0;
    while let Some(offset) = contents[position..]
        .windows(7)
        .position(|word| word == b"CONFIG_")
    {
        let start = position + offset;
        position = start + 7;
        if start > 0 && is_symbol_char(contents[start - 1]) {
            continue;
        }
        let mut end = position;
        while end < contents.len() && is_symbol_char(contents[end]) {
            end += 1;
        }
        let name = &contents[position..end];
        let name = name.strip_suffix(b"_MODULE").unwrap_or(name);
        if !name.is_empty() && configs.insert(name.to_vec()) {
            output.emit(&[b"    $(wildcard include/config/", name, b") \\\n"]);
        }
        position = end;
    }
}

/// Read a token, removing only the backslashes used to quote '#' and ':'.
/// Backslashes quoting whitespace or other backslashes remain in the filename,
/// as in fixdep.c. A backslash-newline ends a token without consuming the pair.
fn token(contents: &[u8], position: &mut usize) -> Vec<u8> {
    let mut name = Vec::new();
    while let Some(&byte) = contents.get(*position) {
        if matches!(byte, b' ' | b'\t' | b'\n' | b'#' | b':') {
            break;
        }
        if byte == b'\\' {
            match contents.get(*position + 1).copied() {
                Some(b'\n') => break,
                Some(next) => {
                    if !matches!(next, b'#' | b':') {
                        name.push(byte);
                    }
                    name.push(next);
                    *position += 2;
                    continue;
                }
                None => {}
            }
        }
        name.push(byte);
        *position += 1;
    }
    name
}

fn parse_dep_file<W: Write>(
    contents: &[u8],
    target: &[u8],
    output: &mut Output<W>,
) -> Result<(), Failure> {
    let mut files = HashSet::new();
    let mut configs = HashSet::new();
    let mut position = 0;
    let mut saw_any_target = false;
    let mut is_target = true;
    let mut is_source = false;

    while let Some(&byte) = contents.get(position) {
        match byte {
            b'#' => {
                position += 1;
                while position < contents.len() && contents[position] != b'\n' {
                    if contents[position] == b'\\' && position + 1 < contents.len() {
                        position += 1;
                    }
                    position += 1;
                }
                continue;
            }
            b' ' | b'\t' => {
                position += 1;
                continue;
            }
            b'\\' if contents.get(position + 1) == Some(&b'\n') => {
                position += 2;
                continue;
            }
            b'\n' => {
                position += 1;
                is_target = true;
                continue;
            }
            b':' => {
                position += 1;
                is_target = false;
                is_source = true;
                continue;
            }
            _ => {}
        }

        let name = token(contents, &mut position);
        if is_target {
            continue;
        }
        let need_parse = if is_source {
            if saw_any_target {
                false
            } else {
                saw_any_target = true;
                output.emit(&[
                    b"source_",
                    target,
                    b" := ",
                    &name,
                    b"\n\ndeps_",
                    target,
                    b" := \\\n",
                ]);
                true
            }
        } else if !name.ends_with(b"include/generated/autoconf.h") && files.insert(name.clone()) {
            output.emit(&[b"  ", &name, b" \\\n"]);
            true
        } else {
            false
        };

        if need_parse
            && ![b".rlib".as_slice(), b".rmeta", b".so"]
                .iter()
                .any(|suffix| name.ends_with(suffix))
        {
            parse_config_file(&read_file(&name)?, &mut configs, output);
        }
        is_source = false;
    }

    if !saw_any_target {
        return Err(Failure::new(1, b"fixdep: parse error; no targets found\n"));
    }
    output.emit(&[
        b"\n",
        target,
        b": $(deps_",
        target,
        b")\n\n$(deps_",
        target,
        b"):\n",
    ]);
    Ok(())
}

fn run<W: Write>(args: &[OsString], output: &mut Output<W>) -> Result<(), Failure> {
    if args.len() != 4 {
        return Err(Failure::new(
            1,
            b"Usage: fixdep <depfile> <target> <cmdline>\n",
        ));
    }
    let target = args[2].as_bytes();
    output.emit(&[b"savedcmd_", target, b" := ", args[3].as_bytes(), b"\n\n"]);
    parse_dep_file(&read_file(args[1].as_bytes())?, target, output)
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().collect();
    let stdout = io::stdout();
    let mut output = Output {
        writer: BufWriter::new(stdout.lock()),
        failed: false,
    };
    let result = run(&args, &mut output);
    output.flush();
    let failure = match result {
        Err(failure) => Some(failure),
        Ok(()) if output.failed => Some(Failure::new(
            1,
            b"fixdep: not all data was written to the output\n",
        )),
        Ok(()) => None,
    };
    if let Some(failure) = failure {
        let _ = io::stderr().lock().write_all(&failure.message);
        ExitCode::from(failure.code)
    } else {
        ExitCode::SUCCESS
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
