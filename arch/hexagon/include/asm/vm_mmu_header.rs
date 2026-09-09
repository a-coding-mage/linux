/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Hexagon VM page table entry definitions
 *
 * Copyright (c) 2010-2011,2013 The Linux Foundation. All rights reserved.
 */

/*
 * Shift, mask, and other constants for the Hexagon Virtual Machine
 * page tables.
 *
 * Virtual machine MMU allows first-level entries to either be
 * single-level lookup PTEs for very large pages, or PDEs pointing
 * to second-level PTEs for smaller pages. If PTE is single-level,
 * the least significant bits cannot be used as software bits to encode
 * virtual memory subsystem information about the page, and that state
 * must be maintained in some parallel data structure.
 */

/* S or Page Size field in PDE */
pub const __HVM_PDE_S: u32 = 0x7 << 0;
pub const __HVM_PDE_S_4KB: u32 = 0;
pub const __HVM_PDE_S_16KB: u32 = 1;
pub const __HVM_PDE_S_64KB: u32 = 2;
pub const __HVM_PDE_S_256KB: u32 = 3;
pub const __HVM_PDE_S_1MB: u32 = 4;
pub const __HVM_PDE_S_4MB: u32 = 5;
pub const __HVM_PDE_S_16MB: u32 = 6;
pub const __HVM_PDE_S_INVALID: u32 = 7;

/* Masks for L2 page table pointer, as function of page size */
pub const __HVM_PDE_PTMASK_4KB: u32 = 0xfffff000;
pub const __HVM_PDE_PTMASK_16KB: u32 = 0xfffffc00;
pub const __HVM_PDE_PTMASK_64KB: u32 = 0xffffff00;
pub const __HVM_PDE_PTMASK_256KB: u32 = 0xffffffc0;
pub const __HVM_PDE_PTMASK_1MB: u32 = 0xfffffff0;

/* Virtual Machine PTE Bits/Fields */
pub const __HVM_PTE_T: u32 = 1 << 4;
pub const __HVM_PTE_U: u32 = 1 << 5;
pub const __HVM_PTE_C: u32 = 0x7 << 6;
#[inline]
pub const fn __HVM_PTE_CVAL(pte: u32) -> u32 {
    (pte & __HVM_PTE_C) >> 6
}
pub const __HVM_PTE_R: u32 = 1 << 9;
pub const __HVM_PTE_W: u32 = 1 << 10;
pub const __HVM_PTE_X: u32 = 1 << 11;

/* Cache Attributes, to be shifted as necessary for virtual/physical PTEs */

pub const __HEXAGON_C_WB: u32 = 0x0; /* Write-back, no L2 */
pub const __HEXAGON_C_WT: u32 = 0x1; /* Write-through, no L2 */
pub const __HEXAGON_C_UNC: u32 = 0x6; /* Uncached memory */
/* CONFIG_HEXAGON_ARCH_VERSION >= 2 selects 0x4; otherwise it selects UNC. */
pub const __HEXAGON_C_DEV: u32 = __HEXAGON_C_UNC;
pub const __HEXAGON_C_WT_L2: u32 = 0x5; /* Write-through, with L2 */
pub const __HEXAGON_C_WB_L2: u32 = 0x7; /* Write-back, with L2 */

/*
 * This can be overridden, but we're defaulting to the most aggressive
 * cache policy, the better to find bugs sooner.
 */

pub const CACHE_DEFAULT: u32 = __HEXAGON_C_WB_L2;

/* Masks for physical page address, as a function of page size */

pub const __HVM_PTE_PGMASK_4KB: u32 = 0xfffff000;
pub const __HVM_PTE_PGMASK_16KB: u32 = 0xffffc000;
pub const __HVM_PTE_PGMASK_64KB: u32 = 0xffff0000;
pub const __HVM_PTE_PGMASK_256KB: u32 = 0xfffc0000;
pub const __HVM_PTE_PGMASK_1MB: u32 = 0xfff00000;

/* Masks for single-level large page lookups */

pub const __HVM_PTE_PGMASK_4MB: u32 = 0xffc00000;
pub const __HVM_PTE_PGMASK_16MB: u32 = 0xff000000;

/*
 * "Big kernel page mappings" (see vm_init_segtable.S)
 * are currently 16MB
 */

pub const BIG_KERNEL_PAGE_SHIFT: u32 = 24;
pub const BIG_KERNEL_PAGE_SIZE: u32 = 1 << BIG_KERNEL_PAGE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
