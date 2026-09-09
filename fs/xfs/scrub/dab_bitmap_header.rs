// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * C header guard: __XFS_SCRUB_DAB_BITMAP_H__
 *
 * Bitmaps, but for type-checked for xfs_dablk_t.
 *
 * The following types and functions are supplied by the translated
 * dependencies of this header.
 */

#[repr(C)]
pub struct xdab_bitmap {
    pub dabitmap: xbitmap32,
}

#[inline]
pub unsafe fn xdab_bitmap_init(bitmap: *mut xdab_bitmap) {
    xbitmap32_init(&mut (*bitmap).dabitmap);
}

#[inline]
pub unsafe fn xdab_bitmap_destroy(bitmap: *mut xdab_bitmap) {
    xbitmap32_destroy(&mut (*bitmap).dabitmap);
}

#[inline]
pub unsafe fn xdab_bitmap_set(
    bitmap: *mut xdab_bitmap,
    dabno: xfs_dablk_t,
    len: xfs_extlen_t,
) -> ::core::ffi::c_int {
    xbitmap32_set(&mut (*bitmap).dabitmap, dabno, len)
}

#[inline]
pub unsafe fn xdab_bitmap_test(
    bitmap: *mut xdab_bitmap,
    dabno: xfs_dablk_t,
    len: *mut xfs_extlen_t,
) -> bool {
    xbitmap32_test(&mut (*bitmap).dabitmap, dabno, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
