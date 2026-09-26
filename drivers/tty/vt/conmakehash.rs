// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 1995-1997 H. Peter Anvin */
//! Create the default console font's Unicode tables.
//!
//! The C entry point and stdio preserve byte-valued filenames, inherited
//! SIGPIPE, fgets chunking, and the original ignored stream-error behavior.
#![no_main]

use std::ffi::{c_char, c_int, c_long, c_void, CStr};

extern "C" {
    static mut stdin: *mut c_void;
    static mut stderr: *mut c_void;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fgets(buffer: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(prefix: *const c_char);
    fn strtol(text: *const c_char, end: *mut *mut c_char, base: c_int) -> c_long;
}

struct Table {
    values: [[u16; 255]; 256],
    counts: [usize; 256],
}

// The original keeps these large zero-initialized arrays out of the stack.
// Only the C entry point accesses them, once, before process termination.
static mut TABLE: Table = Table {
    values: [[0; 255]; 256],
    counts: [0; 256],
};

impl Table {
    fn add(&mut self, position: i32, unicode: i32) -> Result<(), c_int> {
        if unicode > 0xfffe {
            return Ok(());
        }
        let position = position as usize;
        let count = self.counts[position];
        if self.values[position][..count].contains(&(unicode as u16)) {
            return Ok(());
        }
        if count == 255 {
            // SAFETY: stderr is the C runtime's stream; the format is static.
            unsafe {
                fprintf(
                    stderr,
                    c"ERROR: Only 255 unicodes/glyph permitted!\n".as_ptr(),
                );
            }
            return Err(65);
        }
        self.values[position][count] = unicode as u16;
        self.counts[position] += 1;
        Ok(())
    }
}

fn whitespace(line: &[u8], position: &mut usize) {
    while matches!(line[*position], b' ' | b'\t') {
        *position += 1;
    }
}

fn unicode(line: &[u8], position: &mut usize) -> i32 {
    let mut start = *position;
    whitespace(line, &mut start);
    if line.get(start..start + 2) != Some(b"U+") {
        return -1;
    }
    let Some(digits) = line.get(start + 2..start + 6) else {
        return -1;
    };
    if !digits.iter().all(u8::is_ascii_hexdigit)
        || line.get(start + 6).is_some_and(u8::is_ascii_hexdigit)
    {
        return -1;
    }
    *position = start + 6;
    digits.iter().fold(0, |value, digit| {
        value * 16
            + if digit.is_ascii_digit() {
                (digit - b'0') as i32
            } else {
                (digit.to_ascii_lowercase() - b'a' + 10) as i32
            }
    })
}

fn number(line: &[u8], position: &mut usize) -> Option<i32> {
    let start = line[*position..].as_ptr().cast::<c_char>();
    let mut end = std::ptr::null_mut();
    // SAFETY: Every line includes fgets' trailing NUL. strtol preserves the
    // host C long width, base-zero syntax, whitespace and overflow semantics.
    let value = unsafe { strtol(start, &mut end, 0) };
    if std::ptr::eq(start, end) {
        None
    } else {
        // SAFETY: strtol's end pointer is inside the same line allocation.
        *position += unsafe { end.offset_from(start) } as usize;
        Some(value as i32)
    }
}

fn parse(table: &mut Table, line: &[u8], name: *const c_char) -> Result<(), c_int> {
    let mut position = 0;
    whitespace(line, &mut position);
    if matches!(line[position], 0 | b'#') {
        return Ok(());
    }
    let bad_line = || {
        // SAFETY: The complete input line is NUL terminated.
        unsafe {
            fprintf(stderr, c"Bad input line: %s\n".as_ptr(), line.as_ptr());
        }
        65
    };
    let first = number(line, &mut position).ok_or_else(bad_line)?;
    whitespace(line, &mut position);
    let last = if line[position] == b'-' {
        position += 1;
        number(line, &mut position).ok_or_else(bad_line)?
    } else {
        0
    };
    // SAFETY: name is a live C string; integer varargs have their C widths.
    unsafe {
        if !(0..256).contains(&first) {
            fprintf(
                stderr,
                c"%s: Glyph number (0x%x) larger than font length\n".as_ptr(),
                name,
                first as u32,
            );
            return Err(65);
        }
        if last != 0 && (last < first || last >= 256) {
            fprintf(
                stderr,
                c"%s: Bad end of range (0x%x)\n".as_ptr(),
                name,
                last as u32,
            );
            return Err(65);
        }
        if last != 0 {
            whitespace(line, &mut position);
            if line.get(position..position + 4) == Some(b"idem") {
                for glyph in first..=last {
                    table.add(glyph, glyph)?;
                }
                position += 4;
            } else {
                let start = unicode(line, &mut position);
                whitespace(line, &mut position);
                if line[position] != b'-' {
                    fprintf(stderr, c"%s: Corresponding to a range of font positions, there should be a Unicode range\n".as_ptr(), name);
                    return Err(65);
                }
                position += 1;
                let end = unicode(line, &mut position);
                if start < 0 || end < 0 {
                    fprintf(
                        stderr,
                        c"%s: Bad Unicode range corresponding to font position range 0x%x-0x%x\n"
                            .as_ptr(),
                        name,
                        first as u32,
                        last as u32,
                    );
                    return Err(65);
                }
                if end - start != last - first {
                    fprintf(stderr, c"%s: Unicode range U+%x-U+%x not of the same length as font position range 0x%x-0x%x\n".as_ptr(), name, start as u32, end as u32, first as u32, last as u32);
                    return Err(65);
                }
                for glyph in first..=last {
                    table.add(glyph, start - first + glyph)?;
                }
            }
        } else {
            loop {
                let value = unicode(line, &mut position);
                if value < 0 {
                    break;
                }
                table.add(first, value)?;
            }
        }
        whitespace(line, &mut position);
        if !matches!(line[position], 0 | b'#') {
            fprintf(
                stderr,
                c"%s: trailing junk (%s) ignored\n".as_ptr(),
                name,
                line[position..].as_ptr(),
            );
        }
    }
    Ok(())
}

fn emit(table: &Table) {
    // SAFETY: All formats are static C strings with correctly typed varargs.
    // Output failures remain ignored, as in the original printf loops.
    unsafe {
        printf(c"/*\n * Automatically generated file; Do not edit.\n */\n\n#include <linux/types.h>\n\nu8 dfont_unicount[%d] = \n{\n\t".as_ptr(), 256);
        for (index, count) in table.counts.iter().enumerate() {
            printf(c"%3d".as_ptr(), *count as c_int);
            separator(index, 256);
        }
        let total = table.counts.iter().sum::<usize>();
        printf(
            c"\nu16 dfont_unitable[%d] = \n{\n\t".as_ptr(),
            total as c_int,
        );
        let values = table
            .values
            .iter()
            .zip(&table.counts)
            .flat_map(|(row, count)| &row[..*count]);
        for (index, value) in values.enumerate() {
            printf(c"0x%04x".as_ptr(), *value as u32);
            separator(index, total);
        }
    }
}

unsafe fn separator(index: usize, total: usize) {
    let text = if index == total - 1 {
        c"\n};\n"
    } else if index % 8 == 7 {
        c",\n\t"
    } else {
        c", "
    };
    // SAFETY: Each static format contains no vararg conversions.
    unsafe {
        printf(text.as_ptr());
    }
}

#[no_mangle]
unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: The C runtime supplies argc live argv strings. C stdio owns the
    // stream; fgets reads at most the buffer size and supplies its trailing NUL.
    unsafe {
        if !(2..=5).contains(&argc) {
            fprintf(
                stderr,
                c"Usage: \n        %s chartable [hashsize] [hashstep] [maxhashlevel]\n".as_ptr(),
                *argv,
            );
            return 64;
        }
        let argument = *argv.add(1);
        let (stream, name) = if CStr::from_ptr(argument).to_bytes() == b"-" {
            (stdin, c"stdin".as_ptr())
        } else {
            let stream = fopen(argument, c"r".as_ptr());
            if stream.is_null() {
                perror(argument);
                return 66;
            }
            (stream, argument.cast_const())
        };
        // The startup thread is the sole owner for this invocation; no other
        // code can access TABLE while this unique reference is live.
        let table = &mut *std::ptr::addr_of_mut!(TABLE);
        table.counts.fill(0);
        let mut buffer = [0u8; 65536];
        while !fgets(buffer.as_mut_ptr().cast(), buffer.len() as c_int, stream).is_null() {
            let length = CStr::from_ptr(buffer.as_ptr().cast()).to_bytes().len();
            if let Some(newline) = buffer[..length].iter().position(|&byte| byte == b'\n') {
                buffer[newline] = 0;
            } else {
                fprintf(stderr, c"%s: Warning: line too long\n".as_ptr(), name);
            }
            // Include the terminator for strtol and the parser's lookahead.
            if let Err(status) = parse(table, &buffer[..=length], name) {
                return status;
            }
        }
        fclose(stream);
        emit(table);
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
