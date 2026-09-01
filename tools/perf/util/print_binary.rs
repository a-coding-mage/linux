// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/print_binary.c.
// Dependencies from "print_binary.h", <linux/log2.h>, and <linux/ctype.h>
// are expected to be provided by surrounding bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_int, c_uchar, c_uint, c_void};

pub type size_t = usize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

pub type binary__fprintf_t = Option<
    unsafe extern "C" fn(
        c_int,
        size_t,
        *mut c_void,
        *mut FILE,
    ) -> c_int,
>;

unsafe extern "C" {
    /*
     * These print state constants come from print_binary.h in the original C
     * translation unit.
     */
    static BINARY_PRINT_DATA_BEGIN: c_int;
    static BINARY_PRINT_LINE_BEGIN: c_int;
    static BINARY_PRINT_ADDR: c_int;
    static BINARY_PRINT_NUM_DATA: c_int;
    static BINARY_PRINT_NUM_PAD: c_int;
    static BINARY_PRINT_SEP: c_int;
    static BINARY_PRINT_CHAR_DATA: c_int;
    static BINARY_PRINT_CHAR_PAD: c_int;
    static BINARY_PRINT_LINE_END: c_int;
    static BINARY_PRINT_DATA_END: c_int;

    fn roundup_pow_of_two(n: size_t) -> size_t;
    fn isprint(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
}

pub unsafe extern "C" fn binary__fprintf(
    data: *mut c_uchar,
    len: size_t,
    mut bytes_per_line: size_t,
    printer: binary__fprintf_t,
    extra: *mut c_void,
    fp: *mut FILE,
) -> c_int {
    let mut i: size_t;
    let mut j: size_t;
    let mask: size_t;
    let mut printed: c_int = 0;

    let Some(printer) = printer else {
        return 0;
    };

    bytes_per_line = unsafe { roundup_pow_of_two(bytes_per_line) };
    mask = bytes_per_line.wrapping_sub(1);

    printed += unsafe { printer(BINARY_PRINT_DATA_BEGIN, 0, extra, fp) };
    i = 0;
    while i < len {
        if (i & mask) == 0 {
            printed += unsafe { printer(BINARY_PRINT_LINE_BEGIN, -1isize as size_t, extra, fp) };
            printed += unsafe { printer(BINARY_PRINT_ADDR, i, extra, fp) };
        }

        printed += unsafe { printer(BINARY_PRINT_NUM_DATA, unsafe { *data.add(i) } as size_t, extra, fp) };

        if ((i & mask) == mask) || i == len.wrapping_sub(1) {
            j = 0;
            while j < mask.wrapping_sub(i & mask) {
                printed += unsafe { printer(BINARY_PRINT_NUM_PAD, -1isize as size_t, extra, fp) };
                j += 1;
            }

            unsafe {
                printer(BINARY_PRINT_SEP, i, extra, fp);
            }
            j = i & !mask;
            while j <= i {
                printed += unsafe {
                    printer(BINARY_PRINT_CHAR_DATA, *data.add(j) as size_t, extra, fp)
                };
                j += 1;
            }
            j = 0;
            while j < mask.wrapping_sub(i & mask) {
                printed += unsafe { printer(BINARY_PRINT_CHAR_PAD, i, extra, fp) };
                j += 1;
            }
            printed += unsafe { printer(BINARY_PRINT_LINE_END, -1isize as size_t, extra, fp) };
        }
        i += 1;
    }
    printed += unsafe { printer(BINARY_PRINT_DATA_END, -1isize as size_t, extra, fp) };
    printed
}

pub unsafe extern "C" fn is_printable_array(p: *mut i8, mut len: c_uint) -> c_int {
    let mut i: c_uint;

    if p.is_null() || len == 0 || unsafe { *p.add((len - 1) as usize) } != 0 {
        return 0;
    }

    len -= 1;

    i = 0;
    while i < len && unsafe { *p.add(i as usize) } != 0 {
        let ch = unsafe { *p.add(i as usize) } as c_int;
        if unsafe { isprint(ch) } == 0 && unsafe { isspace(ch) } == 0 {
            return 0;
        }
        i += 1;
    }
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
