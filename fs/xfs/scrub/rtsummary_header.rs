// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C header guard: __XFS_SCRUB_RTSUMMARY_H__

#[repr(C)]
pub struct xchk_rtsummary {
    #[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
    pub tempexch: xrep_tempexch,
    pub args: xfs_rtalloc_args,

    pub rextents: u64,
    pub rbmblocks: u64,
    pub rsumblocks: xfs_filblks_t,
    pub rsumlevels: u32,
    pub resblks: u32,

    /* suminfo position of xfile as we write buffers to disk. */
    pub prep_wordoff: xfs_rtsumoff_t,

    /* Memory buffer for the summary comparison. */
    pub words: [xfs_suminfo_raw; 0],
}

extern "C" {
    pub fn xfsum_copyout(
        sc: *mut xfs_scrub,
        sumoff: xfs_rtsumoff_t,
        rawinfo: *mut xfs_suminfo_raw,
    nr_words: u32,
) -> i32;
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_setup_rtsummary(
        sc: *mut xfs_scrub,
        rts: *mut xchk_rtsummary,
) -> i32;
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub unsafe fn xrep_setup_rtsummary(
    _sc: *mut xfs_scrub,
    _rts: *mut xchk_rtsummary,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
