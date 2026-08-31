// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/pkey_alloc.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include <linux/log2.h>

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct strarray {
    pub prefix: *const c_char,
    pub nr_entries: c_int,
    pub entries: *const *const c_char,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    pub fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strarray__scnprintf_flags(
    sa: *mut strarray,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
    flags: c_ulong,
) -> usize {
    let mut i: c_int;
    let mut printed: c_int = 0;

    if flags == 0 {
        let s: *const c_char = *(*sa).entries.offset(0);
        if !s.is_null() {
            return scnprintf(
                bf,
                size,
                b"%s%s\0".as_ptr() as *const c_char,
                if show_prefix {
                    (*sa).prefix
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                s,
            );
        }
        return scnprintf(bf, size, b"%d\0".as_ptr() as *const c_char, 0);
    }

    i = 1;
    while i < (*sa).nr_entries {
        let bit: c_ulong = (1 as c_ulong) << (i - 1);

        if flags & bit == 0 {
            i += 1;
            continue;
        }

        if printed != 0 {
            printed += scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as usize),
                b"|\0".as_ptr() as *const c_char,
            ) as c_int;
        }

        if !(*(*sa).entries.offset(i as isize)).is_null() {
            printed += scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as usize),
                b"%s%s\0".as_ptr() as *const c_char,
                if show_prefix {
                    (*sa).prefix
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                *(*sa).entries.offset(i as isize),
            ) as c_int;
        } else {
            printed += scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as usize),
                b"0x%#\0".as_ptr() as *const c_char,
                bit,
            ) as c_int;
        }

        i += 1;
    }

    printed as usize
}

unsafe extern "C" fn pkey_alloc__scnprintf_access_rights(
    access_rights: c_int,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    // C included generated data here:
    // #include "trace/beauty/generated/pkey_alloc_access_rights_array.c"
    // static DEFINE_STRARRAY(pkey_alloc_access_rights, "PKEY_");
    unsafe extern "C" {
        static mut strarray__pkey_alloc_access_rights: strarray;
    }

    strarray__scnprintf_flags(
        &raw mut strarray__pkey_alloc_access_rights,
        bf,
        size,
        show_prefix,
        access_rights as c_ulong,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_pkey_alloc_access_rights(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let cmd: c_ulong = (*arg).val;

    pkey_alloc__scnprintf_access_rights(cmd as c_int, bf, size, (*arg).show_string_prefix)
}
