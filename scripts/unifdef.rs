// SPDX-License-Identifier: BSD-2-Clause
/*
 * Copyright (c) 2002 - 2011 Tony Finch <dot@dotat.at>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

//! Remove preprocessor branches selected by explicitly defined symbols.
//!
//! The parser and transition table preserve unifdef 2.5 semantics, including
//! unknown expressions, ignored text regions, and obfuscated directives.

use std::env;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::fd::AsFd;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_DEPTH: usize = 64;
const MAX_LINE: usize = 4096;
const MAX_SYMBOLS: usize = 4096;
const USAGE: &str = "usage: unifdef [-bBcdeKknsStV] [-Ipath] [-Dsym[=val]] [-Usym] [-iDsym[=val]] [-iUsym] ... [file]\n";
const VERSION: &str = "Version: unifdef-2.5 \nAuthor: Tony Finch (dot@dotat.at) \nURL: http://dotat.at/prog/unifdef \n";

#[derive(Default)]
struct Options {
    compress_blank: bool,
    blank_deleted: bool,
    complement: bool,
    debug: bool,
    ioccc: bool,
    strict: bool,
    constants: bool,
    line_numbers: bool,
    symbols: bool,
    symbol_depth: bool,
    text: bool,
    output: Option<OsString>,
}

struct Symbol {
    name: Vec<u8>,
    value: Option<Vec<u8>>,
    ignore: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Eval {
    Unknown,
    True,
    False,
    Error,
}
impl Eval {
    fn of(value: i32) -> Self {
        if value == 0 {
            Self::False
        } else {
            Self::True
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Unknown => "IF",
            Self::True => "TRUE",
            Self::False => "FALSE",
            Self::Error => "ERROR",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum Kind {
    TrueIgnore,
    FalseIgnore,
    If,
    True,
    False,
    Elif,
    ElTrue,
    ElFalse,
    Else,
    Endif,
    Plain,
    Eof,
}
#[derive(Clone, Copy)]
struct Line {
    kind: Kind,
    dodgy: bool,
}
impl Line {
    fn index(self) -> usize {
        match self.kind {
            Kind::Plain => 20,
            Kind::Eof => 21,
            kind => kind as usize + if self.dodgy { 10 } else { 0 },
        }
    }
    fn name(self) -> String {
        const NAMES: [&str; 12] = [
            "TRUEI", "FALSEI", "IF", "TRUE", "FALSE", "ELIF", "ELTRUE", "ELFALSE", "ELSE", "ENDIF",
            "PLAIN", "EOF",
        ];
        format!(
            "{}{}",
            if self.dodgy { "DODGY " } else { "" },
            NAMES[self.kind as usize]
        )
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum State {
    Outside,
    FalsePrefix,
    TruePrefix,
    PassMiddle,
    FalseMiddle,
    TrueMiddle,
    PassElse,
    FalseElse,
    TrueElse,
    FalseTrailer,
}
impl State {
    fn name(self) -> &'static str {
        [
            "OUTSIDE",
            "FALSE_PREFIX",
            "TRUE_PREFIX",
            "PASS_MIDDLE",
            "FALSE_MIDDLE",
            "TRUE_MIDDLE",
            "PASS_ELSE",
            "FALSE_ELSE",
            "TRUE_ELSE",
            "FALSE_TRAILER",
        ][self as usize]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Comment {
    None,
    C,
    Cxx,
    Starting,
    Finishing,
    Char,
    String,
}
impl Comment {
    fn name(self) -> &'static str {
        match self {
            Self::None => "NO",
            Self::C => "C",
            Self::Cxx => "CXX",
            Self::Starting => "STARTING",
            Self::Finishing => "FINISHING",
            Self::Char => "CHAR",
            Self::String => "STRING",
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
enum LineState {
    Start,
    Hash,
    Dirty,
}
impl LineState {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "START",
            Self::Hash => "HASH",
            Self::Dirty => "DIRTY",
        }
    }
}

#[derive(Clone, Copy)]
struct Frame {
    state: State,
    ignore: bool,
    start: usize,
}

struct Engine {
    options: Options,
    symbols: Vec<Symbol>,
    filename: String,
    program: String,
    input: Vec<u8>,
    cursor: usize,
    line: Vec<u8>,
    linenum: usize,
    keyword: usize,
    newline: Option<&'static [u8]>,
    comment: Comment,
    line_state: LineState,
    stack: Vec<Frame>,
    deleted: usize,
    blank_count: usize,
    blank_max: usize,
    constant_expression: bool,
    zero_symbols: bool,
    first_symbol: bool,
    status: u8,
    output: Vec<u8>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn sym_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
fn space(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte == 0x0b
}
fn symbol_end(bytes: &[u8], mut pos: usize) -> usize {
    while bytes.get(pos).is_some_and(|&byte| sym_byte(byte)) {
        pos += 1;
    }
    pos
}

impl Engine {
    fn new(program: String) -> Self {
        Self {
            options: Options::default(),
            symbols: Vec::new(),
            filename: "[stdin]".to_owned(),
            program,
            input: Vec::new(),
            cursor: 0,
            line: Vec::new(),
            linenum: 0,
            keyword: 0,
            newline: None,
            comment: Comment::None,
            line_state: LineState::Start,
            stack: vec![Frame {
                state: State::Outside,
                ignore: false,
                start: 0,
            }],
            deleted: 0,
            blank_count: 1000,
            blank_max: 1000,
            constant_expression: false,
            zero_symbols: true,
            first_symbol: false,
            status: 0,
            output: Vec::new(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
    fn depth(&self) -> usize {
        self.stack.len() - 1
    }
    fn frame(&self) -> &Frame {
        self.stack.last().expect("outer frame")
    }
    fn frame_mut(&mut self) -> &mut Frame {
        self.stack.last_mut().expect("outer frame")
    }
    fn byte(&self, pos: usize) -> u8 {
        self.line.get(pos).copied().unwrap_or(0)
    }
    fn starts(&self, pos: usize, text: &[u8]) -> bool {
        self.line.get(pos..).is_some_and(|s| s.starts_with(text))
    }
    fn newline(&self) -> &'static [u8] {
        self.newline.unwrap_or(b"\n")
    }
    fn warn(&mut self, message: &str) {
        self.stderr
            .extend_from_slice(format!("{}: {message}\n", self.program).as_bytes());
    }
    fn debug(&mut self, message: String) {
        if self.options.debug {
            self.warn(&message);
        }
    }
    fn error(&self, message: &str) -> String {
        let context = if self.depth() == 0 {
            String::new()
        } else {
            format!(" (#if line {} depth {})", self.frame().start, self.depth())
        };
        format!("{}: {}: {message}{context}", self.filename, self.linenum)
    }
    fn find_symbol(&mut self, name: &[u8]) -> Option<usize> {
        let name = &name[..symbol_end(name, 0)];
        if name.is_empty() {
            return None;
        }
        if self.options.symbols {
            if self.options.symbol_depth && self.first_symbol {
                self.stdout.extend_from_slice(
                    format!(
                        "{}{:3}",
                        if self.zero_symbols { "" } else { "\n" },
                        self.depth()
                    )
                    .as_bytes(),
                );
            }
            self.first_symbol = false;
            self.zero_symbols = false;
            if self.options.symbol_depth {
                self.stdout.push(b' ');
            }
            self.stdout.extend_from_slice(name);
            if !self.options.symbol_depth {
                self.stdout.push(b'\n');
            }
            return Some(0);
        }
        let result = self.symbols.iter().position(|symbol| symbol.name == name);
        if let Some(index) = result {
            let symbol = &self.symbols[index];
            self.debug(format!(
                "findsym {} {}",
                String::from_utf8_lossy(&symbol.name),
                String::from_utf8_lossy(symbol.value.as_deref().unwrap_or(b""))
            ));
        }
        result
    }
    fn add_symbol(&mut self, ignore: bool, define: bool, argument: &[u8]) -> Result<(), String> {
        let index = match self.find_symbol(argument) {
            Some(index) => index,
            None => {
                if self.symbols.len() == MAX_SYMBOLS {
                    return Err("too many symbols".to_owned());
                }
                self.symbols.len()
            }
        };
        let end = symbol_end(argument, 0);
        let value = if define {
            match argument.get(end) {
                Some(b'=') => Some(argument[end + 1..].to_vec()),
                None => Some(b"1".to_vec()),
                _ => return Err(USAGE.to_owned()),
            }
        } else {
            if end != argument.len() {
                return Err(USAGE.to_owned());
            }
            None
        };
        let symbol = Symbol {
            name: argument[..end].to_vec(),
            value,
            ignore,
        };
        self.debug(format!(
            "addsym {}={}",
            String::from_utf8_lossy(&symbol.name),
            String::from_utf8_lossy(symbol.value.as_deref().unwrap_or(b"undef"))
        ));
        if index == self.symbols.len() {
            self.symbols.push(symbol);
        } else {
            self.symbols[index] = symbol;
        }
        Ok(())
    }

    // fgets semantics matter for line numbers and directives spanning MAX_LINE.
    fn read_chunk(&mut self, size: usize) -> Option<Vec<u8>> {
        if size <= 1 {
            return Some(Vec::new());
        }
        if self.cursor == self.input.len() {
            return None;
        }
        let available = &self.input[self.cursor..];
        let length = available
            .iter()
            .position(|&c| c == b'\n')
            .map_or(available.len(), |n| n + 1)
            .min(size - 1);
        let result = available[..length].to_vec();
        self.cursor += length;
        Some(result)
    }

    fn skip_comment(&mut self, mut pos: usize) -> Result<usize, String> {
        if self.options.text || self.frame().ignore {
            while space(self.byte(pos)) {
                if self.byte(pos) == b'\n' {
                    self.line_state = LineState::Start;
                }
                pos += 1;
            }
            return Ok(pos);
        }
        while self.byte(pos) != 0 {
            if self.starts(pos, b"\\\r\n") {
                pos += 3;
                continue;
            }
            if self.starts(pos, b"\\\n") {
                pos += 2;
                continue;
            }
            match self.comment {
                Comment::None => {
                    if self.starts(pos, b"/\\\r\n") {
                        self.comment = Comment::Starting;
                        pos += 4;
                    } else if self.starts(pos, b"/\\\n") {
                        self.comment = Comment::Starting;
                        pos += 3;
                    } else if self.starts(pos, b"/*") {
                        self.comment = Comment::C;
                        pos += 2;
                    } else if self.starts(pos, b"//") {
                        self.comment = Comment::Cxx;
                        pos += 2;
                    } else if matches!(self.byte(pos), b'\'' | b'"') {
                        self.comment = if self.byte(pos) == b'\'' {
                            Comment::Char
                        } else {
                            Comment::String
                        };
                        self.line_state = LineState::Dirty;
                        pos += 1;
                    } else if self.byte(pos) == b'\n' {
                        self.line_state = LineState::Start;
                        pos += 1;
                    } else if matches!(self.byte(pos), b' ' | b'\r' | b'\t') {
                        pos += 1;
                    } else {
                        return Ok(pos);
                    }
                }
                Comment::Cxx => {
                    if self.byte(pos) == b'\n' {
                        self.comment = Comment::None;
                        self.line_state = LineState::Start;
                    }
                    pos += 1;
                }
                Comment::Char | Comment::String => {
                    if (self.comment == Comment::Char && self.byte(pos) == b'\'')
                        || (self.comment == Comment::String && self.byte(pos) == b'"')
                    {
                        self.comment = Comment::None;
                        pos += 1;
                    } else if self.byte(pos) == b'\\' {
                        pos += if self.byte(pos + 1) == 0 { 1 } else { 2 };
                    } else if self.byte(pos) == b'\n' {
                        return Err(self.error(if self.comment == Comment::Char {
                            "unterminated char literal"
                        } else {
                            "unterminated string literal"
                        }));
                    } else {
                        pos += 1;
                    }
                }
                Comment::C => {
                    if self.starts(pos, b"*\\\r\n") {
                        self.comment = Comment::Finishing;
                        pos += 4;
                    } else if self.starts(pos, b"*\\\n") {
                        self.comment = Comment::Finishing;
                        pos += 3;
                    } else if self.starts(pos, b"*/") {
                        self.comment = Comment::None;
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                }
                Comment::Starting => {
                    if self.byte(pos) == b'*' {
                        self.comment = Comment::C;
                        pos += 1;
                    } else if self.byte(pos) == b'/' {
                        self.comment = Comment::Cxx;
                        pos += 1;
                    } else {
                        self.comment = Comment::None;
                        self.line_state = LineState::Dirty;
                    }
                }
                Comment::Finishing => {
                    if self.byte(pos) == b'/' {
                        self.comment = Comment::None;
                        pos += 1;
                    } else {
                        self.comment = Comment::C;
                    }
                }
            }
        }
        Ok(pos)
    }

    fn skip_args(&mut self, original: usize) -> Result<usize, String> {
        let mut pos = self.skip_comment(original)?;
        if self.byte(pos) != b'(' {
            return Ok(pos);
        }
        let mut level = 0;
        loop {
            if self.byte(pos) == b'(' {
                level += 1;
            }
            if self.byte(pos) == b')' {
                level -= 1;
            }
            pos = self.skip_comment(pos + 1)?;
            if level == 0 {
                return Ok(pos);
            }
            if self.byte(pos) == 0 {
                return Ok(original);
            }
        }
    }

    fn eval_unary(&mut self, cursor: &mut usize, value: &mut i32) -> Result<Eval, String> {
        let mut pos = self.skip_comment(*cursor)?;
        let mut result;
        match self.byte(pos) {
            b'!' => {
                self.debug("eval4 !".to_owned());
                pos += 1;
                result = self.eval_unary(&mut pos, value)?;
                if result == Eval::Error {
                    return Ok(result);
                }
                if result != Eval::Unknown {
                    *value = i32::from(*value == 0);
                    result = Eval::of(*value);
                }
            }
            b'(' => {
                pos += 1;
                self.debug("eval4 (".to_owned());
                result = self.eval_table(0, &mut pos, value)?;
                if result == Eval::Error {
                    return Ok(result);
                }
                pos = self.skip_comment(pos)?;
                if self.byte(pos) != b')' {
                    return Ok(Eval::Error);
                }
                pos += 1;
            }
            b'0'..=b'9' => {
                self.debug("eval4 number".to_owned());
                let Some((number, _)) = c_integer(&self.line[pos..]) else {
                    return Ok(Eval::Error);
                };
                *value = number;
                result = Eval::of(number);
                pos = symbol_end(&self.line, pos);
            }
            _ if self.starts(pos, b"defined") && !sym_byte(self.byte(pos + 7)) => {
                pos = self.skip_comment(pos + 7)?;
                self.debug("eval4 defined".to_owned());
                let parentheses = self.byte(pos) == b'(';
                if parentheses {
                    pos = self.skip_comment(pos + 1)?;
                }
                let name = self.line[pos..symbol_end(&self.line, pos)].to_vec();
                result = match self.find_symbol(&name) {
                    None => Eval::Unknown,
                    Some(index) => {
                        *value = i32::from(
                            self.symbols
                                .get(index)
                                .is_some_and(|symbol| symbol.value.is_some()),
                        );
                        Eval::of(*value)
                    }
                };
                pos = symbol_end(&self.line, pos);
                pos = self.skip_comment(pos)?;
                if parentheses {
                    if self.byte(pos) != b')' {
                        return Ok(Eval::Error);
                    }
                    pos += 1;
                }
                self.constant_expression = false;
            }
            byte if sym_byte(byte) => {
                self.debug("eval4 symbol".to_owned());
                let end = symbol_end(&self.line, pos);
                let name = self.line[pos..end].to_vec();
                let symbol = self.find_symbol(&name);
                pos = end;
                result = match symbol {
                    None => {
                        pos = self.skip_args(pos)?;
                        Eval::Unknown
                    }
                    Some(index) => {
                        match self
                            .symbols
                            .get(index)
                            .and_then(|symbol| symbol.value.as_deref())
                        {
                            None => {
                                *value = 0;
                                Eval::False
                            }
                            Some(bytes) => {
                                let Some((number, consumed)) = c_integer(bytes) else {
                                    return Ok(Eval::Error);
                                };
                                if consumed != bytes.len() {
                                    return Ok(Eval::Error);
                                }
                                *value = number;
                                pos = self.skip_args(pos)?;
                                Eval::of(*value)
                            }
                        }
                    }
                };
                self.constant_expression = false;
            }
            _ => {
                self.debug("eval4 bad expr".to_owned());
                return Ok(Eval::Error);
            }
        }
        *cursor = pos;
        self.debug(format!("eval4 = {value}"));
        Ok(result)
    }

    fn eval_table(
        &mut self,
        level: usize,
        cursor: &mut usize,
        value: &mut i32,
    ) -> Result<Eval, String> {
        const OPS: [&[&[u8]]; 4] = [
            &[b"||"],
            &[b"&&"],
            &[b"==", b"!="],
            &[b"<=", b">=", b"<", b">"],
        ];
        self.debug(format!("eval{level}"));
        let mut pos = *cursor;
        let mut left = if level == 3 {
            self.eval_unary(&mut pos, value)?
        } else {
            self.eval_table(level + 1, &mut pos, value)?
        };
        if left == Eval::Error {
            return Ok(left);
        }
        loop {
            pos = self.skip_comment(pos)?;
            let Some(&operator) = OPS[level]
                .iter()
                .find(|&&operator| self.starts(pos, operator))
            else {
                break;
            };
            pos += operator.len();
            self.debug(format!("eval{level} {}", String::from_utf8_lossy(operator)));
            let mut right_value = 0;
            let right = if level == 3 {
                self.eval_unary(&mut pos, &mut right_value)?
            } else {
                self.eval_table(level + 1, &mut pos, &mut right_value)?
            };
            if right == Eval::Error {
                return Ok(right);
            }
            if !self.options.strict
                && operator == b"||"
                && (left == Eval::True || right == Eval::True)
            {
                *value = 1;
                left = Eval::True;
            } else if !self.options.strict
                && operator == b"&&"
                && (left == Eval::False || right == Eval::False)
            {
                *value = 0;
                left = Eval::False;
            } else if left == Eval::Unknown || right == Eval::Unknown {
                left = Eval::Unknown;
            } else {
                *value = i32::from(match operator {
                    b"||" => *value != 0 || right_value != 0,
                    b"&&" => *value != 0 && right_value != 0,
                    b"==" => *value == right_value,
                    b"!=" => *value != right_value,
                    b"<=" => *value <= right_value,
                    b">=" => *value >= right_value,
                    b"<" => *value < right_value,
                    b">" => *value > right_value,
                    _ => unreachable!(),
                });
                left = Eval::of(*value);
            }
        }
        *cursor = pos;
        self.debug(format!("eval{level} = {value}"));
        self.debug(format!("eval{level} lt = {}", left.name()));
        Ok(left)
    }

    fn if_eval(&mut self, pos: &mut usize) -> Result<Eval, String> {
        self.debug(format!(
            "eval {}",
            String::from_utf8_lossy(&self.line[*pos..])
        ));
        self.constant_expression = !self.options.constants;
        let mut value = 0;
        let result = self.eval_table(0, pos, &mut value)?;
        self.debug(format!("eval = {value}"));
        Ok(if self.constant_expression || result == Eval::Error {
            Eval::Unknown
        } else {
            result
        })
    }

    fn parse_line(&mut self) -> Result<Line, String> {
        self.linenum += 1;
        let Some(line) = self.read_chunk(MAX_LINE) else {
            return Ok(Line {
                kind: Kind::Eof,
                dodgy: false,
            });
        };
        self.line = line;
        if self.newline.is_none() {
            self.newline = Some(if self.line.windows(2).any(|pair| pair == b"\r\n") {
                b"\r\n"
            } else {
                b"\n"
            });
        }
        let mut result = Line {
            kind: Kind::Plain,
            dodgy: false,
        };
        let was_comment = self.comment;
        let mut pos = self.skip_comment(0)?;
        if self.line_state == LineState::Start {
            if self.byte(pos) == b'#' {
                self.line_state = LineState::Hash;
                self.first_symbol = true;
                pos = self.skip_comment(pos + 1)?;
            } else if self.byte(pos) != 0 {
                self.line_state = LineState::Dirty;
            }
        }
        if self.comment == Comment::None && self.line_state == LineState::Hash {
            self.keyword = pos;
            pos = symbol_end(&self.line, pos);
            let keyword = self.line[self.keyword..pos].to_vec();
            if self.starts(pos, b"\\\r\n") || self.starts(pos, b"\\\n") {
                return Err(self.error("Obfuscated preprocessor control line"));
            }
            result.kind = match keyword.as_slice() {
                b"ifdef" | b"ifndef" => {
                    pos = self.skip_comment(pos)?;
                    let name = self.line[pos..symbol_end(&self.line, pos)].to_vec();
                    let kind = match self.find_symbol(&name) {
                        None => Kind::If,
                        Some(index) => {
                            let defined = self
                                .symbols
                                .get(index)
                                .is_some_and(|symbol| symbol.value.is_some());
                            let ignored =
                                self.symbols.get(index).is_some_and(|symbol| symbol.ignore);
                            match (defined ^ (keyword == b"ifndef"), ignored) {
                                (true, true) => Kind::TrueIgnore,
                                (false, true) => Kind::FalseIgnore,
                                (true, false) => Kind::True,
                                (false, false) => Kind::False,
                            }
                        }
                    };
                    pos = symbol_end(&self.line, pos);
                    kind
                }
                b"if" | b"elif" => match (self.if_eval(&mut pos)?, keyword == b"elif") {
                    (Eval::True, false) => Kind::True,
                    (Eval::False, false) => Kind::False,
                    (Eval::True, true) => Kind::ElTrue,
                    (Eval::False, true) => Kind::ElFalse,
                    (_, false) => Kind::If,
                    (_, true) => Kind::Elif,
                },
                b"else" => Kind::Else,
                b"endif" => Kind::Endif,
                _ => {
                    self.line_state = LineState::Dirty;
                    Kind::Plain
                }
            };
            pos = self.skip_comment(pos)?;
            if self.byte(pos) != 0 {
                self.line_state = LineState::Dirty;
                result.kind = match result.kind {
                    Kind::True | Kind::False | Kind::TrueIgnore | Kind::FalseIgnore => Kind::If,
                    Kind::ElTrue | Kind::ElFalse => Kind::Elif,
                    other => other,
                };
            }
            if result.kind != Kind::Plain
                && (was_comment != Comment::None || self.comment != Comment::None)
            {
                result.dodgy = true;
                if self.comment != Comment::None {
                    self.line_state = LineState::Dirty;
                }
            }
            if self.line_state == LineState::Hash {
                match self.read_chunk(MAX_LINE - pos) {
                    None => {
                        self.line.truncate(pos);
                        self.line.extend_from_slice(self.newline());
                        pos += self.newline().len();
                        self.line_state = LineState::Start;
                    }
                    Some(extra) => {
                        self.line.truncate(pos);
                        self.line.extend_from_slice(&extra);
                        self.line_state = LineState::Dirty;
                    }
                }
            }
        }
        if self.line_state == LineState::Dirty {
            while self.byte(pos) != 0 {
                pos = self.skip_comment(pos + 1)?;
            }
        }
        self.debug(format!(
            "parser line {} state {} comment {} line",
            self.linenum,
            self.comment.name(),
            self.line_state.name()
        ));
        Ok(result)
    }

    fn emit(&mut self, bytes: &[u8]) {
        if self.options.output.is_some() {
            self.output.extend_from_slice(bytes);
        } else {
            self.stdout.extend_from_slice(bytes);
        }
    }
    fn flush_line(&mut self, keep: bool) {
        if self.options.symbols {
            return;
        }
        if keep ^ self.options.complement {
            let end = self
                .line
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(self.line.len());
            let blank = self.line[..end]
                .iter()
                .all(|byte| b" \t\r\n".contains(byte));
            if blank && self.options.compress_blank && self.blank_count != self.blank_max {
                self.deleted += 1;
                self.blank_count += 1;
            } else {
                if self.options.line_numbers && self.deleted > 0 {
                    self.stdout
                        .extend_from_slice(format!("#line {}", self.linenum).as_bytes());
                    self.stdout.extend_from_slice(self.newline());
                }
                let line = self.line[..end].to_vec();
                self.emit(&line);
                self.deleted = 0;
                self.blank_count = if blank { self.blank_count + 1 } else { 0 };
                self.blank_max = self.blank_count;
            }
        } else {
            if self.options.blank_deleted {
                self.emit(self.newline());
            }
            self.status = 1;
            self.deleted += 1;
            self.blank_count = 0;
        }
    }
    fn ignore_off(&mut self) {
        let ignored = self.stack[self.depth() - 1].ignore;
        self.frame_mut().ignore = ignored;
    }
    fn nest(&mut self) -> Result<(), String> {
        if self.depth() == MAX_DEPTH - 1 {
            return Err(self.error("Too many levels of nesting"));
        }
        self.stack.push(Frame {
            state: State::Outside,
            ignore: false,
            start: self.linenum,
        });
        Ok(())
    }
    fn edit_keyword(&mut self, replacement: &[u8]) {
        self.line.truncate(self.keyword);
        self.line.extend_from_slice(replacement);
        self.line.extend_from_slice(self.newline());
        self.flush_line(true);
    }

    fn action(&mut self, action: Action) -> Result<bool, String> {
        use Action::*;
        match action {
            Eelif => return Err(self.error("Inappropriate #elif")),
            Eelse => return Err(self.error("Inappropriate #else")),
            Eendif => return Err(self.error("Inappropriate #endif")),
            Eeof => return Err(self.error("Premature EOF")),
            Eioccc => return Err(self.error("Obfuscated preprocessor control line")),
            Done => {
                if self.comment != Comment::None {
                    return Err(self.error("EOF in comment"));
                }
                return Ok(true);
            }
            Print => self.flush_line(true),
            Drop => self.flush_line(false),
            Strue | Sfalse | Pelif | Dfalse | Delif => {
                self.flush_line(matches!(action, Pelif));
                self.ignore_off();
                self.frame_mut().state = match action {
                    Strue => State::TruePrefix,
                    Sfalse => State::FalsePrefix,
                    Pelif => State::PassMiddle,
                    Dfalse => State::FalseTrailer,
                    Delif => State::FalseMiddle,
                    _ => unreachable!(),
                };
            }
            Selse | Pelse | Delse => {
                self.flush_line(matches!(action, Pelse));
                self.frame_mut().state = match action {
                    Selse => State::TrueElse,
                    Pelse => State::PassElse,
                    _ => State::FalseElse,
                };
            }
            Pendif | Dendif => {
                self.flush_line(matches!(action, Pendif));
                self.stack.pop();
            }
            Fdrop | Fpass | Ftrue | Ffalse => {
                self.nest()?;
                self.action(match action {
                    Fdrop => Dfalse,
                    Fpass => Pelif,
                    Ftrue => Strue,
                    _ => Sfalse,
                })?;
            }
            Oiffy | Oif | Oelif => {
                if !self.options.ioccc {
                    return self.action(Eioccc);
                }
                self.action(if matches!(action, Oelif) {
                    Pelif
                } else {
                    Fpass
                })?;
                if matches!(action, Oiffy) {
                    self.frame_mut().ignore = true;
                }
            }
            Idrop | Itrue | Ifalse => {
                self.action(match action {
                    Idrop => Fdrop,
                    Itrue => Ftrue,
                    _ => Ffalse,
                })?;
                self.frame_mut().ignore = true;
            }
            Mpass => {
                self.line[self.keyword..self.keyword + 4].copy_from_slice(b"if  ");
                self.action(Pelif)?;
            }
            Mtrue | Melif | Melse => {
                self.edit_keyword(if matches!(action, Mtrue) {
                    b"else"
                } else {
                    b"endif"
                });
                self.frame_mut().state = match action {
                    Mtrue => State::TrueMiddle,
                    Melif => State::FalseTrailer,
                    _ => State::FalseElse,
                };
            }
        }
        Ok(false)
    }
    fn process(&mut self) -> Result<(), String> {
        loop {
            let line = self.parse_line()?;
            let action = TRANSITIONS[self.frame().state as usize][line.index()];
            if self.action(action)? {
                return Ok(());
            }
            self.debug(format!(
                "process line {} {} -> {} depth {}",
                self.linenum,
                line.name(),
                self.frame().state.name(),
                self.depth()
            ));
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    Eelif,
    Eelse,
    Eendif,
    Eeof,
    Eioccc,
    Print,
    Drop,
    Done,
    Strue,
    Sfalse,
    Selse,
    Pelif,
    Pelse,
    Pendif,
    Dfalse,
    Delif,
    Delse,
    Dendif,
    Fdrop,
    Fpass,
    Ftrue,
    Ffalse,
    Oiffy,
    Oif,
    Oelif,
    Idrop,
    Itrue,
    Ifalse,
    Mpass,
    Mtrue,
    Melif,
    Melse,
}
use Action::*;
const TRANSITIONS: [[Action; 22]; 10] = [
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Eelif, Eelif, Eelif, Eelse, Eendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Eelif, Eelif, Eelif, Eelse, Eendif, Print, Done,
    ],
    [
        Idrop, Idrop, Fdrop, Fdrop, Fdrop, Mpass, Strue, Sfalse, Selse, Dendif, Idrop, Idrop,
        Fdrop, Fdrop, Fdrop, Mpass, Eioccc, Eioccc, Eioccc, Eioccc, Drop, Eeof,
    ],
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Dfalse, Dfalse, Dfalse, Delse, Dendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Eioccc, Eioccc, Eioccc, Eioccc, Eioccc, Print, Eeof,
    ],
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Pelif, Mtrue, Delif, Pelse, Pendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Pelif, Oelif, Oelif, Pelse, Pendif, Print, Eeof,
    ],
    [
        Idrop, Idrop, Fdrop, Fdrop, Fdrop, Pelif, Mtrue, Delif, Pelse, Pendif, Idrop, Idrop, Fdrop,
        Fdrop, Fdrop, Eioccc, Eioccc, Eioccc, Eioccc, Eioccc, Drop, Eeof,
    ],
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Melif, Melif, Melif, Melse, Pendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Eioccc, Eioccc, Eioccc, Eioccc, Pendif, Print, Eeof,
    ],
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Eelif, Eelif, Eelif, Eelse, Pendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Eelif, Eelif, Eelif, Eelse, Pendif, Print, Eeof,
    ],
    [
        Idrop, Idrop, Fdrop, Fdrop, Fdrop, Eelif, Eelif, Eelif, Eelse, Dendif, Idrop, Idrop, Fdrop,
        Fdrop, Fdrop, Eelif, Eelif, Eelif, Eelse, Eioccc, Drop, Eeof,
    ],
    [
        Itrue, Ifalse, Fpass, Ftrue, Ffalse, Eelif, Eelif, Eelif, Eelse, Dendif, Oiffy, Oiffy,
        Fpass, Oif, Oif, Eelif, Eelif, Eelif, Eelse, Eioccc, Print, Eeof,
    ],
    [
        Idrop, Idrop, Fdrop, Fdrop, Fdrop, Dfalse, Dfalse, Dfalse, Delse, Dendif, Idrop, Idrop,
        Fdrop, Fdrop, Fdrop, Dfalse, Dfalse, Dfalse, Delse, Eioccc, Drop, Eeof,
    ],
];

