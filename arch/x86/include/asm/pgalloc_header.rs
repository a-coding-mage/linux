/* SPDX-License-Identifier: GPL-2.0 */

// Original dependencies: linux/threads.h, linux/mm.h, linux/pagemap.h,
// asm/cpufeature.h, asm-generic/pgalloc.h.
// The CONFIG_PARAVIRT_XXL branch is selected by the build configuration.

pub const __HAVE_ARCH_PTE_ALLOC_ONE: bool = true;
pub const __HAVE_ARCH_PGD_FREE: bool = true;

#[inline]
pub unsafe fn __paravirt_pgd_alloc(mm: *mut mm_struct) -> ::core::ffi::c_int {
    let _ = mm;
    0
}

// CONFIG_PARAVIRT_XXL is supplied by the build configuration.  These are the
// non-XXL definitions from the source header.
#[inline]
pub unsafe fn paravirt_pgd_alloc(mm: *mut mm_struct) -> ::core::ffi::c_int {
    __paravirt_pgd_alloc(mm)
}

#[inline]
pub unsafe fn paravirt_pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    let _ = (mm, pgd);
}

#[inline]
pub unsafe fn paravirt_alloc_pte(mm: *mut mm_struct, pfn: c_ulong) {
    let _ = (mm, pfn);
}

#[inline]
pub unsafe fn paravirt_alloc_pmd(mm: *mut mm_struct, pfn: c_ulong) {
    let _ = (mm, pfn);
}

#[inline]
pub unsafe fn paravirt_alloc_pmd_clone(pfn: c_ulong, clonepfn: c_ulong, start: c_ulong, count: c_ulong) {
    let _ = (pfn, clonepfn, start, count);
}

#[inline]
pub unsafe fn paravirt_alloc_pud(mm: *mut mm_struct, pfn: c_ulong) {
    let _ = (mm, pfn);
}

#[inline]
pub unsafe fn paravirt_alloc_p4d(mm: *mut mm_struct, pfn: c_ulong) {
    let _ = (mm, pfn);
}

#[inline]
pub unsafe fn paravirt_release_pte(pfn: c_ulong) { let _ = pfn; }
#[inline]
pub unsafe fn paravirt_release_pmd(pfn: c_ulong) { let _ = pfn; }
#[inline]
pub unsafe fn paravirt_release_pud(pfn: c_ulong) { let _ = pfn; }
#[inline]
pub unsafe fn paravirt_release_p4d(pfn: c_ulong) { let _ = pfn; }

#[inline]
pub unsafe fn pgd_allocation_order() -> c_uint {
    if cpu_feature_enabled(X86_FEATURE_PTI) { 1 } else { 0 }
}

extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
    pub fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t);
    pub fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t;
    pub fn ___pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page);
}

#[inline]
pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page, address: c_ulong) {
    let _ = address;
    ___pte_free_tlb(tlb, pte);
}

#[inline]
pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    paravirt_alloc_pte(mm, __pa(pte) >> PAGE_SHIFT);
    set_pmd(pmd, __pmd(__pa(pte) | _PAGE_TABLE));
}

#[inline]
pub unsafe fn pmd_populate_kernel_safe(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    paravirt_alloc_pte(mm, __pa(pte) >> PAGE_SHIFT);
    set_pmd_safe(pmd, __pmd(__pa(pte) | _PAGE_TABLE));
}

#[inline]
pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut page) {
    let pfn = page_to_pfn(pte);
    paravirt_alloc_pte(mm, pfn);
    set_pmd(pmd, __pmd(((pfn as pteval_t) << PAGE_SHIFT) | _PAGE_TABLE));
}

// CONFIG_PGTABLE_LEVELS > 2
extern "C" { pub fn ___pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t); }

#[inline]
pub unsafe fn __pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t, address: c_ulong) {
    let _ = address;
    ___pmd_free_tlb(tlb, pmd);
}

// CONFIG_X86_PAE: pud_populate is provided externally.
#[inline]
pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    paravirt_alloc_pmd(mm, __pa(pmd) >> PAGE_SHIFT);
    set_pud(pud, __pud(_PAGE_TABLE | __pa(pmd)));
}

#[inline]
pub unsafe fn pud_populate_safe(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    paravirt_alloc_pmd(mm, __pa(pmd) >> PAGE_SHIFT);
    set_pud_safe(pud, __pud(_PAGE_TABLE | __pa(pmd)));
}

// CONFIG_PGTABLE_LEVELS > 3
#[inline]
pub unsafe fn p4d_populate(mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    paravirt_alloc_pud(mm, __pa(pud) >> PAGE_SHIFT);
    set_p4d(p4d, __p4d(_PAGE_TABLE | __pa(pud)));
}

#[inline]
pub unsafe fn p4d_populate_safe(mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    paravirt_alloc_pud(mm, __pa(pud) >> PAGE_SHIFT);
    set_p4d_safe(p4d, __p4d(_PAGE_TABLE | __pa(pud)));
}

extern "C" { pub fn ___pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t); }

#[inline]
pub unsafe fn __pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t, address: c_ulong) {
    let _ = address;
    ___pud_free_tlb(tlb, pud);
}

// CONFIG_PGTABLE_LEVELS > 4
#[inline]
pub unsafe fn pgd_populate(mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
    if !pgtable_l5_enabled() { return; }
    paravirt_alloc_p4d(mm, __pa(p4d) >> PAGE_SHIFT);
    set_pgd(pgd, __pgd(_PAGE_TABLE | __pa(p4d)));
}

#[inline]
pub unsafe fn pgd_populate_safe(mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
    if !pgtable_l5_enabled() { return; }
    paravirt_alloc_p4d(mm, __pa(p4d) >> PAGE_SHIFT);
    set_pgd_safe(pgd, __pgd(_PAGE_TABLE | __pa(p4d)));
}

extern "C" { pub fn ___p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t); }

#[inline]
pub unsafe fn __p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t, address: c_ulong) {
    let _ = address;
    if pgtable_l5_enabled() { ___p4d_free_tlb(tlb, p4d); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
