// SPDX-License-Identifier: GPL-2.0-or-later
//! Byte-preserving command-line and file helpers shared by the FDT utilities.
#![allow(dead_code)] // Each utility needs a different subset of the helpers.
use std::ffi::{OsStr, OsString};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;

pub(crate) const TYPE_USAGE: &str = "<type>\ts=string, i=int, u=unsigned, x=hex, r=raw\n\tOptional modifier prefix:\n\t\thh or b=byte, h=2 byte, l=4 byte (default)";

#[derive(Default)]
pub(crate) struct Output {
    pub(crate) out: Vec<u8>,
    pub(crate) err: Vec<u8>,
    stdin_consumed: bool,
}
impl Output {
    pub(crate) fn report(&mut self, name: &[u8], error: super::Error) {
        self.err.extend_from_slice(b"Error at '");
        self.err.extend_from_slice(name);
        self.err
            .extend_from_slice(format!("': {error}\n").as_bytes());
    }
    pub(crate) fn io_error(&mut self, prefix: &[u8], path: &OsStr, error: &io::Error) {
        self.err.extend_from_slice(prefix);
        self.err.extend_from_slice(path.as_bytes());
        self.err.extend_from_slice(b"': ");
        let text = error.to_string();
        let text = text.split(" (os error ").next().unwrap_or(&text);
        self.err.extend_from_slice(text.as_bytes());
        self.err.push(b'\n');
    }
    pub(crate) fn read(&mut self, path: &OsStr) -> Option<Vec<u8>> {
        let result = if path == "-" {
            // utilfdt_read closes standard input after its first read. Retain
            // that observable CLI behavior without closing a process-global FD.
            if self.stdin_consumed {
                self.err
                    .extend_from_slice(b"Couldn't open blob from '-': Bad file descriptor\n");
                return None;
            }
            self.stdin_consumed = true;
            let mut data = Vec::new();
            io::stdin().read_to_end(&mut data).map(|_| data)
        } else {
            std::fs::read(path)
        };
        match result {
            Ok(data) => Some(data),
            Err(err) => {
                self.io_error(b"Couldn't open blob from '", path, &err);
                None
            }
        }
    }
    pub(crate) fn write(&mut self, path: &OsStr, data: &[u8]) -> bool {
        let size = match super::Header::read(data) {
            Ok(h) if h.totalsize <= data.len() => h.totalsize,
            _ => {
                self.err
                    .extend_from_slice(b"Cannot write a truncated device-tree blob\n");
                return false;
            }
        };
        if path == "-" {
            self.out.extend_from_slice(&data[..size]);
            return true;
        }
        if let Err(err) = std::fs::write(path, &data[..size]) {
            self.io_error(b"Couldn't write blob to '", path, &err);
            false
        } else {
            true
        }
    }
    pub(crate) fn finish(self, status: i32) -> ! {
        let mut code = status;
        if let Err(err) = io::stdout().lock().write_all(&self.out) {
            let _ = writeln!(io::stderr(), "stdout: {err}");
            code = 1;
        }
        if io::stderr().lock().write_all(&self.err).is_err() {
            code = 1;
        }
        std::process::exit(code)
    }
}

#[derive(Debug)]
pub(crate) enum Arg {
    Option(u8, Option<OsString>),
    Operand(OsString),
    Error(Vec<u8>),
}

