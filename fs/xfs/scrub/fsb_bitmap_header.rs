// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Bitmaps, but for type-checked for xfs_fsblock_t */

#[repr(C)]
pub struct xfsb_bitmap {
    pub fsbitmap: xbitmap64,
}

extern "C" {
    pub fn xbitmap64_init(bitmap: *mut xbitmap64);
    pub fn xbitmap64_destroy(bitmap: *mut xbitmap64);
    pub fn xbitmap64_set(
        bitmap: *mut xbitmap64,
        start: xfs_fsblock_t,
        len: xfs_filblks_t,
    ) -> ::core::ffi::c_int;
    pub fn xbitmap64_walk(
        bitmap: *mut xbitmap64,
        fn_: xbitmap64_walk_fn,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn xfsb_bitmap_init(bitmap: *mut xfsb_bitmap) {
    xbitmap64_init(::core::ptr::addr_of_mut!((*bitmap).fsbitmap));
}

#[inline]
pub unsafe fn xfsb_bitmap_destroy(bitmap: *mut xfsb_bitmap) {
    xbitmap64_destroy(::core::ptr::addr_of_mut!((*bitmap).fsbitmap));
}

#[inline]
pub unsafe fn xfsb_bitmap_set(
    bitmap: *mut xfsb_bitmap,
    start: xfs_fsblock_t,
    len: xfs_filblks_t,
) -> ::core::ffi::c_int {
    xbitmap64_set(::core::ptr::addr_of_mut!((*bitmap).fsbitmap), start, len)
}

#[inline]
pub unsafe fn xfsb_bitmap_walk(
    bitmap: *mut xfsb_bitmap,
    fn_: xbitmap64_walk_fn,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    xbitmap64_walk(::core::ptr::addr_of_mut!((*bitmap).fsbitmap), fn_, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
