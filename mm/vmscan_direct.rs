// SPDX-License-Identifier: GPL-2.0-only
// Native source mm/vmscan.c:5969–8131, frozen e1d84f501551.
unsafe fn shrink_lruvec(lv: *mut lruvec, sc: *mut scan_control) {
    #[cfg(CONFIG_LRU_GEN)]
    if (lru_gen_enabled() || lru_gen_switching()) && !root_reclaim(sc) {
        lru_gen_shrink_lruvec(lv, sc);
        if !lru_gen_switching() {
            return;
        }
    }
    let mut nr = [0 as ULong; NR_LRU_LISTS as usize];
    get_scan_count(lv, sc, nr.as_mut_ptr());
    let targets = nr;
    let mut reclaimed: ULong = 0;
    let goal = (*sc).nr_to_reclaim;
    let proportional =
        !cgroup_reclaim(sc) && !current_is_kswapd() && (*sc).priority as i32 == DEF_PRIORITY as i32;
    let mut plug: blk_plug = zeroed();
    blk_start_plug(&mut plug);
    while nr[LRU_INACTIVE_ANON as usize] != 0
        || nr[LRU_ACTIVE_FILE as usize] != 0
        || nr[LRU_INACTIVE_FILE as usize] != 0
    {
        for lru in LRU_INACTIVE_ANON..=LRU_ACTIVE_FILE {
            let i = lru as usize;
            if nr[i] == 0 {
                continue;
            }
            let scan = min(nr[i], SWAP_CLUSTER_MAX as ULong);
            nr[i] -= scan;
            reclaimed = reclaimed.wrapping_add(shrink_list(lru, scan, lv, sc));
        }
        cond_resched_tasks_rcu_qs();
        if reclaimed < goal || proportional {
            continue;
        }
        let file = nr[LRU_INACTIVE_FILE as usize].wrapping_add(nr[LRU_ACTIVE_FILE as usize]);
        let anon = nr[LRU_INACTIVE_ANON as usize].wrapping_add(nr[LRU_ACTIVE_ANON as usize]);
        if file == 0 || anon == 0 {
            break;
        }
        let (mut lru, percentage) = if file > anon {
            let target = targets[LRU_INACTIVE_ANON as usize]
                .wrapping_add(targets[LRU_ACTIVE_ANON as usize])
                .wrapping_add(1);
            (LRU_BASE, anon.wrapping_mul(100) / target)
        } else {
            let target = targets[LRU_INACTIVE_FILE as usize]
                .wrapping_add(targets[LRU_ACTIVE_FILE as usize])
                .wrapping_add(1);
            (LRU_FILE, file.wrapping_mul(100) / target)
        };
        nr[lru as usize] = 0;
        nr[(lru + LRU_ACTIVE) as usize] = 0;
        lru = if lru == LRU_FILE { LRU_BASE } else { LRU_FILE };
        for i in [lru as usize, (lru + LRU_ACTIVE) as usize] {
            let scanned = targets[i].wrapping_sub(nr[i]);
            nr[i] = targets[i].wrapping_mul((100 as ULong).wrapping_sub(percentage)) / 100;
            nr[i] -= min(nr[i], scanned);
        }
    }
    blk_finish_plug(&mut plug);
    (*sc).nr_reclaimed = (*sc).nr_reclaimed.wrapping_add(reclaimed);
    if can_age_anon_pages(lv, sc) && inactive_is_low(lv, LRU_INACTIVE_ANON) {
        shrink_active_list(SWAP_CLUSTER_MAX as ULong, lv, sc, LRU_ACTIVE_ANON);
    }
}
unsafe fn in_reclaim_compaction(sc: *mut scan_control) -> bool {
    gfp_compaction_allowed((*sc).gfp_mask)
        && (*sc).order != 0
        && ((*sc).order as i32 > PAGE_ALLOC_COSTLY_ORDER as i32
            || ((*sc).priority as i32) < DEF_PRIORITY as i32 - 2)
}
unsafe fn should_continue_reclaim(
    pgdat: *mut pglist_data,
    reclaimed: ULong,
    sc: *mut scan_control,
) -> bool {
    if !in_reclaim_compaction(sc) || reclaimed == 0 {
        return false;
    }
    for zid in 0..=(*sc).reclaim_idx as i32 {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if !managed_zone(z) {
            continue;
        }
        let watermark = min_wmark_pages(z);
        if zone_watermark_ok(
            z,
            (*sc).order as u32,
            watermark,
            (*sc).reclaim_idx as i32,
            0,
        ) || compaction_suitable(z, (*sc).order as i32, watermark, (*sc).reclaim_idx as i32)
        {
            return false;
        }
    }
    let mut inactive = node_page_state(pgdat, NR_INACTIVE_FILE);
    if can_reclaim_anon_pages(null_mut(), (*pgdat).node_id, sc) {
        inactive = inactive.wrapping_add(node_page_state(pgdat, NR_INACTIVE_ANON));
    }
    inactive > compact_gap((*sc).order as u32)
}
unsafe fn shrink_node_memcgs(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let target = (*sc).target_mem_cgroup;
    let mut cookie: mem_cgroup_reclaim_cookie = zeroed();
    cookie.pgdat = pgdat;
    let partial = if current_is_kswapd() || (*sc).memcg_full_walk() != 0 {
        null_mut()
    } else {
        addr_of_mut!(cookie)
    };
    let mut memcg = mem_cgroup_iter(target, null_mut(), partial);
    loop {
        let lv = mem_cgroup_lruvec(memcg, pgdat);
        cond_resched();
        mem_cgroup_calculate_protection(target, memcg);
        let mut eligible = !mem_cgroup_below_min(target, memcg);
        if eligible && mem_cgroup_below_low(target, memcg) {
            if (*sc).memcg_low_reclaim() == 0 {
                (*sc).set_memcg_low_skipped(1);
                eligible = false;
            } else {
                memcg_memory_event(memcg, MEMCG_LOW);
            }
        }
        if eligible {
            let reclaimed = (*sc).nr_reclaimed;
            let scanned = (*sc).nr_scanned;
            shrink_lruvec(lv, sc);
            shrink_slab(
                (*sc).gfp_mask,
                (*pgdat).node_id,
                memcg,
                (*sc).priority as i32,
            );
            if (*sc).proactive() == 0 {
                vmpressure(
                    (*sc).gfp_mask,
                    (*sc).order as i32,
                    memcg,
                    false,
                    (*sc).nr_scanned.wrapping_sub(scanned),
                    (*sc).nr_reclaimed.wrapping_sub(reclaimed),
                );
            }
            if !partial.is_null() && (*sc).nr_reclaimed >= (*sc).nr_to_reclaim {
                mem_cgroup_iter_break(target, memcg);
                break;
            }
        }
        memcg = mem_cgroup_iter(target, memcg, partial);
        if memcg.is_null() {
            break;
        }
    }
}
unsafe fn shrink_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let mut reclaimable = false;
    #[cfg(CONFIG_LRU_GEN)]
    if (lru_gen_enabled() || lru_gen_switching()) && root_reclaim(sc) {
        core::ptr::write_bytes(addr_of_mut!((*sc).nr), 0, 1);
        lru_gen_shrink_node(pgdat, sc);
        if !lru_gen_switching() {
            return;
        }
    }
    let lv = mem_cgroup_lruvec((*sc).target_mem_cgroup, pgdat);
    loop {
        core::ptr::write_bytes(addr_of_mut!((*sc).nr), 0, 1);
        let reclaimed = (*sc).nr_reclaimed;
        let scanned = (*sc).nr_scanned;
        prepare_scan_control(pgdat, sc);
        shrink_node_memcgs(pgdat, sc);
        flush_reclaim_state(sc);
        let node_reclaimed = (*sc).nr_reclaimed.wrapping_sub(reclaimed);
        if (*sc).proactive() == 0 {
            vmpressure(
                (*sc).gfp_mask,
                (*sc).order as i32,
                (*sc).target_mem_cgroup,
                true,
                (*sc).nr_scanned.wrapping_sub(scanned),
                node_reclaimed,
            );
        }
        if node_reclaimed != 0 {
            reclaimable = true;
        }
        if current_is_kswapd() {
            if (*sc).nr.writeback != 0 && (*sc).nr.writeback == (*sc).nr.taken {
                set_bit(PGDAT_WRITEBACK as ULong, addr_of_mut!((*pgdat).flags));
            }
            if (*sc).nr.immediate != 0 {
                reclaim_throttle(pgdat, VMSCAN_THROTTLE_WRITEBACK);
            }
        }
        if (*sc).nr.dirty != 0 && (*sc).nr.dirty == (*sc).nr.congested {
            if cgroup_reclaim(sc) && writeback_throttling_sane(sc) {
                set_bit(LRUVEC_CGROUP_CONGESTED as ULong, addr_of_mut!((*lv).flags));
            }
            if current_is_kswapd() {
                set_bit(LRUVEC_NODE_CONGESTED as ULong, addr_of_mut!((*lv).flags));
            }
        }
        if !current_is_kswapd()
            && current_may_throttle()
            && (*sc).hibernation_mode() == 0
            && (test_bit(LRUVEC_CGROUP_CONGESTED as ULong, addr_of!((*lv).flags))
                || test_bit(LRUVEC_NODE_CONGESTED as ULong, addr_of!((*lv).flags)))
        {
            reclaim_throttle(pgdat, VMSCAN_THROTTLE_CONGESTED);
        }
        if !should_continue_reclaim(pgdat, node_reclaimed, sc) {
            break;
        }
    }
    if reclaimable {
        kswapd_try_clear_hopeless(pgdat, (*sc).order as u32, (*sc).reclaim_idx as i32);
    } else if (*sc).cache_trim_mode() != 0 {
        (*sc).set_cache_trim_mode_failed(1);
    }
}
unsafe fn compaction_ready(z: *mut zone, sc: *mut scan_control) -> bool {
    if !gfp_compaction_allowed((*sc).gfp_mask) {
        return false;
    }
    zone_watermark_ok(
        z,
        (*sc).order as u32,
        min_wmark_pages(z),
        (*sc).reclaim_idx as i32,
        0,
    ) || compaction_suitable(
        z,
        (*sc).order as i32,
        high_wmark_pages(z),
        (*sc).reclaim_idx as i32,
    )
}
unsafe fn consider_reclaim_throttle(pgdat: *mut pglist_data, sc: *mut scan_control) {
    if (*sc).nr_reclaimed > (*sc).nr_scanned >> 3 {
        let wqh = (*pgdat)
            .reclaim_wait
            .as_mut_ptr()
            .add(VMSCAN_THROTTLE_NOPROGRESS as usize);
        if waitqueue_active(wqh) {
            wake_up(wqh);
        }
        return;
    }
    if current_is_kswapd() || cgroup_reclaim(sc) {
        return;
    }
    if (*sc).priority == 1 && (*sc).nr_reclaimed == 0 {
        reclaim_throttle(pgdat, VMSCAN_THROTTLE_NOPROGRESS);
    }
}
unsafe fn shrink_zones(zl: *mut zonelist, sc: *mut scan_control) {
    let orig = (*sc).gfp_mask;
    let mut last = null_mut();
    let mut first = null_mut();
    if native_buffer_heads_over_limit() != 0 {
        (*sc).gfp_mask |= __GFP_HIGHMEM;
        (*sc).reclaim_idx = gfp_zone((*sc).gfp_mask) as i8;
    }
    let mut zr = first_zones_zonelist(zl, (*sc).reclaim_idx as i32, (*sc).nodemask);
    while !(*zr).zone.is_null() {
        let z = (*zr).zone;
        zr = next_zones_zonelist(zr.add(1), (*sc).reclaim_idx as i32, (*sc).nodemask);
        if !cgroup_reclaim(sc) {
            if !cpuset_zone_allowed(z, GFP_KERNEL | __GFP_HARDWALL) {
                continue;
            }
            if cfg!(CONFIG_COMPACTION)
                && (*sc).order as i32 > PAGE_ALLOC_COSTLY_ORDER as i32
                && compaction_ready(z, sc)
            {
                (*sc).set_compaction_ready(1);
                continue;
            }
            if (*z).zone_pgdat == last {
                continue;
            }
            let mut scanned = 0;
            let reclaimed = memcg1_soft_limit_reclaim(
                (*z).zone_pgdat,
                (*sc).order as i32,
                (*sc).gfp_mask,
                &mut scanned,
            );
            (*sc).nr_reclaimed = (*sc).nr_reclaimed.wrapping_add(reclaimed);
            (*sc).nr_scanned = (*sc).nr_scanned.wrapping_add(scanned);
        }
        if first.is_null() {
            first = (*z).zone_pgdat;
        }
        if (*z).zone_pgdat == last {
            continue;
        }
        last = (*z).zone_pgdat;
        shrink_node(last, sc);
    }
    if !first.is_null() {
        consider_reclaim_throttle(first, sc);
    }
    (*sc).gfp_mask = orig;
}
unsafe fn snapshot_refaults(target: *mut mem_cgroup, pgdat: *mut pglist_data) {
    if lru_gen_enabled() && !lru_gen_switching() {
        return;
    }
    let lv = mem_cgroup_lruvec(target, pgdat);
    (*lv).refaults[WORKINGSET_ANON as usize] = lruvec_page_state(lv, WORKINGSET_ACTIVATE_ANON);
    (*lv).refaults[WORKINGSET_FILE as usize] = lruvec_page_state(lv, WORKINGSET_ACTIVATE_FILE);
}
unsafe fn do_try_to_free_pages(zl: *mut zonelist, sc: *mut scan_control) -> ULong {
    let initial = (*sc).priority;
    loop {
        delayacct_freepages_start();
        if !cgroup_reclaim(sc) {
            __count_zid_vm_events(ALLOCSTALL, (*sc).reclaim_idx as i32, 1);
        }
        loop {
            if (*sc).proactive() == 0 {
                vmpressure_prio(
                    (*sc).gfp_mask,
                    (*sc).target_mem_cgroup,
                    (*sc).priority as i32,
                );
            }
            (*sc).nr_scanned = 0;
            shrink_zones(zl, sc);
            if (*sc).nr_reclaimed >= (*sc).nr_to_reclaim || (*sc).compaction_ready() != 0 {
                break;
            }
            (*sc).priority -= 1;
            if (*sc).priority < 0 {
                break;
            }
        }
        let mut last = null_mut();
        let mut zr = first_zones_zonelist(zl, (*sc).reclaim_idx as i32, (*sc).nodemask);
        while !(*zr).zone.is_null() {
            let z = (*zr).zone;
            zr = next_zones_zonelist(zr.add(1), (*sc).reclaim_idx as i32, (*sc).nodemask);
            if (*z).zone_pgdat == last {
                continue;
            }
            last = (*z).zone_pgdat;
            snapshot_refaults((*sc).target_mem_cgroup, last);
            if cgroup_reclaim(sc) {
                let lv = mem_cgroup_lruvec((*sc).target_mem_cgroup, last);
                clear_bit(LRUVEC_CGROUP_CONGESTED as ULong, addr_of_mut!((*lv).flags));
            }
        }
        delayacct_freepages_end();
        if (*sc).nr_reclaimed != 0 {
            return (*sc).nr_reclaimed;
        }
        if (*sc).compaction_ready() != 0 {
            return 1;
        }
        if (*sc).memcg_full_walk() == 0 {
            (*sc).priority = initial;
            (*sc).set_memcg_full_walk(1);
            continue;
        }
        if (*sc).skipped_deactivate() != 0 {
            (*sc).priority = initial;
            (*sc).set_force_deactivate(1);
            (*sc).set_skipped_deactivate(0);
            continue;
        }
        if (*sc).memcg_low_skipped() != 0 {
            (*sc).priority = initial;
            (*sc).set_force_deactivate(0);
            (*sc).set_memcg_low_reclaim(1);
            (*sc).set_memcg_low_skipped(0);
            continue;
        }
        return 0;
    }
}
unsafe extern "C" fn allow_direct_reclaim(pgdat: *mut pglist_data) -> bool {
    if kswapd_test_hopeless(pgdat) {
        return true;
    }
    let mut reserve: ULong = 0;
    let mut free: ULong = 0;
    for zid in 0..=ZONE_NORMAL as usize {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid);
        if !managed_zone(z) {
            continue;
        }
        if zone_reclaimable_pages(z) == 0 && zone_page_state_snapshot(z, NR_FREE_PAGES) != 0 {
            continue;
        }
        reserve = reserve.wrapping_add(min_wmark_pages(z));
        free = free.wrapping_add(zone_page_state_snapshot(z, NR_FREE_PAGES));
    }
    if reserve == 0 {
        return true;
    }
    let ok = free > reserve / 2;
    if !ok && waitqueue_active(addr_of!((*pgdat).kswapd_wait)) {
        if read_zone_type(addr_of!((*pgdat).kswapd_highest_zoneidx)) > ZONE_NORMAL as i32 {
            write_zone_type(
                addr_of_mut!((*pgdat).kswapd_highest_zoneidx),
                ZONE_NORMAL as i32,
            );
        }
        wake_up_interruptible(addr_of_mut!((*pgdat).kswapd_wait));
    }
    ok
}
unsafe fn throttle_direct_reclaim(gfp: gfp_t, zl: *mut zonelist, mask: *const nodemask_t) -> bool {
    if (*current()).flags & PF_KTHREAD != 0 || fatal_signal_pending(current()) {
        return false;
    }
    let mut pgdat = null_mut();
    let mut zr = first_zones_zonelist(zl, gfp_zone(gfp) as i32, mask);
    while !(*zr).zone.is_null() {
        let z = (*zr).zone;
        zr = next_zones_zonelist(zr.add(1), gfp_zone(gfp) as i32, mask);
        if zone_idx(z) > ZONE_NORMAL as i32 {
            continue;
        }
        pgdat = (*z).zone_pgdat;
        if allow_direct_reclaim(pgdat) {
            return false;
        }
        break;
    }
    if pgdat.is_null() {
        return false;
    }
    count_vm_event(PGSCAN_DIRECT_THROTTLE);
    if gfp & __GFP_FS == 0 {
        wait_pfmemalloc_interruptible_timeout(pgdat, Some(allow_direct_reclaim), HZ as Long);
    } else {
        wait_pfmemalloc_killable(pgdat, Some(allow_direct_reclaim));
    }
    fatal_signal_pending(current())
}
const _: () = {
    assert!(MAX_PAGE_ORDER < i8::MAX as u32);
    assert!(DEF_PRIORITY <= i8::MAX as u32);
    assert!(MAX_NR_ZONES <= i8::MAX as u32);
};
#[no_mangle]
pub unsafe extern "C" fn try_to_free_pages(
    zl: *mut zonelist,
    order: i32,
    gfp: gfp_t,
    mask: *const nodemask_t,
) -> ULong {
    let mut sc: scan_control = zeroed();
    sc.nr_to_reclaim = SWAP_CLUSTER_MAX as ULong;
    sc.gfp_mask = current_gfp_context(gfp);
    sc.reclaim_idx = gfp_zone(gfp) as i8;
    sc.order = order as i8;
    sc.nodemask = mask;
    sc.priority = DEF_PRIORITY as i8;
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.set_may_swap(1);
    if throttle_direct_reclaim(sc.gfp_mask, zl, mask) {
        return 1;
    }
    set_task_reclaim_state(current(), &mut sc.reclaim_state);
    trace_mm_vmscan_direct_reclaim_begin(sc.gfp_mask, order, null_mut());
    let nr = do_try_to_free_pages(zl, &mut sc);
    trace_mm_vmscan_direct_reclaim_end(nr, null_mut());
    set_task_reclaim_state(current(), null_mut());
    nr
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn mem_cgroup_shrink_node(
    memcg: *mut mem_cgroup,
    gfp: gfp_t,
    noswap: bool,
    pgdat: *mut pglist_data,
    scanned: *mut ULong,
) -> ULong {
    let lv = mem_cgroup_lruvec(memcg, pgdat);
    let mut sc: scan_control = zeroed();
    sc.nr_to_reclaim = SWAP_CLUSTER_MAX as ULong;
    sc.target_mem_cgroup = memcg;
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.reclaim_idx = (MAX_NR_ZONES - 1) as i8;
    sc.set_may_swap((!noswap) as u32);
    warn_softlimit_reclaim_state((*current()).reclaim_state.is_null());
    sc.gfp_mask = (gfp & GFP_RECLAIM_MASK) | (GFP_HIGHUSER_MOVABLE & !GFP_RECLAIM_MASK);
    trace_mm_vmscan_memcg_softlimit_reclaim_begin(sc.gfp_mask, sc.order as i32, memcg);
    shrink_lruvec(lv, &mut sc);
    trace_mm_vmscan_memcg_softlimit_reclaim_end(sc.nr_reclaimed, memcg);
    *scanned = sc.nr_scanned;
    sc.nr_reclaimed
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn try_to_free_mem_cgroup_pages(
    memcg: *mut mem_cgroup,
    nr: ULong,
    gfp: gfp_t,
    options: u32,
    swappiness: *mut i32,
) -> ULong {
    let mut sc: scan_control = zeroed();
    sc.nr_to_reclaim = max(nr, SWAP_CLUSTER_MAX as ULong);
    sc.proactive_swappiness = swappiness;
    sc.gfp_mask =
        (current_gfp_context(gfp) & GFP_RECLAIM_MASK) | (GFP_HIGHUSER_MOVABLE & !GFP_RECLAIM_MASK);
    sc.reclaim_idx = (MAX_NR_ZONES - 1) as i8;
    sc.target_mem_cgroup = memcg;
    sc.priority = DEF_PRIORITY as i8;
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.set_may_swap((options & MEMCG_RECLAIM_MAY_SWAP != 0) as u32);
    sc.set_proactive((options & MEMCG_RECLAIM_PROACTIVE != 0) as u32);
    let zl = node_zonelist(numa_node_id(), sc.gfp_mask);
    set_task_reclaim_state(current(), &mut sc.reclaim_state);
    trace_mm_vmscan_memcg_reclaim_begin(sc.gfp_mask, 0, memcg);
    let saved = memalloc_noreclaim_save();
    let reclaimed = do_try_to_free_pages(zl, &mut sc);
    memalloc_noreclaim_restore(saved);
    trace_mm_vmscan_memcg_reclaim_end(reclaimed, memcg);
    set_task_reclaim_state(current(), null_mut());
    reclaimed
}
#[cfg(not(CONFIG_MEMCG))]
#[no_mangle]
pub unsafe extern "C" fn try_to_free_mem_cgroup_pages(
    _memcg: *mut mem_cgroup,
    _nr: ULong,
    _gfp: gfp_t,
    _options: u32,
    _swappiness: *mut i32,
) -> ULong {
    0
}
unsafe fn kswapd_age_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    #[cfg(CONFIG_LRU_GEN)]
    if lru_gen_enabled() || lru_gen_switching() {
        lru_gen_age_node(pgdat, sc);
        if !lru_gen_switching() {
            return;
        }
    }
    let lv = mem_cgroup_lruvec(null_mut(), pgdat);
    if !can_age_anon_pages(lv, sc) || !inactive_is_low(lv, LRU_INACTIVE_ANON) {
        return;
    }
    let mut memcg = mem_cgroup_iter(null_mut(), null_mut(), null_mut());
    loop {
        shrink_active_list(
            SWAP_CLUSTER_MAX as ULong,
            mem_cgroup_lruvec(memcg, pgdat),
            sc,
            LRU_ACTIVE_ANON,
        );
        memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
        if memcg.is_null() {
            break;
        }
    }
}
unsafe fn pgdat_watermark_boosted(pgdat: *mut pglist_data, highest: i32) -> bool {
    for zid in (0..=highest).rev() {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if managed_zone(z) && (*z).watermark_boost != 0 {
            return true;
        }
    }
    false
}
unsafe fn pgdat_balanced(pgdat: *mut pglist_data, order: i32, highest: i32) -> bool {
    let mut mark = ULong::MAX;
    for zid in 0..=highest {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if !managed_zone(z) {
            continue;
        }
        mark = if native_numa_balancing_mode() & NUMA_BALANCING_MEMORY_TIERING as i32 != 0 {
            promo_wmark_pages(z)
        } else {
            high_wmark_pages(z)
        };
        let item = if defrag_mode != 0 && order != 0 {
            NR_FREE_PAGES_BLOCKS
        } else {
            NR_FREE_PAGES
        };
        let mut free = zone_page_state(z, item);
        if (*z).percpu_drift_mark != 0 && free < (*z).percpu_drift_mark {
            free = zone_page_state_snapshot(z, item);
        }
        if __zone_watermark_ok(z, order as u32, mark, highest, 0, free as Long) {
            return true;
        }
    }
    mark == ULong::MAX
}
unsafe fn clear_pgdat_congested(pgdat: *mut pglist_data) {
    let lv = mem_cgroup_lruvec(null_mut(), pgdat);
    clear_bit(LRUVEC_NODE_CONGESTED as ULong, addr_of_mut!((*lv).flags));
    clear_bit(LRUVEC_CGROUP_CONGESTED as ULong, addr_of_mut!((*lv).flags));
    clear_bit(PGDAT_WRITEBACK as ULong, addr_of_mut!((*pgdat).flags));
}
unsafe fn prepare_kswapd_sleep(pgdat: *mut pglist_data, order: i32, highest: i32) -> bool {
    if waitqueue_active(addr_of!((*pgdat).pfmemalloc_wait)) {
        wake_up_all(addr_of_mut!((*pgdat).pfmemalloc_wait));
    }
    if kswapd_test_hopeless(pgdat) {
        return true;
    }
    if pgdat_balanced(pgdat, order, highest) {
        clear_pgdat_congested(pgdat);
        return true;
    }
    false
}
unsafe fn kswapd_shrink_node(pgdat: *mut pglist_data, sc: *mut scan_control) -> bool {
    let reclaimed = (*sc).nr_reclaimed;
    (*sc).nr_to_reclaim = 0;
    for zid in 0..=(*sc).reclaim_idx as i32 {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if managed_zone(z) {
            (*sc).nr_to_reclaim = (*sc)
                .nr_to_reclaim
                .wrapping_add(max(high_wmark_pages(z), SWAP_CLUSTER_MAX as ULong));
        }
    }
    shrink_node(pgdat, sc);
    if (*sc).order != 0 && (*sc).nr_reclaimed >= compact_gap((*sc).order as u32) {
        (*sc).order = 0;
    }
    max((*sc).nr_scanned, (*sc).nr_reclaimed.wrapping_sub(reclaimed)) >= (*sc).nr_to_reclaim
}
unsafe fn update_reclaim_active(pgdat: *mut pglist_data, highest: i32, active: bool) {
    for zid in 0..=highest {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if !managed_zone(z) {
            continue;
        }
        if active {
            set_bit(ZONE_RECLAIM_ACTIVE as ULong, addr_of_mut!((*z).flags));
        } else {
            clear_bit(ZONE_RECLAIM_ACTIVE as ULong, addr_of_mut!((*z).flags));
        }
    }
}
unsafe fn set_reclaim_active(pgdat: *mut pglist_data, highest: i32) {
    update_reclaim_active(pgdat, highest, true);
}
unsafe fn clear_reclaim_active(pgdat: *mut pglist_data, highest: i32) {
    update_reclaim_active(pgdat, highest, false);
}
unsafe fn balance_pgdat(pgdat: *mut pglist_data, order: i32, highest: i32) -> i32 {
    let mut sc: scan_control = zeroed();
    sc.gfp_mask = GFP_KERNEL;
    sc.order = order as i8;
    sc.set_may_unmap(1);
    let mut pflags = 0;
    let mut boosts = [0 as ULong; MAX_NR_ZONES as usize];
    let mut boost: ULong = 0;
    trace_mm_vmscan_balance_pgdat_begin((*pgdat).node_id, order, highest);
    set_task_reclaim_state(current(), &mut sc.reclaim_state);
    psi_memstall_enter(&mut pflags);
    fs_reclaim_acquire_balance();
    count_vm_event(PAGEOUTRUN);
    for zid in 0..=highest {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if !managed_zone(z) {
            continue;
        }
        boost = boost.wrapping_add((*z).watermark_boost);
        boosts[zid as usize] = (*z).watermark_boost;
    }
    let boosted = boost != 0;
    'out: {
        'restart: loop {
            set_reclaim_active(pgdat, highest);
            sc.priority = DEF_PRIORITY as i8;
            loop {
                let before = sc.nr_reclaimed;
                let mut raise = true;
                sc.reclaim_idx = highest as i8;
                if native_buffer_heads_over_limit() != 0 {
                    for zid in (0..MAX_NR_ZONES as usize).rev() {
                        if managed_zone((*pgdat).node_zones.as_mut_ptr().add(zid)) {
                            sc.reclaim_idx = zid as i8;
                            break;
                        }
                    }
                }
                let balanced = pgdat_balanced(pgdat, sc.order as i32, highest);
                if !balanced && boost != 0 {
                    boost = 0;
                    continue 'restart;
                }
                if boost == 0 && balanced {
                    break 'out;
                }
                if boost != 0 && sc.priority as i32 == DEF_PRIORITY as i32 - 2 {
                    raise = false;
                }
                sc.set_may_writepage((boost == 0) as u32);
                sc.set_may_swap((boost == 0) as u32);
                kswapd_age_node(pgdat, &mut sc);
                sc.nr_scanned = 0;
                let mut soft_scanned = 0;
                sc.nr_reclaimed = sc.nr_reclaimed.wrapping_add(memcg1_soft_limit_reclaim(
                    pgdat,
                    sc.order as i32,
                    sc.gfp_mask,
                    &mut soft_scanned,
                ));
                if kswapd_shrink_node(pgdat, &mut sc) {
                    raise = false;
                }
                if waitqueue_active(addr_of!((*pgdat).pfmemalloc_wait))
                    && allow_direct_reclaim(pgdat)
                {
                    wake_up_all(addr_of_mut!((*pgdat).pfmemalloc_wait));
                }
                fs_reclaim_release_freeze();
                let mut frozen = false;
                let stop = kthread_freezable_should_stop(&mut frozen);
                fs_reclaim_acquire_freeze();
                if frozen || stop {
                    break;
                }
                let reclaimed = sc.nr_reclaimed.wrapping_sub(before);
                boost -= min(boost, reclaimed);
                if boost != 0 && reclaimed == 0 {
                    break;
                }
                if raise || reclaimed == 0 {
                    sc.priority -= 1;
                }
                if sc.priority < 1 {
                    break;
                }
            }
            if sc.nr_reclaimed == 0
                && sc.priority < 1
                && sc.no_cache_trim_mode() == 0
                && sc.cache_trim_mode_failed() != 0
            {
                sc.set_no_cache_trim_mode(1);
                continue;
            }
            break;
        }
        if sc.nr_reclaimed == 0 && !boosted {
            let failures = atomic_inc_return(addr_of_mut!((*pgdat).kswapd_failures));
            trace_mm_vmscan_kswapd_reclaim_fail((*pgdat).node_id, failures);
        }
    }
    clear_reclaim_active(pgdat, highest);
    if boosted {
        for zid in 0..=highest as usize {
            if boosts[zid] == 0 {
                continue;
            }
            let z = (*pgdat).node_zones.as_mut_ptr().add(zid);
            let flags = spin_lock_irqsave(addr_of_mut!((*z).lock));
            (*z).watermark_boost -= min((*z).watermark_boost, boosts[zid]);
            spin_unlock_irqrestore(addr_of_mut!((*z).lock), flags);
        }
        wakeup_kcompactd(pgdat, native_pageblock_order() as i32, highest);
    }
    snapshot_refaults(null_mut(), pgdat);
    fs_reclaim_release_balance();
    psi_memstall_leave(&mut pflags);
    set_task_reclaim_state(current(), null_mut());
    trace_mm_vmscan_balance_pgdat_end((*pgdat).node_id, sc.order as i32, highest, sc.nr_reclaimed);
    sc.order as i32
}
unsafe fn kswapd_highest_zoneidx(pgdat: *mut pglist_data, previous: u32) -> u32 {
    let current = read_zone_type(addr_of!((*pgdat).kswapd_highest_zoneidx)) as u32;
    if current == MAX_NR_ZONES {
        previous
    } else {
        current
    }
}
unsafe fn kswapd_try_to_sleep(
    pgdat: *mut pglist_data,
    alloc_order: i32,
    reclaim_order: i32,
    highest: u32,
) {
    if freezing(current()) || kthread_should_stop() {
        return;
    }
    let mut remaining = 0;
    let mut wait: wait_queue_entry = zeroed();
    init_wait(&mut wait);
    prepare_to_wait(
        addr_of_mut!((*pgdat).kswapd_wait),
        &mut wait,
        TASK_INTERRUPTIBLE as i32,
    );
    if prepare_kswapd_sleep(pgdat, reclaim_order, highest as i32) {
        reset_isolation_suitable(pgdat);
        wakeup_kcompactd(pgdat, alloc_order, highest as i32);
        remaining = schedule_timeout((HZ / 10) as Long);
        if remaining != 0 {
            write_zone_type(
                addr_of_mut!((*pgdat).kswapd_highest_zoneidx),
                kswapd_highest_zoneidx(pgdat, highest) as i32,
            );
            if read_int(addr_of!((*pgdat).kswapd_order)) < reclaim_order {
                write_int(addr_of_mut!((*pgdat).kswapd_order), reclaim_order);
            }
        }
        finish_wait(addr_of_mut!((*pgdat).kswapd_wait), &mut wait);
        prepare_to_wait(
            addr_of_mut!((*pgdat).kswapd_wait),
            &mut wait,
            TASK_INTERRUPTIBLE as i32,
        );
    }
    if remaining == 0 && prepare_kswapd_sleep(pgdat, reclaim_order, highest as i32) {
        trace_mm_vmscan_kswapd_sleep((*pgdat).node_id);
        set_pgdat_normal_threshold(pgdat);
        if !kthread_should_stop() {
            schedule();
        }
        set_pgdat_pressure_threshold(pgdat);
    } else {
        count_vm_event(if remaining != 0 {
            KSWAPD_LOW_WMARK_HIT_QUICKLY
        } else {
            KSWAPD_HIGH_WMARK_HIT_QUICKLY
        });
    }
    finish_wait(addr_of_mut!((*pgdat).kswapd_wait), &mut wait);
}
unsafe extern "C" fn kswapd(p: *mut Void) -> i32 {
    let pgdat = p.cast::<pglist_data>();
    let task = current();
    let mut highest = MAX_NR_ZONES - 1;
    (*task).flags |= PF_MEMALLOC | PF_KSWAPD;
    set_freezable();
    write_int(addr_of_mut!((*pgdat).kswapd_order), 0);
    write_zone_type(
        addr_of_mut!((*pgdat).kswapd_highest_zoneidx),
        MAX_NR_ZONES as i32,
    );
    atomic_set(addr_of_mut!((*pgdat).nr_writeback_throttled), 0);
    'outer: loop {
        let mut alloc_order = read_int(addr_of!((*pgdat).kswapd_order)) as u32;
        let mut reclaim_order = alloc_order;
        highest = kswapd_highest_zoneidx(pgdat, highest);
        loop {
            kswapd_try_to_sleep(pgdat, alloc_order as i32, reclaim_order as i32, highest);
            alloc_order = read_int(addr_of!((*pgdat).kswapd_order)) as u32;
            highest = kswapd_highest_zoneidx(pgdat, highest);
            write_int(addr_of_mut!((*pgdat).kswapd_order), 0);
            write_zone_type(
                addr_of_mut!((*pgdat).kswapd_highest_zoneidx),
                MAX_NR_ZONES as i32,
            );
            let mut frozen = false;
            if kthread_freezable_should_stop(&mut frozen) {
                break 'outer;
            }
            if frozen {
                continue 'outer;
            }
            trace_mm_vmscan_kswapd_wake((*pgdat).node_id, highest as i32, alloc_order as i32);
            reclaim_order = balance_pgdat(pgdat, alloc_order as i32, highest as i32) as u32;
            if reclaim_order >= alloc_order {
                break;
            }
        }
    }
    (*task).flags &= !(PF_MEMALLOC | PF_KSWAPD);
    0
}
#[no_mangle]
pub unsafe extern "C" fn wakeup_kswapd(z: *mut zone, gfp: gfp_t, order: i32, highest: zone_type) {
    if !managed_zone(z) || !cpuset_zone_allowed(z, gfp) {
        return;
    }
    let pgdat = (*z).zone_pgdat;
    let idx = read_zone_type(addr_of!((*pgdat).kswapd_highest_zoneidx)) as u32;
    if idx == MAX_NR_ZONES || idx < highest {
        write_zone_type(
            addr_of_mut!((*pgdat).kswapd_highest_zoneidx),
            highest as i32,
        );
    }
    if read_int(addr_of!((*pgdat).kswapd_order)) < order {
        write_int(addr_of_mut!((*pgdat).kswapd_order), order);
    }
    if !waitqueue_active(addr_of!((*pgdat).kswapd_wait)) {
        return;
    }
    if kswapd_test_hopeless(pgdat)
        || (pgdat_balanced(pgdat, order, highest as i32)
            && !pgdat_watermark_boosted(pgdat, highest as i32))
    {
        if gfp & __GFP_DIRECT_RECLAIM == 0 {
            wakeup_kcompactd(pgdat, order, highest as i32);
        }
        return;
    }
    trace_mm_vmscan_wakeup_kswapd((*pgdat).node_id, highest as i32, order, gfp);
    wake_up_interruptible(addr_of_mut!((*pgdat).kswapd_wait));
}
#[no_mangle]
pub unsafe extern "C" fn kswapd_clear_hopeless(
    pgdat: *mut pglist_data,
    reason: kswapd_clear_hopeless_reason,
) {
    if atomic_xchg(addr_of_mut!((*pgdat).kswapd_failures), 0) != 0 {
        trace_mm_vmscan_kswapd_clear_hopeless((*pgdat).node_id, reason as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn kswapd_try_clear_hopeless(
    pgdat: *mut pglist_data,
    order: u32,
    highest: i32,
) {
    if pgdat_balanced(pgdat, order as i32, highest) {
        kswapd_clear_hopeless(
            pgdat,
            if current_is_kswapd() {
                KSWAPD_CLEAR_HOPELESS_KSWAPD
            } else {
                KSWAPD_CLEAR_HOPELESS_DIRECT
            },
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn kswapd_test_hopeless(pgdat: *mut pglist_data) -> bool {
    atomic_read(addr_of!((*pgdat).kswapd_failures)) >= MAX_RECLAIM_RETRIES as i32
}
#[cfg(CONFIG_HIBERNATION)]
#[no_mangle]
pub unsafe extern "C" fn shrink_all_memory(goal: ULong) -> ULong {
    let mut sc: scan_control = zeroed();
    sc.nr_to_reclaim = goal;
    sc.gfp_mask = GFP_HIGHUSER_MOVABLE;
    sc.reclaim_idx = (MAX_NR_ZONES - 1) as i8;
    sc.priority = DEF_PRIORITY as i8;
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.set_may_swap(1);
    sc.set_hibernation_mode(1);
    let zl = node_zonelist(numa_node_id(), sc.gfp_mask);
    fs_reclaim_acquire(sc.gfp_mask);
    let saved = memalloc_noreclaim_save();
    set_task_reclaim_state(current(), &mut sc.reclaim_state);
    let nr = do_try_to_free_pages(zl, &mut sc);
    set_task_reclaim_state(current(), null_mut());
    memalloc_noreclaim_restore(saved);
    fs_reclaim_release(sc.gfp_mask);
    nr
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn kswapd_run(nid: i32) {
    let pgdat = node_data(nid);
    pgdat_kswapd_lock(pgdat);
    if (*pgdat).kswapd.is_null() {
        (*pgdat).kswapd = create_kswapd(Some(kswapd), pgdat, nid);
        if is_err((*pgdat).kswapd.cast()) {
            error_kswapd_start(nid, (*pgdat).kswapd);
            bug_kswapd_boot(system_state < SYSTEM_RUNNING);
            (*pgdat).kswapd = null_mut();
        } else {
            wake_up_process((*pgdat).kswapd);
        }
    }
    pgdat_kswapd_unlock(pgdat);
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn kswapd_stop(nid: i32) {
    let pgdat = node_data(nid);
    pgdat_kswapd_lock(pgdat);
    let task = (*pgdat).kswapd;
    if !task.is_null() {
        kthread_stop(task);
        (*pgdat).kswapd = null_mut();
    }
    pgdat_kswapd_unlock(pgdat);
}
#[export_name = "rust_vs_kswapd_init"]
#[link_section = ".init.text"]
unsafe extern "C" fn kswapd_init() -> i32 {
    let mut nid = first_memory_node();
    while nid < MAX_NUMNODES as i32 {
        kswapd_run(nid);
        nid = next_memory_node(nid);
    }
    register_vmscan_sysctl();
    0
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut node_reclaim_mode: i32 = 0;
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub static mut sysctl_min_unmapped_ratio: i32 = 1;
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub static mut sysctl_min_slab_ratio: i32 = 5;
// This private policy literal is #define NODE_RECLAIM_PRIORITY 4 in the owner C.
#[cfg(CONFIG_NUMA)]
const NODE_RECLAIM_PRIORITY: i8 = 4;
#[cfg(CONFIG_NUMA)]
unsafe fn node_unmapped_file_pages(pgdat: *mut pglist_data) -> ULong {
    let mapped = node_page_state(pgdat, NR_FILE_MAPPED);
    let lru = node_page_state(pgdat, NR_INACTIVE_FILE)
        .wrapping_add(node_page_state(pgdat, NR_ACTIVE_FILE));
    if lru > mapped {
        lru - mapped
    } else {
        0
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn node_pagecache_reclaimable(pgdat: *mut pglist_data) -> ULong {
    let reclaimable = if node_reclaim_mode & RECLAIM_UNMAP as i32 != 0 {
        node_page_state(pgdat, NR_FILE_PAGES)
    } else {
        node_unmapped_file_pages(pgdat)
    };
    reclaimable - min(node_page_state(pgdat, NR_FILE_DIRTY), reclaimable)
}
#[cfg(CONFIG_NUMA)]
unsafe fn __node_reclaim(pgdat: *mut pglist_data, nr: ULong, sc: *mut scan_control) -> ULong {
    let task = current();
    let mut pflags = 0;
    trace_mm_vmscan_node_reclaim_begin((*pgdat).node_id, (*sc).order as i32, (*sc).gfp_mask);
    cond_resched();
    psi_memstall_enter(&mut pflags);
    delayacct_freepages_start();
    fs_reclaim_acquire((*sc).gfp_mask);
    let saved = memalloc_noreclaim_save();
    set_task_reclaim_state(task, addr_of_mut!((*sc).reclaim_state));
    if node_pagecache_reclaimable(pgdat) > (*pgdat).min_unmapped_pages
        || node_page_state_pages(pgdat, NR_SLAB_RECLAIMABLE_B) > (*pgdat).min_slab_pages
    {
        loop {
            shrink_node(pgdat, sc);
            if (*sc).nr_reclaimed >= nr {
                break;
            }
            (*sc).priority -= 1;
            if (*sc).priority < 0 {
                break;
            }
        }
    }
    set_task_reclaim_state(task, null_mut());
    memalloc_noreclaim_restore(saved);
    fs_reclaim_release((*sc).gfp_mask);
    delayacct_freepages_end();
    psi_memstall_leave(&mut pflags);
    trace_mm_vmscan_node_reclaim_end((*sc).nr_reclaimed, null_mut());
    (*sc).nr_reclaimed
}
#[cfg(not(CONFIG_NUMA))]
unsafe fn __node_reclaim(_pgdat: *mut pglist_data, _nr: ULong, _sc: *mut scan_control) -> ULong {
    0
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn node_reclaim(pgdat: *mut pglist_data, gfp: gfp_t, order: u32) -> ULong {
    let nr = (1 as ULong) << order;
    let mut sc: scan_control = zeroed();
    sc.nr_to_reclaim = max(nr, SWAP_CLUSTER_MAX as ULong);
    sc.gfp_mask = current_gfp_context(gfp);
    sc.order = order as i8;
    sc.priority = NODE_RECLAIM_PRIORITY;
    sc.set_may_writepage((node_reclaim_mode & RECLAIM_WRITE as i32 != 0) as u32);
    sc.set_may_unmap((node_reclaim_mode & RECLAIM_UNMAP as i32 != 0) as u32);
    sc.set_may_swap(1);
    sc.reclaim_idx = gfp_zone(gfp) as i8;
    if node_pagecache_reclaimable(pgdat) <= (*pgdat).min_unmapped_pages
        && node_page_state_pages(pgdat, NR_SLAB_RECLAIMABLE_B) <= (*pgdat).min_slab_pages
    {
        return 0;
    }
    if !gfpflags_allow_blocking(gfp) || (*current()).flags & PF_MEMALLOC != 0 {
        return 0;
    }
    if node_state((*pgdat).node_id, N_CPU) && (*pgdat).node_id != numa_node_id() {
        return 0;
    }
    if test_and_set_bit_lock(PGDAT_RECLAIM_LOCKED as ULong, addr_of_mut!((*pgdat).flags)) {
        return 0;
    }
    let ret = __node_reclaim(pgdat, nr, &mut sc);
    clear_bit_unlock(PGDAT_RECLAIM_LOCKED as ULong, addr_of_mut!((*pgdat).flags));
    count_vm_event(if ret >= nr {
        PGSCAN_ZONE_RECLAIM_SUCCESS
    } else {
        PGSCAN_ZONE_RECLAIM_FAILED
    });
    ret
}
#[no_mangle]
pub unsafe extern "C" fn user_proactive_reclaim(
    mut buf: *mut CChar,
    memcg: *mut mem_cgroup,
    pgdat: *mut pglist_data,
) -> i32 {
    let mut retries = MAX_RECLAIM_RETRIES as u32;
    let mut reclaimed: ULong = 0;
    let mut swappiness = -1;
    let mut args: [substring_t; MAX_OPT_ARGS as usize] = zeroed();
    let gfp = GFP_KERNEL;
    if buf.is_null()
        || (memcg.is_null() && pgdat.is_null())
        || (!memcg.is_null() && !pgdat.is_null())
    {
        return -(EINVAL as i32);
    }
    buf = strstrip(buf);
    let old = buf;
    let goal = (memparse(buf, &mut buf) / PAGE_SIZE as u64) as ULong;
    if buf == old {
        return -(EINVAL as i32);
    }
    buf = strstrip(buf);
    loop {
        let start = strsep(&mut buf, b" \0".as_ptr().cast());
        if start.is_null() {
            break;
        }
        if strlen(start) == 0 {
            continue;
        }
        match match_reclaim_token(start, args.as_mut_ptr()) as u32 {
            MEMORY_RECLAIM_SWAPPINESS => {
                if match_int(args.as_mut_ptr(), &mut swappiness) != 0
                    || swappiness < MIN_SWAPPINESS as i32
                    || swappiness > MAX_SWAPPINESS as i32
                {
                    return -(EINVAL as i32);
                }
            }
            MEMORY_RECLAIM_SWAPPINESS_MAX => {
                swappiness = SWAPPINESS_ANON_ONLY as i32;
            }
            _ => return -(EINVAL as i32),
        }
    }
    while reclaimed < goal {
        let batch = (goal - reclaimed) / 4;
        if signal_pending(current()) {
            return -(ERESTARTSYS as i32);
        }
        if !memcg.is_null() && memcg_is_dying(memcg) {
            return -(EAGAIN as i32);
        }
        if retries == 0 {
            lru_add_drain_all();
        }
        let swappiness_ptr = if swappiness == -1 {
            null_mut()
        } else {
            addr_of_mut!(swappiness)
        };
        let nr = if !memcg.is_null() {
            try_to_free_mem_cgroup_pages(
                memcg,
                batch,
                gfp,
                MEMCG_RECLAIM_MAY_SWAP | MEMCG_RECLAIM_PROACTIVE,
                swappiness_ptr,
            )
        } else {
            let mut sc: scan_control = zeroed();
            sc.gfp_mask = current_gfp_context(gfp);
            sc.reclaim_idx = gfp_zone(gfp) as i8;
            sc.proactive_swappiness = swappiness_ptr;
            sc.priority = DEF_PRIORITY as i8;
            sc.nr_to_reclaim = max(batch, SWAP_CLUSTER_MAX as ULong);
            sc.set_may_writepage(1);
            sc.set_may_unmap(1);
            sc.set_may_swap(1);
            sc.set_proactive(1);
            if test_and_set_bit_lock(PGDAT_RECLAIM_LOCKED as ULong, addr_of_mut!((*pgdat).flags)) {
                return -(EBUSY as i32);
            }
            let nr = __node_reclaim(pgdat, batch, &mut sc);
            clear_bit_unlock(PGDAT_RECLAIM_LOCKED as ULong, addr_of_mut!((*pgdat).flags));
            nr
        };
        if nr == 0 {
            let previous = retries;
            retries = retries.wrapping_sub(1);
            if previous == 0 {
                return -(EAGAIN as i32);
            }
        }
        reclaimed = reclaimed.wrapping_add(nr);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn check_move_unevictable_folios(batch: *mut folio_batch) {
    let mut lv = null_mut();
    let mut scanned: i32 = 0;
    let mut rescued: i32 = 0;
    for i in 0..(*batch).nr as usize {
        let f = (*batch).folios[i];
        let nr = folio_nr_pages(f) as i32;
        scanned = scanned.wrapping_add(nr);
        if !folio_test_clear_lru(f) {
            continue;
        }
        lv = folio_lruvec_relock_irq(f, lv);
        if folio_evictable(f) && folio_test_unevictable(f) {
            lruvec_del_folio(lv, f);
            folio_clear_unevictable(f);
            lruvec_add_folio(lv, f);
            rescued = rescued.wrapping_add(nr);
        }
        folio_set_lru(f);
    }
    if !lv.is_null() {
        __count_vm_events(UNEVICTABLE_PGRESCUED, rescued as ULong);
        __count_vm_events(UNEVICTABLE_PGSCANNED, scanned as ULong);
        lruvec_unlock_irq(lv);
    } else if scanned != 0 {
        count_vm_events(UNEVICTABLE_PGSCANNED, scanned as ULong);
    }
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
#[export_name = "rust_vs_reclaim_store"]
unsafe extern "C" fn reclaim_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const CChar,
    count: usize,
) -> isize {
    let ret = user_proactive_reclaim(buf.cast_mut(), null_mut(), node_data((*dev).id));
    if ret != 0 {
        ret as isize
    } else {
        count as isize
    }
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
#[no_mangle]
pub unsafe extern "C" fn reclaim_register_node(n: *mut node) -> i32 {
    device_create_reclaim_file(addr_of_mut!((*n).dev))
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
#[no_mangle]
pub unsafe extern "C" fn reclaim_unregister_node(n: *mut node) {
    device_remove_reclaim_file(addr_of_mut!((*n).dev));
}
