// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005. */

//! Device tree compiler with an owned, byte-preserving Rust implementation.

#![forbid(unsafe_code)]

mod checks;
mod data;
mod dtc_header;
mod flattree;
mod fstree;
mod lexer;
mod livetree;
mod parser;
mod srcpos;
mod treesource;
mod util;
mod version_gen_header;

use dtc_header::*;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

struct CliOption {
    short: u8,
    long: &'static str,
    argument: bool,
    help: &'static str,
}

const CLI: &[CliOption] = &[
    CliOption { short: b'q', long: "quiet", argument: false, help: "\n\tQuiet: -q suppress warnings, -qq errors, -qqq all" },
    CliOption { short: b'I', long: "in-format", argument: true, help: "\n\tInput formats are:\n\t\tdts - device tree source text\n\t\tdtb - device tree blob\n\t\tfs  - /proc/device-tree style directory" },
    CliOption { short: b'o', long: "out", argument: true, help: "\n\tOutput file" },
    CliOption { short: b'O', long: "out-format", argument: true, help: "\n\tOutput formats are:\n\t\tdts - device tree source text\n\t\tdtb - device tree blob\n\t\tasm - assembler source" },
    CliOption { short: b'V', long: "out-version", argument: true, help: "\n\tBlob version to produce, defaults to 17 (for dtb and asm output)" },
    CliOption { short: b'd', long: "out-dependency", argument: true, help: "\n\tOutput dependency file" },
    CliOption { short: b'R', long: "reserve", argument: true, help: "\n\tMake space for <number> reserve map entries (for dtb and asm output)" },
    CliOption { short: b'S', long: "space", argument: true, help: "\n\tMake the blob at least <bytes> long (extra space)" },
    CliOption { short: b'p', long: "pad", argument: true, help: "\n\tAdd padding to the blob of <bytes> long (extra space)" },
    CliOption { short: b'a', long: "align", argument: true, help: "\n\tMake the blob align to the <bytes> (extra space)" },
    CliOption { short: b'b', long: "boot-cpu", argument: true, help: "\n\tSet the physical boot cpu" },
    CliOption { short: b'f', long: "force", argument: false, help: "\n\tTry to produce output even if the input tree has errors" },
    CliOption { short: b'i', long: "include", argument: true, help: "\n\tAdd a path to search for include files" },
    CliOption { short: b's', long: "sort", argument: false, help: "\n\tSort nodes and properties before outputting (useful for comparing trees)" },
    CliOption { short: b'H', long: "phandle", argument: true, help: "\n\tValid phandle formats are:\n\t\tlegacy - \"linux,phandle\" properties only\n\t\tepapr  - \"phandle\" properties only\n\t\tboth   - Both \"linux,phandle\" and \"phandle\" properties" },
    CliOption { short: b'W', long: "warning", argument: true, help: "\n\tEnable/disable warnings (prefix with \"no-\")" },
    CliOption { short: b'E', long: "error", argument: true, help: "\n\tEnable/disable errors (prefix with \"no-\")" },
    CliOption { short: b'@', long: "symbols", argument: false, help: "\n\tEnable generation of symbols" },
    CliOption { short: b'L', long: "local-fixups", argument: false, help: "\n\tPossibly generates a __local_fixups__ and a __fixups__ node at the root node" },
    CliOption { short: b'A', long: "auto-alias", argument: false, help: "\n\tEnable auto-alias of labels" },
    CliOption { short: b'T', long: "annotate", argument: false, help: "\n\tAnnotate output .dts with input source file and line (-T -T for more details)" },
    CliOption { short: b'h', long: "help", argument: false, help: "\n\tPrint this help and exit" },
    CliOption { short: b'v', long: "version", argument: false, help: "\n\tPrint version and exit" },
];

fn usage(error: Option<&str>, diagnostics: &mut Diagnostics) -> i32 {
    let mut output =
        b"Usage: dtc [options] <input file>\n\nOptions: -[qI:O:o:V:d:R:S:p:a:fb:i:H:sW:E:@LAThv]\n"
            .to_vec();
    let width = CLI
        .iter()
        .map(|opt| opt.long.len() + 1 + if opt.argument { 6 } else { 0 })
        .max()
        .unwrap();
    for opt in CLI {
        let option = if opt.argument {
            format!("{} <arg>", opt.long)
        } else {
            opt.long.to_owned()
        };
        output.extend_from_slice(
            format!("  -{}, --{option:width$}{}\n", opt.short as char, opt.help).as_bytes(),
        );
    }
    if let Some(error) = error {
        output.extend_from_slice(format!("\nError: {error}\n").as_bytes());
        diagnostics.raw(output);
        1
    } else {
        let _ = io::stdout().write_all(&output);
        0
    }
}

