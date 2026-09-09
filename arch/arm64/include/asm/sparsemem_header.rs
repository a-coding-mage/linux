/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency: <asm/pgtable-prot.h>

pub const MAX_PHYSMEM_BITS: usize = PHYS_MASK_SHIFT;
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 52;

/*
 * Section size must be at least 512MB for 64K base
 * page size config. Otherwise it will be less than
 * MAX_PAGE_ORDER and the build process will fail.
 */
#[cfg(feature = "CONFIG_ARM64_64K_PAGES")]
pub const SECTION_SIZE_BITS: usize = 29;

/*
 * Section size must be at least 128MB for 4K base
 * page size config. Otherwise PMD based huge page
 * entries could not be created for vmemmap mappings.
 * 16K follows 4K for simplicity.
 */
#[cfg(not(feature = "CONFIG_ARM64_64K_PAGES"))]
pub const SECTION_SIZE_BITS: usize = 27;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
