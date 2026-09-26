// SPDX-License-Identifier: MIT
/* Copyright © 2023 Intel Corporation */

//! Generate Xe workaround entry fragments and their matching enumeration.

#![no_main]

use std::ffi::{c_char, c_int, c_uint, c_void, CStr};

unsafe extern "C" {
    static stderr: *mut c_void;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fgets(buffer: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

macro_rules! emit {
    ($stream:expr, $format:expr $(, $argument:expr)* $(,)?) => {{
        // SAFETY: callers use live streams, literal formats, and corresponding
        // promoted C integers or terminated strings that outlive this call.
        unsafe { fprintf($stream, $format.as_ptr() $(, $argument)*) };
    }};
}

fn parse_error(message: &CStr, line: &[u8], number: c_uint) -> c_int {
    // SAFETY: libc initializes stderr before entering main.
    let error = unsafe { stderr };
    emit!(error, c"ERROR: %s\nERROR: %u: %.60s\n", message.as_ptr(), number,
          line.as_ptr().cast::<c_char>());
    -22 // -EINVAL, as returned by the original parser.
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn parse(input: *mut c_void, source: *mut c_void, header: *mut c_void,
         prefix: &CStr) -> c_int {
    let mut line = [0u8; 4097];
    let mut have_previous = false;
    let mut number: c_uint = 0;
    let mut index: c_uint = 0;
    loop {
        // SAFETY: input is open; fgets writes at most 4096 bytes plus a NUL.
        if unsafe { fgets(line.as_mut_ptr().cast(), 4097, input) }.is_null() {
            break;
        }
        if matches!(line[0], 0 | b'#' | b'\n') {
            number = number.wrapping_add(1);
            continue;
        }
        let len = line.iter().position(|&byte| byte == 0).unwrap();
        if len == 4096 {
            return parse_error(c"line too long", &line, number);
        }
        let continuation = whitespace(line[0]);
        // The C strip() starts its trailing-space scan at strlen(), which
        // points to NUL. It therefore drops exactly the last byte, including
        // ordinary data when the final input line has no newline.
        line[len - 1] = 0;
        let mut start = 0;
        while whitespace(line[start]) {
            start += 1;
        }
        let (name, rules) = if continuation {
            if !have_previous {
                return parse_error(c"invalid rule continuation", &line, number);
            }
            (None, start)
        } else {
            // Match strtok(name, " \t"), followed by strtok(NULL, "").
            // The latter preserves every byte after the first separator.
            let end = (start..len).find(|&i| matches!(line[i], 0 | b' ' | b'\t')).unwrap();
            if line[end] == 0 {
                // Original C dereferences NULL here. Reject this undefined
                // input safely without assigning it an invented rule.
                return parse_error(c"invalid empty rule\n", &line, number);
            }
            line[end] = 0;
            (Some(start), end + 1)
        };
        if line[rules] == 0 {
            return parse_error(c"invalid empty rule\n", &line, number);
        }
        let rule_pointer = line[rules..].as_ptr().cast::<c_char>();
        if let Some(name) = name {
            let name_pointer = line[name..].as_ptr().cast::<c_char>();
            emit!(header, c"\t%s_%s = %u,\n", prefix.as_ptr(), name_pointer, index);
            if index != 0 {
                emit!(source, c") },\n");
            }
            emit!(source, c"{ XE_RTP_NAME(\"%s\"),\n  XE_RTP_RULES(%s", name_pointer, rule_pointer);
            index = index.wrapping_add(1);
            have_previous = true;
        } else {
            emit!(source, c", OR,\n\t%s", rule_pointer);
        }
        number = number.wrapping_add(1);
    }
    if index != 0 {
        emit!(source, c") },\n");
    }
    emit!(header, c"\t_%s_COUNT = %u\n", prefix.as_ptr(), index);
    0
}

fn basename(path: &CStr) -> &CStr {
    let bytes = path.to_bytes_with_nul();
    let start = bytes.iter().rposition(|&byte| byte == b'/').map_or(0, |index| index + 1);
    CStr::from_bytes_with_nul(&bytes[start..]).unwrap()
}

fn close(streams: [*mut c_void; 3]) {
    for stream in streams {
        if !stream.is_null() {
            // SAFETY: each successful fopen produced one uniquely owned stream.
            // Preserve close order and the original ignored fclose failures.
            unsafe { fclose(stream) };
        }
    }
}

/// C startup preserves inherited signal dispositions and the initial C locale.
///
/// # Safety
/// The C runtime supplies argc live, terminated argument strings through argv.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    // SAFETY: this entry point is called by libc with its standard main ABI.
    let error = unsafe { stderr };
    if argc < 4 {
        // argc == 3 dereferences a missing argument in the original. The Rust
        // translation safely treats it as a wrong-argument invocation too.
        // SAFETY: libc always supplies the terminated program name at argv[0].
        let program = unsafe { *argv };
        emit!(error, c"ERROR: wrong arguments\n");
        emit!(error, c"usage: %s <input-rule-file> <generated-c-source-file> <generated-c-header-file>\n", program);
        return 1;
    }
    // SAFETY: argc proves these three non-null C argument strings exist.
    let paths = unsafe {
        [CStr::from_ptr(*argv.add(1)), CStr::from_ptr(*argv.add(2)), CStr::from_ptr(*argv.add(3))]
    };
    let filename = basename(paths[2]).to_bytes();
    if filename.len() > 127 {
        return 1;
    }
    let mut prefix = [0u8; 128];
    for (destination, &byte) in prefix.iter_mut().zip(filename) {
        if byte == b'.' {
            break;
        }
        *destination = byte.to_ascii_uppercase();
    }
    let prefix = CStr::from_bytes_until_nul(&prefix).unwrap();
    let mut streams = [std::ptr::null_mut(); 3];
    for index in 0..3 {
        let mode = if index == 0 { c"r" } else { c"w" };
        // SAFETY: paths and modes are live terminated C strings.
        streams[index] = unsafe { fopen(paths[index].as_ptr(), mode.as_ptr()) };
        if streams[index].is_null() {
            // GNU %m consumes the errno from fopen, just as in the C original.
            emit!(error, c"ERROR: Can't open %s: %m\n", paths[index].as_ptr());
            close(streams);
            return 1;
        }
    }
    emit!(streams[2], c"// SPDX-License-Identifier: MIT\n\n/*\n * DO NOT MODIFY.\n *\n * This file was generated from rules: %s\n */\n#ifndef _GENERATED_%s_\n#define _GENERATED_%s_\n\nenum {\n",
          basename(paths[0]).as_ptr(), prefix.as_ptr(), prefix.as_ptr());
    let result = parse(streams[0], streams[1], streams[2], prefix);
    if result == 0 {
        emit!(streams[2], c"};\n\n#endif\n");
    }
    close(streams);
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
