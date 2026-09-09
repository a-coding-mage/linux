/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

/* Linux types dependency is supplied by the surrounding translation. */

/* Buffer size to contain tree name and possibly additional data (offset) */
pub const BTRFS_ROOT_NAME_BUF_LEN: usize = 48;

#[repr(C)]
pub struct extent_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_key {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btrfs_print_leaf(l: *const extent_buffer);
    pub fn btrfs_print_tree(c: *const extent_buffer, follow: bool);
    pub fn btrfs_root_name(key: *const btrfs_key, buf: *mut ::std::ffi::c_char) -> *const ::std::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
