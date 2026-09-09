// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Bitmaps, but type-checked for xfs_agino_t.
//
// The types and functions referenced below are supplied by the corresponding
// translated dependencies.

#[repr(C)]
pub struct xagino_bitmap {
    pub aginobitmap: xbitmap32,
}

extern "C" {
    fn xbitmap32_init(bitmap: *mut xbitmap32);
    fn xbitmap32_destroy(bitmap: *mut xbitmap32);
    fn xbitmap32_clear(bitmap: *mut xbitmap32, start: xfs_agino_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn xbitmap32_set(bitmap: *mut xbitmap32, start: xfs_agino_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn xbitmap32_test(
        bitmap: *mut xbitmap32,
        start: xfs_agino_t,
        len: *mut ::core::ffi::c_uint,
    ) -> bool;
    fn xbitmap32_walk(
        bitmap: *mut xbitmap32,
        fn_: xbitmap32_walk_fn,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn xagino_bitmap_init(bitmap: *mut xagino_bitmap) {
    xbitmap32_init(&mut (*bitmap).aginobitmap);
}

#[inline]
pub unsafe fn xagino_bitmap_destroy(bitmap: *mut xagino_bitmap) {
    xbitmap32_destroy(&mut (*bitmap).aginobitmap);
}

#[inline]
pub unsafe fn xagino_bitmap_clear(
    bitmap: *mut xagino_bitmap,
    agino: xfs_agino_t,
    len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    xbitmap32_clear(&mut (*bitmap).aginobitmap, agino, len)
}

#[inline]
pub unsafe fn xagino_bitmap_set(
    bitmap: *mut xagino_bitmap,
    agino: xfs_agino_t,
    len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    xbitmap32_set(&mut (*bitmap).aginobitmap, agino, len)
}

#[inline]
pub unsafe fn xagino_bitmap_test(
    bitmap: *mut xagino_bitmap,
    agino: xfs_agino_t,
    len: *mut ::core::ffi::c_uint,
) -> bool {
    xbitmap32_test(&mut (*bitmap).aginobitmap, agino, len)
}

#[inline]
pub unsafe fn xagino_bitmap_walk(
    bitmap: *mut xagino_bitmap,
    fn_: xbitmap32_walk_fn,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    xbitmap32_walk(&mut (*bitmap).aginobitmap, fn_, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
