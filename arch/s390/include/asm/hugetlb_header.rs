/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  IBM System z Huge TLB Page Support for Kernel.
 *
 *    Copyright IBM Corp. 2008
 *    Author(s): Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

// C header guard: _ASM_S390_HUGETLB_H
// Dependencies: linux/cpufeature.h, linux/pgtable.h, linux/swap.h,
// linux/swapops.h, asm/page.h, and asm-generic/hugetlb.h.

#[inline]
pub unsafe fn hugepages_supported() -> bool {
    cpu_has_edat1()
}

// __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT
extern "C" {
    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: ::core::ffi::c_ulong,
    );
    pub fn __set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
    );
}

// __HAVE_ARCH_HUGE_PTEP_GET
extern "C" {
    pub fn huge_ptep_get(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn __huge_ptep_get_and_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;
}

// __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR
#[inline]
pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _sz: ::core::ffi::c_ulong,
) -> pte_t {
    __huge_ptep_get_and_clear(mm, addr, ptep)
}

// __HAVE_ARCH_HUGE_PTE_CLEAR
#[inline]
pub unsafe fn huge_pte_clear(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _sz: ::core::ffi::c_ulong,
) {
    let _ = (mm, addr);
    if (pte_val(ptep_get(ptep)) & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3 {
        set_pud(ptep as *mut pud_t, __pud(_REGION3_ENTRY_EMPTY));
    } else {
        set_pmd(ptep as *mut pmd_t, __pmd(_SEGMENT_ENTRY_EMPTY));
    }
}

// __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH
#[inline]
pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    address: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    __huge_ptep_get_and_clear((*vma).vm_mm, address, ptep)
}

// __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS
#[inline]
pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    _dirty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let changed = !pte_same(huge_ptep_get((*vma).vm_mm, addr, ptep), pte);

    if changed {
        __huge_ptep_get_and_clear((*vma).vm_mm, addr, ptep);
        __set_huge_pte_at((*vma).vm_mm, addr, ptep, pte);
    }
    changed as ::core::ffi::c_int
}

// __HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT
#[inline]
pub unsafe fn huge_ptep_set_wrprotect(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) {
    let pte = __huge_ptep_get_and_clear(mm, addr, ptep);
    __set_huge_pte_at(mm, addr, ptep, pte_wrprotect(pte));
}

// __HAVE_ARCH_HUGE_PTE_MKUFFD
#[inline]
pub unsafe fn huge_pte_mkuffd(pte: pte_t) -> pte_t {
    pte
}

// __HAVE_ARCH_HUGE_PTE_CLEAR_UFFD
#[inline]
pub unsafe fn huge_pte_clear_uffd(pte: pte_t) -> pte_t {
    pte
}

// __HAVE_ARCH_HUGE_PTE_UFFD
#[inline]
pub unsafe fn huge_pte_uffd(_pte: pte_t) -> ::core::ffi::c_int {
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
