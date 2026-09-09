/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2019-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * Temporary storage for online scrub and repair of extended attributes.
 */
#[repr(C)]
pub struct xchk_xattr_buf {
	/* Bitmap of used space in xattr leaf blocks and shortform forks. */
	pub usedmap: *mut core::ffi::c_ulong,

	/* Bitmap of free space in xattr leaf blocks. */
	pub freemap: *mut core::ffi::c_ulong,

	/* Memory buffer used to hold salvaged xattr names. */
	pub name: *mut u8,

	/* Memory buffer used to extract xattr values. */
	pub value: *mut core::ffi::c_void,
	pub value_sz: usize,
}

pub enum xfs_scrub {}

extern "C" {
	pub fn xchk_xattr_set_map(
		sc: *mut xfs_scrub,
		map: *mut core::ffi::c_ulong,
		start: u32,
		len: u32,
	) -> bool;
	pub fn xchk_setup_xattr_buf(sc: *mut xfs_scrub, value_size: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
