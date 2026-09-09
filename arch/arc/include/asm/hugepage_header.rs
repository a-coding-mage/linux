/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013-15 Synopsys, Inc. (www.synopsys.com)
 */

/* Hugetlb definitions. */
pub const HPAGE_SHIFT: usize = PMD_SHIFT;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);

pub unsafe fn pmd_pte(pmd: pmd_t) -> pte_t {
    __pte(pmd_val(pmd))
}

pub unsafe fn pte_pmd(pte: pte_t) -> pmd_t {
    __pmd(pte_val(pte))
}

pub unsafe fn pmd_wrprotect(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_wrprotect(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkwrite_novma(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkwrite_novma(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkdirty(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkdirty(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkold(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkold(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkyoung(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkyoung(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkhuge(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkhuge(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkinvalid(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mknotpresent(pmd_pte(pmd)))
}

pub unsafe fn pmd_mkclean(pmd: pmd_t) -> pmd_t {
    pte_pmd(pte_mkclean(pmd_pte(pmd)))
}

pub unsafe fn pmd_write(pmd: pmd_t) -> bool {
    pte_write(pmd_pte(pmd))
}

pub unsafe fn pmd_young(pmd: pmd_t) -> bool {
    pte_young(pmd_pte(pmd))
}

pub unsafe fn pmd_dirty(pmd: pmd_t) -> bool {
    pte_dirty(pmd_pte(pmd))
}

pub unsafe fn pmd_trans_huge(pmd: pmd_t) -> bool {
    (pmd_val(pmd) & _PAGE_HW_SZ) != 0
}

pub unsafe fn pfn_pmd(pfn: unsigned_long, prot: pgprot_t) -> pmd_t {
    __pmd((pfn << PAGE_SHIFT) | pgprot_val(prot))
}

pub unsafe fn pmd_modify(pmd: pmd_t, newprot: pgprot_t) -> pmd_t {
    /*
     * Open-coded pte_modify() with additional retaining of HW_SZ bit
     * so that pmd_trans_huge() remains true for this PMD.
     */
    __pmd((pmd_val(pmd) & (_PAGE_CHG_MASK | _PAGE_HW_SZ)) | pgprot_val(newprot))
}

pub unsafe fn set_pmd_at(
    _mm: *mut mm_struct,
    _addr: unsigned_long,
    pmdp: *mut pmd_t,
    pmd: pmd_t,
) {
    *pmdp = pmd;
}

extern "C" {
    pub fn update_mmu_cache_pmd(
        vma: *mut vm_area_struct,
        addr: unsigned_long,
        pmd: *mut pmd_t,
    );

    pub fn flush_pmd_tlb_range(
        vma: *mut vm_area_struct,
        start: unsigned_long,
        end: unsigned_long,
    );
}

/* We don't have hardware dirty/accessed bits, generic_pmdp_establish is fine. */
pub use generic_pmdp_establish as pmdp_establish;

/* __HAVE_ARCH_FLUSH_PMD_TLB_RANGE */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
