// SPDX-License-Identifier: GPL-2.0
/*
 * Linux Security Module infrastructure tests
 * Tests for the lsm_set_self_attr system call
 *
 * Copyright © 2022 Casey Schaufler <casey@schaufler-ca.com>
 */

// C source defined _GNU_SOURCE and included:
// <linux/lsm.h>, <string.h>, <stdio.h>, <unistd.h>, <sys/types.h>,
// "kselftest_harness.h", and "common.h".

use core::ffi::{c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

#[repr(C)]
pub struct lsm_ctx {
    // Layout is supplied by <linux/lsm.h> in the original C source.
    _data: [u8; 0],
}

unsafe extern "C" {
    static LSM_ATTR_CURRENT: c_uint;
    static LSM_ATTR_PREV: c_uint;
    static _SC_PAGESIZE: c_int;

    fn sysconf(name: c_int) -> c_long;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn lsm_set_self_attr(attr: c_uint, ctx: *mut lsm_ctx, size: __u32, flags: c_uint) -> c_long;
    fn lsm_get_self_attr(
        attr: c_uint,
        ctx: *mut lsm_ctx,
        size: *mut __u32,
        flags: c_uint,
    ) -> c_long;
    fn attr_lsm_count() -> c_int;
}

#[test]
fn ctx_null_lsm_set_self_attr() {
    unsafe {
        assert_eq!(
            -1,
            lsm_set_self_attr(
                LSM_ATTR_CURRENT,
                ptr::null_mut(),
                size_of::<lsm_ctx>() as __u32,
                0,
            ),
        );
    }
}

#[test]
fn size_too_small_lsm_set_self_attr() {
    unsafe {
        let page_size: c_long = sysconf(_SC_PAGESIZE);
        let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
        let mut size: __u32 = page_size as __u32;

        assert_ne!(ptr::null_mut(), ctx);
        if attr_lsm_count() != 0 {
            assert!(1 <= lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, 0));
        }
        assert_eq!(-1, lsm_set_self_attr(LSM_ATTR_CURRENT, ctx, 1, 0));

        free(ctx as *mut c_void);
    }
}

#[test]
fn flags_zero_lsm_set_self_attr() {
    unsafe {
        let page_size: c_long = sysconf(_SC_PAGESIZE);
        let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
        let mut size: __u32 = page_size as __u32;

        assert_ne!(ptr::null_mut(), ctx);
        if attr_lsm_count() != 0 {
            assert!(1 <= lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, 0));
        }
        assert_eq!(-1, lsm_set_self_attr(LSM_ATTR_CURRENT, ctx, size, 1));

        free(ctx as *mut c_void);
    }
}

#[test]
fn flags_overset_lsm_set_self_attr() {
    unsafe {
        let page_size: c_long = sysconf(_SC_PAGESIZE);
        let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
        let mut size: __u32 = page_size as __u32;

        assert_ne!(ptr::null_mut(), ctx);
        if attr_lsm_count() != 0 {
            assert!(1 <= lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, 0));
        }
        assert_eq!(
            -1,
            lsm_set_self_attr(LSM_ATTR_CURRENT | LSM_ATTR_PREV, ctx, size, 0),
        );

        free(ctx as *mut c_void);
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
