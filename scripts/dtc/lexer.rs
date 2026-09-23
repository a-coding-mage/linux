// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005. */

//! Byte-oriented implementation of the start conditions in dtc-lexer.l.

use crate::dtc_header::{Data, Diagnostics, SourcePos};
use crate::srcpos::Sources;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Eof,
    Char(u8),
    V1,
    Plugin,
    Memreserve,
    LShift,
    RShift,
    Le,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Bits,
    DeleteProperty,
    DeleteNode,
    Omit,
    Name,
    Integer,
    CharInteger,
    Byte,
    String,
    Label,
    LabelRef,
    PathRef,
    Incbin,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) kind: Kind,
    pub(crate) text: Vec<u8>,
    pub(crate) value: u64,
    pub(crate) pos: SourcePos,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Initial,
    V1,
    Bytes,
    Name,
}

pub(crate) struct Lexer {
    pub(crate) sources: Sources,
    mode: Mode,
    last: SourcePos,
}

fn namechar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b",._+*#?@-".contains(&byte)
}
fn labelstart(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}
fn labelchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn string_len(input: &[u8]) -> usize {
    if input.first() != Some(&b'"') {
        return 0;
    }
    let mut index = 1;
    while let Some(&byte) = input.get(index) {
        if byte == b'"' {
            return index + 1;
        }
        if byte == b'\\' {
            if input.get(index + 1).is_none_or(|&b| b == b'\n') {
                return 0;
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    0
}

fn char_len(input: &[u8]) -> usize {
    if input.first() != Some(&b'\'') {
        return 0;
    }
    let mut index = 1;
    let mut last = 0;
    while let Some(&byte) = input.get(index) {
        if byte == b'\'' {
            return index + 1;
        }
        if byte == b'\\' && input.get(index + 1) == Some(&b'\'') {
            last = index + 2;
            index += 2;
        } else {
            index += 1;
        }
    }
    // Flex can backtrack: the quote after a final slash also closes a literal.
    last
}

impl Lexer {
    pub(crate) fn new(name: &[u8], paths: Vec<Vec<u8>>) -> Result<Self, Vec<u8>> {
        let mut sources = Sources::new(paths);
        sources.push(name)?;
        let initial_file = sources.stack[0].name.clone();
        Ok(Self {
            sources,
            mode: Mode::Initial,
            last: SourcePos {
                file: initial_file,
                file_id: Some(0),
                ..SourcePos::default()
            },
        })
    }

    pub(crate) fn next(&mut self, diagnostics: &mut Diagnostics) -> Result<Token, Vec<u8>> {
        loop {
            while self
                .sources
                .stack
                .last()
                .is_some_and(|s| s.offset == s.bytes.len())
            {
                self.sources.stack.pop();
            }
            let Some(input) = self.sources.stack.last() else {
                return Ok(Token {
                    kind: Kind::Eof,
                    text: Vec::new(),
                    value: 0,
                    pos: self.last.clone(),
                });
            };
            let bytes = &input.bytes[input.offset..];
            // Select the longest match, resolving equal lengths by flex rule order.
            let mut matched = (0usize, 0usize, Kind::Eof);
            let mut candidate = |len: usize, rule: usize, kind: Kind| {
                if len > matched.0 {
                    matched = (len, rule, kind);
                }
            };
            if bytes.starts_with(b"/include/") {
                let mut index = 9;
                while matches!(bytes.get(index), Some(b' ' | b'\t'..=b'\r')) {
                    index += 1;
                }
                let len = string_len(&bytes[index..]);
                if len != 0 {
                    candidate(index + len, 1, Kind::Eof);
                }
            }
            if input.column == 1 && bytes.first() == Some(&b'#') {
                let mut index = if bytes.starts_with(b"#line") { 5 } else { 1 };
                let whitespace = index;
                while matches!(bytes.get(index), Some(b' ' | b'\t')) {
                    index += 1;
                }
                let digits = index;
                while bytes.get(index).is_some_and(u8::is_ascii_digit) {
                    index += 1;
                }
                let end_digits = index;
                while matches!(bytes.get(index), Some(b' ' | b'\t')) {
                    index += 1;
                }
                let len = string_len(&bytes[index..]);
                if digits > whitespace && end_digits > digits && index > end_digits && len != 0 {
                    index += len;
                    loop {
                        let save = index;
                        while matches!(bytes.get(index), Some(b' ' | b'\t')) {
                            index += 1;
                        }
                        let start = index;
                        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
                            index += 1;
                        }
                        if save == start || start == index {
                            index = save;
                            break;
                        }
                    }
                    candidate(index, 2, Kind::Eof);
                }
            }
            candidate(string_len(bytes), 3, Kind::String);
            for (word, kind) in [
                (b"/dts-v1/".as_slice(), Kind::V1),
                (b"/plugin/", Kind::Plugin),
                (b"/memreserve/", Kind::Memreserve),
                (b"/bits/", Kind::Bits),
                (b"/delete-property/", Kind::DeleteProperty),
                (b"/delete-node/", Kind::DeleteNode),
                (b"/omit-if-no-ref/", Kind::Omit),
            ] {
                if bytes.starts_with(word) {
                    candidate(word.len(), 4, kind);
                }
            }
            let label = if labelstart(bytes[0]) {
                bytes.iter().take_while(|&&b| labelchar(b)).count()
            } else {
                0
            };
            if label != 0 && bytes.get(label) == Some(&b':') {
                candidate(label + 1, 5, Kind::Label);
            }
            if self.mode == Mode::V1 {
                candidate(label, 6, Kind::Integer);
                if bytes[0].is_ascii_digit() {
                    let mut len = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
                    if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") {
                        let digits = bytes[2..]
                            .iter()
                            .take_while(|b| b.is_ascii_hexdigit())
                            .count();
                        if digits != 0 {
                            len = 2 + digits;
                        }
                    }
                    for suffix in [b"ULL".as_slice(), b"LL", b"UL", b"U", b"L"] {
                        if bytes[len..].starts_with(suffix) {
                            len += suffix.len();
                            break;
                        }
                    }
                    candidate(len, 7, Kind::Integer);
                }
            }
            candidate(char_len(bytes), 8, Kind::CharInteger);
            if bytes[0] == b'&' {
                if bytes.get(1).is_some_and(|&b| labelstart(b)) {
                    candidate(
                        1 + bytes[1..].iter().take_while(|&&b| labelchar(b)).count(),
                        9,
                        Kind::LabelRef,
                    );
                }
                if bytes.get(1) == Some(&b'{') {
                    let len = bytes[2..]
                        .iter()
                        .take_while(|&&b| namechar(b) || b == b'/')
                        .count();
                    if bytes.get(2 + len) == Some(&b'}') {
                        candidate(3 + len, 10, Kind::PathRef);
                    }
                }
            }
            if self.mode == Mode::Bytes {
                if bytes.len() >= 2 && bytes[..2].iter().all(u8::is_ascii_hexdigit) {
                    candidate(2, 11, Kind::Byte);
                }
                if bytes[0] == b']' {
                    candidate(1, 12, Kind::Char(b']'));
                }
            }
            if self.mode == Mode::Name {
                let skip = usize::from(bytes[0] == b'\\');
                let len = bytes[skip..].iter().take_while(|&&b| namechar(b)).count();
                if len != 0 {
                    candidate(skip + len, 13, Kind::Name);
                }
            }
            if matches!(self.mode, Mode::Initial | Mode::V1) && bytes.starts_with(b"/incbin/") {
                candidate(8, 14, Kind::Incbin);
            }
            candidate(
                bytes
                    .iter()
                    .take_while(|b| matches!(b, b' ' | b'\t'..=b'\r'))
                    .count(),
                15,
                Kind::Eof,
            );
            if bytes.starts_with(b"/*") {
                let mut length = 0;
                while bytes[length..].starts_with(b"/*") {
                    let Some(end) = bytes[length + 2..].windows(2).position(|w| w == b"*/") else {
                        break;
                    };
                    length += end + 4;
                }
                candidate(length, 15, Kind::Eof);
            }
            if bytes.starts_with(b"//") {
                let mut length = 0;
                while bytes[length..].starts_with(b"//") {
                    let Some(end) = bytes[length..].iter().position(|&b| b == b'\n') else {
                        break;
                    };
                    length += end + 1;
                }
                candidate(length, 15, Kind::Eof);
            }
            for (word, kind) in [
                (b"<<", Kind::LShift),
                (b">>", Kind::RShift),
                (b"<=", Kind::Le),
                (b">=", Kind::Ge),
                (b"==", Kind::Eq),
                (b"!=", Kind::Ne),
                (b"&&", Kind::And),
                (b"||", Kind::Or),
            ] {
                if bytes.starts_with(word) {
                    candidate(2, 16, kind);
                }
            }
            candidate(1, 17, Kind::Char(bytes[0]));
            let (len, rule, mut kind) = matched;
            let (mut text, pos) = self.sources.consume(len);
            self.last = pos.clone();
            let mut value = 0;
            match rule {
                1 => {
                    let quote = text.iter().position(|&b| b == b'"').unwrap();
                    let filename = &text[quote + 1..text.len() - 1];
                    self.sources
                        .push(filename.split(|&b| b == 0).next().unwrap_or(b""))?;
                    continue;
                }
                2 => {
                    let digit = text.iter().position(u8::is_ascii_digit).unwrap();
                    let digit_end = digit
                        + text[digit..]
                            .iter()
                            .take_while(|b| b.is_ascii_digit())
                            .count();
                    let line = std::str::from_utf8(&text[digit..digit_end])
                        .unwrap()
                        .parse::<i64>()
                        .unwrap_or(i64::MAX) as i32;
                    let start = text.iter().position(|&b| b == b'"').unwrap();
                    let end = text.iter().rposition(|&b| b == b'"').unwrap();
                    let filename = Data::escape_string(&text[start + 1..end])?.bytes;
                    if filename[..filename.len() - 1].contains(&0) {
                        diagnostics.source(b"Lexical error", &pos, b"nul in line number directive");
                    }
                    self.sources.set_line(
                        filename.split(|&b| b == 0).next().unwrap_or(b"").to_vec(),
                        line.wrapping_sub(1),
                    );
                    continue;
                }
                3 => text = Data::escape_string(&text[1..text.len() - 1])?.bytes,
                4 => match kind {
                    Kind::V1 | Kind::Memreserve | Kind::Bits => self.mode = Mode::V1,
                    Kind::DeleteProperty | Kind::DeleteNode | Kind::Omit => self.mode = Mode::Name,
                    _ => {}
                },
                5 => {
                    text.pop();
                }
                6 => diagnostics.source(
                    b"Lexical error",
                    &pos,
                    format!("Unexpected '{}'", crate::dtc_header::display(&text)),
                ),
                7 => {
                    let (number, end, overflow) = crate::util::integer(&text);
                    value = number;
                    if !text[end..].iter().all(|b| matches!(b, b'U' | b'L')) {
                        diagnostics.source(
                            b"Lexical error",
                            &pos,
                            format!(
                                "Bad integer literal '{}'",
                                crate::dtc_header::display(&text)
                            ),
                        );
                    }
                    if overflow {
                        diagnostics.source(
                            b"Lexical error",
                            &pos,
                            format!(
                                "Integer literal '{}' out of range",
                                crate::dtc_header::display(&text)
                            ),
                        );
                    }
                }
                8 => {
                    let data = Data::escape_string(&text[1..text.len() - 1])?;
                    if data.bytes.len() == 1 {
                        diagnostics.source(b"Lexical error", &pos, b"Empty character literal");
                    } else {
                        value = u64::from(data.bytes[0]);
                        if data.bytes.len() > 2 {
                            diagnostics.source(
                                b"Lexical error",
                                &pos,
                                format!(
                                    "Character literal has {} characters instead of 1",
                                    data.bytes.len() - 1
                                ),
                            );
                        }
                    }
                }
                9 => {
                    text.remove(0);
                }
                10 => {
                    text = text[2..text.len() - 1].to_vec();
                }
                11 => value = u64::from_str_radix(std::str::from_utf8(&text).unwrap(), 16).unwrap(),
                12 => self.mode = Mode::V1,
                13 => {
                    if text[0] == b'\\' {
                        text.remove(0);
                    }
                    self.mode = Mode::V1;
                }
                15 => continue,
                17 => match text[0] {
                    0 => kind = Kind::Eof,
                    b'[' => self.mode = Mode::Bytes,
                    b'{' | b';' => self.mode = Mode::Name,
                    _ => {}
                },
                _ => {}
            }
            return Ok(Token {
                kind,
                text,
                value,
                pos,
            });
        }
    }
}
