/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Dependency intent: declarations below refer to types supplied by other XFS headers. */

/* Fake root for an AG-rooted btree. */
#[repr(C)]
pub struct xbtree_afakeroot {
    /* AG block number of the new btree root. */
    pub af_root: xfs_agblock_t,

    /* Height of the new btree. */
    pub af_levels: ::core::ffi::c_uint,

    /* Number of blocks used by the btree. */
    pub af_blocks: ::core::ffi::c_uint,
}

/* Cursor interactions with fake roots for AG-rooted btrees. */
unsafe extern "C" {
    pub fn xfs_btree_stage_afakeroot(
        cur: *mut xfs_btree_cur,
        afake: *mut xbtree_afakeroot,
    );
    pub fn xfs_btree_commit_afakeroot(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    );
}

/* Fake root for an inode-rooted btree. */
#[repr(C)]
pub struct xbtree_ifakeroot {
    /* Fake inode fork. */
    pub if_fork: *mut xfs_ifork,

    /* Number of blocks used by the btree. */
    pub if_blocks: i64,

    /* Height of the new btree. */
    pub if_levels: ::core::ffi::c_uint,

    /* Number of bytes available for this fork in the inode. */
    pub if_fork_size: ::core::ffi::c_uint,
}

/* Cursor interactions with fake roots for inode-rooted btrees. */
unsafe extern "C" {
    pub fn xfs_btree_stage_ifakeroot(
        cur: *mut xfs_btree_cur,
        ifake: *mut xbtree_ifakeroot,
    );
    pub fn xfs_btree_commit_ifakeroot(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        whichfork: ::core::ffi::c_int,
    );
}

/* Bulk loading of staged btrees. */
pub type xfs_btree_bload_get_records_fn = unsafe extern "C" fn(
    cur: *mut xfs_btree_cur,
    idx: ::core::ffi::c_uint,
    block: *mut xfs_btree_block,
    nr_wanted: ::core::ffi::c_uint,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type xfs_btree_bload_claim_block_fn = unsafe extern "C" fn(
    cur: *mut xfs_btree_cur,
    ptr: *mut xfs_btree_ptr,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type xfs_btree_bload_iroot_size_fn = unsafe extern "C" fn(
    cur: *mut xfs_btree_cur,
    level: ::core::ffi::c_uint,
    nr_this_level: ::core::ffi::c_uint,
    priv_: *mut ::core::ffi::c_void,
) -> usize;

#[repr(C)]
pub struct xfs_btree_bload {
    /* This function loads records into the btree in sort order. */
    pub get_records: Option<xfs_btree_bload_get_records_fn>,
    /* This function claims preallocated space for a new btree block. */
    pub claim_block: Option<xfs_btree_bload_claim_block_fn>,
    /* Return the size of the in-core btree root block. */
    pub iroot_size: Option<xfs_btree_bload_iroot_size_fn>,
    /* Number of records stored in the new btree. */
    pub nr_records: u64,
    /* Number of free records to leave in each leaf block. */
    pub leaf_slack: ::core::ffi::c_int,
    /* Number of free key/ptr pairs to leave in each node block. */
    pub node_slack: ::core::ffi::c_int,
    /* Number of btree blocks needed to store nr_records records. */
    pub nr_blocks: u64,
    /* Height of the new btree. */
    pub btree_height: ::core::ffi::c_uint,
    /* Flush buffers after this many blocks have been formatted. */
    pub max_dirty: u16,
    /* Number of dirty buffers. */
    pub nr_dirty: u16,
}

unsafe extern "C" {
    pub fn xfs_btree_bload_compute_geometry(
        cur: *mut xfs_btree_cur,
        bbl: *mut xfs_btree_bload,
        nr_records: u64,
    ) -> ::core::ffi::c_int;
    pub fn xfs_btree_bload(
        cur: *mut xfs_btree_cur,
        bbl: *mut xfs_btree_bload,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
