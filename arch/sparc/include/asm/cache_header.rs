/* SPDX-License-Identifier: GPL-2.0 */
/* cache.h:  Cache specific code for the Sparc.  These include flushing
 *           and direct tag/data line access.
 *
 * Copyright (C) 1995, 2007 David S. Miller (davem@davemloft.net)
 */

pub const ARCH_SLAB_MINALIGN: usize = core::mem::align_of::<u64>();

pub const L1_CACHE_SHIFT: u32 = 5;
pub const L1_CACHE_BYTES: u32 = 32;

/* CONFIG_SPARC32 selects the 32-bit build-time cache-line shift. */
#[cfg(feature = "CONFIG_SPARC32")]
pub const SMP_CACHE_BYTES_SHIFT: u32 = 5;

#[cfg(not(feature = "CONFIG_SPARC32"))]
pub const SMP_CACHE_BYTES_SHIFT: u32 = 6;

pub const SMP_CACHE_BYTES: u32 = 1u32 << SMP_CACHE_BYTES_SHIFT;

/* C __section(".data..read_mostly") has no direct file-local Rust equivalent. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
