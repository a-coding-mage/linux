/* SPDX-License-Identifier: GPL-2.0 */

// Translated from btrfs/orphan.h.

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_root {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btrfs_insert_orphan_item(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        offset: u64,
    ) -> ::core::ffi::c_int;

    pub fn btrfs_del_orphan_item(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        offset: u64,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
