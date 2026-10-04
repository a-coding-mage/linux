// SPDX-License-Identifier: GPL-2.0-only
// Linux mm/page_alloc.c, C721–1923: capture, buddy coalescing/free preparation,
// PCP draining, initial freeing, splitting, allocation checks and preparation.
// All allocator decisions and loops remain here; C adapters expose header ABI.

#[inline]
unsafe fn task_capc(zone: *mut zone) -> *mut capture_control {
    #[cfg(CONFIG_COMPACTION)]
    {
        let task = rust_pa_b_current();
        let capc = rust_pa_b_task_capture(task);
        if !capc.is_null()
            && (*task).flags & RUST_PA_B_PF_KTHREAD == 0
            && (*capc).page.is_null()
            && (*capc).zone == zone
        {
            return capc;
        }
    }
    let _ = zone;
    core::ptr::null_mut()
}

#[inline]
unsafe fn compaction_capture(
    capc: *mut capture_control,
    page: *mut page,
    order: i32,
    migratetype: i32,
) -> bool {
    #[cfg(CONFIG_COMPACTION)]
    {
        if capc.is_null() || order != (*capc).order {
            return false;
        }
        if rust_pa_b_is_migrate_cma(migratetype) || rust_pa_b_is_migrate_isolate(migratetype) {
            return false;
        }
        if order < rust_pa_pageblock_order() as i32
            && migratetype == MIGRATE_MOVABLE as i32
            && (*capc).migratetype != MIGRATE_MOVABLE as i32
        {
            return false;
        }
        if migratetype != (*capc).migratetype {
            rust_pa_b_trace_extfrag(page, (*capc).order, order, (*capc).migratetype, migratetype);
        }
        (*capc).page = page;
        return true;
    }
    #[cfg(not(CONFIG_COMPACTION))]
    {
        let _ = (capc, page, order, migratetype);
        false
    }
}

#[inline]
unsafe fn account_freepages(zone: *mut zone, nr_pages: i32, migratetype: i32) {
    rust_pa_b_assert_zone_locked(zone);
    if rust_pa_b_is_migrate_isolate(migratetype) {
        return;
    }
    rust_pa_b_mod_zone_state(zone, NR_FREE_PAGES, nr_pages as Long);
    if rust_pa_b_is_migrate_cma(migratetype) {
        rust_pa_b_mod_zone_state(zone, NR_FREE_CMA_PAGES, nr_pages as Long);
    } else if migratetype == MIGRATE_HIGHATOMIC as i32 {
        // Deliberately unsigned wrapping arithmetic, matching the C counter.
        rust_pa_b_write_highatomic(
            zone,
            (*zone).nr_free_highatomic.wrapping_add(nr_pages as ULong),
        );
    }
}

#[inline]
unsafe fn __add_to_free_list(
    page: *mut page,
    zone: *mut zone,
    order: u32,
    migratetype: i32,
    tail: bool,
) {
    let area = core::ptr::addr_of_mut!((*zone).free_area[order as usize]);
    let nr_pages = 1i32 << order;
    rust_pa_b_warn_add_mt(page, migratetype, nr_pages);
    let list = core::ptr::addr_of_mut!((*area).free_list[migratetype as usize]);
    if tail {
        list_add_tail(page_buddy_list(page), list);
    } else {
        list_add(page_buddy_list(page), list);
    }
    (*area).nr_free = (*area).nr_free.wrapping_add(1);
    if order >= rust_pa_pageblock_order() && !rust_pa_b_is_migrate_isolate(migratetype) {
        rust_pa_b_mod_zone_state(zone, NR_FREE_PAGES_BLOCKS, nr_pages as Long);
    }
}

#[inline]
unsafe fn move_to_free_list(
    page: *mut page,
    zone: *mut zone,
    order: u32,
    old_mt: i32,
    new_mt: i32,
) {
    let area = core::ptr::addr_of_mut!((*zone).free_area[order as usize]);
    let mut nr_pages = 1i32 << order;
    rust_pa_b_warn_move_mt(page, old_mt, nr_pages);
    list_move_tail(
        page_buddy_list(page),
        core::ptr::addr_of_mut!((*area).free_list[new_mt as usize]),
    );
    account_freepages(zone, -nr_pages, old_mt);
    account_freepages(zone, nr_pages, new_mt);
    if order >= rust_pa_pageblock_order()
        && rust_pa_b_is_migrate_isolate(old_mt) != rust_pa_b_is_migrate_isolate(new_mt)
    {
        if !rust_pa_b_is_migrate_isolate(old_mt) {
            nr_pages = -nr_pages;
        }
        rust_pa_b_mod_zone_state(zone, NR_FREE_PAGES_BLOCKS, nr_pages as Long);
    }
}

