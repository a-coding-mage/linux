// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Bitmaps, but type-checked for xfs_fileoff_t. */

#[repr(C)]
pub struct xoff_bitmap {
	pub offbitmap: xbitmap64,
}

pub unsafe fn xoff_bitmap_init(bitmap: *mut xoff_bitmap) {
	xbitmap64_init(&mut (*bitmap).offbitmap);
}

pub unsafe fn xoff_bitmap_destroy(bitmap: *mut xoff_bitmap) {
	xbitmap64_destroy(&mut (*bitmap).offbitmap);
}

pub unsafe fn xoff_bitmap_set(
	bitmap: *mut xoff_bitmap,
	off: xfs_fileoff_t,
	len: xfs_filblks_t,
) -> i32 {
	xbitmap64_set(&mut (*bitmap).offbitmap, off, len)
}

pub unsafe fn xoff_bitmap_walk(
	bitmap: *mut xoff_bitmap,
	fn_: xbitmap64_walk_fn,
	priv_: *mut core::ffi::c_void,
) -> i32 {
	xbitmap64_walk(&mut (*bitmap).offbitmap, fn_, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
