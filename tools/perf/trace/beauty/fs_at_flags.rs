// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/fs_at_flags.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::os::raw::{c_char, c_int, c_ulong};

pub type size_t = usize;

/*
 * Dependencies from:
 *   "trace/beauty/beauty.h"
 *   <sys/types.h>
 *   <linux/fcntl.h>
 *   <linux/log2.h>
 *   "trace/beauty/generated/fs_at_flags_array.c"
 */

/*
 * uapi/linux/fcntl.h does not keep a copy in tools headers directory,
 * for system with kernel versions before v5.8, need to sync AT_EACCESS macro.
 */
pub const AT_EACCESS: c_ulong = 0x200;

#[repr(C)]
pub struct strarray {
    pub prefix: *const c_char,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    static strarray__fs_at_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
        flags: c_ulong,
    ) -> size_t;

    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
}

unsafe fn fs_at__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    unsafe {
        strarray__scnprintf_flags(
            &raw const strarray__fs_at_flags,
            bf,
            size,
            show_prefix,
            flags,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_fs_at_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let show_prefix: bool = unsafe { (*arg).show_string_prefix };
    let flags: c_int = unsafe { (*arg).val as c_int };

    unsafe { fs_at__scnprintf_flags(flags as c_ulong, bf, size, show_prefix) }
}

unsafe fn faccessat2__scnprintf_flags(
    mut flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    let mut printed: c_int = 0;

    // AT_EACCESS is the same as AT_REMOVEDIR, that is in fs_at_flags_array,
    // special case it here.
    if flags & AT_EACCESS != 0 {
        flags &= !AT_EACCESS;
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%sEACCESS%s".as_ptr(),
                if show_prefix {
                    strarray__fs_at_flags.prefix
                } else {
                    c"".as_ptr()
                },
                if flags != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
            )
        };
    }

    unsafe {
        strarray__scnprintf_flags(
            &raw const strarray__fs_at_flags,
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            show_prefix,
            flags,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_faccessat2_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let show_prefix: bool = unsafe { (*arg).show_string_prefix };
    let flags: c_int = unsafe { (*arg).val as c_int };

    unsafe { faccessat2__scnprintf_flags(flags as c_ulong, bf, size, show_prefix) }
}
