/* SPDX-License-Identifier: GPL-2.0 */

// Translated from super.h. C includes and the header guard are omitted;
// referenced types, constants, and helpers are supplied by other files.

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut btrfs_fs_info,
    pub s_flags: c_ulong,
}

#[repr(C)]
pub struct btrfs_fs_info {
    pub fs_state: c_ulong,
}

unsafe extern "C" {
    pub fn btrfs_check_options(
        info: *const btrfs_fs_info,
        mount_opt: *mut u64,
        flags: c_ulong,
    ) -> bool;
    pub fn btrfs_sync_fs(sb: *mut super_block, wait: c_int) -> c_int;
    pub fn btrfs_get_subvol_name_from_objectid(
        fs_info: *mut btrfs_fs_info,
        subvol_objectid: u64,
    ) -> *mut c_char;
    pub fn btrfs_set_free_space_cache_settings(fs_info: *mut btrfs_fs_info);

    pub fn set_bit(nr: c_ulong, addr: *mut c_ulong);
    pub fn clear_bit(nr: c_ulong, addr: *mut c_ulong);
}

// Supplied by the filesystem headers.
unsafe extern "C" {
    pub static SB_RDONLY: c_ulong;
    pub static BTRFS_FS_STATE_RO: c_ulong;
}

#[inline]
pub unsafe fn btrfs_sb(sb: *const super_block) -> *mut btrfs_fs_info {
    (*sb).s_fs_info
}

#[inline]
pub unsafe fn btrfs_set_sb_rdonly(sb: *mut super_block) {
    (*sb).s_flags |= SB_RDONLY;
    set_bit(BTRFS_FS_STATE_RO, &mut (*btrfs_sb(sb)).fs_state);
}

#[inline]
pub unsafe fn btrfs_clear_sb_rdonly(sb: *mut super_block) {
    (*sb).s_flags &= !SB_RDONLY;
    clear_bit(BTRFS_FS_STATE_RO, &mut (*btrfs_sb(sb)).fs_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
