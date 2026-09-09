/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(CONFIG_MMU)]
/*
 * Here we define all the compile-time 'special' virtual addresses.
 * The point is to have a constant address at compile time, but to
 * set the physical address only in the boot process.
 *
 * These 'compile-time allocated' memory buffers are page-sized. Use
 * set_fixmap(idx,phys) to associate physical memory with fixmap indices.
 */
#[repr(C)]
pub enum fixed_addresses {
    FIX_HOLE,
    /*
     * The fdt fixmap mapping must be PMD aligned and will be mapped
     * using PMD entries in fixmap_pmd in 64-bit and a PGD entry in 32-bit.
     */
    FIX_FDT_END,
    FIX_FDT = FIX_FDT_END as isize + (FIX_FDT_SIZE / PAGE_SIZE) as isize - 1,

    /* Below fixmaps will be mapped using fixmap_pte */
    FIX_PTE,
    FIX_PMD,
    FIX_PUD,
    FIX_P4D,
    FIX_TEXT_POKE1,
    FIX_TEXT_POKE0,
    FIX_EARLYCON_MEM_BASE,

    __end_of_permanent_fixed_addresses,
    /*
     * Temporary boot-time mappings, used by early_ioremap(),
     * before ioremap() is functional.
     */
    FIX_BTMAP_END = __end_of_permanent_fixed_addresses as isize,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END as isize + TOTAL_FIX_BTMAPS as isize - 1,

    __end_of_fixed_addresses,
}

pub const NR_FIX_BTMAPS: usize = SZ_256K / PAGE_SIZE;
pub const FIX_BTMAPS_SLOTS: usize = 7;
pub const TOTAL_FIX_BTMAPS: usize = NR_FIX_BTMAPS * FIX_BTMAPS_SLOTS;

extern "C" {
    pub fn __set_fixmap(
        idx: fixed_addresses,
        phys: phys_addr_t,
        prot: pgprot_t,
    );
}

/* The C macro aliases __early_set_fixmap to __set_fixmap. */
pub use __set_fixmap as __early_set_fixmap;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