#[inline]
unsafe fn __del_page_from_free_list(
    page: *mut page,
    zone: *mut zone,
    order: u32,
    migratetype: i32,
) {
    let nr_pages = 1i32 << order;
    rust_pa_b_warn_del_mt(page, migratetype, nr_pages);
    if rust_pa_b_page_reported(page) {
        rust_pa_b_clear_reported(page);
    }
    list_del(page_buddy_list(page));
    rust_pa_b_clear_buddy(page);
    set_page_private(page, 0);
    let area = core::ptr::addr_of_mut!((*zone).free_area[order as usize]);
    (*area).nr_free = (*area).nr_free.wrapping_sub(1);
    if order >= rust_pa_pageblock_order() && !rust_pa_b_is_migrate_isolate(migratetype) {
        rust_pa_b_mod_zone_state(zone, NR_FREE_PAGES_BLOCKS, -(nr_pages as Long));
    }
}

#[inline]
unsafe fn del_page_from_free_list(page: *mut page, zone: *mut zone, order: u32, migratetype: i32) {
    __del_page_from_free_list(page, zone, order, migratetype);
    account_freepages(zone, -(1i32 << order), migratetype);
}

#[inline]
unsafe fn get_page_from_free_area(area: *mut free_area, migratetype: i32) -> *mut page {
    let list = core::ptr::addr_of_mut!((*area).free_list[migratetype as usize]);
    if list_empty(list) {
        core::ptr::null_mut()
    } else {
        rust_pa_b_page_from_buddy((*list).next)
    }
}

#[inline]
unsafe fn buddy_merge_likely(pfn: ULong, buddy_pfn: ULong, page: *mut page, order: u32) -> bool {
    if order >= RUST_PA_B_MAX_PAGE_ORDER - 1 {
        return false;
    }
    let higher_pfn = buddy_pfn & pfn;
    let higher = page.wrapping_offset(higher_pfn.wrapping_sub(pfn) as isize);
    !rust_pa_b_find_buddy(higher, higher_pfn, order + 1, core::ptr::null_mut()).is_null()
}

unsafe fn change_pageblock_range(
    mut pageblock_page: *mut page,
    start_order: i32,
    migratetype: i32,
) {
    let mut nr = 1i32 << (start_order - rust_pa_pageblock_order() as i32);
    while nr != 0 {
        set_pageblock_migratetype(pageblock_page, migratetype);
        pageblock_page = pageblock_page.add(rust_pa_pageblock_nr_pages() as usize);
        nr -= 1;
    }
}

#[inline]
unsafe fn __free_one_page(
    mut page: *mut page,
    mut pfn: ULong,
    zone: *mut zone,
    mut order: u32,
    migratetype: i32,
    fpi_flags: i32,
) {
    let capc = task_capc(zone);
    let mut buddy_pfn = 0;
    pa_vm_bug!(!rust_pa_b_zone_initialized(zone));
    pa_vm_bug_page!(
        rust_pa_b_page_flags(page) & rust_pa_b_page_flags_check_at_prep() != 0,
        page,
    );
    pa_vm_bug!(migratetype == -1);
    pa_vm_bug_page!(pfn & (((1 as ULong) << order) - 1) != 0, page);
    pa_vm_bug_page!(bad_range(zone, page), page);
    account_freepages(zone, 1i32 << order, migratetype);
    while order < RUST_PA_B_MAX_PAGE_ORDER {
        let mut buddy_mt = migratetype;
        if compaction_capture(capc, page, order as i32, migratetype) {
            account_freepages(zone, -(1i32 << order), migratetype);
            return;
        }
        let buddy = rust_pa_b_find_buddy(page, pfn, order, &mut buddy_pfn);
        if buddy.is_null() {
            break;
        }
        if order >= rust_pa_pageblock_order() {
            buddy_mt = rust_pa_b_get_pfnblock_mt(buddy, buddy_pfn);
            if migratetype != buddy_mt
                && (!rust_pa_b_mt_mergeable(migratetype) || !rust_pa_b_mt_mergeable(buddy_mt))
            {
                break;
            }
        }
        if rust_pa_b_page_is_guard(buddy) {
            rust_pa_b_clear_guard(zone, buddy, order);
        } else {
            __del_page_from_free_list(buddy, zone, order, buddy_mt);
        }
        if buddy_mt != migratetype {
            change_pageblock_range(buddy, order as i32, migratetype);
        }
        let combined_pfn = buddy_pfn & pfn;
        page = page.wrapping_offset(combined_pfn.wrapping_sub(pfn) as isize);
        pfn = combined_pfn;
        order += 1;
    }
    set_buddy_order(page, order);
    let to_tail = if fpi_flags & RUST_PA_B_FPI_TO_TAIL as i32 != 0 {
        true
    } else if rust_pa_b_is_shuffle_order(order) {
        rust_pa_b_shuffle_pick_tail()
    } else {
        buddy_merge_likely(pfn, buddy_pfn, page, order)
    };
    __add_to_free_list(page, zone, order, migratetype, to_tail);
    if fpi_flags & RUST_PA_B_FPI_SKIP_REPORT_NOTIFY as i32 == 0 {
        rust_pa_b_reporting_notify(order);
    }
}

