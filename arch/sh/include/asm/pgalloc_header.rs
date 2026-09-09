/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux memory-management and SH page-table code.
// The C header's include guard and include directives are intentionally omitted.

// C build-time architecture feature markers.
pub const __HAVE_ARCH_PMD_ALLOC_ONE: bool = true;
pub const __HAVE_ARCH_PMD_FREE: bool = true;
pub const __HAVE_ARCH_PGD_FREE: bool = true;

extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
    pub fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t);
}

// Preserved from: #if PAGETABLE_LEVELS > 2. These declarations and the TLB
// helper are intended to be enabled only when that build-time condition holds.
extern "C" {
    pub fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmd: *mut pmd_t);
    pub fn pmd_alloc_one(mm: *mut mm_struct, address: ::core::ffi::c_ulong) -> *mut pmd_t;
    pub fn pmd_free(mm: *mut mm_struct, pmd: *mut pmd_t);
}

#[inline(always)]
pub unsafe fn __pmd_free_tlb(tlb: *mut mmu_gather, pmdp: *mut pmd_t, _addr: ::core::ffi::c_ulong) {
    pmd_free((*tlb).mm, pmdp);
}

#[inline(always)]
pub unsafe fn pmd_populate_kernel(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    set_pmd(pmd, __pmd(pte as ::core::ffi::c_ulong));
}

#[inline(always)]
pub unsafe fn pmd_populate(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: pgtable_t,
) {
    set_pmd(pmd, __pmd(page_address(pte) as ::core::ffi::c_ulong));
}

#[inline(always)]
pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: pgtable_t,
    _addr: ::core::ffi::c_ulong,
) {
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