// strtol(base=0), converted to C's int. Literal suffixes are skipped by the
// expression lexer, but -D values must consume the whole argument.
fn c_integer(bytes: &[u8]) -> Option<(i32, usize)> {
    let mut pos = 0;
    while bytes.get(pos).is_some_and(|&byte| space(byte)) {
        pos += 1;
    }
    let negative = bytes.get(pos) == Some(&b'-');
    if negative || bytes.get(pos) == Some(&b'+') {
        pos += 1;
    }
    let digit = |byte: u8| -> Option<u64> {
        match byte {
            b'0'..=b'9' => Some(u64::from(byte - b'0')),
            b'a'..=b'f' => Some(u64::from(byte - b'a' + 10)),
            b'A'..=b'F' => Some(u64::from(byte - b'A' + 10)),
            _ => None,
        }
    };
    let base = if bytes.get(pos) == Some(&b'0') {
        if matches!(bytes.get(pos + 1), Some(b'x' | b'X'))
            && bytes.get(pos + 2).and_then(|&byte| digit(byte)).is_some()
        {
            pos += 2;
            16
        } else {
            8
        }
    } else {
        10
    };
    let start = pos;
    let mut magnitude = 0u64;
    while let Some(value) = bytes
        .get(pos)
        .and_then(|&byte| digit(byte))
        .filter(|&value| value < base)
    {
        magnitude = magnitude.saturating_mul(base).saturating_add(value);
        pos += 1;
    }
    if pos == start {
        return None;
    }
    let maximum = std::ffi::c_long::MAX as u64;
    let value = if negative {
        if magnitude > maximum {
            std::ffi::c_long::MIN as i64
        } else {
            -(magnitude as i64)
        }
    } else {
        magnitude.min(maximum) as i64
    };
    Some((value as i32, pos))
}