#[inline]
unsafe fn page_expected_state(page: *mut page, check_flags: ULong) -> bool {
    if rust_pa_b_mapcount(page) != -1 {
        return false;
    }
    let state = rust_pa_b_mapping(page) as ULong
        | rust_pa_b_page_ref_count(page) as ULong
        | rust_pa_b_memcg_data(page)
        | rust_pa_b_pool_page_is_pp(page) as ULong
        | (rust_pa_b_page_flags(page) & check_flags);
    state == 0
}

unsafe fn page_bad_reason(page: *mut page, flags: ULong) -> *const CChar {
    let mut reason = core::ptr::null();
    if rust_pa_b_mapcount(page) != -1 {
        reason = b"nonzero mapcount\0".as_ptr().cast();
    }
    if !rust_pa_b_mapping(page).is_null() {
        reason = b"non-NULL mapping\0".as_ptr().cast();
    }
    if rust_pa_b_page_ref_count(page) != 0 {
        reason = b"nonzero _refcount\0".as_ptr().cast();
    }
    if rust_pa_b_page_flags(page) & flags != 0 {
        reason = if flags == rust_pa_b_page_flags_check_at_prep() {
            b"PAGE_FLAGS_CHECK_AT_PREP flag(s) set\0".as_ptr().cast()
        } else {
            b"PAGE_FLAGS_CHECK_AT_FREE flag(s) set\0".as_ptr().cast()
        };
    }
    if rust_pa_b_memcg_data(page) != 0 {
        reason = b"page still charged to cgroup\0".as_ptr().cast();
    }
    if rust_pa_b_pool_page_is_pp(page) {
        reason = b"page_pool leak\0".as_ptr().cast();
    }
    reason
}

#[inline]
unsafe fn free_page_is_bad(page: *mut page) -> bool {
    if page_expected_state(page, RUST_PA_B_PAGE_FLAGS_CHECK_AT_FREE) {
        return false;
    }
    bad_page(
        page,
        page_bad_reason(page, RUST_PA_B_PAGE_FLAGS_CHECK_AT_FREE),
    );
    true
}

#[inline]
unsafe fn is_check_pages_enabled() -> bool {
    rust_pa_b_check_pages_enabled()
}

