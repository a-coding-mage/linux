/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// C header dependency: declarations such as `xfs_mount`, `xfs_da_args`,
// `xfs_inode`, `xfs_bmbt_irec`, `xfs_attr_intent`, `xfs_buf_flags_t`, and
// `XFS_XATTR_SIZE_MAX` are supplied by other translation units.

extern "C" {
    pub fn xfs_attr3_rmt_blocks(mp: *mut xfs_mount, attrlen: u32) -> u32;

    pub fn xfs_attr_rmtval_get(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_rmtval_stale(
        ip: *mut xfs_inode,
        map: *mut xfs_bmbt_irec,
        incore_flags: xfs_buf_flags_t,
    ) -> i32;
    pub fn xfs_attr_rmtval_invalidate(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_rmtval_remove(attr: *mut xfs_attr_intent) -> i32;
    pub fn xfs_attr_rmt_find_hole(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_rmtval_set_value(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_rmtval_set_blk(attr: *mut xfs_attr_intent) -> i32;
    pub fn xfs_attr_rmtval_find_space(attr: *mut xfs_attr_intent) -> i32;
}

/// Number of rmt blocks needed to store the maximally sized attr value
#[inline]
pub unsafe fn xfs_attr3_max_rmt_blocks(mp: *mut xfs_mount) -> u32 {
    xfs_attr3_rmt_blocks(mp, XFS_XATTR_SIZE_MAX)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
