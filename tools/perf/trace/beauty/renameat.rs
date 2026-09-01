// SPDX-License-Identifier: LGPL-2.1
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

// C dependency: trace/beauty/beauty.h

use core::ffi::{c_char, c_ulong};

extern "C" {
    static strarray__rename_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;
}

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_arg {
    pub val: u64,
    pub show_string_prefix: bool,
}

unsafe fn renameat2__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    // C included generated data here:
    // #include "trace/beauty/generated/rename_flags_array.c"
    // static DEFINE_STRARRAY(rename_flags, "RENAME_");
    strarray__scnprintf_flags(
        &strarray__rename_flags as *const strarray,
        bf,
        size,
        show_prefix,
        flags,
    )
}

pub unsafe extern "C" fn syscall_arg__scnprintf_renameat2_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags = (*arg).val as c_ulong;
    renameat2__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
