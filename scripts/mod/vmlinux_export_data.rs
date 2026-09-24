// SPDX-License-Identifier: GPL-2.0-only
//! Normalize target-preprocessed export fields, not target table construction.
//!
//! Original C syntax validation must precede record preprocessing. The original
//! outer macros, stringification and MODULE_INFO declaration remain authoritative
//! for frontend expansion. Rust export macros construct all output tables.

use std::fmt::Write;
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

pub(crate) fn header(original: &str, count: usize) -> String {
    let mut out = String::from("/* Generated by modpost; validate as C, then preprocess records. */\n#line 1 \".vmlinux.export.c\"\n");
    for (index, line) in original.lines().enumerate() {
        writeln!(out, "{line}").unwrap();
        if line == "#include <linux/export-internal.h>" {
            writeln!(
                out,
                "#ifdef LUPOS_RUST_VMLINUX_RECORDS\nLUPOS_EXPORT_COUNT {count};"
            )
            .unwrap();
            out.push_str("#ifdef __GNUC_EXECUTION_CHARSET_NAME\nLUPOS_EXPORT_CHARSET __GNUC_EXECUTION_CHARSET_NAME;\n#elif defined(__clang__)\nLUPOS_EXPORT_CHARSET \"UTF-8\";\n#else\n#error Unknown C frontend execution charset\n#endif\nLUPOS_EXPORT_INPUT_UTF8 \"é\";\nLUPOS_EXPORT_LOGICAL_SOURCE __FILE__, __BASE_FILE__;\n#define LUPOS_STRINGIFY_REF(sym) #sym\n#undef __KSYMTAB\n#define __KSYMTAB(name, sym, ns) LUPOS_EXPORT_KSYM #name, LUPOS_STRINGIFY_REF(sym), ns\n#undef SYMBOL_FLAGS\n#define SYMBOL_FLAGS(sym, flags) LUPOS_EXPORT_FLAGS #sym, #flags\n#undef SYMBOL_CRC\n#define SYMBOL_CRC(sym, crc) LUPOS_EXPORT_CRC #sym, #crc\n#endif\n");
            writeln!(out, "#line {} \".vmlinux.export.c\"", index + 2).unwrap();
        } else if line == "#define __MODULE_INFO_PREFIX" {
            out.push_str("#ifdef LUPOS_RUST_VMLINUX_RECORDS\n#undef __MODULE_INFO_PREFIX\n#define __MODULE_INFO_PREFIX LUPOS_EXPORT_ALIAS\n#endif\n");
            writeln!(out, "#line {} \".vmlinux.export.c\"", index + 2).unwrap();
        }
    }
    out.push_str("#ifdef LUPOS_RUST_VMLINUX_RECORDS\nLUPOS_EXPORT_END;\n#endif\n");
    out
}

// This intentionally follows module-common-data's ordinary narrow-literal
// decoder. The frontend validation phase owns target/compiler diagnostics;
// retained accepted extensions must have the same bytes here.

