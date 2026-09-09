/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_HUGETLB_PAGE and architecture-specific includes are build-time
// conditions supplied by the surrounding kernel translation.

#[cfg(CONFIG_HUGETLB_PAGE)]
extern "C" {
    pub static mut hugetlb_disabled: bool;
}

#[cfg(CONFIG_HUGETLB_PAGE)]
#[inline]
pub unsafe fn hugepages_supported() -> bool {
    if hugetlb_disabled {
        return false;
    }

    HPAGE_SHIFT != 0
}

#[cfg(CONFIG_HUGETLB_PAGE)]
extern "C" {
    pub fn hugetlbpage_init_defaultsize();

    pub fn slice_is_hugepage_only_range(
        mm: *mut mm_struct,
        addr: c_ulong,
        len: c_ulong,
    ) -> c_int;
}

#[cfg(CONFIG_HUGETLB_PAGE)]
#[inline]
pub unsafe fn is_hugepage_only_range(
    mm: *mut mm_struct,
    addr: c_ulong,
    len: c_ulong,
) -> c_int {
    // IS_ENABLED(CONFIG_PPC_64S_HASH_MMU) && !radix_enabled()
    if IS_ENABLED_CONFIG_PPC_64S_HASH_MMU && !radix_enabled() {
        return slice_is_hugepage_only_range(mm, addr, len);
    }
    0
}

// __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT
#[cfg(CONFIG_HUGETLB_PAGE)]
extern "C" {
    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: c_ulong,
    );
}

// __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR
#[cfg(CONFIG_HUGETLB_PAGE)]
#[inline]
pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    sz: c_ulong,
) -> pte_t {
    __pte(pte_update(mm, addr, ptep, !0 as c_ulong, 0, 1))
}

// __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH
#[cfg(CONFIG_HUGETLB_PAGE)]
#[inline]
pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    let sz: c_ulong = huge_page_size(hstate_vma(vma));
    let pte = huge_ptep_get_and_clear((*vma).vm_mm, addr, ptep, sz);
    flush_hugetlb_page(vma, addr);
    pte
}

// __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS
#[cfg(CONFIG_HUGETLB_PAGE)]
extern "C" {
    pub fn huge_ptep_set_access_flags(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        dirty: c_int,
    ) -> c_int;
}

#[cfg(not(CONFIG_HUGETLB_PAGE))]
#[inline]
pub unsafe fn flush_hugetlb_page(_vma: *mut vm_area_struct, _vmaddr: c_ulong) {}

#[cfg(not(CONFIG_HUGETLB_PAGE))]
#[inline]
pub unsafe fn hugetlbpage_init_defaultsize() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
