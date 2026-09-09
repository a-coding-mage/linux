/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2008-2021 Jean-Pierre Andre
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

// Types are supplied by the corresponding NTFS implementation.
pub struct ntfs_inode;
pub struct ntfs_volume;
pub struct dentry;

pub type mode_t = u32;

extern "C" {
    pub static mut reparse_index_name: [u16; 0];

    pub fn ntfs_parse_reparse(ni: *mut ntfs_inode, mode: *mut c_uint) -> c_int;
    pub fn ntfs_reparse_tag_dt_types(vol: *mut ntfs_volume, mref: c_ulong) -> c_uint;
    pub fn ntfs_translate_symlink_path(
        dentry: *mut dentry,
        target: *const c_char,
        translated: *mut *mut c_char,
    ) -> c_int;
    pub fn ntfs_reparse_set_wsl_symlink(
        ni: *mut ntfs_inode,
        target: *const c_char,
        target_len: c_int,
    ) -> c_int;
    pub fn ntfs_reparse_set_native_symlink(
        ni: *mut ntfs_inode,
        symname: *const c_char,
        symlen: c_int,
    ) -> c_int;
    pub fn ntfs_reparse_set_wsl_not_symlink(ni: *mut ntfs_inode, mode: mode_t) -> c_int;
    pub fn ntfs_delete_reparse_index(ni: *mut ntfs_inode) -> c_int;
    pub fn ntfs_remove_ntfs_reparse_data(ni: *mut ntfs_inode) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
