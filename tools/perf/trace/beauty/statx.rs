// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/statx.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::os::raw::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: c_ulong,
}

extern "C" {
    /*
     * From:
     *   #include "trace/beauty/generated/statx_mask_array.c"
     *   static DEFINE_STRARRAY(statx_mask, "STATX_");
     */
    static strarray__statx_mask: strarray;

    fn strarray__scnprintf_flags(
        strarray: *const strarray,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;
}

unsafe fn statx__scnprintf_mask(
    mask: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    strarray__scnprintf_flags(
        &strarray__statx_mask as *const strarray,
        bf,
        size,
        show_prefix,
        mask,
    )
}

pub unsafe extern "C" fn syscall_arg__scnprintf_statx_mask(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix: bool = (*arg).show_string_prefix;
    let mask: c_int = (*arg).val as c_int;

    statx__scnprintf_mask(mask as c_ulong, bf, size, show_prefix)
}
