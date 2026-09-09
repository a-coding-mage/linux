/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/parser.h
 *
 * Header for lib/parser.c
 * Intended use of these functions is parsing filesystem argument lists,
 * but could potentially be used anywhere else that simple option=arg
 * parsing is required.
 */

/* associates an integer enumerator with a pattern string. */
#[repr(C)]
pub struct match_token {
    pub token: ::core::ffi::c_int,
    pub pattern: *const ::core::ffi::c_char,
}

pub type match_table_t = [match_token; 0];

/* Maximum number of arguments that match_token will find in a pattern */
pub const MAX_OPT_ARGS: ::core::ffi::c_int = 3;

/* Describe the location within a string of a substring */
#[repr(C)]
pub struct substring_t {
    pub from: *mut ::core::ffi::c_char,
    pub to: *mut ::core::ffi::c_char,
}

unsafe extern "C" {
    pub fn match_token(
        s: *mut ::core::ffi::c_char,
        table: *const match_token,
        args: *mut substring_t,
    ) -> ::core::ffi::c_int;
    pub fn match_int(
        s: *mut substring_t,
        result: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn match_uint(
        s: *mut substring_t,
        result: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn match_u64(
        s: *mut substring_t,
        result: *mut u64,
    ) -> ::core::ffi::c_int;
    pub fn match_octal(
        s: *mut substring_t,
        result: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn match_hex(
        s: *mut substring_t,
        result: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn match_wildcard(
        pattern: *const ::core::ffi::c_char,
        str_: *const ::core::ffi::c_char,
    ) -> bool;
    pub fn match_strlcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const substring_t,
        size: usize,
    ) -> usize;
    pub fn match_strdup(s: *const substring_t) -> *mut ::core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
