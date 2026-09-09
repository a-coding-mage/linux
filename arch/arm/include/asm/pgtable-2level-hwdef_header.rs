/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/pgtable-2level-hwdef.h
 *
 *  Copyright (C) 1995-2002 Russell King
 */

/* Hardware page table definitions. */

/* Level 1 descriptor (PMD), common. */
pub const PMD_TYPE_MASK: pmdval_t = (3 as pmdval_t) << 0;
pub const PMD_TYPE_FAULT: pmdval_t = (0 as pmdval_t) << 0;
pub const PMD_TYPE_TABLE: pmdval_t = (1 as pmdval_t) << 0;
pub const PMD_TYPE_SECT: pmdval_t = (2 as pmdval_t) << 0;
pub const PMD_PXNTABLE: pmdval_t = (1 as pmdval_t) << 2; /* v7 */
pub const PMD_BIT4: pmdval_t = (1 as pmdval_t) << 4;
macro_rules! PMD_DOMAIN { ($x:expr) => { ($x as pmdval_t) << 5 }; }
pub const PMD_DOMAIN_MASK: pmdval_t = PMD_DOMAIN!(0x0f);
pub const PMD_PROTECTION: pmdval_t = (1 as pmdval_t) << 9; /* v5 */

/* Level 1 descriptor (PMD), section. */
pub const PMD_SECT_PXN: pmdval_t = (1 as pmdval_t) << 0; /* v7 */
pub const PMD_SECT_BUFFERABLE: pmdval_t = (1 as pmdval_t) << 2;
pub const PMD_SECT_CACHEABLE: pmdval_t = (1 as pmdval_t) << 3;
pub const PMD_SECT_XN: pmdval_t = (1 as pmdval_t) << 4; /* v6 */
pub const PMD_SECT_AP_WRITE: pmdval_t = (1 as pmdval_t) << 10;
pub const PMD_SECT_AP_READ: pmdval_t = (1 as pmdval_t) << 11;
macro_rules! PMD_SECT_TEX { ($x:expr) => { ($x as pmdval_t) << 12 }; } /* v5 */
pub const PMD_SECT_APX: pmdval_t = (1 as pmdval_t) << 15; /* v6 */
pub const PMD_SECT_S: pmdval_t = (1 as pmdval_t) << 16; /* v6 */
pub const PMD_SECT_nG: pmdval_t = (1 as pmdval_t) << 17; /* v6 */
pub const PMD_SECT_SUPER: pmdval_t = (1 as pmdval_t) << 18; /* v6 */
pub const PMD_SECT_AF: pmdval_t = 0 as pmdval_t;

pub const PMD_SECT_UNCACHED: pmdval_t = 0 as pmdval_t;
pub const PMD_SECT_BUFFERED: pmdval_t = PMD_SECT_BUFFERABLE;
pub const PMD_SECT_WT: pmdval_t = PMD_SECT_CACHEABLE;
pub const PMD_SECT_WB: pmdval_t = PMD_SECT_CACHEABLE | PMD_SECT_BUFFERABLE;
pub const PMD_SECT_MINICACHE: pmdval_t = PMD_SECT_TEX!(1) | PMD_SECT_CACHEABLE;
pub const PMD_SECT_WBWA: pmdval_t = PMD_SECT_TEX!(1) | PMD_SECT_CACHEABLE | PMD_SECT_BUFFERABLE;
pub const PMD_SECT_CACHE_MASK: pmdval_t = PMD_SECT_TEX!(1) | PMD_SECT_CACHEABLE | PMD_SECT_BUFFERABLE;
pub const PMD_SECT_NONSHARED_DEV: pmdval_t = PMD_SECT_TEX!(2);

/* Level 1 descriptor, coarse table (not used). */

/* Level 2 descriptor (PTE), common. */
pub const PTE_TYPE_MASK: pteval_t = (3 as pteval_t) << 0;
pub const PTE_TYPE_FAULT: pteval_t = (0 as pteval_t) << 0;
pub const PTE_TYPE_LARGE: pteval_t = (1 as pteval_t) << 0;
pub const PTE_TYPE_SMALL: pteval_t = (2 as pteval_t) << 0;
pub const PTE_TYPE_EXT: pteval_t = (3 as pteval_t) << 0; /* v5 */
pub const PTE_BUFFERABLE: pteval_t = (1 as pteval_t) << 2;
pub const PTE_CACHEABLE: pteval_t = (1 as pteval_t) << 3;

/* Level 2 descriptor, extended small page/tiny page. */
pub const PTE_EXT_XN: pteval_t = (1 as pteval_t) << 0; /* v6 */
pub const PTE_EXT_AP_MASK: pteval_t = (3 as pteval_t) << 4;
pub const PTE_EXT_AP0: pteval_t = (1 as pteval_t) << 4;
pub const PTE_EXT_AP1: pteval_t = (2 as pteval_t) << 4;
pub const PTE_EXT_AP_UNO_SRO: pteval_t = (0 as pteval_t) << 4;
pub const PTE_EXT_AP_UNO_SRW: pteval_t = PTE_EXT_AP0;
pub const PTE_EXT_AP_URO_SRW: pteval_t = PTE_EXT_AP1;
pub const PTE_EXT_AP_URW_SRW: pteval_t = PTE_EXT_AP1 | PTE_EXT_AP0;
macro_rules! PTE_EXT_TEX { ($x:expr) => { ($x as pteval_t) << 6 }; } /* v5 */
pub const PTE_EXT_APX: pteval_t = (1 as pteval_t) << 9; /* v6 */
pub const PTE_EXT_COHERENT: pteval_t = (1 as pteval_t) << 9; /* XScale3 */
pub const PTE_EXT_SHARED: pteval_t = (1 as pteval_t) << 10; /* v6 */
pub const PTE_EXT_NG: pteval_t = (1 as pteval_t) << 11; /* v6 */

/* Level 2 descriptor, small page. */
pub const PTE_SMALL_AP_MASK: pteval_t = (0xff as pteval_t) << 4;
pub const PTE_SMALL_AP_UNO_SRO: pteval_t = (0x00 as pteval_t) << 4;
pub const PTE_SMALL_AP_UNO_SRW: pteval_t = (0x55 as pteval_t) << 4;
pub const PTE_SMALL_AP_URO_SRW: pteval_t = (0xaa as pteval_t) << 4;
pub const PTE_SMALL_AP_URW_SRW: pteval_t = (0xff as pteval_t) << 4;

pub const PHYS_MASK: usize = !0usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
