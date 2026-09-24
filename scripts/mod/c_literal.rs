// SPDX-License-Identifier: GPL-2.0-only
//! Shared original-frontend narrow C literal and token decoding.

pub(crate) fn universal(bytes: &mut Vec<u8>, value: u32) -> Result<(), String> {
    if value > 0x7fff_ffff
        || (0xd800..=0xdfff).contains(&value)
        || (value < 0xa0 && !matches!(value, 0x24 | 0x40 | 0x60))
    {
        return Err("invalid universal character name in a C string".into());
    }
    // GCC accepts historical five/six-byte UTF-8 for non-Unicode UCN values
    // below 2^31 with a warning. Clang rejects those values. Byte emission
    // preserves GCC's accepted values; compiler-specific diagnostics belong
    // to the original frontend's no-output validation stage.
    let count = match value {
        0..=0x7f => 1,
        0x80..=0x7ff => 2,
        0x800..=0xffff => 3,
        0x10000..=0x1fffff => 4,
        0x200000..=0x3ffffff => 5,
        _ => 6,
    };
    if count == 1 {
        bytes.push(value as u8);
    } else {
        bytes.push((0xff_u8 << (8 - count)) | (value >> (6 * (count - 1))) as u8);
        let mut index = count - 1;
        while index != 0 {
            index -= 1;
            bytes.push(0x80 | ((value >> (index * 6)) & 0x3f) as u8);
        }
    }
    Ok(())
}

pub(crate) fn strings(mut source: &[u8]) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    let mut count = 0;
    while !source.is_empty() {
        while source.first().is_some_and(u8::is_ascii_whitespace) {
            source = &source[1..];
        }
        if source.is_empty() {
            break;
        }
        if source[0] != b'"' {
            return Err("expected an ordinary narrow C string literal".into());
        }
        source = &source[1..];
        count += 1;
        loop {
            let Some(&byte) = source.first() else {
                return Err("unterminated C string".into());
            };
            source = &source[1..];
            match byte {
                b'"' => break,
                b'\n' | b'\r' => return Err("newline in C string".into()),
                b'\\' => {
                    let Some(&escape) = source.first() else {
                        return Err("incomplete C escape".into());
                    };
                    source = &source[1..];
                    let value = match escape {
                        b'a' => 7,
                        b'b' => 8,
                        b't' => 9,
                        b'n' => 10,
                        b'v' => 11,
                        b'f' => 12,
                        b'r' => 13,
                        b'e' => 27,
                        b'\\' | b'\'' | b'"' | b'?' => escape,
                        b'0'..=b'7' => {
                            let mut value = u16::from(escape - b'0');
                            let mut remaining = 2;
                            while remaining != 0
                                && source.first().is_some_and(|c| matches!(c, b'0'..=b'7'))
                            {
                                value = value * 8 + u16::from(source[0] - b'0');
                                source = &source[1..];
                                remaining -= 1;
                            }
                            value as u8
                        }
                        b'x' => {
                            let mut value = 0_u8;
                            let mut digits = 0;
                            while let Some(digit) =
                                source.first().and_then(|c| char::from(*c).to_digit(16))
                            {
                                value = value.wrapping_mul(16).wrapping_add(digit as u8);
                                digits += 1;
                                source = &source[1..];
                            }
                            if digits == 0 {
                                return Err("empty hexadecimal escape".into());
                            }
                            value
                        }
                        b'u' | b'U' => {
                            let count = if escape == b'u' { 4 } else { 8 };
                            let digits = source
                                .get(..count)
                                .ok_or("incomplete universal character name")?;
                            let mut value = 0_u32;
                            for digit in digits {
                                value = value * 16
                                    + char::from(*digit)
                                        .to_digit(16)
                                        .ok_or("invalid universal character name digit")?;
                            }
                            source = &source[count..];
                            universal(&mut result, value)?;
                            continue;
                        }
                        // GCC and Clang both accept unknown escapes with a
                        // diagnostic and use the escaped character verbatim.
                        _ => escape,
                    };
                    result.push(value);
                }
                _ => result.push(byte),
            }
        }
    }
    if count == 0 {
        return Err("missing C string literal".into());
    }
    result.push(0);
    Ok(result)
}

pub(crate) fn identifier(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

pub(crate) fn quoted_end(input: &[u8], mut at: usize) -> Result<usize, String> {
    let quote = input[at];
    at += 1;
    while at < input.len() {
        match input[at] {
            b'\\' => at += 2,
            byte if byte == quote => return Ok(at + 1),
            _ => at += 1,
        }
    }
    Err("unterminated quoted token in preprocessed export input".into())
}

pub(crate) fn fields(input: &[u8]) -> Result<Vec<&[u8]>, String> {
    let mut out = Vec::new();
    let mut begin = 0;
    let mut at = 0;
    while at < input.len() {
        match input[at] {
            b'\'' | b'"' => at = quoted_end(input, at)?,
            b',' => {
                out.push(input[begin..at].trim_ascii());
                at += 1;
                begin = at;
            }
            _ => at += 1,
        }
    }
    out.push(input[begin..].trim_ascii());
    Ok(out)
}

pub(crate) fn text(input: &[u8]) -> Result<String, String> {
    let mut value = strings(input)?;
    value.pop();
    if value.contains(&0) {
        return Err("embedded NUL in export field".into());
    }
    String::from_utf8(value).map_err(|_| {
        "Rust export fields require UTF-8 decoded text; use retained C selection".into()
    })
}
