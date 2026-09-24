// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
//! Packed-trie verification and the original normalization cursor.

use super::model::{encode, is_hangul, Database, HANGUL, LIMIT};
use super::trie::Forest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn clen(byte: u8) -> usize {
    1 + usize::from(byte >= 0xc0) + usize::from(byte >= 0xe0) + usize::from(byte >= 0xf0)
}

/// A saved position owns its decomposition, so a later Hangul lookup cannot
/// overwrite a decomposition to which a CCC scan will return.
#[derive(Clone, Default)]
struct Position {
    input: usize,
    mapping: Option<Vec<u8>>,
    offset: usize,
}

pub(super) struct Cursor<'a> {
    forest: &'a Forest,
    database: &'a Database,
    tree: usize,
    input: &'a [u8],
    position: Position,
    saved: Position,
    len: usize,
    saved_len: usize,
    ccc: i16,
    next_ccc: i16,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(
        forest: &'a Forest,
        database: &'a Database,
        tree: usize,
        input: &'a [u8],
        len: usize,
    ) -> Result<Self, ()> {
        if tree >= forest.trees.len()
            || len > u32::MAX as usize
            || (len > 0 && input.first().is_some_and(|b| b & 0xc0 == 0x80))
        {
            return Err(());
        }
        Ok(Self {
            forest,
            database,
            tree,
            input,
            position: Position::default(),
            saved: Position::default(),
            len,
            saved_len: 0,
            ccc: 0,
            next_ccc: 0,
        })
    }

    fn remaining(&self) -> &[u8] {
        if let Some(mapping) = &self.position.mapping {
            mapping.get(self.position.offset..).unwrap_or_default()
        } else {
            self.input.get(self.position.input..).unwrap_or_default()
        }
    }

    fn advance(&mut self, count: usize) -> Result<(), ()> {
        if self.position.mapping.is_some() {
            self.position.offset = self.position.offset.checked_add(count).ok_or(())?;
        } else {
            self.len = self.len.checked_sub(count).ok_or(())?;
            self.position.input = self.position.input.checked_add(count).ok_or(())?;
        }
        Ok(())
    }

    fn next_byte(&mut self) -> Result<u8, ()> {
        loop {
            if self.position.mapping.is_some()
                && self.remaining().first().copied().unwrap_or(0) == 0
            {
                self.position.mapping = None;
                self.position.offset = 0;
            }
            let byte = self.remaining().first().copied().unwrap_or(0);
            let ccc;
            if self.position.mapping.is_none() && (self.len == 0 || byte == 0) {
                if self.ccc == 0 {
                    return Ok(0);
                }
                ccc = 0;
            } else if byte & 0xc0 == 0x80 {
                self.advance(1)?;
                return Ok(byte);
            } else {
                let bytes = self.remaining();
                let limit = if self.position.mapping.is_some() {
                    bytes.len()
                } else {
                    self.len.min(bytes.len())
                };
                let leaf = self.forest.lookup(self.tree, &bytes[..limit]).ok_or(())?;
                let age = *self
                    .database
                    .ages
                    .get(usize::from(leaf.generation))
                    .ok_or(())?;
                let mut empty = false;
                if age > self.forest.trees[self.tree].max_age {
                    ccc = 0;
                } else if leaf.ccc == 255 {
                    // The generator has already recursively expanded mappings.
                    if self.position.mapping.is_some() {
                        return Err(());
                    }
                    self.advance(clen(byte))?;
                    self.position.mapping = Some(leaf.mapping.ok_or(())?);
                    self.position.offset = 0;
                    if self.remaining().first().copied().unwrap_or(0) == 0 {
                        if self.ccc == 0 {
                            continue;
                        }
                        ccc = 0;
                        empty = true;
                    } else {
                        ccc = i16::from(
                            self.forest
                                .lookup(self.tree, self.remaining())
                                .ok_or(())?
                                .ccc,
                        );
                        if ccc == 255 {
                            return Err(());
                        }
                    }
                } else {
                    ccc = i16::from(leaf.ccc);
                }
                if !empty {
                    if ccc != 0 && self.ccc < ccc && ccc < self.next_ccc {
                        self.next_ccc = ccc;
                    }
                    if ccc == self.ccc {
                        let byte = *self.remaining().first().ok_or(())?;
                        self.advance(1)?;
                        return Ok(byte);
                    }
                }
            }
            // The four original ccc_mismatch transitions. Empty mappings are
            // stoppers even though they contribute no output bytes.
            if self.next_ccc == 0 {
                if self.ccc != 0 {
                    return Err(());
                }
                self.ccc = -1;
                self.next_ccc = ccc;
                self.saved = self.position.clone();
                self.saved_len = self.len;
                self.advance(clen(*self.remaining().first().ok_or(())?))?;
            } else if ccc != 0 {
                self.advance(clen(*self.remaining().first().ok_or(())?))?;
            } else if self.next_ccc != 255 {
                self.ccc = self.next_ccc;
                self.next_ccc = 255;
                self.position = self.saved.clone();
                self.len = self.saved_len;
            } else {
                self.ccc = 0;
                self.next_ccc = 0;
                self.saved = Position::default();
                self.saved_len = 0;
            }
        }
    }

    /// C utf8byte convention: byte > 0, NUL/end = 0, malformed = -1.
    pub(super) fn byte(&mut self) -> i32 {
        self.next_byte().map(i32::from).unwrap_or(-1)
    }
}

