/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by linux/mm.h, linux/mmzone.h, and asm-generic/pgalloc.h

/*
 * Allocate and free page tables. The xxx_kernel() versions are
 * used to allocate a kernel page table - this turns on ASN bits
 * if any.
 */

#[inline]
pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    pmd_set(
        pmd,
        (page_to_pa(pte).wrapping_add(PAGE_OFFSET)) as *mut pte_t,
    );
}

#[inline]
pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    pmd_set(pmd, pte);
}

#[inline]
pub unsafe fn pud_populate(
    mm: *mut mm_struct,
    pud: *mut pud_t,
    pmd: *mut pmd_t,
) {
    pud_set(pud, pmd);
}

extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
