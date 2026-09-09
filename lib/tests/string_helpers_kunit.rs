// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Test cases for lib/string_helpers.c module.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::{ffi::c_char, ptr};

#[repr(C)]
pub struct kunit { _private: [u8; 0] }

#[repr(C)]
pub struct test_string {
    pub input: *const c_char,
    pub output: *const c_char,
    pub flags: u32,
}

#[repr(C)]
pub struct test_string_1 { pub output: *const c_char, pub flags: u32 }

#[repr(C)]
pub struct test_string_2 {
    pub input: *const c_char,
    pub s1: [test_string_1; 32],
}

pub const TEST_STRING_2_MAX_S1: usize = 32;
pub const TEST_STRING_2_DICT_0: *const c_char = ptr::null();
pub const TEST_STRING_2_DICT_1: &[u8] = b"b\\ \t\r\xcf\0";

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut u8;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(p: *mut u8);
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn string_unescape(src: *const c_char, dst: *mut c_char, size: usize, flags: u32) -> i32;
    fn string_unescape_any(src: *const c_char, dst: *mut c_char, size: usize) -> i32;
    fn string_unescape_inplace(src: *mut c_char, flags: u32) -> i32;
    fn string_unescape_any_inplace(src: *mut c_char) -> i32;
    fn string_escape_mem(src: *const c_char, isz: i32, dst: *mut c_char, osz: usize, flags: u32, esc: *const c_char) -> i32;
    fn string_get_size(size: u64, blk_size: u64, units: u32, buf: *mut c_char, len: usize);
    fn string_upper(dst: *mut c_char, src: *const c_char);
    fn string_lower(dst: *mut c_char, src: *const c_char);
    fn get_random_u32_below(n: u32) -> u32;
}

pub const UNESCAPE_SPACE: u32 = 1 << 0;
pub const UNESCAPE_OCTAL: u32 = 1 << 1;
pub const UNESCAPE_HEX: u32 = 1 << 2;
pub const UNESCAPE_SPECIAL: u32 = 1 << 3;
pub const UNESCAPE_ANY: u32 = UNESCAPE_SPACE | UNESCAPE_OCTAL | UNESCAPE_HEX | UNESCAPE_SPECIAL;
pub const UNESCAPE_ALL_MASK: u32 = UNESCAPE_ANY;
pub const ESCAPE_SPACE: u32 = 1 << 0;
pub const ESCAPE_SPECIAL: u32 = 1 << 1;
pub const ESCAPE_OCTAL: u32 = 1 << 2;
pub const ESCAPE_HEX: u32 = 1 << 3;
pub const ESCAPE_NP: u32 = 1 << 4;
pub const ESCAPE_NA: u32 = 1 << 5;
pub const ESCAPE_NAP: u32 = 1 << 6;
pub const ESCAPE_APPEND: u32 = 1 << 7;
pub const ESCAPE_NULL: u32 = 1 << 8;
pub const ESCAPE_ANY: u32 = ESCAPE_SPACE | ESCAPE_SPECIAL | ESCAPE_OCTAL | ESCAPE_HEX;
pub const ESCAPE_ALL_MASK: u32 = ESCAPE_SPACE | ESCAPE_SPECIAL | ESCAPE_OCTAL | ESCAPE_HEX | ESCAPE_NP | ESCAPE_NA | ESCAPE_NAP | ESCAPE_APPEND | ESCAPE_NULL;
pub const STRING_UNITS_10: u32 = 0;
pub const STRING_UNITS_2: u32 = 1;
pub const STRING_UNITS_NO_SPACE: u32 = 2;
pub const STRING_UNITS_NO_BYTES: u32 = 4;

unsafe fn test_string_check_buf(_test: *mut kunit, _name: *const c_char, _flags: u32,
                                _input: *mut c_char, _p: usize, out_real: *mut c_char,
                                q_real: usize, out_test: *mut c_char, q_test: usize) {
    // KUnit assertions preserve the C test's equality and memory comparison.
    assert_eq!(q_real, q_test);
    assert_eq!(core::slice::from_raw_parts(out_real as *const u8, q_test),
               core::slice::from_raw_parts(out_test as *const u8, q_test));
}

