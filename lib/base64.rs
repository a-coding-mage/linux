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
//! Allocation-free Base64 with the original kernel's strict trailing-bit rules.

/// The three alphabets supported by the original library.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Base64Variant {
    /// RFC 4648 standard alphabet (`+` and `/`).
    Standard,
    /// RFC 4648 URL-safe alphabet (`-` and `_`).
    UrlSafe,
    /// RFC 3501 modified alphabet (`+` and `,`).
    Imap,
}

/// Checked slice-interface failures; any already-written output is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Base64Error {
    /// Invalid alphabet, padding, input length or nonzero unused trailing bits.
    InvalidEncoding,
    /// The output slice cannot hold the next output byte.
    OutputTooSmall,
    /// The encoded output length is not representable by `usize`.
    LengthOverflow,
}

static TABLES: [[u8; 64]; 3] = [
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,",
];

const fn reverse(extra62: u8, extra63: u8) -> [i8; 256] {
    let mut table = [-1; 256];
    let mut index = 0;
    while index < 256 {
        let byte = index as u8;
        table[index] = match byte {
            b'A'..=b'Z' => (byte - b'A') as i8,
            b'a'..=b'z' => (byte - b'a' + 26) as i8,
            b'0'..=b'9' => (byte - b'0' + 52) as i8,
            _ if byte == extra62 => 62,
            _ if byte == extra63 => 63,
            _ => -1,
        };
        index += 1;
    }
    table
}

static REVERSE: [[i8; 256]; 3] = [
    reverse(b'+', b'/'),
    reverse(b'-', b'_'),
    reverse(b'+', b','),
];

/// Exact encoded byte count, without any NUL terminator.
pub const fn encoded_len(bytes: usize, padding: bool) -> Option<usize> {
    let whole = match (bytes / 3).checked_mul(4) {
        Some(value) => value,
        None => return None,
    };
    let extra = match bytes % 3 {
        0 => 0,
        _ if padding => 4,
        remainder => remainder + 1,
    };
    whole.checked_add(extra)
}

/// Checked equivalent of the original unpadded `BASE64_CHARS` size macro.
pub const fn base64_chars(bytes: usize) -> Option<usize> {
    match bytes.checked_mul(4) {
        Some(value) => match value.checked_add(2) {
            Some(value) => Some(value / 3),
            None => None,
        },
        None => None,
    }
}

/// Encode into a caller-owned slice; no NUL is appended.
///
/// An insufficient buffer is rejected before writing any output.
pub fn base64_encode(
    src: &[u8],
    dst: &mut [u8],
    padding: bool,
    variant: Base64Variant,
) -> Result<usize, Base64Error> {
    let count = encoded_len(src.len(), padding).ok_or(Base64Error::LengthOverflow)?;
    if dst.len() < count {
        return Err(Base64Error::OutputTooSmall);
    }
    encode_with(
        src.len(),
        |index| src[index],
        |index, byte| {
            dst[index] = byte;
            Ok(())
        },
        padding,
        variant,
    )
}

/// Decode without requiring a NUL terminator; preserve complete output groups
/// preceding an invalid group, exactly like the original C library.
///
/// A short destination returns `OutputTooSmall` at the first unwritable byte.
/// The native C interface instead requires sufficient caller-owned storage.
pub fn base64_decode(
    src: &[u8],
    dst: &mut [u8],
    padding: bool,
    variant: Base64Variant,
) -> Result<usize, Base64Error> {
    decode_with(
        src.len(),
        |index| src[index],
        |index, byte| {
            let output = dst.get_mut(index).ok_or(Base64Error::OutputTooSmall)?;
            *output = byte;
            Ok(())
        },
        padding,
        variant,
    )
}

// Index/value transports let the native boundary preserve defined in-place
// decoding without ever constructing aliased Rust input/output references.
pub(crate) fn encode_with(
    mut length: usize,
    mut read: impl FnMut(usize) -> u8,
    mut write: impl FnMut(usize, u8) -> Result<(), Base64Error>,
    padding: bool,
    variant: Base64Variant,
) -> Result<usize, Base64Error> {
    let table = &TABLES[variant as usize];
    let (mut input, mut output) = (0, 0);
    while length >= 3 {
        let value = (u32::from(read(input)) << 16)
            | (u32::from(read(input + 1)) << 8)
            | u32::from(read(input + 2));
        write(output, table[(value >> 18) as usize])?;
        write(output + 1, table[((value >> 12) & 63) as usize])?;
        write(output + 2, table[((value >> 6) & 63) as usize])?;
        write(output + 3, table[(value & 63) as usize])?;
        input += 3;
        output += 4;
        length -= 3;
    }
    if length != 0 {
        let mut value = u32::from(read(input)) << 16;
        if length == 2 {
            value |= u32::from(read(input + 1)) << 8;
        }
        write(output, table[(value >> 18) as usize])?;
        write(output + 1, table[((value >> 12) & 63) as usize])?;
        output += 2;
        if length == 2 {
            write(output, table[((value >> 6) & 63) as usize])?;
            output += 1;
        } else if padding {
            write(output, b'=')?;
            output += 1;
        }
        if padding {
            write(output, b'=')?;
            output += 1;
        }
    }
    Ok(output)
}

pub(crate) fn decode_with(
    mut length: usize,
    mut read: impl FnMut(usize) -> u8,
    mut write: impl FnMut(usize, u8) -> Result<(), Base64Error>,
    mut padding: bool,
    variant: Base64Variant,
) -> Result<usize, Base64Error> {
    let table = &REVERSE[variant as usize];
    let (mut input, mut output) = (0, 0);
    while length >= 4 {
        let a = i32::from(table[usize::from(read(input))]);
        let b = i32::from(table[usize::from(read(input + 1))]);
        let c = i32::from(table[usize::from(read(input + 2))]);
        let d = i32::from(table[usize::from(read(input + 3))]);
        let value = (a << 18) | (b << 12) | (c << 6) | d;
        if value < 0 {
            if !padding || length != 4 || read(input + 3) != b'=' {
                return Err(Base64Error::InvalidEncoding);
            }
            padding = false;
            length = if read(input + 2) == b'=' { 2 } else { 3 };
            break;
        }
        write(output, (value >> 16) as u8)?;
        write(output + 1, (value >> 8) as u8)?;
        write(output + 2, value as u8)?;
        input += 4;
        output += 3;
        length -= 4;
    }
    if length == 0 {
        return Ok(output);
    }
    if padding || length == 1 {
        return Err(Base64Error::InvalidEncoding);
    }
    let mut value = (i32::from(table[usize::from(read(input))]) << 12)
        | (i32::from(table[usize::from(read(input + 1))]) << 6);
    if length == 2 {
        if value as u32 & 0x8000_03ff != 0 {
            return Err(Base64Error::InvalidEncoding);
        }
        write(output, (value >> 10) as u8)?;
        output += 1;
    } else {
        value |= i32::from(table[usize::from(read(input + 2))]);
        if value as u32 & 0x8000_0003 != 0 {
            return Err(Base64Error::InvalidEncoding);
        }
        write(output, (value >> 10) as u8)?;
        write(output + 1, (value >> 2) as u8)?;
        output += 2;
    }
    Ok(output)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
