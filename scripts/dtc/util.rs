// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2011 The Chromium Authors, all rights reserved.
 * Copyright 2008 Jon Loeliger, Freescale Semiconductor, Inc.
 */

/// C strtoull base-zero lexical conversion, retaining the end position.
pub(crate) fn integer(input: &[u8]) -> (u64, usize, bool) {
    let mut index = 0;
    while matches!(input.get(index), Some(b' ' | b'\t'..=b'\r')) {
        index += 1;
    }
    let negative = input.get(index) == Some(&b'-');
    if matches!(input.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    let start = index;
    let radix = if input.get(index) == Some(&b'0') {
        if matches!(input.get(index + 1), Some(b'x' | b'X'))
            && input.get(index + 2).is_some_and(u8::is_ascii_hexdigit)
        {
            index += 2;
            16
        } else {
            8
        }
    } else {
        10
    };
    let mut value = 0u64;
    let mut overflow = false;
    while let Some(digit) = input.get(index).and_then(|&b| (b as char).to_digit(radix)) {
        let (next, first) = value.overflowing_mul(u64::from(radix));
        let (next, second) = next.overflowing_add(u64::from(digit));
        overflow |= first || second;
        value = next;
        index += 1;
    }
    if index == start {
        return (0, 0, false);
    }
    if overflow {
        value = u64::MAX;
    } else if negative {
        value = value.wrapping_neg();
    }
    (value, index, overflow)
}

pub(crate) fn strtol(input: &[u8]) -> i64 {
    let negative = input
        .iter()
        .copied()
        .find(|b| !matches!(b, b' ' | b'\t'..=b'\r'))
        == Some(b'-');
    let (value, _, overflow) = integer(input);
    if overflow {
        return if negative { i64::MIN } else { i64::MAX };
    }
    if negative {
        if value.wrapping_neg() > (1u64 << 63) {
            i64::MIN
        } else {
            value as i64
        }
    } else {
        value.min(i64::MAX as u64) as i64
    }
}

pub(crate) fn escaped_path(path: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for &byte in path {
        if byte == b' ' {
            result.push(b'\\');
        }
        result.push(byte);
    }
    result
}
