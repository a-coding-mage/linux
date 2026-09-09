/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Macros for manipulating and testing flags related to a
 * pageblock_nr_pages number of pages.
 *
 * Copyright (C) IBM Corporation, 2006
 *
 * Original author, Mel Gorman
 * Major cleanups and reduction of bit operations, Andy Whitcroft
 */

/* linux/types.h */

/* Bit indices that affect a whole block of pages */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pageblock_bits {
    PB_migrate_0,
    PB_migrate_1,
    PB_migrate_2,
    PB_compact_skip, /* If set the block is skipped by compaction */

    /*
     * Pageblock isolation is represented with a separate bit, so that
     * the migratetype of a block is not overwritten by isolation.
     */
    #[cfg(feature = "CONFIG_MEMORY_ISOLATION")]
    PB_migrate_isolate, /* If set the block is isolated */

    /*
     * Assume the bits will always align on a word. If this assumption
     * changes then get/set pageblock needs updating.
     */
    __NR_PAGEBLOCK_BITS,
}

/* Build-time kernel helpers/constants are supplied by other dependencies. */
pub const NR_PAGEBLOCK_BITS: usize = roundup_pow_of_two(__NR_PAGEBLOCK_BITS as usize);
pub const PAGEBLOCK_MIGRATETYPE_MASK: usize =
    BIT(PB_migrate_0 as usize) | BIT(PB_migrate_1 as usize) | BIT(PB_migrate_2 as usize);

#[cfg(feature = "CONFIG_MEMORY_ISOLATION")]
pub const PAGEBLOCK_ISO_MASK: usize = BIT(PB_migrate_isolate as usize);
#[cfg(not(feature = "CONFIG_MEMORY_ISOLATION"))]
pub const PAGEBLOCK_ISO_MASK: usize = 0;

/*
 * Huge-page configuration selects pageblock_order. The referenced constants
 * and helper are provided by the surrounding kernel translation.
 */
#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_VARIABLE")]
extern "C" {
    pub static mut pageblock_order: ::core::ffi::c_uint;
}

#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
#[cfg(not(feature = "CONFIG_HUGETLB_PAGE_SIZE_VARIABLE"))]
pub const pageblock_order: u32 = MIN_T(HUGETLB_PAGE_ORDER, PAGE_BLOCK_MAX_ORDER);

#[cfg(all(not(feature = "CONFIG_HUGETLB_PAGE"), feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
pub const pageblock_order: u32 = MIN_T(HPAGE_PMD_ORDER, PAGE_BLOCK_MAX_ORDER);

#[cfg(all(not(feature = "CONFIG_HUGETLB_PAGE"), not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
pub const pageblock_order: u32 = PAGE_BLOCK_MAX_ORDER;

pub const pageblock_nr_pages: usize = 1usize << pageblock_order;

#[inline]
pub const fn pageblock_align(pfn: usize) -> usize {
    ALIGN(pfn, pageblock_nr_pages)
}

#[inline]
pub const fn pageblock_aligned(pfn: usize) -> bool {
    IS_ALIGNED(pfn, pageblock_nr_pages)
}

#[inline]
pub const fn pageblock_start_pfn(pfn: usize) -> usize {
    ALIGN_DOWN(pfn, pageblock_nr_pages)
}

#[inline]
pub const fn pageblock_end_pfn(pfn: usize) -> usize {
    ALIGN(pfn + 1, pageblock_nr_pages)
}

/* Forward declaration */
#[repr(C)]
pub struct page;

extern "C" {
    pub fn get_pfnblock_migratetype(page: *const page, pfn: usize) -> migratetype;
    pub fn get_pfnblock_bit(page: *const page, pfn: usize, pb_bit: pageblock_bits) -> bool;
    pub fn set_pfnblock_bit(page: *const page, pfn: usize, pb_bit: pageblock_bits);
    pub fn clear_pfnblock_bit(page: *const page, pfn: usize, pb_bit: pageblock_bits);
}

/* Declarations for getting and setting flags. See mm/page_alloc.c */
#[cfg(feature = "CONFIG_COMPACTION")]
#[inline]
pub unsafe fn get_pageblock_skip(page: *mut page) -> bool {
    get_pfnblock_bit(page, page_to_pfn(page), pageblock_bits::PB_compact_skip)
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[inline]
pub unsafe fn clear_pageblock_skip(page: *mut page) {
    clear_pfnblock_bit(page, page_to_pfn(page), pageblock_bits::PB_compact_skip)
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[inline]
pub unsafe fn set_pageblock_skip(page: *mut page) {
    set_pfnblock_bit(page, page_to_pfn(page), pageblock_bits::PB_compact_skip)
}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn get_pageblock_skip(_page: *mut page) -> bool { false }

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn clear_pageblock_skip(_page: *mut page) {}

#[cfg(not(feature = "CONFIG_COMPACTION"))]
#[inline]
pub unsafe fn set_pageblock_skip(_page: *mut page) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
