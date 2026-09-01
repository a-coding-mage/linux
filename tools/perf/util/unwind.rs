// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/unwind.c. Original includes:
// "debug.h", "symbol_conf.h", "unwind.h", <linux/string.h>, <string.h>, <stdlib.h>

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type bool_ = bool;

pub enum thread {}
pub enum perf_sample {}
pub enum option {}

pub type unwind_entry_cb_t = Option<unsafe extern "C" fn()>;

pub const UNWIND_STYLE_UNKNOWN: c_int = 0;
pub const UNWIND_STYLE_LIBDW: c_int = 1;
pub const UNWIND_STYLE_LIBUNWIND: c_int = 2;
pub const MAX_UNWIND_STYLE: c_int = 3;

const UNWIND_STYLE_ARRAY_SIZE: usize = 2;

#[repr(C)]
pub struct symbol_conf_t {
    pub unwind_style: [c_int; UNWIND_STYLE_ARRAY_SIZE],
}

unsafe extern "C" {
    pub static mut symbol_conf: symbol_conf_t;

    fn libdw__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        data: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool_,
    ) -> c_int;
    fn libunwind__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        data: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool_,
    ) -> c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn strim(s: *mut c_char) -> *mut c_char;
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unwind__get_entries(
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
    thread: *mut thread,
    data: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool_,
) -> c_int {
    let mut ret: c_int = 0;

    // Original C condition:
    // #if defined(HAVE_LIBDW_SUPPORT) || defined(HAVE_LIBUNWIND_SUPPORT)
    #[cfg(any(HAVE_LIBDW_SUPPORT, HAVE_LIBUNWIND_SUPPORT))]
    {
        if symbol_conf.unwind_style[0] == UNWIND_STYLE_UNKNOWN {
            let mut i: usize = 0;
            #[cfg(HAVE_LIBDW_SUPPORT)]
            {
                symbol_conf.unwind_style[i] = UNWIND_STYLE_LIBDW;
                i += 1;
            }
            #[cfg(HAVE_LIBUNWIND_SUPPORT)]
            {
                symbol_conf.unwind_style[i] = UNWIND_STYLE_LIBUNWIND;
                i += 1;
            }
            let _ = i;
        }
    }

    let mut i: size_t = 0;
    while i < UNWIND_STYLE_ARRAY_SIZE {
        match symbol_conf.unwind_style[i] {
            UNWIND_STYLE_LIBDW => {
                ret = libdw__get_entries(cb, arg, thread, data, max_stack, best_effort);
            }
            UNWIND_STYLE_LIBUNWIND => {
                ret = libunwind__get_entries(cb, arg, thread, data, max_stack, best_effort);
            }
            UNWIND_STYLE_UNKNOWN | _ => {
                // Original C condition:
                // #if !defined(HAVE_LIBDW_SUPPORT) && !defined(HAVE_LIBUNWIND_SUPPORT)
                #[cfg(not(any(HAVE_LIBDW_SUPPORT, HAVE_LIBUNWIND_SUPPORT)))]
                {
                    pr_warning_once(
                        b"Error: dwarf unwinding not supported, build perf with libdw or libunwind.\n\0"
                            .as_ptr() as *const c_char,
                    );
                }
                ret = 0;
            }
        }
        if ret > 0 {
            ret = 0;
            break;
        }
        if ret < 0 {
            break;
        }
        i += 1;
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unwind__configure(
    var: *const c_char,
    value: *const c_char,
    _cb: *mut c_void,
) -> c_int {
    static UNWIND_STYLE_NAME_LIBDW: &[u8] = b"libdw\0";
    static UNWIND_STYLE_NAME_LIBUNWIND: &[u8] = b"libunwind\0";
    static UNWIND_STYLE_NAMES: [*const c_char; MAX_UNWIND_STYLE as usize] = [
        core::ptr::null(),
        UNWIND_STYLE_NAME_LIBDW.as_ptr() as *const c_char,
        UNWIND_STYLE_NAME_LIBUNWIND.as_ptr() as *const c_char,
    ];
    let mut s: *mut c_char;
    let mut p: *mut c_char;
    let mut saveptr: *mut c_char = core::ptr::null_mut();
    let mut i: size_t = 0;

    if strcmp(var, b"unwind.style\0".as_ptr() as *const c_char) != 0 {
        return 0;
    }

    if value.is_null() {
        return -1;
    }

    s = strdup(value);
    if s.is_null() {
        return -1;
    }

    memset(
        symbol_conf.unwind_style.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&symbol_conf.unwind_style),
    );

    p = strtok_r(
        s,
        b",\0".as_ptr() as *const c_char,
        &mut saveptr as *mut *mut c_char,
    );
    while !p.is_null() && i < UNWIND_STYLE_ARRAY_SIZE {
        let mut found: bool_ = false;
        let q: *mut c_char = strim(p);

        let mut j: size_t = UNWIND_STYLE_LIBDW as size_t;
        while j < MAX_UNWIND_STYLE as size_t {
            if strcasecmp(q, UNWIND_STYLE_NAMES[j]) == 0 {
                symbol_conf.unwind_style[i] = j as c_int;
                i += 1;
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            pr_warning(
                b"Unknown unwind style: %s\n\0".as_ptr() as *const c_char,
                q,
            );
        }
        p = strtok_r(
            core::ptr::null_mut(),
            b",\0".as_ptr() as *const c_char,
            &mut saveptr as *mut *mut c_char,
        );
    }

    free(s as *mut c_void);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unwind__option(
    _opt: *const option,
    arg: *const c_char,
    _unset: c_int,
) -> c_int {
    unwind__configure(
        b"unwind.style\0".as_ptr() as *const c_char,
        arg,
        core::ptr::null_mut(),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
