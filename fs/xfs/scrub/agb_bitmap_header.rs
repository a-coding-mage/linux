// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* C header guard: __XFS_SCRUB_AGB_BITMAP_H__ */

/* Bitmaps, but for type-checked for xfs_agblock_t */

#[repr(C)]
pub struct xagb_bitmap {
    pub agbitmap: xbitmap32,
}

unsafe extern "C" {
    fn xbitmap32_init(bitmap: *mut xbitmap32);
    fn xbitmap32_destroy(bitmap: *mut xbitmap32);
    fn xbitmap32_clear(bitmap: *mut xbitmap32, start: xfs_agblock_t, len: xfs_extlen_t) -> i32;
    fn xbitmap32_set(bitmap: *mut xbitmap32, start: xfs_agblock_t, len: xfs_extlen_t) -> i32;
    fn xbitmap32_test(
        bitmap: *mut xbitmap32,
        start: xfs_agblock_t,
        len: *mut xfs_extlen_t,
    ) -> bool;
    fn xbitmap32_disunion(bitmap: *mut xbitmap32, sub: *mut xbitmap32) -> i32;
    fn xbitmap32_hweight(bitmap: *mut xbitmap32) -> u32;
    fn xbitmap32_empty(bitmap: *mut xbitmap32) -> bool;
    fn xbitmap32_walk(
        bitmap: *mut xbitmap32,
        fn_: xbitmap32_walk_fn,
        priv_: *mut core::ffi::c_void,
    ) -> i32;
    fn xbitmap32_count_set_regions(bitmap: *mut xbitmap32) -> u32;

    fn xagb_bitmap_set_btblocks(bitmap: *mut xagb_bitmap, cur: *mut xfs_btree_cur) -> i32;
    fn xagb_bitmap_set_btcur_path(bitmap: *mut xagb_bitmap, cur: *mut xfs_btree_cur) -> i32;
}

#[inline]
pub unsafe fn xagb_bitmap_init(bitmap: *mut xagb_bitmap) {
    xbitmap32_init(&mut (*bitmap).agbitmap);
}

#[inline]
pub unsafe fn xagb_bitmap_destroy(bitmap: *mut xagb_bitmap) {
    xbitmap32_destroy(&mut (*bitmap).agbitmap);
}

#[inline]
pub unsafe fn xagb_bitmap_clear(
    bitmap: *mut xagb_bitmap,
    start: xfs_agblock_t,
    len: xfs_extlen_t,
) -> i32 {
    xbitmap32_clear(&mut (*bitmap).agbitmap, start, len)
}

#[inline]
pub unsafe fn xagb_bitmap_set(
    bitmap: *mut xagb_bitmap,
    start: xfs_agblock_t,
    len: xfs_extlen_t,
) -> i32 {
    xbitmap32_set(&mut (*bitmap).agbitmap, start, len)
}

#[inline]
pub unsafe fn xagb_bitmap_test(
    bitmap: *mut xagb_bitmap,
    start: xfs_agblock_t,
    len: *mut xfs_extlen_t,
) -> bool {
    xbitmap32_test(&mut (*bitmap).agbitmap, start, len)
}

#[inline]
pub unsafe fn xagb_bitmap_disunion(
    bitmap: *mut xagb_bitmap,
    sub: *mut xagb_bitmap,
) -> i32 {
    xbitmap32_disunion(&mut (*bitmap).agbitmap, &mut (*sub).agbitmap)
}

#[inline]
pub unsafe fn xagb_bitmap_hweight(bitmap: *mut xagb_bitmap) -> u32 {
    xbitmap32_hweight(&mut (*bitmap).agbitmap)
}

#[inline]
pub unsafe fn xagb_bitmap_empty(bitmap: *mut xagb_bitmap) -> bool {
    xbitmap32_empty(&mut (*bitmap).agbitmap)
}

#[inline]
pub unsafe fn xagb_bitmap_walk(
    bitmap: *mut xagb_bitmap,
    fn_: xbitmap32_walk_fn,
    priv_: *mut core::ffi::c_void,
) -> i32 {
    xbitmap32_walk(&mut (*bitmap).agbitmap, fn_, priv_)
}

#[inline]
pub unsafe fn xagb_bitmap_count_set_regions(bitmap: *mut xagb_bitmap) -> u32 {
    xbitmap32_count_set_regions(&mut (*bitmap).agbitmap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
