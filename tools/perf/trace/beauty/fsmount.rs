// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/fsmount.c
 *
 *  Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::ffi::{c_char, c_ulong};

type size_t = usize;

// Dependencies from:
// - trace/beauty/beauty.h
// - trace/beauty/generated/fsmount_arrays.c
// - trace/beauty/generated/fsmount_attr_arrays.c
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
    static strarray__fsmount_flags: strarray;
    static strarray__fsmount_attr_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
        flags: c_ulong,
    ) -> size_t;

    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
}

const MOUNT_ATTR__ATIME: c_ulong = 0x00000070; /* Setting on how atime should be updated */
const MOUNT_ATTR_RELATIME: c_ulong = 0x00000000; /* - Update atime relative to mtime/ctime. */

unsafe fn fsmount__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // C included trace/beauty/generated/fsmount_arrays.c here and declared:
    // static DEFINE_STRARRAY(fsmount_flags, "FSMOUNT_");
    unsafe { strarray__scnprintf_flags(&raw const strarray__fsmount_flags, bf, size, show_prefix, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_fsmount_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let flags: c_ulong = unsafe { (*arg).val };

    unsafe { fsmount__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}

unsafe fn fsmount__scnprintf_attr_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // C included trace/beauty/generated/fsmount_attr_arrays.c here and declared:
    // static DEFINE_STRARRAY(fsmount_attr_flags, "MOUNT_ATTR_");
    let mut printed: size_t = 0;

    if (flags & !MOUNT_ATTR__ATIME) != 0 {
        printed = printed.wrapping_add(unsafe {
            strarray__scnprintf_flags(
                &raw const strarray__fsmount_attr_flags,
                bf,
                size,
                show_prefix,
                flags,
            )
        });
    }

    if (flags & MOUNT_ATTR__ATIME) == MOUNT_ATTR_RELATIME {
        let sep = if printed != 0 {
            c"|".as_ptr()
        } else {
            c"".as_ptr()
        };
        let prefix = if show_prefix {
            c"MOUNT_ATTR_".as_ptr()
        } else {
            c"".as_ptr()
        };

        printed = printed.wrapping_add(unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                c"%s%s%s".as_ptr(),
                sep,
                prefix,
                c"RELATIME".as_ptr(),
            )
        });
    }

    printed
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_fsmount_attr_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let flags: c_ulong = unsafe { (*arg).val };

    unsafe { fsmount__scnprintf_attr_flags(flags, bf, size, (*arg).show_string_prefix) }
}
