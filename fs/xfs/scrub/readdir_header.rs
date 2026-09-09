/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

use core::ffi::{c_int, c_void};

/* Types declared by the XFS scrub and filesystem headers. */
#[repr(C)]
pub struct xfs_scrub {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_name {
    _private: [u8; 0],
}

pub type xfs_dir2_dataptr_t = u64;
pub type xfs_ino_t = u64;

pub type xchk_dirent_fn = unsafe extern "C" fn(
    sc: *mut xfs_scrub,
    dp: *mut xfs_inode,
    dapos: xfs_dir2_dataptr_t,
    name: *const xfs_name,
    ino: xfs_ino_t,
    priv_: *mut c_void,
) -> c_int;

unsafe extern "C" {
    pub fn xchk_dir_walk(
        sc: *mut xfs_scrub,
        dp: *mut xfs_inode,
        dirent_fn: xchk_dirent_fn,
        priv_: *mut c_void,
    ) -> c_int;

    pub fn xchk_dir_lookup(
        sc: *mut xfs_scrub,
        dp: *mut xfs_inode,
        name: *const xfs_name,
        ino: *mut xfs_ino_t,
    ) -> c_int;

    pub fn xchk_dir_trylock_for_pptrs(
        sc: *mut xfs_scrub,
        ip: *mut xfs_inode,
        lockmode: *mut u32,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