unsafe fn free_tail_page_prepare(head_page: *mut page, page: *mut page) -> i32 {
    let folio = head_page.cast::<folio>();
    let mut ret = 1;
    rust_pa_b_assert_tail_poison_alignment();
    // The labelled block preserves every original goto-out cleanup edge.
    'check: {
        if !is_check_pages_enabled() {
            ret = 0;
            break 'check;
        }
        match page.offset_from(head_page) {
            1 => {
                if rust_pa_b_folio_large_mapcount(folio) != 0 {
                    bad_page(page, b"nonzero large_mapcount\0".as_ptr().cast());
                    break 'check;
                }
                if cfg!(CONFIG_PAGE_MAPCOUNT) && rust_pa_b_folio_nr_mapped(folio) != 0 {
                    bad_page(page, b"nonzero nr_pages_mapped\0".as_ptr().cast());
                    break 'check;
                }
                if cfg!(CONFIG_MM_ID) {
                    if rust_pa_b_folio_mm_mapcount(folio, 0) != -1 {
                        bad_page(page, b"nonzero mm mapcount 0\0".as_ptr().cast());
                        break 'check;
                    }
                    if rust_pa_b_folio_mm_mapcount(folio, 1) != -1 {
                        bad_page(page, b"nonzero mm mapcount 1\0".as_ptr().cast());
                        break 'check;
                    }
                }
                if cfg!(CONFIG_64BIT) {
                    if rust_pa_b_folio_entire_mapcount(folio).wrapping_add(1) != 0 {
                        bad_page(page, b"nonzero entire_mapcount\0".as_ptr().cast());
                        break 'check;
                    }
                    if rust_pa_b_folio_pincount(folio) != 0 {
                        bad_page(page, b"nonzero pincount\0".as_ptr().cast());
                        break 'check;
                    }
                }
            }
            2 => {
                if !rust_pa_b_folio_deferred_empty(folio) {
                    bad_page(page, b"on deferred list\0".as_ptr().cast());
                    break 'check;
                }
                if !cfg!(CONFIG_64BIT) {
                    if rust_pa_b_folio_entire_mapcount(folio).wrapping_add(1) != 0 {
                        bad_page(page, b"nonzero entire_mapcount\0".as_ptr().cast());
                        break 'check;
                    }
                    if rust_pa_b_folio_pincount(folio) != 0 {
                        bad_page(page, b"nonzero pincount\0".as_ptr().cast());
                        break 'check;
                    }
                }
            }
            3 if cfg!(CONFIG_HUGETLB_PAGE) => {}
            _ => {
                if rust_pa_b_mapping(page) as ULong != RUST_PA_B_TAIL_MAPPING {
                    bad_page(page, b"corrupted mapping in tail page\0".as_ptr().cast());
                    break 'check;
                }
            }
        }
        if !rust_pa_b_page_tail(page) {
            bad_page(page, b"PageTail not set\0".as_ptr().cast());
            break 'check;
        }
        if rust_pa_b_compound_head(page) != head_page {
            bad_page(page, b"compound_head not consistent\0".as_ptr().cast());
            break 'check;
        }
        ret = 0;
    }
    rust_pa_b_clear_mapping(page);
    rust_pa_b_clear_compound_head(page);
    ret
}

#[inline]
unsafe fn should_skip_kasan_poison(page: *mut page) -> bool {
    if cfg!(CONFIG_KASAN_GENERIC) {
        return rust_pa_b_deferred_pages_enabled();
    }
    rust_pa_b_page_kasan_tag(page) as u32 == RUST_PA_B_KASAN_TAG_KERNEL
}

unsafe fn clear_highpages_kasan_tagged(page: *mut page, numpages: i32) {
    rust_pa_b_kasan_disable_current();
    if !cfg!(CONFIG_HIGHMEM) {
        rust_pa_b_clear_low_pages(page, numpages);
    } else {
        for i in 0..numpages {
            rust_pa_b_clear_highpage(page.add(i as usize));
        }
    }
    rust_pa_b_kasan_enable_current();
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[no_mangle]
pub unsafe extern "C" fn __clear_page_tag_ref(page: *mut page) {
    let mut handle = core::mem::MaybeUninit::<pgtag_ref_handle>::uninit();
    let mut tagref = core::mem::MaybeUninit::<codetag_ref>::uninit();
    if rust_pa_b_get_page_tag_ref(page, tagref.as_mut_ptr(), handle.as_mut_ptr()) {
        rust_pa_b_set_codetag_empty(tagref.as_mut_ptr());
        rust_pa_b_update_page_tag_ref(handle.assume_init(), tagref.as_mut_ptr());
        rust_pa_b_put_page_tag_ref(handle.assume_init());
    }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline(never)]
unsafe fn __pgalloc_tag_add(page: *mut page, task: *mut task_struct, nr: u32, alloc_flags: u32) {
    let mut handle = core::mem::MaybeUninit::<pgtag_ref_handle>::uninit();
    let mut tagref = core::mem::MaybeUninit::<codetag_ref>::uninit();
    if rust_pa_b_get_page_tag_ref(page, tagref.as_mut_ptr(), handle.as_mut_ptr()) {
        rust_pa_b_alloc_tag_add(
            tagref.as_mut_ptr(),
            rust_pa_b_task_alloc_tag(task),
            RUST_PA_B_PAGE_SIZE * nr as ULong,
        );
        rust_pa_b_update_page_tag_ref(handle.assume_init(), tagref.as_mut_ptr());
        rust_pa_b_put_page_tag_ref(handle.assume_init());
    } else {
        rust_pa_b_alloc_tag_add_early_pfn(rust_pa_page_to_pfn(page), alloc_flags);
        let tag = rust_pa_b_task_alloc_tag(task);
        if !tag.is_null() {
            rust_pa_b_alloc_tag_inaccurate(tag);
        }
    }
}

#[inline]
unsafe fn pgalloc_tag_add(page: *mut page, task: *mut task_struct, nr: u32, alloc_flags: u32) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    if rust_pa_b_profiling_enabled() {
        __pgalloc_tag_add(page, task, nr, alloc_flags);
    }
    #[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
    let _ = (page, task, nr, alloc_flags);
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline(never)]
unsafe fn __pgalloc_tag_sub(page: *mut page, nr: u32) {
    let mut handle = core::mem::MaybeUninit::<pgtag_ref_handle>::uninit();
    let mut tagref = core::mem::MaybeUninit::<codetag_ref>::uninit();
    if rust_pa_b_get_page_tag_ref(page, tagref.as_mut_ptr(), handle.as_mut_ptr()) {
        rust_pa_b_alloc_tag_sub(tagref.as_mut_ptr(), RUST_PA_B_PAGE_SIZE * nr as ULong);
        rust_pa_b_update_page_tag_ref(handle.assume_init(), tagref.as_mut_ptr());
        rust_pa_b_put_page_tag_ref(handle.assume_init());
    }
}

