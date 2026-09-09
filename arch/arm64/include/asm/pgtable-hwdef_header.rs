/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 ARM Ltd. */
/* Translated from asm/pgtable-hwdef.h; asm/memory.h supplies dependent symbols. */

pub const PTDESC_ORDER: usize = 3;
pub const PTDESC_TABLE_SHIFT: usize = PAGE_SHIFT - PTDESC_ORDER;

#[inline]
pub const fn arm64_hw_pgtable_levels(va_bits: usize) -> usize {
    (va_bits - PTDESC_ORDER - 1) / PTDESC_TABLE_SHIFT
}

#[inline]
pub const fn arm64_hw_pgtable_level_shift(n: usize) -> usize {
    PTDESC_TABLE_SHIFT * (4 - n) + PTDESC_ORDER
}

pub const PTRS_PER_PTE: usize = 1usize << PTDESC_TABLE_SHIFT;

/* CONFIG_PGTABLE_LEVELS > 2 */
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
pub const PMD_SHIFT: usize = arm64_hw_pgtable_level_shift(2);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_2")]
pub const PTRS_PER_PMD: usize = 1usize << PTDESC_TABLE_SHIFT;

/* CONFIG_PGTABLE_LEVELS > 3 */
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
pub const PUD_SHIFT: usize = arm64_hw_pgtable_level_shift(1);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_3")]
pub const PTRS_PER_PUD: usize = 1usize << PTDESC_TABLE_SHIFT;

/* CONFIG_PGTABLE_LEVELS > 4 */
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_4")]
pub const P4D_SHIFT: usize = arm64_hw_pgtable_level_shift(0);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_4")]
pub const P4D_SIZE: usize = 1usize << P4D_SHIFT;
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_4")]
pub const P4D_MASK: usize = !(P4D_SIZE - 1);
#[cfg(feature = "CONFIG_PGTABLE_LEVELS_GT_4")]
pub const PTRS_PER_P4D: usize = 1usize << PTDESC_TABLE_SHIFT;

pub const PGDIR_SHIFT: usize = arm64_hw_pgtable_level_shift(4 - CONFIG_PGTABLE_LEVELS);
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PGD: usize = 1usize << (VA_BITS - PGDIR_SHIFT);

pub const CONT_PTE_SHIFT: usize = CONFIG_ARM64_CONT_PTE_SHIFT + PAGE_SHIFT;
pub const CONT_PTES: usize = 1usize << (CONT_PTE_SHIFT - PAGE_SHIFT);
pub const CONT_PTE_SIZE: usize = CONT_PTES * PAGE_SIZE;
pub const CONT_PTE_MASK: usize = !(CONT_PTE_SIZE - 1);
pub const CONT_PMD_SHIFT: usize = CONFIG_ARM64_CONT_PMD_SHIFT + PMD_SHIFT;
pub const CONT_PMDS: usize = 1usize << (CONT_PMD_SHIFT - PMD_SHIFT);
pub const CONT_PMD_SIZE: usize = CONT_PMDS * PMD_SIZE;
pub const CONT_PMD_MASK: usize = !(CONT_PMD_SIZE - 1);

pub const PGD_TYPE_TABLE: pgdval_t = (3 as pgdval_t) << 0;
pub const PGD_TYPE_MASK: pgdval_t = (3 as pgdval_t) << 0;
pub const PGD_TABLE_AF: pgdval_t = (1 as pgdval_t) << 10;
pub const PGD_TABLE_PXN: pgdval_t = (1 as pgdval_t) << 59;
pub const PGD_TABLE_UXN: pgdval_t = (1 as pgdval_t) << 60;

pub const P4D_TYPE_TABLE: p4dval_t = (3 as p4dval_t) << 0;
pub const P4D_TYPE_MASK: p4dval_t = (3 as p4dval_t) << 0;
pub const P4D_TYPE_SECT: p4dval_t = (1 as p4dval_t) << 0;
pub const P4D_SECT_RDONLY: p4dval_t = (1 as p4dval_t) << 7;
pub const P4D_TABLE_AF: p4dval_t = (1 as p4dval_t) << 10;
pub const P4D_TABLE_PXN: p4dval_t = (1 as p4dval_t) << 59;
pub const P4D_TABLE_UXN: p4dval_t = (1 as p4dval_t) << 60;

pub const PUD_TYPE_TABLE: pudval_t = (3 as pudval_t) << 0;
pub const PUD_TYPE_MASK: pudval_t = (3 as pudval_t) << 0;
pub const PUD_TYPE_SECT: pudval_t = (1 as pudval_t) << 0;
pub const PUD_SECT_RDONLY: pudval_t = (1 as pudval_t) << 7;
pub const PUD_TABLE_AF: pudval_t = (1 as pudval_t) << 10;
pub const PUD_TABLE_PXN: pudval_t = (1 as pudval_t) << 59;
pub const PUD_TABLE_UXN: pudval_t = (1 as pudval_t) << 60;

