/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm/vaddrs.h. The C include and header guard are omitted.
// The symbols supplied by asm/head.h and asm/kmap_size.h remain external dependencies.

/*
 * asm/vaddrs.h: Here we define the virtual addresses at
 * which important things will be mapped.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 2000 Anton Blanchard (anton@samba.org)
 */

pub const SRMMU_MAXMEM: usize = 0x0c000000;

pub const SRMMU_NOCACHE_VADDR: usize = KERNBASE + SRMMU_MAXMEM;
// = 0x0fc000000
// XXX Empiricals - this needs to go away - KMW
pub const SRMMU_MIN_NOCACHE_PAGES: usize = 550;
pub const SRMMU_MAX_NOCACHE_PAGES: usize = 1280;

/* The following constant is used in mm/srmmu.c::srmmu_nocache_calcsize()
 * to determine the amount of memory that will be reserved as nocache:
 *
 * 256 pages will be taken as nocache per each
 * SRMMU_NOCACHE_ALCRATIO MB of system memory.
 *
 * limits enforced: nocache minimum = 256 pages
 *                   nocache maximum = 1280 pages
 */
pub const SRMMU_NOCACHE_ALCRATIO: usize = 64; // 256 pages per 64MB of system RAM

/* C condition: !__ASSEMBLER__. */
#[repr(i32)]
pub enum fixed_addresses {
    FIX_HOLE,
    /* C condition: CONFIG_HIGHMEM. */
    FIX_KMAP_BEGIN,
    FIX_KMAP_END = KM_MAX_IDX * NR_CPUS,
    __end_of_fixed_addresses,
}

/* Leave one empty page between IO pages at 0xfd000000 and
 * the top of the fixmap.
 */
pub const FIXADDR_TOP: usize = 0xfcfff000;
pub const FIXADDR_SIZE: usize = (FIX_KMAP_END as usize + 1) << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;

#[inline]
pub const fn __fix_to_virt(x: usize) -> usize {
    FIXADDR_TOP - (x << PAGE_SHIFT)
}

pub const SUN4M_IOBASE_VADDR: usize = 0xfd000000; // Base for mapping pages
pub const IOBASE_VADDR: usize = 0xfe000000;
pub const IOBASE_END: usize = 0xfe600000;

pub const KADB_DEBUGGER_BEGVM: usize = 0xffc00000; // Where kern debugger is in virt-mem
pub const KADB_DEBUGGER_ENDVM: usize = 0xffd00000;
pub const DEBUG_FIRSTVADDR: usize = KADB_DEBUGGER_BEGVM;
pub const DEBUG_LASTVADDR: usize = KADB_DEBUGGER_ENDVM;

pub const LINUX_OPPROM_BEGVM: usize = 0xffd00000;
pub const LINUX_OPPROM_ENDVM: usize = 0xfff00000;

pub const DVMA_VADDR: usize = 0xfff00000; // Base area of the DVMA on suns
pub const DVMA_END: usize = 0xfffc0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