#[inline]
unsafe fn pgalloc_tag_sub(page: *mut page, nr: u32) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    if rust_pa_b_profiling_enabled() {
        __pgalloc_tag_sub(page, nr);
    }
    #[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
    let _ = (page, nr);
}

#[inline]
unsafe fn pgalloc_tag_sub_pages(tag: *mut alloc_tag, nr: u32) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    if !tag.is_null() {
        rust_pa_b_tag_counter_sub(tag, RUST_PA_B_PAGE_SIZE * nr as ULong);
    }
    #[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
    let _ = (tag, nr);
}

#[inline(always)]
unsafe fn __free_pages_prepare(page: *mut page, order: u32, fpi_flags: i32) -> bool {
    let mut bad = 0;
    let skip_kasan_poison = should_skip_kasan_poison(page);
    let mut init = rust_pa_b_want_init_on_free();
    let compound = rust_pa_b_page_compound(page);
    let folio = rust_pa_b_page_folio(page);
    if fpi_flags & RUST_PA_B_FPI_PREPARED as i32 != 0 {
        return true;
    }
    pa_vm_bug_page!(rust_pa_b_page_tail(page), page);
    rust_pa_b_trace_free(page, order);
    rust_pa_b_kmsan_free_page(page, order);
    if rust_pa_b_memcg_kmem_online() && rust_pa_b_page_memcg_kmem(page) {
        rust_pa_b_memcg_kmem_uncharge_page(page, order);
    }
    if rust_pa_b_folio_mlocked(folio) {
        let nr_pages = rust_pa_b_folio_nr_pages(folio);
        rust_pa_b_folio_clear_mlocked(folio);
        rust_pa_b_zone_stat_mod_folio(folio, NR_MLOCK, -nr_pages);
        rust_pa_b_count_vm_events(UNEVICTABLE_PGCLEARED, nr_pages);
    }
    if rust_pa_b_page_hwpoison(page) && order == 0 {
        rust_pa_b_reset_page_owner(page, order);
        rust_pa_b_page_table_check_free(page, order);
        pgalloc_tag_sub(page, 1 << order);
        rust_pa_b_clear_page_tag_ref(page);
        return false;
    }
    pa_vm_bug_page!(compound && rust_pa_b_compound_order(page) != order, page);
    if order != 0 {
        if compound {
            let second = page.add(1);
            rust_pa_b_write_page_flags(
                second,
                rust_pa_b_page_flags(second) & !RUST_PA_B_PAGE_FLAGS_SECOND,
            );
            rust_pa_b_clear_folio_nr_pages(folio);
        }
        for i in 1..(1u32 << order) {
            let tail_page = page.add(i as usize);
            if compound {
                bad += free_tail_page_prepare(page, tail_page);
            }
            if is_check_pages_enabled() {
                if free_page_is_bad(tail_page) {
                    bad += 1;
                    continue;
                }
                if page_private(tail_page) != 0 {
                    bad_page(tail_page, b"nonzero private\0".as_ptr().cast());
                    bad += 1;
                    continue;
                }
            }
            rust_pa_b_write_page_flags(
                tail_page,
                rust_pa_b_page_flags(tail_page) & !rust_pa_b_page_flags_check_at_prep(),
            );
        }
    }
    if rust_pa_b_folio_anon(folio) {
        rust_pa_b_mod_mthp_anon(order as i32, -1);
        rust_pa_b_clear_folio_mapping(folio);
    }
    if rust_pa_b_page_has_type(page) {
        rust_pa_b_reset_page_type(page);
    }
    if is_check_pages_enabled() {
        if free_page_is_bad(page) {
            bad += 1;
        }
        if bad != 0 {
            return false;
        }
    }
    rust_pa_b_page_cpupid_reset(page);
    rust_pa_b_write_page_flags(
        page,
        rust_pa_b_page_flags(page) & !rust_pa_b_page_flags_check_at_prep(),
    );
    set_page_private(page, 0);
    rust_pa_b_reset_page_owner(page, order);
    rust_pa_b_page_table_check_free(page, order);
    pgalloc_tag_sub(page, 1 << order);
    if !rust_pa_b_page_highmem(page) && fpi_flags & RUST_PA_B_FPI_NOLOCK as i32 == 0 {
        rust_pa_b_debug_check_no_locks_freed(
            rust_pa_b_page_address(page),
            RUST_PA_B_PAGE_SIZE << order,
        );
        rust_pa_b_debug_check_no_obj_freed(
            rust_pa_b_page_address(page),
            RUST_PA_B_PAGE_SIZE << order,
        );
    }
    rust_pa_b_kernel_poison_pages(page, 1 << order);
    if !skip_kasan_poison {
        rust_pa_b_kasan_poison_pages(page, order, init);
        if rust_pa_b_kasan_integrated_init() {
            init = false;
        }
    }
    if init {
        clear_highpages_kasan_tagged(page, 1 << order);
    }
    // Nothing touching page contents may follow arch_free_page (e.g. s390).
    rust_pa_b_arch_free_page(page, order);
    rust_pa_b_debug_unmap_pages(page, 1 << order);
    true
}

