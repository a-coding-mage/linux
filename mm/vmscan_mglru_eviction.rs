// SPDX-License-Identifier: GPL-2.0-only
// Original mm/vmscan.c eviction and reclaim orchestration.
unsafe fn sort_folio(v: *mut lruvec, f: *mut folio, sc: *mut scan_control, tier_idx: i32) -> bool {
    let mut gen = folio_lru_gen(f) as usize;
    let typ = folio_is_file_lru(f) as usize;
    let zone = folio_zonenum(f) as usize;
    let delta = folio_nr_pages(f) as ULong;
    let refs = folio_lru_refs(f);
    let ws = folio_test_workingset(f);
    let tier = lru_tier_from_refs(refs, ws);
    let g = addr_of_mut!((*v).lrugen);
    folio_warn!(7, gen >= MAX_NR_GENS as usize, f);
    if !folio_evictable(f) {
        return false;
    }
    if gen != lru_gen_from_seq((*g).min_seq[typ]) as usize {
        list_move(folio_lru_ptr(f), addr_of_mut!((*g).folios[gen][typ][zone]));
        return true;
    }
    if tier > tier_idx || refs + ws as i32 == (1 << LRU_REFS_WIDTH) + 1 {
        gen = folio_inc_gen(v, f);
        list_move(folio_lru_ptr(f), addr_of_mut!((*g).folios[gen][typ][zone]));
        if refs + ws as i32 != (1 << LRU_REFS_WIDTH) + 1 {
            let hist = lru_hist_from_seq((*g).min_seq[typ]) as usize;
            wr(
                addr_of_mut!((*g).protected[hist][typ][tier as usize]),
                (*g).protected[hist][typ][tier as usize].wrapping_add(delta),
            );
        }
        return true;
    }
    if zone as i32 > (*sc).reclaim_idx as i32 {
        gen = folio_inc_gen(v, f);
        list_move_tail(folio_lru_ptr(f), addr_of_mut!((*g).folios[gen][typ][zone]));
        return true;
    }
    false
}
unsafe fn isolate_folio(v: *mut lruvec, f: *mut folio, _sc: *mut scan_control) -> bool {
    if !folio_try_get(f) {
        return false;
    }
    if !folio_test_clear_lru(f) {
        folio_put(f);
        return false;
    }
    if !folio_test_referenced(f) {
        rust_mglru_set_mask_bits(folio_flags_ptr(f), LRU_REFS_MASK as ULong, 0);
    }
    let success = lru_gen_del_folio(v, f, true);
    folio_warn!(8, !success, f);
    true
}
unsafe fn scan_folios(
    nr: ULong,
    v: *mut lruvec,
    sc: *mut scan_control,
    typ: usize,
    tier: i32,
    list: *mut list_head,
    isolatedp: *mut i32,
) -> i32 {
    let mut sorted = 0i32;
    let mut scanned = 0i32;
    let mut isolated = 0i32;
    let mut skipped = 0i32;
    let mut remaining = nr;
    let g = addr_of_mut!((*v).lrugen);
    vmwarn!(31, nr > MAX_LRU_BATCH as ULong);
    vmwarn!(32, !list_empty(list));
    if get_nr_gens(v, typ) == MIN_NR_GENS as i32 {
        return 0;
    }
    let gen = lru_gen_from_seq((*g).min_seq[typ]) as usize;
    for i in (1..=MAX_NR_ZONES as usize).rev() {
        let mut moved: list_head = zeroed();
        init_list_head(&mut moved);
        let mut skipped_zone = 0i32;
        let zone = ((*sc).reclaim_idx as usize + i) % MAX_NR_ZONES as usize;
        let head = addr_of_mut!((*g).folios[gen][typ][zone]);
        while !list_empty(head) {
            let f = lru_to_folio(head);
            let delta = folio_nr_pages(f) as i32;
            folio_warn!(9, folio_test_unevictable(f), f);
            folio_warn!(10, folio_test_active(f), f);
            folio_warn!(11, folio_is_file_lru(f) as usize != typ, f);
            folio_warn!(12, folio_zonenum(f) as usize != zone, f);
            scanned = scanned.wrapping_add(delta);
            if sort_folio(v, f, sc, tier) {
                sorted = sorted.wrapping_add(delta);
            } else if isolate_folio(v, f, sc) {
                list_add(folio_lru_ptr(f), list);
                isolated = isolated.wrapping_add(delta);
            } else {
                list_move(folio_lru_ptr(f), &mut moved);
                skipped_zone = skipped_zone.wrapping_add(delta);
            }
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 || max(isolated, skipped_zone) >= MIN_LRU_BATCH as i32 {
                break;
            }
        }
        if skipped_zone != 0 {
            list_splice(&mut moved, head);
            __count_zid_vm_events(PGSCAN_SKIP as _, zone as _, skipped_zone as _);
            skipped = skipped.wrapping_add(skipped_zone);
        }
        if remaining == 0 || isolated >= MIN_LRU_BATCH as i32 {
            break;
        }
    }
    let item = PGSCAN_KSWAPD as i32 + reclaimer_offset(sc) as i32;
    mod_lruvec_state(v, item as _, isolated as _);
    mod_lruvec_state(v, PGREFILL as _, sorted as _);
    mod_lruvec_state(v, (PGSCAN_ANON as usize + typ) as _, isolated as _);
    rust_mglru_trace_isolate(
        (*sc).reclaim_idx as i32,
        (*sc).order as i32,
        nr,
        scanned as ULong,
        skipped as ULong,
        isolated as ULong,
        if typ != 0 {
            LRU_INACTIVE_FILE as i32
        } else {
            LRU_INACTIVE_ANON as i32
        },
    );
    *isolatedp = isolated;
    scanned
}
unsafe fn get_tier_idx(v: *mut lruvec, typ: usize) -> i32 {
    let mut sp: ctrl_pos = zeroed();
    let mut pv: ctrl_pos = zeroed();
    read_ctrl_pos(v, typ, 0, 2, &mut sp);
    let mut tier = 1usize;
    while tier < MAX_NR_TIERS as usize {
        read_ctrl_pos(v, typ, tier, 3, &mut pv);
        if !positive_ctrl_err(&sp, &pv) {
            break;
        }
        tier += 1;
    }
    tier as i32 - 1
}
unsafe fn get_type_to_scan(v: *mut lruvec, sw: i32) -> usize {
    let mut sp: ctrl_pos = zeroed();
    let mut pv: ctrl_pos = zeroed();
    if sw <= MIN_SWAPPINESS as i32 + 1 {
        return LRU_GEN_FILE as usize;
    }
    if sw >= MAX_SWAPPINESS as i32 {
        return LRU_GEN_ANON as usize;
    }
    read_ctrl_pos(v, LRU_GEN_ANON as usize, MAX_NR_TIERS as usize, sw, &mut sp);
    read_ctrl_pos(
        v,
        LRU_GEN_FILE as usize,
        MAX_NR_TIERS as usize,
        MAX_SWAPPINESS as i32 - sw,
        &mut pv,
    );
    positive_ctrl_err(&sp, &pv) as usize
}
unsafe fn isolate_folios(
    nr: ULong,
    v: *mut lruvec,
    sc: *mut scan_control,
    sw: i32,
    list: *mut list_head,
    isolated: *mut i32,
    isolate_type: *mut usize,
    isolate_scanned: *mut i32,
) -> i32 {
    let mut total = 0i32;
    let mut typ = get_type_to_scan(v, sw);
    for _ in min_type(sw)..=max_type(sw) {
        let tier = get_tier_idx(v, typ);
        let scanned = scan_folios(nr, v, sc, typ, tier, list, isolated);
        total = total.wrapping_add(scanned);
        if *isolated != 0 {
            *isolate_type = typ;
            *isolate_scanned = scanned;
            break;
        }
        if scanned == 0 {
            typ = (typ == 0) as usize;
        }
    }
    total
}
unsafe fn evict_folios(nr: ULong, v: *mut lruvec, sc: *mut scan_control, sw: i32) -> i32 {
    let mut list: list_head = zeroed();
    let mut clean: list_head = zeroed();
    init_list_head(&mut list);
    init_list_head(&mut clean);
    let mut stat: reclaim_stat = zeroed();
    let mut isolated = 0i32;
    let mut typ = 0usize;
    let mut type_scanned = 0;
    let mut total_reclaimed: ULong = 0;
    let mut skip_retry = false;
    let memcg = lruvec_memcg(v);
    let pgdat = lruvec_pgdat(v);
    lruvec_lock_irq(v);
    try_to_inc_min_seq(v, sw);
    let scanned = isolate_folios(
        nr,
        v,
        sc,
        sw,
        &mut list,
        &mut isolated,
        &mut typ,
        &mut type_scanned,
    );
    let nr_isolated = isolated;
    if scanned != 0 {
        try_to_inc_min_seq(v, sw);
    }
    lruvec_unlock_irq(v);
    if list_empty(&list) {
        return scanned;
    }
    loop {
        let reclaimed = shrink_folio_list(&mut list, pgdat, sc, &mut stat, false, memcg);
        (*sc).nr_reclaimed = (*sc).nr_reclaimed.wrapping_add(reclaimed as ULong);
        total_reclaimed = total_reclaimed.wrapping_add(reclaimed as ULong);
        if isolated != 0 {
            handle_reclaim_writeback(isolated as ULong, pgdat, sc, &mut stat);
        }
        rust_mglru_trace_shrink_inactive(
            (*pgdat).node_id,
            type_scanned as ULong,
            reclaimed as ULong,
            &mut stat,
            (*sc).priority as i32,
            if typ != 0 {
                LRU_INACTIVE_FILE as i32
            } else {
                LRU_INACTIVE_ANON as i32
            },
        );
        let head = addr_of_mut!(list);
        let mut pos = list.prev;
        while pos != head {
            let next = (*pos).prev;
            let f = rust_mglru_folio_from_lru(pos);
            let seq = min_seq(v);
            if folio_evictable(f) {
                if !skip_retry
                    && !folio_test_active(f)
                    && !folio_mapped(f)
                    && !folio_test_dirty(f)
                    && !folio_test_writeback(f)
                {
                    list_move(folio_lru_ptr(f), &mut clean);
                } else if lru_gen_folio_seq(v, f, false) == seq[typ] {
                    rust_mglru_set_mask_bits(
                        folio_flags_ptr(f),
                        LRU_REFS_FLAGS as ULong,
                        1 << PG_active,
                    );
                }
            }
            pos = next;
        }
        move_folios_to_lru(&mut list);
        let w = (*(*current()).reclaim_state).mm_walk;
        if !w.is_null() && (*w).batched != 0 {
            (*w).lruvec = v;
            reset_batch_size(w);
        }
        mod_lruvec_state(
            v,
            (PGDEMOTE_KSWAPD as i32 + reclaimer_offset(sc) as i32) as _,
            stat.nr_demoted as _,
        );
        mod_lruvec_state(
            v,
            (PGSTEAL_KSWAPD as i32 + reclaimer_offset(sc) as i32) as _,
            reclaimed as _,
        );
        mod_lruvec_state(v, (PGSTEAL_ANON as usize + typ) as _, reclaimed as _);
        list_splice_init(&mut clean, &mut list);
        if list_empty(&list) {
            break;
        }
        skip_retry = true;
        isolated = 0;
    }
    if nr_isolated as ULong > total_reclaimed {
        mod_lruvec_state(
            v,
            (PGROTATE_ANON as usize + typ) as _,
            (nr_isolated as ULong - total_reclaimed) as _,
        );
    }
    scanned
}
unsafe fn should_run_aging(v: *mut lruvec, max: ULong, sc: *mut scan_control, sw: i32) -> bool {
    let seq = min_seq(v);
    let floor = evictable_min_seq(&seq, sw).wrapping_add(MIN_NR_GENS as ULong);
    if floor > max {
        return true;
    }
    if (*sc).priority as i32 == DEF_PRIORITY as i32 {
        return false;
    }
    floor == max
}
unsafe fn get_nr_to_scan(
    v: *mut lruvec,
    sc: *mut scan_control,
    memcg: *mut mem_cgroup,
    sw: i32,
) -> Long {
    let pgdat = lruvec_pgdat(v);
    if sw == SWAPPINESS_ANON_ONLY as i32 {
        warn!(7, (*sc).proactive() == 0);
        if !can_reclaim_anon_pages(memcg, (*pgdat).node_id, sc) {
            return 0;
        }
    }
    let evictable = lruvec_evictable_size(v, sw);
    if !mem_cgroup_online(memcg) {
        return evictable as Long;
    }
    (apply_proportional_protection(memcg, sc, evictable) >> (*sc).priority) as Long
}
unsafe fn should_abort_scan(v: *mut lruvec, sc: *mut scan_control) -> bool {
    if (*sc).proactive() != 0 && signal_pending(current()) {
        return true;
    }
    if (*sc).nr_reclaimed >= max((*sc).nr_to_reclaim, compact_gap((*sc).order as _) as ULong) {
        return true;
    }
    if !current_is_kswapd() || (*sc).order != 0 {
        return false;
    }
    let mark = if rust_mglru_numa_balancing_mode() & NUMA_BALANCING_MEMORY_TIERING as i32 != 0 {
        WMARK_PROMO
    } else {
        WMARK_HIGH
    };
    for i in 0..=(*sc).reclaim_idx as usize {
        let zone = (*lruvec_pgdat(v)).node_zones.as_mut_ptr().add(i);
        let size = rust_mglru_wmark_pages(zone, mark as i32) + MIN_LRU_BATCH as ULong;
        if managed_zone(zone) && !zone_watermark_ok(zone, 0, size, (*sc).reclaim_idx as _, 0) {
            return false;
        }
    }
    true
}
unsafe fn try_to_shrink_lruvec(v: *mut lruvec, sc: *mut scan_control) -> bool {
    let mut need_rotate = false;
    let mut should_age = false;
    let sw = get_swappiness(v, sc);
    let memcg = lruvec_memcg(v);
    let mut nr = get_nr_to_scan(v, sc, memcg, sw);
    while nr > 0 {
        let max = max_seq(v);
        if mem_cgroup_below_min((*sc).target_mem_cgroup, memcg) {
            need_rotate = true;
            break;
        }
        if should_run_aging(v, max, sc, sw) {
            if try_to_inc_max_seq(v, max, sw, false) {
                need_rotate = true;
            }
            should_age = true;
        }
        let batch = min(nr, MIN_LRU_BATCH as Long);
        let delta = evict_folios(batch as ULong, v, sc, sw);
        if delta == 0 || should_abort_scan(v, sc) || root_reclaim(sc) && should_age {
            break;
        }
        nr = nr.wrapping_sub(delta as Long);
        cond_resched();
    }
    need_rotate
}
unsafe fn shrink_one(v: *mut lruvec, sc: *mut scan_control) -> i32 {
    let scanned = (*sc).nr_scanned;
    let reclaimed = (*sc).nr_reclaimed;
    let memcg = lruvec_memcg(v);
    let pgdat = lruvec_pgdat(v);
    if mem_cgroup_below_min(null_mut(), memcg) {
        return MEMCG_LRU_YOUNG;
    }
    if mem_cgroup_below_low(null_mut(), memcg) {
        if rd(addr_of!((*v).lrugen.seg)) as i32 != MEMCG_LRU_TAIL {
            return MEMCG_LRU_TAIL;
        }
        memcg_memory_event(memcg, MEMCG_LOW as _);
    }
    let rotate = try_to_shrink_lruvec(v, sc);
    shrink_slab((*sc).gfp_mask, (*pgdat).node_id, memcg, (*sc).priority as _);
    if (*sc).proactive() == 0 {
        vmpressure(
            (*sc).gfp_mask,
            (*sc).order as _,
            memcg,
            false,
            (*sc).nr_scanned.wrapping_sub(scanned),
            (*sc).nr_reclaimed.wrapping_sub(reclaimed),
        );
    }
    flush_reclaim_state(sc);
    if rotate && mem_cgroup_online(memcg) {
        return MEMCG_LRU_YOUNG;
    }
    if !rotate && lruvec_is_sizable(v, sc) {
        return MEMCG_LRU_NOP;
    }
    if rd(addr_of!((*v).lrugen.seg)) as i32 != MEMCG_LRU_TAIL {
        MEMCG_LRU_TAIL
    } else {
        MEMCG_LRU_YOUNG
    }
}
#[cfg(CONFIG_MEMCG)]
unsafe fn shrink_many(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let gen = get_memcg_gen(rd(addr_of!((*pgdat).memcg_lru.seq)));
    let first_bin = get_random_u32_below(MEMCG_NR_BINS as u32) as usize;
    let mut bin = first_bin;
    loop {
        let mut op = 0;
        let mut memcg = null_mut();
        let mut v: *mut lruvec = null_mut();
        rcu_read_lock();
        let mut pos = rust_mglru_hlist_first_rcu(addr_of!((*pgdat).memcg_lru.fifo[gen][bin]));
        while !rust_mglru_is_a_nulls(pos) {
            let g = rust_mglru_lrugen_from_list(pos);
            if op != 0 {
                lru_gen_rotate_memcg(v, op);
                op = 0;
            }
            mem_cgroup_put(memcg);
            memcg = null_mut();
            if gen == rd(addr_of!((*g).gen)) as usize {
                v = rust_mglru_lruvec_from_lrugen(g);
                memcg = lruvec_memcg(v);
                if !mem_cgroup_tryget(memcg) {
                    lru_gen_release_memcg(memcg);
                    memcg = null_mut();
                } else {
                    rcu_read_unlock();
                    op = shrink_one(v, sc);
                    rcu_read_lock();
                    if should_abort_scan(v, sc) {
                        break;
                    }
                }
            }
            pos = rust_mglru_hlist_next_rcu(pos);
        }
        rcu_read_unlock();
        if op != 0 {
            lru_gen_rotate_memcg(v, op);
        }
        mem_cgroup_put(memcg);
        if !rust_mglru_is_a_nulls(pos) {
            return;
        }
        if gen as ULong != rust_mglru_get_nulls_value(pos) {
            continue;
        }
        bin = get_memcg_bin(bin + 1);
        if bin == first_bin {
            return;
        }
    }
}
pub(super) unsafe fn lru_gen_shrink_lruvec(v: *mut lruvec, sc: *mut scan_control) {
    let mut plug: blk_plug = zeroed();
    vmwarn!(33, root_reclaim(sc));
    vmwarn!(34, (*sc).may_writepage() == 0 || (*sc).may_unmap() == 0);
    lru_add_drain();
    blk_start_plug(&mut plug);
    set_mm_walk(null_mut(), (*sc).proactive() != 0);
    if try_to_shrink_lruvec(v, sc) {
        lru_gen_rotate_memcg(v, MEMCG_LRU_YOUNG);
    }
    clear_mm_walk();
    blk_finish_plug(&mut plug);
}
pub(super) unsafe fn lru_gen_shrink_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let mut plug: blk_plug = zeroed();
    let reclaimed = (*sc).nr_reclaimed;
    vmwarn!(35, !root_reclaim(sc));
    if (*sc).may_writepage() != 0 && (*sc).may_unmap() != 0 {
        lru_add_drain();
        blk_start_plug(&mut plug);
        set_mm_walk(pgdat, (*sc).proactive() != 0);
        set_initial_priority(pgdat, sc);
        if current_is_kswapd() {
            (*sc).nr_reclaimed = 0;
        }
        #[cfg(CONFIG_MEMCG)]
        if mem_cgroup_disabled() {
            shrink_one(addr_of_mut!((*pgdat).__lruvec), sc);
        } else {
            shrink_many(pgdat, sc);
        }
        #[cfg(not(CONFIG_MEMCG))]
        shrink_one(addr_of_mut!((*pgdat).__lruvec), sc);
        if current_is_kswapd() {
            (*sc).nr_reclaimed = (*sc).nr_reclaimed.wrapping_add(reclaimed);
        }
        clear_mm_walk();
        blk_finish_plug(&mut plug);
    }
    if (*sc).nr_reclaimed > reclaimed {
        kswapd_try_clear_hopeless(pgdat, (*sc).order as _, (*sc).reclaim_idx as _);
    }
}
