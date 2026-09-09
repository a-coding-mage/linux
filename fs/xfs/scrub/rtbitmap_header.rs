// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * We use an xfile to construct new bitmap blocks for the portion of the
 * rtbitmap file that we're replacing.  Whereas the ondisk bitmap must be
 * accessed through the buffer cache, the xfile bitmap supports direct
 * word-level accesses.  Therefore, we create a small abstraction for linear
 * access.
 */
pub type xrep_wordoff_t = ::core::ffi::c_ulonglong;
pub type xrep_wordcnt_t = ::core::ffi::c_uint;

/* Mask to round an rtx down to the nearest bitmap word. */
pub const XREP_RTBMP_WORDMASK: u64 = (1u64 << XFS_NBWORDLOG) - 1;

#[repr(C)]
pub struct xchk_rtbitmap {
    pub sc: *mut xfs_scrub,

    pub rextents: u64,
    pub rbmblocks: u64,
    pub rextslog: ::core::ffi::c_uint,
    pub resblks: ::core::ffi::c_uint,

    /* The next free rt group block number that we expect to see. */
    pub next_free_rgbno: xfs_rgblock_t,

    #[cfg(CONFIG_XFS_ONLINE_REPAIR)]
    /* stuff for staging a new bitmap */
    pub args: xfs_rtalloc_args,
    #[cfg(CONFIG_XFS_ONLINE_REPAIR)]
    pub tempexch: xrep_tempexch,

    /* The next rtgroup block we expect to see during our rtrmapbt walk. */
    pub next_rgbno: xfs_rgblock_t,

    /* rtgroup lock flags */
    pub rtglock_flags: ::core::ffi::c_uint,

    /* rtword position of xfile as we write buffers to disk. */
    pub prep_wordoff: xrep_wordoff_t,

    /* In-Memory rtbitmap for repair. */
    pub words: [xfs_rtword_raw; 0],
}

#[cfg(CONFIG_XFS_ONLINE_REPAIR)]
extern "C" {
    pub fn xrep_setup_rtbitmap(
        sc: *mut xfs_scrub,
        rtb: *mut xchk_rtbitmap,
    ) -> ::core::ffi::c_int;
}

/*
 * How big should the words[] buffer be?
 *
 * For repairs, we want a full fsblock worth of space so that we can memcpy a
 * buffer full of 1s into the xfile bitmap.  The xfile bitmap doesn't have
 * rtbitmap block headers, so we don't use blockwsize.  Scrub doesn't use the
 * words buffer at all.
 */
#[cfg(CONFIG_XFS_ONLINE_REPAIR)]
#[inline]
pub unsafe fn xchk_rtbitmap_wordcnt(sc: *mut xfs_scrub) -> ::core::ffi::c_uint {
    if xchk_could_repair(sc) {
        return (*(*sc).mp).m_sb.sb_blocksize >> XFS_WORDLOG;
    }
    0
}

#[cfg(not(CONFIG_XFS_ONLINE_REPAIR))]
#[inline]
pub unsafe fn xrep_setup_rtbitmap(
    _sc: *mut xfs_scrub,
    _rtb: *mut xchk_rtbitmap,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_XFS_ONLINE_REPAIR))]
#[inline]
pub unsafe fn xchk_rtbitmap_wordcnt(_sc: *mut xfs_scrub) -> ::core::ffi::c_uint {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
