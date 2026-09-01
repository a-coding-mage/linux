// SPDX-License-Identifier: GPL-2.0-only
/************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


Debug macro translation.

************************************************************************/

// C dependencies: "hpi_internal.h", "hpidebug.h"

use core::ffi::{c_char, c_int};

extern "C" {
    fn printk(fmt: *const c_char, ...) -> c_int;
}

/* Debug level; 0 quiet; 1 informative, 2 debug, 3 verbose debug.  */
#[no_mangle]
pub static mut hpi_debug_level: c_int = HPI_DEBUG_LEVEL_DEFAULT;

#[no_mangle]
pub unsafe extern "C" fn hpi_debug_init() {
    // C used KERN_INFO string-prefix concatenation.
    printk(b"debug start\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn hpi_debug_level_set(level: c_int) -> c_int {
    let old_level: c_int;

    old_level = hpi_debug_level;
    hpi_debug_level = level;
    old_level
}

#[no_mangle]
pub unsafe extern "C" fn hpi_debug_level_get() -> c_int {
    hpi_debug_level
}

#[no_mangle]
pub unsafe extern "C" fn hpi_debug_message(phm: *mut hpi_message, sz_fileline: *mut c_char) {
    let _ = sz_fileline;

    if !phm.is_null() {
        // C used KERN_DEBUG string-prefix concatenation.
        printk(
            b"HPI_MSG%d,%d,%d,%d,%d\n\0".as_ptr() as *const c_char,
            (*phm).version,
            (*phm).adapter_index,
            (*phm).obj_index,
            (*phm).function,
            (*phm).u.c.attribute,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn hpi_debug_data(pdata: *mut u16, len: u32) {
    let mut i: u32;
    let mut j: c_int;
    let mut k: c_int;
    let mut lines: c_int;
    let cols: c_int = 8;

    lines = DIV_ROUND_UP(len, cols as u32) as c_int;
    if lines > 8 {
        lines = 8;
    }

    i = 0;
    j = 0;
    while j < lines {
        // C used KERN_DEBUG string-prefix concatenation.
        printk(
            b"%p:\0".as_ptr() as *const c_char,
            pdata.add(i as usize),
        );

        k = 0;
        while k < cols && i < len {
            // C used KERN_CONT string-prefix concatenation.
            printk(
                b"%s%04x\0".as_ptr() as *const c_char,
                if k == 0 {
                    b"\0".as_ptr()
                } else {
                    b" \0".as_ptr()
                } as *const c_char,
                *pdata.add(i as usize) as c_int,
            );
            i = i.wrapping_add(1);
            k += 1;
        }

        // C used KERN_CONT string-prefix concatenation.
        printk(b"\n\0".as_ptr() as *const c_char);
        j += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