enum Command {
    Process(Option<PathBuf>),
    Version,
}

fn arguments(engine: &mut Engine) -> Result<Command, String> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let mut files = Vec::new();
    let mut scan = true;
    let posix = env::var_os("POSIXLY_CORRECT").is_some();
    while let Some(arg) = args.next() {
        let bytes = arg.as_encoded_bytes();
        if scan && bytes == b"--" {
            scan = false;
            continue;
        }
        if !scan || bytes.first() != Some(&b'-') || bytes.len() == 1 {
            files.push(PathBuf::from(arg));
            if posix {
                scan = false;
            }
            continue;
        }
        let mut pos = 1;
        while pos < bytes.len() {
            let option = bytes[pos];
            pos += 1;
            let value = if b"iDUIo".contains(&option) {
                if pos < bytes.len() {
                    let value = bytes[pos..].to_vec();
                    pos = bytes.len();
                    value
                } else if let Some(value) = args.next() {
                    value.into_encoded_bytes()
                } else {
                    engine.stderr.extend_from_slice(
                        format!(
                            "{}: option requires an argument -- '{}'\n",
                            program.to_string_lossy(),
                            char::from(option)
                        )
                        .as_bytes(),
                    );
                    return Err(USAGE.to_owned());
                }
            } else {
                Vec::new()
            };
            match option {
                b'i' => match value.first() {
                    Some(b'D') => engine.add_symbol(true, true, &value[1..])?,
                    Some(b'U') => engine.add_symbol(true, false, &value[1..])?,
                    _ => return Err(USAGE.to_owned()),
                },
                b'D' => engine.add_symbol(false, true, &value)?,
                b'U' => engine.add_symbol(false, false, &value)?,
                b'I' => (),
                b'b' | b'l' => engine.options.blank_deleted = true,
                b'B' => engine.options.compress_blank = true,
                b'c' => engine.options.complement = true,
                b'd' => engine.options.debug = true,
                b'e' => engine.options.ioccc = true,
                b'K' => engine.options.strict = true,
                b'k' => engine.options.constants = true,
                b'n' => engine.options.line_numbers = true,
                b'o' => engine.options.output = Some(OsString::from_vec(value)),
                b's' => engine.options.symbols = true,
                b'S' => {
                    engine.options.symbols = true;
                    engine.options.symbol_depth = true;
                }
                b't' => engine.options.text = true,
                b'V' => return Ok(Command::Version),
                _ => {
                    engine.stderr.extend_from_slice(
                        format!(
                            "{}: invalid option -- '{}'\n",
                            program.to_string_lossy(),
                            char::from(option)
                        )
                        .as_bytes(),
                    );
                    return Err(USAGE.to_owned());
                }
            }
        }
    }
    if engine.options.compress_blank && engine.options.blank_deleted {
        return Err("-B and -b are mutually exclusive".to_owned());
    }
    if files.len() > 1 {
        return Err("can only do one file".to_owned());
    }
    Ok(Command::Process(
        files.pop().filter(|path| path != Path::new("-")),
    ))
}

