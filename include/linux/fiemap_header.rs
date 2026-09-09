/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/linux/fiemap.h>
// Dependency: <linux/fs.h>

/**
 * struct fiemap_extent_info - fiemap request to a filesystem
 * @fi_flags:          Flags as passed from user
 * @fi_extents_mapped: Number of mapped extents
 * @fi_extents_max:    Size of fiemap_extent array
 * @fi_extents_start:  Start of fiemap_extent array
 */
#[repr(C)]
pub struct fiemap_extent_info {
    pub fi_flags: ::std::os::raw::c_uint,
    pub fi_extents_mapped: ::std::os::raw::c_uint,
    pub fi_extents_max: ::std::os::raw::c_uint,
    pub fi_extents_start: *mut fiemap_extent,
}

unsafe extern "C" {
    pub fn fiemap_prep(
        inode: *mut inode,
        fieinfo: *mut fiemap_extent_info,
        start: u64,
        len: *mut u64,
        supported_flags: u32,
    ) -> ::std::os::raw::c_int;

    pub fn fiemap_fill_next_extent(
        info: *mut fiemap_extent_info,
        logical: u64,
        phys: u64,
        len: u64,
        flags: u32,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
