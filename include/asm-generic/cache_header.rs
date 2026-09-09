/* SPDX-License-Identifier: GPL-2.0 */
/*
 * 32 bytes appears to be the most common cache line size,
 * so make that the default here. Architectures with larger
 * cache lines need to provide their own cache.h.
 */

pub const L1_CACHE_SHIFT: usize = 5;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
