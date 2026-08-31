// SPDX-License-Identifier: GPL-2.0
// C dependencies: <sys/types.h>, <linux/kernel.h>, <stdio.h>

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, VaListImpl};

pub type size_t = usize;
pub type ssize_t = isize;

unsafe extern "C" {
    pub fn vsnprintf(
        buf: *mut c_char,
        size: size_t,
        fmt: *const c_char,
        args: VaListImpl<'_, '_>,
    ) -> c_int;
}

pub unsafe extern "C" fn vscnprintf(
    buf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    args: VaListImpl<'_, '_>,
) -> c_int {
    let i: c_int = unsafe { vsnprintf(buf, size, fmt, args) };
    let ssize: ssize_t = size as ssize_t;

    if (i as ssize_t) >= ssize {
        (ssize - 1) as c_int
    } else {
        i
    }
}

pub unsafe extern "C" fn scnprintf(
    buf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let ssize: ssize_t = size as ssize_t;
    let i: c_int;

    i = unsafe { vsnprintf(buf, size, fmt, args.as_va_list()) };

    if (i as ssize_t) >= ssize {
        (ssize - 1) as c_int
    } else {
        i
    }
}

pub unsafe extern "C" fn scnprintf_pad(
    buf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let ssize: ssize_t = size as ssize_t;
    let mut i: c_int;

    i = unsafe { vscnprintf(buf, size, fmt, args.as_va_list()) };

    if i < size as c_int {
        while i < size as c_int {
            unsafe {
                *buf.offset(i as isize) = b' ' as c_char;
            }
            i += 1;
        }
        unsafe {
            *buf.offset(i as isize) = 0x0 as c_char;
        }
    }

    if (i as ssize_t) >= ssize {
        (ssize - 1) as c_int
    } else {
        i
    }
}
