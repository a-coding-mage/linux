// SPDX-License-Identifier: GPL-2.0
unsafe extern "C" fn mark_allocated_noprof(
    page: *mut page,
    order: c_uint,
    _gfp_flags: gfp_t,
) -> *mut page {
    post_alloc_hook(
        page,
        order,
        RUST_COMPACTION___GFP_MOVABLE,
        RUST_COMPACTION_ALLOC_DEFAULT,
    );
    rust_compaction_set_page_refcounted(page);
    page
}
// alloc_hooks is a native allocation-profiling boundary; it invokes the Rust
// noprof callback, not a second C implementation of allocation/compaction.
unsafe fn mark_allocated(page: *mut page, order: c_uint, gfp_flags: gfp_t) -> *mut page {
    rust_compaction_mark_allocated_hooks(Some(mark_allocated_noprof), page, order, gfp_flags)
}
unsafe fn release_free_list(freepages: *mut list_head) -> c_ulong {
    let mut high_pfn = 0;
    for order in 0..RUST_COMPACTION_NR_PAGE_ORDERS {
        let head = freepages.add(order as usize);
        let mut entry = (*head).next;
        while entry != head {
            let next = (*entry).next;
            let page = rust_compaction_lru_page(entry);
            let pfn = rust_compaction_page_to_pfn(page);
            list_del(entry);
            mark_allocated(page, order, RUST_COMPACTION___GFP_MOVABLE);
            __free_pages(page, order);
            high_pfn = max(high_pfn, pfn);
            entry = next;
        }
    }
    high_pfn
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn defer_compaction(zone: *mut zone, order: c_int) {
    (*zone).compact_considered = 0;
    (*zone).compact_defer_shift = (*zone).compact_defer_shift.wrapping_add(1);
    if order < (*zone).compact_order_failed {
        (*zone).compact_order_failed = order;
    }
    if (*zone).compact_defer_shift > COMPACT_MAX_DEFER_SHIFT {
        (*zone).compact_defer_shift = COMPACT_MAX_DEFER_SHIFT;
    }
    rust_compaction_trace_defer_compaction(zone, order);
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn compaction_deferred(zone: *mut zone, order: c_int) -> bool {
    let defer_limit = (1 as c_ulong).wrapping_shl((*zone).compact_defer_shift);
    if order < (*zone).compact_order_failed {
        return false;
    }
    (*zone).compact_considered = (*zone).compact_considered.wrapping_add(1);
    if (*zone).compact_considered as c_ulong >= defer_limit {
        (*zone).compact_considered = defer_limit as _;
        return false;
    }
    rust_compaction_trace_deferred(zone, order);
    true
}
#[cfg(CONFIG_COMPACTION)]
#[no_mangle]
pub unsafe extern "C" fn compaction_defer_reset(
    zone: *mut zone,
    order: c_int,
    alloc_success: bool,
) {
    if alloc_success {
        (*zone).compact_considered = 0;
        (*zone).compact_defer_shift = 0;
    }
    if order >= (*zone).compact_order_failed {
        (*zone).compact_order_failed = order.wrapping_add(1);
    }
    rust_compaction_trace_defer_reset(zone, order);
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn compaction_restarting(zone: *mut zone, order: c_int) -> bool {
    if order < (*zone).compact_order_failed {
        return false;
    }
    (*zone).compact_defer_shift == COMPACT_MAX_DEFER_SHIFT
        && (*zone).compact_considered as c_ulong
            >= (1 as c_ulong).wrapping_shl((*zone).compact_defer_shift)
}
unsafe fn isolation_suitable(cc: *mut compact_control, page: *mut page) -> bool {
    #[cfg(CONFIG_COMPACTION)]
    {
        (*cc).ignore_skip_hint || !rust_compaction_get_pageblock_skip(page)
    }
    #[cfg(not(CONFIG_COMPACTION))]
    {
        true
    }
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn reset_cached_positions(zone: *mut zone) {
    (*zone).compact_cached_migrate_pfn[0] = (*zone).zone_start_pfn;
    (*zone).compact_cached_migrate_pfn[1] = (*zone).zone_start_pfn;
    (*zone).compact_cached_free_pfn =
        pageblock_start_pfn(rust_compaction_zone_end_pfn(zone).wrapping_sub(1));
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn skip_offline_sections(start_pfn: c_ulong) -> c_ulong {
    #[cfg(CONFIG_SPARSEMEM)]
    {
        let mut start_nr = rust_compaction_pfn_to_section_nr(start_pfn);
        if rust_compaction_online_section_nr(start_nr) {
            return 0;
        }
        loop {
            start_nr = start_nr.wrapping_add(1);
            if start_nr > rust_compaction_highest_present_section_nr() {
                break;
            }
            if rust_compaction_online_section_nr(start_nr) {
                return rust_compaction_section_nr_to_pfn(start_nr);
            }
        }
    }
    0
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn skip_offline_sections_reverse(start_pfn: c_ulong) -> c_ulong {
    #[cfg(CONFIG_SPARSEMEM)]
    {
        let mut start_nr = rust_compaction_pfn_to_section_nr(start_pfn);
        if start_nr == 0 || rust_compaction_online_section_nr(start_nr) {
            return 0;
        }
        while start_nr > 0 {
            start_nr = start_nr.wrapping_sub(1);
            if rust_compaction_online_section_nr(start_nr) {
                return rust_compaction_section_nr_to_pfn(start_nr)
                    .wrapping_add(RUST_COMPACTION_PAGES_PER_SECTION);
            }
        }
    }
    0
}
unsafe fn pageblock_skip_persistent(mut page: *mut page) -> bool {
    #[cfg(CONFIG_COMPACTION)]
    {
        if !rust_compaction_PageCompound(page) {
            return false;
        }
        page = rust_compaction_compound_head(page);
        rust_compaction_compound_order(page) >= rust_compaction_pageblock_order()
    }
    #[cfg(not(CONFIG_COMPACTION))]
    {
        false
    }
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn __reset_isolation_pfn(
    zone: *mut zone,
    mut pfn: c_ulong,
    check_source: bool,
    check_target: bool,
) -> bool {
    let mut page = rust_compaction_pfn_to_online_page(pfn);
    if page.is_null() || zone != rust_compaction_page_zone(page) || pageblock_skip_persistent(page)
    {
        return false;
    }
    if check_source && check_target && !rust_compaction_get_pageblock_skip(page) {
        return true;
    }
    if !check_source
        && check_target
        && rust_compaction_get_pageblock_migratetype(page) != MIGRATE_MOVABLE as c_int
    {
        return false;
    }
    let mut block_pfn = max(pageblock_start_pfn(pfn), (*zone).zone_start_pfn);
    let block_page = rust_compaction_pfn_to_online_page(block_pfn);
    if !block_page.is_null() {
        page = block_page;
        pfn = block_pfn;
    }
    block_pfn = min(
        pageblock_end_pfn(pfn).wrapping_sub(1),
        rust_compaction_zone_end_pfn(zone).wrapping_sub(1),
    );
    let end_page = rust_compaction_pfn_to_online_page(block_pfn);
    if end_page.is_null() {
        return false;
    }
    loop {
        if (check_source && rust_compaction_PageLRU(page))
            || (check_target && rust_compaction_PageBuddy(page))
        {
            rust_compaction_clear_pageblock_skip(page);
            return true;
        }
        page = page.wrapping_add(1usize.wrapping_shl(RUST_COMPACTION_PAGE_ALLOC_COSTLY_ORDER));
        if page > end_page {
            break;
        }
    }
    false
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn __reset_isolation_suitable(zone: *mut zone) {
    let mut migrate_pfn = (*zone).zone_start_pfn;
    let mut free_pfn = rust_compaction_zone_end_pfn(zone).wrapping_sub(1);
    let mut reset_migrate = free_pfn;
    let mut reset_free = migrate_pfn;
    let mut source_set = false;
    let mut free_set = false;
    if !(*zone).compact_blockskip_flush {
        return;
    }
    (*zone).compact_blockskip_flush = false;
    while migrate_pfn < free_pfn {
        rust_compaction_cond_resched();
        if __reset_isolation_pfn(zone, migrate_pfn, true, source_set) && migrate_pfn < reset_migrate
        {
            source_set = true;
            reset_migrate = migrate_pfn;
            (*zone).compact_init_migrate_pfn = reset_migrate;
            (*zone).compact_cached_migrate_pfn[0] = reset_migrate;
            (*zone).compact_cached_migrate_pfn[1] = reset_migrate;
        }
        if __reset_isolation_pfn(zone, free_pfn, free_set, true) && free_pfn > reset_free {
            free_set = true;
            reset_free = free_pfn;
            (*zone).compact_init_free_pfn = reset_free;
            (*zone).compact_cached_free_pfn = reset_free;
        }
        migrate_pfn = migrate_pfn.wrapping_add(pageblock_nr_pages());
        free_pfn = free_pfn.wrapping_sub(pageblock_nr_pages());
    }
    if reset_migrate >= reset_free {
        (*zone).compact_cached_migrate_pfn[0] = migrate_pfn;
        (*zone).compact_cached_migrate_pfn[1] = migrate_pfn;
        (*zone).compact_cached_free_pfn = free_pfn;
    }
}
#[cfg(CONFIG_COMPACTION)]
#[no_mangle]
pub unsafe extern "C" fn reset_isolation_suitable(pgdat: *mut pg_data_t) {
    for zoneid in 0..MAX_NR_ZONES as usize {
        let zone = addr_of_mut!((*pgdat).node_zones[zoneid]);
        if rust_compaction_populated_zone(zone) {
            __reset_isolation_suitable(zone);
        }
    }
}
unsafe fn test_and_set_skip(cc: *mut compact_control, page: *mut page) -> bool {
    #[cfg(CONFIG_COMPACTION)]
    {
        if (*cc).ignore_skip_hint {
            return false;
        }
        let skip = rust_compaction_get_pageblock_skip(page);
        if !skip && !(*cc).no_set_skip_hint {
            rust_compaction_set_pageblock_skip(page);
        }
        skip
    }
    #[cfg(not(CONFIG_COMPACTION))]
    {
        false
    }
}
unsafe fn update_cached_migrate(cc: *mut compact_control, mut pfn: c_ulong) {
    #[cfg(CONFIG_COMPACTION)]
    {
        let zone = (*cc).zone;
        if (*cc).no_set_skip_hint {
            return;
        }
        pfn = pageblock_end_pfn(pfn);
        if pfn > (*zone).compact_cached_migrate_pfn[0] {
            (*zone).compact_cached_migrate_pfn[0] = pfn;
        }
        if (*cc).mode != MIGRATE_ASYNC && pfn > (*zone).compact_cached_migrate_pfn[1] {
            (*zone).compact_cached_migrate_pfn[1] = pfn;
        }
    }
}
unsafe fn update_pageblock_skip(cc: *mut compact_control, page: *mut page, pfn: c_ulong) {
    #[cfg(CONFIG_COMPACTION)]
    {
        let zone = (*cc).zone;
        if (*cc).no_set_skip_hint {
            return;
        }
        rust_compaction_set_pageblock_skip(page);
        if pfn < (*zone).compact_cached_free_pfn {
            (*zone).compact_cached_free_pfn = pfn;
        }
    }
}
unsafe fn compact_lock_irqsave(
    lock: *mut spinlock_t,
    flags: *mut c_ulong,
    cc: *mut compact_control,
) -> bool {
    if (*cc).mode == MIGRATE_ASYNC && !(*cc).contended {
        if rust_compaction_spin_trylock_irqsave(lock, flags) {
            return true;
        }
        (*cc).contended = true;
    }
    rust_compaction_spin_lock_irqsave(lock, flags);
    true
}
unsafe fn compact_folio_lruvec_lock_irqsave(
    folio: *mut folio,
    flags: *mut c_ulong,
    cc: *mut compact_control,
) -> *mut lruvec {
    rust_compaction_rcu_read_lock();
    loop {
        let lruvec = rust_compaction_folio_lruvec(folio);
        compact_lock_irqsave(addr_of_mut!((*lruvec).lru_lock), flags, cc);
        if rust_compaction_lruvec_memcg(lruvec) == rust_compaction_folio_memcg(folio) {
            return lruvec;
        }
        rust_compaction_spin_unlock_irqrestore(addr_of_mut!((*lruvec).lru_lock), *flags);
    }
}
unsafe fn compact_unlock_should_abort(
    lock: *mut spinlock_t,
    flags: c_ulong,
    locked: *mut bool,
    cc: *mut compact_control,
) -> bool {
    if *locked {
        rust_compaction_spin_unlock_irqrestore(lock, flags);
        *locked = false;
    }
    if rust_compaction_fatal_signal_pending(rust_compaction_current()) {
        (*cc).contended = true;
        return true;
    }
    rust_compaction_cond_resched();
    false
}
unsafe fn isolate_freepages_block(
    cc: *mut compact_control,
    start_pfn: *mut c_ulong,
    end_pfn: c_ulong,
    freelist: *mut list_head,
    mut stride: c_uint,
    strict: bool,
) -> c_ulong {
    let mut nr_scanned: c_int = 0;
    let mut total_isolated: c_int = 0;
    let mut flags = 0;
    let mut locked = false;
    let mut blockpfn = *start_pfn;
    if strict {
        stride = 1;
    }
    let mut page = rust_compaction_pfn_to_page(blockpfn);
    while blockpfn < end_pfn {
        if blockpfn % RUST_COMPACTION_COMPACT_CLUSTER_MAX == 0
            && compact_unlock_should_abort(addr_of_mut!((*(*cc).zone).lock), flags, &mut locked, cc)
        {
            break;
        }
        nr_scanned = nr_scanned.wrapping_add(1);
        let mut failed = false;
        if rust_compaction_PageCompound(page) {
            let order = rust_compaction_compound_order(page);
            let nr = (1 as c_ulong).wrapping_shl(order);
            if order <= RUST_COMPACTION_MAX_PAGE_ORDER && blockpfn.wrapping_add(nr) <= end_pfn {
                blockpfn = blockpfn.wrapping_add(nr.wrapping_sub(1));
                page = page.wrapping_add(nr.wrapping_sub(1) as usize);
                nr_scanned = nr_scanned.wrapping_add(nr.wrapping_sub(1) as c_int);
            }
            failed = true;
        } else if !rust_compaction_PageBuddy(page) {
            failed = true;
        }
        if !failed && !locked {
            locked = compact_lock_irqsave(addr_of_mut!((*(*cc).zone).lock), &mut flags, cc);
            if !rust_compaction_PageBuddy(page) {
                failed = true;
            }
        }
        if failed {
            if strict {
                break;
            }
        } else {
            let order = rust_compaction_buddy_order(page);
            let isolated = __isolate_free_page(page, order) as c_int;
            if isolated == 0 {
                break;
            }
            nr_scanned = nr_scanned.wrapping_add(isolated.wrapping_sub(1));
            total_isolated = total_isolated.wrapping_add(isolated);
            (*cc).nr_freepages = (*cc).nr_freepages.wrapping_add(isolated as c_uint);
            list_add_tail(rust_compaction_page_lru(page), freelist.add(order as usize));
            if !strict && (*cc).nr_migratepages <= (*cc).nr_freepages {
                blockpfn = blockpfn.wrapping_add(isolated as c_ulong);
                break;
            }
            blockpfn = blockpfn.wrapping_add(isolated.wrapping_sub(1) as c_ulong);
            page = page.wrapping_offset(isolated.wrapping_sub(1) as isize);
        }
        blockpfn = blockpfn.wrapping_add(stride as c_ulong);
        page = page.wrapping_add(stride as usize);
    }
    if locked {
        rust_compaction_spin_unlock_irqrestore(addr_of_mut!((*(*cc).zone).lock), flags);
    }
    if blockpfn > end_pfn {
        blockpfn = end_pfn;
    }
    rust_compaction_trace_isolate_freepages(
        *start_pfn,
        blockpfn,
        nr_scanned as c_ulong,
        total_isolated as c_ulong,
    );
    *start_pfn = blockpfn;
    if strict && blockpfn < end_pfn {
        total_isolated = 0;
    }
    (*cc).total_free_scanned = (*cc).total_free_scanned.wrapping_add(nr_scanned as c_ulong);
    if total_isolated != 0 {
        count_compact_events!(COMPACTISOLATED, total_isolated);
    }
    total_isolated as c_ulong
}
#[no_mangle]
pub unsafe extern "C" fn isolate_freepages_range(
    cc: *mut compact_control,
    start_pfn: c_ulong,
    end_pfn: c_ulong,
) -> c_ulong {
    for order in 0..RUST_COMPACTION_NR_PAGE_ORDERS as usize {
        list_init(addr_of_mut!((*cc).freepages[order]));
    }
    let mut pfn = start_pfn;
    let mut block_start = max(pageblock_start_pfn(pfn), (*(*cc).zone).zone_start_pfn);
    let mut block_end = pageblock_end_pfn(pfn);
    while pfn < end_pfn {
        let mut isolate_start_pfn = pfn;
        if pfn >= block_end {
            block_start = pageblock_start_pfn(pfn);
            block_end = pageblock_end_pfn(pfn);
        }
        block_end = min(block_end, end_pfn);
        if rust_compaction_pageblock_pfn_to_page(block_start, block_end, (*cc).zone).is_null() {
            break;
        }
        let isolated = isolate_freepages_block(
            cc,
            &mut isolate_start_pfn,
            block_end,
            (*cc).freepages.as_mut_ptr(),
            0,
            true,
        );
        if isolated == 0 {
            break;
        }
        pfn = pfn.wrapping_add(isolated);
        block_start = block_end;
        block_end = block_end.wrapping_add(pageblock_nr_pages());
    }
    if pfn < end_pfn {
        release_free_list((*cc).freepages.as_mut_ptr());
        return 0;
    }
    pfn
}
unsafe fn too_many_isolated(cc: *mut compact_control) -> bool {
    let pgdat = (*(*cc).zone).zone_pgdat;
    let mut inactive = rust_compaction_node_page_state(pgdat, NR_INACTIVE_FILE)
        .wrapping_add(rust_compaction_node_page_state(pgdat, NR_INACTIVE_ANON));
    let mut active = rust_compaction_node_page_state(pgdat, NR_ACTIVE_FILE)
        .wrapping_add(rust_compaction_node_page_state(pgdat, NR_ACTIVE_ANON));
    let isolated = rust_compaction_node_page_state(pgdat, NR_ISOLATED_FILE)
        .wrapping_add(rust_compaction_node_page_state(pgdat, NR_ISOLATED_ANON));
    if (*cc).gfp_mask & RUST_COMPACTION___GFP_FS != 0 {
        inactive >>= 3;
        active >>= 3;
    }
    let too_many = isolated > inactive.wrapping_add(active) / 2;
    if !too_many {
        rust_compaction_wake_throttle_isolated(pgdat);
    }
    too_many
}
unsafe fn skip_isolation_on_order(order: c_int, target_order: c_int) -> bool {
    if !is_via_compact_memory(target_order) && order >= target_order {
        return true;
    }
    order as c_uint >= rust_compaction_pageblock_order()
}
// Source gotos are represented by explicit outcomes so all folio puts and
// lruvec/RCU unlocks retain the source ordering.
enum IsolationStep {
    Continue,
    Fail,
    FailPut,
    Success,
    SuccessNoList,
    Abort,
}
unsafe fn isolate_migratepages_block(
    cc: *mut compact_control,
    mut low_pfn: c_ulong,
    end_pfn: c_ulong,
    mode: isolate_mode_t,
) -> c_int {
    let pgdat = (*(*cc).zone).zone_pgdat;
    let mut nr_scanned: c_ulong = 0;
    let mut nr_isolated: c_ulong = 0;
    let mut lruvec: *mut lruvec = null_mut();
    let mut flags = 0;
    let mut locked: *mut lruvec = null_mut();
    let mut folio: *mut folio = null_mut();
    let mut valid_page: *mut page = null_mut();
    let start_pfn = low_pfn;
    let mut skip_on_failure = false;
    let mut next_skip_pfn = 0;
    let mut skip_updated = false;
    let mut ret = 0;
    let mut fatal_pending = false;
    let mut abort = false;
    (*cc).migrate_pfn = low_pfn;
    while too_many_isolated(cc) {
        if (*cc).nr_migratepages != 0 || (*cc).mode == MIGRATE_ASYNC {
            return -(RUST_COMPACTION_EAGAIN as c_int);
        }
        reclaim_throttle(pgdat, VMSCAN_THROTTLE_ISOLATED);
        if rust_compaction_fatal_signal_pending(rust_compaction_current()) {
            return -(RUST_COMPACTION_EINTR as c_int);
        }
    }
    rust_compaction_cond_resched();
    if (*cc).direct_compaction && (*cc).mode == MIGRATE_ASYNC {
        skip_on_failure = true;
        next_skip_pfn = block_end_pfn(low_pfn, (*cc).order as c_uint);
    }
    while low_pfn < end_pfn {
        if skip_on_failure && low_pfn >= next_skip_pfn {
            if nr_isolated != 0 {
                break;
            }
            next_skip_pfn = block_end_pfn(low_pfn, (*cc).order as c_uint);
        }
        if low_pfn % RUST_COMPACTION_COMPACT_CLUSTER_MAX == 0 {
            if !locked.is_null() {
                rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                locked = null_mut();
            }
            if rust_compaction_fatal_signal_pending(rust_compaction_current()) {
                (*cc).contended = true;
                ret = -(RUST_COMPACTION_EINTR as c_int);
                fatal_pending = true;
                break;
            }
            rust_compaction_cond_resched();
        }
        nr_scanned = nr_scanned.wrapping_add(1);
        let page = rust_compaction_pfn_to_page(low_pfn);
        let step = 'attempt: {
            if valid_page.is_null()
                && (low_pfn & pageblock_nr_pages().wrapping_sub(1) == 0
                    || low_pfn == (*(*cc).zone).zone_start_pfn)
            {
                if !isolation_suitable(cc, page) {
                    low_pfn = end_pfn;
                    folio = null_mut();
                    break 'attempt IsolationStep::Abort;
                }
                valid_page = page;
            }
            if rust_compaction_PageHuge(page) {
                let order = rust_compaction_compound_order(page);
                if !(*cc).alloc_contig {
                    if order <= RUST_COMPACTION_MAX_PAGE_ORDER {
                        let skip = (1 as c_ulong).wrapping_shl(order).wrapping_sub(1);
                        low_pfn = low_pfn.wrapping_add(skip);
                        nr_scanned = nr_scanned.wrapping_add(skip);
                    }
                    break 'attempt IsolationStep::Fail;
                }
                if !locked.is_null() {
                    rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                    locked = null_mut();
                }
                folio = rust_compaction_page_folio(page);
                ret = rust_compaction_isolate_or_dissolve_huge_folio(
                    folio,
                    addr_of_mut!((*cc).migratepages),
                );
                if ret < 0 {
                    if ret == -(RUST_COMPACTION_EBUSY as c_int) {
                        ret = 0;
                    }
                    let skip = (1 as c_ulong).wrapping_shl(order).wrapping_sub(1);
                    low_pfn = low_pfn.wrapping_add(skip);
                    nr_scanned = nr_scanned.wrapping_add(skip);
                    break 'attempt IsolationStep::Fail;
                }
                if rust_compaction_folio_test_hugetlb(folio) {
                    low_pfn = low_pfn.wrapping_add(
                        rust_compaction_folio_nr_pages(folio)
                            .wrapping_sub(rust_compaction_folio_page_idx(folio, page))
                            .wrapping_sub(1),
                    );
                    break 'attempt IsolationStep::SuccessNoList;
                }
            }
            if rust_compaction_PageBuddy(page) {
                let order = rust_compaction_buddy_order_unsafe(page);
                if order > 0 && order <= RUST_COMPACTION_MAX_PAGE_ORDER as c_ulong {
                    let skip = (1 as c_ulong).wrapping_shl(order as c_uint).wrapping_sub(1);
                    low_pfn = low_pfn.wrapping_add(skip);
                    nr_scanned = nr_scanned.wrapping_add(skip);
                }
                break 'attempt IsolationStep::Continue;
            }
            if rust_compaction_PageCompound(page) && !(*cc).alloc_contig {
                let order = rust_compaction_compound_order(page);
                if skip_isolation_on_order(order as c_int, (*cc).order) {
                    if order <= RUST_COMPACTION_MAX_PAGE_ORDER {
                        let skip = (1 as c_ulong).wrapping_shl(order).wrapping_sub(1);
                        low_pfn = low_pfn.wrapping_add(skip);
                        nr_scanned = nr_scanned.wrapping_add(skip);
                    }
                    break 'attempt IsolationStep::Fail;
                }
            }
            if !rust_compaction_PageLRU(page) {
                if rust_compaction_page_has_movable_ops(page)
                    && !rust_compaction_PageMovableOpsIsolated(page)
                {
                    if !locked.is_null() {
                        rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                        locked = null_mut();
                    }
                    if isolate_movable_ops_page(page, mode) {
                        folio = rust_compaction_page_folio(page);
                        break 'attempt IsolationStep::Success;
                    }
                }
                break 'attempt IsolationStep::Fail;
            }
            folio = rust_compaction_folio_get_nontail_page(page);
            if folio.is_null() {
                break 'attempt IsolationStep::Fail;
            }
            let mut mapping = rust_compaction_folio_mapping(folio);
            if mapping.is_null()
                && rust_compaction_folio_ref_count(folio).wrapping_sub(1)
                    > rust_compaction_folio_mapcount(folio)
            {
                break 'attempt IsolationStep::FailPut;
            }
            if (*cc).gfp_mask & RUST_COMPACTION___GFP_FS == 0 && !mapping.is_null() {
                break 'attempt IsolationStep::FailPut;
            }
            if !rust_compaction_folio_test_lru(folio) {
                break 'attempt IsolationStep::FailPut;
            }
            let is_unevictable = rust_compaction_folio_test_unevictable(folio);
            if mode & RUST_COMPACTION_ISOLATE_UNEVICTABLE == 0 && is_unevictable {
                break 'attempt IsolationStep::FailPut;
            }
            if mode & RUST_COMPACTION_ISOLATE_ASYNC_MIGRATE != 0
                && rust_compaction_folio_test_writeback(folio)
            {
                break 'attempt IsolationStep::FailPut;
            }
            let is_dirty = rust_compaction_folio_test_dirty(folio);
            if (mode & RUST_COMPACTION_ISOLATE_ASYNC_MIGRATE != 0 && is_dirty)
                || (!mapping.is_null() && is_unevictable)
            {
                let mut migrate_dirty = true;
                if !rust_compaction_folio_trylock(folio) {
                    break 'attempt IsolationStep::FailPut;
                }
                mapping = rust_compaction_folio_mapping(folio);
                if mode & RUST_COMPACTION_ISOLATE_ASYNC_MIGRATE != 0 && is_dirty {
                    migrate_dirty =
                        mapping.is_null() || rust_compaction_mapping_has_migrate_folio(mapping);
                }
                let is_inaccessible =
                    !mapping.is_null() && rust_compaction_mapping_inaccessible(mapping);
                folio_unlock(folio);
                if !migrate_dirty || is_inaccessible {
                    break 'attempt IsolationStep::FailPut;
                }
            }
            if !rust_compaction_folio_test_clear_lru(folio) {
                break 'attempt IsolationStep::FailPut;
            }
            if !locked.is_null() {
                lruvec = rust_compaction_folio_lruvec(folio);
            }
            if lruvec != locked || locked.is_null() {
                if !locked.is_null() {
                    rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                }
                lruvec = compact_folio_lruvec_lock_irqsave(folio, &mut flags, cc);
                locked = lruvec;
                if !skip_updated && !valid_page.is_null() {
                    skip_updated = true;
                    if test_and_set_skip(cc, valid_page) && !(*cc).finish_pageblock {
                        low_pfn = end_pfn;
                        break 'attempt IsolationStep::Abort;
                    }
                }
                if skip_isolation_on_order(rust_compaction_folio_order(folio) as c_int, (*cc).order)
                    && !(*cc).alloc_contig
                {
                    let skip = rust_compaction_folio_nr_pages(folio).wrapping_sub(1);
                    low_pfn = low_pfn.wrapping_add(skip);
                    nr_scanned = nr_scanned.wrapping_add(skip);
                    rust_compaction_folio_set_lru(folio);
                    break 'attempt IsolationStep::FailPut;
                }
            }
            if rust_compaction_folio_test_large(folio) {
                low_pfn =
                    low_pfn.wrapping_add(rust_compaction_folio_nr_pages(folio).wrapping_sub(1));
            }
            rust_compaction_lruvec_del_folio(lruvec, folio);
            rust_compaction_node_stat_mod_folio(
                folio,
                NR_ISOLATED_ANON.wrapping_add(rust_compaction_folio_is_file_lru(folio) as _),
                rust_compaction_folio_nr_pages(folio) as c_long,
            );
            IsolationStep::Success
        };
        match step {
            IsolationStep::Abort => {
                abort = true;
                break;
            }
            IsolationStep::Success | IsolationStep::SuccessNoList => {
                if matches!(step, IsolationStep::Success) {
                    list_add(
                        rust_compaction_folio_lru(folio),
                        addr_of_mut!((*cc).migratepages),
                    );
                }
                let nr = rust_compaction_folio_nr_pages(folio);
                (*cc).nr_migratepages = (*cc).nr_migratepages.wrapping_add(nr as c_uint);
                nr_isolated = nr_isolated.wrapping_add(nr);
                nr_scanned = nr_scanned.wrapping_add(nr.wrapping_sub(1));
                if (*cc).nr_migratepages >= RUST_COMPACTION_COMPACT_CLUSTER_MAX as c_uint
                    && !(*cc).finish_pageblock
                    && !(*cc).contended
                {
                    low_pfn = low_pfn.wrapping_add(1);
                    break;
                }
            }
            IsolationStep::Fail | IsolationStep::FailPut => {
                if matches!(step, IsolationStep::FailPut) {
                    if !locked.is_null() {
                        rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                        locked = null_mut();
                    }
                    rust_compaction_folio_put(folio);
                }
                if skip_on_failure || ret == -(RUST_COMPACTION_ENOMEM as c_int) {
                    if nr_isolated != 0 {
                        if !locked.is_null() {
                            rust_compaction_lruvec_unlock_irqrestore(locked, flags);
                            locked = null_mut();
                        }
                        putback_movable_pages(addr_of_mut!((*cc).migratepages));
                        (*cc).nr_migratepages = 0;
                        nr_isolated = 0;
                    }
                    if low_pfn < next_skip_pfn {
                        low_pfn = next_skip_pfn.wrapping_sub(1);
                        next_skip_pfn = next_skip_pfn
                            .wrapping_add((1 as c_ulong).wrapping_shl((*cc).order as c_uint));
                    }
                    if ret == -(RUST_COMPACTION_ENOMEM as c_int) {
                        break;
                    }
                }
            }
            IsolationStep::Continue => {}
        }
        low_pfn = low_pfn.wrapping_add(1);
    }
    if !fatal_pending {
        if !abort {
            if low_pfn > end_pfn {
                low_pfn = end_pfn;
            }
            folio = null_mut();
        }
        if !locked.is_null() {
            rust_compaction_lruvec_unlock_irqrestore(locked, flags);
        }
        if !folio.is_null() {
            rust_compaction_folio_set_lru(folio);
            rust_compaction_folio_put(folio);
        }
        if low_pfn == end_pfn && (nr_isolated == 0 || (*cc).finish_pageblock) {
            if !(*cc).no_set_skip_hint && !valid_page.is_null() && !skip_updated {
                rust_compaction_set_pageblock_skip(valid_page);
            }
            update_cached_migrate(cc, low_pfn);
        }
        rust_compaction_trace_isolate_migratepages(start_pfn, low_pfn, nr_scanned, nr_isolated);
    }
    (*cc).total_migrate_scanned = (*cc).total_migrate_scanned.wrapping_add(nr_scanned);
    if nr_isolated != 0 {
        count_compact_events!(COMPACTISOLATED, nr_isolated);
    }
    (*cc).migrate_pfn = low_pfn;
    ret
}
#[no_mangle]
pub unsafe extern "C" fn isolate_migratepages_range(
    cc: *mut compact_control,
    start_pfn: c_ulong,
    end_pfn: c_ulong,
) -> c_int {
    let mut ret = 0;
    let mut pfn = start_pfn;
    let mut block_start = max(pageblock_start_pfn(pfn), (*(*cc).zone).zone_start_pfn);
    let mut block_end = pageblock_end_pfn(pfn);
    while pfn < end_pfn {
        block_end = min(block_end, end_pfn);
        if !rust_compaction_pageblock_pfn_to_page(block_start, block_end, (*cc).zone).is_null() {
            ret =
                isolate_migratepages_block(cc, pfn, block_end, RUST_COMPACTION_ISOLATE_UNEVICTABLE);
            if ret != 0 || (*cc).nr_migratepages >= RUST_COMPACTION_COMPACT_CLUSTER_MAX as c_uint {
                break;
            }
        }
        pfn = block_end;
        block_start = block_end;
        block_end = block_end.wrapping_add(pageblock_nr_pages());
    }
    ret
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
