// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
//! Owned Unicode records shared by the loader, trie builder and verifier.

pub(super) const LIMIT: usize = 0x110000;
pub(super) const HANGUL: u8 = 255;

#[derive(Clone, Default)]
pub(super) struct Record {
    pub code: u32,
    pub ccc: i32,
    pub generation: i32,
    pub correction: u32,
    // None means identity; Some(empty) means default ignorable. Terminators are
    // represented by the slice length, rather than copied into owned buffers.
    pub utf32_nfdi: Option<Vec<u32>>,
    pub utf32_cf: Option<Vec<u32>>,
    pub nfdi: Option<Vec<u8>>,
    pub cf: Option<Vec<u8>>,
}

pub(super) struct Database {
    // The first LIMIT records are indexed by code point. Historical corrections
    // follow them in source order, allowing leaves to use one stable index type.
    pub records: Vec<Record>,
    pub ages: Vec<u32>,
    pub max_age: u32,
}

impl Database {
    pub(super) fn new() -> Self {
        Self {
            records: (0..LIMIT)
                .map(|code| Record {
                    code: code as u32,
                    ..Record::default()
                })
                .collect(),
            ages: Vec::new(),
            max_age: 0,
        }
    }

    pub(super) fn record_at_age(&self, code: usize, age: u32) -> usize {
        self.records[LIMIT..]
            .iter()
            .position(|r| r.code as usize == code)
            .map(|i| i + LIMIT)
            .filter(|&i| self.records[i].correction > age)
            .unwrap_or(code)
    }
}

pub(super) fn is_hangul(code: u32) -> bool {
    (0xac00..=0xd7a3).contains(&code)
}

// Unlike char::encode_utf8, this also encodes the surrogate interval so the
// verifier can prove that those bit patterns are absent from the packed trie.
pub(super) fn encode(code: u32, out: &mut Vec<u8>) {
    match code {
        0..=0x7f => out.push(code as u8),
        0x80..=0x7ff => out.extend([(0xc0 | (code >> 6)) as u8, (0x80 | (code & 63)) as u8]),
        0x800..=0xffff => out.extend([
            (0xe0 | (code >> 12)) as u8,
            (0x80 | ((code >> 6) & 63)) as u8,
            (0x80 | (code & 63)) as u8,
        ]),
        0x10000..=0x10ffff => out.extend([
            (0xf0 | (code >> 18)) as u8,
            (0x80 | ((code >> 12) & 63)) as u8,
            (0x80 | ((code >> 6) & 63)) as u8,
            (0x80 | (code & 63)) as u8,
        ]),
        _ => crate::output(format!("{code:#x}: illegal val\n").as_bytes()),
    }
}

pub(super) fn hangul(code: u32) -> Vec<u8> {
    let index = code - 0xac00;
    let mut bytes = Vec::with_capacity(9);
    encode(0x1100 + index / 588, &mut bytes);
    encode(0x1161 + (index % 588) / 28, &mut bytes);
    if index % 28 != 0 {
        encode(0x11a7 + index % 28, &mut bytes);
    }
    bytes
}
