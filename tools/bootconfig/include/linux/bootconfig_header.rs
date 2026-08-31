/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent:
// #include <stdio.h>
// #include <stdlib.h>
// #include <stdint.h>
// #include <stdbool.h>
// #include <ctype.h>
// #include <errno.h>
// #include <string.h>

// C compatibility macro intent:
// #ifndef fallthrough
// # define fallthrough
// #endif

#[macro_export]
macro_rules! fallthrough {
    () => {};
}

unsafe extern "C" {
    fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn isspace(c: core::ffi::c_int) -> core::ffi::c_int;
}

#[macro_export]
macro_rules! WARN_ON {
    ($cond:expr) => {{
        if $cond {
            unsafe {
                printf(
                    concat!(
                        "Internal warning(%s:%d, %s): ",
                        stringify!($cond),
                        "\n\0"
                    )
                    .as_ptr() as *const core::ffi::c_char,
                    concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
                    line!() as core::ffi::c_int,
                    concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                )
            }
        } else {
            0
        }
    }};
}

#[macro_export]
macro_rules! unlikely {
    ($cond:expr) => {
        $cond
    };
}

/* Copied from lib/string.c */
#[inline]
pub unsafe fn skip_spaces(str_: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    let mut str_ = str_;

    while unsafe { isspace(unsafe { *str_ } as core::ffi::c_int) } != 0 {
        str_ = unsafe { str_.add(1) };
    }

    str_ as *mut core::ffi::c_char
}

#[inline]
pub unsafe fn strim(s: *mut core::ffi::c_char) -> *mut core::ffi::c_char {
    let size: usize;
    let mut end: *mut core::ffi::c_char;

    size = unsafe { strlen(s as *const core::ffi::c_char) };
    if size == 0 {
        return s;
    }

    end = unsafe { s.add(size - 1) };
    while end >= s && unsafe { isspace(unsafe { *end } as core::ffi::c_int) } != 0 {
        end = unsafe { end.sub(1) };
    }
    unsafe {
        *end.add(1) = b'\0' as core::ffi::c_char;
    }

    unsafe { skip_spaces(s as *const core::ffi::c_char) }
}

// C section annotation macro intent:
// #define __init
// #define __initdata

#[macro_export]
macro_rules! __init {
    () => {};
}

#[macro_export]
macro_rules! __initdata {
    () => {};
}

// External dependency intent:
// #include "../../../../include/linux/bootconfig.h"
