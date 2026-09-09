/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 1999 Niibe Yutaka
 * But consider these trivial functions to be public domain.
 */

/* __HAVE_ARCH_STRCPY */
pub unsafe fn strcpy(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    let xdest = dest;

    loop {
        let value = core::ptr::read(src);
        src = src.add(1);
        core::ptr::write(dest, value);
        dest = dest.add(1);
        if value == 0 {
            break;
        }
    }

    xdest
}

/* __HAVE_ARCH_STRCMP */
pub unsafe fn strcmp(mut cs: *const u8, mut ct: *const u8) -> i32 {
    let mut ct_value = core::ptr::read(ct);
    ct = ct.add(1);

    loop {
        let cs_value = core::ptr::read(cs);
        cs = cs.add(1);

        if ct_value == 0 {
            return (cs_value as i32) - (ct_value as i32);
        }
        if cs_value == ct_value {
            ct_value = core::ptr::read(ct);
            ct = ct.add(1);
            continue;
        }

        ct = ct.sub(1);
        ct_value = core::ptr::read(ct);
        return (cs_value as i32) - (ct_value as i32);
    }
}

/* __HAVE_ARCH_STRNCMP */
pub unsafe fn strncmp(mut cs: *const u8, mut ct: *const u8, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    let end = cs.add(n);
    let mut ct_value = core::ptr::read(ct);
    ct = ct.add(1);

    loop {
        let cs_value = core::ptr::read(cs);
        cs = cs.add(1);

        if cs == end || ct_value == 0 {
            return (cs_value as i32) - (ct_value as i32);
        }
        if cs_value == ct_value {
            ct_value = core::ptr::read(ct);
            ct = ct.add(1);
            continue;
        }

        ct = ct.sub(1);
        ct_value = core::ptr::read(ct);
        return (cs_value as i32) - (ct_value as i32);
    }
}

/* __HAVE_ARCH_MEMSET */
unsafe extern "C" {
    pub fn memset(s: *mut core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void;
}

/* __HAVE_ARCH_MEMCPY */
unsafe extern "C" {
    pub fn memcpy(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

/* __HAVE_ARCH_MEMMOVE */
unsafe extern "C" {
    pub fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

/* __HAVE_ARCH_MEMCHR */
unsafe extern "C" {
    pub fn memchr(
        s: *const core::ffi::c_void,
        c: i32,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

/* __HAVE_ARCH_STRLEN */
unsafe extern "C" {
    pub fn strlen(s: *const u8) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
