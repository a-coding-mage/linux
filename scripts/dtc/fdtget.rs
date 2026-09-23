// SPDX-License-Identifier: GPL-2.0-or-later
//! Read properties and list nodes in a flattened device tree.
// Copyright (c) 2011 The Chromium OS Authors. All rights reserved.
// Portions from U-Boot cmd_fdt.c (C) Copyright 2007 Gerald Van Baren,
// Custom IDEAS; based on code by Pantelis Antoniou and Matthew McClintock.
mod libfdt;
use libfdt::tools::{self, Arg, Output};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;

const USAGE: &str = "fdtget - read values from device tree\n\nEach value is printed on a new line.\n\nUsage:\n\tfdtget <options> <dt file> [<node> <property>]...\n\tfdtget -p <options> <dt file> [<node> ]...\nOptions:\n\t-t <type>\tType of data\n\t-p\t\tList properties for each node\n\t-l\t\tList subnodes for each node\n\t-d\t\tDefault value to display when the property is missing\n\t-h\t\tPrint this help\n\n";
fn usage(out: &mut Output, message: Option<&str>) -> i32 {
    if let Some(message) = message {
        out.err
            .extend_from_slice(format!("Error: {message}\n\n").as_bytes());
    }
    out.err.extend_from_slice(USAGE.as_bytes());
    out.err.extend_from_slice(tools::TYPE_USAGE.as_bytes());
    2
}
fn show(out: &mut Output, data: &[u8], kind: u8, size: Option<usize>) -> bool {
    if data.is_empty() {
        return true;
    }
    if kind == b'r' {
        out.out.extend_from_slice(data);
        return true;
    }
    if kind == b's' || (kind == 0 && tools::printable_string(data)) {
        if data.last() != Some(&0) {
            out.err.extend_from_slice(b"Unterminated string\n");
            return false;
        }
        for (i, value) in data[..data.len() - 1].split(|&b| b == 0).enumerate() {
            if i != 0 {
                out.out.push(b' ');
            }
            out.out.extend_from_slice(value);
        }
        return true;
    }
    let size = size.unwrap_or(if data.len() % 4 == 0 { 4 } else { 1 });
    if data.len() % size != 0 {
        out.err
            .extend_from_slice(b"Property length must be a multiple of selected data size\n");
        return false;
    }
    for (i, chunk) in data.chunks_exact(size).enumerate() {
        if i != 0 {
            out.out.push(b' ');
        }
        let value = chunk.iter().fold(0u32, |v, &b| (v << 8) | b as u32);
        let text = match kind {
            b'x' => format!("{value:x}"),
            b'u' => format!("{value}"),
            _ => format!("{}", value as i32),
        };
        out.out.extend_from_slice(text.as_bytes());
    }
    true
}
fn list_subnodes(out: &mut Output, blob: &[u8], mut node: i32) -> bool {
    let mut level = 0;
    loop {
        let (tag, next) = libfdt::next_tag(blob, node);
        match tag {
            libfdt::FDT_BEGIN_NODE => {
                if level == 1 {
                    let name = libfdt::get_name(blob, node).unwrap_or(b"/* NULL pointer error */");
                    out.out
                        .extend_from_slice(if name.is_empty() { b"/" } else { name });
                    out.out.push(b'\n');
                }
                level += 1;
                if level >= 32 {
                    out.out.extend_from_slice(b"Nested too deep, aborting.\n");
                    return false;
                }
            }
            libfdt::FDT_END_NODE => {
                level -= 1;
                if level == 0 {
                    return true;
                }
            }
            libfdt::FDT_END => return false,
            libfdt::FDT_PROP => {}
            _ => {
                if level <= 1 {
                    out.out
                        .extend_from_slice(format!("Unknown tag 0x{tag:08X}\n").as_bytes());
                }
                return false;
            }
        }
        let Ok(offset) = next else {
            return false;
        };
        node = offset;
    }
}
fn run(out: &mut Output) -> i32 {
    let argv: Vec<OsString> = std::env::args_os().collect();
    let mut operands = Vec::new();
    let (mut kind, mut size) = (0, None);
    let mut mode = 0;
    let mut default = None;
    for arg in tools::arguments(&argv, b"d:hlpt:", &[]) {
        match arg {
            Arg::Operand(value) => operands.push(value),
            Arg::Error(message) => {
                out.err.extend_from_slice(&message);
                return usage(out, None);
            }
            Arg::Option(b'h', _) => return usage(out, None),
            Arg::Option(b't', Some(value)) => {
                let Some((t, s)) = tools::decode_type(value.as_bytes()) else {
                    return usage(out, Some("Invalid type string"));
                };
                kind = t;
                size = s;
            }
            Arg::Option(b'p', _) => mode = 1,
            Arg::Option(b'l', _) => mode = 2,
            Arg::Option(b'd', value) => default = value,
            _ => {}
        }
    }
    if operands.is_empty() {
        return usage(out, Some("Missing filename"));
    }
    let step = if mode == 0 { 2 } else { 1 };
    if operands.len() == 1 {
        return 0;
    }
    if (operands.len() - 1) % step != 0 {
        return usage(out, Some("Must have an even number of arguments"));
    }
    let Some(blob) = out.read(&operands[0]) else {
        return 1;
    };
    for args in operands[1..].chunks_exact(step) {
        let node = match libfdt::path_offset(&blob, args[0].as_bytes()) {
            Ok(node) => node,
            Err(err) => {
                if let Some(value) = &default {
                    out.out.extend_from_slice(value.as_bytes());
                    out.out.push(b'\n');
                    continue;
                }
                out.report(args[0].as_bytes(), err);
                return 1;
            }
        };
        match mode {
            1 => {
                let mut prop = libfdt::first_property_offset(&blob, node);
                while let Ok(off) = prop {
                    let Ok(property) = libfdt::get_property_by_offset(&blob, off) else {
                        return 1;
                    };
                    if let Ok(name) = libfdt::get_string(&blob, property.name_offset) {
                        out.out.extend_from_slice(name);
                        out.out.push(b'\n');
                    }
                    prop = libfdt::next_property_offset(&blob, off);
                }
                if prop != Err(libfdt::Error::NotFound) {
                    return 1;
                }
            }
            2 => {
                if !list_subnodes(out, &blob, node) {
                    return 1;
                }
            }
            _ => match libfdt::getprop(&blob, node, args[1].as_bytes()) {
                Ok(value) => {
                    if !show(out, value, kind, size) {
                        return 1;
                    }
                    out.out.push(b'\n');
                }
                Err(err) => {
                    if let Some(value) = &default {
                        out.out.extend_from_slice(value.as_bytes());
                        out.out.push(b'\n');
                    } else {
                        out.report(args[1].as_bytes(), err);
                        return 1;
                    }
                }
            },
        }
    }
    0
}
fn main() {
    let mut out = Output::default();
    let status = run(&mut out);
    out.finish(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
