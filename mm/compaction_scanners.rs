// SPDX-License-Identifier: GPL-2.0
unsafe fn suitable_migration_source(cc: *mut compact_control, page: *mut page) -> bool {
    if pageblock_skip_persistent(page) {
        return false;
    }
    if !(*cc).direct_compaction {
        return true;
    }
    let block_mt = rust_compaction_get_pageblock_migratetype(page);
    if rust_compaction_is_migrate_cma(block_mt)
        && (*cc).alloc_flags & RUST_COMPACTION_ALLOC_CMA == 0
    {
        return false;
    }
    if (*cc).mode != MIGRATE_ASYNC {
        return true;
    }
    if (*cc).migratetype == MIGRATE_MOVABLE as c_int
        || (*cc).order >= rust_compaction_pageblock_order() as c_int
    {
        rust_compaction_is_migrate_movable(block_mt)
    } else {
        block_mt == (*cc).migratetype
    }
}
unsafe fn suitable_migration_target(cc: *mut compact_control, page: *mut page) -> bool {
    if rust_compaction_PageBuddy(page) {
        let order = if (*cc).order > 0 {
            (*cc).order
        } else {
            rust_compaction_pageblock_order() as c_int
        };
        if rust_compaction_buddy_order_unsafe(page) >= order as c_ulong {
            return false;
        }
    }
    (*cc).ignore_block_suitable
        || rust_compaction_is_migrate_movable(rust_compaction_get_pageblock_migratetype(page))
}
unsafe fn freelist_scan_limit(cc: *mut compact_control) -> c_uint {
    let shift = RUST_COMPACTION_BITS_PER_LONG.wrapping_sub(1) as u16;
    (RUST_COMPACTION_COMPACT_CLUSTER_MAX
        .wrapping_shr(min(shift, (*cc).fast_search_fail) as c_uint)
        .wrapping_add(1)) as c_uint
}
unsafe fn compact_scanners_met(cc: *mut compact_control) -> bool {
    (*cc)
        .free_pfn
        .wrapping_shr(rust_compaction_pageblock_order())
        <= (*cc)
            .migrate_pfn
            .wrapping_shr(rust_compaction_pageblock_order())
}
unsafe fn move_freelist_head(freelist: *mut list_head, freepage: *mut page) {
    let mut sublist: list_head = zeroed();
    list_init(&mut sublist);
    let entry = rust_compaction_page_buddy_list(freepage);
    if (*entry).prev != freelist {
        rust_compaction_list_cut_before(&mut sublist, freelist, entry);
        rust_compaction_list_splice_tail(&mut sublist, freelist);
    }
}
unsafe fn move_freelist_tail(freelist: *mut list_head, freepage: *mut page) {
    let mut sublist: list_head = zeroed();
    list_init(&mut sublist);
    let entry = rust_compaction_page_buddy_list(freepage);
    if (*entry).next != freelist {
        rust_compaction_list_cut_position(&mut sublist, freelist, entry);
        rust_compaction_list_splice_tail(&mut sublist, freelist);
    }
}
unsafe fn fast_isolate_around(cc: *mut compact_control, pfn: c_ulong) {
    if (*cc).nr_freepages >= (*cc).nr_migratepages
        || ((*cc).direct_compaction && (*cc).mode == MIGRATE_ASYNC)
    {
        return;
    }
    let mut start_pfn = max(pageblock_start_pfn(pfn), (*(*cc).zone).zone_start_pfn);
    let end_pfn = min(
        pageblock_end_pfn(pfn),
        rust_compaction_zone_end_pfn((*cc).zone),
    );
    let page = rust_compaction_pageblock_pfn_to_page(start_pfn, end_pfn, (*cc).zone);
    if page.is_null() {
        return;
    }
    isolate_freepages_block(
        cc,
        &mut start_pfn,
        end_pfn,
        (*cc).freepages.as_mut_ptr(),
        1,
        false,
    );
    if start_pfn == end_pfn && !(*cc).no_set_skip_hint {
        rust_compaction_set_pageblock_skip(page);
    }
}
unsafe fn next_search_order(cc: *mut compact_control, mut order: c_int) -> c_int {
    order = order.wrapping_sub(1);
    if order < 0 {
        order = (*cc).order.wrapping_sub(1);
    }
    if order == (*cc).search_order as c_int {
        (*cc).search_order = (*cc).search_order.wrapping_sub(1);
        if (*cc).search_order < 0 {
            (*cc).search_order = (*cc).order.wrapping_sub(1) as _;
        }
        return -1;
    }
    order
}
unsafe fn fast_isolate_freepages(cc: *mut compact_control) {
    let mut limit = max(1, freelist_scan_limit(cc) >> 1);
    let mut nr_scanned: c_uint = 0;
    let mut total_isolated: c_uint = 0;
    let mut highest = 0;
    let mut page: *mut page = null_mut();
    let mut scan_start = false;
    if (*cc).order <= 0 {
        return;
    }
    if (*cc).free_pfn >= (*(*cc).zone).compact_init_free_pfn {
        limit = (pageblock_nr_pages() >> 1) as c_uint;
        scan_start = true;
    }
    let distance = (*cc).free_pfn.wrapping_sub((*cc).migrate_pfn);
    let mut low_pfn = pageblock_start_pfn((*cc).free_pfn.wrapping_sub(distance >> 2));
    let min_pfn = pageblock_start_pfn((*cc).free_pfn.wrapping_sub(distance >> 1));
    if rust_compaction_warn_min_pfn(min_pfn > low_pfn) {
        low_pfn = min_pfn;
    }
    (*cc).search_order = min(
        (*cc).order.wrapping_sub(1) as c_uint,
        (*cc).search_order as c_uint,
    ) as _;
    let mut order = (*cc).search_order as c_int;
    while page.is_null() && order >= 0 {
        let area = addr_of_mut!((*(*cc).zone).free_area[order as usize]);
        let mut order_scanned: c_uint = 0;
        let mut high_pfn = 0;
        if (*area).nr_free == 0 {
            order = next_search_order(cc, order);
            continue;
        }
        let mut flags = 0;
        rust_compaction_spin_lock_irqsave(addr_of_mut!((*(*cc).zone).lock), &mut flags);
        let freelist = addr_of_mut!((*area).free_list[MIGRATE_MOVABLE as usize]);
        let mut entry = (*freelist).prev;
        let mut freepage = rust_compaction_buddy_list_page(entry);
        while entry != freelist {
            freepage = rust_compaction_buddy_list_page(entry);
            order_scanned = order_scanned.wrapping_add(1);
            nr_scanned = nr_scanned.wrapping_add(1);
            let pfn = rust_compaction_page_to_pfn(freepage);
            if pfn >= highest {
                highest = max(pageblock_start_pfn(pfn), (*(*cc).zone).zone_start_pfn);
            }
            if pfn >= low_pfn {
                (*cc).fast_search_fail = 0;
                (*cc).search_order = order as _;
                page = freepage;
                break;
            }
            if pfn >= min_pfn && pfn > high_pfn {
                high_pfn = pfn;
                limit >>= 1;
            }
            if order_scanned >= limit {
                break;
            }
            entry = (*entry).prev;
            freepage = rust_compaction_buddy_list_page(entry);
        }
        if page.is_null() && high_pfn != 0 {
            page = rust_compaction_pfn_to_page(high_pfn);
            freepage = page;
        }
        move_freelist_head(freelist, freepage);
        if !page.is_null() {
            if __isolate_free_page(page, order as c_uint) != 0 {
                let nr_isolated = (1 as c_int).wrapping_shl(order as c_uint) as c_ulong;
                nr_scanned = nr_scanned.wrapping_add(nr_isolated.wrapping_sub(1) as c_uint);
                total_isolated = total_isolated.wrapping_add(nr_isolated as c_uint);
                (*cc).nr_freepages = (*cc).nr_freepages.wrapping_add(nr_isolated as c_uint);
                list_add_tail(
                    rust_compaction_page_lru(page),
                    addr_of_mut!((*cc).freepages[order as usize]),
                );
                count_compact_events!(COMPACTISOLATED, nr_isolated);
            } else {
                order = ((*cc).search_order as c_int).wrapping_add(1);
                page = null_mut();
            }
        }
        rust_compaction_spin_unlock_irqrestore(addr_of_mut!((*(*cc).zone).lock), flags);
        if (*cc).nr_freepages >= (*cc).nr_migratepages {
            break;
        }
        if order_scanned >= limit {
            limit = max(1, limit >> 1);
        }
        order = next_search_order(cc, order);
    }
    rust_compaction_trace_fast_isolate_freepages(
        min_pfn,
        (*cc).free_pfn,
        nr_scanned as c_ulong,
        total_isolated as c_ulong,
    );
    if page.is_null() {
        (*cc).fast_search_fail = (*cc).fast_search_fail.wrapping_add(1);
        if scan_start {
            if highest >= min_pfn {
                page = rust_compaction_pfn_to_page(highest);
                (*cc).free_pfn = highest;
            } else if (*cc).direct_compaction && rust_compaction_pfn_valid(min_pfn) {
                page = rust_compaction_pageblock_pfn_to_page(
                    min_pfn,
                    min(
                        pageblock_end_pfn(min_pfn),
                        rust_compaction_zone_end_pfn((*cc).zone),
                    ),
                    (*cc).zone,
                );
                if !page.is_null() && !suitable_migration_target(cc, page) {
                    page = null_mut();
                }
                (*cc).free_pfn = min_pfn;
            }
        }
    }
    if highest != 0 && highest >= (*(*cc).zone).compact_cached_free_pfn {
        highest = highest.wrapping_sub(pageblock_nr_pages());
        (*(*cc).zone).compact_cached_free_pfn = highest;
    }
    (*cc).total_free_scanned = (*cc).total_free_scanned.wrapping_add(nr_scanned as c_ulong);
    if page.is_null() {
        return;
    }
    low_pfn = rust_compaction_page_to_pfn(page);
    fast_isolate_around(cc, low_pfn);
}
unsafe fn isolate_freepages(cc: *mut compact_control) {
    let zone = (*cc).zone;
    fast_isolate_freepages(cc);
    if (*cc).nr_freepages != 0 {
        return;
    }
    let mut isolate_start = (*cc).free_pfn;
    let mut block_start = pageblock_start_pfn(isolate_start);
    let mut block_end = min(
        block_start.wrapping_add(pageblock_nr_pages()),
        rust_compaction_zone_end_pfn(zone),
    );
    let low_pfn = pageblock_end_pfn((*cc).migrate_pfn);
    let mut stride = if (*cc).mode == MIGRATE_ASYNC {
        RUST_COMPACTION_COMPACT_CLUSTER_MAX as c_uint
    } else {
        1
    };
    while block_start >= low_pfn {
        if block_start % RUST_COMPACTION_COMPACT_CLUSTER_MAX.wrapping_mul(pageblock_nr_pages()) == 0
        {
            rust_compaction_cond_resched();
        }
        let page = rust_compaction_pageblock_pfn_to_page(block_start, block_end, zone);
        if page.is_null() {
            let next_pfn = skip_offline_sections_reverse(block_start);
            if next_pfn != 0 {
                block_start = max(next_pfn, low_pfn);
            }
        } else if suitable_migration_target(cc, page) && isolation_suitable(cc, page) {
            let nr_isolated = isolate_freepages_block(
                cc,
                &mut isolate_start,
                block_end,
                (*cc).freepages.as_mut_ptr(),
                stride,
                false,
            );
            if isolate_start == block_end {
                update_pageblock_skip(cc, page, block_start.wrapping_sub(pageblock_nr_pages()));
            }
            if (*cc).nr_freepages >= (*cc).nr_migratepages {
                if isolate_start >= block_end {
                    isolate_start = block_start.wrapping_sub(pageblock_nr_pages());
                }
                break;
            } else if isolate_start < block_end {
                break;
            }
            if nr_isolated != 0 {
                stride = 1;
            } else {
                stride = min(
                    RUST_COMPACTION_COMPACT_CLUSTER_MAX as c_uint,
                    stride.wrapping_shl(1),
                );
            }
        }
        block_end = block_start;
        block_start = block_start.wrapping_sub(pageblock_nr_pages());
        isolate_start = block_start;
    }
    (*cc).free_pfn = isolate_start;
}
unsafe extern "C" fn compaction_alloc_noprof(src: *mut folio, data: c_ulong) -> *mut folio {
    let cc = data as *mut compact_control;
    let order = rust_compaction_folio_order(src) as c_int;
    let mut has_isolated_pages = false;
    let mut start_order;
    loop {
        start_order = order;
        while start_order < RUST_COMPACTION_NR_PAGE_ORDERS as c_int {
            if !list_empty(addr_of!((*cc).freepages[start_order as usize])) {
                break;
            }
            start_order = start_order.wrapping_add(1);
        }
        if start_order != RUST_COMPACTION_NR_PAGE_ORDERS as c_int {
            break;
        }
        if has_isolated_pages {
            return null_mut();
        }
        isolate_freepages(cc);
        has_isolated_pages = true;
    }
    let freepage = rust_compaction_lru_page((*cc).freepages[start_order as usize].next);
    let mut size = (1 as c_int).wrapping_shl(start_order as c_uint) as c_ulong;
    list_del(rust_compaction_page_lru(freepage));
    while start_order > order {
        start_order = start_order.wrapping_sub(1);
        size >>= 1;
        list_add(
            rust_compaction_page_lru(freepage.wrapping_add(size as usize)),
            addr_of_mut!((*cc).freepages[start_order as usize]),
        );
    }
    post_alloc_hook(
        freepage,
        order as c_uint,
        RUST_COMPACTION___GFP_MOVABLE,
        RUST_COMPACTION_ALLOC_DEFAULT,
    );
    rust_compaction_set_page_refcounted(freepage);
    if order != 0 {
        prep_compound_page(freepage, order as c_uint);
    }
    let nr = (1 as c_int).wrapping_shl(order as c_uint) as c_ulong;
    (*cc).nr_freepages = (*cc).nr_freepages.wrapping_sub(nr as c_uint);
    (*cc).nr_migratepages = (*cc).nr_migratepages.wrapping_sub(nr as c_uint);
    rust_compaction_page_rmappable_folio(freepage)
}
unsafe extern "C" fn compaction_alloc(src: *mut folio, data: c_ulong) -> *mut folio {
    rust_compaction_alloc_hooks(Some(compaction_alloc_noprof), src, data)
}
unsafe extern "C" fn compaction_free(dst: *mut folio, data: c_ulong) {
    let cc = data as *mut compact_control;
    let order = rust_compaction_folio_order(dst) as c_int;
    let page = rust_compaction_folio_page(dst);
    let nr = (1 as c_int).wrapping_shl(order as c_uint) as c_ulong;
    if rust_compaction_folio_put_testzero(dst) && free_pages_prepare(page, order as c_uint) {
        list_add(
            rust_compaction_folio_lru(dst),
            addr_of_mut!((*cc).freepages[order as usize]),
        );
        (*cc).nr_freepages = (*cc).nr_freepages.wrapping_add(nr as c_uint);
    }
    (*cc).nr_migratepages = (*cc).nr_migratepages.wrapping_add(nr as c_uint);
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum IsolateMigrate {
    Abort,
    None,
    Success,
}
unsafe fn update_fast_start_pfn(cc: *mut compact_control, pfn: c_ulong) {
    if (*cc).fast_start_pfn == c_ulong::MAX {
        return;
    }
    if (*cc).fast_start_pfn == 0 {
        (*cc).fast_start_pfn = pfn;
    }
    (*cc).fast_start_pfn = min((*cc).fast_start_pfn, pfn);
}
unsafe fn reinit_migrate_pfn(cc: *mut compact_control) -> c_ulong {
    if (*cc).fast_start_pfn == 0 || (*cc).fast_start_pfn == c_ulong::MAX {
        return (*cc).migrate_pfn;
    }
    (*cc).migrate_pfn = (*cc).fast_start_pfn;
    (*cc).fast_start_pfn = c_ulong::MAX;
    (*cc).migrate_pfn
}
unsafe fn fast_find_migrateblock(cc: *mut compact_control) -> c_ulong {
    let limit = freelist_scan_limit(cc);
    let mut nr_scanned: c_uint = 0;
    let mut pfn = (*cc).migrate_pfn;
    let mut found_block = false;
    if (*cc).ignore_skip_hint || (*cc).finish_pageblock {
        return pfn;
    }
    if pfn != (*(*cc).zone).zone_start_pfn && pfn != pageblock_start_pfn(pfn) {
        return pfn;
    }
    if (*cc).order <= RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER as c_int {
        return pfn;
    }
    if (*cc).direct_compaction
        && (*cc).migratetype != MIGRATE_MOVABLE as c_int
        && (*cc).order < rust_compaction_pageblock_order() as c_int
    {
        return pfn;
    }
    let mut distance = (*cc).free_pfn.wrapping_sub((*cc).migrate_pfn) >> 1;
    if (*cc).migrate_pfn != (*(*cc).zone).zone_start_pfn {
        distance >>= 2;
    }
    let high_pfn = pageblock_start_pfn((*cc).migrate_pfn.wrapping_add(distance));
    let mut order = (*cc).order.wrapping_sub(1);
    while order >= RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER as c_int
        && !found_block
        && nr_scanned < limit
    {
        let area = addr_of_mut!((*(*cc).zone).free_area[order as usize]);
        if (*area).nr_free != 0 {
            let mut flags = 0;
            rust_compaction_spin_lock_irqsave(addr_of_mut!((*(*cc).zone).lock), &mut flags);
            let freelist = addr_of_mut!((*area).free_list[MIGRATE_MOVABLE as usize]);
            let mut entry = (*freelist).next;
            while entry != freelist {
                let freepage = rust_compaction_buddy_list_page(entry);
                let before = nr_scanned;
                nr_scanned = nr_scanned.wrapping_add(1);
                if before >= limit {
                    move_freelist_tail(freelist, freepage);
                    break;
                }
                let free_pfn = rust_compaction_page_to_pfn(freepage);
                if free_pfn < high_pfn && !rust_compaction_get_pageblock_skip(freepage) {
                    move_freelist_tail(freelist, freepage);
                    update_fast_start_pfn(cc, free_pfn);
                    pfn = max(pageblock_start_pfn(free_pfn), (*(*cc).zone).zone_start_pfn);
                    (*cc).fast_search_fail = 0;
                    found_block = true;
                    break;
                }
                entry = (*entry).next;
            }
            rust_compaction_spin_unlock_irqrestore(addr_of_mut!((*(*cc).zone).lock), flags);
        }
        order = order.wrapping_sub(1);
    }
    (*cc).total_migrate_scanned = (*cc)
        .total_migrate_scanned
        .wrapping_add(nr_scanned as c_ulong);
    if !found_block {
        (*cc).fast_search_fail = (*cc).fast_search_fail.wrapping_add(1);
        pfn = reinit_migrate_pfn(cc);
    }
    pfn
}
unsafe fn isolate_migratepages(cc: *mut compact_control) -> IsolateMigrate {
    let isolate_mode = (if sysctl_compact_unevictable_allowed != 0 {
        RUST_COMPACTION_ISOLATE_UNEVICTABLE
    } else {
        0
    }) | (if (*cc).mode != MIGRATE_SYNC {
        RUST_COMPACTION_ISOLATE_ASYNC_MIGRATE
    } else {
        0
    });
    let mut low_pfn = fast_find_migrateblock(cc);
    let mut block_start = max(pageblock_start_pfn(low_pfn), (*(*cc).zone).zone_start_pfn);
    let mut fast_find_block = low_pfn != (*cc).migrate_pfn && (*cc).fast_search_fail == 0;
    let mut block_end = pageblock_end_pfn(low_pfn);
    while block_end <= (*cc).free_pfn {
        if low_pfn % RUST_COMPACTION_COMPACT_CLUSTER_MAX.wrapping_mul(pageblock_nr_pages()) == 0 {
            rust_compaction_cond_resched();
        }
        let page = rust_compaction_pageblock_pfn_to_page(block_start, block_end, (*cc).zone);
        if page.is_null() {
            let next_pfn = skip_offline_sections(block_start);
            if next_pfn != 0 {
                block_end = min(next_pfn, (*cc).free_pfn);
            }
        } else if !((low_pfn & pageblock_nr_pages().wrapping_sub(1) == 0
            || low_pfn == (*(*cc).zone).zone_start_pfn)
            && !fast_find_block
            && !isolation_suitable(cc, page))
        {
            if !suitable_migration_source(cc, page) {
                update_cached_migrate(cc, block_end);
            } else {
                if isolate_migratepages_block(cc, low_pfn, block_end, isolate_mode) != 0 {
                    return IsolateMigrate::Abort;
                }
                break;
            }
        }
        fast_find_block = false;
        low_pfn = block_end;
        (*cc).migrate_pfn = low_pfn;
        block_start = block_end;
        block_end = block_end.wrapping_add(pageblock_nr_pages());
    }
    if (*cc).nr_migratepages != 0 {
        IsolateMigrate::Success
    } else {
        IsolateMigrate::None
    }
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
