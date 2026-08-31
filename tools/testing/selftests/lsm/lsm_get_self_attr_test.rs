// SPDX-License-Identifier: GPL-2.0
/*
 * Linux Security Module infrastructure tests
 * Tests for the lsm_get_self_attr system call
 *
 * Copyright (C) 2022 Casey Schaufler <casey@schaufler-ca.com>
 */

// C dependencies: <linux/lsm.h>, <fcntl.h>, <string.h>, <stdio.h>,
// <unistd.h>, <sys/types.h>, "kselftest_harness.h", and "common.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct lsm_ctx {
    pub id: __u64,
    pub flags: __u64,
    pub len: __u64,
    pub ctx_len: __u64,
    pub ctx: [c_char; 0],
}

const NULL: *mut c_void = core::ptr::null_mut();

// Constants from the C headers included by the original file.
const _SC_PAGESIZE: c_int = 30;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;

const LSM_ID_SELINUX: __u64 = 101;
const LSM_ID_SMACK: __u64 = 102;
const LSM_ID_APPARMOR: __u64 = 103;

const LSM_ATTR_CURRENT: __u64 = 100;
const LSM_ATTR_EXEC: __u64 = 101;
const LSM_ATTR_FSCREATE: __u64 = 102;
const LSM_ATTR_KEYCREATE: __u64 = 103;
const LSM_ATTR_PREV: __u64 = 104;
const LSM_ATTR_SOCKCREATE: __u64 = 105;

const LSM_FLAG_SINGLE: __u64 = 0x0001;

const __NR_lsm_list_modules: c_long = 461;

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: c_int) -> c_long;
    fn syscall(num: c_long, ...) -> c_long;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    static mut errno: c_int;

    fn lsm_get_self_attr(
        attr: __u64,
        ctx: *mut lsm_ctx,
        size: *mut __u32,
        flags: __u64,
    ) -> c_int;
    fn attr_lsm_count() -> c_int;
    fn read_proc_attr(attr: *const c_char, ctx: *mut c_char, size: c_long) -> c_int;
}

unsafe fn next_ctx(ctxp: *mut lsm_ctx) -> *mut lsm_ctx {
    let vp: *mut c_void = (ctxp as *mut u8)
        .add(core::mem::size_of::<lsm_ctx>() + (*ctxp).ctx_len as usize)
        as *mut c_void;

    vp as *mut lsm_ctx
}

// TEST(size_null_lsm_get_self_attr)
unsafe fn size_null_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;

    ASSERT_NE(NULL, ctx as *mut c_void);
    errno = 0;
    ASSERT_EQ(
        -1,
        lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, core::ptr::null_mut(), 0),
    );
    ASSERT_EQ(EINVAL, errno);

    free(ctx as *mut c_void);
}

// TEST(ctx_null_lsm_get_self_attr)
unsafe fn ctx_null_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size as __u32;
    let rc: c_int;

    rc = lsm_get_self_attr(
        LSM_ATTR_CURRENT,
        core::ptr::null_mut(),
        &mut size,
        0,
    );

    if attr_lsm_count() != 0 {
        ASSERT_NE(-1, rc);
        ASSERT_NE(1, size);
    } else {
        ASSERT_EQ(-1, rc);
    }
}

// TEST(size_too_small_lsm_get_self_attr)
unsafe fn size_too_small_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
    let mut size: __u32 = 1;

    ASSERT_NE(NULL, ctx as *mut c_void);
    errno = 0;
    ASSERT_EQ(
        -1,
        lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, 0),
    );
    if attr_lsm_count() != 0 {
        ASSERT_EQ(E2BIG, errno);
    } else {
        ASSERT_EQ(EOPNOTSUPP, errno);
    }
    ASSERT_NE(1, size);

    free(ctx as *mut c_void);
}

// TEST(flags_zero_lsm_get_self_attr)
unsafe fn flags_zero_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;
    let mut size: __u32;
    let lsmcount: c_int;
    let mut i: c_int;

    ASSERT_NE(NULL, ctx as *mut c_void);
    errno = 0;
    size = page_size as __u32;
    ASSERT_EQ(
        -1,
        lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, LSM_FLAG_SINGLE),
    );
    ASSERT_EQ(EINVAL, errno);
    ASSERT_EQ(page_size, size as c_long);

    lsmcount = syscall(__NR_lsm_list_modules, syscall_lsms, &mut size, 0 as c_int) as c_int;
    ASSERT_LE(1, lsmcount);
    ASSERT_NE(NULL, syscall_lsms as *mut c_void);

    i = 0;
    while i < lsmcount {
        errno = 0;
        size = page_size as __u32;
        (*ctx).id = *syscall_lsms.add(i as usize);

        if *syscall_lsms.add(i as usize) == LSM_ID_SELINUX
            || *syscall_lsms.add(i as usize) == LSM_ID_SMACK
            || *syscall_lsms.add(i as usize) == LSM_ID_APPARMOR
        {
            ASSERT_EQ(
                1,
                lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, LSM_FLAG_SINGLE),
            );
        } else {
            ASSERT_EQ(
                -1,
                lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, LSM_FLAG_SINGLE),
            );
        }
        i += 1;
    }

    free(ctx as *mut c_void);
}