#[no_mangle]
pub unsafe extern "C" fn free_pages_prepare(page: *mut page, order: u32) -> bool {
    __free_pages_prepare(page, order, 0)
}

unsafe fn free_pcppages_bulk(
    zone: *mut zone,
    mut count: i32,
    pcp: *mut per_cpu_pages,
    mut pindex: i32,
) {
    count = core::cmp::min((*pcp).count, count);
    pindex -= 1;
    let flags = rust_pa_b_zone_lock_irqsave(zone);
    while count > 0 {
        let mut list;
        loop {
            pindex += 1;
            if pindex > RUST_PA_B_NR_PCP_LISTS as i32 - 1 {
                pindex = 0;
            }
            list = core::ptr::addr_of_mut!((*pcp).lists[pindex as usize]);
            if !list_empty(list) {
                break;
            }
        }
        let order = pindex_to_order(pindex as u32) as u32;
        let nr_pages = 1i32 << order;
        loop {
            let page = rust_pa_b_page_from_pcp((*list).prev);
            let pfn = rust_pa_page_to_pfn(page);
            let mt = rust_pa_b_get_pfnblock_mt(page, pfn);
            list_del(page_pcp_list(page));
            count -= nr_pages;
            (*pcp).count -= nr_pages;
            __free_one_page(page, pfn, zone, order, mt, 0);
            rust_pa_b_trace_pcpu_drain(page, order, mt);
            if count <= 0 || list_empty(list) {
                break;
            }
        }
    }
    rust_pa_b_zone_unlock_irqrestore(zone, flags);
}

unsafe fn split_large_buddy(
    zone: *mut zone,
    mut page: *mut page,
    mut pfn: ULong,
    mut order: i32,
    fpi: i32,
) {
    let end = pfn + ((1 as ULong) << order);
    rust_pa_b_warn_split_alignment(pfn, order as u32);
    rust_pa_b_warn_split_buddy(page);
    if order > rust_pa_pageblock_order() as i32 {
        order = rust_pa_pageblock_order() as i32;
    }
    loop {
        let mt = rust_pa_b_get_pfnblock_mt(page, pfn);
        __free_one_page(page, pfn, zone, order as u32, mt, fpi);
        pfn += (1 as ULong) << order;
        if pfn == end {
            break;
        }
        page = rust_pa_b_pfn_to_page(pfn);
    }
}

unsafe fn add_page_to_zone_llist(zone: *mut zone, page: *mut page, order: u32) {
    set_page_private(page, order as ULong);
    rust_pa_b_llist_add(
        rust_pa_b_page_pcp_llist(page),
        core::ptr::addr_of_mut!((*zone).trylock_free_pages),
    );
}