pub const PMD_TYPE_MASK: pmdval_t = (3 as pmdval_t) << 0;
pub const PMD_TYPE_TABLE: pmdval_t = (3 as pmdval_t) << 0;
pub const PMD_TYPE_SECT: pmdval_t = (1 as pmdval_t) << 0;
pub const PMD_TABLE_AF: pmdval_t = (1 as pmdval_t) << 10;
pub const PMD_SECT_USER: pmdval_t = (1 as pmdval_t) << 6;
pub const PMD_SECT_RDONLY: pmdval_t = (1 as pmdval_t) << 7;
pub const PMD_SECT_S: pmdval_t = (3 as pmdval_t) << 8;
pub const PMD_SECT_AF: pmdval_t = (1 as pmdval_t) << 10;
pub const PMD_SECT_NG: pmdval_t = (1 as pmdval_t) << 11;
pub const PMD_SECT_CONT: pmdval_t = (1 as pmdval_t) << 52;
pub const PMD_SECT_PXN: pmdval_t = (1 as pmdval_t) << 53;
pub const PMD_SECT_UXN: pmdval_t = (1 as pmdval_t) << 54;
pub const PMD_TABLE_PXN: pmdval_t = (1 as pmdval_t) << 59;
pub const PMD_TABLE_UXN: pmdval_t = (1 as pmdval_t) << 60;

#[macro_export]
macro_rules! PMD_ATTRINDX { ($t:expr) => { (($t as pmdval_t) << 2) }; }
pub const PMD_ATTRINDX_MASK: pmdval_t = (7 as pmdval_t) << 2;

pub const PTE_VALID: pteval_t = (1 as pteval_t) << 0;
pub const PTE_TYPE_MASK: pteval_t = (3 as pteval_t) << 0;
pub const PTE_TYPE_PAGE: pteval_t = (3 as pteval_t) << 0;
pub const PTE_USER: pteval_t = (1 as pteval_t) << 6;
pub const PTE_RDONLY: pteval_t = (1 as pteval_t) << 7;
pub const PTE_SHARED: pteval_t = (3 as pteval_t) << 8;
pub const PTE_AF: pteval_t = (1 as pteval_t) << 10;
pub const PTE_NG: pteval_t = (1 as pteval_t) << 11;
pub const PTE_GP: pteval_t = (1 as pteval_t) << 50;
pub const PTE_DBM: pteval_t = (1 as pteval_t) << 51;
pub const PTE_CONT: pteval_t = (1 as pteval_t) << 52;
pub const PTE_PXN: pteval_t = (1 as pteval_t) << 53;
pub const PTE_UXN: pteval_t = (1 as pteval_t) << 54;
pub const PTE_SWBITS_MASK: pteval_t = (1u64 << 63 | (((1u64 << 4) - 1) << 55)) as pteval_t;
pub const PTE_ADDR_LOW: pteval_t = (((1u64 << (50 - PAGE_SHIFT)) - 1) << PAGE_SHIFT) as pteval_t;

pub const PTE_PI_IDX_0: usize = 6;
pub const PTE_PI_IDX_1: usize = 51;
pub const PTE_PI_IDX_2: usize = 53;
pub const PTE_PI_IDX_3: usize = 54;
pub const PTE_PO_IDX_0: pteval_t = (1 as pteval_t) << 60;
pub const PTE_PO_IDX_1: pteval_t = (1 as pteval_t) << 61;
pub const PTE_PO_IDX_2: pteval_t = (1 as pteval_t) << 62;
pub const PTE_PO_IDX_MASK: pteval_t = (0x7u64 << 60) as pteval_t;

#[macro_export]
macro_rules! PTE_ATTRINDX { ($t:expr) => { (($t as pteval_t) << 2) }; }
pub const PTE_ATTRINDX_MASK: pteval_t = (7 as pteval_t) << 2;
#[macro_export]
macro_rules! PTE_S2_MEMATTR { ($t:expr) => { (($t as pteval_t) << 2) }; }
pub const S1_TABLE_AP: pmdval_t = (3 as pmdval_t) << 61;

