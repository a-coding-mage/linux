// SPDX-License-Identifier: GPL-2.0-or-later
// Simplified ASN.1 notation parser.
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com).
//! Compile the kernel's ASN.1 grammars to BER decoder state machines.

#![forbid(unsafe_code)]

// The translated kernel header exports its definitions for other consumers;
// this standalone host program uses it as a private module.
#[allow(dead_code, non_camel_case_types, missing_docs, unreachable_pub)]
#[path = "../include/linux/asn1_header.rs"]
mod asn1;

use asn1::asn1_class::*;
use asn1::asn1_tag::*;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::process::ExitCode;

const DIRECTIVES: &[&str] = &[
    "ABSENT",
    "ALL",
    "ANY",
    "APPLICATION",
    "AUTOMATIC",
    "BEGIN",
    "BIT",
    "BMPString",
    "BOOLEAN",
    "BY",
    "CHARACTER",
    "CHOICE",
    "CLASS",
    "COMPONENT",
    "COMPONENTS",
    "CONSTRAINED",
    "CONTAINING",
    "DEFAULT",
    "DEFINED",
    "DEFINITIONS",
    "EMBEDDED",
    "ENCODED",
    "ENCODING-CONTROL",
    "END",
    "ENUMERATED",
    "EXCEPT",
    "EXPLICIT",
    "EXPORTS",
    "EXTENSIBILITY",
    "EXTERNAL",
    "FALSE",
    "FROM",
    "GeneralString",
    "GeneralizedTime",
    "GraphicString",
    "IA5String",
    "IDENTIFIER",
    "IMPLICIT",
    "IMPLIED",
    "IMPORTS",
    "INCLUDES",
    "INSTANCE",
    "INSTRUCTIONS",
    "INTEGER",
    "INTERSECTION",
    "ISO646String",
    "MAX",
    "MIN",
    "MINUS-INFINITY",
    "NULL",
    "NumericString",
    "OBJECT",
    "OCTET",
    "OF",
    "OPTIONAL",
    "ObjectDescriptor",
    "PATTERN",
    "PDV",
    "PLUS-INFINITY",
    "PRESENT",
    "PRIVATE",
    "PrintableString",
    "REAL",
    "RELATIVE-OID",
    "SEQUENCE",
    "SET",
    "SIZE",
    "STRING",
    "SYNTAX",
    "T61String",
    "TAGS",
    "TRUE",
    "TeletexString",
    "UNION",
    "UNIQUE",
    "UNIVERSAL",
    "UTCTime",
    "UTF8String",
    "UniversalString",
    "VideotexString",
    "VisibleString",
    "WITH",
];
const CLASSES: [&str; 4] = ["UNIV", "APPL", "CONT", "PRIV"];
const METHODS: [&str; 2] = ["PRIM", "CONS"];
const TAGS: [&str; 32] = [
    "EOC", "BOOL", "INT", "BTS", "OTS", "NULL", "OID", "ODE", "EXT", "REAL", "ENUM", "EPDV",
    "UTF8STR", "RELOID", "(null)", "(null)", "SEQ", "SET", "NUMSTR", "PRNSTR", "TEXSTR", "VIDSTR",
    "IA5STR", "UNITIM", "GENTIM", "GRASTR", "VISSTR", "GENSTR", "UNISTR", "CHRSTR", "BMPSTR",
    "(null)",
];

struct Failure {
    code: u8,
    message: Vec<u8>,
}
type Result<T> = std::result::Result<T, Failure>;

impl Failure {
    fn path(path: &[u8], message: impl AsRef<[u8]>) -> Self {
        let mut bytes = path.to_vec();
        bytes.extend_from_slice(message.as_ref());
        Self {
            code: 1,
            message: bytes,
        }
    }
    fn io(path: &OsStr, error: io::Error) -> Self {
        let text = error.to_string();
        let suffix = error.raw_os_error().map(|n| format!(" (os error {n})"));
        let text = suffix
            .as_deref()
            .and_then(|s| text.strip_suffix(s))
            .unwrap_or(&text);
        Self::path(path.as_bytes(), format!(": {text}\n"))
    }
}

/// Defer stream errors to the checkpoints used by the original compiler.
struct Output {
    writer: BufWriter<File>,
    path: OsString,
    error: Option<io::Error>,
}

impl Output {
    fn create(path: &OsStr) -> Result<Self> {
        let file = File::create(path).map_err(|error| Failure::io(path, error))?;
        // stdio sizes regular-file buffers from the filesystem block size.
        let capacity = file
            .metadata()
            .map_or(4096, |m| m.blksize().min(8192) as usize);
        Ok(Self {
            writer: BufWriter::with_capacity(capacity, file),
            path: path.to_owned(),
            error: None,
        })
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        let _ = self.write_all(bytes);
    }

