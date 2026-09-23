// SPDX-License-Identifier: GPL-2.0-only

//! Check that each tracepoint defined in a kernel or module is used.

#[path = "elf-parse.rs"]
mod elf;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn strings(data: &[u8]) -> Result<impl Iterator<Item = &[u8]>, String> {
    if !data.is_empty() && data.last() != Some(&0) {
        return Err("unterminated tracepoint string:".into());
    }
    Ok(data
        .split(|&byte| byte == 0)
        .filter(|name| !name.is_empty()))
}

fn check(data: &[u8], path: &Path, module: bool, output: &mut impl Write) -> Result<(), String> {
    let elf = elf::ElfFile::parse(data, 1 << 1)?;
    let used = elf.find_section(b"__tracepoint_check")?;
    let defined = elf.find_section(b"__tracepoints_strings")?;
    match (used, defined) {
        (None, None) if module => Ok(()),
        (None, _) if module => {
            output
                .write_all(b"warning: Module ")
                .map_err(|e| e.to_string())?;
            output
                .write_all(path.as_os_str().as_bytes())
                .map_err(|e| e.to_string())?;
            writeln!(output, " has only unused tracepoints").map_err(|e| e.to_string())?;
            Ok(())
        }
        (None, _) => Err("no __tracepoint_check in file:".into()),
        (Some(_), None) if module => Ok(()),
        (Some(_), None) => Err("no __tracepoint_strings in file:".into()),
        (Some(used), Some(defined)) => {
            let names: HashSet<&[u8]> = strings(elf.section_data(&used)?)?.collect();
            // An empty check section means verification was disabled.
            if names.is_empty() {
                return Ok(());
            }
            for name in strings(elf.section_data(&defined)?)? {
                if names.contains(name) {
                    continue;
                }
                output
                    .write_all(b"warning: tracepoint '")
                    .map_err(|e| e.to_string())?;
                output.write_all(name).map_err(|e| e.to_string())?;
                output
                    .write_all(b"' is unused")
                    .map_err(|e| e.to_string())?;
                if module {
                    output
                        .write_all(b" in module ")
                        .map_err(|e| e.to_string())?;
                    output
                        .write_all(path.as_os_str().as_bytes())
                        .map_err(|e| e.to_string())?;
                }
                writeln!(output).map_err(|e| e.to_string())?;
            }
            Ok(())
        }
    }
}

fn run(output: &mut impl Write) -> io::Result<bool> {
    let mut args = std::env::args_os().skip(1).peekable();
    let module = args.peek().is_some_and(|arg| arg == OsStr::new("--module"));
    if module {
        args.next();
    }
    if args.peek().is_none() {
        writeln!(
            output,
            "usage: tracepoint-update {}...",
            if module { "--module module" } else { "vmlinux" }
        )?;
        return Ok(true);
    }
    let mut success = true;
    for arg in args {
        let read = || -> io::Result<Vec<u8>> {
            let mut file = File::open(&arg)?;
            if !file.metadata()?.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "not a regular file",
                ));
            }
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            Ok(data)
        };
        let data = match read() {
            Ok(data) => data,
            Err(error) => {
                let message = error.to_string();
                let message = message.split(" (os error ").next().unwrap_or(&message);
                if error.raw_os_error().is_some() {
                    output.write_all(arg.as_bytes())?;
                    writeln!(output, ": {message}")?;
                } else {
                    write!(output, "{message}: ")?;
                    output.write_all(arg.as_bytes())?;
                    writeln!(output)?;
                }
                success = false;
                continue;
            }
        };
        if let Err(error) = check(&data, Path::new(&arg), module, output) {
            write!(output, "{error} ")?;
            output.write_all(arg.as_bytes())?;
            writeln!(output)?;
            success = false;
        }
    }
    Ok(success)
}

fn main() {
    let stderr = io::stderr();
    let mut output = stderr.lock();
    if !run(&mut output).unwrap_or(false) {
        std::process::exit(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
