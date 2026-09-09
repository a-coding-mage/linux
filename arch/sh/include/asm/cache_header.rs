/* SPDX-License-Identifier: GPL-2.0 */
/* $Id: cache.h,v 1.6 2004/03/11 18:08:05 lethal Exp $
 *
 * include/asm-sh/cache.h
 *
 * Copyright 1999 (C) Niibe Yutaka
 * Copyright 2002, 2003 (C) Paul Mundt
 */

// Corresponds to: #include <linux/init.h>
// Corresponds to: #include <cpu/cache.h>

/// L1 cache line size in bytes.
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

/*
 * Some drivers need to perform DMA into kmalloc'ed buffers
 * and so we have to increase the kmalloc minalign for this.
 */
pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

// Corresponds to the C section attribute macro __read_mostly.

#[repr(C)]
pub struct cache_info {
    pub ways: ::core::ffi::c_uint,      /* Number of cache ways */
    pub sets: ::core::ffi::c_uint,      /* Number of cache sets */
    pub linesz: ::core::ffi::c_uint,    /* Cache line size (bytes) */

    pub way_size: ::core::ffi::c_uint,  /* sets * line size */

    /*
     * way_incr is the address offset for accessing the next way
     * in memory mapped cache array ops.
     */
    pub way_incr: ::core::ffi::c_uint,
    pub entry_shift: ::core::ffi::c_uint,
    pub entry_mask: ::core::ffi::c_uint,

    /*
     * Compute a mask which selects the address bits which overlap between
     * 1. those used to select the cache set during indexing
     * 2. those in the physical page number.
     */
    pub alias_mask: ::core::ffi::c_uint,
    pub n_aliases: ::core::ffi::c_uint, /* Number of aliases */

    pub flags: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
