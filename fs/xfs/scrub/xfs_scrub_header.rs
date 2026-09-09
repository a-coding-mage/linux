// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C header guard: __XFS_SCRUB_H__

// When CONFIG_XFS_ONLINE_SCRUB is not enabled, these macros return -ENOTTY.
#[cfg(not(CONFIG_XFS_ONLINE_SCRUB))]
macro_rules! xfs_ioc_scrub_metadata {
    ($f:expr, $a:expr) => { -ENOTTY };
}

#[cfg(not(CONFIG_XFS_ONLINE_SCRUB))]
macro_rules! xfs_ioc_scrubv_metadata {
    ($f:expr, $a:expr) => { -ENOTTY };
}

// The C declaration is supplied by the surrounding kernel type definitions.
#[cfg(CONFIG_XFS_ONLINE_SCRUB)]
unsafe extern "C" {
    pub fn xfs_ioc_scrub_metadata(
        file: *mut file,
        arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn xfs_ioc_scrubv_metadata(
        file: *mut file,
        arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
