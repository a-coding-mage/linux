/* SPDX-License-Identifier: GPL-2.0 */

/*
 * We have these possible memory map layouts:
 * Astro: 0-3.75, 67.75-68, 4-64
 * zx1: 0-1, 257-260, 4-256
 * Stretch (N-class): 0-2, 4-32, 34-xxx
 */

pub const MAX_PHYSMEM_BITS: u32 = 39; /* 512 GB */
pub const SECTION_SIZE_BITS: u32 = 27; /* 128 MB */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
