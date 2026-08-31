// SPDX-License-Identifier: LGPL-2.1
// Translated from perf/trace/beauty/seccomp.c.
// Depends on declarations from trace/beauty/beauty.h.

use core::ffi::{c_char, c_int};

pub type size_t = usize;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_int,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    fn scnprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
}

const SECCOMP_SET_MODE_STRICT: c_int = 0;
const SECCOMP_SET_MODE_FILTER: c_int = 1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_seccomp_op(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let prefix = c"SECCOMP_SET_MODE_".as_ptr();
    let op = unsafe { (*arg).val };
    let mut printed: size_t = 0;

    match op {
        SECCOMP_SET_MODE_STRICT => {
            printed = unsafe {
                scnprintf(
                    bf,
                    size,
                    c"%s%s".as_ptr(),
                    if show_prefix {
                        prefix
                    } else {
                        c"".as_ptr()
                    },
                    c"STRICT".as_ptr(),
                )
            } as size_t;
        }
        SECCOMP_SET_MODE_FILTER => {
            printed = unsafe {
                scnprintf(
                    bf,
                    size,
                    c"%s%s".as_ptr(),
                    if show_prefix {
                        prefix
                    } else {
                        c"".as_ptr()
                    },
                    c"FILTER".as_ptr(),
                )
            } as size_t;
        }
        _ => {
            printed = unsafe { scnprintf(bf, size, c"%#x".as_ptr(), op) } as size_t;
        }
    }

    printed
}

const SECCOMP_FILTER_FLAG_TSYNC: c_int = 1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_seccomp_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let prefix = c"SECCOMP_FILTER_FLAG_".as_ptr();
    let mut printed: c_int = 0;
    let mut flags = unsafe { (*arg).val };

    if flags & SECCOMP_FILTER_FLAG_TSYNC != 0 {
        printed += unsafe {
            scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as size_t),
                c"%s%s%s".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                if show_prefix {
                    prefix
                } else {
                    c"".as_ptr()
                },
                c"TSYNC".as_ptr(),
            )
        };
        flags &= !SECCOMP_FILTER_FLAG_TSYNC;
    }

    if flags != 0 {
        printed += unsafe {
            scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as size_t),
                c"%s%#x".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                flags,
            )
        };
    }

    printed as size_t
}