fn io_error(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .to_owned()
}

struct OutputFile {
    file: Option<File>,
    path: PathBuf,
    temporary: Option<PathBuf>,
}

impl OutputFile {
    fn open(path: &Path, input: Option<&fs::Metadata>) -> Result<Self, String> {
        let same_file =
            input
                .zip(fs::metadata(path).ok().as_ref())
                .is_some_and(|(input, output)| {
                    input.dev() == output.dev() && input.ino() == output.ino()
                });
        if !same_file {
            return File::create(path)
                .map(|file| Self {
                    file: Some(file),
                    path: path.to_owned(),
                    temporary: None,
                })
                .map_err(|error| format!("can't open {}: {}", path.display(), io_error(&error)));
        }
        let directory = path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
            ^ u64::from(std::process::id());
        for _ in 0..1024 {
            let mut suffix = String::new();
            for _ in 0..6 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                suffix.push(char::from(
                    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                        [(seed % 62) as usize],
                ));
            }
            let temporary = directory.join(format!("unifdef.{suffix}"));
            match OpenOptions::new()
                .create_new(true)
                .read(true)
                .write(true)
                .mode(0o600)
                .open(&temporary)
            {
                Ok(file) => {
                    if let Some(input) = input {
                        let _ =
                            file.set_permissions(fs::Permissions::from_mode(input.mode() & 0o777));
                    }
                    return Ok(Self {
                        file: Some(file),
                        path: path.to_owned(),
                        temporary: Some(temporary),
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(format!("can't create temporary file: {}", io_error(&error)))
                }
            }
        }
        Err("can't create temporary file: File exists".to_owned())
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut file = self.file.take().expect("output file is open");
        file.write_all(bytes)?;
        file.flush()
    }

    fn commit(&mut self) -> io::Result<()> {
        if let Some(temporary) = &self.temporary {
            fs::rename(temporary, &self.path)?;
            self.temporary = None;
        }
        Ok(())
    }
}

impl std::ops::Drop for OutputFile {
    fn drop(&mut self) {
        // An error must leave the original input intact. Only remove the
        // exclusively created temporary file owned by this invocation.
        if let Some(temporary) = &self.temporary {
            let _ = fs::remove_file(temporary);
        }
    }
}

fn execute(engine: &mut Engine, path: Option<PathBuf>) -> u8 {
    let (mut input, metadata): (Box<dyn Read>, _) = if let Some(path) = path {
        engine.filename = path.to_string_lossy().into_owned();
        match File::open(&path) {
            Ok(file) => {
                let metadata = file.metadata().ok();
                (Box::new(file), metadata)
            }
            Err(error) => {
                engine.warn(&format!(
                    "can't open {}: {}",
                    path.display(),
                    io_error(&error)
                ));
                return 2;
            }
        }
    } else {
        let stdin = io::stdin();
        let metadata = stdin
            .as_fd()
            .try_clone_to_owned()
            .ok()
            .and_then(|fd| File::from(fd).metadata().ok());
        (Box::new(stdin), metadata)
    };
    let mut output = if let Some(path) = &engine.options.output {
        match OutputFile::open(Path::new(path), metadata.as_ref()) {
            Ok(file) => Some(file),
            Err(error) => {
                engine.warn(&error);
                return 2;
            }
        }
    } else {
        None
    };
    // The reference treats a failed fgets as EOF, including any preceding data.
    let _ = input.read_to_end(&mut engine.input);
    let result = engine.process();
    if let Err(error) = &result {
        engine.warn(error);
    }
    if engine.options.symbol_depth && !engine.zero_symbols {
        engine.stdout.push(b'\n');
    }
    if let Some(file) = &mut output {
        if let Err(error) = file.write(&engine.output) {
            engine.warn(&format!(
                "couldn't write to {}: {}",
                file.path.display(),
                io_error(&error)
            ));
            if file.temporary.is_some() {
                engine.warn(&format!("{} unchanged", engine.filename));
            }
            return 2;
        }
    }
    if result.is_err() {
        engine.warn("output may be truncated");
        return 2;
    }
    if let Some(file) = &mut output {
        if let Err(error) = file.commit() {
            engine.warn(&format!(
                "couldn't rename temporary file: {}",
                io_error(&error)
            ));
            engine.warn(&format!("{} unchanged", file.path.display()));
            return 2;
        }
    }
    engine.status
}

fn main() -> ExitCode {
    let program = env::args_os().next().unwrap_or_default();
    let program = Path::new(&program)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let mut engine = Engine::new(program);
    let mut status = match arguments(&mut engine) {
        Ok(Command::Version) => {
            engine.stderr.extend_from_slice(VERSION.as_bytes());
            0
        }
        Ok(Command::Process(path)) => execute(&mut engine, path),
        Err(error) => {
            if error == USAGE {
                engine.stderr.extend_from_slice(USAGE.as_bytes());
            } else {
                engine.warn(&error);
            }
            2
        }
    };
    if let Err(error) = io::stdout()
        .write_all(&engine.stdout)
        .and_then(|()| io::stdout().flush())
    {
        if engine.options.output.is_none() {
            engine.warn(&format!("couldn't write to [stdout]: {}", io_error(&error)));
            status = 2;
        }
    }
    let _ = io::stderr().write_all(&engine.stderr);
    ExitCode::from(status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
