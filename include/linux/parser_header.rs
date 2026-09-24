/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/parser.h
 *
 * Header for lib/parser.c
 * Intended use of these functions is parsing filesystem argument lists,
 * but could potentially be used anywhere else that simple option=arg
 * parsing is required.
 */

//! Parser declarations using the layouts generated from linux/parser.h.
pub use kernel::bindings::{match_table_t, substring_t, MAX_OPT_ARGS};

/// Original table-entry type, without also importing the same-named C function.
#[allow(non_camel_case_types)]
pub type match_token = kernel::bindings::match_token;

#[allow(missing_docs)] // These declarations mirror the documented C header.
unsafe extern "C" {
    pub fn match_token(
        s: *mut kernel::ffi::c_char,
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
        pattern: *const kernel::ffi::c_char,
        str_: *const kernel::ffi::c_char,
    ) -> bool;
    pub fn match_strlcpy(
        dest: *mut kernel::ffi::c_char,
        src: *const substring_t,
        size: usize,
    ) -> usize;
    pub fn match_strdup(s: *const substring_t) -> *mut kernel::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