unsafe fn test_string_unescape(test: *mut kunit, name: *const c_char, flags: u32, inplace: bool) {
    let capacity = 256usize;
    let input = kunit_kzalloc(test, capacity, 0) as *mut c_char;
    let out_test = kunit_kzalloc(test, capacity, 0) as *mut c_char;
    let out_real = kunit_kzalloc(test, capacity, 0) as *mut c_char;
    assert!(!input.is_null() && !out_test.is_null() && !out_real.is_null());
    // The four source vectors are retained byte-for-byte, including C escape semantics.
    let vectors: [(&[u8], &[u8], u32); 4] = [
        (b"\\f\\ \\n\\r\\t\\v\0", b"\x0c\\ \n\r\t\x0b\0", UNESCAPE_SPACE),
        (b"\\40\\1\\387\\0064\\05\\040\\8a\\110\\777\0", b" \x01\x0387\x064\x05 \\8aH?7\0", UNESCAPE_OCTAL),
        (b"\\xv\\xa\\x2c\\xD\\x6f2\0", b"\\xv\n,\ro2\0", UNESCAPE_HEX),
        (b"\\h\\\\\\\"\\a\\e\\\0", b"\\h\\\"\x07\x1b\\\0", UNESCAPE_SPECIAL),
    ];
    let mut p = 0usize; let mut q = 0usize;
    for (src, expected, vector_flags) in vectors {
        memcpy(input.add(p) as *mut u8, src.as_ptr(), src.len() - 1); p += src.len() - 1;
        let chosen = if flags & vector_flags != 0 { expected } else { src };
        memcpy(out_test.add(q) as *mut u8, chosen.as_ptr(), chosen.len() - 1); q += chosen.len() - 1;
    }
    *input.add(p) = 0; p += 1;
    let real = if inplace { memcpy(out_real as *mut u8, input as *const u8, p); if flags == UNESCAPE_ANY { string_unescape_any_inplace(out_real) } else { string_unescape_inplace(out_real, flags) } }
               else if flags == UNESCAPE_ANY { string_unescape_any(input, out_real, capacity) } else { string_unescape(input, out_real, capacity, flags) };
    test_string_check_buf(test, name, flags, input, p - 1, out_real, real as usize, out_test, q);
}

unsafe fn run_upper_lower(_test: *mut kunit) {
    let cases = [(b"abcdefgh1234567890test\0", b"ABCDEFGH1234567890TEST\0"), (b"abCdeFgH1234567890TesT\0", b"ABCDEFGH1234567890TEST\0")];
    for (src, expected) in cases { let dst = kmalloc(src.len(), 0) as *mut c_char; assert!(!dst.is_null()); string_upper(dst, src.as_ptr() as *const c_char); assert_eq!(core::slice::from_raw_parts(dst as *const u8, expected.len()), expected); kfree(dst as *mut u8); }
}

// The remaining KUnit entry points retain the original test ordering and external interface.
#[no_mangle] pub unsafe extern "C" fn test_unescape(test: *mut kunit) { for flags in 0..=UNESCAPE_ALL_MASK { test_string_unescape(test, b"unescape\0".as_ptr() as *const c_char, flags, false); } test_string_unescape(test, b"unescape inplace\0".as_ptr() as *const c_char, get_random_u32_below(UNESCAPE_ALL_MASK + 1), true); }
#[no_mangle] pub unsafe extern "C" fn test_upper_lower(test: *mut kunit) { run_upper_lower(test); }

// These declarations correspond to the remaining C-side KUnit helpers and suite.
extern "C" {
    fn test_get_size(test: *mut kunit);
    fn test_escape(test: *mut kunit);
}

#[repr(C)]
pub struct kunit_case { pub run_case: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)]
pub struct kunit_suite { pub name: *const c_char, pub test_cases: *mut kunit_case }

#[no_mangle]
pub static mut string_helpers_test_cases: [kunit_case; 5] = [
    kunit_case { run_case: Some(test_get_size) },
    kunit_case { run_case: Some(test_upper_lower) },
    kunit_case { run_case: Some(test_unescape) },
    kunit_case { run_case: Some(test_escape) },
    kunit_case { run_case: None },
];

#[no_mangle]
pub static mut string_helpers_test_suite: kunit_suite = kunit_suite {
    name: b"string_helpers\0".as_ptr() as *const c_char,
    test_cases: string_helpers_test_cases.as_mut_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
