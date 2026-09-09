/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_SPARSEMEM */

/*
 * SECTION_SIZE_BITS        2^N: how big each section will be
 * MAX_PHYSMEM_BITS        2^N: how much memory we can have in that space
 */

/*
 * Source condition:
 * defined(CONFIG_MIPS_HUGE_TLB_SUPPORT) && defined(CONFIG_PAGE_SIZE_64KB)
 */
#[cfg(all(feature = "CONFIG_MIPS_HUGE_TLB_SUPPORT", feature = "CONFIG_PAGE_SIZE_64KB"))]
pub const SECTION_SIZE_BITS: u32 = 29;

#[cfg(not(all(feature = "CONFIG_MIPS_HUGE_TLB_SUPPORT", feature = "CONFIG_PAGE_SIZE_64KB")))]
pub const SECTION_SIZE_BITS: u32 = 28;

pub const MAX_PHYSMEM_BITS: u32 = 48;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
