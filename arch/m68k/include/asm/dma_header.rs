/* SPDX-License-Identifier: GPL-2.0 */

/*
 * It's useless on the m68k, but unfortunately needed by the new
 * bootmem allocator (but this should do it for this).
 */
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
