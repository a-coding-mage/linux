// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Bitmaps, but for type-checked for xfs_rtblock_t */

#[repr(C)]
pub struct xrtb_bitmap {
    pub rtbitmap: xbitmap64,
}

#[inline]
pub unsafe fn xrtb_bitmap_init(bitmap: *mut xrtb_bitmap) {
    xbitmap64_init(&mut (*bitmap).rtbitmap);
}

#[inline]
pub unsafe fn xrtb_bitmap_destroy(bitmap: *mut xrtb_bitmap) {
    xbitmap64_destroy(&mut (*bitmap).rtbitmap);
}

#[inline]
pub unsafe fn xrtb_bitmap_set(
    bitmap: *mut xrtb_bitmap,
    start: xfs_rtblock_t,
    len: xfs_filblks_t,
) -> ::std::os::raw::c_int {
    xbitmap64_set(&mut (*bitmap).rtbitmap, start, len)
}

#[inline]
pub unsafe fn xrtb_bitmap_walk(
    bitmap: *mut xrtb_bitmap,
    fn_: xbitmap64_walk_fn,
    priv_: *mut ::std::ffi::c_void,
) -> ::std::os::raw::c_int {
    xbitmap64_walk(&mut (*bitmap).rtbitmap, fn_, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
