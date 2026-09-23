// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Generate exported-symbol versions from DWARF debugging information.

mod cache;
mod die;
mod dwarf;
// The checked ELF reader is shared with tools that use its other operations.
#[allow(dead_code)]
#[path = "../elf-parse.rs"]
mod elf;
mod gendwarfksyms_header;
mod kabi;
mod reader;
mod symbols;
mod types;

use gendwarfksyms_header::{bytes, error, io_error, Diagnostics, Error, Options, Result};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

const USAGE: &[u8] = concat!(
    "Usage: gendwarfksyms [options] elf-object-file ... < symbol-list\n\n",
    "Options:\n",
    "  -d, --debug          Print debugging information\n",
    "      --dump-dies      Dump DWARF DIE contents\n",
    "      --dump-die-map   Print debugging information about die_map changes\n",
    "      --dump-types     Dump type strings\n",
    "      --dump-versions  Dump expanded type strings used for symbol versions\n",
    "  -s, --stable         Support kABI stability features\n",
    "  -T, --symtypes file  Write a symtypes file\n",
    "  -h, --help           Print this message\n\n",
)
.as_bytes();

struct Arguments {
    options: Options,
    symtypes_file: Option<Vec<u8>>,
    files: Vec<Vec<u8>>,
}

/// GNU getopt_long ordering, abbreviation, and diagnostics, on unmodified bytes.
fn arguments(argv: &[Vec<u8>], posix: bool) -> Result<Option<Arguments>> {
    const LONG: &[&[u8]] = &[
        b"debug",
        b"dump-dies",
        b"dump-die-map",
        b"dump-types",
        b"dump-versions",
        b"stable",
        b"symtypes",
        b"help",
    ];
    let program = argv.first().map(Vec::as_slice).unwrap_or_default();
    let invalid = |parts: &[&[u8]]| Error(bytes(&[program, b": ", &bytes(parts), b"\n", USAGE]));
    let mut result = Arguments {
        options: Options::default(),
        symtypes_file: None,
        files: Vec::new(),
    };
    let mut position = 1;
    let mut stopped = false;
    while let Some(argument) = argv.get(position) {
        position += 1;
        if stopped || argument.len() < 2 || argument[0] != b'-' {
            result.files.push(argument.clone());
            stopped |= posix;
            continue;
        }
        if argument == b"--" {
            stopped = true;
            continue;
        }
        if let Some(long) = argument.strip_prefix(b"--") {
            let (name, value) = match long.iter().position(|&byte| byte == b'=') {
                Some(at) => (&long[..at], Some(&long[at + 1..])),
                None => (long, None),
            };
            let exact = LONG.iter().position(|&candidate| candidate == name);
            let candidates: Vec<_> = LONG
                .iter()
                .enumerate()
                .filter_map(|(index, candidate)| candidate.starts_with(name).then_some(index))
                .collect();
            let option = if let Some(index) = exact {
                index
            } else if candidates.len() == 1 {
                candidates[0]
            } else if candidates.is_empty() {
                return Err(invalid(&[b"unrecognized option '", argument, b"'"]));
            } else {
                let mut message =
                    bytes(&[b"option '", argument, b"' is ambiguous; possibilities:"]);
                for index in candidates {
                    message.extend_from_slice(&bytes(&[b" '--", LONG[index], b"'"]));
                }
                return Err(invalid(&[&message]));
            };
            if option != 6 && value.is_some() {
                return Err(invalid(&[
                    b"option '--",
                    LONG[option],
                    b"' doesn't allow an argument",
                ]));
            }
            match option {
                0 => result.options.debug = true,
                1 => result.options.dump_dies = true,
                2 => result.options.dump_die_map = true,
                3 => result.options.dump_types = true,
                4 => result.options.dump_versions = true,
                5 => result.options.stable = true,
                6 => {
                    let value = if let Some(value) = value {
                        value
                    } else if let Some(value) = argv.get(position) {
                        position += 1;
                        value
                    } else {
                        return Err(invalid(&[
                            b"option '--",
                            LONG[option],
                            b"' requires an argument",
                        ]));
                    };
                    result.options.symtypes = true;
                    result.symtypes_file = Some(value.to_vec());
                }
                7 => return Ok(None),
                _ => unreachable!(),
            }
        } else {
            let mut letters = argument[1..].iter().enumerate();
            while let Some((index, &letter)) = letters.next() {
                match letter {
                    b'd' => result.options.debug = true,
                    b's' => result.options.stable = true,
                    b'h' => return Ok(None),
                    b'T' => {
                        let value = if index + 2 < argument.len() {
                            &argument[index + 2..]
                        } else if let Some(value) = argv.get(position) {
                            position += 1;
                            value
                        } else {
                            return Err(invalid(&[b"option requires an argument -- 'T'"]));
                        };
                        result.options.symtypes = true;
                        result.symtypes_file = Some(value.to_vec());
                        break;
                    }
                    _ => return Err(invalid(&[b"invalid option -- '", &[letter], b"'"])),
                }
            }
        }
    }
    result.options.dump_dies |= result.options.dump_die_map;
    if result.files.is_empty() {
        return Err(Error(bytes(&[
            USAGE,
            &error("main", &[b"no input files?"]).0,
        ])));
    }
    Ok(Some(result))
}

