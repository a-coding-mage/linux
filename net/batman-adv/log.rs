// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

// Dependencies supplied by the corresponding C headers:
// #include "log.h"
// #include "main.h"
// #include <linux/stdarg.h>
// #include "trace.h"

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut c_void,
}

extern "C" {
    fn trace_batadv_dbg(bat_priv: *mut batadv_priv, vaf: *mut va_format);
}

/**
 * batadv_debug_log() - Add debug log entry
 * @bat_priv: the bat priv with all the mesh interface information
 * @fmt: format string
 *
 * Return: 0 on success or negative error number in case of failure
 */
#[no_mangle]
pub unsafe extern "C" fn batadv_debug_log(
    bat_priv: *mut batadv_priv,
    fmt: *const c_char,
    // The C varargs list is represented opaquely here; its address is passed
    // to va_format exactly as in the source.
    args: *mut c_void,
) -> i32 {
    let mut vaf = va_format {
        fmt,
        va: args,
    };

    trace_batadv_dbg(bat_priv, &mut vaf);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