fn identifier(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn quoted_end(input: &[u8], mut at: usize) -> Result<usize, String> {
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

fn fields(input: &[u8]) -> Result<Vec<&[u8]>, String> {
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

fn text(input: &[u8]) -> Result<String, String> {
    let mut value = strings(input)?;
    value.pop();
    if value.contains(&0) {
        return Err("embedded NUL in export field".into());
    }
    String::from_utf8(value).map_err(|_| {
        "Rust export fields require UTF-8 decoded text; use retained C selection".into()
    })
}

pub(crate) fn generate(input: &[u8]) -> Result<String, String> {
    let mut at = 0;
    let mut expected = None;
    let mut charset = false;
    let mut input_utf8 = false;
    let mut logical_source = false;
    let mut ended = false;
    let mut count = 0;
    let mut out = String::from("// Generated from original target-preprocessed metadata fields.\n");
    while at < input.len() {
        match input[at] {
            b'\'' | b'"' => {
                at = quoted_end(input, at)?;
                continue;
            }
            byte if identifier(byte) => (),
            _ => {
                at += 1;
                continue;
            }
        }
        let begin = at;
        while at < input.len() && identifier(input[at]) {
            at += 1;
        }
        let name = &input[begin..at];
        if !name.starts_with(b"LUPOS_EXPORT_") {
            continue;
        }
        if ended {
            return Err("export record after terminal marker".into());
        }
        let begin = at;
        while at < input.len() && input[at] != b';' {
            if matches!(input[at], b'\'' | b'"') {
                at = quoted_end(input, at)?;
            } else {
                at += 1;
            }
        }
        if at == input.len() {
            return Err("unterminated export record".into());
        }
        let value = input[begin..at].trim_ascii();
        at += 1;
        match name {
            b"LUPOS_EXPORT_COUNT" => {
                if expected.is_some() || count != 0 {
                    return Err("duplicate or misplaced export count".into());
                }
                expected = Some(
                    std::str::from_utf8(value)
                        .map_err(|_| "invalid export count")?
                        .parse::<usize>()
                        .map_err(|_| "invalid export count")?,
                );
            }
            b"LUPOS_EXPORT_CHARSET" => {
                if expected.is_none() || charset || count != 0 {
                    return Err("duplicate or misplaced execution charset".into());
                }
                let value = text(value)?;
                if !value.eq_ignore_ascii_case("UTF-8") && !value.eq_ignore_ascii_case("UTF8") {
                    return Err(format!("Rust vmlinux metadata requires UTF-8 execution encoding, got {value}; use retained C selection"));
                }
                charset = true;
            }
            b"LUPOS_EXPORT_INPUT_UTF8" => {
                if expected.is_none() || input_utf8 || count != 0 {
                    return Err("duplicate or misplaced input charset probe".into());
                }
                if strings(value)? != b"\xc3\xa9\0" {
                    return Err("Rust vmlinux metadata requires UTF-8 input encoding; use retained C selection".into());
                }
                input_utf8 = true;
            }
            b"LUPOS_EXPORT_LOGICAL_SOURCE" => {
                if expected.is_none() || logical_source || count != 0 {
                    return Err("duplicate or misplaced logical source probe".into());
                }
                let value = fields(value)?;
                if value.len() != 2 || strings(value[0])? != strings(value[1])? {
                    return Err("source filename mapping differs for __FILE__ and __BASE_FILE__; use retained C selection".into());
                }
                logical_source = true;
            }
            b"LUPOS_EXPORT_END" => {
                if !value.is_empty()
                    || expected != Some(count)
                    || !charset
                    || !input_utf8
                    || !logical_source
                {
                    return Err("missing, extra, or incomplete export records".into());
                }
                ended = true;
            }
            _ => {
                if expected.is_none() || !charset || !input_utf8 || !logical_source {
                    return Err("export record precedes required format guards".into());
                }
                let values = fields(value)?;
                match name {
                    b"LUPOS_EXPORT_KSYM" if values.len() == 3 => {
                        writeln!(
                            out,
                            "__KSYMTAB_NORMALIZED!({:?}, {:?}, {:?});",
                            text(values[0])?,
                            text(values[1])?,
                            text(values[2])?
                        )
                        .unwrap();
                    }
                    b"LUPOS_EXPORT_FLAGS" | b"LUPOS_EXPORT_CRC" if values.len() == 2 => {
                        let symbol = text(values[0])?;
                        let number = text(values[1])?;
                        let number = number
                            .strip_prefix("0x")
                            .ok_or("expected generated hexadecimal export value")?;
                        let number = u32::from_str_radix(number, 16)
                            .map_err(|_| "invalid generated export value")?;
                        if name == b"LUPOS_EXPORT_FLAGS" {
                            let number = u8::try_from(number)
                                .map_err(|_| "export flags do not fit in a byte")?;
                            writeln!(out, "SYMBOL_FLAGS_NORMALIZED!({symbol:?}, 0x{number:02x});")
                                .unwrap();
                        } else {
                            writeln!(out, "SYMBOL_CRC_NORMALIZED!({symbol:?}, 0x{number:08x});")
                                .unwrap();
                        }
                    }
                    b"LUPOS_EXPORT_ALIAS" if values.len() == 1 => {
                        let value = strings(values[0])?;
                        if value[..value.len() - 1].contains(&0) {
                            return Err("MODULE_INFO rejects embedded NUL".into());
                        }
                        writeln!(out, "#[used]\n#[link_section = \".modinfo\"]\nstatic __VMLINUX_ALIAS_{count}: [u8; {}] = {:?};", value.len(), value).unwrap();
                    }
                    _ => {
                        return Err(format!(
                            "unknown export record or invalid field count: {}",
                            String::from_utf8_lossy(name)
                        ))
                    }
                }
                count += 1;
                if count > expected.unwrap() {
                    return Err("too many export records".into());
                }
            }
        }
    }
    if !ended {
        return Err("missing terminal export marker".into());
    }
    Ok(out)
}
