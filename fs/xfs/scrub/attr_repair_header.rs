// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C header guard: __XFS_SCRUB_ATTR_REPAIR_H__

#[repr(C)]
pub struct xrep_tempexch {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_scrub {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn xrep_xattr_swap(sc: *mut xfs_scrub, tx: *mut xrep_tempexch) -> ::std::os::raw::c_int;
    pub fn xrep_xattr_reset_fork(sc: *mut xfs_scrub) -> ::std::os::raw::c_int;
    pub fn xrep_xattr_reset_tempfile_fork(sc: *mut xfs_scrub) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