unsafe fn free_one_page(zone: *mut zone, page: *mut page, pfn: ULong, order: u32, fpi_flags: i32) {
    let mut flags = 0;
    if fpi_flags & RUST_PA_B_FPI_NOLOCK as i32 != 0 {
        if !rust_pa_b_can_spin_trylock() || !rust_pa_b_zone_trylock_irqsave(zone, &mut flags) {
            add_page_to_zone_llist(zone, page, order);
            return;
        }
    } else {
        flags = rust_pa_b_zone_lock_irqsave(zone);
    }
    let llhead = core::ptr::addr_of_mut!((*zone).trylock_free_pages);
    if !rust_pa_b_llist_empty(llhead) && fpi_flags & RUST_PA_B_FPI_NOLOCK as i32 == 0 {
        let mut node = rust_pa_b_llist_del_all(llhead);
        while !node.is_null() {
            let next = (*node).next;
            let p = rust_pa_b_page_from_llist(node);
            let p_order = page_private(p) as u32;
            split_large_buddy(zone, p, rust_pa_page_to_pfn(p), p_order as i32, fpi_flags);
            rust_pa_b_count_vm_events_local(PGFREE, (1 as ULong) << p_order);
            node = next;
        }
    }
    split_large_buddy(zone, page, pfn, order as i32, fpi_flags);
    rust_pa_b_zone_unlock_irqrestore(zone, flags);
    rust_pa_b_count_vm_events_local(PGFREE, (1 as ULong) << order);
}

unsafe fn __free_pages_ok(page: *mut page, order: u32, fpi_flags: i32) {
    let pfn = rust_pa_page_to_pfn(page);
    let zone = rust_pa_page_zone(page);
    if __free_pages_prepare(page, order, fpi_flags) {
        free_one_page(zone, page, pfn, order, fpi_flags);
    }
}

#[export_name = "rust_pa_impl_free_pages_core"]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
pub unsafe extern "C" fn __free_pages_core(page: *mut page, order: u32, context: meminit_context) {
    let nr_pages = 1u32 << order;
    if cfg!(CONFIG_MEMORY_HOTPLUG) && context == MEMINIT_HOTPLUG {
        for i in 0..nr_pages {
            let p = page.add(i as usize);
            rust_pa_b_warn_core_reserved(p);
            rust_pa_b_clear_offline(p);
            rust_pa_b_set_page_count(p, 0);
        }
        adjust_managed_page_count(page, nr_pages as Long);
    } else {
        for i in 0..nr_pages {
            let p = page.add(i as usize);
            rust_pa_b_clear_reserved(p);
            rust_pa_b_set_page_count(p, 0);
        }
        rust_pa_b_managed_pages_add(rust_pa_page_zone(page), nr_pages as Long);
    }
    if page_contains_unaccepted(page, order) {
        if order == RUST_PA_B_MAX_PAGE_ORDER && __free_unaccepted(page) {
            return;
        }
        rust_pa_b_accept_memory(rust_pa_b_page_to_phys(page), RUST_PA_B_PAGE_SIZE << order);
    }
    __free_pages_ok(page, order, RUST_PA_B_FPI_TO_TAIL as i32);
}

#[no_mangle]
pub unsafe extern "C" fn __pageblock_pfn_to_page(
    start_pfn: ULong,
    end_pfn: ULong,
    zone: *mut zone,
) -> *mut page {
    let end_pfn = end_pfn.wrapping_sub(1);
    if !rust_pa_b_pfn_valid(end_pfn) {
        return core::ptr::null_mut();
    }
    let start = rust_pa_b_pfn_to_online_page(start_pfn);
    if start.is_null() || rust_pa_page_zone(start) != zone {
        return core::ptr::null_mut();
    }
    let end = rust_pa_b_pfn_to_page(end_pfn);
    if rust_pa_b_page_zone_id(start) != rust_pa_b_page_zone_id(end) {
        return core::ptr::null_mut();
    }
    start
}

#[inline]
unsafe fn expand(
    zone: *mut zone,
    page: *mut page,
    low: i32,
    mut high: i32,
    migratetype: i32,
) -> u32 {
    let mut size = 1u32 << high;
    let mut nr_added = 0;
    while high > low {
        high -= 1;
        size >>= 1;
        let split = page.add(size as usize);
        pa_vm_bug_page!(bad_range(zone, split), split);
        if rust_pa_b_set_guard(zone, split, high as u32) {
            continue;
        }
        __add_to_free_list(split, zone, high as u32, migratetype, false);
        set_buddy_order(split, high as u32);
        nr_added += size;
    }
    nr_added
}

#[inline(always)]
unsafe fn page_del_and_expand(
    zone: *mut zone,
    page: *mut page,
    low: i32,
    high: i32,
    migratetype: i32,
) {
    __del_page_from_free_list(page, zone, high as u32, migratetype);
    let nr_pages = (1u32 << high) - expand(zone, page, low, high, migratetype);
    account_freepages(zone, -(nr_pages as i32), migratetype);
}

