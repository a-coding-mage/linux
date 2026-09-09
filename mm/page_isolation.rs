// SPDX-License-Identifier: GPL-2.0
/* linux/mm/page_isolation.c — source-level Rust translation */

// Kernel headers and trace-event declarations are supplied by the surrounding
// kernel translation unit.

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)] pub struct zone { pub zone_start_pfn: c_ulong, pub nr_isolate_pageblock: c_ulong, pub lock: [u8; 0] }
#[repr(C)] pub struct page;
#[repr(C)] pub struct folio { pub page: page }
#[repr(C)] pub struct hstate;

pub type pb_isolate_mode = c_int;
pub const PB_ISOLATE_MODE_MEM_OFFLINE: pb_isolate_mode = 0;
pub const PB_ISOLATE_MODE_CMA_ALLOC: pb_isolate_mode = 1;
pub const ZONE_MOVABLE: c_int = 3;
pub const MAX_FOLIO_ORDER: c_int = 31;
pub const MAX_PAGE_ORDER: c_int = 11;

extern "C" {
    fn PageReserved(page: *mut page) -> bool;
    fn zone_idx(zone: *mut zone) -> c_int;
    fn PageCompound(page: *mut page) -> bool;
    fn page_folio(page: *mut page) -> *mut folio;
    fn compound_order(page: *mut page) -> c_int;
    fn folio_test_hugetlb(folio: *mut folio) -> bool;
    fn size_to_hstate(size: c_ulong) -> *mut hstate;
    fn hugepage_migration_supported(h: *mut hstate) -> bool;
    fn PageLRU(page: *mut page) -> bool;
    fn page_to_pfn(page: *mut page) -> c_ulong;
    fn page_ref_count(page: *mut page) -> c_int;
    fn PageBuddy(page: *mut page) -> bool;
    fn buddy_order(page: *mut page) -> c_int;
    fn PageHWPoison(page: *mut page) -> bool;
    fn PageOffline(page: *mut page) -> bool;
    fn page_has_movable_ops(page: *mut page) -> bool;
    fn pfn_to_page(pfn: c_ulong) -> *mut page;
    fn page_zone(page: *mut page) -> *mut zone;
    fn pageblock_start_pfn(pfn: c_ulong) -> c_ulong;
    fn is_migrate_cma_page(page: *mut page) -> bool;
    fn PageUnaccepted(page: *mut page) -> bool;
    fn accept_page(page: *mut page);
    fn is_migrate_isolate_page(page: *mut page) -> bool;
    fn pageblock_end_pfn(pfn: c_ulong) -> c_ulong;
    fn pageblock_isolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;
    fn dump_page(page: *mut page, reason: *const c_char);
    fn find_buddy_page_pfn(page: *mut page, pfn: c_ulong, order: c_int, pfn2: *mut c_ulong) -> *mut page;
    fn __isolate_free_page(page: *mut page, order: c_int) -> bool;
    fn clear_pageblock_isolate(page: *mut page);
    fn __putback_isolated_page(page: *mut page, order: c_int, migratetype: c_int);
    fn get_pageblock_migratetype(page: *mut page) -> c_int;
    fn pageblock_unisolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;
    fn pfn_to_online_page(pfn: c_ulong) -> *mut page;
    fn pageblock_aligned(pfn: c_ulong) -> bool;
    fn pageblock_align(pfn: c_ulong) -> c_ulong;
    fn compound_head(page: *mut page) -> *mut page;
    fn compound_nr(page: *mut page) -> c_ulong;
    fn PageHuge(page: *mut page) -> bool;
    fn wait_for_freed_hugetlb_folios();
    fn page_count(page: *mut page) -> c_int;
    fn trace_test_pages_isolated(start: c_ulong, end: c_ulong, pfn: c_ulong);
}

