// SPDX-License-Identifier: LGPL-2.1
// C dependencies: "trace/beauty/beauty.h", <sys/types.h>, <sys/wait.h>

use core::ffi::{c_char, c_int};

extern "C" {
    static WNOHANG: c_int;
    static WUNTRACED: c_int;
    static WCONTINUED: c_int;

    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_waitid_options(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix: bool = (*arg).show_string_prefix;
    let prefix: *const c_char = b"W\0".as_ptr() as *const c_char;
    let mut printed: c_int = 0;
    let mut options: c_int = (*arg).val;

    if options & WNOHANG != 0 {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            b"%s%s%s\0".as_ptr() as *const c_char,
            if printed != 0 {
                b"|\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"NOHANG\0".as_ptr() as *const c_char,
        );
        options &= !WNOHANG;
    }

    if options & WUNTRACED != 0 {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            b"%s%s%s\0".as_ptr() as *const c_char,
            if printed != 0 {
                b"|\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"UNTRACED\0".as_ptr() as *const c_char,
        );
        options &= !WUNTRACED;
    }

    if options & WCONTINUED != 0 {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            b"%s%s%s\0".as_ptr() as *const c_char,
            if printed != 0 {
                b"|\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"CONTINUED\0".as_ptr() as *const c_char,
        );
        options &= !WCONTINUED;
    }

    if options != 0 {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            b"%s%#x\0".as_ptr() as *const c_char,
            if printed != 0 {
                b"|\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            options,
        );
    }

    printed as usize
}
