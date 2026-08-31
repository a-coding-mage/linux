/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/string2.h.
// C dependencies: <linux/string.h>, <linux/types.h>, <sys/types.h>,
// <stddef.h>, and <string.h>.

use std::os::raw::{c_char, c_int, c_uint};

pub type s64 = i64;
pub type pid_t = c_int;

unsafe extern "C" {
    pub static graph_dotted_line: *const c_char;
    pub static dots: *const c_char;

    pub fn perf_atoll(str: *const c_char) -> s64;
    pub fn strglobmatch(str: *const c_char, pat: *const c_char) -> bool;
    pub fn strglobmatch_nocase(str: *const c_char, pat: *const c_char) -> bool;
    pub fn strlazymatch(str: *const c_char, pat: *const c_char) -> bool;
    pub fn strtailcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    pub fn asprintf_expr_inout_ints(
        var: *const c_char,
        in_: bool,
        nints: usize,
        ints: *mut c_int,
    ) -> *mut c_char;

    pub fn asprintf__tp_filter_pids(npids: usize, pids: *mut pid_t) -> *mut c_char;

    pub fn strpbrk_esc(str: *mut c_char, stopset: *const c_char) -> *mut c_char;
    pub fn strdup_esc(str: *const c_char) -> *mut c_char;
    pub fn strpbrk_esq(str: *mut c_char, stopset: *const c_char) -> *mut c_char;
    pub fn strdup_esq(str: *const c_char) -> *mut c_char;

    pub fn hex(c: c_char) -> c_uint;
    pub fn strreplace_chars(
        needle: c_char,
        haystack: *const c_char,
        replace: *const c_char,
    ) -> *mut c_char;

    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
}

#[inline]
pub unsafe fn strisglob(str: *const c_char) -> bool {
    unsafe { !strpbrk(str, c"*?[".as_ptr()).is_null() }
}

#[inline]
pub unsafe fn asprintf_expr_in_ints(
    var: *const c_char,
    nints: usize,
    ints: *mut c_int,
) -> *mut c_char {
    unsafe { asprintf_expr_inout_ints(var, true, nints, ints) }
}

#[inline]
pub unsafe fn asprintf_expr_not_in_ints(
    var: *const c_char,
    nints: usize,
    ints: *mut c_int,
) -> *mut c_char {
    unsafe { asprintf_expr_inout_ints(var, false, nints, ints) }
}
