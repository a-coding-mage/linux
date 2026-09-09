/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Synopsys, Inc. (www.synopsys.com)
 */

/* Helpers for implementing paging levels. */

/* Build-time conditional: CONFIG_PGTABLE_LEVELS == 2. */
#[cfg(CONFIG_PGTABLE_LEVELS = "2")]
pub const PGDIR_SHIFT: u32 = if cfg!(CONFIG_ARC_HUGEPAGE_16M) {
    24
} else if cfg!(CONFIG_ARC_HUGEPAGE_2M) {
    21
} else if cfg!(CONFIG_ARC_PAGE_SIZE_4K) {
    22
} else {
    21
};

/* Build-time conditional: CONFIG_PGTABLE_LEVELS != 2. */
#[cfg(not(CONFIG_PGTABLE_LEVELS = "2"))]
pub const PGDIR_SHIFT: u32 = 28;

/* These constants depend on the external BIT and PAGE_SHIFT definitions. */
pub const PGDIR_SIZE: u64 = BIT(PGDIR_SHIFT);
pub const PGDIR_MASK: u64 = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PGD: u64 = BIT(32 - PGDIR_SHIFT);

#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4"))]
pub const PMD_SHIFT: u32 = 21;

#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_SHIFT: u32 = 25;

#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_SIZE: u64 = BIT(PUD_SHIFT);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PUD_MASK: u64 = !(PUD_SIZE - 1);
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
pub const PTRS_PER_PUD: u64 = BIT(PGDIR_SHIFT - PUD_SHIFT);

#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4"))]
pub const PMD_SIZE: u64 = BIT(PMD_SHIFT);
#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4"))]
pub const PMD_MASK: u64 = !(PMD_SIZE - 1);
#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4"))]
pub const PTRS_PER_PMD: u64 = BIT(PUD_SHIFT - PMD_SHIFT);

pub const PTRS_PER_PTE: u64 = BIT(PMD_SHIFT - PAGE_SHIFT);

/* The following declarations mirror the C macros and use external kernel symbols. */
#[macro_export]
macro_rules! pgd_ERROR { ($e:expr) => { pr_crit!("%s:%d: bad pgd %08lx.\\n", file!(), line!(), pgd_val($e)); }; }

#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_none { ($x:expr) => { !p4d_val($x) }; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_bad { ($x:expr) => { p4d_val($x) & !PAGE_MASK }; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_present { ($x:expr) => { p4d_val($x) }; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_clear { ($xp:expr) => {{ p4d_val(*$xp) = 0; }}; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_pgtable { ($p4d:expr) => { (p4d_val($p4d) & PAGE_MASK) as *mut pud_t }; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! p4d_page { ($p4d:expr) => { virt_to_page(p4d_pgtable!($p4d)) }; }
#[cfg(CONFIG_PGTABLE_LEVELS = "4")]
#[macro_export]
macro_rules! set_p4d { ($p4dp:expr, $p4d:expr) => {{ *$p4dp = $p4d; }}; }

#[macro_export]
macro_rules! pud_none { ($x:expr) => { !pud_val($x) }; }
#[macro_export]
macro_rules! pud_bad { ($x:expr) => { pud_val($x) & !PAGE_MASK }; }
#[macro_export]
macro_rules! pud_present { ($x:expr) => { pud_val($x) }; }
#[macro_export]
macro_rules! pud_clear { ($xp:expr) => {{ pud_val(*$xp) = 0; }}; }
#[macro_export]
macro_rules! pud_pgtable { ($pud:expr) => { (pud_val($pud) & PAGE_MASK) as *mut pmd_t }; }
#[macro_export]
macro_rules! pud_page { ($pud:expr) => { virt_to_page(pud_pgtable!($pud)) }; }
#[macro_export]
macro_rules! set_pud { ($pudp:expr, $pud:expr) => {{ *$pudp = $pud; }}; }

#[macro_export]
macro_rules! pmd_none { ($x:expr) => { !pmd_val($x) }; }
#[macro_export]
macro_rules! pmd_bad { ($x:expr) => { pmd_val($x) & !PAGE_MASK }; }
#[macro_export]
macro_rules! pmd_present { ($x:expr) => { pmd_val($x) }; }
#[macro_export]
macro_rules! pmd_clear { ($xp:expr) => {{ pmd_val(*$xp) = 0; }}; }
#[macro_export]
macro_rules! pmd_page_vaddr { ($pmd:expr) => { pmd_val($pmd) & PAGE_MASK }; }
#[macro_export]
macro_rules! pmd_pfn { ($pmd:expr) => { (pmd_val($pmd) & PAGE_MASK) >> PAGE_SHIFT }; }
#[macro_export]
macro_rules! pmd_page { ($pmd:expr) => { virt_to_page(pmd_page_vaddr!($pmd) as *mut core::ffi::c_void) }; }
#[macro_export]
macro_rules! set_pmd { ($pmdp:expr, $pmd:expr) => {{ *$pmdp = $pmd; }}; }
#[macro_export]
macro_rules! pmd_pgtable { ($pmd:expr) => { pmd_page!($pmd) as pgtable_t }; }

pub const PFN_PTE_SHIFT: u32 = PAGE_SHIFT;
#[macro_export]
macro_rules! pte_none { ($x:expr) => { !pte_val($x) }; }
#[macro_export]
macro_rules! pte_present { ($x:expr) => { pte_val($x) & _PAGE_PRESENT }; }
#[macro_export]
macro_rules! pte_clear { ($mm:expr, $addr:expr, $ptep:expr) => { set_pte_at($mm, $addr, $ptep, __pte(0)); }; }
#[macro_export]
macro_rules! pte_page { ($pte:expr) => { pfn_to_page(pte_pfn!($pte)) }; }
#[macro_export]
macro_rules! set_pte { ($ptep:expr, $pte:expr) => {{ *$ptep = $pte; }}; }
#[macro_export]
macro_rules! pte_pfn { ($pte:expr) => { pte_val($pte) >> PAGE_SHIFT }; }
#[macro_export]
macro_rules! pfn_pte { ($pfn:expr, $prot:expr) => { __pte(__pfn_to_phys($pfn) | pgprot_val($prot)) }; }

#[cfg(CONFIG_ISA_ARCV2)]
#[macro_export]
macro_rules! pmd_leaf { ($x:expr) => { pmd_val($x) & _PAGE_HW_SZ }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
