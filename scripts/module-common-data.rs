// SPDX-License-Identifier: GPL-2.0
//! Convert preprocessed original kernel metadata macros into Rust byte arrays.
//!
//! Input is the output of preprocessing `module-common-data.h` with the target
//! configuration. Validate the unchanged `module-common.c` with the same C
//! frontend and warning flags first: this preserves compiler-specific literal
//! diagnostics without emitting a C object. Output is included by the Rust
//! common-metadata object. Raw literal bytes and UTF-8 universal escapes are
//! preserved, including accepted GNU low-byte and historical UTF-8 extensions.
//! Non-UTF-8 execution encodings are explicitly rejected; the retained C
//! selection still supports them. Input encodings are converted by the C
//! preprocessor before this helper sees string tokens.
use std::io::{self, Read, Write};

fn universal(bytes: &mut Vec<u8>, value: u32) -> Result<(), String> {
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

fn strings(mut source: &[u8]) -> Result<Vec<u8>, String> {
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
                            let mut value = 0u8;
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
                            let mut value = 0u32;
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

fn generate(input: &[u8]) -> Result<String, String> {
    let mut values = std::collections::BTreeMap::new();
    for line in input.split(|byte| *byte == b'\n') {
        if !line.starts_with(b"LUPOS_") {
            continue;
        }
        let offset = line
            .iter()
            .position(u8::is_ascii_whitespace)
            .ok_or("missing metadata value")?;
        let name = std::str::from_utf8(&line[..offset]).map_err(|_| "non-ASCII metadata name")?;
        let value = line[offset..].trim_ascii();
        if values.insert(name, value).is_some() {
            return Err(format!("duplicate metadata record: {name}"));
        }
    }
    let get = |name| {
        values
            .get(name)
            .copied()
            .ok_or_else(|| format!("missing {name}"))
    };
    let charset = strings(get("LUPOS_EXEC_CHARSET")?)?;
    let charset = &charset[..charset.len() - 1];
    if !charset.eq_ignore_ascii_case(b"UTF-8") && !charset.eq_ignore_ascii_case(b"UTF8") {
        return Err(format!(
            "Rust common metadata requires UTF-8 execution encoding, got {}; use the retained C selection for this encoding",
            String::from_utf8_lossy(charset)
        ));
    }
    let mut vermagic = b"vermagic=".to_vec();
    let text = strings(get("LUPOS_VERMAGIC")?)?;
    if text[..text.len() - 1].contains(&0) {
        return Err("MODULE_INFO rejects embedded NUL in vermagic".into());
    }
    vermagic.extend(text);
    let salt = strings(get("LUPOS_BUILD_SALT")?)?;
    let lto: i32 = std::str::from_utf8(get("LUPOS_LTO")?)
        .map_err(|_| "invalid LTO bytes")?
        .parse()
        .map_err(|_| "invalid LTO value")?;
    if !matches!(lto, 0 | 1) {
        return Err("LTO value must be zero or one".into());
    }
    let mut out = format!(
        "// Generated from original target headers; do not edit.\n\
        pub(super) const VERMAGIC: [u8; {}] = {:?};\n\
        pub(super) const BUILD_SALT: [u8; {}] = {:?};\n\
        pub(super) const LTO: i32 = {};\n",
        vermagic.len(),
        vermagic,
        salt.len(),
        salt,
        lto
    );
    if let Some(hash) = values.get("LUPOS_ORC") {
        let bytes = std::str::from_utf8(hash)
            .map_err(|_| "invalid ORC hash bytes")?
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.strip_prefix("0x")
                    .ok_or("ORC hash byte is not hexadecimal")
                    .and_then(|s| u8::from_str_radix(s, 16).map_err(|_| "invalid ORC hash byte"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if bytes.len() != 20 {
            return Err("ORC hash must contain exactly twenty bytes".into());
        }
        out.push_str(&format!(
            "pub(super) const ORC_HASH: [u8; 20] = {bytes:?};\n"
        ));
    }
    if values.keys().any(|k| {
        !matches!(
            *k,
            "LUPOS_EXEC_CHARSET"
                | "LUPOS_VERMAGIC"
                | "LUPOS_BUILD_SALT"
                | "LUPOS_LTO"
                | "LUPOS_ORC"
        )
    }) {
        return Err("unknown metadata record".into());
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    io::stdout()
        .lock()
        .write_all(generate(&input)?.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn literal_concatenation_and_escapes() {
        assert_eq!(
            strings(br#" "a\"\\\n\r\t\v\f\a\b\e\?\'\101\x42" "09" "\377" "\0z" "" "#).unwrap(),
            b"a\"\\\n\r\t\x0b\x0c\x07\x08\x1b?'AB09\xff\0z\0"
        );
    }
    #[test]
    fn reject_unresolved_or_invalid_input() {
        for value in [
            b"UNKNOWN".as_slice(),
            b"L\"wide\"",
            b"\"bad\\x\"",
            b"\"bad",
            b"\"bad\\u0000\"",
            b"\"bad\\uD800\"",
            b"\"bad\\U80000000\"",
            b"\"bad\\u12\"",
        ] {
            assert!(strings(value).is_err(), "{value:?}");
        }
        assert!(
            generate(b"LUPOS_EXEC_CHARSET \"UTF-8\"\nLUPOS_VERMAGIC \"a\\0b\"\nLUPOS_BUILD_SALT \"\"\nLUPOS_LTO 0\n").is_err()
        );
        assert!(generate(
            b"LUPOS_EXEC_CHARSET \"UTF-8\"\nLUPOS_VERMAGIC \"x\"\nLUPOS_BUILD_SALT \"\"\nLUPOS_LTO 0\nLUPOS_ORC 0x00\n"
        )
        .is_err());
    }
    #[test]
    fn universal_characters_and_gcc_extension_bytes() {
        assert_eq!(
            strings(br#""\u1234\U0001f642\u0024""#).unwrap(),
            "ሴ🙂$\0".as_bytes()
        );
        assert_eq!(
            strings(br#""\400\777\x123456789abcdef01234567890\q\8\`""#).unwrap(),
            b"\0\xff\x90q8`\0"
        );
        assert_eq!(strings(br#""\U00110000\U00200000\U04000000\U7fffffff""#).unwrap(),
            b"\xf4\x90\x80\x80\xf8\x88\x80\x80\x80\xfc\x84\x80\x80\x80\x80\xfd\xbf\xbf\xbf\xbf\xbf\0");
    }
    #[test]
    fn raw_non_utf8_literal_bytes() {
        let result =
            generate(b"LUPOS_EXEC_CHARSET \"UTF-8\"\nLUPOS_VERMAGIC \"x\\t\xff\"\nLUPOS_BUILD_SALT \"\xfe\"\nLUPOS_LTO 0\n")
                .unwrap();
        assert!(result.contains("[254, 0]"));
        assert!(result.contains("120, 9, 255, 0]"));
    }
}
