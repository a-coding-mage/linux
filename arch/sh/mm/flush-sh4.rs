// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the corresponding architecture and cache headers
// are intentionally referenced here rather than reimplemented.

extern "C" {
    static mut __flush_wback_region: unsafe extern "C" fn(*mut core::ffi::c_void, i32);
    static mut __flush_invalidate_region: unsafe extern "C" fn(*mut core::ffi::c_void, i32);
    static mut __flush_purge_region: unsafe extern "C" fn(*mut core::ffi::c_void, i32);

    fn register_align(start: *mut core::ffi::c_void) -> reg_size_t;
    fn __ocbwb(address: reg_size_t);
    fn __ocbp(address: reg_size_t);
    fn __ocbi(address: reg_size_t);
}

// Supplied by the architecture headers.
type reg_size_t = usize;
const L1_CACHE_BYTES: reg_size_t = 32;

/*
 * Write back the dirty D-caches, but not invalidate them.
 *
 * START: Virtual Address (U0, P1, or P3)
 * SIZE: Size of the region.
 */
unsafe extern "C" fn sh4__flush_wback_region(start: *mut core::ffi::c_void, size: i32) {
    let aligned_start: reg_size_t;
    let mut v: reg_size_t;
    let mut cnt: reg_size_t;
    let end: reg_size_t;

    aligned_start = register_align(start);
    v = aligned_start & !(L1_CACHE_BYTES - 1);
    end = (aligned_start + size as reg_size_t + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    cnt = (end - v) / L1_CACHE_BYTES;

    while cnt >= 8 {
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        __ocbwb(v); v += L1_CACHE_BYTES;
        cnt -= 8;
    }

    while cnt != 0 {
        __ocbwb(v); v += L1_CACHE_BYTES;
        cnt -= 1;
    }
}

/*
 * Write back the dirty D-caches and invalidate them.
 *
 * START: Virtual Address (U0, P1, or P3)
 * SIZE: Size of the region.
 */
unsafe extern "C" fn sh4__flush_purge_region(start: *mut core::ffi::c_void, size: i32) {
    let aligned_start: reg_size_t;
    let mut v: reg_size_t;
    let mut cnt: reg_size_t;
    let end: reg_size_t;

    aligned_start = register_align(start);
    v = aligned_start & !(L1_CACHE_BYTES - 1);
    end = (aligned_start + size as reg_size_t + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    cnt = (end - v) / L1_CACHE_BYTES;

    while cnt >= 8 {
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        __ocbp(v); v += L1_CACHE_BYTES;
        cnt -= 8;
    }
    while cnt != 0 {
        __ocbp(v); v += L1_CACHE_BYTES;
        cnt -= 1;
    }
}

/*
 * No write back please
 */
unsafe extern "C" fn sh4__flush_invalidate_region(start: *mut core::ffi::c_void, size: i32) {
    let aligned_start: reg_size_t;
    let mut v: reg_size_t;
    let mut cnt: reg_size_t;
    let end: reg_size_t;

    aligned_start = register_align(start);
    v = aligned_start & !(L1_CACHE_BYTES - 1);
    end = (aligned_start + size as reg_size_t + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    cnt = (end - v) / L1_CACHE_BYTES;

    while cnt >= 8 {
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        __ocbi(v); v += L1_CACHE_BYTES;
        cnt -= 8;
    }

    while cnt != 0 {
        __ocbi(v); v += L1_CACHE_BYTES;
        cnt -= 1;
    }
}

pub unsafe extern "C" fn sh4__flush_region_init() {
    __flush_wback_region = sh4__flush_wback_region;
    __flush_invalidate_region = sh4__flush_invalidate_region;
    __flush_purge_region = sh4__flush_purge_region;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
