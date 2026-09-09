// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2012 Red Hat, Inc. All rights reserved.
 */

/* Kernel only symlink definitions */

use std::os::raw::c_char;

extern "C" {
    pub fn xfs_symlink(
        idmap: *mut mnt_idmap,
        dp: *mut xfs_inode,
        link_name: *mut xfs_name,
        target_path: *const c_char,
        mode: umode_t,
        ipp: *mut *mut xfs_inode,
    ) -> ::std::os::raw::c_int;

    pub fn xfs_readlink(ip: *mut xfs_inode, link: *mut c_char) -> ::std::os::raw::c_int;

    pub fn xfs_inactive_symlink(ip: *mut xfs_inode) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
