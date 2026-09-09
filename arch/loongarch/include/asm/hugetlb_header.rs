/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Translated from asm/hugetlb.h.  Definitions from asm/page.h and
// asm-generic/hugetlb.h are supplied by the surrounding kernel bindings.

extern "C" {
    pub fn pmd_to_entrylo(pmd_val: ::core::ffi::c_ulong) -> u64;
}

pub const __HAVE_ARCH_HUGE_PTE_CLEAR: bool = true;

#[inline]
pub unsafe fn huge_pte_clear(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _sz: ::core::ffi::c_ulong,
) {
    let mut clear: pte_t = core::mem::zeroed();

    pte_val_set(&mut clear, invalid_pte_table as ::core::ffi::c_ulong);
    set_pte_at(mm, addr, ptep, clear);
}

pub const __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR: bool = true;

#[inline]
pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _sz: ::core::ffi::c_ulong,
) -> pte_t {
    let mut clear: pte_t = core::mem::zeroed();
    let pte: pte_t = ptep_get(ptep);

    pte_val_set(&mut clear, invalid_pte_table as ::core::ffi::c_ulong);
    set_pte_at(mm, addr, ptep, clear);
    pte
}

pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;

#[inline]
pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    let sz: ::core::ffi::c_ulong = huge_page_size(hstate_vma(vma));

    let pte = huge_ptep_get_and_clear((*vma).vm_mm, addr, ptep, sz);
    flush_tlb_page(vma, addr);
    pte
}

pub const __HAVE_ARCH_HUGE_PTE_NONE: bool = true;

#[inline]
pub unsafe fn huge_pte_none(pte: pte_t) -> i32 {
    let val = pte_val(pte) & !(_PAGE_GLOBAL as ::core::ffi::c_ulong);
    if val == 0 || val == invalid_pte_table as ::core::ffi::c_ulong {
        1
    } else {
        0
    }
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS: bool = true;

#[inline]
pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    _dirty: i32,
) -> i32 {
    let changed: i32 = if !pte_same(ptep_get(ptep), pte) { 1 } else { 0 };

    if changed != 0 {
        set_pte_at((*vma).vm_mm, addr, ptep, pte);
        /*
         * There could be some standard sized pages in there,
         * get them all.
         */
        flush_tlb_range(vma, addr, addr + HPAGE_SIZE as ::core::ffi::c_ulong);
    }
    changed
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
