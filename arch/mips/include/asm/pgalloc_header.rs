/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 2001, 2003 by Ralf Baechle
 * Copyright (C) 1999, 2000, 2001 Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external. The original header defines __HAVE_ARCH_PMD_ALLOC_ONE and
// __HAVE_ARCH_PUD_ALLOC_ONE and includes asm-generic/pgalloc.h.

pub unsafe extern "C" {
    pub fn pmd_init(addr: *mut core::ffi::c_void);
    pub fn pgd_init(addr: *mut core::ffi::c_void);
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
}

pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    let _ = mm;
    set_pmd(pmd, __pmd(pte as usize as core::ffi::c_ulong));
}

pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    let _ = mm;
    set_pmd(
        pmd,
        __pmd(page_address(pte) as usize as core::ffi::c_ulong),
    );
}

// Initialize a new pmd table with invalid pointers.

// This declaration is present only when __PAGETABLE_PMD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    let _ = mm;
    set_pud(pud, __pud(pmd as usize as core::ffi::c_ulong));
}

// Initialize a new pgd table with invalid pointers.

pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, pte: pgtable_t, address: core::ffi::c_ulong) {
    let _ = address;
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// This declaration is present only when __PAGETABLE_PMD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_alloc_one(mm: *mut mm_struct, address: core::ffi::c_ulong) -> *mut pmd_t {
    let _ = address;
    let mut ptdesc: *mut ptdesc = pagetable_alloc(GFP_KERNEL_ACCOUNT, PMD_TABLE_ORDER);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }

    if !pagetable_pmd_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }

    let pmd = ptdesc_address(ptdesc);
    pmd_init(pmd as *mut core::ffi::c_void);
    pmd
}

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn __pmd_free_tlb(
    tlb: *mut mmu_gather,
    x: *mut pmd_t,
    addr: core::ffi::c_ulong,
) {
    let _ = addr;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(x));
}

// This declaration is present only when __PAGETABLE_PUD_FOLDED is not defined.
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn pud_alloc_one(mm: *mut mm_struct, address: core::ffi::c_ulong) -> *mut pud_t {
    let _ = address;
    let ptdesc: *mut ptdesc = pagetable_alloc(GFP_KERNEL, PUD_TABLE_ORDER);

    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    pagetable_pud_ctor(ptdesc);
    let pud = ptdesc_address(ptdesc);

    pud_init(pud as *mut core::ffi::c_void);
    let _ = mm;
    pud
}

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_populate(mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    let _ = mm;
    set_p4d(p4d, __p4d(pud as usize as core::ffi::c_ulong));
}

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn __pud_free_tlb(
    tlb: *mut mmu_gather,
    x: *mut pud_t,
    addr: core::ffi::c_ulong,
) {
    let _ = addr;
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(x));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
