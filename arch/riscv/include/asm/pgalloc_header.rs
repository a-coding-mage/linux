/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependencies supplied by the Linux/RISC-V environment are intentionally external.
// The following declarations are active when CONFIG_MMU is enabled in the source.

pub const __HAVE_ARCH_PUD_FREE: bool = true;

pub unsafe fn pmd_populate_kernel(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    let pfn: c_ulong = virt_to_pfn(pte);
    set_pmd(pmd, __pmd((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
}

pub unsafe fn pmd_populate(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: pgtable_t,
) {
    let pfn: c_ulong = virt_to_pfn(page_address(pte));
    set_pmd(pmd, __pmd((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
}

// Preserved from the source's !__PAGETABLE_PMD_FOLDED conditional.
pub unsafe fn pud_populate(_mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    let pfn: c_ulong = virt_to_pfn(pmd);
    set_pud(pud, __pud((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
}

pub unsafe fn p4d_populate(_mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    if pgtable_l4_enabled {
        let pfn: c_ulong = virt_to_pfn(pud);
        set_p4d(p4d, __p4d((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
    }
}

pub unsafe fn p4d_populate_safe(_mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    if pgtable_l4_enabled {
        let pfn: c_ulong = virt_to_pfn(pud);
        set_p4d_safe(p4d, __p4d((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
    }
}

pub unsafe fn pgd_populate(_mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
    if pgtable_l5_enabled {
        let pfn: c_ulong = virt_to_pfn(p4d);
        set_pgd(pgd, __pgd((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
    }
}

pub unsafe fn pgd_populate_safe(_mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
    if pgtable_l5_enabled {
        let pfn: c_ulong = virt_to_pfn(p4d);
        set_pgd_safe(pgd, __pgd((pfn << _PAGE_PFN_SHIFT) | _PAGE_TABLE));
    }
}

pub unsafe fn pud_free(mm: *mut mm_struct, pud: *mut pud_t) {
    if pgtable_l4_enabled {
        __pud_free(mm, pud);
    }
}

pub unsafe fn __pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t, _addr: c_ulong) {
    if pgtable_l4_enabled {
        tlb_remove_ptdesc(tlb, virt_to_ptdesc(pud));
    }
}

pub unsafe fn __p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t, _addr: c_ulong) {
    if pgtable_l5_enabled {
        tlb_remove_ptdesc(tlb, virt_to_ptdesc(p4d));
    }
}

pub unsafe fn sync_kernel_mappings(pgd: *mut pgd_t) {
    core::ptr::copy_nonoverlapping(
        pgd.add(USER_PTRS_PER_PGD),
        init_mm.pgd.add(USER_PTRS_PER_PGD),
        (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
    );
}

pub unsafe fn pgd_alloc(_mm: *mut mm_struct) -> *mut pgd_t {
    let pgd: *mut pgd_t = __pgd_alloc(_mm, 0);
    if likely(!pgd.is_null()) {
        /* Copy kernel mappings */
        sync_kernel_mappings(pgd);
    }
    pgd
}

pub unsafe fn __pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t, _addr: c_ulong) {
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pmd));
}

pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, pte: pgtable_t, _addr: c_ulong) {
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
