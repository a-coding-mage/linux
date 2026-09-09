// SPDX-License-Identifier: GPL-2.0
/*
 * HugeTLB Vmemmap Optimization (HVO)
 *
 * Copyright (c) 2020, ByteDance. All rights reserved.
 *
 *     Author: Muchun Song <songmuchun@bytedance.com>
 */

// C dependencies: <linux/hugetlb.h>, <linux/io.h>, and <linux/memblock.h>.

/*
 * Reserve one vmemmap page, all vmemmap addresses are mapped to it. See
 * Documentation/mm/vmemmap_dedup.rst.
 */
pub const HUGETLB_VMEMMAP_RESERVE_SIZE: usize = PAGE_SIZE;
pub const HUGETLB_VMEMMAP_RESERVE_PAGES: usize =
    HUGETLB_VMEMMAP_RESERVE_SIZE / core::mem::size_of::<struct_page>();

// CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP selects the implementation below.
#[cfg(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP)]
extern "C" {
    pub fn hugetlb_vmemmap_restore_folio(h: *const hstate, folio: *mut folio) -> i32;
    pub fn hugetlb_vmemmap_restore_folios(
        h: *const hstate,
        folio_list: *mut list_head,
        non_hvo_folios: *mut list_head,
    ) -> i64;
    pub fn hugetlb_vmemmap_optimize_folio(h: *const hstate, folio: *mut folio);
    pub fn hugetlb_vmemmap_optimize_folios(h: *mut hstate, folio_list: *mut list_head);
    pub fn hugetlb_vmemmap_optimize_bootmem_folios(
        h: *mut hstate,
        folio_list: *mut list_head,
    );

    #[cfg(CONFIG_SPARSEMEM_VMEMMAP_PREINIT)]
    pub fn hugetlb_vmemmap_init_early(nid: i32);
}

#[cfg(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP)]
#[inline]
pub unsafe fn hugetlb_vmemmap_size(h: *const hstate) -> usize {
    pages_per_huge_page(h) * core::mem::size_of::<struct_page>()
}

/*
 * Return how many vmemmap size associated with a HugeTLB page that can be
 * optimized and can be freed to the buddy allocator.
 */
#[cfg(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP)]
#[inline]
pub unsafe fn hugetlb_vmemmap_optimizable_size(h: *const hstate) -> u32 {
    let size = hugetlb_vmemmap_size(h) as isize
        - HUGETLB_VMEMMAP_RESERVE_SIZE as isize;

    if !is_power_of_2(core::mem::size_of::<struct_page>()) {
        return 0;
    }
    if size > 0 { size as u32 } else { 0 }
}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_restore_folio(_h: *const hstate, _folio: *mut folio) -> i32 {
    0
}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_restore_folios(
    _h: *const hstate,
    folio_list: *mut list_head,
    non_hvo_folios: *mut list_head,
) -> i64 {
    list_splice_init(folio_list, non_hvo_folios);
    0
}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_optimize_folio(_h: *const hstate, _folio: *mut folio) {}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_optimize_folios(_h: *mut hstate, _folio_list: *mut list_head) {}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_optimize_bootmem_folios(
    _h: *mut hstate,
    _folio_list: *mut list_head,
) {
}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_init_early(_nid: i32) {}

#[cfg(not(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP))]
#[inline]
pub unsafe fn hugetlb_vmemmap_optimizable_size(_h: *const hstate) -> u32 {
    0
}

#[inline]
pub unsafe fn hugetlb_vmemmap_optimizable(h: *const hstate) -> bool {
    hugetlb_vmemmap_optimizable_size(h) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
