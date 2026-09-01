// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/fspick.c
 *
 *  Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies:
//   #include "trace/beauty/beauty.h"
//   #include <linux/log2.h>

use core::ffi::{c_char, c_ulong};

pub type size_t = usize;

extern "C" {
    static strarray__fspick_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
        flags: c_ulong,
    ) -> size_t;
}

// External type supplied by trace/beauty/beauty.h.
pub enum strarray {}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

unsafe fn fspick__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // C included "trace/beauty/generated/fspick_arrays.c" here and declared:
    //   static DEFINE_STRARRAY(fspick_flags, "FSPICK_");
    strarray__scnprintf_flags(
        &strarray__fspick_flags as *const strarray,
        bf,
        size,
        show_prefix,
        flags,
    )
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_fspick_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let flags: c_ulong = (*arg).val;

    fspick__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
