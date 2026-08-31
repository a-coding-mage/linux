/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Minimal errno definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: #include "nolibc.h" */

/* C dependency: #include <linux/errno.h> */

/*
 * If NOLIBC_IGNORE_ERRNO is not defined, C exposes these as weak globals and
 * SET_ERRNO(v) assigns to errno. If NOLIBC_IGNORE_ERRNO is defined, SET_ERRNO
 * is empty and the program invocation names are string constants.
 */

#[macro_export]
macro_rules! SET_ERRNO {
    ($v:expr) => {{
        unsafe {
            errno = $v;
        }
    }};
}

/* C attribute preserved in intent: __attribute__((weak)) */
#[no_mangle]
pub static mut errno: i32 = 0;

static PROGRAM_INVOCATION_NAME_BYTES: &[u8] = b"\0";
static PROGRAM_INVOCATION_SHORT_NAME_BYTES: &[u8] = b"\0";

/* C attribute preserved in intent: __attribute__((weak)) */
#[no_mangle]
pub static mut program_invocation_name: *mut i8 =
    PROGRAM_INVOCATION_NAME_BYTES.as_ptr() as *mut i8;

/* C attribute preserved in intent: __attribute__((weak)) */
#[no_mangle]
pub static mut program_invocation_short_name: *mut i8 =
    PROGRAM_INVOCATION_SHORT_NAME_BYTES.as_ptr() as *mut i8;

/*
 * errno codes all ensure that they will not conflict with a valid pointer
 * because they all correspond to the highest addressable memory page.
 */
pub const MAX_ERRNO: i32 = 4095;
