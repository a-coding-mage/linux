/* SPDX-License-Identifier: GPL-2.0 */

/* The default of 128 bytes wastes too much, use 32 (the largest cacheline, I) */
pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
