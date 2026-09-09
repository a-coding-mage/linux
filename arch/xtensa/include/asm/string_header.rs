/*
 * include/asm-xtensa/string.h
 *
 * These trivial string functions are considered part of the public domain.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/* We should optimize these. See arch/xtensa/lib/strncpy_user.S */

/* __HAVE_ARCH_STRCPY */
#[inline]
pub unsafe fn strcpy(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    let original_dest = dest;
    loop {
        let byte = unsafe { core::ptr::read(src) };
        unsafe { core::ptr::write(dest, byte) };
        src = unsafe { src.add(1) };
        dest = unsafe { dest.add(1) };
        if byte == 0 {
            break;
        }
    }
    original_dest
}

/* __HAVE_ARCH_STRCMP */
#[inline]
pub unsafe fn strcmp(mut cs: *const u8, mut ct: *const u8) -> i32 {
    loop {
        let left = unsafe { core::ptr::read(cs) };
        cs = unsafe { cs.add(1) };
        let right = unsafe { core::ptr::read(ct) };
        ct = unsafe { ct.add(1) };
        if left == 0 || left != right {
            return (left as i32).wrapping_sub(right as i32);
        }
    }
}

/* __HAVE_ARCH_STRNCMP */
#[inline]
pub unsafe fn strncmp(mut cs: *const u8, mut ct: *const u8, n: usize) -> i32 {
    let end = unsafe { cs.add(n) };
    loop {
        if cs == end {
            return 0;
        }
        let right = unsafe { core::ptr::read(ct) };
        ct = unsafe { ct.add(1) };
        let left = unsafe { core::ptr::read(cs) };
        cs = unsafe { cs.add(1) };
        if left == 0 || right == 0 || left != right {
            return (left as i32).wrapping_sub(right as i32);
        }
    }
}

/* __HAVE_ARCH_MEMSET */
/* __HAVE_ARCH_MEMCPY */
/* __HAVE_ARCH_MEMMOVE */
unsafe extern "C" {
    pub fn memset(s: *mut core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void;
    pub fn __memset(s: *mut core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void;
    pub fn memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn __memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn __memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

/*
 * For files that are not instrumented (e.g. mm/slub.c) we
 * should use not instrumented version of mem* functions.
 *
 * When CONFIG_KASAN is defined and __SANITIZE_ADDRESS__ is not defined,
 * the C macros memcpy, memmove, and memset refer to __memcpy, __memmove,
 * and __memset respectively. __NO_FORTIFY is also defined in that case.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
