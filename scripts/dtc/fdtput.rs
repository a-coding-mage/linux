// SPDX-License-Identifier: GPL-2.0-or-later
//! Write properties or create nodes in an existing flattened device tree.
// Copyright (c) 2011 The Chromium OS Authors. All rights reserved.
mod libfdt;
use libfdt::tools::{self, Arg, Output};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
const USAGE:&str="fdtput - write a property value to a device tree\n\nThe command line arguments are joined together into a single value.\n\nUsage:\n\tfdtput <options> <dt file> <node> <property> [<value>...]\n\tfdtput -c <options> <dt file> [<node>...]\nOptions:\n\t-c\t\tCreate nodes if they don't already exist\n\t-p\t\tAutomatically create nodes as needed for the node path\n\t-t <type>\tType of data\n\t-v\t\tVerbose: display each value decoded from command line\n\t-h\t\tPrint this help\n\n";
fn usage(out: &mut Output, message: Option<&str>) -> i32 {
    if let Some(message) = message {
        out.err
            .extend_from_slice(format!("Error: {message}\n\n").as_bytes());
    }
    out.err.extend_from_slice(USAGE.as_bytes());
    out.err.extend_from_slice(tools::TYPE_USAGE.as_bytes());
    2
}
fn create_paths(out: &mut Output, blob: &mut [u8], path: &[u8]) -> bool {
    let mut path = path;
    while path.first() == Some(&b'/') {
        path = &path[1..];
    }
    let mut parent = 0;
    if path.is_empty() {
        return true;
    }
    for name in path.split(|&b| b == b'/') {
        let result = match libfdt::subnode_offset(blob, parent, name) {
            Err(libfdt::Error::NotFound) => libfdt::add_subnode(blob, parent, name),
            other => other,
        };
        match result {
            Ok(node) => parent = node,
            Err(err) => {
                out.report(name, err);
                return false;
            }
        }
    }
    true
}
fn create_node(out: &mut Output, blob: &mut [u8], path: &[u8]) -> bool {
    let Some(split) = path.iter().rposition(|&b| b == b'/') else {
        out.report(path, libfdt::Error::BadPath);
        return false;
    };
    let parent = if split == 0 {
        0
    } else {
        match libfdt::path_offset(blob, &path[..split]) {
            Ok(node) => node,
            Err(err) => {
                out.report(&path[..split], err);
                return false;
            }
        }
    };
    match libfdt::add_subnode(blob, parent, &path[split + 1..]) {
        Ok(_) => true,
        Err(err) => {
            out.report(&path[split + 1..], err);
            false
        }
    }
}
fn encode(
    out: &mut Output,
    args: &[OsString],
    kind: u8,
    size: Option<usize>,
    verbose: bool,
) -> Option<Vec<u8>> {
    let mut value = Vec::new();
    if verbose {
        out.err.extend_from_slice(b"Decoding value:\n");
    }
    for arg in args {
        if kind == b's' || kind == b'r' {
            value.extend_from_slice(arg.as_bytes());
            if kind == b's' {
                value.push(0);
            }
            if verbose {
                out.err.extend_from_slice(b"\tstring: '");
                out.err.extend_from_slice(arg.as_bytes());
                out.err.extend_from_slice(b"'\n");
            }
        } else {
            let Some(number) = tools::integer(arg.as_bytes(), kind) else {
                out.err.extend_from_slice(b"Invalid integer: '");
                out.err.extend_from_slice(arg.as_bytes());
                out.err.extend_from_slice(b"'\n");
                return None;
            };
            let width = size.unwrap_or(4);
            let bytes = number.to_be_bytes();
            value.extend_from_slice(&bytes[4 - width..]);
            if verbose {
                out.err.extend_from_slice(
                    format!(
                        "\t{}: {}\n",
                        match size {
                            Some(1) => "byte",
                            Some(2) => "short",
                            _ => "int",
                        },
                        number as i32
                    )
                    .as_bytes(),
                );
            }
        }
    }
    if verbose {
        out.err
            .extend_from_slice(format!("Value size {}\n", value.len()).as_bytes());
    }
    Some(value)
}
fn run(out: &mut Output) -> i32 {
    let argv: Vec<OsString> = std::env::args_os().collect();
    let mut args = Vec::new();
    let (mut kind, mut size) = (0, None);
    let (mut create, mut auto_path, mut verbose) = (false, false, false);
    for arg in tools::arguments(&argv, b"chpt:v", &[]) {
        match arg {
            Arg::Operand(value) => args.push(value),
            Arg::Error(message) => {
                out.err.extend_from_slice(&message);
                return usage(out, None);
            }
            Arg::Option(b'h', _) => return usage(out, None),
            Arg::Option(b'c', _) => create = true,
            Arg::Option(b'p', _) => auto_path = true,
            Arg::Option(b'v', _) => verbose = true,
            Arg::Option(b't', Some(value)) => {
                let Some((t, s)) = tools::decode_type(value.as_bytes()) else {
                    return usage(out, Some("Invalid type string"));
                };
                kind = t;
                size = s;
            }
            _ => {}
        }
    }
    if args.is_empty() {
        return usage(out, Some("Missing filename"));
    }
    if !create && args.len() < 2 {
        return usage(out, Some("Missing node"));
    }
    if !create && args.len() < 3 {
        return usage(out, Some("Missing property"));
    }
    let Some(mut blob) = out.read(&args[0]) else {
        return 1;
    };
    if create {
        for path in &args[1..] {
            let ok = if auto_path {
                create_paths(out, &mut blob, path.as_bytes())
            } else {
                create_node(out, &mut blob, path.as_bytes())
            };
            if !ok {
                return 1;
            }
        }
    } else {
        let path = args[1].as_bytes();
        if auto_path && !create_paths(out, &mut blob, path) {
            return 1;
        }
        let Some(value) = encode(out, &args[3..], kind, size, verbose) else {
            return 1;
        };
        let node = match libfdt::path_offset(&blob, path) {
            Ok(node) => node,
            Err(err) => {
                out.report(path, err);
                return 1;
            }
        };
        if let Err(err) = libfdt::setprop(&mut blob, node, args[2].as_bytes(), &value) {
            out.report(args[2].as_bytes(), err);
            return 1;
        }
    }
    if out.write(&args[0], &blob) {
        0
    } else {
        1
    }
}
fn main() {
    let mut out = Output::default();
    let status = run(&mut out);
    out.finish(status);
}