// TEST(flags_overset_lsm_get_self_attr)
unsafe fn flags_overset_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
    let mut size: __u32;

    ASSERT_NE(NULL, ctx as *mut c_void);

    errno = 0;
    size = page_size as __u32;
    ASSERT_EQ(
        -1,
        lsm_get_self_attr(LSM_ATTR_CURRENT | LSM_ATTR_PREV, ctx, &mut size, 0),
    );
    ASSERT_EQ(EOPNOTSUPP, errno);

    errno = 0;
    size = page_size as __u32;
    ASSERT_EQ(
        -1,
        lsm_get_self_attr(
            LSM_ATTR_CURRENT,
            ctx,
            &mut size,
            LSM_FLAG_SINGLE | (LSM_FLAG_SINGLE << 1),
        ),
    );
    ASSERT_EQ(EINVAL, errno);

    free(ctx as *mut c_void);
}

// TEST(basic_lsm_get_self_attr)
unsafe fn basic_lsm_get_self_attr() {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size as __u32;
    let ctx: *mut lsm_ctx = calloc(page_size as usize, 1) as *mut lsm_ctx;
    let mut tctx: *mut lsm_ctx = core::ptr::null_mut();
    let syscall_lsms: *mut __u64 = calloc(page_size as usize, 1) as *mut __u64;
    let attr: *mut c_char = calloc(page_size as usize, 1) as *mut c_char;
    let mut cnt_current: c_int = 0;
    let mut cnt_exec: c_int = 0;
    let mut cnt_fscreate: c_int = 0;
    let mut cnt_keycreate: c_int = 0;
    let mut cnt_prev: c_int = 0;
    let mut cnt_sockcreate: c_int = 0;
    let lsmcount: c_int;
    let mut count: c_int;
    let mut i: c_int;

    ASSERT_NE(NULL, ctx as *mut c_void);
    ASSERT_NE(NULL, syscall_lsms as *mut c_void);

    lsmcount = syscall(__NR_lsm_list_modules, syscall_lsms, &mut size, 0 as c_int) as c_int;
    ASSERT_LE(1, lsmcount);

    i = 0;
    while i < lsmcount {
        match *syscall_lsms.add(i as usize) {
            LSM_ID_SELINUX => {
                cnt_current += 1;
                cnt_exec += 1;
                cnt_fscreate += 1;
                cnt_keycreate += 1;
                cnt_prev += 1;
                cnt_sockcreate += 1;
            }
            LSM_ID_SMACK => {
                cnt_current += 1;
            }
            LSM_ID_APPARMOR => {
                cnt_current += 1;
                cnt_exec += 1;
                cnt_prev += 1;
            }
            _ => {}
        }
        i += 1;
    }

    if cnt_current != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &mut size, 0);
        ASSERT_EQ(cnt_current, count);
        tctx = ctx;
        ASSERT_EQ(0, read_proc_attr(c"current".as_ptr(), attr, page_size));
        ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
        i = 1;
        while i < count {
            tctx = next_ctx(tctx);
            ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i += 1;
        }
    }
    if cnt_exec != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_EXEC, ctx, &mut size, 0);
        ASSERT_GE(cnt_exec, count);
        if count > 0 {
            tctx = ctx;
            if read_proc_attr(c"exec".as_ptr(), attr, page_size) == 0 {
                ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
            }
        }
        i = 1;
        while i < count {
            tctx = next_ctx(tctx);
            ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i += 1;
        }
    }
    if cnt_fscreate != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_FSCREATE, ctx, &mut size, 0);
        ASSERT_GE(cnt_fscreate, count);
        if count > 0 {
            tctx = ctx;
            if read_proc_attr(c"fscreate".as_ptr(), attr, page_size) == 0 {
                ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
            }
        }
        i = 1;
        while i < count {
            tctx = next_ctx(tctx);
            ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i += 1;
        }
    }
    if cnt_keycreate != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_KEYCREATE, ctx, &mut size, 0);
        ASSERT_GE(cnt_keycreate, count);
        if count > 0 {
            tctx = ctx;
            if read_proc_attr(c"keycreate".as_ptr(), attr, page_size) == 0 {
                ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
            }
        }
        i = 1;
        while i < count {
            tctx = next_ctx(tctx);
            ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i += 1;
        }
    }
    if cnt_prev != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_PREV, ctx, &mut size, 0);
        ASSERT_GE(cnt_prev, count);
        if count > 0 {
            tctx = ctx;
            ASSERT_EQ(0, read_proc_attr(c"prev".as_ptr(), attr, page_size));
            ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i = 1;
            while i < count {
                tctx = next_ctx(tctx);
                ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
                i += 1;
            }
        }
    }
    if cnt_sockcreate != 0 {
        size = page_size as __u32;
        count = lsm_get_self_attr(LSM_ATTR_SOCKCREATE, ctx, &mut size, 0);
        ASSERT_GE(cnt_sockcreate, count);
        if count > 0 {
            tctx = ctx;
            if read_proc_attr(c"sockcreate".as_ptr(), attr, page_size) == 0 {
                ASSERT_EQ(0, strcmp((*tctx).ctx.as_ptr(), attr));
            }
        }
        i = 1;
        while i < count {
            tctx = next_ctx(tctx);
            ASSERT_NE(0, strcmp((*tctx).ctx.as_ptr(), attr));
            i += 1;
        }
    }

    free(ctx as *mut c_void);
    free(attr as *mut c_void);
    free(syscall_lsms as *mut c_void);
}

// TEST_HARNESS_MAIN