pub(crate) fn arguments(argv: &[OsString], spec: &[u8], longs: &[(&[u8], u8, bool)]) -> Vec<Arg> {
    let mut result = Vec::new();
    let mut index = 1;
    let mut stopped = false;
    let posix = std::env::var_os("POSIXLY_CORRECT").is_some();
    let program = argv.first().map(|s| s.as_bytes()).unwrap_or(b"fdt");
    let diagnostic = |tail: &[u8]| {
        let mut msg = program.to_vec();
        msg.extend_from_slice(b": ");
        msg.extend_from_slice(tail);
        Arg::Error(msg)
    };
    while index < argv.len() {
        let token = argv[index].as_bytes();
        index += 1;
        if stopped || token == b"-" || !token.starts_with(b"-") {
            result.push(Arg::Operand(argv[index - 1].clone()));
            if posix {
                stopped = true;
            }
            continue;
        }
        if token == b"--" {
            stopped = true;
            continue;
        }
        if token.starts_with(b"--") && !longs.is_empty() {
            let text = &token[2..];
            let split = text.iter().position(|&b| b == b'=');
            let name = &text[..split.unwrap_or(text.len())];
            let matches: Vec<_> = longs
                .iter()
                .filter(|(full, _, _)| full.starts_with(name))
                .collect();
            let found = longs.iter().find(|(full, _, _)| *full == name).or_else(|| {
                if matches.len() == 1 {
                    Some(matches[0])
                } else {
                    None
                }
            });
            let Some((full, opt, needs)) = found else {
                if matches.len() > 1 {
                    let mut msg = b"option '".to_vec();
                    msg.extend_from_slice(token);
                    msg.extend_from_slice(b"' is ambiguous; possibilities:");
                    for (full, _, _) in matches {
                        msg.extend_from_slice(b" '--");
                        msg.extend_from_slice(full);
                        msg.push(b'\'');
                    }
                    msg.push(b'\n');
                    result.push(diagnostic(&msg));
                    continue;
                }
                let mut msg = b"unrecognized option '".to_vec();
                msg.extend_from_slice(token);
                msg.extend_from_slice(b"'\n");
                result.push(diagnostic(&msg));
                continue;
            };
            let value = if let Some(split) = split {
                if !needs {
                    let mut msg = b"option '--".to_vec();
                    msg.extend_from_slice(full);
                    msg.extend_from_slice(b"' doesn't allow an argument\n");
                    result.push(diagnostic(&msg));
                    continue;
                }
                Some(OsStr::from_bytes(&text[split + 1..]).to_owned())
            } else if *needs {
                if index == argv.len() {
                    let mut msg = b"option '--".to_vec();
                    msg.extend_from_slice(full);
                    msg.extend_from_slice(b"' requires an argument\n");
                    result.push(diagnostic(&msg));
                    continue;
                }
                index += 1;
                Some(argv[index - 1].clone())
            } else {
                None
            };
            result.push(Arg::Option(*opt, value));
            continue;
        }
        let mut at = 1;
        while at < token.len() {
            let opt = token[at];
            at += 1;
            let Some(where_) = spec.iter().position(|&b| b == opt && b != b':') else {
                let mut msg = b"invalid option -- '".to_vec();
                msg.push(opt);
                msg.extend_from_slice(b"'\n");
                result.push(diagnostic(&msg));
                continue;
            };
            let value = if spec.get(where_ + 1) == Some(&b':') {
                if at < token.len() {
                    let value = OsStr::from_bytes(&token[at..]).to_owned();
                    at = token.len();
                    Some(value)
                } else if index < argv.len() {
                    index += 1;
                    Some(argv[index - 1].clone())
                } else {
                    let mut msg = b"option requires an argument -- '".to_vec();
                    msg.push(opt);
                    msg.extend_from_slice(b"'\n");
                    result.push(diagnostic(&msg));
                    break;
                }
            } else {
                None
            };
            result.push(Arg::Option(opt, value));
        }
    }
    result
}

pub(crate) fn decode_type(mut value: &[u8]) -> Option<(u8, Option<usize>)> {
    let mut qualifier = 0;
    if value.first().is_some_and(|b| b"hlLb".contains(b)) {
        qualifier = value[0];
        value = &value[1..];
        if value.first() == Some(&qualifier) {
            if qualifier == b'h' {
                qualifier = b'b';
            }
            value = &value[1..];
        }
    }
    if value.len() != 1 || !b"iuxsr".contains(&value[0]) {
        return None;
    }
    let size = if value[0] == b's' || value[0] == b'r' {
        None
    } else {
        match qualifier {
            b'b' => Some(1),
            b'h' => Some(2),
            b'l' => Some(4),
            _ => None,
        }
    };
    Some((value[0], size))
}
pub(crate) fn printable_string(value: &[u8]) -> bool {
    if value.is_empty() || value.last() != Some(&0) {
        return false;
    }
    value[..value.len() - 1]
        .split(|&b| b == 0)
        .all(|s| !s.is_empty() && s.iter().all(|&b| (32..127).contains(&b)))
}
pub(crate) fn integer(value: &[u8], kind: u8) -> Option<u32> {
    let mut at = 0;
    while value
        .get(at)
        .is_some_and(|&b| b == b' ' || (b'\t'..=b'\r').contains(&b))
    {
        at += 1;
    }
    let negative = value.get(at) == Some(&b'-');
    if value.get(at).is_some_and(|&b| b == b'-' || b == b'+') {
        at += 1;
    }
    let mut radix = if kind == b'x' { 16 } else { 10 };
    let prefix = (kind == b'i' || kind == b'x')
        && value
            .get(at..at + 2)
            .is_some_and(|s| s == b"0x" || s == b"0X");
    if prefix {
        radix = 16;
        at += 2;
    } else if kind == b'i' && value.get(at) == Some(&b'0') {
        radix = 8;
    }
    let start = at;
    let mut number = 0u64;
    while let Some(digit) = value.get(at).and_then(|&b| (b as char).to_digit(radix)) {
        number = number
            .saturating_mul(radix as u64)
            .saturating_add(digit as u64);
        at += 1;
    }
    if at == start && !prefix {
        return None;
    }
    if negative {
        number = number.wrapping_neg();
    }
    Some(number as u32)
}
