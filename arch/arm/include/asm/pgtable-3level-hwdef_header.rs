/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from arch/arm/include/asm/pgtable-3level-hwdef.h. */

/* Hardware page table definitions. */

pub const PUD_TABLE_BIT: pmdval_t = (1 as pmdval_t) << 1;
pub const PMD_TYPE_MASK: pmdval_t = (3 as pmdval_t) << 0;
pub const PMD_TYPE_FAULT: pmdval_t = (0 as pmdval_t) << 0;
pub const PMD_TYPE_TABLE: pmdval_t = (3 as pmdval_t) << 0;
pub const PMD_TYPE_SECT: pmdval_t = (1 as pmdval_t) << 0;
pub const PMD_TABLE_BIT: pmdval_t = (1 as pmdval_t) << 1;
pub const PMD_BIT4: pmdval_t = 0 as pmdval_t;
#[macro_export]
macro_rules! PMD_DOMAIN { ($x:expr) => { 0 as pmdval_t }; }
pub const PMD_APTABLE_SHIFT: u32 = 61;
pub const PMD_APTABLE: pgdval_t = (3 as pgdval_t) << PGD_APTABLE_SHIFT;
pub const PMD_PXNTABLE: pgdval_t = (1 as pgdval_t) << 59;

pub const PMD_SECT_BUFFERABLE: pmdval_t = (1 as pmdval_t) << 2;
pub const PMD_SECT_CACHEABLE: pmdval_t = (1 as pmdval_t) << 3;
pub const PMD_SECT_USER: pmdval_t = (1 as pmdval_t) << 6; /* AP[1] */
pub const PMD_SECT_AP2: pmdval_t = (1 as pmdval_t) << 7; /* read only */
pub const PMD_SECT_S: pmdval_t = (3 as pmdval_t) << 8;
pub const PMD_SECT_AF: pmdval_t = (1 as pmdval_t) << 10;
pub const PMD_SECT_nG: pmdval_t = (1 as pmdval_t) << 11;
pub const PMD_SECT_PXN: pmdval_t = (1 as pmdval_t) << 53;
pub const PMD_SECT_XN: pmdval_t = (1 as pmdval_t) << 54;
pub const PMD_SECT_AP_WRITE: pmdval_t = 0 as pmdval_t;
pub const PMD_SECT_AP_READ: pmdval_t = 0 as pmdval_t;
pub const PMD_SECT_AP1: pmdval_t = (1 as pmdval_t) << 6;
#[macro_export]
macro_rules! PMD_SECT_TEX { ($x:expr) => { 0 as pmdval_t }; }

/* AttrIndx[2:0] encoding (mapping attributes defined in the MAIR* registers). */
pub const PMD_SECT_UNCACHED: pmdval_t = (0 as pmdval_t) << 2; /* strongly ordered */
pub const PMD_SECT_BUFFERED: pmdval_t = (1 as pmdval_t) << 2; /* normal non-cacheable */
pub const PMD_SECT_WT: pmdval_t = (2 as pmdval_t) << 2; /* normal inner write-through */
pub const PMD_SECT_WB: pmdval_t = (3 as pmdval_t) << 2; /* normal inner write-back */
pub const PMD_SECT_WBWA: pmdval_t = (7 as pmdval_t) << 2; /* normal inner write-alloc */
pub const PMD_SECT_CACHE_MASK: pmdval_t = (7 as pmdval_t) << 2;

/* Level 3 descriptor (PTE). */
pub const PTE_TYPE_MASK: pteval_t = (3 as pteval_t) << 0;
pub const PTE_TYPE_FAULT: pteval_t = (0 as pteval_t) << 0;
pub const PTE_TYPE_PAGE: pteval_t = (3 as pteval_t) << 0;
pub const PTE_TABLE_BIT: pteval_t = (1 as pteval_t) << 1;
pub const PTE_BUFFERABLE: pteval_t = (1 as pteval_t) << 2; /* AttrIndx[0] */
pub const PTE_CACHEABLE: pteval_t = (1 as pteval_t) << 3; /* AttrIndx[1] */
pub const PTE_AP2: pteval_t = (1 as pteval_t) << 7; /* AP[2] */
pub const PTE_EXT_SHARED: pteval_t = (3 as pteval_t) << 8; /* SH[1:0], inner shareable */
pub const PTE_EXT_AF: pteval_t = (1 as pteval_t) << 10; /* Access Flag */
pub const PTE_EXT_NG: pteval_t = (1 as pteval_t) << 11; /* nG */
pub const PTE_EXT_PXN: pteval_t = (1 as pteval_t) << 53; /* PXN */
pub const PTE_EXT_XN: pteval_t = (1 as pteval_t) << 54; /* XN */

pub const PHYS_MASK_SHIFT: u32 = 40;
pub const PHYS_MASK: u64 = (1u64 << PHYS_MASK_SHIFT) - 1;

/* CONFIG_CPU_TTBR0_PAN selects the alternate TTBR1 definitions. */
#[cfg(not(feature = "CONFIG_CPU_TTBR0_PAN"))]
#[cfg(feature = "CONFIG_VMSPLIT_2G")]
pub const TTBR1_OFFSET: u64 = 16;
#[cfg(not(feature = "CONFIG_CPU_TTBR0_PAN"))]
#[cfg(all(not(feature = "CONFIG_VMSPLIT_2G"), feature = "CONFIG_VMSPLIT_3G"))]
pub const TTBR1_OFFSET: u64 = 4096 * (1 + 3);
#[cfg(not(feature = "CONFIG_CPU_TTBR0_PAN"))]
#[cfg(all(not(feature = "CONFIG_VMSPLIT_2G"), not(feature = "CONFIG_VMSPLIT_3G")))]
pub const TTBR1_OFFSET: u64 = 0;
#[cfg(not(feature = "CONFIG_CPU_TTBR0_PAN"))]
pub const TTBR1_SIZE: u64 = ((PAGE_OFFSET >> 30) - 1) << 16;
#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
pub const TTBR1_OFFSET: u64 = 0;
#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
pub const TTBR1_SIZE: u64 = 0;

pub const TTBCR_EAE: u32 = 1 << 31;
pub const TTBCR_IMP: u32 = 1 << 30;
pub const TTBCR_SH1_MASK: u32 = 3 << 28;
pub const TTBCR_ORGN1_MASK: u32 = 3 << 26;
pub const TTBCR_IRGN1_MASK: u32 = 3 << 24;
pub const TTBCR_EPD1: u32 = 1 << 23;
pub const TTBCR_A1: u32 = 1 << 22;
pub const TTBCR_T1SZ_MASK: u32 = 7 << 16;
pub const TTBCR_SH0_MASK: u32 = 3 << 12;
pub const TTBCR_ORGN0_MASK: u32 = 3 << 10;
pub const TTBCR_IRGN0_MASK: u32 = 3 << 8;
pub const TTBCR_EPD0: u32 = 1 << 7;
pub const TTBCR_T0SZ_MASK: u32 = 7 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