#[inline]
pub unsafe fn page_is_unmovable(zone: *mut zone, page: *mut page, mode: pb_isolate_mode, step: *mut c_ulong) -> bool {
    if PageReserved(page) { return true; }
    if zone_idx(zone) == ZONE_MOVABLE { return false; }
    if PageCompound(page) {
        let folio = page_folio(page);
        let order = compound_order(&mut (*folio).page);
        if order > MAX_FOLIO_ORDER { return true; }
        if folio_test_hugetlb(folio) {
            // CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION is a build-time condition.
            let h = size_to_hstate((4096u64) << order);
            if h.is_null() || !hugepage_migration_supported(h) { return true; }
        } else if !PageLRU(page) { return true; }
        let nr_pages = 1u64 << order;
        let pfn = page_to_pfn(page);
        *step = (pfn | (nr_pages - 1)).wrapping_add(1).wrapping_sub(pfn);
        return false;
    }
    if page_ref_count(page) == 0 {
        if PageBuddy(page) { *step = 1u64 << buddy_order(page); }
        return false;
    }
    if mode == PB_ISOLATE_MODE_MEM_OFFLINE && PageHWPoison(page) { return false; }
    if mode == PB_ISOLATE_MODE_MEM_OFFLINE && PageOffline(page) { return false; }
    if PageLRU(page) || page_has_movable_ops(page) { return false; }
    true
}

unsafe fn has_unmovable_pages(mut start_pfn: c_ulong, end_pfn: c_ulong, mode: pb_isolate_mode) -> *mut page {
    let mut page = pfn_to_page(start_pfn);
    let zone = page_zone(page);
    if pageblock_start_pfn(start_pfn) != pageblock_start_pfn(end_pfn - 1) { return page; }
    if is_migrate_cma_page(page) {
        if mode == PB_ISOLATE_MODE_CMA_ALLOC { return core::ptr::null_mut(); }
        return page;
    }
    while start_pfn < end_pfn {
        let mut step = 1;
        page = pfn_to_page(start_pfn);
        if page_is_unmovable(zone, page, mode, &mut step) { return page; }
        start_pfn += step;
    }
    core::ptr::null_mut()
}

unsafe fn set_migratetype_isolate(page: *mut page, mode: pb_isolate_mode, start_pfn: c_ulong, end_pfn: c_ulong) -> c_int {
    let zone = page_zone(page);
    if PageUnaccepted(page) { accept_page(page); }
    if is_migrate_isolate_page(page) { return -16; }
    let check_start = core::cmp::max(page_to_pfn(page), start_pfn);
    let check_end = core::cmp::min(pageblock_end_pfn(page_to_pfn(page)), end_pfn);
    let unmovable = has_unmovable_pages(check_start, check_end, mode);
    if unmovable.is_null() {
        if !pageblock_isolate_and_move_free_pages(zone, page) { return -16; }
        (*zone).nr_isolate_pageblock += 1;
        return 0;
    }
    if mode == PB_ISOLATE_MODE_MEM_OFFLINE { dump_page(unmovable, b"unmovable page\0".as_ptr() as *const c_char); }
    -16
}

unsafe fn unset_migratetype_isolate(page: *mut page) {
    let zone = page_zone(page);
    if !is_migrate_isolate_page(page) { return; }
    let mut isolated_page = false;
    if PageBuddy(page) {
        let order = buddy_order(page);
        if order >= 0 && order < MAX_PAGE_ORDER {
            let buddy = find_buddy_page_pfn(page, page_to_pfn(page), order, core::ptr::null_mut());
            if !buddy.is_null() && !is_migrate_isolate_page(buddy) { isolated_page = __isolate_free_page(page, order); }
        }
    }
    if !isolated_page {
        let _ = pageblock_unisolate_and_move_free_pages(zone, page);
    } else {
        clear_pageblock_isolate(page);
        __putback_isolated_page(page, buddy_order(page), get_pageblock_migratetype(page));
    }
    (*zone).nr_isolate_pageblock -= 1;
}

unsafe fn __first_valid_page(pfn: c_ulong, nr_pages: c_ulong) -> *mut page {
    for i in 0..nr_pages { let page = pfn_to_online_page(pfn + i); if !page.is_null() { return page; } }
    core::ptr::null_mut()
}

