/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <linux/ucs2_string.h>.
// <linux/types.h> supplies u16/u8 and size_t; <linux/stddef.h> supplies NULL.

pub type ucs2_char_t = u16;

unsafe extern "C" {
    pub fn ucs2_strnlen(s: *const ucs2_char_t, maxlength: usize) -> core::ffi::c_ulong;
    pub fn ucs2_strlen(s: *const ucs2_char_t) -> core::ffi::c_ulong;
    pub fn ucs2_strsize(
        data: *const ucs2_char_t,
        maxlength: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn ucs2_strscpy(
        dst: *mut ucs2_char_t,
        src: *const ucs2_char_t,
        count: usize,
    ) -> isize;
    pub fn ucs2_strncmp(
        a: *const ucs2_char_t,
        b: *const ucs2_char_t,
        len: usize,
    ) -> core::ffi::c_int;

    pub fn ucs2_utf8size(src: *const ucs2_char_t) -> core::ffi::c_ulong;
    pub fn ucs2_as_utf8(
        dest: *mut u8,
        src: *const ucs2_char_t,
        maxlength: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
