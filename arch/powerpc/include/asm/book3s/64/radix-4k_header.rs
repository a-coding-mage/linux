/* SPDX-License-Identifier: GPL-2.0 */

/*
 * For 4K page size supported index is 13/9/9/9
 */
pub const RADIX_PTE_INDEX_SIZE: usize = 9; // size: 8B <<  9 =  4KB, maps 2^9  x    4K =   2MB
pub const RADIX_PMD_INDEX_SIZE: usize = 9; // size: 8B <<  9 =  4KB, maps 2^9  x   2MB =   1GB
pub const RADIX_PUD_INDEX_SIZE: usize = 9; // size: 8B <<  9 =  4KB, maps 2^9  x   1GB = 512GB
pub const RADIX_PGD_INDEX_SIZE: usize = 13; // size: 8B << 13 = 64KB, maps 2^13 x 512GB =   4PB

/*
 * One fragment per page
 */
pub const RADIX_PTE_FRAG_SIZE_SHIFT: usize = RADIX_PTE_INDEX_SIZE + 3;
pub const RADIX_PTE_FRAG_NR: usize = PAGE_SIZE >> RADIX_PTE_FRAG_SIZE_SHIFT;

pub const RADIX_PMD_FRAG_SIZE_SHIFT: usize = RADIX_PMD_INDEX_SIZE + 3;
pub const RADIX_PMD_FRAG_NR: usize = PAGE_SIZE >> RADIX_PMD_FRAG_SIZE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
