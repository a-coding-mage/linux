// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/cone.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies translated from:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include <linux/log2.h>
// #include <sys/types.h>
// #include <sched.h>

use std::ffi::c_char;

pub type size_t = usize;

#[repr(C)]
pub struct syscall_arg {
    pub val: u64,
    pub mask: u64,
    pub show_string_prefix: bool,
}

pub enum strarray {}

unsafe extern "C" {
    static strarray__clone_flags: strarray;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
        flags: u64,
    ) -> size_t;
}

const CLONE_SETTLS: u64 = 0x0008_0000;
const CLONE_PARENT_SETTID: u64 = 0x0010_0000;
const CLONE_CHILD_CLEARTID: u64 = 0x0020_0000;
const CLONE_CHILD_SETTID: u64 = 0x0100_0000;

unsafe fn clone__scnprintf_flags(
    flags: u64,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // Generated in C by:
    // #include "trace/beauty/generated/clone_flags_array.c"
    // static DEFINE_STRARRAY(clone_flags, "CLONE_");

    unsafe {
        strarray__scnprintf_flags(
            &strarray__clone_flags as *const strarray,
            bf,
            size,
            show_prefix,
            flags,
        )
    }
}

#[repr(u64)]
enum syscall_clone_args {
    SCC_FLAGS = 1 << 0,
    SCC_CHILD_STACK = 1 << 1,
    SCC_PARENT_TIDPTR = 1 << 2,
    SCC_CHILD_TIDPTR = 1 << 3,
    SCC_TLS = 1 << 4,
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_clone_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let flags = unsafe { (*arg).val };

    if (flags & CLONE_PARENT_SETTID) == 0 {
        unsafe {
            (*arg).mask |= syscall_clone_args::SCC_PARENT_TIDPTR as u64;
        }
    }

    if (flags & (CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID)) == 0 {
        unsafe {
            (*arg).mask |= syscall_clone_args::SCC_CHILD_TIDPTR as u64;
        }
    }

    if (flags & CLONE_SETTLS) == 0 {
        unsafe {
            (*arg).mask |= syscall_clone_args::SCC_TLS as u64;
        }
    }

    unsafe { clone__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
