// SPDX-License-Identifier: LGPL-2.1
// Copyright (C) 2022, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

// C dependencies in the original source:
// #include "trace/beauty/beauty.h"
// #include <inttypes.h>
// #include <time.h>

use std::ffi::{c_char, c_ulong};

pub type size_t = usize;

#[repr(C)]
pub struct timespec {
    pub tv_sec: u64,
    pub tv_nsec: u64,
}

#[repr(C)]
pub struct syscall_arg_augmented_args {
    pub value: *mut std::ffi::c_void,
}

#[repr(C)]
pub struct syscall_arg_augmented {
    pub args: *mut syscall_arg_augmented_args,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub augmented: syscall_arg_augmented,
}

unsafe extern "C" {
    pub fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
}

unsafe fn syscall_arg__scnprintf_augmented_timespec(
    arg: *mut syscall_arg,
    bf: *mut c_char,
    size: size_t,
) -> size_t {
    let ts = (*(*arg).augmented.args).value as *mut timespec;

    // PRIu64 from <inttypes.h> is represented here with the Linux unsigned long
    // spelling used for uint64_t formatting on this target family.
    scnprintf(
        bf,
        size,
        c"{ .tv_sec: %lu, .tv_nsec: %lu }".as_ptr(),
        (*ts).tv_sec,
        (*ts).tv_nsec,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_timespec(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    if !(*arg).augmented.args.is_null() {
        return syscall_arg__scnprintf_augmented_timespec(arg, bf, size);
    }

    scnprintf(bf, size, c"%#lx".as_ptr(), (*arg).val)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
