/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/pgtable-2level.h
 *
 *  Copyright (C) 1995-2002 Russell King
 */

pub const __PAGETABLE_PMD_FOLDED: usize = 1;

/* Two-level ARM page-table layout and bit definitions. */
pub const PTRS_PER_PTE: usize = 512;
pub const PTRS_PER_PMD: usize = 1;
pub const PTRS_PER_PGD: usize = 2048;

pub const PTE_HWTABLE_PTRS: usize = PTRS_PER_PTE;
pub const PTE_HWTABLE_OFF: usize = PTE_HWTABLE_PTRS * core::mem::size_of::<pte_t>();
pub const PTE_HWTABLE_SIZE: usize = PTRS_PER_PTE * core::mem::size_of::<u32>();
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 32;

pub const PMD_SHIFT: usize = 21;
pub const PGDIR_SHIFT: usize = 21;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

pub const SECTION_SHIFT: usize = 20;
pub const SECTION_SIZE: usize = 1usize << SECTION_SHIFT;
pub const SECTION_MASK: usize = !(SECTION_SIZE - 1);

pub const SUPERSECTION_SHIFT: usize = 24;
pub const SUPERSECTION_SIZE: usize = 1usize << SUPERSECTION_SHIFT;
pub const SUPERSECTION_MASK: usize = !(SUPERSECTION_SIZE - 1);

pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;

/* Linux PTE definitions. */
pub const L_PTE_VALID: pteval_t = (1 as pteval_t) << 0;
pub const L_PTE_PRESENT: pteval_t = (1 as pteval_t) << 0;
pub const L_PTE_YOUNG: pteval_t = (1 as pteval_t) << 1;
pub const L_PTE_DIRTY: pteval_t = (1 as pteval_t) << 6;
pub const L_PTE_RDONLY: pteval_t = (1 as pteval_t) << 7;
pub const L_PTE_USER: pteval_t = (1 as pteval_t) << 8;
pub const L_PTE_XN: pteval_t = (1 as pteval_t) << 9;
pub const L_PTE_SHARED: pteval_t = (1 as pteval_t) << 10;
pub const L_PTE_NONE: pteval_t = (1 as pteval_t) << 11;
pub const L_PTE_SWP_EXCLUSIVE: pteval_t = L_PTE_RDONLY;

pub const L_PTE_MT_UNCACHED: pteval_t = (0x00 as pteval_t) << 2;
pub const L_PTE_MT_BUFFERABLE: pteval_t = (0x01 as pteval_t) << 2;
pub const L_PTE_MT_WRITETHROUGH: pteval_t = (0x02 as pteval_t) << 2;
pub const L_PTE_MT_WRITEBACK: pteval_t = (0x03 as pteval_t) << 2;
pub const L_PTE_MT_MINICACHE: pteval_t = (0x06 as pteval_t) << 2;
pub const L_PTE_MT_WRITEALLOC: pteval_t = (0x07 as pteval_t) << 2;
pub const L_PTE_MT_DEV_SHARED: pteval_t = (0x04 as pteval_t) << 2;
pub const L_PTE_MT_DEV_NONSHARED: pteval_t = (0x0c as pteval_t) << 2;
pub const L_PTE_MT_DEV_WC: pteval_t = (0x09 as pteval_t) << 2;
pub const L_PTE_MT_DEV_CACHED: pteval_t = (0x0b as pteval_t) << 2;
pub const L_PTE_MT_VECTORS: pteval_t = (0x0f as pteval_t) << 2;
pub const L_PTE_MT_MASK: pteval_t = (0x0f as pteval_t) << 2;

/* The following items correspond to the non-assembly inline definitions. */
#[inline]
pub fn pud_none(_pud: pud_t) -> i32 { 0 }

#[inline]
pub fn pud_bad(_pud: pud_t) -> i32 { 0 }

#[inline]
pub fn pud_present(_pud: pud_t) -> i32 { 1 }

#[inline]
pub fn pud_clear(_pudp: *mut pud_t) {}

#[inline]
pub fn set_pud(_pudp: *mut pud_t, _pud: pud_t) {}

#[inline]
pub unsafe fn pmd_offset(pud: *mut pud_t, _addr: c_ulong) -> *mut pmd_t {
    pud as *mut pmd_t
}

/* pmd_pfn(pmd) = __phys_to_pfn(pmd_val(pmd) & PHYS_MASK). */
#[inline]
pub fn pmd_pfn(pmd: pmd_t) -> impl Copy {
    __phys_to_pfn(pmd_val(pmd) & PHYS_MASK)
}

#[inline]
pub fn pmd_leaf(pmd: pmd_t) -> impl Copy { pmd_val(pmd) & PMD_TYPE_SECT }

#[inline]
pub fn pmd_bad(pmd: pmd_t) -> impl Copy { pmd_leaf(pmd) }

#[inline]
pub fn pmd_present(pmd: pmd_t) -> impl Copy { pmd_val(pmd) }

#[inline]
pub unsafe fn copy_pmd(pmdpd: *mut pmd_t, pmdps: *const pmd_t) {
    *pmdpd.add(0) = *pmdps.add(0);
    *pmdpd.add(1) = *pmdps.add(1);
    flush_pmd_entry(pmdpd);
}

#[inline]
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) {
    *pmdp.add(0) = __pmd(0);
    *pmdp.add(1) = __pmd(0);
    clean_pmd_entry(pmdp);
}

#[inline]
pub fn pmd_addr_end(_addr: c_ulong, end: c_ulong) -> c_ulong { end }

#[inline]
pub fn set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: c_ulong) {
    cpu_set_pte_ext(ptep, pte, ext)
}

#[inline]
pub const fn pmd_hugewillfault(_pmd: pmd_t) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
