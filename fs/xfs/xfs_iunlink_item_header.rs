// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020-2022, Red Hat, Inc.
 * All Rights Reserved.
 */

// Forward declarations from the C header.
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_perag {
    _private: [u8; 0],
}

// These types are supplied by the surrounding XFS implementation.
#[repr(C)]
pub struct xfs_log_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

pub type xfs_agino_t = u32;

/* in memory log item structure */
#[repr(C)]
pub struct xfs_iunlink_item {
    pub item: xfs_log_item,
    pub ip: *mut xfs_inode,
    pub pag: *mut xfs_perag,
    pub next_agino: xfs_agino_t,
    pub old_agino: xfs_agino_t,
}

extern "C" {
    pub static mut xfs_iunlink_cache: *mut kmem_cache;

    pub fn xfs_iunlink_log_inode(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        pag: *mut xfs_perag,
        next_agino: xfs_agino_t,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
