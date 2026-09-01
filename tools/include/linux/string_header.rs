/* SPDX-License-Identifier: GPL-2.0 */

// C header guard _TOOLS_LINUX_STRING_H_ omitted.
// C dependencies: <linux/types.h> for size_t and <string.h>.

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    pub fn memdup(src: *const c_void, len: usize) -> *mut c_void;

    pub fn argv_split(str: *const c_char, argcp: *mut c_int) -> *mut *mut c_char;
    pub fn argv_free(argv: *mut *mut c_char);

    pub fn strtobool(s: *const c_char, res: *mut bool) -> c_int;

    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    /*
     * glibc based builds needs the extern while uClibc doesn't.
     * However uClibc headers also define __GLIBC__ hence the hack below.
     *
     * Original C condition:
     * #if defined(__GLIBC__) && !defined(__UCLIBC__)
     * with GCC diagnostic pragmas suppressing redundant declarations.
     */
    pub fn strlcpy(dest: *mut c_char, src: *const c_char, size: usize) -> usize;

    pub fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;

    pub fn strreplace(s: *mut c_char, old: c_char, new: c_char) -> *mut c_char;

    pub fn strlen(s: *const c_char) -> usize;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    #[must_use]
    pub fn skip_spaces(s: *const c_char) -> *mut c_char;

    pub fn strim(s: *mut c_char) -> *mut c_char;

    pub fn remove_spaces(s: *mut c_char);

    pub fn memchr_inv(start: *const c_void, c: c_int, bytes: usize) -> *mut c_void;
    pub fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> u64;
}

// #define strscpy strcpy
pub use strcpy as strscpy;

/**
 * strstarts - does @str start with @prefix?
 * @str: string to examine
 * @prefix: prefix to look for.
 */
pub unsafe fn strstarts(str: *const c_char, prefix: *const c_char) -> bool {
    unsafe { strncmp(str, prefix, strlen(prefix)) == 0 }
}

/*
 * Checks if a string ends with another.
 */
pub unsafe fn str_ends_with(str: *const c_char, substr: *const c_char) -> bool {
    let len: usize = unsafe { strlen(str) };
    let sublen: usize = unsafe { strlen(substr) };

    if sublen > len {
        return false;
    }

    unsafe { strcmp(str.add(len - sublen), substr) == 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
