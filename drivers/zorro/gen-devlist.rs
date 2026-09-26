// SPDX-License-Identifier: GPL-2.0
/*
 * Generate devlist.h from the Zorro ID file.
 *
 * (c) 2000 Geert Uytterhoeven <geert@linux-m68k.org>
 * Based on the PCI version by Martin Mares <mj@ucw.cz> (1999--2000).
 */

//! Translate the Zorro manufacturer/product list into the kernel name table.

#![no_main]

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr::NonNull;

// Use opaque libc streams to retain fgets chunking, ignored I/O errors and
// stdio buffering. All parsing and name-table generation are implemented here.
unsafe extern "C" {
    static stdin: *mut c_void;
    static stderr: *mut c_void;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fgets(buffer: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn fputc(byte: c_int, stream: *mut c_void) -> c_int;
    fn fputs(text: *const c_char, stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

struct Output(NonNull<c_void>);

impl Output {
    fn open() -> Option<Self> {
        // SAFETY: both arguments are terminated literals; fopen owns its result.
        NonNull::new(unsafe { fopen(c"devlist.h".as_ptr(), c"w".as_ptr()) }).map(Self)
    }

    fn write(&self, text: &CStr) {
        // SAFETY: the stream remains open and text is a terminated string.
        unsafe { fputs(text.as_ptr(), self.0.as_ptr()) };
    }

    fn quoted(&self, text: &[u8]) {
        for &byte in text {
            if byte == b'"' {
                // SAFETY: this literal contains no format substitutions.
                unsafe { fprintf(self.0.as_ptr(), c"\\\"".as_ptr()) };
            } else {
                // SAFETY: fputc accepts an unsigned byte promoted to int.
                unsafe { fputc(c_int::from(byte), self.0.as_ptr()) };
            }
        }
    }

    fn close(self) {
        // SAFETY: this consumes the sole owner of the open stream. The original
        // ignores fclose failures, and leaves error-path streams to libc exit.
        unsafe { fclose(self.0.as_ptr()) };
    }
}

macro_rules! emit {
    ($stream:expr, $format:expr $(, $argument:expr)* $(,)?) => {{
        // SAFETY: each call supplies a live stream, a literal format, and
        // matching C-promoted integers or live terminated character buffers.
        unsafe { fprintf($stream, $format.as_ptr() $(, $argument)*) };
    }};
}

fn syntax_error(line: &[u8], number: c_int, mode: c_int) -> c_int {
    // SAFETY: libc initializes stderr before calling main.
    let error = unsafe { stderr };
    emit!(error, c"Line %d: Syntax error in mode %d: %s\n", number, mode,
          line.as_ptr().cast::<c_char>());
    1
}

/// C startup preserves the caller's signal dispositions, including SIGPIPE.
#[no_mangle]
pub extern "C" fn main() -> c_int {
    // SAFETY: libc initializes its standard streams before calling main.
    let (input, error) = unsafe { (stdin, stderr) };
    let Some(output) = Output::open() else {
        emit!(error, c"Cannot create output file!\n");
        return 1;
    };
    let mut line = [0u8; 1024];
    let mut manufacturer = [0u8; 8];
    let mut have_manufacturer = false;
    let mut manufacturer_len = 0;
    let mut mode = 0;
    let mut number: c_int = 0;
    loop {
        // SAFETY: fgets receives the original sizeof(line)-1 bound, writes at
        // most 1022 input bytes plus NUL, and input is libc's live stdin stream.
        if unsafe { fgets(line.as_mut_ptr().cast(), 1023, input) }.is_null() {
            break;
        }
        number = number.wrapping_add(1);
        let mut end = line.iter().position(|&byte| byte == 0).unwrap();
        if let Some(newline) = line[..end].iter().position(|&byte| byte == b'\n') {
            end = newline;
            line[end] = 0;
        }
        if end == 0 || line[0] == b'#' {
            continue;
        }
        if line[0] == b'\t' {
            if mode != 1 || end <= 5 || line[5] != b' ' {
                return syntax_error(&line, number, mode);
            }
            let mut name = 5;
            while line[name] == b' ' {
                line[name] = 0;
                name += 1;
            }
            if manufacturer_len + end - name + 1 > 63 {
                if let Some(bracket) = line[name..end].iter().position(|&byte| byte == b'[') {
                    if bracket > 0 && line[name + bracket - 1] == b' ' {
                        end = name + bracket - 1;
                        line[end] = 0;
                    }
                }
                if manufacturer_len + end - name + 1 > 63 {
                    emit!(error, c"Line %d: Product name too long\n", number);
                    return 1;
                }
            }
            emit!(output.0.as_ptr(), c"\tPRODUCT(%s,%s,\"", manufacturer.as_ptr().cast::<c_char>(),
                  line[1..].as_ptr().cast::<c_char>());
            output.quoted(&line[name..end]);
            output.write(c"\")\n");
        } else if end > 4 && line[4] == b' ' {
            let mut name = 4;
            while line[name] == b' ' {
                line[name] = 0;
                name += 1;
            }
            if have_manufacturer {
                output.write(c"ENDMANUF()\n\n");
            }
            have_manufacturer = true;
            manufacturer[..4].copy_from_slice(&line[..4]);
            manufacturer_len = end - name;
            if manufacturer_len + 24 > 63 {
                emit!(error, c"Line %d: manufacturer name too long\n", number);
                return 1;
            }
            emit!(output.0.as_ptr(), c"MANUF(%s,\"", manufacturer.as_ptr().cast::<c_char>());
            output.quoted(&line[name..end]);
            output.write(c"\")\n");
            mode = 1;
        } else {
            return syntax_error(&line, number, mode);
        }
    }
    output.write(c"ENDMANUF()\n\n#undef MANUF\n#undef PRODUCT\n#undef ENDMANUF\n");
    output.close();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
