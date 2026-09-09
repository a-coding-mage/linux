// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// The definitions of these C types are supplied by the surrounding headers.
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

pub type loff_t = i64;

extern "C" {
    pub static xfs_file_operations: file_operations;
    pub static xfs_dir_file_operations: file_operations;

    pub fn xfs_is_falloc_aligned(
        ip: *mut xfs_inode,
        pos: loff_t,
        len: i64,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
