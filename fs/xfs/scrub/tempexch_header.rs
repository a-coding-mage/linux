// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Conditional on CONFIG_XFS_ONLINE_REPAIR in the original C header.
#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
#[repr(C)]
pub struct xrep_tempexch {
    pub req: xfs_exchmaps_req,
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_tempexch_trans_reserve(
        sc: *mut xfs_scrub,
        whichfork: ::std::os::raw::c_int,
        off: xfs_fileoff_t,
        len: xfs_filblks_t,
        ti: *mut xrep_tempexch,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_tempexch_trans_alloc(
        sc: *mut xfs_scrub,
        whichfork: ::std::os::raw::c_int,
        ti: *mut xrep_tempexch,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_tempexch_contents(
        sc: *mut xfs_scrub,
        ti: *mut xrep_tempexch,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
