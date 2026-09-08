/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2007 Jon Loeliger, Freescale Semiconductor, Inc.
 */

// C header guard SRCPOS_H.
// Dependencies: stdio.h, stdbool.h, and util.h are supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_void};

// Opaque C FILE type supplied by stdio.h.
pub enum FILE {}

#[repr(C)]
pub struct srcfile_state {
    pub f: *mut FILE,
    pub name: *mut c_char,
    pub dir: *mut c_char,
    pub lineno: c_int,
    pub colno: c_int,
    pub prev: *mut srcfile_state,
}

extern "C" {
    pub static mut depfile: *mut FILE; // = NULL
    pub static mut current_srcfile: *mut srcfile_state; // = NULL
}

/**
 * Open a source file.
 *
 * If the source file is a relative pathname, then it is searched for in the
 * current directory (the directory of the last source file read) and after
 * that in the search path.
 *
 * We work through the search path in order from the first path specified to the
 * last.
 *
 * If the file is not found, then this function does not return, but calls
 * die().
 *
 * @param fname      Filename to search
 * @param fullnamep  If non-NULL, it is set to the allocated filename of the
 *                   file that was opened. The caller is then responsible
 *                   for freeing the pointer.
 * @return pointer to opened FILE
 */
extern "C" {
    pub fn srcfile_relative_open(fname: *const c_char, fullnamep: *mut *mut c_char) -> *mut FILE;
    pub fn srcfile_push(fname: *const c_char);
    pub fn srcfile_pop() -> bool;
    pub fn srcfile_add_search_path(dirname: *const c_char);
}

#[repr(C)]
pub struct srcpos {
    pub first_line: c_int,
    pub first_column: c_int,
    pub last_line: c_int,
    pub last_column: c_int,
    pub file: *mut srcfile_state,
    pub next: *mut srcpos,
}

// #define YYLTYPE struct srcpos
pub type YYLTYPE = srcpos;

// YYRHSLOC is supplied by the parser translation.
#[macro_export]
macro_rules! YYLLOC_DEFAULT {
    ($Current:expr, $Rhs:expr, $N:expr) => {{
        if $N != 0 {
            $Current.first_line = $Rhs[1].first_line;
            $Current.first_column = $Rhs[1].first_column;
            $Current.last_line = $Rhs[$N].last_line;
            $Current.last_column = $Rhs[$N].last_column;
            $Current.file = $Rhs[$N].file;
        } else {
            $Current.first_line = $Rhs[0].last_line;
            $Current.last_line = $Rhs[0].last_line;
            $Current.first_column = $Rhs[0].last_column;
            $Current.last_column = $Rhs[0].last_column;
            $Current.file = $Rhs[0].file;
        }
        $Current.next = core::ptr::null_mut();
    }};
}

extern "C" {
    pub fn srcpos_update(pos: *mut srcpos, text: *const c_char, len: c_int);
    pub fn srcpos_copy(pos: *mut srcpos) -> *mut srcpos;
    pub fn srcpos_extend(new_srcpos: *mut srcpos, old_srcpos: *mut srcpos) -> *mut srcpos;
    pub fn srcpos_free(pos: *mut srcpos);
    pub fn srcpos_string(pos: *mut srcpos) -> *mut c_char;
    pub fn srcpos_string_first(pos: *mut srcpos, level: c_int) -> *mut c_char;
    pub fn srcpos_string_last(pos: *mut srcpos, level: c_int) -> *mut c_char;

    // C declaration uses PRINTF(3, 0).
    pub fn srcpos_verror(
        pos: *mut srcpos,
        prefix: *const c_char,
        fmt: *const c_char,
        va: *mut c_void,
    );
    // C declaration uses PRINTF(3, 4).
    pub fn srcpos_error(pos: *mut srcpos, prefix: *const c_char, fmt: *const c_char, ...);

    pub fn srcpos_set_line(f: *mut c_char, l: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
