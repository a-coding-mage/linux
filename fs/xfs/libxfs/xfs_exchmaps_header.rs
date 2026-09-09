/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* In-core deferred operation info about a file mapping exchange request. */
#[repr(C)]
pub struct xfs_exchmaps_intent {
    /* List of other incore deferred work. */
    pub xmi_list: list_head,

    /* Inodes participating in the operation. */
    pub xmi_ip1: *mut xfs_inode,
    pub xmi_ip2: *mut xfs_inode,

    /* File offset range information. */
    pub xmi_startoff1: xfs_fileoff_t,
    pub xmi_startoff2: xfs_fileoff_t,
    pub xmi_blockcount: xfs_filblks_t,

    /* Set these file sizes after the operation, unless negative. */
    pub xmi_isize1: xfs_fsize_t,
    pub xmi_isize2: xfs_fsize_t,

    pub xmi_flags: u64, /* XFS_EXCHMAPS_* flags */
}

/* Try to convert inode2 from block to short format at the end, if possible. */
pub const __XFS_EXCHMAPS_INO2_SHORTFORM: u64 = 1u64 << 63;

pub const XFS_EXCHMAPS_INTERNAL_FLAGS: u64 = __XFS_EXCHMAPS_INO2_SHORTFORM;

/* flags that can be passed to xfs_exchmaps_{estimate,mappings} */
pub const XFS_EXCHMAPS_PARAMS: u64 =
    XFS_EXCHMAPS_ATTR_FORK | XFS_EXCHMAPS_SET_SIZES | XFS_EXCHMAPS_INO1_WRITTEN;

pub unsafe fn xfs_exchmaps_whichfork(xmi: *const xfs_exchmaps_intent) -> ::core::ffi::c_int {
    if (*xmi).xmi_flags & XFS_EXCHMAPS_ATTR_FORK != 0 {
        return XFS_ATTR_FORK;
    }
    XFS_DATA_FORK
}

/* Parameters for a mapping exchange request. */
#[repr(C)]
pub struct xfs_exchmaps_req {
    /* Inodes participating in the operation. */
    pub ip1: *mut xfs_inode,
    pub ip2: *mut xfs_inode,

    /* File offset range information. */
    pub startoff1: xfs_fileoff_t,
    pub startoff2: xfs_fileoff_t,
    pub blockcount: xfs_filblks_t,

    /* XFS_EXCHMAPS_* operation flags */
    pub flags: u64,

    /*
     * Fields below this line are filled out by xfs_exchmaps_estimate;
     * callers should initialize this part of the struct to zero.
     */

    /*
     * Data device blocks to be moved out of ip1, and free space needed to
     * handle the bmbt changes.
     */
    pub ip1_bcount: xfs_filblks_t,

    /*
     * Data device blocks to be moved out of ip2, and free space needed to
     * handle the bmbt changes.
     */
    pub ip2_bcount: xfs_filblks_t,

    /* rt blocks to be moved out of ip1. */
    pub ip1_rtbcount: xfs_filblks_t,

    /* rt blocks to be moved out of ip2. */
    pub ip2_rtbcount: xfs_filblks_t,

    /* Free space needed to handle the bmbt changes */
    pub resblks: u64,

    /* Number of exchanges needed to complete the operation */
    pub nr_exchanges: u64,
}

pub unsafe fn xfs_exchmaps_reqfork(req: *const xfs_exchmaps_req) -> ::core::ffi::c_int {
    if (*req).flags & XFS_EXCHMAPS_ATTR_FORK != 0 {
        return XFS_ATTR_FORK;
    }
    XFS_DATA_FORK
}

extern "C" {
    pub fn xfs_exchmaps_estimate_overhead(req: *mut xfs_exchmaps_req) -> ::core::ffi::c_int;
    pub fn xfs_exchmaps_estimate(req: *mut xfs_exchmaps_req) -> ::core::ffi::c_int;

    pub static mut xfs_exchmaps_intent_cache: *mut kmem_cache;

    pub fn xfs_exchmaps_intent_init_cache() -> ::core::ffi::c_int;
    pub fn xfs_exchmaps_intent_destroy_cache();

    pub fn xfs_exchmaps_init_intent(
        req: *const xfs_exchmaps_req,
    ) -> *mut xfs_exchmaps_intent;
    pub fn xfs_exchmaps_ensure_reflink(
        tp: *mut xfs_trans,
        xmi: *const xfs_exchmaps_intent,
    );
    pub fn xfs_exchmaps_upgrade_extent_counts(
        tp: *mut xfs_trans,
        xmi: *const xfs_exchmaps_intent,
    );

    pub fn xfs_exchmaps_finish_one(
        tp: *mut xfs_trans,
        xmi: *mut xfs_exchmaps_intent,
    ) -> ::core::ffi::c_int;

    pub fn xfs_exchmaps_check_forks(
        mp: *mut xfs_mount,
        req: *const xfs_exchmaps_req,
    ) -> ::core::ffi::c_int;

    pub fn xfs_exchange_mappings(tp: *mut xfs_trans, req: *const xfs_exchmaps_req);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
