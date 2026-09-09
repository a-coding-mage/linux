// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header dependency declarations.
pub enum xfs_mount {}
pub enum xfs_growfs_data {}
pub enum xfs_growfs_log {}
pub enum xfs_free_counter {}

unsafe extern "C" {
    pub fn xfs_growfs_data(mp: *mut xfs_mount, input: *mut xfs_growfs_data) -> ::std::os::raw::c_int;
    pub fn xfs_growfs_log(mp: *mut xfs_mount, input: *mut xfs_growfs_log) -> ::std::os::raw::c_int;
    pub fn xfs_reserve_blocks(
        mp: *mut xfs_mount,
        cnt: xfs_free_counter,
        request: u64,
    ) -> ::std::os::raw::c_int;
    pub fn xfs_fs_goingdown(mp: *mut xfs_mount, inflags: u32) -> ::std::os::raw::c_int;

    pub fn xfs_fs_reserve_ag_blocks(mp: *mut xfs_mount) -> ::std::os::raw::c_int;
    pub fn xfs_fs_unreserve_ag_blocks(mp: *mut xfs_mount);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
