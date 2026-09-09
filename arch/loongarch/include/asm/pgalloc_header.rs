/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.
// The original header includes <linux/mm.h>, <linux/sched.h>, and
// <asm-generic/pgalloc.h>.

// Build-time feature markers from the original header.
pub const __HAVE_ARCH_PMD_ALLOC_ONE: bool = true;
pub const __HAVE_ARCH_PUD_ALLOC_ONE: bool = true;
pub const __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL: bool = true;

pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    set_pmd(pmd, __pmd(pte as unsigned_long));
}

pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    set_pmd(pmd, __pmd(page_address(pte) as unsigned_long));
}

// Original condition: compiled when __PAGETABLE_PMD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    set_pud(pud, __pud(pmd as unsigned_long));
}

// Original condition: compiled when __PAGETABLE_PUD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_populate(mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    set_p4d(p4d, __p4d(pud as unsigned_long));
}

pub unsafe extern "C" fn pagetable_init();

pub unsafe extern "C" fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let pte: *mut pte_t = __pte_alloc_one_kernel(mm);

    if !pte.is_null() {
        kernel_pte_init(pte);
    }

    pte
}

// #define __pte_free_tlb(tlb, pte, address) tlb_remove_ptdesc((tlb), page_ptdesc(pte))
#[inline]
pub unsafe fn __pte_free_tlb(tlb: *mut core::ffi::c_void, pte: *mut pte_t, address: unsigned_long) {
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// Original condition: compiled when __PAGETABLE_PMD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_alloc_one(mm: *mut mm_struct, address: unsigned_long) -> *mut pmd_t {
    let ptdesc: *mut ptdesc = pagetable_alloc(GFP_KERNEL_ACCOUNT, 0);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }

    if !pagetable_pmd_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }

    let pmd: *mut pmd_t = ptdesc_address(ptdesc);
    pmd_init(pmd);
    pmd
}

// #define __pmd_free_tlb(tlb, x, addr) tlb_remove_ptdesc((tlb), virt_to_ptdesc(x))
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[inline]
pub unsafe fn __pmd_free_tlb(tlb: *mut core::ffi::c_void, x: *mut pmd_t, addr: unsigned_long) {
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(x));
}

// Original condition: compiled when __PAGETABLE_PUD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn pud_alloc_one(mm: *mut mm_struct, address: unsigned_long) -> *mut pud_t {
    let ptdesc: *mut ptdesc = pagetable_alloc(GFP_KERNEL, 0);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    pagetable_pud_ctor(ptdesc);
    let pud: *mut pud_t = ptdesc_address(ptdesc);

    pud_init(pud);
    pud
}

// #define __pud_free_tlb(tlb, x, addr) tlb_remove_ptdesc((tlb), virt_to_ptdesc(x))
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
#[inline]
pub unsafe fn __pud_free_tlb(tlb: *mut core::ffi::c_void, x: *mut pud_t, addr: unsigned_long) {
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(x));
}

pub unsafe extern "C" fn populate_kernel_pte(addr: unsigned_long) -> *mut pte_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
