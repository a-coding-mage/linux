// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/kcmp.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::os::raw::{c_char, c_int, c_ulong};

pub type size_t = usize;
pub type pid_t = c_int;

pub const KCMP_FILE: c_int = 0;

#[repr(C)]
pub struct trace;

#[repr(C)]
pub struct strarray;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub idx: c_int,
    pub mask: c_ulong,
    pub trace: *mut trace,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    fn syscall_arg__val(arg: *mut syscall_arg, idx: c_int) -> c_ulong;
    fn syscall_arg__scnprintf_long(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn pid__scnprintf_fd(
        trace: *mut trace,
        pid: pid_t,
        fd: c_ulong,
        bf: *mut c_char,
        size: size_t,
    ) -> size_t;

    // From trace/beauty/generated/kcmp_type_array.c via DEFINE_STRARRAY(kcmp_types, "KCMP_").
    static strarray__kcmp_types: strarray;

    fn strarray__scnprintf(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        fmt: *const c_char,
        show_prefix: bool,
        val: c_int,
    ) -> size_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_kcmp_idx(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let fd: c_ulong = unsafe { (*arg).val };
    let type_: c_int = unsafe { syscall_arg__val(arg, 2) as c_int };
    let pid: pid_t;

    if type_ != KCMP_FILE {
        return unsafe { syscall_arg__scnprintf_long(bf, size, arg) };
    }

    pid = unsafe { syscall_arg__val(arg, if (*arg).idx == 3 { 0 } else { 1 }) as pid_t };
    /* idx1 -> pid1, idx2 -> pid2 */
    unsafe { pid__scnprintf_fd((*arg).trace, pid, fd, bf, size) }
}

unsafe fn kcmp__scnprintf_type(
    type_: c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    unsafe {
        strarray__scnprintf(
            &strarray__kcmp_types,
            bf,
            size,
            c"%d".as_ptr(),
            show_prefix,
            type_,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_kcmp_type(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let type_: c_ulong = unsafe { (*arg).val };

    if type_ != KCMP_FILE as c_ulong {
        unsafe {
            (*arg).mask |= (1 << 3) | (1 << 4);
        }
        /* Ignore idx1 and idx2 */
    }

    unsafe { kcmp__scnprintf_type(type_ as c_int, bf, size, (*arg).show_string_prefix) }
}
