// SPDX-License-Identifier: GPL-2.0-or-later
//! Byte-preserving command-line and diagnostic helpers for x86 decoder tests.

#[path = "../../../tools/arch/x86/lib/insn.rs"]
pub(crate) mod decoder;

use std::ffi::{OsStr, OsString};
use std::io::{self, BufRead};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

pub(crate) struct Options<'a> {
    args: &'a [OsString],
    spec: &'static [u8],
    index: usize,
    byte: usize,
    stopped: bool,
    stop_at_nonoption: bool,
}

impl<'a> Options<'a> {
    pub(crate) fn new(args: &'a [OsString], spec: &'static [u8]) -> Self {
        Self {
            args,
            spec,
            index: 1,
            byte: 0,
            stopped: false,
            stop_at_nonoption: std::env::var_os("POSIXLY_CORRECT").is_some(),
        }
    }

    fn error(&self, text: &[u8], option: u8) -> Vec<u8> {
        let mut message = self.args[0].as_bytes().to_vec();
        message.extend_from_slice(text);
        message.push(option);
        message.extend_from_slice(b"'\n");
        message
    }
}

impl Iterator for Options<'_> {
    type Item = Result<(u8, Option<OsString>), Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stopped && self.index < self.args.len() {
            let arg = self.args[self.index].as_bytes();
            if self.byte == 0 {
                if arg == b"--" {
                    self.stopped = true;
                    return None;
                }
                if arg.len() < 2 || arg[0] != b'-' {
                    if self.stop_at_nonoption {
                        self.stopped = true;
                        return None;
                    }
                    self.index += 1;
                    continue;
                }
                self.byte = 1;
            }
            let option = arg[self.byte];
            self.byte += 1;
            let position = self
                .spec
                .iter()
                .position(|&value| value == option && value != b':');
            let requires_argument = position.is_some_and(|at| self.spec.get(at + 1) == Some(&b':'));
            let value = if requires_argument && self.byte < arg.len() {
                let value = OsString::from_vec(arg[self.byte..].to_vec());
                self.index += 1;
                self.byte = 0;
                Some(value)
            } else if requires_argument {
                self.index += 1;
                self.byte = 0;
                if self.index == self.args.len() {
                    return Some(Err(
                        self.error(b": option requires an argument -- '", option)
                    ));
                }
                let value = self.args[self.index].clone();
                self.index += 1;
                Some(value)
            } else {
                if self.byte == arg.len() {
                    self.index += 1;
                    self.byte = 0;
                }
                None
            };
            return Some(if position.is_none() {
                Err(self.error(b": invalid option -- '", option))
            } else {
                Ok((option, value))
            });
        }
        None
    }
}

pub(crate) fn cstr(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())]
}

pub(crate) fn unsigned(bytes: &[u8], mut radix: u32) -> (u64, usize) {
    let digit = |byte: u8| match byte {
        b'0'..=b'9' => Some((byte - b'0') as u32),
        b'a'..=b'z' => Some((byte - b'a') as u32 + 10),
        b'A'..=b'Z' => Some((byte - b'A') as u32 + 10),
        _ => None,
    };
    let mut index = 0;
    while bytes
        .get(index)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'..=b'\r'))
    {
        index += 1;
    }
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    if (radix == 0 || radix == 16)
        && bytes.get(index) == Some(&b'0')
        && matches!(bytes.get(index + 1), Some(b'x' | b'X'))
        && bytes
            .get(index + 2)
            .and_then(|&b| digit(b))
            .is_some_and(|d| d < 16)
    {
        index += 2;
        radix = 16;
    } else if radix == 0 {
        radix = if bytes.get(index) == Some(&b'0') {
            8
        } else {
            10
        };
    }
    let start = index;
    let (mut value, mut overflow) = (0u64, false);
    let maximum = usize::MAX as u64; // Linux unsigned long has pointer width.
    while let Some(number) = bytes
        .get(index)
        .and_then(|&b| digit(b))
        .filter(|&d| d < radix)
    {
        match value
            .checked_mul(radix as u64)
            .and_then(|v| v.checked_add(number as u64))
        {
            Some(next) if next <= maximum => value = next,
            _ => {
                value = maximum;
                overflow = true;
            }
        }
        index += 1;
    }
    if index == start {
        return (0, 0);
    }
    if negative && !overflow {
        value = value.wrapping_neg() & maximum;
    }
    (value, index)
}

pub(crate) struct Line {
    pub(crate) bytes: Vec<u8>,
    pub(crate) eof: bool,
}

pub(crate) fn fgets(reader: &mut dyn BufRead, limit: usize) -> io::Result<Option<Line>> {
    let mut bytes = Vec::new();
    while bytes.len() < limit.saturating_sub(1) {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return Ok((!bytes.is_empty()).then_some(Line { bytes, eof: true }));
        }
        let count = available
            .iter()
            .position(|&b| b == b'\n')
            .map_or(available.len(), |at| at + 1)
            .min(limit - 1 - bytes.len());
        let newline = available[count - 1] == b'\n';
        bytes.extend_from_slice(&available[..count]);
        reader.consume(count);
        if newline {
            break;
        }
    }
    Ok(Some(Line { bytes, eof: false }))
}

pub(crate) fn error(prefix: &[u8], err: &io::Error) -> Vec<u8> {
    let mut message = prefix.to_vec();
    message.extend_from_slice(b": ");
    let text = err.to_string();
    message.extend_from_slice(text.split(" (os error ").next().unwrap_or(&text).as_bytes());
    message.push(b'\n');
    message
}

pub(crate) fn dump_insn(output: &mut Vec<u8>, insn: &decoder::Instruction<'_>) {
    output.extend_from_slice(b"Instruction = {\n");
    for (name, field) in [
        ("prefixes", &insn.prefixes),
        ("rex_prefix", &insn.rex_prefix),
        ("vex_prefix", &insn.vex_prefix),
        ("opcode", &insn.opcode),
        ("modrm", &insn.modrm),
        ("sib", &insn.sib),
        ("displacement", &insn.displacement),
        ("immediate1", &insn.immediate1),
        ("immediate2", &insn.immediate2),
    ] {
        let b = field.bytes;
        output.extend_from_slice(format!("\t.{name} = {{\n\t\t.value = {}, bytes[] = {{{:x}, {:x}, {:x}, {:x}}},\n\t\t.got = {}, .nbytes = {}}},\n",
            field.value(), b[0], b[1], b[2], b[3], field.got as u8, field.nbytes).as_bytes());
    }
    output.extend_from_slice(format!("\t.attr = {:x}, .opnd_bytes = {}, .addr_bytes = {},\n\t.length = {}, .x86_64 = {}, .kaddr = {:p}}}\n",
        insn.attr, insn.opnd_bytes, insn.addr_bytes, insn.length, insn.x86_64 as u8,
        insn.bytes.as_ptr()).as_bytes());
}

pub(crate) fn program(args: &[OsString]) -> &OsStr {
    &args[0]
}