pub(super) fn normalize(
    forest: &Forest,
    database: &Database,
    tree: usize,
    input: &[u8],
    len: usize,
) -> Result<Vec<u8>, ()> {
    let mut cursor = Cursor::new(forest, database, tree, input, len)?;
    let mut result = Vec::new();
    loop {
        match cursor.byte() {
            -1 => return Err(()),
            0 => return Ok(result),
            byte => result.push(byte as u8),
        }
    }
}

// The C generator exports these helpers but its main path does not call them.
// Keep their independently tested counterparts without suppressing other lints.
#[allow(dead_code)]
pub(super) fn age(
    forest: &Forest,
    database: &Database,
    tree: usize,
    input: &[u8],
    len: usize,
    minimum: bool,
) -> Result<u32, ()> {
    let max_age = forest.trees.get(tree).ok_or(())?.max_age;
    let mut age = if minimum { max_age } else { 0 };
    let mut bytes = &input[..len.min(input.len())];
    while let Some(&byte) = bytes.first().filter(|&&b| b != 0) {
        let leaf = forest.lookup(tree, bytes).ok_or(())?;
        let leaf_age = *database.ages.get(usize::from(leaf.generation)).ok_or(())?;
        if leaf_age <= max_age {
            age = if minimum {
                age.min(leaf_age)
            } else {
                age.max(leaf_age)
            };
        }
        bytes = bytes.get(clen(byte)..).ok_or(())?;
    }
    Ok(age)
}

#[allow(dead_code)]
pub(super) fn normalized_len(
    forest: &Forest,
    database: &Database,
    tree: usize,
    input: &[u8],
    len: usize,
) -> Result<usize, ()> {
    let max_age = forest.trees.get(tree).ok_or(())?.max_age;
    let mut result = 0usize;
    let mut bytes = &input[..len.min(input.len())];
    while let Some(&byte) = bytes.first().filter(|&&b| b != 0) {
        let leaf = forest.lookup(tree, bytes).ok_or(())?;
        let age = *database.ages.get(usize::from(leaf.generation)).ok_or(())?;
        let count = if age <= max_age && leaf.ccc == 255 {
            leaf.mapping
                .as_ref()
                .ok_or(())?
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(leaf.mapping.as_ref().ok_or(())?.len())
        } else {
            clen(byte)
        };
        result = result.checked_add(count).ok_or(())?;
        bytes = bytes.get(clen(byte)..).ok_or(())?;
    }
    Ok(result)
}

fn cstring(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())]
}

/// As in C, discrepancies are diagnostic rather than fatal.
pub(super) fn trees_verify(forest: &Forest, database: &Database, verbose: i32) -> usize {
    let mut reports = 0;
    let mut key = Vec::with_capacity(4);
    for (index, tree) in forest.trees.iter().enumerate() {
        if verbose > 0 {
            crate::output(format!("Verifying {}_{:x}\n", tree.name(), tree.max_age).as_bytes());
        }
        for code in 0..LIMIT {
            let data = &database.records[database.record_at_age(code, tree.max_age)];
            key.clear();
            encode(code as u32, &mut key);
            let leaf = forest.lookup(index, &key);
            let surrogate = (0xd800..=0xdfff).contains(&code);
            let report = if let Some(leaf) = &leaf {
                let bad_mapping = if leaf.ccc == 255 {
                    if is_hangul(data.code) {
                        data.nfdi.as_ref().and_then(|b| b.first()) != Some(&HANGUL)
                    } else {
                        let expected = if tree.casefold {
                            data.cf.as_ref().or(data.nfdi.as_ref())
                        } else {
                            data.nfdi.as_ref()
                        };
                        match (expected, leaf.mapping.as_ref()) {
                            (Some(a), Some(b)) => cstring(a) != cstring(b),
                            _ => true,
                        }
                    }
                } else {
                    data.ccc != i32::from(leaf.ccc)
                };
                surrogate
                    || data.generation == -1
                    || data.generation != i32::from(leaf.generation)
                    || bad_mapping
            } else {
                data.generation != -1 || !surrogate
            };
            if report {
                reports += 1;
                crate::output(
                    format!(
                        "{code:X} code {:X} gen {} ccc {} nfdi -> \"",
                        data.code, data.generation, data.ccc
                    )
                    .as_bytes(),
                );
                crate::output(cstring(data.nfdi.as_deref().unwrap_or(b"(null)")));
                crate::output(b"\"");
                if let Some(leaf) = leaf {
                    crate::output(
                        format!(" gen {} ccc {} nfdi -> \"", leaf.generation, leaf.ccc).as_bytes(),
                    );
                    if leaf.ccc == 255 {
                        crate::output(cstring(leaf.mapping.as_deref().unwrap_or_default()));
                    }
                    crate::output(b"\"");
                }
                crate::output(b"\n");
            }
        }
    }
    reports
}

