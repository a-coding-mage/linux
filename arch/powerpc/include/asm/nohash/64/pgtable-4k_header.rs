/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm-generic/pgtable-nop4d.h>

/*
 * Entries per page directory level. The PTE level must use a 64b record
 * for each page table entry. The PMD and PGD level use a 32b record for
 * each entry by assuming that each entry is page aligned.
 */
pub const PTE_INDEX_SIZE: usize = 9;
pub const PMD_INDEX_SIZE: usize = 7;
pub const PUD_INDEX_SIZE: usize = 9;
pub const PGD_INDEX_SIZE: usize = 9;

pub const PTE_TABLE_SIZE: usize = core::mem::size_of::<pte_t>() << PTE_INDEX_SIZE;
pub const PMD_TABLE_SIZE: usize = core::mem::size_of::<pmd_t>() << PMD_INDEX_SIZE;
pub const PUD_TABLE_SIZE: usize = core::mem::size_of::<pud_t>() << PUD_INDEX_SIZE;
pub const PGD_TABLE_SIZE: usize = core::mem::size_of::<pgd_t>() << PGD_INDEX_SIZE;

pub const PTRS_PER_PTE: usize = 1 << PTE_INDEX_SIZE;
pub const PTRS_PER_PMD: usize = 1 << PMD_INDEX_SIZE;
pub const PTRS_PER_PUD: usize = 1 << PUD_INDEX_SIZE;
pub const PTRS_PER_PGD: usize = 1 << PGD_INDEX_SIZE;

/* PMD_SHIFT determines what a second-level page table entry can map */
pub const PMD_SHIFT: usize = PAGE_SHIFT + PTE_INDEX_SIZE;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

/* PUD_SHIFT determines what a third-level page table entry can map */
pub const PUD_SHIFT: usize = PMD_SHIFT + PMD_INDEX_SIZE;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);

/* PGDIR_SHIFT determines what a fourth-level page table entry can map */
pub const PGDIR_SHIFT: usize = PUD_SHIFT + PUD_INDEX_SIZE;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/* Bits to mask out from a PMD to get to the PTE page */
pub const PMD_MASKED_BITS: usize = 0;
/* Bits to mask out from a PUD to get to the PMD page */
pub const PUD_MASKED_BITS: usize = 0;
/* Bits to mask out from a P4D to get to the PUD page */
pub const P4D_MASKED_BITS: usize = 0;

/* 4-level page tables related bits */

#[inline]
pub fn p4d_none(p4d: p4d_t) -> bool {
    p4d_val(p4d) == 0
}

#[inline]
pub fn p4d_bad(p4d: p4d_t) -> bool {
    p4d_val(p4d) == 0
}

#[inline]
pub fn p4d_present(p4d: p4d_t) -> bool {
    p4d_val(p4d) != 0
}

#[inline]
pub unsafe fn p4d_pgtable(p4d: p4d_t) -> *mut pud_t {
    (p4d_val(p4d) & !P4D_MASKED_BITS) as *mut pud_t
}

#[inline]
pub unsafe fn p4d_clear(p4dp: *mut p4d_t) {
    *p4dp = __p4d(0);
}

#[inline]
pub fn p4d_pte(p4d: p4d_t) -> pte_t {
    __pte(p4d_val(p4d))
}

#[inline]
pub fn pte_p4d(pte: pte_t) -> p4d_t {
    __p4d(pte_val(pte))
}

extern "C" {
    pub fn p4d_page(p4d: p4d_t) -> *mut page;
}

#[macro_export]
macro_rules! pud_ERROR {
    ($e:expr) => {
        pr_err!("{}:{}: bad pud {:08lx}.\n", file!(), line!(), pud_val($e))
    };
}

/* On all 4K setups, remap_4k_pfn() equates to remap_pfn_range() */
#[macro_export]
macro_rules! remap_4k_pfn {
    ($vma:expr, $addr:expr, $pfn:expr, $prot:expr) => {
        remap_pfn_range!($vma, $addr, $pfn, PAGE_SIZE, $prot)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
