// SPDX-License-Identifier: LGPL-2.1

// C dependency intent: #include "trace/beauty/beauty.h"

use core::ffi::{c_char, c_int};

pub type size_t = usize;

pub const EFD_SEMAPHORE: c_int = 1;
pub const EFD_NONBLOCK: c_int = 0o0004000;
pub const EFD_CLOEXEC: c_int = 0o2000000;

#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: c_int,
}

unsafe extern "C" {
    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_eventfd_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let show_prefix: bool = unsafe { (*arg).show_string_prefix };
    let prefix: *const c_char = c"EFD_".as_ptr();
    let mut printed: c_int = 0;
    let mut flags: c_int = unsafe { (*arg).val };

    if flags == 0 {
        return unsafe { scnprintf(bf, size, c"NONE".as_ptr()) as size_t };
    }

    if flags & EFD_SEMAPHORE != 0 {
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
                if show_prefix { prefix } else { c"".as_ptr() },
                c"SEMAPHORE".as_ptr(),
            )
        };
        flags &= !EFD_SEMAPHORE;
    }

    if flags & EFD_CLOEXEC != 0 {
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
                if show_prefix { prefix } else { c"".as_ptr() },
                c"CLOEXEC".as_ptr(),
            )
        };
        flags &= !EFD_CLOEXEC;
    }

    if flags & EFD_NONBLOCK != 0 {
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
                if show_prefix { prefix } else { c"".as_ptr() },
                c"NONBLOCK".as_ptr(),
            )
        };
        flags &= !EFD_NONBLOCK;
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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
