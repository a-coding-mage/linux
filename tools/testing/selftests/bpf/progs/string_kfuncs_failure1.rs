// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Red Hat, Inc.*/
// C includes translated as external dependencies:
// "vmlinux.h", <bpf/bpf_helpers.h>, <linux/limits.h>, "bpf_misc.h", "errno.h"

use core::ffi::{c_char, c_int, c_void};

const EFAULT: c_int = 14;
const ERANGE: c_int = 34;

#[no_mangle]
pub static mut user_ptr: *mut c_char = 1 as *mut c_char;

#[no_mangle]
pub static mut invalid_kern_ptr: *mut c_char = (-1isize) as *mut c_char;

/*
 * When passing userspace pointers, the error code differs based on arch:
 *   -ERANGE on arches with non-overlapping address spaces
 *   -EFAULT on other arches
 */
#[cfg(any(
    target_arch = "arm",
    target_arch = "loongarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "x86",
    target_arch = "x86_64"
))]
const USER_PTR_ERR: c_int = -ERANGE;

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "loongarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
const USER_PTR_ERR: c_int = -EFAULT;

extern "C" {
    fn bpf_strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn bpf_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn bpf_strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn bpf_strchr(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strchrnul(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strnchr(s: *const c_char, n: usize, c: c_int) -> c_int;
    fn bpf_strrchr(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strlen(s: *const c_char) -> c_int;
    fn bpf_strnlen(s: *const c_char, n: usize) -> c_int;
    fn bpf_strspn(s: *const c_char, accept: *const c_char) -> c_int;
    fn bpf_strcspn(s: *const c_char, reject: *const c_char) -> c_int;
    fn bpf_strstr(haystack: *const c_char, needle: *const c_char) -> c_int;
    fn bpf_strcasestr(haystack: *const c_char, needle: *const c_char) -> c_int;
    fn bpf_strnstr(haystack: *const c_char, needle: *const c_char, len: usize) -> c_int;
    fn bpf_strncasestr(haystack: *const c_char, needle: *const c_char, len: usize) -> c_int;
}

/*
 * On s390, __get_kernel_nofault (used in string kfuncs) returns 0 for NULL and
 * user_ptr (instead of causing an exception) so the below two groups of tests
 * are not applicable.
 */

/* Passing NULL to string kfuncs (treated as a userspace ptr) */
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(core::ptr::null(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(c"hello".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(core::ptr::null(), c"HELLO".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(c"HELLO".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(core::ptr::null(), c"HELLO".as_ptr(), 5) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(c"HELLO".as_ptr(), core::ptr::null(), 5) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(core::ptr::null(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchrnul(core::ptr::null(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(core::ptr::null(), 1, 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(core::ptr::null(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strlen_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strlen(core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnlen_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnlen(core::ptr::null(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(core::ptr::null(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(c"hello".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(core::ptr::null(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(c"hello".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(core::ptr::null(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(c"hello".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(core::ptr::null(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(c"hello".as_ptr(), core::ptr::null()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(core::ptr::null(), c"hello".as_ptr(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(c"hello".as_ptr(), core::ptr::null(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_null1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(core::ptr::null(), c"hello".as_ptr(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_null2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(c"hello".as_ptr(), core::ptr::null(), 1) } }

/* Passing userspace ptr to string kfuncs */
#[cfg(not(target_arch = "s390x"))]
macro_rules! user_ptr_const { () => { unsafe { user_ptr as *const c_char } }; }

#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(user_ptr_const!(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(c"hello".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(user_ptr_const!(), c"HELLO".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(c"HELLO".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(user_ptr_const!(), c"HELLO".as_ptr(), 5) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(c"HELLO".as_ptr(), user_ptr_const!(), 5) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(user_ptr_const!(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchrnul(user_ptr_const!(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(user_ptr_const!(), 1, 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(user_ptr_const!(), 'a' as c_int) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strlen_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strlen(user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnlen_user_ptr(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnlen(user_ptr_const!(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(user_ptr_const!(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(c"hello".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(user_ptr_const!(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(c"hello".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(user_ptr_const!(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(c"hello".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(user_ptr_const!(), c"hello".as_ptr()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(c"hello".as_ptr(), user_ptr_const!()) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(user_ptr_const!(), c"hello".as_ptr(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(c"hello".as_ptr(), user_ptr_const!(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_user_ptr1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(user_ptr_const!(), c"hello".as_ptr(), 1) } }
#[cfg(not(target_arch = "s390x"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_user_ptr2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(c"hello".as_ptr(), user_ptr_const!(), 1) } }

/* Passing invalid kernel ptr to string kfuncs should always return -EFAULT */
macro_rules! invalid_kern_ptr_const { () => { unsafe { invalid_kern_ptr as *const c_char } }; }

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(invalid_kern_ptr_const!(), c"hello".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(c"hello".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(invalid_kern_ptr_const!(), c"HELLO".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(c"HELLO".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(invalid_kern_ptr_const!(), c"HELLO".as_ptr(), 5) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(c"HELLO".as_ptr(), invalid_kern_ptr_const!(), 5) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(invalid_kern_ptr_const!(), 'a' as c_int) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchrnul(invalid_kern_ptr_const!(), 'a' as c_int) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(invalid_kern_ptr_const!(), 1, 'a' as c_int) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(invalid_kern_ptr_const!(), 'a' as c_int) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strlen_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strlen(invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnlen_pagefault(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnlen(invalid_kern_ptr_const!(), 1) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(invalid_kern_ptr_const!(), c"hello".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(c"hello".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(invalid_kern_ptr_const!(), c"hello".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(c"hello".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(invalid_kern_ptr_const!(), c"hello".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(c"hello".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(invalid_kern_ptr_const!(), c"hello".as_ptr()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(c"hello".as_ptr(), invalid_kern_ptr_const!()) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(invalid_kern_ptr_const!(), c"hello".as_ptr(), 1) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(c"hello".as_ptr(), invalid_kern_ptr_const!(), 1) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_pagefault1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(invalid_kern_ptr_const!(), c"hello".as_ptr(), 1) } }
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_pagefault2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(c"hello".as_ptr(), invalid_kern_ptr_const!(), 1) } }

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
