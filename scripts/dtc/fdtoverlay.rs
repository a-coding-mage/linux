// SPDX-License-Identifier: GPL-2.0-or-later
//! Apply one or more device-tree overlays with automatic buffer growth.
// Copyright (c) 2017 Konsulko Group Inc. All rights reserved.
// Author: Pantelis Antoniou <pantelis.antoniou@konsulko.com>
mod libfdt;
mod version_gen_header;
use libfdt::tools::{self, Arg, Output};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
fn usage(out: &mut Output, message: Option<&str>) -> i32 {
    let text=b"Usage: apply a number of overlays to a base blob\n\tfdtoverlay <options> [<overlay.dtbo> [<overlay.dtbo>]]\n\nOptions: -[i:o:vhV]\n  -i, --input <arg>  Input base DT blob\n  -o, --output <arg> Output DT blob\n  -v, --verbose      Verbose messages\n  -h, --help         Print this help and exit\n  -V, --version      Print version and exit\n";
    if let Some(message) = message {
        out.err.extend_from_slice(text);
        out.err
            .extend_from_slice(format!("\nError: {message}\n").as_bytes());
        1
    } else {
        out.out.extend_from_slice(text);
        0
    }
}
fn failed(out: &mut Output, action: &[u8], path: &OsStr) {
    out.err.extend_from_slice(b"\nFailed to ");
    out.err.extend_from_slice(action);
    out.err.extend_from_slice(b" '");
    out.err.extend_from_slice(path.as_bytes());
    out.err.extend_from_slice(b"'\n");
}
fn read(out: &mut Output, path: &OsStr, base: bool) -> Option<Vec<u8>> {
    let Some(blob) = out.read(path) else {
        failed(out, b"read", path);
        return None;
    };
    // C utilfdt_read accidentally reports allocation size, not bytes read.
    // Use the actual input length so truncated files cannot expose heap bytes.
    let total = blob
        .get(4..8)
        .map(|v| u32::from_be_bytes(v.try_into().unwrap()) as usize);
    if total.is_none_or(|size| size > blob.len()) {
        if base {
            out.err.extend_from_slice(b"\nBase blob is incomplete (");
        } else {
            out.err.extend_from_slice(b"\nOverlay '");
            out.err.extend_from_slice(path.as_bytes());
            out.err.extend_from_slice(b"' is incomplete (");
        }
        out.err.extend_from_slice(
            format!("{} / {} bytes read)\n", blob.len(), total.unwrap_or(8)).as_bytes(),
        );
        return None;
    }
    Some(blob)
}
fn apply(
    out: &mut Output,
    base: &[u8],
    overlay: &[u8],
    capacity: &mut usize,
    name: &OsStr,
) -> Option<Vec<u8>> {
    let total = u32::from_be_bytes(overlay.get(4..8)?.try_into().ok()?) as usize;
    let mut temp = Vec::new();
    loop {
        if *capacity > i32::MAX as usize
            || temp
                .try_reserve(capacity.saturating_sub(temp.len()))
                .is_err()
        {
            out.err.extend_from_slice(b"FATAL ERROR: Out of memory\n");
            return None;
        }
        temp.resize(*capacity, 0);
        if let Err(err) = libfdt::open_into(base, &mut temp) {
            out.err
                .extend_from_slice(format!("\nFailed to make temporary copy: {err}\n").as_bytes());
            return None;
        }
        let has_symbols = libfdt::path_offset(&temp, b"/__symbols__").is_ok();
        let mut overlay = overlay[..total].to_vec();
        match libfdt::overlay_apply(&mut temp, &mut overlay) {
            Ok(()) => return Some(temp),
            Err(libfdt::Error::NoSpace) => {
                *capacity = capacity.checked_add(65536)?;
            }
            Err(err) => {
                out.err.extend_from_slice(b"\nFailed to apply '");
                out.err.extend_from_slice(name.as_bytes());
                out.err.extend_from_slice(format!("': {err}\n").as_bytes());
                if !has_symbols {
                    out.err.extend_from_slice(b"base blob does not have a '/__symbols__' node, make sure you have compiled the base blob with '-@' option\n");
                }
                return None;
            }
        }
    }
}
fn run(out: &mut Output) -> i32 {
    let argv: Vec<OsString> = std::env::args_os().collect();
    let (mut input, mut output) = (None, None);
    let mut verbose = false;
    let mut overlays = Vec::new();
    let longs: [(&[u8], u8, bool); 5] = [
        (b"input", b'i', true),
        (b"output", b'o', true),
        (b"verbose", b'v', false),
        (b"help", b'h', false),
        (b"version", b'V', false),
    ];
    for arg in tools::arguments(&argv, b"i:o:vhV", &longs) {
        match arg {
            Arg::Operand(value) => overlays.push(value),
            Arg::Error(message) => {
                out.err.extend_from_slice(&message);
                return usage(out, Some("unknown option"));
            }
            Arg::Option(b'h', _) => return usage(out, None),
            Arg::Option(b'V', _) => {
                let version = version_gen_header::DTC_VERSION;
                out.out
                    .extend_from_slice(format!("Version: {version}\n").as_bytes());
                return 0;
            }
            Arg::Option(b'i', value) => input = value,
            Arg::Option(b'o', value) => output = value,
            Arg::Option(b'v', _) => verbose = true,
            _ => {}
        }
    }
    let Some(input) = input else {
        return usage(out, Some("missing input file"));
    };
    let Some(output) = output else {
        return usage(out, Some("missing output file"));
    };
    if overlays.is_empty() {
        return usage(out, Some("missing overlay file(s)"));
    }
    if verbose {
        out.out.extend_from_slice(b"input  = ");
        out.out.extend_from_slice(input.as_bytes());
        out.out.extend_from_slice(b"\noutput = ");
        out.out.extend_from_slice(output.as_bytes());
        out.out.push(b'\n');
        for (i, path) in overlays.iter().enumerate() {
            out.out
                .extend_from_slice(format!("overlay[{i}] = ").as_bytes());
            out.out.extend_from_slice(path.as_bytes());
            out.out.push(b'\n');
        }
    }
    let Some(mut base) = read(out, &input, true) else {
        return 1;
    };
    let mut blobs = Vec::new();
    for path in &overlays {
        let Some(blob) = read(out, path, false) else {
            return 1;
        };
        blobs.push(blob);
    }
    let mut capacity = u32::from_be_bytes(base[4..8].try_into().unwrap()) as usize;
    for (path, blob) in overlays.iter().zip(blobs) {
        let Some(result) = apply(out, &base, &blob, &mut capacity, path) else {
            return 1;
        };
        base = result;
    }
    if let Err(err) = libfdt::pack(&mut base) {
        out.err
            .extend_from_slice(format!("\nFailed to pack output: {err}\n").as_bytes());
        return 1;
    }
    if !out.write(&output, &base) {
        failed(out, b"write", &output);
        return 1;
    }
    0
}
fn main() {
    let mut out = Output::default();
    let status = run(&mut out);
    out.finish(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
