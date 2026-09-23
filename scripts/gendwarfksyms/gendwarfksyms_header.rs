// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Owned options, byte-preserving diagnostics, and shared result types.

use std::io;

pub(crate) const SYMBOL_PTR_PREFIX: &[u8] = b"__gendwarfksyms_ptr_";

#[derive(Clone, Copy, Default)]
pub(crate) struct Options {
    pub(crate) debug: bool,
    pub(crate) dump_dies: bool,
    pub(crate) dump_die_map: bool,
    pub(crate) dump_types: bool,
    pub(crate) dump_versions: bool,
    pub(crate) stable: bool,
    pub(crate) symtypes: bool,
}

#[derive(Debug)]
pub(crate) struct Error(pub(crate) Vec<u8>);

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub(crate) fn bytes(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

pub(crate) fn error(function: &str, parts: &[&[u8]]) -> Error {
    Error(bytes(&[
        b"error: gendwarfksyms: ",
        function.as_bytes(),
        b": ",
        &bytes(parts),
        b"\n",
    ]))
}

pub(crate) fn io_error(function: &str, err: io::Error) -> Error {
    let message = err.to_string();
    let reason = message.split(" (os error ").next().unwrap_or(&message);
    error(function, &[b"I/O error: ", reason.as_bytes()])
}

pub(crate) struct Diagnostics {
    pub(crate) options: Options,
    pub(crate) bytes: Vec<u8>,
}

impl Diagnostics {
    pub(crate) fn new(options: Options) -> Self {
        Self {
            options,
            bytes: Vec::new(),
        }
    }

    pub(crate) fn print(&mut self, parts: &[&[u8]]) {
        for part in parts {
            self.bytes.extend_from_slice(part);
        }
    }

    pub(crate) fn debug(&mut self, function: &str, parts: &[&[u8]]) {
        if self.options.debug {
            self.print(&[b"gendwarfksyms: ", function.as_bytes(), b": "]);
            self.print(parts);
            self.print(&[b"\n"]);
        }
    }

    pub(crate) fn warn(&mut self, function: &str, parts: &[&[u8]]) {
        self.print(&[b"warning: gendwarfksyms: ", function.as_bytes(), b": "]);
        self.print(parts);
        self.print(&[b"\n"]);
    }
}

/// Preserve the C table's FNV bucket ordering, including host char signedness.
pub(crate) fn hash_bytes(value: &[u8]) -> u32 {
    value.iter().fold(2_166_136_261u32, |hash, &byte| {
        (hash ^ byte as std::ffi::c_char as u32).wrapping_mul(0x0100_0193)
    })
}

pub(crate) fn hash_32(value: u32) -> u32 {
    value.wrapping_mul(0x61c8_8647)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