    fn check(&mut self) -> Result<()> {
        if let Some(error) = self.error.take() {
            Err(Failure::io(&self.path, error))
        } else {
            Ok(())
        }
    }

    fn finish(mut self) -> Result<()> {
        let _ = self.flush();
        self.check()
    }
}

impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if let Err(error) = self.writer.write_all(bytes) {
            if self.error.is_none() {
                self.error = Some(error);
            }
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Err(error) = self.writer.flush() {
            if self.error.is_none() {
                self.error = Some(error);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Directive,
    TypeName,
    ElementName,
    Number,
    Punctuation,
}
#[derive(Clone)]
struct Token {
    line: u16,
    kind: TokenKind,
    text: String,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Compound {
    Primitive,
    Set,
    SetOf,
    Sequence,
    SequenceOf,
    Choice,
    Any,
    Reference,
    Override,
}
#[derive(Clone)]
struct Element {
    definition: Option<usize>,
    name: Option<String>,
    compound: Compound,
    class: u8,
    constructed: bool,
    tag: u8,
    implicit: bool,
    explicit: bool,
    tag_specified: bool,
    skippable: bool,
    conditional: bool,
    rendered: bool,
    children: Vec<usize>,
    reference: Option<usize>,
    action: Option<String>,
    entry: usize,
}
impl Element {
    fn new(name: Option<String>, tag: u8) -> Self {
        Self {
            definition: None,
            name,
            compound: Compound::Primitive,
            class: ASN1_UNIV as u8,
            constructed: false,
            tag,
            implicit: false,
            explicit: false,
            tag_specified: false,
            skippable: false,
            conditional: false,
            rendered: false,
            children: Vec::new(),
            reference: None,
            action: None,
            entry: 0,
        }
    }
}
struct Type {
    token: usize,
    element: usize,
    references: usize,
}
struct Grammar {
    filename: Vec<u8>,
    tokens: Vec<Token>,
    types: Vec<Type>,
    type_index: Vec<usize>,
    elements: Vec<Element>,
    actions: BTreeSet<String>,
    verbose: bool,
    log: Vec<u8>,
}

fn token_tag(token: &str) -> u8 {
    match token {
        "BOOLEAN" => ASN1_BOOL as u8,
        "INTEGER" => ASN1_INT as u8,
        "BIT" => ASN1_BTS as u8,
        "OCTET" => ASN1_OTS as u8,
        "NULL" => ASN1_NULL as u8,
        "OBJECT" => ASN1_OID as u8,
        "ObjectDescriptor" => ASN1_ODE as u8,
        "EXTERNAL" => ASN1_EXT as u8,
        "REAL" => ASN1_REAL as u8,
        "ENUMERATED" => ASN1_ENUM as u8,
        "UTF8String" => ASN1_UTF8STR as u8,
        "RELATIVE-OID" => ASN1_RELOID as u8,
        "SEQUENCE" => ASN1_SEQ as u8,
        "SET" => ASN1_SET as u8,
        "NumericString" => ASN1_NUMSTR as u8,
        "PrintableString" => ASN1_PRNSTR as u8,
        "T61String" | "TeletexString" => ASN1_TEXSTR as u8,
        "VideotexString" => ASN1_VIDSTR as u8,
        "IA5String" => ASN1_IA5STR as u8,
        // Preserve the compiler's historical UniversalString mapping.
        "UTCTime" | "UniversalString" => ASN1_UNITIM as u8,
        "GeneralizedTime" => ASN1_GENTIM as u8,
        "GraphicString" => ASN1_GRASTR as u8,
        "VisibleString" => ASN1_VISSTR as u8,
        "GeneralString" => ASN1_GENSTR as u8,
        "CHARACTER" => ASN1_CHRSTR as u8,
        "BMPString" => ASN1_BMPSTR as u8,
        _ => 0,
    }
}

impl Grammar {
    fn new(filename: &[u8], verbose: bool) -> Self {
        Self {
            filename: filename.to_vec(),
            tokens: Vec::new(),
            types: Vec::new(),
            type_index: Vec::new(),
            elements: Vec::new(),
            actions: BTreeSet::new(),
            verbose,
            log: Vec::new(),
        }
    }
    fn eof(&self) -> Failure {
        Failure::path(&self.filename, b": Unexpectedly hit EOF\n")
    }
    fn error(&self, cursor: usize, prefix: &str, suffix: &str) -> Failure {
        if let Some(token) = self.tokens.get(cursor) {
            Failure::path(
                &self.filename,
                format!(":{}: {prefix}'{}'{suffix}\n", token.line, token.text),
            )
        } else {
            self.eof()
        }
    }
    fn require(&self, cursor: usize, end: usize) -> Result<()> {
        if cursor >= end {
            Err(self.eof())
        } else {
            Ok(())
        }
    }
    fn tokenise(&mut self, contents: &[u8]) -> Result<()> {
        for (number, original) in contents.split(|&b| b == b'\n').enumerate() {
            let mut line = original.to_vec();
            let mut limit = line.len();
            let mut scan = 0;
            // The C implementation keeps the original end-of-line boundary
            // when it moves the text after a delimited comment.
            while scan + 1 < limit {
                if line[scan..scan + 2] != *b"--" {
                    scan += 1;
                    continue;
                }
                if let Some(offset) = line[scan + 2..limit].windows(2).position(|s| s == b"--") {
                    let end = scan + 2 + offset + 2;
                    if end == limit {
                        // Avoid the original's non-progressing memmove loop.
                        limit = scan;
                        break;
                    }
                    line.copy_within(end..limit, scan);
                } else {
                    limit = scan;
                    break;
                }
            }
            let mut cursor = 0;
            while cursor < limit {
                if line[cursor].is_ascii_whitespace() || line[cursor] == 0x0b {
                    cursor += 1;
                    continue;
                }
                let start = cursor;
                let kind;
                if line[cursor].is_ascii_alphabetic() {
                    cursor += 1;
                    while cursor < limit
                        && (line[cursor].is_ascii_alphanumeric()
                            || matches!(line[cursor], b'-' | b'_'))
                    {
                        cursor += 1;
                    }
                    let size = (cursor - start) as u8 as usize;
                    let text = String::from_utf8(line[start..start + size].to_vec()).unwrap();
                    kind = if text.as_bytes().first().is_some_and(u8::is_ascii_lowercase) {
                        TokenKind::ElementName
                    } else if DIRECTIVES.contains(&text.as_str()) {
                        TokenKind::Directive
                    } else {
                        TokenKind::TypeName
                    };
                    self.tokens.push(Token {
                        line: (number + 1) as u16,
                        kind,
                        text,
                    });
                    continue;
                } else if line[cursor].is_ascii_digit() {
                    cursor += 1;
                    while cursor < limit && line[cursor].is_ascii_digit() {
                        cursor += 1;
                    }
                    kind = TokenKind::Number;
                } else {
                    let rest = &line[cursor..limit];
                    let size = if rest.starts_with(b"::=") {
                        3
                    } else if rest.starts_with(b"({") || rest.starts_with(b"})") {
                        2
                    } else if matches!(line[cursor], b'{' | b'}' | b'[' | b']' | b',') {
                        1
                    } else {
                        let mut message =
                            format!(":{}: Unknown character in grammar: '", number + 1)
                                .into_bytes();
                        message.extend_from_slice(&[line[cursor], b'\'', b'\n']);
                        return Err(Failure::path(&self.filename, message));
                    };
                    cursor += size;
                    kind = TokenKind::Punctuation;
                }
                let size = (cursor - start) as u8 as usize;
                let text = String::from_utf8(line[start..start + size].to_vec()).unwrap();
                self.tokens.push(Token {
                    line: (number + 1) as u16,
                    kind,
                    text,
                });
            }
        }
        if self.verbose {
            let _ = writeln!(self.log, "Extracted {} tokens", self.tokens.len());
        }
        Ok(())
    }
    fn build_types(&mut self) -> Result<()> {
        for (index, pair) in self.tokens.windows(2).enumerate() {
            if pair[0].kind == TokenKind::TypeName && pair[1].text == "::=" {
                self.types.push(Type {
                    token: index,
                    element: 0,
                    references: 0,
                });
            }
        }
        if self.types.is_empty() {
            return Err(Failure::path(&self.filename, b": No defined types\n"));
        }
        self.type_index = (0..self.types.len()).collect();
        self.type_index.sort_by_key(|&i| {
            let name = &self.tokens[self.types[i].token].text;
            (name.len(), name.clone())
        });
        if self.verbose {
            let _ = writeln!(self.log, "Extracted {} types", self.types.len());
        }
        Ok(())
    }
    fn lookup(&self, name: &str) -> Option<usize> {
        let mut left = 0;
        let mut right = self.type_index.len();
        while left < right {
            let middle = (left + right) / 2;
            let index = self.type_index[middle];
            let candidate = &self.tokens[self.types[index].token].text;
            match (name.len(), name).cmp(&(candidate.len(), candidate.as_str())) {
                std::cmp::Ordering::Equal => return Some(index),
                std::cmp::Ordering::Less => right = middle,
                std::cmp::Ordering::Greater => left = middle + 1,
            }
        }
        None
    }
    fn alloc(&mut self, name: Option<String>, tag: u8) -> usize {
        let index = self.elements.len();
        self.elements.push(Element::new(name, tag));
        index
    }
    fn parse(&mut self) -> Result<()> {
        for index in 0..self.types.len() {
            let end = self
                .types
                .get(index + 1)
                .map_or(self.tokens.len(), |ty| ty.token);
            let mut cursor = self.types[index].token + 2;
            let element = self.parse_type(&mut cursor, end, None)?;
            self.types[index].element = element;
            self.elements[element].definition = Some(index);
            if cursor != end {
                return Err(self.error(cursor, "Parse error at token ", ""));
            }
        }
        if self.verbose {
            let _ = writeln!(self.log, "Extracted {} actions", self.actions.len());
        }
        Ok(())
    }
    fn parse_type(
        &mut self,
        cursor: &mut usize,
        end: usize,
        name: Option<String>,
    ) -> Result<usize> {
        self.require(*cursor, end)?;
        let top = self.alloc(name.clone(), token_tag(&self.tokens[*cursor].text));
        let mut element = top;
        let mut labelled = false;
        if self.tokens[*cursor].text == "[" {
            *cursor += 1;
            self.require(*cursor, end)?;
            self.elements[top].class = match self.tokens[*cursor].text.as_str() {
                "UNIVERSAL" => {
                    *cursor += 1;
                    ASN1_UNIV as u8
                }
                "APPLICATION" => {
                    *cursor += 1;
                    ASN1_APPL as u8
                }
                "PRIVATE" => {
                    *cursor += 1;
                    ASN1_PRIV as u8
                }
                _ if self.tokens[*cursor].kind == TokenKind::Number => ASN1_CONT as u8,
                _ => return Err(self.error(*cursor, "Unrecognised tag class token ", "")),
            };
            self.require(*cursor, end)?;
            if self.tokens[*cursor].kind != TokenKind::Number {
                return Err(self.error(*cursor, "Missing tag number ", ""));
            }
            self.elements[top].tag = self.tokens[*cursor]
                .text
                .parse::<usize>()
                .unwrap_or(usize::MAX) as u8;
            self.elements[top].tag_specified = true;
            *cursor += 1;
            self.require(*cursor, end)?;
            if self.tokens[*cursor].text != "]" {
                return Err(self.error(*cursor, "Missing closing square bracket ", ""));
            }
            *cursor += 1;
            self.require(*cursor, end)?;
            labelled = true;
        }
        if self.tokens[*cursor].text == "IMPLICIT" || self.tokens[*cursor].text == "EXPLICIT" {
            self.elements[top].implicit = self.tokens[*cursor].text == "IMPLICIT";
            self.elements[top].explicit = !self.elements[top].implicit;
            *cursor += 1;
            self.require(*cursor, end)?;
        }
        if labelled {
            self.elements[top].constructed = !self.elements[top].implicit;
            self.elements[top].compound = if self.elements[top].implicit {
                Compound::Override
            } else {
                Compound::Sequence
            };
            element = self.alloc(name, token_tag(&self.tokens[*cursor].text));
            self.elements[top].children.push(element);
        }
        let token = self.tokens[*cursor].clone();
        match token.text.as_str() {
            "ANY" => {
                self.elements[element].compound = Compound::Any;
                *cursor += 1;
            }
            "NULL" | "BOOLEAN" | "ENUMERATED" | "INTEGER" | "BMPString" | "GeneralString"
            | "GraphicString" | "IA5String" | "ISO646String" | "NumericString"
            | "PrintableString" | "T61String" | "TeletexString" | "UniversalString"
            | "UTF8String" | "VideotexString" | "VisibleString" | "ObjectDescriptor"
            | "GeneralizedTime" | "UTCTime" | "EXTERNAL" => {
                self.elements[element].constructed = token.text == "EXTERNAL";
                *cursor += 1;
            }
            "BIT" | "OCTET" | "OBJECT" => {
                *cursor += 1;
                self.require(*cursor, end)?;
                let expected = if token.text == "OBJECT" {
                    "IDENTIFIER"
                } else {
                    "STRING"
                };
                if self.tokens[*cursor].text != expected {
                    return Err(self.error(*cursor, "Unexpected token ", ""));
                }
                *cursor += 1;
            }
            "CHOICE" => {
                self.elements[element].compound = Compound::Choice;
                *cursor += 1;
                let children = self.parse_compound(cursor, end, true)?;
                self.elements[element].children = children;
            }
            "SEQUENCE" | "SET" => {
                let sequence = token.text == "SEQUENCE";
                self.elements[element].compound = if sequence {
                    Compound::Sequence
                } else {
                    Compound::Set
                };
                self.elements[element].constructed = true;
                *cursor += 1;
                self.require(*cursor, end)?;
                let children = if self.tokens[*cursor].text == "OF" {
                    self.elements[element].compound = if sequence {
                        Compound::SequenceOf
                    } else {
                        Compound::SetOf
                    };
                    *cursor += 1;
                    self.require(*cursor, end)?;
                    vec![self.parse_type(cursor, end, None)?]
                } else {
                    self.parse_compound(cursor, end, !sequence)?
                };
                self.elements[element].children = children;
            }
            _ if token.kind == TokenKind::TypeName => {
                let reference = self
                    .lookup(&token.text)
                    .ok_or_else(|| self.error(*cursor, "Type ", " undefined"))?;
                self.types[reference].references += 1;
                self.elements[element].compound = Compound::Reference;
                self.elements[element].reference = Some(reference);
                *cursor += 1;
            }
            _ => return Err(self.error(*cursor, "Token ", " does not introduce a type")),
        }
        if *cursor < end && matches!(self.tokens[*cursor].text.as_str(), "OPTIONAL" | "DEFAULT") {
            self.elements[top].skippable = true;
            *cursor += 1;
        }
        if *cursor < end && self.tokens[*cursor].text == "({" {
            *cursor += 1;
            self.require(*cursor, end)?;
            if self.tokens[*cursor].kind != TokenKind::ElementName {
                return Err(self.error(*cursor, "Token ", " is not an action function name"));
            }
            let action = self.tokens[*cursor].text.clone();
            self.actions.insert(action.clone());
            self.elements[element].action = Some(action);
            *cursor += 1;
            self.require(*cursor, end)?;
            if self.tokens[*cursor].text != "})" {
                return Err(self.error(*cursor, "Missing close action, got ", ""));
            }
            *cursor += 1;
        }
        Ok(top)
    }
    fn parse_compound(
        &mut self,
        cursor: &mut usize,
        end: usize,
        alternates: bool,
    ) -> Result<Vec<usize>> {
        self.require(*cursor, end)?;
        if self.tokens[*cursor].text != "{" {
            return Err(self.error(*cursor, "Expected compound to start with brace not ", ""));
        }
        *cursor += 1;
        self.require(*cursor, end)?;
        if self.tokens[*cursor].text == "{" {
            return Err(Failure::path(
                &self.filename,
                format!(":{}: Empty compound\n", self.tokens[*cursor].line),
            ));
        }
        let mut children = Vec::new();
        loop {
            let name = if self.tokens[*cursor].kind == TokenKind::ElementName {
                let name = self.tokens[*cursor].text.clone();
                *cursor += 1;
                self.require(*cursor, end)?;
                Some(name)
            } else {
                None
            };
            let child = self.parse_type(cursor, end, name)?;
            if alternates {
                self.elements[child].skippable = true;
                self.elements[child].conditional = !children.is_empty();
            }
            children.push(child);
            self.require(*cursor, end)?;
            if self.tokens[*cursor].text != "," {
                break;
            }
            *cursor += 1;
            self.require(*cursor, end)?;
        }
        if self.tokens[*cursor].text != "}" {
            return Err(self.error(*cursor, "Expected compound closure, got ", ""));
        }
        *cursor += 1;
        Ok(children)
    }
    fn dump(&mut self, index: usize, level: usize, visiting: &mut HashSet<usize>) -> Result<()> {
        if !visiting.insert(index) {
            return Err(Failure::path(
                &self.filename,
                b": Recursive type cannot be displayed\n",
            ));
        }
        let e = self.elements[index].clone();
        let type_name = e
            .definition
            .map_or(".", |ty| self.tokens[self.types[ty].token].text.as_str());
        let tag = if e.class == 0 && !e.constructed && e.tag == 0 {
            "<...>".to_owned()
        } else if e.class == ASN1_UNIV as u8 {
            format!(
                "{} {} {}",
                CLASSES[e.class as usize],
                METHODS[e.constructed as usize],
                TAGS.get(e.tag as usize).unwrap_or(&"(null)")
            )
        } else {
            format!(
                "{} {} {}",
                CLASSES[e.class as usize], METHODS[e.constructed as usize], e.tag
            )
        };
        let mark = |enabled, c| if enabled { c } else { '-' };
        let compound = match e.compound {
            Compound::Primitive => '-',
            Compound::Set => 't',
            Compound::SetOf => 'T',
            Compound::Sequence => 'q',
            Compound::SequenceOf => 'Q',
            Compound::Choice => 'c',
            Compound::Any => 'a',
            Compound::Reference => 'r',
            Compound::Override => 'o',
        };
        let _ = writeln!(
            self.log,
            "{}{}{}{}{} {} {:level$}[*] \x1b[33m{}\x1b[m {} {} \x1b[35m{}\x1b[m",
            mark(e.implicit, 'I'),
            mark(e.explicit, 'E'),
            mark(e.tag_specified, 'T'),
            mark(e.skippable, 'S'),
            mark(e.conditional, 'C'),
            compound,
            "",
            tag,
            type_name,
            e.name.as_deref().unwrap_or("."),
            e.action.as_deref().unwrap_or("")
        );
        if let Some(reference) = e.reference {
            self.dump(self.types[reference].element, level + 3, visiting)?;
        } else {
            for child in e.children {
                self.dump(child, level + 3, visiting)?;
            }
        }
        visiting.remove(&index);
        Ok(())
    }
}

struct Renderer<'a> {
    grammar: &'a mut Grammar,
    output: &'a mut Output,
    emitting: bool,
    entries: usize,
    depth: usize,
    queue: VecDeque<usize>,
    visiting: HashSet<usize>,
}
impl Renderer<'_> {
    fn more(&mut self, text: impl AsRef<[u8]>) {
        if self.emitting {
            self.output.extend_from_slice(text.as_ref());
        }
    }
    fn opcode(&mut self, text: impl AsRef<[u8]>) {
        if self.emitting {
            let _ = write!(
                self.output,
                "\t[{:4}] ={:width$}",
                self.entries,
                "",
                width = self.depth
            );
            self.output.extend_from_slice(text.as_ref());
        }
        self.entries += 1;
    }
    fn render_element(&mut self, index: usize, mut tag: Option<usize>) -> Result<()> {
        if !self.visiting.insert(index) {
            return Err(Failure::path(
                &self.grammar.filename,
                b": Recursive type cannot be rendered\n",
            ));
        }
        let e = self.grammar.elements[index].clone();
        let tagged = tag.map(|tag| &self.grammar.elements[tag]);
        let skippable = e.skippable || tagged.is_some_and(|t| t.skippable);
        let conditional = e.conditional || tagged.is_some_and(|t| t.conditional);
        let out_of_line = skippable
            || e.definition
                .is_some_and(|ty| self.grammar.types[ty].references > 1);
        let condition = if conditional { "COND_" } else { "" };
        let action = if e.action.is_some() { "_ACT" } else { "" };
        let skip = if skippable { "_OR_SKIP" } else { "" };
        if let Some(ty) = e.definition {
            self.more(format!(
                "\t// {}\n",
                self.grammar.tokens[self.grammar.types[ty].token].text
            ));
        }
        let mut emit_tag = true;
        match e.compound {
            Compound::Any => {
                self.opcode(format!("ASN1_OP_{condition}MATCH_ANY{action}{skip},"));
                if let Some(name) = &e.name {
                    self.more(format!("\t\t// {name}"));
                }
                self.more("\n");
                emit_tag = false;
            }
            Compound::Override => {
                self.render_element(e.children[0], Some(index))?;
                self.visiting.remove(&index);
                return Ok(());
            }
            Compound::Sequence | Compound::SequenceOf | Compound::Set | Compound::SetOf => {
                let jump = if out_of_line { "_JUMP" } else { "" };
                self.opcode(format!("ASN1_OP_{condition}MATCH{jump}{skip},"));
            }
            Compound::Choice => emit_tag = false,
            Compound::Reference if e.class == 0 && !e.constructed && e.tag == 0 => emit_tag = false,
            _ => self.opcode(format!("ASN1_OP_{condition}MATCH{action}{skip},")),
        }
        if emit_tag {
            let label = tag.unwrap_or(index);
            if let Some(name) = &self.grammar.elements[label].name {
                self.more(format!("\t\t// {name}"));
            }
            self.more("\n");
            let tag_index = tag
                .filter(|&t| self.grammar.elements[t].tag_specified)
                .unwrap_or(index);
            let t = &self.grammar.elements[tag_index];
            let class = CLASSES[t.class as usize];
            let method = METHODS[(t.constructed || e.constructed) as usize];
            if t.class == ASN1_UNIV as u8 && !matches!(t.tag, 14 | 15 | 31) {
                self.opcode(format!(
                    "_tag({class}, {method}, {}),\n",
                    TAGS.get(t.tag as usize).unwrap_or(&"(null)")
                ));
            } else {
                self.opcode(format!("_tagn({class}, {method}, {:2}),\n", t.tag));
            }
            tag = None;
        }
        match e.compound {
            Compound::Reference => {
                let reference = self.grammar.types[e.reference.unwrap()].element;
                self.render_element(reference, tag)?;
                if e.action.is_some() {
                    self.opcode(format!(
                        "ASN1_OP_{}ACT,\n",
                        if skippable { "MAYBE_" } else { "" }
                    ));
                }
            }
            Compound::Sequence | Compound::SequenceOf | Compound::SetOf if out_of_line => {
                self.opcode(format!("_jump_target({}),", e.entry));
                if let Some(ty) = e.definition {
                    self.more(format!(
                        "\t\t// --> {}",
                        self.grammar.tokens[self.grammar.types[ty].token].text
                    ));
                }
                self.more("\n");
                if !self.grammar.elements[index].rendered {
                    self.grammar.elements[index].rendered = true;
                    self.queue.push_back(index);
                }
                self.visiting.remove(&index);
                return Ok(());
            }
            Compound::Sequence | Compound::SequenceOf | Compound::SetOf => {
                let entry = self.entries;
                self.depth += 1;
                for child in e.children {
                    self.render_element(child, None)?;
                }
                self.depth -= 1;
                let op = match e.compound {
                    Compound::Sequence => "SEQ",
                    Compound::SequenceOf => "SEQ_OF",
                    _ => "SET_OF",
                };
                self.opcode(format!("ASN1_OP_END_{op}{action},\n"));
                if e.compound != Compound::Sequence {
                    self.opcode(format!("_jump_target({entry}),\n"));
                }
            }
            Compound::Set => {
                return Err(Failure::path(
                    b"",
                    b"The ASN.1 SET type is not currently supported.\n",
                ))
            }
            Compound::Choice => {
                for child in e.children {
                    self.render_element(child, Some(child))?;
                }
                if !skippable {
                    self.opcode("ASN1_OP_COND_FAIL,\n");
                }
                if e.action.is_some() {
                    self.opcode("ASN1_OP_ACT,\n");
                }
            }
            _ => {}
        }
        if let Some(action) = &e.action {
            self.opcode(format!("_action(ACT_{action}),\n"));
        }
        self.visiting.remove(&index);
        Ok(())
    }
    fn render_queue(&mut self) -> Result<()> {
        while let Some(index) = self.queue.pop_front() {
            self.more("\n");
            let entry = self.entries;
            self.grammar.elements[index].entry = entry;
            let e = self.grammar.elements[index].clone();
            self.depth += 1;
            for child in e.children {
                self.render_element(child, None)?;
            }
            self.depth -= 1;
            let action = if e.action.is_some() { "_ACT" } else { "" };
            let op = match e.compound {
                Compound::Sequence => "SEQ",
                Compound::SequenceOf => "SEQ_OF",
                Compound::Set => "SET",
                Compound::SetOf => "SET_OF",
                _ => "",
            };
            if !op.is_empty() {
                self.opcode(format!("ASN1_OP_END_{op}{action},\n"));
            }
            if matches!(e.compound, Compound::SequenceOf | Compound::SetOf) {
                self.opcode(format!("_jump_target({entry}),\n"));
            }
            if let Some(action) = e.action {
                self.opcode(format!("_action(ACT_{action}),\n"));
            }
            self.opcode("ASN1_OP_RETURN,\n");
        }
        Ok(())
    }
    fn name(&mut self, before: &[u8], name: &[u8], after: &[u8]) {
        self.more(before);
        self.more(name);
        self.more(after);
    }
    fn render(&mut self, name: &[u8], header: &mut Output) -> Result<()> {
        let banner = b"/*\n * Automatically generated by asn1_compiler.  Do not edit\n *\n * ASN.1 parser for ";
        header.extend_from_slice(banner);
        header.extend_from_slice(name);
        header.extend_from_slice(
            b"\n */\n#include <linux/asn1_decoder.h>\n\nextern const struct asn1_decoder ",
        );
        header.extend_from_slice(name);
        header.extend_from_slice(b"_decoder;\n");
        header.check()?;
        self.name(
            banner,
            name,
            b"\n */\n#include <linux/asn1_ber_bytecode.h>\n",
        );
        self.name(b"#include \"", name, b".asn1.h\"\n\n");
        self.output.check()?;
        header.extend_from_slice(b"\n");
        let actions: Vec<_> = self.grammar.actions.iter().cloned().collect();
        for action in &actions {
            let _ = writeln!(
                header,
                "extern int {action}(void *, size_t, unsigned char, const void *, size_t);"
            );
        }
        header.extend_from_slice(b"\n");
        self.name(b"enum ", name, b"_actions {\n");
        for (index, action) in actions.iter().enumerate() {
            self.more(format!("\tACT_{action} = {},\n", index as u8));
        }
        self.name(
            b"\tNR__",
            name,
            format!("_actions = {}\n}};\n\n", actions.len()).as_bytes(),
        );
        self.name(b"static const asn1_action_t ", name, b"_action_table[NR__");
        self.name(b"", name, b"_actions] = {\n");
        for (index, action) in actions.iter().enumerate() {
            self.more(format!("\t[{:4}] = {action},\n", index as u8));
        }
        self.more("};\n");
        self.output.check()?;
        if self.grammar.verbose {
            self.grammar.log.extend_from_slice(b"Pass 1\n");
        }
        self.emitting = false;
        self.render_element(self.grammar.types[0].element, None)?;
        self.opcode("ASN1_OP_COMPLETE,\n");
        self.render_queue()?;
        for element in &mut self.grammar.elements {
            element.rendered = false;
        }
        if self.grammar.verbose {
            self.grammar.log.extend_from_slice(b"Pass 2\n");
        }
        self.emitting = true;
        self.entries = 0;
        self.name(b"\nstatic const unsigned char ", name, b"_machine[] = {\n");
        self.render_element(self.grammar.types[0].element, None)?;
        self.opcode("ASN1_OP_COMPLETE,\n");
        self.render_queue()?;
        self.more("};\n\n");
        self.name(b"const struct asn1_decoder ", name, b"_decoder = {\n");
        self.name(b"\t.machine = ", name, b"_machine,\n");
        self.name(b"\t.machlen = sizeof(", name, b"_machine),\n");
        self.name(b"\t.actions = ", name, b"_action_table,\n};\n");
        Ok(())
    }
}

fn read_file(path: &OsStr) -> Result<Vec<u8>> {
    let mut file = File::open(path).map_err(|e| Failure::io(path, e))?;
    let size = file.metadata().map_err(|e| Failure::io(path, e))?.len();
    let size =
        usize::try_from(size).map_err(|_| Failure::path(b"", b"Cannot allocate memory\n"))?;
    let mut contents = Vec::new();
    contents
        .try_reserve_exact(size)
        .map_err(|_| Failure::path(b"", b"Cannot allocate memory\n"))?;
    contents.resize(size, 0);
    if file.read(&mut contents).map_err(|e| Failure::io(path, e))? != size {
        return Err(Failure::path(path.as_bytes(), b": Short read\n"));
    }
    Ok(contents)
}
fn run(args: &[OsString], log: &mut Vec<u8>) -> Result<()> {
    let mut verbose =
        std::env::var_os("KBUILD_VERBOSE").is_some_and(|v| v.as_bytes().contains(&b'1'));
    let mut debug = false;
    let mut first = 1;
    while args.len() - first > 3 {
        match args[first].as_bytes() {
            b"-v" => verbose = true,
            b"-d" => debug = true,
            _ => break,
        }
        first += 1;
    }
    if args.len() - first != 3 {
        let mut failure = Failure::path(b"Format: ", args[0].as_bytes());
        failure
            .message
            .extend_from_slice(b" [-v] [-d] <grammar-file> <c-file> <hdr-file>\n");
        failure.code = 2;
        return Err(failure);
    }
    let filename = &args[first];
    let outputname = &args[first + 1];
    let headername = &args[first + 2];
    let contents = read_file(filename)?;
    let mut grammar = Grammar::new(filename.as_bytes(), verbose);
    let result = (|| {
        grammar.tokenise(&contents)?;
        grammar.build_types()?;
        grammar.parse()?;
        if debug {
            grammar.dump(grammar.types[0].element, 0, &mut HashSet::new())?;
        }
        let mut output = Output::create(outputname)?;
        let mut header = Output::create(headername)?;
        let name = filename
            .as_bytes()
            .rsplit(|&b| b == b'/')
            .next()
            .unwrap_or(b"");
        let name = name.split(|&b| b == b'.').next().unwrap_or(b"");
        let mut renderer = Renderer {
            grammar: &mut grammar,
            output: &mut output,
            emitting: true,
            entries: 0,
            depth: 1,
            queue: VecDeque::new(),
            visiting: HashSet::new(),
        };
        renderer.render(name, &mut header)?;
        output.finish()?;
        header.finish()?;
        Ok(())
    })();
    log.extend_from_slice(&grammar.log);
    result
}
fn main() -> ExitCode {
    let mut log = Vec::new();
    let result = run(&std::env::args_os().collect::<Vec<_>>(), &mut log);
    let _ = io::stdout().lock().write_all(&log);
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            let _ = io::stderr().lock().write_all(&failure.message);
            ExitCode::from(failure.code)
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
