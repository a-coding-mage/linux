/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header is active when CONFIG_SPARSEMEM is enabled.
 *
 * SECTION_SIZE_BITS  2^N: how big each section will be
 * MAX_PHYSMEM_BITS   2^N: how much memory we can have in that space
 */
#[cfg(feature = "CONFIG_SPARSEMEM")]
pub const SECTION_SIZE_BITS: usize = 29; /* 2^29 = Largest Huge Page Size */

#[cfg(feature = "CONFIG_SPARSEMEM")]
pub const MAX_PHYSMEM_BITS: usize = 48;

/* CONFIG_SPARSEMEM_VMEMMAP supplies the VMEMMAP_SIZE definition. */
#[cfg(all(feature = "CONFIG_SPARSEMEM", feature = "CONFIG_SPARSEMEM_VMEMMAP"))]
pub const VMEMMAP_SIZE: usize =
    core::mem::size_of::<Page>() * (1usize << (cpu_pabits + 1 - PAGE_SHIFT));

/* 1, For FLATMEM; 2, For SPARSEMEM without VMEMMAP. */
#[cfg(not(all(feature = "CONFIG_SPARSEMEM", feature = "CONFIG_SPARSEMEM_VMEMMAP")))]
pub const VMEMMAP_SIZE: usize = 0;

pub const INIT_MEMBLOCK_RESERVED_REGIONS: usize = INIT_MEMBLOCK_REGIONS + NR_CPUS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
