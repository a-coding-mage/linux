// SPDX-License-Identifier: GPL-2.0
/*
 * Helper function for splitting a string into an argv-like array.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;

    /* From linux/ctype.h and linux/string.h. */
    fn isspace(c: c_int) -> c_int;
    fn skip_spaces(str: *const c_char) -> *const c_char;
}

unsafe fn skip_arg(mut cp: *const c_char) -> *const c_char {
    while unsafe { *cp } != 0 && unsafe { isspace(*cp as c_int) } == 0 {
        cp = unsafe { cp.add(1) };
    }

    cp
}

unsafe fn count_argc(mut str: *const c_char) -> c_int {
    let mut count: c_int = 0;

    while unsafe { *str } != 0 {
        str = unsafe { skip_spaces(str) };
        if unsafe { *str } != 0 {
            count += 1;
            str = unsafe { skip_arg(str) };
        }
    }

    count
}

/**
 * argv_free - free an argv
 * @argv - the argument vector to be freed
 *
 * Frees an argv and the strings it points to.
 */
#[no_mangle]
pub unsafe extern "C" fn argv_free(argv: *mut *mut c_char) {
    let mut p: *mut *mut c_char;

    p = argv;
    while unsafe { *p }.is_null() == false {
        unsafe { free(*p as *mut c_void) };
        unsafe { *p = ptr::null_mut() };
        p = unsafe { p.add(1) };
    }

    unsafe { free(argv as *mut c_void) };
}

/**
 * argv_split - split a string at whitespace, returning an argv
 * @str: the string to be split
 * @argcp: returned argument count
 *
 * Returns an array of pointers to strings which are split out from
 * @str.  This is performed by strictly splitting on white-space; no
 * quote processing is performed.  Multiple whitespace characters are
 * considered to be a single argument separator.  The returned array
 * is always NULL-terminated.  Returns NULL on memory allocation
 * failure.
 */
#[no_mangle]
pub unsafe extern "C" fn argv_split(mut str: *const c_char, argcp: *mut c_int) -> *mut *mut c_char {
    let argc: c_int = unsafe { count_argc(str) };
    let argv: *mut *mut c_char =
        unsafe { calloc((argc + 1) as usize, size_of::<*mut c_char>()) as *mut *mut c_char };
    let mut argvp: *mut *mut c_char;

    if argv.is_null() {
        return argv;
    }

    if !argcp.is_null() {
        unsafe { *argcp = argc };
    }

    argvp = argv;

    while unsafe { *str } != 0 {
        str = unsafe { skip_spaces(str) };

        if unsafe { *str } != 0 {
            let p: *const c_char = str;
            let t: *mut c_char;

            str = unsafe { skip_arg(str) };

            t = unsafe { strndup(p, str.offset_from(p) as usize) };
            if t.is_null() {
                unsafe { argv_free(argv) };
                return ptr::null_mut();
            }
            unsafe { *argvp = t };
            argvp = unsafe { argvp.add(1) };
        }
    }
    unsafe { *argvp = ptr::null_mut() };

    argv
}