#[macro_export]
macro_rules! TCR_T0SZ { ($x:expr) => { ((64usize - ($x)) << TCR_EL1_T0SZ_SHIFT) }; }
#[macro_export]
macro_rules! TCR_T1SZ { ($x:expr) => { ((64usize - ($x)) << TCR_EL1_T1SZ_SHIFT) }; }
pub const TCR_T0SZ_MASK: usize = TCR_EL1_T0SZ_MASK;
pub const TCR_T1SZ_MASK: usize = TCR_EL1_T1SZ_MASK;
pub const TCR_EPD0_MASK: usize = TCR_EL1_EPD0_MASK;
pub const TCR_EPD1_MASK: usize = TCR_EL1_EPD1_MASK;
pub const TCR_IRGN0_MASK: usize = TCR_EL1_IRGN0_MASK;
pub const TCR_IRGN0_WBWA: usize = TCR_EL1_IRGN0_WBWA << TCR_EL1_IRGN0_SHIFT;
pub const TCR_ORGN0_MASK: usize = TCR_EL1_ORGN0_MASK;
pub const TCR_ORGN0_WBWA: usize = TCR_EL1_ORGN0_WBWA << TCR_EL1_ORGN0_SHIFT;
pub const TCR_SH0_MASK: usize = TCR_EL1_SH0_MASK;
pub const TCR_SH0_INNER: usize = TCR_EL1_SH0_INNER << TCR_EL1_SH0_SHIFT;
pub const TCR_SH1_MASK: usize = TCR_EL1_SH1_MASK;
pub const TCR_TG0_SHIFT: usize = TCR_EL1_TG0_SHIFT;
pub const TCR_TG0_MASK: usize = TCR_EL1_TG0_MASK;
pub const TCR_TG0_4K: usize = TCR_EL1_TG0_4K << TCR_EL1_TG0_SHIFT;
pub const TCR_TG0_64K: usize = TCR_EL1_TG0_64K << TCR_EL1_TG0_SHIFT;
pub const TCR_TG0_16K: usize = TCR_EL1_TG0_16K << TCR_EL1_TG0_SHIFT;
pub const TCR_TG1_SHIFT: usize = TCR_EL1_TG1_SHIFT;
pub const TCR_TG1_MASK: usize = TCR_EL1_TG1_MASK;
pub const TCR_TG1_16K: usize = TCR_EL1_TG1_16K << TCR_EL1_TG1_SHIFT;
pub const TCR_TG1_4K: usize = TCR_EL1_TG1_4K << TCR_EL1_TG1_SHIFT;
pub const TCR_TG1_64K: usize = TCR_EL1_TG1_64K << TCR_EL1_TG1_SHIFT;
pub const TCR_IPS_SHIFT: usize = TCR_EL1_IPS_SHIFT;
pub const TCR_IPS_MASK: usize = TCR_EL1_IPS_MASK;
pub const TCR_A1: usize = TCR_EL1_A1;
pub const TCR_ASID16: usize = TCR_EL1_AS;
pub const TCR_TBI0: usize = TCR_EL1_TBI0;
pub const TCR_TBI1: usize = TCR_EL1_TBI1;
pub const TCR_HA: usize = TCR_EL1_HA;
pub const TCR_HD: usize = TCR_EL1_HD;
pub const TCR_HPD0: usize = TCR_EL1_HPD0;
pub const TCR_HPD1: usize = TCR_EL1_HPD1;
pub const TCR_TBID0: usize = TCR_EL1_TBID0;
pub const TCR_TBID1: usize = TCR_EL1_TBID1;
pub const TCR_E0PD0: usize = TCR_EL1_E0PD0;
pub const TCR_E0PD1: usize = TCR_EL1_E0PD1;
pub const TCR_DS: usize = TCR_EL1_DS;

/* CONFIG_ARM64_PA_BITS_52: TTBR_ELx[1] is RES0. */
#[cfg(feature = "CONFIG_ARM64_PA_BITS_52")]
pub const TTBR_BADDR_MASK_52: u64 = 0x0000_ffff_ffff_fffc;
/* CONFIG_ARM64_VA_BITS_52 */
#[cfg(feature = "CONFIG_ARM64_VA_BITS_52")]
pub const PTRS_PER_PGD_52_VA: usize = 1usize << (52 - PGDIR_SHIFT);
#[cfg(feature = "CONFIG_ARM64_VA_BITS_52")]
pub const PTRS_PER_PGD_48_VA: usize = 1usize << (48 - PGDIR_SHIFT);
#[cfg(feature = "CONFIG_ARM64_VA_BITS_52")]
pub const PTRS_PER_PGD_EXTRA: usize = PTRS_PER_PGD_52_VA - PTRS_PER_PGD_48_VA;
#[cfg(feature = "CONFIG_ARM64_VA_BITS_52")]
pub const TTBR1_BADDR_4852_OFFSET: usize = PTRS_PER_PGD_EXTRA << PTDESC_ORDER;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
