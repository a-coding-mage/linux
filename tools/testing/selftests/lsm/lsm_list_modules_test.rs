// SPDX-License-Identifier: GPL-2.0
/*
 * Linux Security Module infrastructure tests
 * Tests for the lsm_list_modules system call
 *
 * Copyright (C) 2022 Casey Schaufler <casey@schaufler-ca.com>
 */

// C dependencies: linux/lsm.h, string.h, stdio.h, unistd.h, sys/types.h,
// kselftest_harness.h, common.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type __u32 = u32;
type __u64 = u64;

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: c_int) -> c_long;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;

    fn lsm_list_modules(ids: *mut __u64, size: *mut __u32, flags: __u32) -> c_int;
    fn read_sysfs_lsms(buf: *mut c_char, size: c_long) -> c_int;
}

// External constants supplied by the headers in the original C source.
unsafe extern "C" {
    static _SC_PAGESIZE: c_int;
    static EFAULT: c_int;
    static E2BIG: c_int;
    static EINVAL: c_int;

    static LSM_ID_CAPABILITY: __u64;
    static LSM_ID_SELINUX: __u64;
    static LSM_ID_SMACK: __u64;
    static LSM_ID_TOMOYO: __u64;
    static LSM_ID_APPARMOR: __u64;
    static LSM_ID_YAMA: __u64;
    static LSM_ID_LOADPIN: __u64;
    static LSM_ID_SAFESETID: __u64;
    static LSM_ID_LOCKDOWN: __u64;
    static LSM_ID_BPF: __u64;
    static LSM_ID_LANDLOCK: __u64;
    static LSM_ID_IMA: __u64;
    static LSM_ID_EVM: __u64;
    static LSM_ID_IPE: __u64;
}

unsafe extern "C" {
    // errno is a C macro on many platforms; this preserves the source-level
    // dependency without providing a local implementation.
    static mut errno: c_int;
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        assert!($left <= $right)
    };
}

unsafe fn size_null_lsm_list_modules() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;

    ASSERT_NE!(core::ptr::null_mut::<__u64>(), syscall_lsms);
    errno = 0;
    ASSERT_EQ!(-1, lsm_list_modules(syscall_lsms, core::ptr::null_mut(), 0));
    ASSERT_EQ!(EFAULT, errno);

    free(syscall_lsms as *mut c_void);
}

unsafe fn ids_null_lsm_list_modules() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size as __u32;

    errno = 0;
    ASSERT_EQ!(-1, lsm_list_modules(core::ptr::null_mut(), &mut size, 0));
    ASSERT_EQ!(EFAULT, errno);
    ASSERT_NE!(1, size);
}

unsafe fn size_too_small_lsm_list_modules() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;
    let mut size: __u32 = 1;

    ASSERT_NE!(core::ptr::null_mut::<__u64>(), syscall_lsms);
    errno = 0;
    ASSERT_EQ!(-1, lsm_list_modules(syscall_lsms, &mut size, 0));
    ASSERT_EQ!(E2BIG, errno);
    ASSERT_NE!(1, size);

    free(syscall_lsms as *mut c_void);
}

unsafe fn flags_set_lsm_list_modules() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;
    let mut size: __u32 = page_size as __u32;

    ASSERT_NE!(core::ptr::null_mut::<__u64>(), syscall_lsms);
    errno = 0;
    ASSERT_EQ!(-1, lsm_list_modules(syscall_lsms, &mut size, 7));
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(page_size, size as c_long);

    free(syscall_lsms as *mut c_void);
}

unsafe fn correct_lsm_list_modules() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size as __u32;
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;
    let sysfs_lsms: *mut c_char = calloc(page_size as usize, 1) as *mut c_char;
    let mut name: *const c_char;
    let mut cp: *mut c_char;
    let count: c_int;
    let mut i: c_int;

    ASSERT_NE!(core::ptr::null_mut::<c_char>(), sysfs_lsms);
    ASSERT_NE!(core::ptr::null_mut::<__u64>(), syscall_lsms);
    ASSERT_EQ!(0, read_sysfs_lsms(sysfs_lsms, page_size));

    count = lsm_list_modules(syscall_lsms, &mut size, 0);
    ASSERT_LE!(1, count);
    cp = sysfs_lsms;
    i = 0;
    while i < count {
        let id = *syscall_lsms.add(i as usize);
        if id == LSM_ID_CAPABILITY {
            name = b"capability\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_SELINUX {
            name = b"selinux\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_SMACK {
            name = b"smack\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_TOMOYO {
            name = b"tomoyo\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_APPARMOR {
            name = b"apparmor\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_YAMA {
            name = b"yama\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_LOADPIN {
            name = b"loadpin\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_SAFESETID {
            name = b"safesetid\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_LOCKDOWN {
            name = b"lockdown\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_BPF {
            name = b"bpf\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_LANDLOCK {
            name = b"landlock\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_IMA {
            name = b"ima\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_EVM {
            name = b"evm\0".as_ptr() as *const c_char;
        } else if id == LSM_ID_IPE {
            name = b"ipe\0".as_ptr() as *const c_char;
        } else {
            name = b"INVALID\0".as_ptr() as *const c_char;
        }
        ASSERT_EQ!(0, strncmp(cp, name, strlen(name)));
        cp = cp.add(strlen(name) + 1);
        i += 1;
    }

    free(sysfs_lsms as *mut c_void);
    free(syscall_lsms as *mut c_void);
}

fn main() {
    unsafe {
        size_null_lsm_list_modules();
        ids_null_lsm_list_modules();
        size_too_small_lsm_list_modules();
        flags_set_lsm_list_modules();
        correct_lsm_list_modules();
    }
}
