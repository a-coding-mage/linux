/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-m68k/cache.h
 */

/* bytes per L1 cache line */
pub const L1_CACHE_SHIFT: usize = 4;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
