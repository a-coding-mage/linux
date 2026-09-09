/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 SGI.
 * All rights reserved.
 */

// C dependencies: linux/types.h, linux/export.h, linux/string.h,
// linux/module.h, and linux/unicode.h.

extern "C" {
    pub fn utf8version_is_supported(
        um: *const unicode_map,
        version: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

/*
 * Determine the length of the normalized from of the string,
 * excluding any terminating NULL byte.
 * Returns 0 if only ignorable code points are present.
 * Returns -1 if the input is not valid UTF-8.
 */
extern "C" {
    pub fn utf8nlen(
        um: *const unicode_map,
        n: utf8_normalization,
        s: *const core::ffi::c_char,
        len: usize,
    ) -> isize;
}

/* Needed in struct utf8cursor below. */
pub const UTF8HANGULLEAF: usize = 12;

/*
 * Cursor structure used by the normalizer.
 */
#[repr(C)]
pub struct utf8cursor {
    pub um: *const unicode_map,
    pub n: utf8_normalization,
    pub s: *const core::ffi::c_char,
    pub p: *const core::ffi::c_char,
    pub ss: *const core::ffi::c_char,
    pub sp: *const core::ffi::c_char,
    pub len: core::ffi::c_uint,
    pub slen: core::ffi::c_uint,
    pub ccc: i16,
    pub nccc: i16,
    pub hangul: [u8; UTF8HANGULLEAF],
}

/*
 * Initialize a utf8cursor to normalize a string.
 * Returns 0 on success.
 * Returns -1 on failure.
 */
extern "C" {
    pub fn utf8ncursor(
        u8c: *mut utf8cursor,
        um: *const unicode_map,
        n: utf8_normalization,
        s: *const core::ffi::c_char,
        len: usize,
    ) -> core::ffi::c_int;
}

/*
 * Get the next byte in the normalization.
 * Returns a value > 0 && < 256 on success.
 * Returns 0 when the end of the normalization is reached.
 * Returns -1 if the string being normalized is not valid UTF-8.
 */
extern "C" {
    pub fn utf8byte(u8c: *mut utf8cursor) -> core::ffi::c_int;
}

#[repr(C)]
pub struct utf8data {
    pub maxage: core::ffi::c_uint,
    pub offset: core::ffi::c_uint,
}

#[repr(C)]
pub struct utf8data_table {
    pub utf8agetab: *const core::ffi::c_uint,
    pub utf8agetab_size: core::ffi::c_int,

    pub utf8nfdicfdata: *const utf8data,
    pub utf8nfdicfdata_size: core::ffi::c_int,

    pub utf8nfdidata: *const utf8data,
    pub utf8nfdidata_size: core::ffi::c_int,

    pub utf8data: *const u8,
}

extern "C" {
    pub static utf8_data_table: utf8data_table;
}

// `unicode_map` and `utf8_normalization` are supplied by linux/unicode.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