unsafe fn isolate_single_pageblock(boundary_pfn: c_ulong, mode: pb_isolate_mode, isolate_before: bool, skip_isolation: bool) -> c_int {
    let isolate_pageblock = if isolate_before { boundary_pfn - 1 } else { boundary_pfn };
    let zone = page_zone(pfn_to_page(isolate_pageblock));
    let start_pfn = core::cmp::max(isolate_pageblock & !(2048 - 1), (*zone).zone_start_pfn);
    if !skip_isolation && set_migratetype_isolate(pfn_to_page(isolate_pageblock), mode, isolate_pageblock, isolate_pageblock + 1) != 0 { return -16; }
    if (isolate_before && pfn_to_online_page(boundary_pfn).is_null()) || (!isolate_before && pfn_to_online_page(boundary_pfn - 1).is_null()) { return 0; }
    let mut pfn = start_pfn;
    while pfn < boundary_pfn {
        let page = __first_valid_page(pfn, boundary_pfn - pfn); if page.is_null() { break; }
        pfn = page_to_pfn(page);
        if PageUnaccepted(page) { pfn += 2048; continue; }
        if PageBuddy(page) { pfn += 1u64 << buddy_order(page); continue; }
        if PageCompound(page) {
            let head = compound_head(page); let head_pfn = page_to_pfn(head); let nr_pages = compound_nr(head);
            if head_pfn + nr_pages <= boundary_pfn || PageHuge(page) { pfn = head_pfn + nr_pages; continue; }
            if !skip_isolation { unset_migratetype_isolate(pfn_to_page(isolate_pageblock)); }
            return -16;
        }
        pfn += 1;
    }
    0
}

pub unsafe fn start_isolate_page_range(start_pfn: c_ulong, end_pfn: c_ulong, mode: pb_isolate_mode) -> c_int {
    let isolate_start = pageblock_start_pfn(start_pfn); let isolate_end = pageblock_align(end_pfn); let mut skip = false;
    let mut ret = isolate_single_pageblock(isolate_start, mode, false, skip); if ret != 0 { return ret; }
    if isolate_start == isolate_end - 1 { skip = true; }
    ret = isolate_single_pageblock(isolate_end, mode, true, skip); if ret != 0 { unset_migratetype_isolate(pfn_to_page(isolate_start)); return ret; }
    let mut pfn = isolate_start + 1;
    while pfn < isolate_end - 1 { let page = __first_valid_page(pfn, 1); if !page.is_null() && set_migratetype_isolate(page, mode, start_pfn, end_pfn) != 0 { undo_isolate_page_range(isolate_start, pfn); unset_migratetype_isolate(pfn_to_page(isolate_end - 1)); return -16; } pfn += 1; }
    0
}

pub unsafe fn undo_isolate_page_range(start_pfn: c_ulong, end_pfn: c_ulong) {
    let mut pfn = pageblock_start_pfn(start_pfn); let end = pageblock_align(end_pfn);
    while pfn < end { let page = __first_valid_page(pfn, 1); if !page.is_null() && is_migrate_isolate_page(page) { unset_migratetype_isolate(page); } pfn += 1; }
}

unsafe fn __test_page_isolated_in_pageblock(mut pfn: c_ulong, end_pfn: c_ulong, mode: pb_isolate_mode) -> c_ulong {
    while pfn < end_pfn { let page = pfn_to_page(pfn); if PageBuddy(page) { pfn += 1u64 << buddy_order(page); } else if mode == PB_ISOLATE_MODE_MEM_OFFLINE && PageHWPoison(page) { pfn += 1; } else if mode == PB_ISOLATE_MODE_MEM_OFFLINE && PageOffline(page) && page_count(page) == 0 { pfn += 1; } else { break; } } pfn
}

pub unsafe fn test_pages_isolated(start_pfn: c_ulong, end_pfn: c_ulong, mode: pb_isolate_mode) -> c_int {
    wait_for_freed_hugetlb_folios(); let mut pfn = start_pfn;
    while pfn < end_pfn { let page = __first_valid_page(pfn, 1); if !page.is_null() && !is_migrate_isolate_page(page) { break; } pfn += 1; }
    let page = __first_valid_page(start_pfn, end_pfn - start_pfn); let ret;
    if pfn < end_pfn || page.is_null() { ret = -16; } else { pfn = __test_page_isolated_in_pageblock(start_pfn, end_pfn, mode); ret = if pfn < end_pfn { -16 } else { 0 }; }
    trace_test_pages_isolated(start_pfn, end_pfn, pfn); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
