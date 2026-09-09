// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Bitmaps, but for type-checked for xfs_ino_t */

#[repr(C)]
pub struct xino_bitmap {
    pub inobitmap: xbitmap64,
}

extern "C" {
    pub fn xbitmap64_init(bitmap: *mut xbitmap64);
    pub fn xbitmap64_destroy(bitmap: *mut xbitmap64);
    pub fn xbitmap64_set(bitmap: *mut xbitmap64, start: xfs_ino_t, len: u64) -> i32;
    pub fn xbitmap64_test(bitmap: *mut xbitmap64, start: xfs_ino_t, len: *mut u64) -> i32;
}

#[inline]
pub unsafe fn xino_bitmap_init(bitmap: *mut xino_bitmap) {
    xbitmap64_init(&mut (*bitmap).inobitmap);
}

#[inline]
pub unsafe fn xino_bitmap_destroy(bitmap: *mut xino_bitmap) {
    xbitmap64_destroy(&mut (*bitmap).inobitmap);
}

#[inline]
pub unsafe fn xino_bitmap_set(bitmap: *mut xino_bitmap, ino: xfs_ino_t) -> i32 {
    xbitmap64_set(&mut (*bitmap).inobitmap, ino, 1)
}

#[inline]
pub unsafe fn xino_bitmap_test(bitmap: *mut xino_bitmap, ino: xfs_ino_t) -> i32 {
    let mut len: u64 = 1;

    xbitmap64_test(&mut (*bitmap).inobitmap, ino, &mut len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
