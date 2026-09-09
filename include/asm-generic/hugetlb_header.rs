/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm-generic/hugetlb.h.
// Dependencies supplied by linux/swap.h and linux/swapops.h are external.

pub unsafe fn huge_pte_write(pte: pte_t) -> usize {
    pte_write(pte)
}

pub unsafe fn huge_pte_dirty(pte: pte_t) -> usize {
    pte_dirty(pte)
}

pub unsafe fn huge_pte_mkwrite(pte: pte_t) -> pte_t {
    pte_mkwrite_novma(pte)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_WRPROTECT"))]
pub unsafe fn huge_pte_wrprotect(pte: pte_t) -> pte_t {
    pte_wrprotect(pte)
}

pub unsafe fn huge_pte_mkdirty(pte: pte_t) -> pte_t {
    pte_mkdirty(pte)
}

pub unsafe fn huge_pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte_modify(pte, newprot)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_MKUFFD"))]
pub unsafe fn huge_pte_mkuffd(pte: pte_t) -> pte_t {
    huge_pte_wrprotect(pte_mkuffd(pte))
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_CLEAR_UFFD"))]
pub unsafe fn huge_pte_clear_uffd(pte: pte_t) -> pte_t {
    pte_clear_uffd(pte)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_UFFD"))]
pub unsafe fn huge_pte_uffd(pte: pte_t) -> i32 {
    pte_uffd(pte)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_CLEAR"))]
pub unsafe fn huge_pte_clear(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
    _sz: usize,
) {
    pte_clear(mm, addr, ptep);
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_SET_HUGE_PTE_AT"))]
pub unsafe fn set_huge_pte_at(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
    pte: pte_t,
    _sz: usize,
) {
    set_pte_at(mm, addr, ptep, pte);
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR"))]
pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
    _sz: usize,
) -> pte_t {
    ptep_get_and_clear(mm, addr, ptep)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH"))]
pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    addr: usize,
    ptep: *mut pte_t,
) -> pte_t {
    ptep_clear_flush(vma, addr, ptep)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTE_NONE"))]
pub unsafe fn huge_pte_none(pte: pte_t) -> i32 {
    pte_none(pte)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT"))]
pub unsafe fn huge_ptep_set_wrprotect(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) {
    ptep_set_wrprotect(mm, addr, ptep);
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS"))]
pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: usize,
    ptep: *mut pte_t,
    pte: pte_t,
    dirty: i32,
) -> i32 {
    ptep_set_access_flags(vma, addr, ptep, pte, dirty)
}

#[cfg(not(feature = "__HAVE_ARCH_HUGE_PTEP_GET"))]
pub unsafe fn huge_ptep_get(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) -> pte_t {
    ptep_get(ptep)
}

#[cfg(not(feature = "__HAVE_ARCH_GIGANTIC_PAGE_RUNTIME_SUPPORTED"))]
pub fn gigantic_page_runtime_supported() -> bool {
    // Equivalent build-time condition for IS_ENABLED(CONFIG_ARCH_HAS_GIGANTIC_PAGE).
    cfg!(feature = "CONFIG_ARCH_HAS_GIGANTIC_PAGE")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
