/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Cache definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2011,2014 The Linux Foundation. All rights reserved.
 */

/* Bytes per L1 cache line */
pub const L1_CACHE_SHIFT: u32 = 5;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

/*
 * C macros __cacheline_aligned and ____cacheline_aligned expand to the
 * __aligned(L1_CACHE_BYTES) declaration attribute.
 */

/* See http://lwn.net/Articles/262554/ */
/* C macro __read_mostly is intentionally empty. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
