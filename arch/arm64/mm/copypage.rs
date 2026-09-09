// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/mm/copypage.c
 *
 * Copyright (C) 2002 Deep Blue Solutions Ltd, All Rights Reserved.
 * Copyright (C) 2012 ARM Ltd.
 */

use core::ffi::c_void;

extern "C" {
    fn page_address(page: *mut page) -> *mut c_void;
    fn page_folio(page: *mut page) -> *mut folio;
    fn copy_page(to: *mut c_void, from: *mut c_void);
    fn kasan_hw_tags_enabled() -> bool;
    fn page_kasan_tag_reset(page: *mut page);
    fn system_supports_mte() -> bool;
    fn folio_test_hugetlb(folio: *mut folio) -> bool;
    fn folio_test_hugetlb_mte_tagged(folio: *mut folio) -> bool;
    fn folio_page(folio: *mut folio, index: usize) -> *mut page;
    fn folio_try_hugetlb_mte_tagging(folio: *mut folio);
    fn folio_nr_pages(folio: *mut folio) -> u32;
    fn mte_copy_page_tags(to: *mut c_void, from: *mut c_void);
    fn folio_set_hugetlb_mte_tagged(folio: *mut folio);
    fn page_mte_tagged(page: *mut page) -> bool;
    fn try_page_mte_tagging(page: *mut page);
    fn set_page_mte_tagged(page: *mut page);
    fn flush_dcache_page(page: *mut page);
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

pub unsafe fn copy_highpage(to: *mut page, from: *mut page) {
    let mut kto: *mut c_void = page_address(to);
    let mut kfrom: *mut c_void = page_address(from);
    let src: *mut folio = page_folio(from);
    let dst: *mut folio = page_folio(to);
    let mut i: u32;
    let mut nr_pages: u32;

    copy_page(kto, kfrom);

    if kasan_hw_tags_enabled() {
        page_kasan_tag_reset(to);
    }

    if !system_supports_mte() {
        return;
    }

    if folio_test_hugetlb(src) {
        if !folio_test_hugetlb_mte_tagged(src)
            || !core::ptr::eq(from, folio_page(src, 0))
        {
            return;
        }

        folio_try_hugetlb_mte_tagging(dst);

        /*
         * Populate tags for all subpages.
         *
         * Don't assume the first page is head page since
         * huge page copy may start from any subpage.
         */
        nr_pages = folio_nr_pages(src);
        i = 0;
        while i < nr_pages {
            kfrom = page_address(folio_page(src, i as usize));
            kto = page_address(folio_page(dst, i as usize));
            mte_copy_page_tags(kto, kfrom);
            i = i.wrapping_add(1);
        }
        folio_set_hugetlb_mte_tagged(dst);
    } else if page_mte_tagged(from) {
        /*
         * Most of the time it's a new page that shouldn't have been
         * tagged yet. However, folio migration can end up reusing the
         * same page without untagging it. Ignore the warning if the
         * page is already tagged.
         */
        try_page_mte_tagging(to);

        mte_copy_page_tags(kto, kfrom);
        set_page_mte_tagged(to);
    }
}

// EXPORT_SYMBOL(copy_highpage);

pub unsafe fn copy_user_highpage(
    to: *mut page,
    from: *mut page,
    _vaddr: usize,
    _vma: *mut vm_area_struct,
) {
    copy_highpage(to, from);
    flush_dcache_page(to);
}

// EXPORT_SYMBOL_GPL(copy_user_highpage);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
