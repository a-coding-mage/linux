// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/mount_flags.c
 *
 *  Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_ulong, c_void};

// Dependencies from:
// - trace/beauty/beauty.h
// - linux/compiler.h
// - linux/kernel.h
// - linux/log2.h
// - sys/mount.h
// - trace/beauty/generated/mount_flags_array.c

unsafe extern "C" {
    static strarray__mount_flags: c_void;

    fn strarray__scnprintf_flags(
        strarray: *const c_void,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;
}

const MS_MGC_VAL: c_ulong = 0xC0ED0000;
const MS_MGC_MSK: c_ulong = 0xffff0000;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

unsafe fn mount__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    /*
     * C source includes "trace/beauty/generated/mount_flags_array.c" here and
     * then declares:
     * static DEFINE_STRARRAY(mount_flags, "MS_");
     */
    unsafe {
        strarray__scnprintf_flags(
            &strarray__mount_flags as *const c_void,
            bf,
            size,
            show_prefix,
            flags,
        )
    }
}

pub unsafe extern "C" fn syscall_arg__mask_val_mount_flags(
    _arg: *mut syscall_arg,
    mut flags: c_ulong,
) -> c_ulong {
    // do_mount in fs/namespace.c:
    /*
     * Pre-0.97 versions of mount() didn't have a flags word.  When the
     * flags word was introduced its top half was required to have the
     * magic value 0xC0ED, and this remained so until 2.4.0-test9.
     * Therefore, if this magic number is present, it carries no
     * information and must be discarded.
     */
    if (flags & MS_MGC_MSK) == MS_MGC_VAL {
        flags &= !MS_MGC_MSK;
    }

    flags
}

pub unsafe extern "C" fn syscall_arg__scnprintf_mount_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags: c_ulong = unsafe { (*arg).val };

    unsafe { mount__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}
