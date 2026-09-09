/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008, 2009 Cavium Networks, Inc.
 */

// Dependency supplied by asm/page.h.

pub const __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR: bool = true;

pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _sz: ::core::ffi::c_ulong,
) -> pte_t {
    // C assigns pte_val(clear) from invalid_pte_table.  The transmute preserves
    // the source representation assignment; pte_t is supplied by asm/page.h.
    let clear: pte_t = ::core::mem::transmute::<usize, pte_t>(
        invalid_pte_table as usize,
    );
    let pte = *ptep;

    set_pte_at(mm, addr, ptep, clear);
    pte
}

pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;

pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    let sz = huge_page_size(hstate_vma(vma));

    /*
     * clear the huge pte entry firstly, so that the other smp threads will
     * not get old pte entry after finishing flush_tlb_page and before
     * setting new huge pte entry
     */
    let pte = huge_ptep_get_and_clear((*vma).vm_mm, addr, ptep, sz);
    flush_tlb_page(vma, addr);
    pte
}

pub const __HAVE_ARCH_HUGE_PTE_NONE: bool = true;

pub unsafe fn huge_pte_none(pte: pte_t) -> ::core::ffi::c_int {
    let val = pte_val(pte) & !_PAGE_GLOBAL;
    if val == 0 || val == invalid_pte_table as usize {
        1
    } else {
        0
    }
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS: bool = true;

pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    _dirty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let changed = if !pte_same(*ptep, pte) { 1 } else { 0 };

    if changed != 0 {
        set_pte_at((*vma).vm_mm, addr, ptep, pte);
        /*
         * There could be some standard sized pages in there,
         * get them all.
         */
        flush_tlb_range(vma, addr, addr.wrapping_add(HPAGE_SIZE));
    }
    changed
}

// Declarations and constants referenced above are supplied by the included
// asm/page.h and asm-generic/hugetlb.h interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