fn fatal(diagnostics: &mut Diagnostics, message: impl AsRef<[u8]>) -> i32 {
    diagnostics.raw(b"FATAL ERROR: ");
    diagnostics.raw(message);
    1
}

fn guess_name(name: &[u8]) -> Option<&'static [u8]> {
    let index = name.iter().rposition(|&b| b == b'.')?;
    let extension = &name[index..];
    if extension.eq_ignore_ascii_case(b".dts") {
        Some(b"dts")
    } else if extension.eq_ignore_ascii_case(b".yaml") {
        Some(b"yaml")
    } else if extension.eq_ignore_ascii_case(b".dtb") || extension.eq_ignore_ascii_case(b".dtbo") {
        Some(b"dtb")
    } else {
        None
    }
}

fn guess_input(name: &[u8]) -> &'static [u8] {
    let path = Path::new(OsStr::from_bytes(name));
    let Ok(metadata) = fs::metadata(path) else {
        return b"dts";
    };
    if metadata.is_dir() {
        return b"fs";
    }
    if !metadata.is_file() {
        return b"dts";
    }
    let mut magic = [0; 4];
    if File::open(path)
        .and_then(|mut file| file.read_exact(&mut magic))
        .is_err()
    {
        return b"dts";
    }
    if magic == 0xd00dfeedu32.to_be_bytes() {
        b"dtb"
    } else {
        guess_name(name).unwrap_or(b"dts")
    }
}

