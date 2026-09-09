/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// #include <linux/exportfs.h>
// #include <linux/types.h>

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct export_operations {
    _private: [u8; 0],
}

extern "C" {
    pub static btrfs_export_ops: export_operations;
}

#[repr(C, packed)]
pub struct btrfs_fid {
    pub objectid: u64,
    pub root_objectid: u64,
    pub gen: u32,

    pub parent_objectid: u64,
    pub parent_gen: u32,

    pub parent_root_objectid: u64,
}

extern "C" {
    pub fn btrfs_get_dentry(
        sb: *mut super_block,
        objectid: u64,
        root_objectid: u64,
        generation: u64,
    ) -> *mut dentry;
    pub fn btrfs_get_parent(child: *mut dentry) -> *mut dentry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
