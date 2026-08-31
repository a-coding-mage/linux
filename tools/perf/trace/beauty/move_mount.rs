// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/move_mount.c
 *
 *  Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <linux/log2.h>

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

unsafe extern "C" {
    // Generated in C by:
    // #include "trace/beauty/generated/move_mount_flags_array.c"
    // static DEFINE_STRARRAY(move_mount_flags, "MOVE_MOUNT_");
    static strarray__move_mount_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;
}

unsafe fn move_mount__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    unsafe {
        strarray__scnprintf_flags(
            &raw const strarray__move_mount_flags,
            bf,
            size,
            show_prefix,
            flags,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_move_mount_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags: c_ulong = unsafe { (*arg).val };

    unsafe { move_mount__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}