fn run(diagnostics: &mut Diagnostics) -> i32 {
    let args: Vec<Vec<u8>> = std::env::args_os().map(|a| a.as_bytes().to_vec()).collect();
    let program = args.first().map(Vec::as_slice).unwrap_or(b"dtc");
    let mut options = Options::default();
    let mut inform = None;
    let mut outform = None;
    let mut outname = b"-".to_vec();
    let mut depname = None;
    let mut boot_cpuid = -1i64;
    let mut operands = Vec::new();
    let mut index = 1;
    let mut ended = false;
    let posix = std::env::var_os("POSIXLY_CORRECT").is_some();
    while index < args.len() {
        let arg = &args[index];
        index += 1;
        if ended || !arg.starts_with(b"-") || arg == b"-" {
            operands.push(arg.clone());
            ended |= posix;
            continue;
        }
        if arg == b"--" {
            ended = true;
            continue;
        }
        let mut short_index = 1;
        loop {
            let mut value = None;
            let opt;
            if arg.starts_with(b"--") {
                let end = arg.iter().position(|&b| b == b'=').unwrap_or(arg.len());
                let name = &arg[2..end];
                let matching: Vec<_> = CLI
                    .iter()
                    .filter(|o| o.long.as_bytes().starts_with(name))
                    .collect();
                let exact = matching.iter().copied().find(|o| o.long.as_bytes() == name);
                if exact.is_none() && matching.len() != 1 {
                    diagnostics.raw(program);
                    if matching.is_empty() {
                        diagnostics.raw(format!(": unrecognized option '{}'\n", display(arg)));
                    } else {
                        diagnostics.raw(format!(
                            ": option '{}' is ambiguous; possibilities:",
                            display(arg)
                        ));
                        for entry in matching {
                            diagnostics.raw(format!(" '--{}'", entry.long));
                        }
                        diagnostics.raw(b"\n");
                    }
                    return usage(Some("unknown option"), diagnostics);
                }
                opt = exact.unwrap_or(matching[0]);
                if end < arg.len() {
                    if !opt.argument {
                        diagnostics.raw(program);
                        diagnostics.raw(format!(
                            ": option '{}' doesn't allow an argument\n",
                            display(&arg[..end])
                        ));
                        return usage(Some("unknown option"), diagnostics);
                    }
                    value = Some(arg[end + 1..].to_vec());
                }
                short_index = arg.len();
            } else {
                let code = arg[short_index];
                short_index += 1;
                let Some(found) = CLI.iter().find(|o| o.short == code) else {
                    diagnostics.raw(program);
                    diagnostics.raw(format!(": invalid option -- '{}'\n", code as char));
                    return usage(Some("unknown option"), diagnostics);
                };
                opt = found;
                if opt.argument && short_index < arg.len() {
                    value = Some(arg[short_index..].to_vec());
                    short_index = arg.len();
                }
            }
            if opt.argument && value.is_none() {
                if index == args.len() {
                    diagnostics.raw(program);
                    if arg.starts_with(b"--") {
                        diagnostics
                            .raw(format!(": option '--{}' requires an argument\n", opt.long));
                    } else {
                        diagnostics.raw(format!(
                            ": option requires an argument -- '{}'\n",
                            opt.short as char
                        ));
                    }
                    return usage(Some("unknown option"), diagnostics);
                }
                value = Some(args[index].clone());
                index += 1;
            }
            let value = value.unwrap_or_default();
            match opt.short {
                b'I' => inform = Some(value),
                b'O' => outform = Some(value),
                b'o' => outname = value,
                b'd' => depname = Some(value),
                b'V' => options.version = util::strtol(&value) as i32,
                b'R' => options.reservenum = util::integer(&value).0 as u32,
                b'S' => options.minsize = util::strtol(&value) as i32,
                b'p' => options.padsize = util::strtol(&value) as i32,
                b'a' => {
                    options.alignsize = util::strtol(&value) as i32;
                    if options.alignsize <= 0 || !(options.alignsize as u32).is_power_of_two() {
                        return fatal(
                            diagnostics,
                            format!("Invalid argument \"{}\" to -a option\n", options.alignsize),
                        );
                    }
                }
                b'f' => options.force = true,
                b'q' => options.quiet += 1,
                b'b' => boot_cpuid = util::strtol(&value),
                b'i' => options.include_paths.push(value),
                b'v' => {
                    let _ = io::stdout().write_all(
                        format!("Version: {}\n", version_gen_header::DTC_VERSION).as_bytes(),
                    );
                    return 0;
                }
                b'H' => {
                    options.phandle_format = match value.as_slice() {
                        b"legacy" => PHANDLE_LEGACY,
                        b"epapr" => PHANDLE_EPAPR,
                        b"both" => PHANDLE_BOTH,
                        _ => {
                            return fatal(
                                diagnostics,
                                format!("Invalid argument \"{}\" to -H option\n", display(&value)),
                            )
                        }
                    }
                }
                b's' => options.sort = true,
                b'W' | b'E' => {
                    options
                        .checks
                        .push((opt.short == b'W', opt.short == b'E', value));
                    if let Err(status) = checks::validate_check_options(&options, diagnostics) {
                        return status;
                    }
                }
                b'@' => options.generate_symbols = true,
                b'L' => options.generate_fixups = true,
                b'A' => options.auto_label_aliases = true,
                b'T' => options.annotate += 1,
                b'h' => return usage(None, diagnostics),
                _ => unreachable!(),
            }
            if short_index == arg.len() {
                break;
            }
        }
    }
    if operands.len() > 1 {
        return usage(Some("missing files"), diagnostics);
    }
    let input = operands.first().map(Vec::as_slice).unwrap_or(b"-");
    if options.minsize != 0 && options.padsize != 0 {
        return fatal(diagnostics, b"Can't set both -p and -S\n");
    }
    let mut depfile = if let Some(name) = depname {
        match File::create(Path::new(OsStr::from_bytes(&name))) {
            Ok(mut file) => {
                if let Err(error) = file
                    .write_all(&util::escaped_path(&outname))
                    .and_then(|()| file.write_all(b":"))
                {
                    return fatal(
                        diagnostics,
                        format!(
                            "Error writing dependency file: {}\n",
                            srcpos::errno_text(&error)
                        ),
                    );
                }
                Some(file)
            }
            Err(error) => {
                return fatal(
                    diagnostics,
                    format!(
                        "Couldn't open dependency file {}: {}\n",
                        display(&name),
                        srcpos::errno_text(&error)
                    ),
                )
            }
        }
    } else {
        None
    };
    let inform = inform.unwrap_or_else(|| guess_input(input).to_vec());
    let outform = outform.unwrap_or_else(|| {
        guess_name(&outname)
            .unwrap_or(if inform == b"dts" { b"dtb" } else { b"dts" })
            .to_vec()
    });
    if options.annotate != 0 && (inform != b"dts" || outform != b"dts") {
        return fatal(diagnostics, b"--annotate requires -I dts -O dts\n");
    }
    let tree = match inform.as_slice() {
        b"dts" => parser::from_source(input, &options, diagnostics),
        b"dtb" => flattree::from_blob(input, &options, diagnostics),
        b"fs" => fstree::from_fs(input, &options, diagnostics),
        _ => {
            return fatal(
                diagnostics,
                format!("Unknown input format \"{}\"\n", display(&inform)),
            )
        }
    };
    if let Some(file) = &mut depfile {
        let dependencies = match &tree {
            Ok(tree) => &tree.dependencies,
            Err(_) => &diagnostics.input_dependencies,
        };
        for dependency in dependencies {
            if let Err(error) = file
                .write_all(b" ")
                .and_then(|()| file.write_all(&util::escaped_path(dependency)))
            {
                return fatal(
                    diagnostics,
                    format!(
                        "Error writing dependency file: {}\n",
                        srcpos::errno_text(&error)
                    ),
                );
            }
        }
        if tree.is_ok() {
            if let Err(error) = file.write_all(b"\n").and_then(|()| file.flush()) {
                return fatal(
                    diagnostics,
                    format!(
                        "Error writing dependency file: {}\n",
                        srcpos::errno_text(&error)
                    ),
                );
            }
        }
    }
    let mut tree = match tree {
        Ok(tree) => tree,
        Err(error) => return fatal(diagnostics, error),
    };
    tree.outname = outname.clone();
    if boot_cpuid != -1 {
        tree.boot_cpuid_phys = boot_cpuid as u32;
    }
    tree.fill_fullpaths();
    if tree.dtsflags & DTSF_PLUGIN != 0 {
        options.generate_fixups = true;
    }
    if let Err(status) = checks::process_checks(&mut tree, &options, diagnostics) {
        return status;
    }
    if options.auto_label_aliases {
        if let Err(error) = tree.generate_label_tree(b"aliases", false, &options, diagnostics) {
            return fatal(diagnostics, error);
        }
    }
    tree.generate_labels_from_tree(b"__symbols__", &options, diagnostics);
    if options.generate_symbols {
        if let Err(error) = tree.generate_label_tree(b"__symbols__", true, &options, diagnostics) {
            return fatal(diagnostics, error);
        }
    }
    tree.fixup_phandles(b"__fixups__", &options, diagnostics);
    tree.local_fixup_phandles(b"__local_fixups__", &options, diagnostics);
    if options.generate_fixups {
        if let Err(error) = tree.generate_fixups_tree(b"__fixups__", diagnostics) {
            return fatal(diagnostics, error);
        }
        tree.generate_local_fixups_tree(b"__local_fixups__", diagnostics);
    }
    if options.sort {
        tree.sort_tree();
    }
    let mut output: Box<dyn Write> = if outname == b"-" {
        Box::new(io::stdout())
    } else {
        match File::create(Path::new(OsStr::from_bytes(&outname))) {
            Ok(file) => Box::new(file),
            Err(error) => {
                return fatal(
                    diagnostics,
                    format!(
                        "Couldn't open output file {}: {}\n",
                        display(&outname),
                        srcpos::errno_text(&error)
                    ),
                )
            }
        }
    };
    let content = match outform.as_slice() {
        b"dts" => treesource::to_source(&tree, &options, diagnostics),
        b"dtb" => flattree::to_blob(&tree, &options, diagnostics),
        b"asm" => flattree::to_asm(&tree, &options, diagnostics),
        b"null" => Ok(Vec::new()),
        _ => {
            return fatal(
                diagnostics,
                format!("Unknown output format \"{}\"\n", display(&outform)),
            )
        }
    };
    let content = match content {
        Ok(content) => content,
        Err(error) => return fatal(diagnostics, error),
    };
    if let Err(error) = output.write_all(&content).and_then(|()| output.flush()) {
        return fatal(
            diagnostics,
            format!("Error writing output: {}\n", srcpos::errno_text(&error)),
        );
    }
    0
}

fn main() {
    let mut diagnostics = Diagnostics::default();
    let status = run(&mut diagnostics);
    let _ = io::stderr().write_all(&diagnostics.bytes);
    std::process::exit(status);
}
