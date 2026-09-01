// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/sync_file_range.c
 *
 *  Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <linux/log2.h>
// #include <linux/fs.h>

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
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;

    static strarray__sync_file_range_flags: strarray;
}

// If SYNC_FILE_RANGE_WRITE_AND_WAIT is not supplied by <linux/fs.h>, the C file
// defines these fallback constants.
const SYNC_FILE_RANGE_WAIT_BEFORE: c_ulong = 1;
const SYNC_FILE_RANGE_WRITE: c_ulong = 2;
const SYNC_FILE_RANGE_WAIT_AFTER: c_ulong = 4;
const SYNC_FILE_RANGE_WRITE_AND_WAIT: c_ulong =
    SYNC_FILE_RANGE_WRITE | SYNC_FILE_RANGE_WAIT_BEFORE | SYNC_FILE_RANGE_WAIT_AFTER;

unsafe fn sync_file_range__scnprintf_flags(
    mut flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    // C included generated data here:
    // #include "trace/beauty/generated/sync_file_range_arrays.c"
    // static DEFINE_STRARRAY(sync_file_range_flags, "SYNC_FILE_RANGE_");
    let mut printed: usize = 0;

    if flags & SYNC_FILE_RANGE_WRITE_AND_WAIT == SYNC_FILE_RANGE_WRITE_AND_WAIT {
        printed += unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                c"%s%s".as_ptr(),
                if show_prefix {
                    c"SYNC_FILE_RANGE_".as_ptr()
                } else {
                    c"".as_ptr()
                },
                c"WRITE_AND_WAIT".as_ptr(),
            )
        };
        flags &= !SYNC_FILE_RANGE_WRITE_AND_WAIT;
    }

    printed
        + unsafe {
            strarray__scnprintf_flags(
                &raw const strarray__sync_file_range_flags,
                bf.add(printed),
                size.wrapping_sub(printed),
                show_prefix,
                flags,
            )
        }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_sync_file_range_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags: c_ulong = unsafe { (*arg).val };

    unsafe { sync_file_range__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
