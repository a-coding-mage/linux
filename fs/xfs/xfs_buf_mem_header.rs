// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

pub const XMBUF_BLOCKSIZE: usize = PAGE_SIZE;
pub const XMBUF_BLOCKSHIFT: usize = PAGE_SHIFT;

// The following declarations are enabled when CONFIG_XFS_MEMORY_BUFS is set.
#[cfg(CONFIG_XFS_MEMORY_BUFS)]
#[inline]
pub unsafe fn xfs_buftarg_is_mem(btp: *const xfs_buftarg) -> bool {
    (*btp).bt_bdev.is_null()
}

#[cfg(CONFIG_XFS_MEMORY_BUFS)]
extern "C" {
    pub fn xmbuf_alloc(
        mp: *mut xfs_mount,
        descr: *const ::core::ffi::c_char,
        btpp: *mut *mut xfs_buftarg,
    ) -> ::core::ffi::c_int;
    pub fn xmbuf_free(btp: *mut xfs_buftarg);

    pub fn xmbuf_verify_daddr(btp: *mut xfs_buftarg, daddr: xfs_daddr_t) -> bool;
    pub fn xmbuf_trans_bdetach(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xmbuf_finalize(bp: *mut xfs_buf) -> ::core::ffi::c_int;
}

// When CONFIG_XFS_MEMORY_BUFS is not set, the C variadic macros expand to false.
#[cfg(not(CONFIG_XFS_MEMORY_BUFS))]
#[inline]
pub const fn xfs_buftarg_is_mem() -> bool {
    false
}

#[cfg(not(CONFIG_XFS_MEMORY_BUFS))]
#[inline]
pub const fn xmbuf_verify_daddr() -> bool {
    false
}

extern "C" {
    pub fn xmbuf_map_backing_mem(bp: *mut xfs_buf) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
