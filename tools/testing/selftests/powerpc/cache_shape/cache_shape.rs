// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2017, Michael Ellerman, IBM Corp.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C includes removed: elf.h, errno.h, fcntl.h, link.h, stdio.h, stdlib.h,
// string.h, sys/stat.h, sys/types.h, sys/wait.h, unistd.h, and "utils.h".

// Fallback definitions used by the C source when these auxv constants are not
// supplied by the platform headers.
const AT_L1I_CACHESIZE: c_ulong = 40;
const AT_L1I_CACHEGEOMETRY: c_ulong = 41;
const AT_L1D_CACHESIZE: c_ulong = 42;
const AT_L1D_CACHEGEOMETRY: c_ulong = 43;
const AT_L2_CACHESIZE: c_ulong = 44;
const AT_L2_CACHEGEOMETRY: c_ulong = 45;
const AT_L3_CACHESIZE: c_ulong = 46;
const AT_L3_CACHEGEOMETRY: c_ulong = 47;

const KSFT_SKIP: c_int = 4;

#[repr(C)]
union Elf_auxv_t_un {
    a_val: c_ulong,
}

#[repr(C)]
struct Elf_auxv_t {
    a_type: c_ulong,
    a_un: Elf_auxv_t_un,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;

    fn read_auxv(buf: *mut c_void, size: usize) -> c_int;
    fn find_auxv_entry(type_: c_ulong, auxv: *mut c_void) -> *mut Elf_auxv_t;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe fn FAIL_IF(cond: bool) -> Option<c_int> {
    if cond {
        Some(1)
    } else {
        None
    }
}

unsafe fn SKIP_IF(cond: bool) -> Option<c_int> {
    if cond {
        Some(KSFT_SKIP)
    } else {
        None
    }
}

unsafe fn print_size(label: *const c_char, val: u32) {
    unsafe {
        printf(
            c"%s cache size: %#10x %10dB %10dK\n".as_ptr(),
            label,
            val,
            val,
            val / 1024,
        );
    }
}

unsafe fn print_geo(label: *const c_char, val: u32) {
    let assoc: u16;

    unsafe {
        printf(
            c"%s line size:  %#10x       ".as_ptr(),
            label,
            val & 0xFFFF,
        );
    }

    assoc = (val >> 16) as u16;
    unsafe {
        if assoc != 0 {
            printf(c"%u-way".as_ptr(), assoc as c_int);
        } else {
            printf(c"fully".as_ptr());
        }

        printf(c" associative\n".as_ptr());
    }
}

unsafe extern "C" fn test_cache_shape() -> c_int {
    static mut buffer: [c_char; 4096] = [0; 4096];
    let mut p: *mut Elf_auxv_t;
    let mut found: c_int;

    unsafe {
        if let Some(ret) = FAIL_IF(read_auxv(&raw mut buffer as *mut c_void, core::mem::size_of_val(&buffer)) != 0) {
            return ret;
        }
    }

    found = 0;

    unsafe {
        p = find_auxv_entry(AT_L1I_CACHESIZE, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_size(c"L1I ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L1I_CACHEGEOMETRY, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_geo(c"L1I ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L1D_CACHESIZE, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_size(c"L1D ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L1D_CACHEGEOMETRY, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_geo(c"L1D ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L2_CACHESIZE, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_size(c"L2  ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L2_CACHEGEOMETRY, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_geo(c"L2  ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L3_CACHESIZE, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_size(c"L3  ".as_ptr(), (*p).a_un.a_val as u32);
        }

        p = find_auxv_entry(AT_L3_CACHEGEOMETRY, &raw mut buffer as *mut c_void);
        if !p.is_null() {
            found += 1;
            print_geo(c"L3  ".as_ptr(), (*p).a_un.a_val as u32);
        }

        /* If we found none we're probably on a system where they don't exist */
        if let Some(ret) = SKIP_IF(found == 0) {
            return ret;
        }

        /* But if we found any, we expect to find them all */
        if let Some(ret) = FAIL_IF(found != 8) {
            return ret;
        }
    }

    0
}

fn main() -> c_int {
    unsafe { test_harness(test_cache_shape, c"cache_shape".as_ptr()) }
}
