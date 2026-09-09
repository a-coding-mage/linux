/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/fiemap.h>

#[repr(C)]
pub struct inode;

#[repr(C)]
pub struct fiemap_extent_info;

unsafe extern "C" {
    pub fn btrfs_fiemap(
        inode: *mut inode,
        fieinfo: *mut fiemap_extent_info,
        start: u64,
        len: u64,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
