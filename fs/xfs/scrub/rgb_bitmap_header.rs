// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Bitmaps, but for type-checked for xfs_rgblock_t */

#[repr(C)]
pub struct xrgb_bitmap {
    pub rgbitmap: xbitmap32,
}

extern "C" {
    fn xbitmap32_init(bitmap: *mut xbitmap32);
    fn xbitmap32_destroy(bitmap: *mut xbitmap32);
    fn xbitmap32_set(bitmap: *mut xbitmap32, start: xfs_rgblock_t, len: xfs_extlen_t) -> ::std::os::raw::c_int;
    fn xbitmap32_walk(
        bitmap: *mut xbitmap32,
        fn_: xbitmap32_walk_fn,
        priv_: *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}

pub unsafe fn xrgb_bitmap_init(bitmap: *mut xrgb_bitmap) {
    xbitmap32_init(&mut (*bitmap).rgbitmap);
}

pub unsafe fn xrgb_bitmap_destroy(bitmap: *mut xrgb_bitmap) {
    xbitmap32_destroy(&mut (*bitmap).rgbitmap);
}

pub unsafe fn xrgb_bitmap_set(
    bitmap: *mut xrgb_bitmap,
    start: xfs_rgblock_t,
    len: xfs_extlen_t,
) -> ::std::os::raw::c_int {
    xbitmap32_set(&mut (*bitmap).rgbitmap, start, len)
}

pub unsafe fn xrgb_bitmap_walk(
    bitmap: *mut xrgb_bitmap,
    fn_: xbitmap32_walk_fn,
    priv_: *mut ::std::ffi::c_void,
) -> ::std::os::raw::c_int {
    xbitmap32_walk(&mut (*bitmap).rgbitmap, fn_, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
