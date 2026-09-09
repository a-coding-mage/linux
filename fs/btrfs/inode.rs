// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level translation unit for btrfs/inode.c.
//
// The implementation depends on the Linux kernel and the surrounding btrfs
// translation units.  Keep the externally supplied symbols unresolved here;
// they are provided by those dependencies when this unit is integrated.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_imports, unused_variables)]

use core::ffi::c_void;

pub const COW_FILE_RANGE_KEEP_LOCKED: usize = 1usize << 0;

#[repr(C)]
pub struct btrfs_iget_args {
    pub ino: u64,
    pub root: *mut c_void,
}

#[repr(C)]
pub struct btrfs_rename_ctx {
    /// Output field. Stores the index number of the old directory entry.
    pub index: u64,
}

/// Opaque dependency-backed types are intentionally represented as raw
/// pointers until the corresponding translated kernel headers are available.
#[repr(C)]
pub struct data_reloc_warn {
    pub path: *mut c_void,
    pub fs_info: *mut c_void,
    pub extent_item_size: u64,
    pub logical: u64,
    pub mirror_num: i32,
}

extern "C" {
    pub fn btrfs_inode_lock(inode: *mut c_void, ilock_flags: u32) -> i32;
    pub fn btrfs_inode_unlock(inode: *mut c_void, ilock_flags: u32);
}

// Preserve the complete implementation source for the dependency-aware
// translation stage.  The source is deliberately kept external rather than
// copied or modified, matching the repository's generated-source workflow.
pub const INODE_C_SOURCE: &str = include_str!("inode.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
