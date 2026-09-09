/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 */

/* The declarations below are present only when CONFIG_HIGHMEM is enabled. */
#[cfg(CONFIG_HIGHMEM)]
pub const FIX_KMAP_BEGIN: usize = 0;

#[cfg(CONFIG_HIGHMEM)]
pub const FIX_KMAP_END: usize =
    FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS * DCACHE_N_COLORS) - 1;

#[cfg(CONFIG_HIGHMEM)]
pub const __end_of_fixed_addresses: usize = FIX_KMAP_END + 1;

#[cfg(CONFIG_HIGHMEM)]
pub const FIXADDR_END: usize = XCHAL_KSEG_CACHED_VADDR - PAGE_SIZE;

#[cfg(CONFIG_HIGHMEM)]
pub const FIXADDR_SIZE: usize = __end_of_fixed_addresses << PAGE_SHIFT;

/* Enforce that FIXADDR_START is PMD aligned to handle cache aliasing. */
#[cfg(CONFIG_HIGHMEM)]
pub const FIXADDR_START: usize = (FIXADDR_END - FIXADDR_SIZE) & PMD_MASK;

#[cfg(CONFIG_HIGHMEM)]
pub const FIXADDR_TOP: usize = FIXADDR_START + FIXADDR_SIZE - PAGE_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
