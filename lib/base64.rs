// SPDX-License-Identifier: GPL-2.0
/*
 * base64.c - Base64 with support for multiple variants
 *
 * Copyright (c) 2020 Hannes Reinecke, SUSE
 *
 * Based on the base64url routines from fs/crypto/fname.c
 * (which are using the URL-safe Base64 encoding),
 * modified to support multiple Base64 variants.
 */

// Dependencies supplied by the surrounding kernel translation.
pub type base64_variant = i32;
const BASE64_STD: usize = 0;
const BASE64_URLSAFE: usize = 1;
const BASE64_IMAP: usize = 2;

static BASE64_TABLES: [[u8; 65]; 3] = [
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0",
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_\0",
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,\0",
];

const fn make_reverse_table(ch_62: u8, ch_63: u8) -> [i8; 256] {
    let mut table = [-1i8; 256];
    let mut v = 0usize;
    while v < 256 {
        table[v] = if v >= b'A' as usize && v <= b'Z' as usize {
            (v - b'A' as usize) as i8
        } else if v >= b'a' as usize && v <= b'z' as usize {
            (v - b'a' as usize + 26) as i8
        } else if v >= b'0' as usize && v <= b'9' as usize {
            (v - b'0' as usize + 52) as i8
        } else if v == ch_62 as usize {
            62
        } else if v == ch_63 as usize {
            63
        } else {
            -1
        };
        v += 1;
    }
    table
}

static BASE64_REV_MAPS: [[i8; 256]; 3] = [
    make_reverse_table(b'+', b'/'),
    make_reverse_table(b'-', b'_'),
    make_reverse_table(b'+', b','),
];

pub unsafe fn base64_encode(
    mut src: *const u8,
    mut srclen: i32,
    dst: *mut u8,
    padding: bool,
    variant: base64_variant,
) -> i32 {
    let mut ac: u32 = 0;
    let mut cp = dst;
    let base64_table = &BASE64_TABLES[variant as usize];

    while srclen >= 3 {
        ac = (*src as u32) << 16 | (*src.add(1) as u32) << 8 | *src.add(2) as u32;
        *cp = base64_table[(ac >> 18) as usize]; cp = cp.add(1);
        *cp = base64_table[((ac >> 12) & 0x3f) as usize]; cp = cp.add(1);
        *cp = base64_table[((ac >> 6) & 0x3f) as usize]; cp = cp.add(1);
        *cp = base64_table[(ac & 0x3f) as usize]; cp = cp.add(1);
        src = src.add(3);
        srclen -= 3;
    }

    match srclen {
        2 => {
            ac = (*src as u32) << 16 | (*src.add(1) as u32) << 8;
            *cp = base64_table[(ac >> 18) as usize]; cp = cp.add(1);
            *cp = base64_table[((ac >> 12) & 0x3f) as usize]; cp = cp.add(1);
            *cp = base64_table[((ac >> 6) & 0x3f) as usize]; cp = cp.add(1);
            if padding { *cp = b'='; cp = cp.add(1); }
        }
        1 => {
            ac = (*src as u32) << 16;
            *cp = base64_table[(ac >> 18) as usize]; cp = cp.add(1);
            *cp = base64_table[((ac >> 12) & 0x3f) as usize]; cp = cp.add(1);
            if padding { *cp = b'='; cp = cp.add(1); *cp = b'='; cp = cp.add(1); }
        }
        _ => {}
    }
    cp.offset_from(dst) as i32
}

pub unsafe fn base64_decode(
    src: *const u8,
    mut srclen: i32,
    dst: *mut u8,
    mut padding: bool,
    variant: base64_variant,
) -> i32 {
    let mut bp = dst;
    let mut s = src;
    let base64_rev_tables = &BASE64_REV_MAPS[variant as usize];

    while srclen >= 4 {
        let input = [base64_rev_tables[*s.add(0) as usize], base64_rev_tables[*s.add(1) as usize], base64_rev_tables[*s.add(2) as usize], base64_rev_tables[*s.add(3) as usize]];
        let val = (input[0] as i32) << 18 | (input[1] as i32) << 12 | (input[2] as i32) << 6 | input[3] as i32;
        if val < 0 {
            if !padding || srclen != 4 || *s.add(3) != b'=' { return -1; }
            padding = false;
            srclen = if *s.add(2) == b'=' { 2 } else { 3 };
            break;
        }
        *bp = (val >> 16) as u8; bp = bp.add(1);
        *bp = (val >> 8) as u8; bp = bp.add(1);
        *bp = val as u8; bp = bp.add(1);
        s = s.add(4); srclen -= 4;
    }
    if srclen == 0 { return bp.offset_from(dst) as i32; }
    if padding || srclen == 1 { return -1; }
    let mut val = (base64_rev_tables[*s as usize] as i32) << 12 | (base64_rev_tables[*s.add(1) as usize] as i32) << 6;
    if srclen == 2 {
        if val & 0x800003ff != 0 { return -1; }
        *bp = (val >> 10) as u8; bp = bp.add(1);
    } else {
        val |= base64_rev_tables[*s.add(2) as usize] as i32;
        if val & 0x80000003 != 0 { return -1; }
        *bp = (val >> 10) as u8; bp = bp.add(1);
        *bp = (val >> 2) as u8; bp = bp.add(1);
    }
    bp.offset_from(dst) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