fn file_error(operation: &[u8], path: &[u8], reason: io::Error) -> Error {
    let message = reason.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    error(
        "main",
        &[
            operation,
            b" failed for '",
            path,
            b"': ",
            message.as_bytes(),
        ],
    )
}

fn process_files(
    args: &Arguments,
    symbols: &mut symbols::Symbols,
    symfile: &mut Option<BufWriter<File>>,
    diag: &mut Diagnostics,
) -> Result<()> {
    // C's DIE-map counters span every module, although its entries do not.
    let mut dies = die::DieMap::default();
    for filename in &args.files {
        let path = Path::new(OsStr::from_bytes(filename));
        let mut file = File::open(path).map_err(|err| file_error(b"open", filename, err))?;
        let metadata = file
            .metadata()
            .map_err(|err| io_error("elf_for_each_global", err))?;
        if metadata.is_dir() {
            return Err(error(
                "elf_for_each_global",
                &[b"elf_begin failed: invalid file descriptor"],
            ));
        }
        let mut data = Vec::new();
        // libelf reads the fstat-sized image. In particular, a character
        // device such as /dev/zero must not become an unbounded input stream.
        (&mut file)
            .take(metadata.len())
            .read_to_end(&mut data)
            .map_err(|err| io_error("elf_for_each_global", err))?;
        symbols.read_symtab(&data, diag)?;
        let rules = kabi::Rules::read(&data, diag)?;
        file.rewind().map_err(|err| io_error("main", err))?;
        let session = reader::Session::open(file, path)?;
        for module in session.modules()? {
            dwarf::process_module(module, symbols, &mut dies, &rules, diag)?;
            types::generate(
                &mut dies,
                symbols,
                &rules,
                diag,
                symfile.as_mut().map(|writer| writer as &mut dyn Write),
            )?;
            dies.clear(diag);
        }
    }
    Ok(())
}

fn run(args: &Arguments, diag: &mut Diagnostics) -> Result<Vec<u8>> {
    let mut symbols = symbols::Symbols::read_exports(io::stdin().lock(), diag)?;
    if symbols.entries.is_empty() {
        return Ok(Vec::new());
    }
    let mut symfile = args
        .symtypes_file
        .as_ref()
        .map(|filename| {
            File::create(Path::new(OsStr::from_bytes(filename)))
                .map(BufWriter::new)
                .map_err(|err| file_error(b"fopen", filename, err))
        })
        .transpose()?;
    let result = process_files(args, &mut symbols, &mut symfile, diag);
    // Like exit() after C's error(), flush earlier modules' symtypes even if a
    // later input fails. The original processing error retains precedence.
    let flushed = symfile.as_mut().map_or(Ok(()), Write::flush);
    result?;
    flushed.map_err(|_| error("main", &[b"`fclose(symfile)` failed: -1"]))?;
    let mut output = Vec::new();
    symbols.print_versions(&mut output, diag)?;
    Ok(output)
}

fn main() {
    let argv: Vec<_> = std::env::args_os()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    let mut diagnostics = Diagnostics::new(Options::default());
    let result = match arguments(&argv, std::env::var_os("POSIXLY_CORRECT").is_some()) {
        Ok(Some(args)) => {
            diagnostics.options = args.options;
            run(&args, &mut diagnostics)
        }
        Ok(None) => {
            diagnostics.print(&[USAGE]);
            Ok(Vec::new())
        }
        Err(err) => Err(err),
    };
    let mut failed = false;
    let output = match result {
        Ok(output) => output,
        Err(err) => {
            diagnostics.print(&[&err.0]);
            failed = true;
            Vec::new()
        }
    };
    let mut stderr = io::stderr().lock();
    if stderr
        .write_all(&diagnostics.bytes)
        .and_then(|()| stderr.flush())
        .is_err()
    {
        failed = true;
    }
    // Diagnostics precede the final version stream, including when both
    // descriptors refer to the same output file.
    let mut stdout = io::stdout().lock();
    if let Err(err) = stdout.write_all(&output).and_then(|()| stdout.flush()) {
        let _ = stderr.write_all(&io_error("symbol_print_versions", err).0);
        let _ = stderr.flush();
        failed = true;
    }
    if failed {
        std::process::exit(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