#[derive(Debug)]
pub(super) enum TestError {
    Open(io::Error),
    File,
}

// fgets reads at most 1023 bytes and includes a newline if encountered.
fn fgets(reader: &mut impl BufRead, line: &mut Vec<u8>) -> io::Result<bool> {
    line.clear();
    while line.len() < 1023 {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            break;
        }
        let available = buffer.len().min(1023 - line.len());
        let n = buffer[..available]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(available, |i| i + 1);
        let newline = buffer[n - 1] == b'\n';
        line.extend_from_slice(&buffer[..n]);
        reader.consume(n);
        if newline {
            break;
        }
    }
    Ok(!line.is_empty())
}

fn columns(line: &[u8]) -> Option<(&[u8], &[u8])> {
    let line = cstring(line);
    if line.first() == Some(&b'#') {
        return None;
    }
    let mut fields = line.split(|&b| b == b';');
    let first = fields.next().filter(|s| !s.is_empty())?;
    fields.next().filter(|s| !s.is_empty())?;
    let third = fields.next().filter(|s| !s.is_empty())?;
    Some((first, third))
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 11 | 12)
}

fn digit(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
        _ => None,
    }
}

// Safe strtoul(base 16) subset: all defined Unicode input values, prefixes,
// signs and whitespace. Reject a non-advancing conversion instead of hanging.
fn codepoints(mut text: &[u8]) -> Result<Vec<u32>, TestError> {
    let mut result = Vec::new();
    while !text.is_empty() {
        let mut i = 0;
        while text.get(i).is_some_and(|&b| whitespace(b)) {
            i += 1;
        }
        let negative = text.get(i) == Some(&b'-');
        if negative || text.get(i) == Some(&b'+') {
            i += 1;
        }
        if text.get(i) == Some(&b'0')
            && text.get(i + 1).is_some_and(|b| matches!(b, b'x' | b'X'))
            && text.get(i + 2).and_then(|&b| digit(b)).is_some()
        {
            i += 2;
        }
        let start = i;
        let mut value = 0 as std::ffi::c_ulong;
        let mut overflow = false;
        while let Some(d) = text.get(i).and_then(|&b| digit(b)) {
            match value
                .checked_mul(16)
                .and_then(|v| v.checked_add(std::ffi::c_ulong::from(d)))
            {
                Some(v) => value = v,
                None => overflow = true,
            }
            i += 1;
        }
        if i == start {
            return Err(TestError::File);
        }
        if overflow {
            value = std::ffi::c_ulong::MAX;
        } else if negative {
            value = value.wrapping_neg();
        }
        // The C destination is unsigned int, after unsigned long conversion.
        let value = value as u32;
        result.push(value);
        text = &text[i..];
    }
    Ok(result)
}

pub(super) fn normalization_test(
    forest: &Forest,
    database: &Database,
    test_name: &Path,
    verbose: i32,
) -> Result<(), TestError> {
    if verbose > 0 {
        crate::output(b"Parsing ");
        crate::output(test_name.as_os_str().as_bytes());
        crate::output(b"\n");
    }
    let file = File::open(test_name).map_err(TestError::Open)?;
    let mut reader = BufReader::new(file);
    let mut line = Vec::new();
    let mut tests = 0;
    let mut failures = 0;
    let tree = forest
        .trees
        .iter()
        .rposition(|t| !t.casefold)
        .ok_or(TestError::File)?;
    // C ignores ferror: an error fetching the next line terminates the loop.
    while fgets(&mut reader, &mut line).unwrap_or(false) {
        let Some((source, expected)) = columns(&line) else {
            continue;
        };
        let mut input = Vec::new();
        for code in codepoints(source)? {
            encode(code, &mut input);
        }
        let mut output = Vec::new();
        let mut ignorables = false;
        for code in codepoints(expected)? {
            if database
                .records
                .get(code as usize)
                .ok_or(TestError::File)?
                .nfdi
                .as_ref()
                .is_some_and(Vec::is_empty)
            {
                ignorables = true;
            } else {
                encode(code, &mut output);
            }
        }
        tests += 1;
        let expected_output = cstring(&output);
        // Preserve both original normalize_line passes, including its retained
        // NUL before the poison byte in the second nominally bounded pass.
        let first = normalize(forest, database, tree, &input, u32::MAX as usize);
        input.push(0);
        input.push(255);
        let second = normalize(forest, database, tree, &input, u32::MAX as usize);
        if first.as_deref() != Ok(expected_output) || second.as_deref() != Ok(expected_output) {
            crate::output(b"Line ");
            crate::output(source);
            crate::output(b" -> ");
            crate::output(expected);
            if ignorables {
                crate::output(b" (ignorables removed)");
            }
            crate::output(b" failure\n");
            failures += 1;
        }
    }
    if verbose > 0 {
        crate::output(format!("Ran {tests} tests with {failures} failures\n").as_bytes());
    }
    if failures != 0 {
        Err(TestError::File)
    } else {
        Ok(())
    }
}
