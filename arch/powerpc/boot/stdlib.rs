// SPDX-License-Identifier: GPL-2.0-only
/*
 * stdlib functions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependency intent: declarations supplied by "stdlib.h" are represented
// directly here; no local implementation of those dependencies is added.

/* Not currently supported: leading whitespace, sign, 0x prefix, zero base */
#[no_mangle]
pub unsafe extern "C" fn strtoull(
    mut ptr: *const core::ffi::c_char,
    end: *mut *mut core::ffi::c_char,
    base: core::ffi::c_int,
) -> core::ffi::c_ulonglong {
    let mut ret: core::ffi::c_ulonglong = 0;

    if base > 36 {
        if !end.is_null() {
            *end = ptr as *mut core::ffi::c_char;
        }
        return ret;
    }

    while *ptr != 0 {
        let digit: core::ffi::c_int;
        let ch = *ptr as u8;

        if ch >= b'0' && ch <= b'9' && (ch as core::ffi::c_int) < b'0' as core::ffi::c_int + base {
            digit = (ch - b'0') as core::ffi::c_int;
        } else if (ch as core::ffi::c_int) >= b'A' as core::ffi::c_int
            && (ch as core::ffi::c_int) < b'A' as core::ffi::c_int + base - 10
        {
            digit = (ch - b'A' + 10) as core::ffi::c_int;
        } else if (ch as core::ffi::c_int) >= b'a' as core::ffi::c_int
            && (ch as core::ffi::c_int) < b'a' as core::ffi::c_int + base - 10
        {
            digit = (ch - b'a' + 10) as core::ffi::c_int;
        } else {
            break;
        }

        ret = ret.wrapping_mul(base as core::ffi::c_ulonglong);
        ret = ret.wrapping_add(digit as core::ffi::c_ulonglong);
        ptr = ptr.add(1);
    }

    if !end.is_null() {
        *end = ptr as *mut core::ffi::c_char;
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
