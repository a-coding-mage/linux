// SPDX-License-Identifier: GPL-2.0-only

//! Record ftrace call sites and remove instrumentation from excluded sections.
// Copyright 2009 John F. Reiser <jreiser@BitWagon.com>
// Copyright 2010 Steven Rostedt <srostedt@redhat.com>, Red Hat Inc.

#[path = "elf-parse.rs"]
mod elf;
mod recordmcount_header;

use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;

fn diagnostic(prefix: &[u8], error: &io::Error) {
    let text = error.to_string();
    let text = text.split(" (os error ").next().unwrap_or(&text);
    let mut stderr = io::stderr().lock();
    let _ = stderr.write_all(prefix);
    let _ = writeln!(stderr, ": {text}");
}

fn file(path: &OsStr, warn: bool) -> bool {
    let mut input = match File::open(path) {
        Ok(input) => input,
        Err(error) => {
            diagnostic(path.as_bytes(), &error);
            return false;
        }
    };
    let metadata = match input.metadata() {
        Ok(metadata) => metadata,
        Err(error) => {
            diagnostic(path.as_bytes(), &error);
            return false;
        }
    };
    if !metadata.is_file() {
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(b"not a regular file: ");
        let _ = stderr.write_all(path.as_bytes());
        let _ = stderr.write_all(b"\n");
        return false;
    }
    let mut data = Vec::new();
    if let Err(error) = input.read_to_end(&mut data) {
        diagnostic(path.as_bytes(), &error);
        return false;
    }
    drop(input);
    let result = recordmcount_header::process(&data, path.as_bytes(), warn);
    if let Err(error) = io::stdout().lock().write_all(&result.stdout) {
        diagnostic(b"write", &error);
        return false;
    }
    let _ = io::stderr().lock().write_all(&result.stderr);
    if result.failed {
        return false;
    }
    let Some(output) = result.output else {
        return true;
    };
    let mut temporary = OsString::from(path);
    temporary.push(".rc");
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(metadata.mode())
        .open(&temporary)
    {
        Ok(file) => file,
        Err(error) => {
            diagnostic(path.as_bytes(), &error);
            return false;
        }
    };
    if let Err(error) = file.write_all(&output) {
        diagnostic(b"write", &error);
        return false;
    }
    drop(file);
    if let Err(error) = fs::rename(Path::new(&temporary), Path::new(path)) {
        diagnostic(path.as_bytes(), &error);
        return false;
    }
    true
}

fn main() {
    const USAGE: &[u8] = b"usage: recordmcount [-w] file.o...\n";
    let mut args = std::env::args_os();
    let program = args.next().unwrap_or_else(|| "recordmcount".into());
    let mut warn = false;
    let mut stopped = false;
    let mut files = Vec::new();
    let posix = std::env::var_os("POSIXLY_CORRECT").is_some();
    for argument in args {
        let bytes = argument.as_bytes();
        if stopped || bytes.first() != Some(&b'-') || bytes == b"-" {
            files.push(argument);
            stopped |= posix;
        } else if bytes == b"--" {
            stopped = true;
        } else {
            for &option in &bytes[1..] {
                if option == b'w' {
                    warn = true;
                } else {
                    let mut stderr = io::stderr().lock();
                    let _ = stderr.write_all(program.as_bytes());
                    let _ = stderr.write_all(b": invalid option -- '");
                    let _ = stderr.write_all(&[option]);
                    let _ = stderr.write_all(b"'\n");
                    let _ = stderr.write_all(USAGE);
                    return;
                }
            }
        }
    }
    if files.is_empty() {
        let _ = io::stderr().lock().write_all(USAGE);
        return;
    }
    let mut failed = false;
    for path in files {
        if path.as_bytes().ends_with(b"/ftrace.o") {
            continue;
        }
        if !file(&path, warn) {
            let mut stderr = io::stderr().lock();
            let _ = stderr.write_all(path.as_bytes());
            let _ = stderr.write_all(b": failed\n");
            failed = true;
        }
    }
    if failed {
        std::process::exit(1);
    }
}
