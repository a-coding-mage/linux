// SPDX-License-Identifier: GPL-2.0-or-later
// Lexical analysis for genksyms.
// Copyright 1996, 1997 Linux International.
// Original implementation by Richard Henderson and Bjorn Ekwall.

use super::keywords::keyword;
use super::parser_tables::*;
use super::{Kind, Symbols, Word};

/// A semantic value identifies a mutable link, not the token at that link.
/// Stable arena indices replace the original pointer-to-pointer lists.
pub(super) type Link = Option<usize>;

#[derive(Default, Clone)]
pub(super) struct Node {
    pub(super) word: Word,
    pub(super) source: bool,
    pub(super) next: Link,
}

#[derive(Default)]
pub(super) struct Arena {
    pub(super) nodes: Vec<Node>,
}

impl Arena {
    pub(super) fn get(&self, link: Link) -> Link {
        link.and_then(|index| self.nodes[index].next)
    }
    pub(super) fn set(&mut self, link: Link, node: Link) {
        if let Some(index) = link {
            self.nodes[index].next = node;
        }
    }
    pub(super) fn remove(&mut self, link: Link) {
        let node = self.get(link);
        self.set(link, self.get(node));
    }
    pub(super) fn range(&self, mut start: Link, end: Link) -> Vec<Word> {
        let mut words = Vec::new();
        while start != end {
            let Some(index) = start else {
                break;
            };
            words.push(self.nodes[index].word.clone());
            start = self.nodes[index].next;
        }
        words.reverse();
        words
    }
    pub(super) fn copy(&mut self, start: Link) -> Link {
        let mut next = None;
        for word in self.range(start, None) {
            let index = self.nodes.len();
            self.nodes.push(Node {
                word,
                source: false,
                next,
            });
            next = Some(index);
        }
        next
    }
    pub(super) fn name(&self, link: Link) -> Option<Vec<u8>> {
        self.get(link)
            .map(|index| self.nodes[index].word.text.clone())
    }
    pub(super) fn tag(&mut self, link: Link, kind: Kind) {
        if let Some(index) = self.get(link) {
            self.nodes[index].word.kind = kind;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    Attribute,
    Asm,
    Typeof,
    TypeofFirst,
    Bracket,
    Brace,
    Expression,
    StaticAssert,
}

struct Raw {
    kind: i16,
    text: Vec<u8>,
}

pub(super) struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    pending: Vec<Raw>,
    state: State,
    next: usize,
    source_file: Option<Vec<u8>>,
    in_source: bool,
    suppress_type: u8,
    want_body: u8,
    pub(super) dont_want_type: bool,
    pub(super) arena: Arena,
    trace_started: bool,
    trace_loaded: usize,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            position: 0,
            pending: Vec::new(),
            state: State::Normal,
            next: 0,
            source_file: None,
            in_source: false,
            suppress_type: 0,
            want_body: 0,
            dont_want_type: false,
            arena: Arena {
                nodes: vec![Node::default()],
            },
            trace_started: false,
            trace_loaded: 0,
        }
    }
    fn append(&mut self, text: &[u8], symbols: &Symbols) {
        let text = &text[..text
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(text.len())];
        let kind = if symbols.find(text, Kind::EnumConst, true).is_some() {
            Kind::EnumConst
        } else {
            Kind::Normal
        };
        let node = &mut self.arena.nodes[self.next];
        node.word = Word {
            text: text.to_vec(),
            kind,
        };
        node.source = self.in_source;
        let next = self.arena.nodes.len();
        self.arena.nodes.push(Node {
            next: Some(self.next),
            ..Node::default()
        });
        self.next = next;
    }
    pub(super) fn next(&mut self, symbols: &mut Symbols) -> (i16, Link) {
        let mut count = 0i32;
        loop {
            let raw = self.raw(symbols);
            let mut token = raw.kind;
            if token == 0 {
                return (0, Some(self.next));
            }
            match self.state {
                State::Normal => {
                    self.append(&raw.text, symbols);
                    if token == IDENT {
                        if let Some(keyword) = keyword(&raw.text) {
                            token = keyword;
                            match keyword {
                                ATTRIBUTE_KEYW => {
                                    self.state = State::Attribute;
                                    continue;
                                }
                                ASM_KEYW => {
                                    self.state = State::Asm;
                                    continue;
                                }
                                TYPEOF_KEYW => {
                                    self.state = State::Typeof;
                                    continue;
                                }
                                STATIC_ASSERT_KEYW => {
                                    self.state = State::StaticAssert;
                                    continue;
                                }
                                STRUCT_KEYW | UNION_KEYW | ENUM_KEYW => {
                                    self.want_body = 3;
                                    self.suppress_type = 2;
                                }
                                _ => {}
                            }
                        }
                        if self.suppress_type == 0
                            && !self.dont_want_type
                            && symbols.find(&raw.text, Kind::Typedef, true).is_some()
                        {
                            token = TYPE;
                        }
                    } else {
                        match token as u8 {
                            b'[' if token < 256 => {
                                self.state = State::Bracket;
                                count = 1;
                                continue;
                            }
                            b'{' if token < 256 && self.want_body == 0 => {
                                self.state = State::Brace;
                                count = 1;
                                continue;
                            }
                            b'=' | b':' if token < 256 => {
                                self.state = State::Expression;
                            }
                            _ => {}
                        }
                    }
                }
                State::Attribute | State::Asm | State::StaticAssert => {
                    self.append(&raw.text, symbols);
                    if token == i16::from(b'(') {
                        count += 1;
                    } else if token == i16::from(b')') {
                        count -= 1;
                        if count == 0 {
                            token = match self.state {
                                State::Attribute => ATTRIBUTE_PHRASE,
                                State::Asm => ASM_PHRASE,
                                _ => STATIC_ASSERT_PHRASE,
                            };
                            self.state = State::Normal;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    if self.state != State::Normal {
                        continue;
                    }
                }
                State::Typeof | State::TypeofFirst => {
                    if self.state == State::TypeofFirst {
                        if token == IDENT {
                            if keyword(&raw.text).is_some()
                                || symbols.find(&raw.text, Kind::Typedef, true).is_some()
                            {
                                self.pending.push(raw);
                                self.pending.push(Raw {
                                    kind: i16::from(b'('),
                                    text: b"(".to_vec(),
                                });
                                self.state = State::Normal;
                                token = TYPEOF_KEYW;
                                return self.finish(token);
                            }
                            self.append(b"(", symbols);
                        }
                        self.state = State::Typeof;
                    }
                    if token == i16::from(b'(') {
                        count += 1;
                        if count == 1 {
                            self.state = State::TypeofFirst;
                        } else {
                            self.append(&raw.text, symbols);
                        }
                        continue;
                    }
                    self.append(&raw.text, symbols);
                    if token == i16::from(b')') {
                        count -= 1;
                        if count == 0 {
                            self.state = State::Normal;
                            token = TYPEOF_PHRASE;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                State::Bracket | State::Brace => {
                    self.append(&raw.text, symbols);
                    let (open, close, phrase) = if self.state == State::Bracket {
                        (b'[', b']', BRACKET_PHRASE)
                    } else {
                        (b'{', b'}', BRACE_PHRASE)
                    };
                    if token == i16::from(open) {
                        count += 1;
                    }
                    if token == i16::from(close) {
                        count -= 1;
                        if count == 0 {
                            self.state = State::Normal;
                            token = phrase;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                State::Expression => {
                    if count == 0 && matches!(token, 125 | 44 | 59) {
                        self.pending.push(raw);
                        self.state = State::Normal;
                        token = EXPRESSION_PHRASE;
                    } else {
                        if matches!(token, 40 | 91 | 123) {
                            count += 1;
                        }
                        if matches!(token, 41 | 93 | 125) {
                            count -= 1;
                        }
                        self.append(&raw.text, symbols);
                        continue;
                    }
                }
            }
            return self.finish(token);
        }
    }
    fn finish(&mut self, token: i16) -> (i16, Link) {
        self.suppress_type = self.suppress_type.saturating_sub(1);
        if token != ATTRIBUTE_PHRASE {
            self.want_body = self.want_body.saturating_sub(1);
        }
        (token, Some(self.next))
    }
    fn raw(&mut self, symbols: &mut Symbols) -> Raw {
        if let Some(raw) = self.pending.pop() {
            self.trace_accept(symbols, raw_rule(raw.kind), &raw.text);
            return raw;
        }
        if symbols.debug > 2 && !self.trace_started {
            symbols
                .stderr
                .extend_from_slice(b"--(end of buffer or a NUL)\n");
            self.trace_started = true;
            self.trace_loaded = self.input.len().min(8192);
        }
        loop {
            let start = self.position;
            let rest = &self.input[start..];
            let Some(&byte) = rest.first() else {
                if symbols.debug > 2 {
                    if self.position != 0 {
                        symbols
                            .stderr
                            .extend_from_slice(b"--(end of buffer or a NUL)\n");
                    }
                    symbols
                        .stderr
                        .extend_from_slice(b"--EOF (start condition 0)\n");
                }
                return Raw {
                    kind: 0,
                    text: Vec::new(),
                };
            };
            if byte == b'#' && (start == 0 || self.input[start - 1] == b'\n') {
                if let Some(end) = rest.iter().position(|&byte| byte == b'\n') {
                    let filename_marker = marker(&rest[..end]);
                    self.trace_raw(
                        symbols,
                        if filename_marker.is_some() { 57 } else { 58 },
                        &rest[..end + 1],
                        start + end + 1,
                        false,
                    );
                    if let Some((line, filename)) = filename_marker {
                        symbols.line = line;
                        symbols.filename = filename.to_vec();
                        if self.source_file.is_none() {
                            self.source_file = Some(filename.to_vec());
                        }
                        self.in_source = self.source_file.as_deref() == Some(filename);
                    } else {
                        symbols.line = symbols.line.wrapping_add(1);
                    }
                    self.position += end + 1;
                    continue;
                }
            }
            if byte == b'\n' {
                self.trace_raw(symbols, 59, b"\n", start + 1, false);
                symbols.line = symbols.line.wrapping_add(1);
                self.position += 1;
                continue;
            }
            if matches!(byte, b' ' | b'\t' | b'\r' | 0x0b | 0x0c) {
                let length = rest
                    .iter()
                    .take_while(|&&byte| matches!(byte, b' ' | b'\t' | b'\r' | 0x0b | 0x0c))
                    .count();
                self.trace_raw(symbols, 62, &rest[..length], start + length, true);
                self.position += length;
                continue;
            }
            let mut length = 1;
            let mut kind = i16::from(byte as std::os::raw::c_char);
            let quote_start = usize::from(byte == b'L');
            if let Some(&quote @ (b'\'' | b'"')) = rest.get(quote_start) {
                if let Some(end) = quote_end(rest, quote_start, quote) {
                    length = end;
                    kind = if quote == b'"' { STRING } else { CHAR };
                }
            }
            if ident_start(byte) {
                let end = rest
                    .iter()
                    .take_while(|&&byte| ident_continue(byte))
                    .count();
                if end >= length && kind != STRING && kind != CHAR {
                    length = end;
                    kind = if vector_integer(&rest[..end]) {
                        BUILTIN_INT_KEYW
                    } else {
                        IDENT
                    };
                }
            }
            if byte.is_ascii_digit() || byte == b'.' {
                let integer = integer_length(rest);
                if integer > 0 {
                    length = integer;
                    kind = INT;
                }
                let real = real_length(rest);
                if real > length {
                    length = real;
                    kind = REAL;
                }
            }
            if rest.starts_with(b"...") {
                length = 3;
                kind = DOTS;
            } else if rest.len() >= 2
                && ((b"~%^&*+=|<>/-".contains(&byte) && rest[1] == b'=')
                    || matches!(&rest[..2], b"&&" | b"||" | b"->" | b"<<" | b">>"))
            {
                length = 2;
                kind = OTHER;
            }
            self.position += length;
            let extendable = match kind {
                IDENT | BUILTIN_INT_KEYW => true,
                INT => !matches!(
                    rest.get(length.wrapping_sub(2)..length),
                    Some(b"UL" | b"Ul" | b"uL" | b"ul" | b"LU" | b"Lu" | b"lU" | b"lu")
                ),
                REAL => !matches!(rest[length - 1], b'f' | b'F' | b'l' | b'L'),
                kind if kind < 256 => b"~%^&*+=|<>/-L.\"'".contains(&byte),
                _ => false,
            };
            self.trace_raw(
                symbols,
                raw_rule(kind),
                &rest[..length],
                self.position,
                extendable,
            );
            return Raw {
                kind,
                text: rest[..length].to_vec(),
            };
        }
    }
    fn trace_raw(
        &mut self,
        symbols: &mut Symbols,
        rule: usize,
        text: &[u8],
        end: usize,
        extendable: bool,
    ) {
        if symbols.debug <= 2 {
            return;
        }
        while end > self.trace_loaded || (end == self.trace_loaded && extendable) {
            symbols
                .stderr
                .extend_from_slice(b"--(end of buffer or a NUL)\n");
            if self.trace_loaded == self.input.len() {
                break;
            }
            self.trace_loaded = (self.trace_loaded + 8192).min(self.input.len());
        }
        for _ in text.iter().filter(|&&byte| byte == 0) {
            symbols
                .stderr
                .extend_from_slice(b"--(end of buffer or a NUL)\n");
        }
        self.trace_accept(symbols, rule, text);
    }
    fn trace_accept(&self, symbols: &mut Symbols, rule: usize, text: &[u8]) {
        if symbols.debug > 2 {
            symbols
                .stderr
                .extend_from_slice(format!("--accepting rule at line {rule} (\"").as_bytes());
            symbols.stderr.extend_from_slice(
                &text[..text
                    .iter()
                    .position(|&byte| byte == 0)
                    .unwrap_or(text.len())],
            );
            symbols.stderr.extend_from_slice(b"\")\n");
        }
    }
}

fn raw_rule(kind: i16) -> usize {
    match kind {
        BUILTIN_INT_KEYW => 54,
        STRING => 65,
        CHAR => 66,
        IDENT => 67,
        OTHER => 73,
        INT => 74,
        REAL => 75,
        DOTS => 77,
        _ => 80,
    }
}

fn ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$')
}
fn ident_continue(byte: u8) -> bool {
    ident_start(byte) || byte.is_ascii_digit()
}

fn quote_end(input: &[u8], start: usize, quote: u8) -> Option<usize> {
    let mut offset = start + 1;
    while let Some(&byte) = input.get(offset) {
        if byte == quote {
            return Some(offset + 1);
        }
        if byte == b'\\' {
            if input.get(offset + 1).is_none_or(|&byte| byte == b'\n') {
                return None;
            }
            offset += 1;
        }
        offset += 1;
    }
    None
}

fn integer_length(input: &[u8]) -> usize {
    if input.first().is_none_or(|byte| !byte.is_ascii_digit()) {
        return 0;
    }
    let mut end = if input.starts_with(b"0x") || input.starts_with(b"0X") {
        let count = input[2..]
            .iter()
            .take_while(|byte| byte.is_ascii_hexdigit())
            .count();
        if count == 0 {
            1
        } else {
            count + 2
        }
    } else if input[0] == b'0' {
        input
            .iter()
            .take_while(|&&byte| (b'0'..=b'7').contains(&byte))
            .count()
    } else {
        input
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count()
    };
    if let Some(&suffix @ (b'u' | b'U' | b'l' | b'L')) = input.get(end) {
        end += 1;
        if input.get(end).is_some_and(|byte| match suffix {
            b'u' | b'U' => matches!(byte, b'l' | b'L'),
            _ => matches!(byte, b'u' | b'U'),
        }) {
            end += 1;
        }
    }
    end
}

fn real_length(input: &[u8]) -> usize {
    let digits = input
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    let mut end = digits;
    let mut real = false;
    if input.get(end) == Some(&b'.') {
        end += 1;
        let fraction = input[end..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digits + fraction == 0 {
            return 0;
        }
        end += fraction;
        real = true;
    }
    if input
        .get(end)
        .is_some_and(|byte| matches!(byte, b'e' | b'E'))
    {
        let mut exponent = end + 1;
        if input
            .get(exponent)
            .is_some_and(|byte| matches!(byte, b'+' | b'-'))
        {
            exponent += 1;
        }
        let count = input[exponent..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if count != 0 {
            end = exponent + count;
            real = true;
        }
    }
    if !real {
        return 0;
    }
    if input
        .get(end)
        .is_some_and(|byte| matches!(byte, b'f' | b'F' | b'l' | b'L'))
    {
        end += 1;
    }
    end
}

fn marker(input: &[u8]) -> Option<(i32, &[u8])> {
    let mut offset = 1;
    while input
        .get(offset)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        offset += 1;
    }
    if offset == 1 {
        return None;
    }
    let count = integer_length(&input[offset..]);
    if count == 0 {
        return None;
    }
    offset += count;
    let whitespace = offset;
    while input
        .get(offset)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        offset += 1;
    }
    if offset == whitespace || input.get(offset) != Some(&b'"') {
        return None;
    }
    offset += 1;
    let end = input[offset..].iter().position(|&byte| byte == b'"')? + offset;
    if offset == end {
        return None;
    }
    let mut line = 0i32;
    for &byte in input[2..]
        .iter()
        .skip_while(|byte| byte.is_ascii_whitespace())
        .take_while(|byte| byte.is_ascii_digit())
    {
        line = line.wrapping_mul(10).wrapping_add(i32::from(byte - b'0'));
    }
    Some((line, &input[offset..end]))
}

fn vector_integer(text: &[u8]) -> bool {
    let text = text.strip_prefix(b"u").unwrap_or(text);
    let Some(text) = text
        .strip_prefix(b"int")
        .and_then(|text| text.strip_suffix(b"_t"))
    else {
        return false;
    };
    let Some(split) = text.iter().position(|&byte| byte == b'x') else {
        return false;
    };
    matches!(&text[..split], b"8" | b"16" | b"32" | b"64")
        && matches!(&text[split + 1..], b"1" | b"2" | b"4" | b"8" | b"16")
}
