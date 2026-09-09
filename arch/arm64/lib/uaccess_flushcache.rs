// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 ARM Ltd.
 */

// Dependencies supplied by the kernel headers:
unsafe extern "C" {
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize);
    fn dcache_clean_pop(start: usize, end: usize);
    fn raw_copy_from_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;
}

pub unsafe extern "C" fn memcpy_flushcache(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    cnt: usize,
) {
    /*
     * We assume this should not be called with @dst pointing to
     * non-cacheable memory, such that we don't need an explicit
     * barrier to order the cache maintenance against the memcpy.
     */
    unsafe {
        memcpy(dst, src, cnt);
        dcache_clean_pop(dst as usize, (dst as usize).wrapping_add(cnt));
    }
}

// EXPORT_SYMBOL_GPL(memcpy_flushcache);

pub unsafe extern "C" fn __copy_user_flushcache(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: usize,
) -> usize {
    let rc: usize;

    unsafe {
        rc = raw_copy_from_user(to, from, n);

        /* See above */
        dcache_clean_pop(to as usize, (to as usize).wrapping_add(n.wrapping_sub(rc)));
    }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
