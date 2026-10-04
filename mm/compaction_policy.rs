// SPDX-License-Identifier: GPL-2.0
unsafe fn kswapd_is_running(pgdat: *mut pg_data_t) -> bool {
    rust_compaction_pgdat_kswapd_lock(pgdat);
    let running = !(*pgdat).kswapd.is_null() && rust_compaction_task_is_running((*pgdat).kswapd);
    rust_compaction_pgdat_kswapd_unlock(pgdat);
    running
}
unsafe fn fragmentation_score_zone(zone: *mut zone) -> c_uint {
    extfrag_for_order(zone, rust_compaction_hpage_order())
}
unsafe fn fragmentation_score_zone_weighted(zone: *mut zone) -> c_uint {
    let score = (*zone)
        .present_pages
        .wrapping_mul(fragmentation_score_zone(zone) as c_ulong);
    rust_compaction_div64_ul(
        score as u64,
        (*(*zone).zone_pgdat).node_present_pages.wrapping_add(1),
    ) as c_uint
}
unsafe fn fragmentation_score_node(pgdat: *mut pg_data_t) -> c_uint {
    let mut score: c_uint = 0;
    for zoneid in 0..MAX_NR_ZONES as usize {
        let zone = addr_of_mut!((*pgdat).node_zones[zoneid]);
        if rust_compaction_populated_zone(zone) {
            score = score.wrapping_add(fragmentation_score_zone_weighted(zone));
        }
    }
    score
}
unsafe fn fragmentation_score_wmark(low: bool) -> c_uint {
    let wmark_low = 100u32.wrapping_sub(sysctl_compaction_proactiveness);
    let leeway = min(10, wmark_low / 2);
    if low {
        wmark_low
    } else {
        min(wmark_low.wrapping_add(leeway), 100)
    }
}
unsafe fn should_proactive_compact_node(pgdat: *mut pg_data_t) -> bool {
    if sysctl_compaction_proactiveness == 0 || kswapd_is_running(pgdat) {
        return false;
    }
    let wmark_high = fragmentation_score_wmark(false) as c_int;
    fragmentation_score_node(pgdat) > wmark_high as c_uint
}
unsafe fn __compact_finished(cc: *mut compact_control) -> compact_result {
    let migratetype = (*cc).migratetype;
    if compact_scanners_met(cc) {
        reset_cached_positions((*cc).zone);
        if (*cc).direct_compaction {
            (*(*cc).zone).compact_blockskip_flush = true;
        }
        return if (*cc).whole_zone {
            COMPACT_COMPLETE
        } else {
            COMPACT_PARTIAL_SKIPPED
        };
    }
    let mut ret;
    if (*cc).proactive_compaction {
        let pgdat = (*(*cc).zone).zone_pgdat;
        if kswapd_is_running(pgdat) {
            return COMPACT_PARTIAL_SKIPPED;
        }
        let score = fragmentation_score_zone((*cc).zone) as c_int;
        let wmark_low = fragmentation_score_wmark(true) as c_int;
        ret = if score > wmark_low {
            COMPACT_CONTINUE
        } else {
            COMPACT_SUCCESS
        };
    } else {
        if is_via_compact_memory((*cc).order) {
            return COMPACT_CONTINUE;
        }
        if (*cc).migrate_pfn & pageblock_nr_pages().wrapping_sub(1) != 0 {
            return COMPACT_CONTINUE;
        }
        if defrag_mode != 0 && !(*cc).direct_compaction {
            return if rust_compaction_zone_watermark_ok(
                (*cc).zone,
                (*cc).order as c_uint,
                rust_compaction_high_wmark_pages((*cc).zone),
                (*cc).highest_zoneidx,
                (*cc).alloc_flags,
                rust_compaction_zone_page_state((*cc).zone, NR_FREE_PAGES_BLOCKS),
            ) {
                COMPACT_SUCCESS
            } else {
                COMPACT_CONTINUE
            };
        }
        ret = COMPACT_NO_SUITABLE_PAGE;
        for order in (*cc).order as c_uint..RUST_COMPACTION_NR_PAGE_ORDERS {
            let area = addr_of_mut!((*(*cc).zone).free_area[order as usize]);
            if !rust_compaction_free_area_empty(area, migratetype) {
                return COMPACT_SUCCESS;
            }
            #[cfg(CONFIG_CMA)]
            if migratetype == MIGRATE_MOVABLE as c_int
                && !rust_compaction_free_area_empty(area, MIGRATE_CMA as c_int)
            {
                return COMPACT_SUCCESS;
            }
            if find_suitable_fallback(area, order, migratetype, true, null_mut()) == FALLBACK_FOUND
            {
                return COMPACT_SUCCESS;
            }
        }
    }
    if (*cc).contended || rust_compaction_fatal_signal_pending(rust_compaction_current()) {
        ret = COMPACT_CONTENDED;
    }
    ret
}
unsafe fn compact_finished(cc: *mut compact_control) -> compact_result {
    let mut ret = __compact_finished(cc);
    rust_compaction_trace_finished((*cc).zone, (*cc).order, ret as c_int);
    if ret == COMPACT_NO_SUITABLE_PAGE {
        ret = COMPACT_CONTINUE;
    }
    ret
}
unsafe fn __compaction_suitable(
    zone: *mut zone,
    order: c_int,
    mut watermark: c_ulong,
    highest_zoneidx: c_int,
    free_pages: c_ulong,
) -> bool {
    watermark = watermark.wrapping_add(rust_compaction_compact_gap(order as c_uint));
    if order > RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER as c_int {
        watermark = watermark.wrapping_add(
            rust_compaction_low_wmark_pages(zone)
                .wrapping_sub(rust_compaction_min_wmark_pages(zone)),
        );
    }
    rust_compaction_zone_watermark_ok(
        zone,
        0,
        watermark,
        highest_zoneidx,
        RUST_COMPACTION_ALLOC_CMA,
        free_pages,
    )
}
#[no_mangle]
pub unsafe extern "C" fn compaction_suitable(
    zone: *mut zone,
    order: c_int,
    watermark: c_ulong,
    highest_zoneidx: c_int,
) -> bool {
    let mut suitable = __compaction_suitable(
        zone,
        order,
        watermark,
        highest_zoneidx,
        rust_compaction_zone_page_state(zone, NR_FREE_PAGES),
    );
    let mut result;
    if suitable {
        result = COMPACT_CONTINUE;
        if order > RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER as c_int {
            let fragindex = fragmentation_index(zone, order as c_uint);
            if fragindex >= 0 && fragindex <= sysctl_extfrag_threshold {
                suitable = false;
                result = COMPACT_NOT_SUITABLE_ZONE;
            }
        }
    } else {
        result = COMPACT_SKIPPED;
    }
    rust_compaction_trace_suitable(zone, order, result as c_int);
    suitable
}
#[no_mangle]
pub unsafe extern "C" fn compaction_zonelist_suitable(
    ac: *mut alloc_context,
    order: c_int,
    alloc_flags: c_int,
    gfp_mask: gfp_t,
) -> bool {
    let mut z = rust_compaction_first_zones_zonelist(
        (*ac).zonelist,
        (*ac).highest_zoneidx as zone_type,
        (*ac).nodemask,
    );
    loop {
        let zone = rust_compaction_zonelist_zone(z);
        if zone.is_null() {
            break;
        }
        if !(rust_compaction_cpusets_enabled()
            && alloc_flags & RUST_COMPACTION_ALLOC_CPUSET as c_int != 0
            && !rust_compaction_cpuset_zone_allowed(zone, gfp_mask))
        {
            let available = (zone_reclaimable_pages(zone) / order as c_ulong).wrapping_add(
                rust_compaction_zone_page_state_snapshot(zone, NR_FREE_PAGES),
            );
            if __compaction_suitable(
                zone,
                order,
                rust_compaction_min_wmark_pages(zone),
                (*ac).highest_zoneidx,
                available,
            ) {
                return true;
            }
        }
        z = rust_compaction_next_zones_zonelist(
            z.wrapping_add(1),
            (*ac).highest_zoneidx as zone_type,
            (*ac).nodemask,
        );
    }
    false
}
unsafe fn compaction_suit_allocation_order(
    zone: *mut zone,
    order: c_uint,
    highest_zoneidx: c_int,
    alloc_flags: c_uint,
    async_mode: bool,
    kcompactd: bool,
) -> compact_result {
    let free_pages = rust_compaction_zone_page_state(
        zone,
        if kcompactd && defrag_mode != 0 {
            NR_FREE_PAGES_BLOCKS
        } else {
            NR_FREE_PAGES
        },
    );
    let watermark =
        rust_compaction_wmark_pages(zone, alloc_flags & RUST_COMPACTION_ALLOC_WMARK_MASK);
    if rust_compaction_zone_watermark_ok(
        zone,
        order,
        watermark,
        highest_zoneidx,
        alloc_flags,
        free_pages,
    ) {
        return COMPACT_SUCCESS;
    }
    if order > RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER
        && async_mode
        && alloc_flags & RUST_COMPACTION_ALLOC_CMA == 0
    {
        if !rust_compaction_zone_watermark_ok(
            zone,
            0,
            watermark.wrapping_add(rust_compaction_compact_gap(order)),
            highest_zoneidx,
            0,
            rust_compaction_zone_page_state(zone, NR_FREE_PAGES),
        ) {
            return COMPACT_SKIPPED;
        }
    }
    if !compaction_suitable(zone, order as c_int, watermark, highest_zoneidx) {
        return COMPACT_SKIPPED;
    }
    COMPACT_CONTINUE
}
unsafe fn compact_zone(cc: *mut compact_control, capc: *mut capture_control) -> compact_result {
    let start_pfn = (*(*cc).zone).zone_start_pfn;
    let end_pfn = rust_compaction_zone_end_pfn((*cc).zone);
    let sync = (*cc).mode != MIGRATE_ASYNC;
    let mut nr_succeeded: c_uint = 0;
    (*cc).total_migrate_scanned = 0;
    (*cc).total_free_scanned = 0;
    (*cc).nr_migratepages = 0;
    (*cc).nr_freepages = 0;
    for order in 0..RUST_COMPACTION_NR_PAGE_ORDERS as usize {
        list_init(addr_of_mut!((*cc).freepages[order]));
    }
    list_init(addr_of_mut!((*cc).migratepages));
    (*cc).migratetype = rust_compaction_gfp_migratetype((*cc).gfp_mask);
    if !is_via_compact_memory((*cc).order) {
        let ret = compaction_suit_allocation_order(
            (*cc).zone,
            (*cc).order as c_uint,
            (*cc).highest_zoneidx,
            (*cc).alloc_flags,
            (*cc).mode == MIGRATE_ASYNC,
            !(*cc).direct_compaction,
        );
        if ret != COMPACT_CONTINUE {
            return ret;
        }
    }
    if compaction_restarting((*cc).zone, (*cc).order) {
        __reset_isolation_suitable((*cc).zone);
    }
    (*cc).fast_start_pfn = 0;
    if (*cc).whole_zone {
        (*cc).migrate_pfn = start_pfn;
        (*cc).free_pfn = pageblock_start_pfn(end_pfn.wrapping_sub(1));
    } else {
        (*cc).migrate_pfn = (*(*cc).zone).compact_cached_migrate_pfn[sync as usize];
        (*cc).free_pfn = (*(*cc).zone).compact_cached_free_pfn;
        if (*cc).free_pfn < start_pfn || (*cc).free_pfn >= end_pfn {
            (*cc).free_pfn = pageblock_start_pfn(end_pfn.wrapping_sub(1));
            (*(*cc).zone).compact_cached_free_pfn = (*cc).free_pfn;
        }
        if (*cc).migrate_pfn < start_pfn || (*cc).migrate_pfn >= end_pfn {
            (*cc).migrate_pfn = start_pfn;
            (*(*cc).zone).compact_cached_migrate_pfn[0] = (*cc).migrate_pfn;
            (*(*cc).zone).compact_cached_migrate_pfn[1] = (*cc).migrate_pfn;
        }
        if (*cc).migrate_pfn <= (*(*cc).zone).compact_init_migrate_pfn {
            (*cc).whole_zone = true;
        }
    }
    let mut last_migrated_pfn = 0;
    let mut update_cached = !sync
        && (*(*cc).zone).compact_cached_migrate_pfn[0]
            == (*(*cc).zone).compact_cached_migrate_pfn[1];
    rust_compaction_trace_begin(cc, start_pfn, end_pfn, sync);
    lru_add_drain();
    let ret = 'compact: loop {
        let ret = compact_finished(cc);
        if ret != COMPACT_CONTINUE {
            break ret;
        }
        let iteration_start_pfn = (*cc).migrate_pfn;
        (*cc).finish_pageblock =
            pageblock_start_pfn(last_migrated_pfn) == pageblock_start_pfn(iteration_start_pfn);
        loop {
            match isolate_migratepages(cc) {
                IsolateMigrate::Abort => {
                    putback_movable_pages(addr_of_mut!((*cc).migratepages));
                    (*cc).nr_migratepages = 0;
                    break 'compact COMPACT_CONTENDED;
                }
                IsolateMigrate::None => {
                    if update_cached {
                        (*(*cc).zone).compact_cached_migrate_pfn[1] =
                            (*(*cc).zone).compact_cached_migrate_pfn[0];
                    }
                    break;
                }
                IsolateMigrate::Success => {
                    update_cached = false;
                    last_migrated_pfn = max(
                        (*(*cc).zone).zone_start_pfn,
                        pageblock_start_pfn((*cc).migrate_pfn.wrapping_sub(1)),
                    );
                }
            }
            let nr_migratepages = (*cc).nr_migratepages as c_uint;
            let err = migrate_pages(
                addr_of_mut!((*cc).migratepages),
                Some(compaction_alloc),
                Some(compaction_free),
                cc as c_ulong,
                (*cc).mode,
                MR_COMPACTION,
                &mut nr_succeeded,
            );
            rust_compaction_trace_migratepages(nr_migratepages, nr_succeeded);
            (*cc).nr_migratepages = 0;
            if err != 0 {
                putback_movable_pages(addr_of_mut!((*cc).migratepages));
                if err == -(RUST_COMPACTION_ENOMEM as c_int) && !compact_scanners_met(cc) {
                    break 'compact COMPACT_CONTENDED;
                }
                if (*cc).migrate_pfn & pageblock_nr_pages().wrapping_sub(1) != 0
                    && !(*cc).ignore_skip_hint
                    && !(*cc).finish_pageblock
                    && (*cc).mode < MIGRATE_SYNC
                {
                    (*cc).finish_pageblock = true;
                    if (*cc).order == rust_compaction_hpage_order() as c_int {
                        last_migrated_pfn = 0;
                    }
                    continue;
                }
            }
            if !capc.is_null() && !(*capc).page.is_null() {
                break 'compact COMPACT_SUCCESS;
            }
            break;
        }
        if (*cc).order > 0 && last_migrated_pfn != 0 {
            let current_block_start = block_start_pfn((*cc).migrate_pfn, (*cc).order as c_uint);
            if last_migrated_pfn < current_block_start {
                lru_add_drain_cpu_zone((*cc).zone);
                last_migrated_pfn = 0;
            }
        }
    };
    if (*cc).nr_freepages > 0 {
        let mut free_pfn = release_free_list((*cc).freepages.as_mut_ptr());
        (*cc).nr_freepages = 0;
        #[cfg(CONFIG_DEBUG_VM)]
        rust_compaction_bug_free_pfn(free_pfn == 0);
        free_pfn = pageblock_start_pfn(free_pfn);
        if free_pfn > (*(*cc).zone).compact_cached_free_pfn {
            (*(*cc).zone).compact_cached_free_pfn = free_pfn;
        }
    }
    count_compact_events!(COMPACTMIGRATE_SCANNED, (*cc).total_migrate_scanned);
    count_compact_events!(COMPACTFREE_SCANNED, (*cc).total_free_scanned);
    rust_compaction_trace_end(cc, start_pfn, end_pfn, sync, ret as c_int);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_compaction_bug_migratepages(!list_empty(addr_of!((*cc).migratepages)));
    ret
}
unsafe fn compact_zone_order(
    zone: *mut zone,
    order: c_int,
    gfp_mask: gfp_t,
    prio: compact_priority,
    alloc_flags: c_uint,
    highest_zoneidx: c_int,
    capc: *mut capture_control,
) -> compact_result {
    // C's designated initializer zero-initializes all omitted native fields.
    let mut cc: compact_control = zeroed();
    cc.order = order;
    cc.search_order = order as _;
    cc.gfp_mask = gfp_mask;
    cc.zone = zone;
    cc.mode = if prio == COMPACT_PRIO_ASYNC {
        MIGRATE_ASYNC
    } else {
        MIGRATE_SYNC_LIGHT
    };
    cc.alloc_flags = alloc_flags;
    cc.highest_zoneidx = highest_zoneidx;
    cc.direct_compaction = true;
    cc.whole_zone = prio == MIN_COMPACT_PRIORITY;
    cc.ignore_skip_hint = prio == MIN_COMPACT_PRIORITY;
    cc.ignore_block_suitable = prio == MIN_COMPACT_PRIORITY;
    compact_zone(&mut cc, capc)
}
#[no_mangle]
pub unsafe extern "C" fn try_to_compact_pages(
    gfp_mask: gfp_t,
    order: c_uint,
    alloc_flags: c_uint,
    ac: *const alloc_context,
    prio: compact_priority,
    capc: *mut capture_control,
) -> compact_result {
    let mut rc = COMPACT_SKIPPED;
    if !rust_compaction_gfp_compaction_allowed(gfp_mask) {
        return COMPACT_SKIPPED;
    }
    rust_compaction_trace_try_to_compact_pages(order as c_int, gfp_mask, prio as c_int);
    let mut z = rust_compaction_first_zones_zonelist(
        (*ac).zonelist,
        (*ac).highest_zoneidx as zone_type,
        (*ac).nodemask,
    );
    loop {
        let zone = rust_compaction_zonelist_zone(z);
        if zone.is_null() {
            break;
        }
        if !(rust_compaction_cpusets_enabled()
            && alloc_flags & RUST_COMPACTION_ALLOC_CPUSET != 0
            && !rust_compaction_cpuset_zone_allowed(zone, gfp_mask))
        {
            if prio > MIN_COMPACT_PRIORITY && compaction_deferred(zone, order as c_int) {
                rc = max(COMPACT_DEFERRED, rc);
            } else {
                rust_compaction_write_capture_zone(capc, zone);
                let mut status = compact_zone_order(
                    zone,
                    order as c_int,
                    gfp_mask,
                    prio,
                    alloc_flags,
                    (*ac).highest_zoneidx,
                    capc,
                );
                rust_compaction_write_capture_zone(capc, null_mut());
                if !rust_compaction_read_capture_page(capc).is_null() {
                    status = COMPACT_SUCCESS;
                }
                rc = max(status, rc);
                if status == COMPACT_SUCCESS {
                    compaction_defer_reset(zone, order as c_int, false);
                    break;
                }
                if prio != COMPACT_PRIO_ASYNC
                    && (status == COMPACT_COMPLETE || status == COMPACT_PARTIAL_SKIPPED)
                {
                    defer_compaction(zone, order as c_int);
                }
                if (prio == COMPACT_PRIO_ASYNC && rust_compaction_need_resched())
                    || rust_compaction_fatal_signal_pending(rust_compaction_current())
                {
                    break;
                }
            }
        }
        z = rust_compaction_next_zones_zonelist(
            z.wrapping_add(1),
            (*ac).highest_zoneidx as zone_type,
            (*ac).nodemask,
        );
    }
    rc
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
