/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generate kernel symbol version hashes.
   Copyright 1996, 1997 Linux International.

   New implementation contributed by Richard Henderson <rth@tamu.edu>
   Based on original work by Bjorn Ekwall <bj0rn@blox.se>

   This file is part of the Linux modutils.

 */

// C dependencies: <stdbool.h>, <stdio.h>, and <list_types.h>.

use core::ffi::{c_char, c_int, c_void};

// `hlist_node` is supplied by the translated list_types dependency.
use crate::list_types::hlist_node;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_type {
    SYM_NORMAL,
    SYM_TYPEDEF,
    SYM_ENUM,
    SYM_STRUCT,
    SYM_UNION,
    SYM_ENUM_CONST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_status {
    STATUS_UNCHANGED,
    STATUS_DEFINED,
    STATUS_MODIFIED,
}

#[repr(C)]
pub struct string_list {
    pub next: *mut string_list,
    pub tag: symbol_type,
    pub in_source_file: c_int,
    pub string: *mut c_char,
}

#[repr(C)]
pub struct symbol {
    pub hnode: hlist_node,
    pub name: *mut c_char,
    pub type_: symbol_type,
    pub defn: *mut string_list,
    pub expansion_trail: *mut symbol,
    pub visited: *mut symbol,
    pub is_extern: c_int,
    pub is_declared: c_int,
    pub status: symbol_status,
    pub is_override: c_int,
}

pub type yystype = *mut *mut string_list;
pub type YYSTYPE = yystype;

extern "C" {
    pub static mut cur_line: c_int;
    pub static mut cur_filename: *mut c_char;
    pub static mut in_source_file: c_int;

    pub fn find_symbol(name: *const c_char, ns: symbol_type, exact: c_int) -> *mut symbol;
    pub fn add_symbol(
        name: *const c_char,
        type_: symbol_type,
        defn: *mut string_list,
        is_extern: c_int,
    ) -> *mut symbol;
    pub fn export_symbol(name: *const c_char);

    pub fn free_node(list: *mut string_list);
    pub fn free_list(s: *mut string_list, e: *mut string_list);
    pub fn copy_node(list: *mut string_list) -> *mut string_list;
    pub fn copy_list_range(start: *mut string_list, end: *mut string_list) -> *mut string_list;

    pub fn yylex() -> c_int;
    pub fn yyparse() -> c_int;

    pub static mut dont_want_type_specifier: bool;

    pub fn error_with_pos(fmt: *const c_char, ...);
}

/*----------------------------------------------------------------------*/
#[macro_export]
macro_rules! xmalloc {
    ($size:expr) => {{
        let __ptr = unsafe { ::libc::malloc($size) };
        if __ptr.is_null() && $size != 0 {
            unsafe {
                ::libc::fprintf(::libc::stderr, b"out of memory\n\0".as_ptr() as *const c_char);
                ::libc::exit(1);
            }
        }
        __ptr
    }};
}

#[macro_export]
macro_rules! xstrdup {
    ($str:expr) => {{
        let __str = unsafe { ::libc::strdup($str) };
        if __str.is_null() {
            unsafe {
                ::libc::fprintf(::libc::stderr, b"out of memory\n\0".as_ptr() as *const c_char);
                ::libc::exit(1);
            }
        }
        __str
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
