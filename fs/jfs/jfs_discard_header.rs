/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) Tino Reichardt, 2012
 */

// Forward declarations corresponding to the C structs used by this header.
#[repr(C)]
pub struct fstrim_range;

#[repr(C)]
pub struct inode;

pub unsafe extern "C" {
    pub fn jfs_issue_discard(ip: *mut inode, blkno: u64, nblocks: u64);
    pub fn jfs_ioc_trim(ip: *mut inode, range: *mut fstrim_range) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
