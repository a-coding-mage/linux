// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* dir/attr btree */

#[repr(C)]
pub struct xchk_da_btree {
    pub dargs: xfs_da_args,
    pub hashes: [xfs_dahash_t; XFS_DA_NODE_MAXDEPTH],
    pub maxrecs: [::core::ffi::c_int; XFS_DA_NODE_MAXDEPTH],
    pub state: *mut xfs_da_state,
    pub sc: *mut xfs_scrub,
    pub private: *mut ::core::ffi::c_void,

    /*
     * Lowest and highest directory block address in which we expect
     * to find dir/attr btree node blocks.  For a directory this
     * (presumably) means between LEAF_OFFSET and FREE_OFFSET; for
     * attributes there is no limit.
     */
    pub lowest: xfs_dablk_t,
    pub highest: xfs_dablk_t,

    pub tree_level: ::core::ffi::c_int,
}

pub type xchk_da_btree_rec_fn =
    unsafe extern "C" fn(ds: *mut xchk_da_btree, level: ::core::ffi::c_int) -> ::core::ffi::c_int;

unsafe extern "C" {
    /* Check for da btree operation errors. */
    pub fn xchk_da_process_error(
        ds: *mut xchk_da_btree,
        level: ::core::ffi::c_int,
        error: *mut ::core::ffi::c_int,
    ) -> bool;

    /* Check for da btree corruption. */
    pub fn xchk_da_set_corrupt(ds: *mut xchk_da_btree, level: ::core::ffi::c_int);
    pub fn xchk_da_set_preen(ds: *mut xchk_da_btree, level: ::core::ffi::c_int);

    pub fn xchk_da_btree_hash(
        ds: *mut xchk_da_btree,
        level: ::core::ffi::c_int,
        hashp: *mut __be32,
    ) -> ::core::ffi::c_int;
    pub fn xchk_da_btree(
        sc: *mut xfs_scrub,
        whichfork: ::core::ffi::c_int,
        scrub_fn: xchk_da_btree_rec_fn,
        private: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// The C header repeats the xchk_da_set_preen declaration; Rust declarations
// cannot be duplicated without defining the same item twice.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
