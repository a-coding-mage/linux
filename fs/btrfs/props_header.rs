/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Filipe David Borba Manana <fdmanana@gmail.com>
 */

// Translated from the C header. Linux kernel build-time annotations and
// included type definitions are supplied by the surrounding translation.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct btrfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

extern "C" {
    // C declaration: int __init btrfs_props_init(void);
    pub fn btrfs_props_init() -> c_int;

    pub fn btrfs_set_prop(
        trans: *mut btrfs_trans_handle,
        inode: *mut btrfs_inode,
        name: *const c_char,
        value: *const c_char,
        value_len: usize,
        flags: c_int,
    ) -> c_int;

    pub fn btrfs_validate_prop(
        inode: *const btrfs_inode,
        name: *const c_char,
        value: *const c_char,
        value_len: usize,
    ) -> c_int;

    pub fn btrfs_ignore_prop(inode: *const btrfs_inode, name: *const c_char) -> bool;

    pub fn btrfs_load_inode_props(
        inode: *mut btrfs_inode,
        path: *mut btrfs_path,
    ) -> c_int;

    pub fn btrfs_inode_inherit_props(
        trans: *mut btrfs_trans_handle,
        inode: *mut btrfs_inode,
        dir: *const btrfs_inode,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
