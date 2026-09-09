// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* btree scrub */

/* Check for btree operation errors. */
extern "C" {
    pub fn xchk_btree_process_error(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        level: ::core::ffi::c_int,
        error: *mut ::core::ffi::c_int,
    ) -> bool;

    /* Check for btree xref operation errors. */
    pub fn xchk_btree_xref_process_error(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        level: ::core::ffi::c_int,
        error: *mut ::core::ffi::c_int,
    ) -> bool;

    /* Check for btree corruption. */
    pub fn xchk_btree_set_corrupt(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        level: ::core::ffi::c_int,
    );
    pub fn xchk_btree_set_preen(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        level: ::core::ffi::c_int,
    );

    /* Check for btree xref discrepancies. */
    pub fn xchk_btree_xref_set_corrupt(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        level: ::core::ffi::c_int,
    );
}

pub type xchk_btree_rec_fn = unsafe extern "C" fn(
    bs: *mut xchk_btree,
    rec: *const xfs_btree_rec,
) -> ::core::ffi::c_int;

#[repr(C)]
pub struct xchk_btree_key {
    pub key: xfs_btree_key,
    pub valid: bool,
}

#[repr(C)]
pub struct xchk_btree {
    /* caller-provided scrub state */
    pub sc: *mut xfs_scrub,
    pub cur: *mut xfs_btree_cur,
    pub scrub_rec: xchk_btree_rec_fn,
    pub oinfo: *const xfs_owner_info,
    pub private: *mut ::core::ffi::c_void,

    /* internal scrub state */
    pub lastrec_valid: bool,
    pub lastrec: xfs_btree_rec,
    pub to_check: list_head,

    /* this element must come last! */
    pub lastkey: [xchk_btree_key; 0],
}

/*
 * Calculate the size of a xchk_btree structure.  There are nlevels-1 slots for
 * keys because we track leaf records separately in lastrec.
 */
#[inline]
pub unsafe fn xchk_btree_sizeof(nlevels: ::core::ffi::c_uint) -> usize {
    ::core::mem::size_of::<xchk_btree>()
        + (nlevels.wrapping_sub(1) as usize) * ::core::mem::size_of::<xchk_btree_key>()
}

extern "C" {
    pub fn xchk_btree(
        sc: *mut xfs_scrub,
        cur: *mut xfs_btree_cur,
        scrub_fn: xchk_btree_rec_fn,
        oinfo: *const xfs_owner_info,
        private: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