unsafe fn check_new_page_bad(page: *mut page) {
    if rust_pa_b_page_hwpoison(page) {
        if rust_pa_b_page_buddy(page) {
            rust_pa_b_clear_buddy(page);
        }
        return;
    }
    bad_page(
        page,
        page_bad_reason(page, rust_pa_b_page_flags_check_at_prep()),
    );
}

unsafe fn check_new_page(page: *mut page) -> bool {
    if page_expected_state(
        page,
        rust_pa_b_page_flags_check_at_prep() | RUST_PA_B_PG_HWPOISON,
    ) {
        return false;
    }
    check_new_page_bad(page);
    true
}

#[inline]
unsafe fn check_new_pages(page: *mut page, order: u32) -> bool {
    if is_check_pages_enabled() {
        for i in 0..(1u32 << order) {
            if check_new_page(page.add(i as usize)) {
                return true;
            }
        }
    }
    false
}

#[inline]
unsafe fn should_skip_kasan_unpoison(flags: gfp_t) -> bool {
    if cfg!(CONFIG_KASAN_GENERIC) || cfg!(CONFIG_KASAN_SW_TAGS) {
        return false;
    }
    if !rust_pa_b_kasan_hw_tags_enabled() {
        return true;
    }
    flags & RUST_PA_B_GFP_SKIP_KASAN != 0
}

#[inline]
unsafe fn should_skip_init(flags: gfp_t) -> bool {
    if !rust_pa_b_kasan_hw_tags_enabled() {
        return false;
    }
    flags & RUST_PA_B_GFP_SKIP_ZERO != 0
}

#[no_mangle]
pub unsafe extern "C" fn post_alloc_hook(
    page: *mut page,
    order: u32,
    gfp_flags: gfp_t,
    alloc_flags: u32,
) {
    let zero_tags = gfp_flags & RUST_PA_B_GFP_ZEROTAGS != 0;
    let mut init = !rust_pa_b_want_init_on_free()
        && rust_pa_b_want_init_on_alloc(gfp_flags)
        && !should_skip_init(gfp_flags);
    set_page_private(page, 0);
    rust_pa_b_arch_alloc_page(page, order);
    rust_pa_b_debug_map_pages(page, 1 << order);
    rust_pa_b_kernel_unpoison_pages(page, 1 << order);
    if zero_tags {
        init = rust_pa_b_tag_clear_highpages(page, 1 << order, init);
    }
    if !should_skip_kasan_unpoison(gfp_flags) && rust_pa_b_kasan_unpoison_pages(page, order, init) {
        if rust_pa_b_kasan_integrated_init() {
            init = false;
        }
    } else {
        for i in 0..(1u32 << order) {
            rust_pa_b_page_kasan_tag_reset(page.add(i as usize));
        }
    }
    if init {
        clear_highpages_kasan_tagged(page, 1 << order);
    }
    rust_pa_b_set_page_owner(page, order, gfp_flags);
    rust_pa_b_page_table_check_alloc(page, order);
    pgalloc_tag_add(page, rust_pa_b_current(), 1 << order, alloc_flags);
}

unsafe fn prep_new_page(page: *mut page, order: u32, gfp_flags: gfp_t, alloc_flags: u32) {
    post_alloc_hook(page, order, gfp_flags, alloc_flags);
    if order != 0 && gfp_flags & RUST_PA_B_GFP_COMP != 0 {
        prep_compound_page(page, order);
    }
    if alloc_flags & RUST_PA_B_ALLOC_NO_WATERMARKS != 0 {
        rust_pa_b_set_pfmemalloc(page);
    } else {
        rust_pa_b_clear_pfmemalloc(page);
    }
}

#[inline(always)]
unsafe fn __rmqueue_smallest(zone: *mut zone, order: u32, migratetype: i32) -> *mut page {
    for current_order in order..RUST_PA_B_NR_PAGE_ORDERS {
        let area = core::ptr::addr_of_mut!((*zone).free_area[current_order as usize]);
        let page = get_page_from_free_area(area, migratetype);
        if page.is_null() {
            continue;
        }
        page_del_and_expand(zone, page, order as i32, current_order as i32, migratetype);
        rust_pa_b_trace_zone_locked(
            page,
            order,
            migratetype,
            pcp_allowed_order(order) && migratetype < MIGRATE_PCPTYPES as i32,
        );
        return page;
    }
    core::ptr::null_mut()
}
