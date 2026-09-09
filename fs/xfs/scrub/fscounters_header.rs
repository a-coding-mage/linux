/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The definition of `xfs_scrub` is supplied by the corresponding dependency.
pub struct xfs_scrub;

#[repr(C)]
pub struct xchk_fscounters {
    pub sc: *mut xfs_scrub,
    pub icount: u64,
    pub ifree: u64,
    pub fdblocks: u64,
    pub frextents: u64,
    pub frextents_delayed: u64,
    pub icount_min: u64,
    pub icount_max: u64,
    pub frozen: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
