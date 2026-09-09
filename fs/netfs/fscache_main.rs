// SPDX-License-Identifier: GPL-2.0-or-later
/* General filesystem local caching manager
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #define FSCACHE_DEBUG_LEVEL CACHE
// C dependencies: <linux/module.h>, <linux/init.h>, "internal.h",
// and <trace/events/fscache.h>.
// CREATE_TRACE_POINTS

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fscache_cookie {
    _private: [u8; 0],
}

pub type __le32 = u32;

extern "C" {
    pub static mut fscache_wq: *mut workqueue_struct;
    pub static mut fscache_cookie_jar: *mut kmem_cache;
    pub static mut fscache_cookie_lru_timer: core::ffi::c_ulong;

    pub fn __hash_32(value: u32) -> u32;
    pub fn fscache_proc_init() -> c_int;
    pub fn fscache_proc_cleanup();
    pub fn alloc_workqueue(
        name: *const c_char,
        flags: c_uint,
        max_active: c_uint,
    ) -> *mut workqueue_struct;
    pub fn kmem_cache_create(
        name: *const c_char,
        size: usize,
        align: usize,
        flags: c_uint,
        ctor: *mut c_void,
    ) -> *mut kmem_cache;
    pub fn kmem_cache_destroy(cache: *mut kmem_cache);
    pub fn destroy_workqueue(wq: *mut workqueue_struct);
    pub fn timer_shutdown_sync(timer: *mut core::ffi::c_ulong);
}

// EXPORT_TRACEPOINT_SYMBOL(fscache_access_cache);
// EXPORT_TRACEPOINT_SYMBOL(fscache_access_volume);
// EXPORT_TRACEPOINT_SYMBOL(fscache_access);
// EXPORT_SYMBOL(fscache_wq);

/*
 * Mixing scores (in bits) for (7,20):
 * Input delta: 1-bit      2-bit
 * 1 round:     330.3     9201.6
 * 2 rounds:   1246.4    25475.4
 * 3 rounds:   1907.1    31295.1
 * 4 rounds:   2042.3    31718.6
 * Perfect:    2048      31744
 *            (32*64)   (32*31/2 * 64)
 */
#[inline]
unsafe fn hash_mix(x: &mut u32, y: &mut u32, a: u32) {
    *x ^= a;
    *y ^= *x;
    *x = x.rotate_left(7);
    *x = x.wrapping_add(*y);
    *y = y.rotate_left(20);
    *y = y.wrapping_mul(9);
}

#[inline]
unsafe fn fold_hash(x: usize, y: usize) -> c_uint {
    /* Use arch-optimized multiply if one exists */
    __hash_32((y as u32) ^ __hash_32(x as u32))
}

/*
 * Generate a hash.  This is derived from full_name_hash(), but we want to be
 * sure it is arch independent and that it doesn't change as bits of the
 * computed hash value might appear on disk.  The caller must guarantee that
 * the source data is a multiple of four bytes in size.
 */
pub unsafe fn fscache_hash(salt: c_uint, data: *const c_void, len: usize) -> c_uint {
    let mut p = data as *const __le32;
    let mut x: u32 = 0;
    let mut y: u32 = salt;
    let mut n = len / core::mem::size_of::<__le32>();

    while n != 0 {
        let a = u32::from_le(core::ptr::read(p));
        p = p.add(1);
        hash_mix(&mut x, &mut y, a);
        n -= 1;
    }
    fold_hash(x as usize, y as usize)
}

/*
 * initialise the fs caching module
 */
pub unsafe fn fscache_init() -> c_int {
    let mut ret: c_int = -12; // -ENOMEM

    fscache_wq = alloc_workqueue(b"fscache\0".as_ptr() as *const c_char, 0, 0);
    if fscache_wq.is_null() {
        return goto_error_wq(ret);
    }

    ret = fscache_proc_init();
    if ret < 0 {
        return goto_error_proc(ret);
    }

    fscache_cookie_jar = kmem_cache_create(
        b"fscache_cookie_jar\0".as_ptr() as *const c_char,
        core::mem::size_of::<fscache_cookie>(),
        0,
        0,
        core::ptr::null_mut(),
    );
    if fscache_cookie_jar.is_null() {
        // pr_notice("Failed to allocate a cookie jar\n");
        ret = -12; // -ENOMEM
        return goto_error_cookie_jar(ret);
    }

    // pr_notice("FS-Cache loaded\n");
    0
}

#[inline(never)]
unsafe fn goto_error_cookie_jar(ret: c_int) -> c_int {
    fscache_proc_cleanup();
    goto_error_proc(ret)
}

#[inline(never)]
unsafe fn goto_error_proc(ret: c_int) -> c_int {
    destroy_workqueue(fscache_wq);
    goto_error_wq(ret)
}

#[inline(never)]
unsafe fn goto_error_wq(ret: c_int) -> c_int {
    ret
}

/*
 * clean up on module removal
 */
pub unsafe fn fscache_exit() {
    // _enter("");

    kmem_cache_destroy(fscache_cookie_jar);
    fscache_proc_cleanup();
    timer_shutdown_sync(&mut fscache_cookie_lru_timer);
    destroy_workqueue(fscache_wq);
    // pr_notice("FS-Cache unloaded\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
