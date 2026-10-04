// SPDX-License-Identifier: GPL-2.0-only
// Native source mm/vmscan.c:203–2690, frozen e1d84f501551.
#[no_mangle]
pub static mut vm_swappiness: i32 = 60;
unsafe fn sc_swappiness(sc: *mut scan_control, memcg: *mut mem_cgroup) -> i32 {
    if (*sc).proactive() != 0 && !(*sc).proactive_swappiness.is_null() {
        *(*sc).proactive_swappiness
    } else {
        mem_cgroup_swappiness(memcg)
    }
}
unsafe fn cgroup_reclaim(sc: *mut scan_control) -> bool {
    #[cfg(CONFIG_MEMCG)]
    {
        return !(*sc).target_mem_cgroup.is_null();
    }
    #[cfg(not(CONFIG_MEMCG))]
    {
        false
    }
}
unsafe fn root_reclaim(sc: *mut scan_control) -> bool {
    #[cfg(CONFIG_MEMCG)]
    {
        return (*sc).target_mem_cgroup.is_null() || mem_cgroup_is_root((*sc).target_mem_cgroup);
    }
    #[cfg(not(CONFIG_MEMCG))]
    {
        true
    }
}
unsafe fn writeback_throttling_sane(sc: *mut scan_control) -> bool {
    #[cfg(CONFIG_MEMCG)]
    {
        if !cgroup_reclaim(sc) {
            return true;
        }
        #[cfg(CONFIG_CGROUP_WRITEBACK)]
        {
            if memory_cgroup_on_dfl() {
                return true;
            }
        }
        false
    }
    #[cfg(not(CONFIG_MEMCG))]
    {
        true
    }
}
unsafe fn is_exec_file_folio(f: *const folio, flags: *const vma_flags_t) -> bool {
    vma_flags_test(flags, VMA_EXEC_BIT) && folio_is_file_lru(f)
}
unsafe fn set_task_reclaim_state(task: *mut task_struct, rs: *mut reclaim_state) {
    warn_reclaim_overwrite(!rs.is_null() && !(*task).reclaim_state.is_null());
    warn_reclaim_null(rs.is_null() && (*task).reclaim_state.is_null());
    (*task).reclaim_state = rs;
}
unsafe fn flush_reclaim_state(sc: *mut scan_control) {
    let rs = (*current()).reclaim_state;
    if !rs.is_null() && root_reclaim(sc) {
        (*sc).nr_reclaimed = (*sc).nr_reclaimed.wrapping_add((*rs).reclaimed);
        (*rs).reclaimed = 0;
    }
}
unsafe fn can_demote(nid: i32, sc: *mut scan_control, memcg: *mut mem_cgroup) -> bool {
    let pgdat = node_data(nid);
    let mut allowed: nodemask_t = zeroed();
    if pgdat.is_null() || !native_numa_demotion_enabled() {
        return false;
    }
    if !sc.is_null() && (*sc).no_demotion() != 0 {
        return false;
    }
    node_get_allowed_targets(pgdat, &mut allowed);
    if nodes_empty(&allowed) {
        return false;
    }
    mem_cgroup_node_filter_allowed(memcg, &mut allowed);
    !nodes_empty(&allowed)
}
unsafe fn can_reclaim_anon_pages(memcg: *mut mem_cgroup, nid: i32, sc: *mut scan_control) -> bool {
    if memcg.is_null() {
        if get_nr_swap_pages() > 0 {
            return true;
        }
    } else if mem_cgroup_get_nr_swap_pages(memcg) > 0 {
        return true;
    }
    can_demote(nid, sc, memcg)
}
#[no_mangle]
pub unsafe extern "C" fn zone_reclaimable_pages(z: *mut zone) -> ULong {
    let mut nr = zone_page_state_snapshot(z, NR_ZONE_INACTIVE_FILE)
        .wrapping_add(zone_page_state_snapshot(z, NR_ZONE_ACTIVE_FILE));
    if can_reclaim_anon_pages(null_mut(), zone_to_nid(z), null_mut()) {
        nr = nr
            .wrapping_add(zone_page_state_snapshot(z, NR_ZONE_INACTIVE_ANON))
            .wrapping_add(zone_page_state_snapshot(z, NR_ZONE_ACTIVE_ANON));
    }
    nr
}
#[no_mangle]
pub unsafe extern "C" fn lruvec_lru_size(lv: *mut lruvec, lru: lru_list, zone_idx: i32) -> ULong {
    let mut size: ULong = 0;
    let pgdat = lruvec_pgdat(lv);
    for zid in 0..=zone_idx {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid as usize);
        if !managed_zone(z) {
            continue;
        }
        size = size.wrapping_add(if !mem_cgroup_disabled() {
            mem_cgroup_get_zone_lru_size(lv, lru, zid)
        } else {
            zone_page_state(z, NR_ZONE_LRU_BASE + lru)
        });
    }
    size
}
unsafe fn drop_slab_node(nid: i32) -> ULong {
    let mut freed: ULong = 0;
    let mut memcg = mem_cgroup_iter(null_mut(), null_mut(), null_mut());
    loop {
        freed = freed.wrapping_add(shrink_slab(GFP_KERNEL, nid, memcg, 0));
        memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
        if memcg.is_null() {
            return freed;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn drop_slab() {
    let mut shift = 0;
    loop {
        let mut freed: ULong = 0;
        let mut nid = first_online_node();
        while nid < MAX_NUMNODES as i32 {
            if fatal_signal_pending(current()) {
                return;
            }
            freed = freed.wrapping_add(drop_slab_node(nid));
            nid = next_online_node(nid);
        }
        let more = freed.wrapping_shr(shift) > 1;
        shift += 1;
        if !more {
            break;
        }
    }
}
unsafe fn reclaimer_offset(sc: *mut scan_control) -> u32 {
    if current_is_kswapd() {
        0
    } else if current_is_khugepaged() {
        PGSTEAL_KHUGEPAGED - PGSTEAL_KSWAPD
    } else if (*sc).proactive() != 0 {
        PGSTEAL_PROACTIVE - PGSTEAL_KSWAPD
    } else {
        PGSTEAL_DIRECT - PGSTEAL_KSWAPD
    }
}
unsafe fn handle_write_error(mapping: *mut address_space, f: *mut folio, error: i32) {
    folio_lock(f);
    if folio_mapping(f) == mapping {
        mapping_set_error(mapping, error);
    }
    folio_unlock(f);
}
unsafe fn skip_throttle_noprogress(pgdat: *mut pglist_data) -> bool {
    let mut reclaimable: i32 = 0;
    let mut write_pending: i32 = 0;
    if kswapd_test_hopeless(pgdat) {
        return true;
    }
    for zid in 0..MAX_NR_ZONES as usize {
        let z = (*pgdat).node_zones.as_mut_ptr().add(zid);
        if !managed_zone(z) {
            continue;
        }
        reclaimable = reclaimable.wrapping_add(zone_reclaimable_pages(z) as i32);
        write_pending =
            write_pending.wrapping_add(zone_page_state_snapshot(z, NR_ZONE_WRITE_PENDING) as i32);
    }
    write_pending.wrapping_mul(2) <= reclaimable
}
#[no_mangle]
pub unsafe extern "C" fn reclaim_throttle(pgdat: *mut pglist_data, reason: vmscan_throttle_state) {
    if !current_is_kswapd() && (*current()).flags & (PF_USER_WORKER | PF_KTHREAD) != 0 {
        cond_resched();
        return;
    }
    let timeout: Long;
    if reason == VMSCAN_THROTTLE_WRITEBACK {
        timeout = (HZ / 10) as Long;
        if atomic_inc_return(addr_of_mut!((*pgdat).nr_writeback_throttled)) == 1 {
            write_ulong(
                addr_of_mut!((*pgdat).nr_reclaim_start),
                node_page_state(pgdat, NR_THROTTLED_WRITTEN),
            );
        }
    } else if reason == VMSCAN_THROTTLE_CONGESTED || reason == VMSCAN_THROTTLE_NOPROGRESS {
        if skip_throttle_noprogress(pgdat) {
            cond_resched();
            return;
        }
        timeout = 1;
    } else if reason == VMSCAN_THROTTLE_ISOLATED {
        timeout = (HZ / 50) as Long;
    } else {
        warn_throttle_reason();
        timeout = HZ as Long;
    }
    let mut wait: wait_queue_entry = zeroed();
    init_wait(&mut wait);
    let wqh = (*pgdat).reclaim_wait.as_mut_ptr().add(reason as usize);
    prepare_to_wait(wqh, &mut wait, TASK_UNINTERRUPTIBLE as i32);
    let ret = schedule_timeout(timeout);
    finish_wait(wqh, &mut wait);
    if reason == VMSCAN_THROTTLE_WRITEBACK {
        atomic_dec(addr_of_mut!((*pgdat).nr_writeback_throttled));
    }
    trace_mm_vmscan_throttled(
        (*pgdat).node_id,
        jiffies_to_usecs(timeout as ULong) as i32,
        jiffies_to_usecs(timeout.wrapping_sub(ret) as ULong) as i32,
        reason as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __acct_reclaim_writeback(
    pgdat: *mut pglist_data,
    f: *mut folio,
    nr_throttled: i32,
) {
    node_stat_add_folio(f, NR_THROTTLED_WRITTEN);
    let nr_written = node_page_state(pgdat, NR_THROTTLED_WRITTEN)
        .wrapping_sub(read_ulong(addr_of!((*pgdat).nr_reclaim_start)));
    if nr_written > (SWAP_CLUSTER_MAX as ULong).wrapping_mul(nr_throttled as ULong) {
        wake_up(
            (*pgdat)
                .reclaim_wait
                .as_mut_ptr()
                .add(VMSCAN_THROTTLE_WRITEBACK as usize),
        );
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pageout {
    Keep,
    Activate,
    Success,
    Clean,
}
unsafe fn pageout(
    ctx: *mut swap_io_ctx,
    mapping: *mut address_space,
    f: *mut folio,
    list: *mut list_head,
) -> Pageout {
    if folio_ref_count(f) as ULong != folio_nr_pages(f).wrapping_add(1) || mapping.is_null() {
        return Pageout::Keep;
    }
    if !shmem_mapping(mapping) && !folio_test_anon(f) {
        return Pageout::Activate;
    }
    if !folio_clear_dirty_for_io(f) {
        return Pageout::Clean;
    }
    folio_set_reclaim(f);
    let res = if shmem_mapping(mapping) {
        shmem_writeout(ctx, f, list)
    } else {
        swap_writeout(ctx, f)
    };
    if res < 0 {
        handle_write_error(mapping, f, res);
    }
    if res == AOP_WRITEPAGE_ACTIVATE as i32 {
        folio_clear_reclaim(f);
        return Pageout::Activate;
    }
    if !folio_test_writeback(f) {
        folio_clear_reclaim(f);
    }
    trace_mm_vmscan_write_folio(f);
    lruvec_stat_mod_folio(f, NR_VMSCAN_WRITE, folio_nr_pages(f) as Long);
    Pageout::Success
}
unsafe fn __remove_mapping(
    mapping: *mut address_space,
    f: *mut folio,
    reclaimed: bool,
    target: *mut mem_cgroup,
) -> i32 {
    bug_remove_unlocked(!folio_test_locked(f));
    bug_remove_mapping(mapping != folio_mapping(f));
    let mut shadow = null_mut();
    let mut ci = null_mut();
    if folio_test_swapcache(f) {
        ci = swap_cluster_get_and_lock_irq(f);
    } else {
        spin_lock(addr_of_mut!((*(*mapping).host).i_lock));
        xa_lock_irq(addr_of_mut!((*mapping).i_pages));
    }
    let refcount = folio_nr_pages(f).wrapping_add(1) as i32;
    let mut can_free = folio_ref_freeze(f, refcount);
    if can_free && folio_test_dirty(f) {
        folio_ref_unfreeze(f, refcount);
        can_free = false;
    }
    if !can_free {
        if folio_test_swapcache(f) {
            swap_cluster_unlock_irq(ci);
        } else {
            xa_unlock_irq(addr_of_mut!((*mapping).i_pages));
            spin_unlock(addr_of_mut!((*(*mapping).host).i_lock));
        }
        return 0;
    }
    if folio_test_swapcache(f) {
        let swap = folio_swap(f);
        if reclaimed && !mapping_exiting(mapping) {
            shadow = workingset_eviction(f, target);
        }
        __memcg1_swapout(f, ci);
        __swap_cache_del_folio(ci, f, swap, shadow);
        swap_cluster_unlock_irq(ci);
    } else {
        let free_folio = (*(*mapping).a_ops).free_folio;
        if reclaimed && folio_is_file_lru(f) && !mapping_exiting(mapping) && !dax_mapping(mapping) {
            shadow = workingset_eviction(f, target);
        }
        __filemap_remove_folio(f, shadow);
        xa_unlock_irq(addr_of_mut!((*mapping).i_pages));
        if mapping_shrinkable(mapping) {
            inode_lru_list_add((*mapping).host);
        }
        spin_unlock(addr_of_mut!((*(*mapping).host).i_lock));
        if let Some(free) = free_folio {
            free(f);
        }
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn remove_mapping(mapping: *mut address_space, f: *mut folio) -> Long {
    if __remove_mapping(mapping, f, false, null_mut()) != 0 {
        folio_ref_unfreeze(f, 1);
        return folio_nr_pages(f) as Long;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn folio_putback_lru(f: *mut folio) {
    folio_add_lru(f);
    folio_put(f);
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum FolioReferences {
    Reclaim,
    Keep,
    Activate,
}
#[cfg(CONFIG_LRU_GEN)]
unsafe fn lru_gen_set_refs(f: *mut folio, flags: *const vma_flags_t) -> bool {
    if !folio_test_referenced(f) && !folio_test_workingset(f) {
        if is_exec_file_folio(f, flags) {
            set_mask_bits(
                folio_flags_ptr(f),
                LRU_REFS_FLAGS as ULong,
                1 << PG_workingset,
            );
            return true;
        }
        set_mask_bits(
            folio_flags_ptr(f),
            LRU_REFS_MASK as ULong,
            1 << PG_referenced,
        );
        return false;
    }
    if folio_lru_refs(f) > 1 {
        set_mask_bits(
            folio_flags_ptr(f),
            LRU_REFS_FLAGS as ULong,
            1 << PG_workingset,
        );
    } else {
        folio_mark_accessed(f);
    }
    true
}
#[cfg(not(CONFIG_LRU_GEN))]
unsafe fn lru_gen_set_refs(_f: *mut folio, _flags: *const vma_flags_t) -> bool {
    false
}
unsafe fn folio_check_references(f: *mut folio, sc: *mut scan_control) -> FolioReferences {
    let mut flags: vma_flags_t = zeroed();
    let ptes = folio_referenced(f, 1, (*sc).target_mem_cgroup, &mut flags);
    if vma_flags_test(&flags, VMA_LOCKED_BIT) {
        return FolioReferences::Activate;
    }
    if ptes == -1 {
        return FolioReferences::Keep;
    }
    if lru_gen_enabled() && !lru_gen_switching() {
        if ptes == 0 {
            return FolioReferences::Reclaim;
        }
        return if lru_gen_set_refs(f, &flags) {
            FolioReferences::Activate
        } else {
            FolioReferences::Keep
        };
    }
    let referenced = folio_test_clear_referenced(f);
    if ptes != 0 {
        folio_set_referenced(f);
        if referenced || ptes > 1 || is_exec_file_folio(f, &flags) {
            return FolioReferences::Activate;
        }
        return FolioReferences::Keep;
    }
    FolioReferences::Reclaim
}
unsafe fn folio_check_dirty_writeback(f: *mut folio, dirty: *mut bool, writeback: *mut bool) {
    if !folio_is_file_lru(f) || folio_test_lazyfree(f) {
        *dirty = false;
        *writeback = false;
        return;
    }
    *dirty = folio_test_dirty(f);
    *writeback = folio_test_writeback(f);
    if !folio_test_private(f) {
        return;
    }
    let mapping = folio_mapping(f);
    if !mapping.is_null() {
        if let Some(check) = (*(*mapping).a_ops).is_dirty_writeback {
            check(f, dirty, writeback);
        }
    }
}
unsafe extern "C" fn alloc_demote_folio(src: *mut folio, private: ULong) -> *mut folio {
    let mtc = private as *mut migration_target_control;
    let mut target = *mtc;
    target.nmask = null_mut();
    target.gfp_mask |= __GFP_THISNODE;
    let dst = alloc_migration_target(src, addr_of_mut!(target) as ULong);
    if !dst.is_null() {
        dst
    } else {
        alloc_migration_target(src, mtc as ULong)
    }
}
unsafe fn demote_folio_list(
    list: *mut list_head,
    pgdat: *mut pglist_data,
    memcg: *mut mem_cgroup,
) -> u32 {
    if list_empty(list) {
        return 0;
    }
    let mut allowed: nodemask_t = zeroed();
    node_get_allowed_targets(pgdat, &mut allowed);
    mem_cgroup_node_filter_allowed(memcg, &mut allowed);
    if nodes_empty(&allowed) {
        return 0;
    }
    let nid = next_demotion_node((*pgdat).node_id, &mut allowed);
    if nid == NUMA_NO_NODE {
        return 0;
    }
    let mut mtc: migration_target_control = zeroed();
    mtc.gfp_mask = (GFP_HIGHUSER_MOVABLE & !__GFP_RECLAIM) | __GFP_NOMEMALLOC | GFP_NOWAIT;
    mtc.nmask = &mut allowed;
    mtc.reason = MR_DEMOTION as _;
    mtc.nid = nid;
    let mut succeeded = 0;
    migrate_pages(
        list,
        Some(alloc_demote_folio),
        None,
        addr_of_mut!(mtc) as ULong,
        MIGRATE_ASYNC,
        MR_DEMOTION as _,
        &mut succeeded,
    );
    succeeded
}
unsafe fn may_enter_fs(f: *mut folio, gfp: gfp_t) -> bool {
    if gfp & __GFP_FS != 0 {
        return true;
    }
    folio_test_swapcache(f)
        && gfp & __GFP_IO != 0
        && (*(*__swap_entry_to_info(folio_swap(f))).ops).flags & SWAP_OPS_F_REQUIRE_NOFS == 0
}
// The original goto cleanup ladder is represented explicitly. Each exit retains
// the native lock/refcount ownership; in particular Free already has no lock.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FolioExit {
    Done,
    Free,
    ActivateSplit,
    Activate,
    KeepLocked,
    Keep,
}
unsafe fn shrink_folio_list(
    list: *mut list_head,
    pgdat: *mut pglist_data,
    sc: *mut scan_control,
    stat: *mut reclaim_stat,
    ignore_references: bool,
    memcg: *mut mem_cgroup,
) -> u32 {
    let mut free: folio_batch = zeroed();
    folio_batch_init(&mut free);
    let mut ret: list_head = zeroed();
    init_list_head(&mut ret);
    let mut demote: list_head = zeroed();
    init_list_head(&mut demote);
    let mut reclaimed: u32 = 0;
    let mut ctx: swap_io_ctx = zeroed();
    core::ptr::write_bytes(stat, 0, 1);
    cond_resched();
    let mut do_demote = can_demote((*pgdat).node_id, sc, memcg);
    loop {
        while !list_empty(list) {
            cond_resched();
            let f = lru_to_folio(list);
            list_del(folio_lru_ptr(f));
            let mut nr_pages: u32 = 0;
            let mut exit = 'process: {
                if !folio_trylock(f) {
                    break 'process FolioExit::Keep;
                }
                if folio_contain_hwpoisoned_page(f) {
                    if folio_test_large(f) {
                        break 'process FolioExit::KeepLocked;
                    }
                    unmap_poisoned_folio(f, folio_pfn(f), false);
                    folio_unlock(f);
                    folio_put(f);
                    break 'process FolioExit::Done;
                }
                #[cfg(CONFIG_DEBUG_VM)]
                bug_shrink_active(folio_test_active(f), f);
                nr_pages = folio_nr_pages(f) as u32;
                (*sc).nr_scanned = (*sc).nr_scanned.wrapping_add(nr_pages as ULong);
                if !folio_evictable(f) {
                    break 'process FolioExit::Activate;
                }
                if (*sc).may_unmap() == 0 && folio_mapped(f) {
                    break 'process FolioExit::KeepLocked;
                }
                let (mut dirty, mut writeback) = (false, false);
                folio_check_dirty_writeback(f, &mut dirty, &mut writeback);
                if dirty || writeback {
                    (*stat).nr_dirty = (*stat).nr_dirty.wrapping_add(nr_pages);
                }
                if dirty && !writeback {
                    (*stat).nr_unqueued_dirty = (*stat).nr_unqueued_dirty.wrapping_add(nr_pages);
                }
                if writeback && folio_test_reclaim(f) {
                    (*stat).nr_congested = (*stat).nr_congested.wrapping_add(nr_pages);
                }
                if folio_test_writeback(f) {
                    let mapping = folio_mapping(f);
                    if current_is_kswapd()
                        && folio_test_reclaim(f)
                        && test_bit(PGDAT_WRITEBACK as ULong, addr_of!((*pgdat).flags))
                    {
                        (*stat).nr_immediate = (*stat).nr_immediate.wrapping_add(nr_pages);
                        break 'process FolioExit::Activate;
                    } else if writeback_throttling_sane(sc)
                        || !folio_test_reclaim(f)
                        || !may_enter_fs(f, (*sc).gfp_mask)
                        || (!mapping.is_null()
                            && mapping_writeback_may_deadlock_on_reclaim(mapping))
                    {
                        folio_set_reclaim(f);
                        (*stat).nr_writeback = (*stat).nr_writeback.wrapping_add(nr_pages);
                        break 'process FolioExit::Activate;
                    } else {
                        folio_unlock(f);
                        folio_wait_writeback(f);
                        list_add_tail(folio_lru_ptr(f), list);
                        break 'process FolioExit::Done;
                    }
                }
                let references = if ignore_references {
                    FolioReferences::Reclaim
                } else {
                    folio_check_references(f, sc)
                };
                match references {
                    FolioReferences::Activate => break 'process FolioExit::Activate,
                    FolioReferences::Keep => {
                        (*stat).nr_ref_keep = (*stat).nr_ref_keep.wrapping_add(nr_pages);
                        break 'process FolioExit::KeepLocked;
                    }
                    FolioReferences::Reclaim => {}
                }
                if do_demote && (thp_migration_supported() || !folio_test_large(f)) {
                    list_add(folio_lru_ptr(f), &mut demote);
                    folio_unlock(f);
                    break 'process FolioExit::Done;
                }
                if folio_test_anon(f) && folio_test_swapbacked(f) && !folio_test_swapcache(f) {
                    if (*sc).gfp_mask & __GFP_IO == 0 || folio_maybe_dma_pinned(f) {
                        break 'process FolioExit::KeepLocked;
                    }
                    if folio_test_large(f) {
                        if folio_expected_ref_count(f) != folio_ref_count(f).wrapping_sub(1) {
                            break 'process FolioExit::Activate;
                        }
                        if folio_deferred_partially_mapped(f) && split_folio_to_list(f, list) != 0 {
                            break 'process FolioExit::Activate;
                        }
                    }
                    if folio_alloc_swap(f) != 0 {
                        let order = folio_order(f);
                        if !folio_test_large(f) {
                            break 'process FolioExit::ActivateSplit;
                        }
                        if split_folio_to_list(f, list) != 0 {
                            break 'process FolioExit::Activate;
                        }
                        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
                        if nr_pages >= HPAGE_PMD_NR as u32 {
                            count_memcg_folio_events(f, THP_SWPOUT_FALLBACK, 1);
                            count_vm_event(THP_SWPOUT_FALLBACK);
                        }
                        count_mthp_stat(order, MTHP_STAT_SWPOUT_FALLBACK);
                        if folio_alloc_swap(f) != 0 {
                            break 'process FolioExit::ActivateSplit;
                        }
                    }
                    folio_mark_dirty(f);
                }
                if nr_pages > 1 && !folio_test_large(f) {
                    (*sc).nr_scanned = (*sc).nr_scanned.wrapping_sub((nr_pages - 1) as ULong);
                    nr_pages = 1;
                }
                if folio_mapped(f) {
                    let mut flags = TTU_BATCH_FLUSH;
                    let was_swapbacked = folio_test_swapbacked(f);
                    if folio_test_pmd_mappable(f) {
                        flags |= TTU_SPLIT_HUGE_PMD;
                    }
                    if folio_test_large(f) {
                        flags |= TTU_SYNC;
                    }
                    try_to_unmap(f, flags);
                    if folio_mapped(f) {
                        (*stat).nr_unmap_fail = (*stat).nr_unmap_fail.wrapping_add(nr_pages);
                        if !was_swapbacked && folio_test_swapbacked(f) {
                            (*stat).nr_lazyfree_fail =
                                (*stat).nr_lazyfree_fail.wrapping_add(nr_pages);
                        }
                        break 'process FolioExit::Activate;
                    }
                }
                if folio_maybe_dma_pinned(f) {
                    break 'process FolioExit::Activate;
                }
                let mut mapping = folio_mapping(f);
                if folio_test_dirty(f) {
                    if folio_is_file_lru(f) {
                        node_stat_mod_folio(f, NR_VMSCAN_IMMEDIATE, nr_pages as Long);
                        if !folio_test_reclaim(f) {
                            folio_set_reclaim(f);
                        }
                        break 'process FolioExit::Activate;
                    }
                    if !may_enter_fs(f, (*sc).gfp_mask) || (*sc).may_writepage() == 0 {
                        break 'process FolioExit::KeepLocked;
                    }
                    try_to_unmap_flush_dirty();
                    match pageout(&mut ctx, mapping, f, list) {
                        Pageout::Keep => break 'process FolioExit::KeepLocked,
                        Pageout::Activate => {
                            if nr_pages > 1 && !folio_test_large(f) {
                                (*sc).nr_scanned =
                                    (*sc).nr_scanned.wrapping_sub((nr_pages - 1) as ULong);
                                nr_pages = 1;
                            }
                            break 'process FolioExit::Activate;
                        }
                        Pageout::Success => {
                            if nr_pages > 1 && !folio_test_large(f) {
                                (*sc).nr_scanned =
                                    (*sc).nr_scanned.wrapping_sub((nr_pages - 1) as ULong);
                                nr_pages = 1;
                            }
                            if folio_test_writeback(f) || folio_test_dirty(f) || !folio_trylock(f) {
                                break 'process FolioExit::Keep;
                            }
                            if folio_test_dirty(f) || folio_test_writeback(f) {
                                break 'process FolioExit::KeepLocked;
                            }
                            mapping = folio_mapping(f);
                        }
                        Pageout::Clean => {}
                    }
                }
                if folio_needs_release(f) {
                    if !filemap_release_folio(f, (*sc).gfp_mask) {
                        break 'process FolioExit::Activate;
                    }
                    if mapping.is_null() && folio_ref_count(f) == 1 {
                        folio_unlock(f);
                        if folio_put_testzero(f) {
                            break 'process FolioExit::Free;
                        }
                        reclaimed = reclaimed.wrapping_add(nr_pages);
                        break 'process FolioExit::Done;
                    }
                }
                if folio_test_lazyfree(f) {
                    if !folio_ref_freeze(f, 1) {
                        break 'process FolioExit::KeepLocked;
                    }
                    count_vm_events(PGLAZYFREED, nr_pages as ULong);
                    count_memcg_folio_events(f, PGLAZYFREED, nr_pages as ULong);
                } else if mapping.is_null()
                    || __remove_mapping(mapping, f, true, (*sc).target_mem_cgroup) == 0
                {
                    break 'process FolioExit::KeepLocked;
                }
                folio_unlock(f);
                FolioExit::Free
            };
            if exit == FolioExit::Done {
                continue;
            }
            if exit == FolioExit::Free {
                reclaimed = reclaimed.wrapping_add(nr_pages);
                folio_unqueue_deferred_split(f);
                if folio_batch_add(&mut free, f) == 0 {
                    mem_cgroup_uncharge_folios(&mut free);
                    try_to_unmap_flush();
                    free_unref_folios(&mut free);
                }
                continue;
            }
            if exit == FolioExit::ActivateSplit {
                if nr_pages > 1 {
                    (*sc).nr_scanned = (*sc).nr_scanned.wrapping_sub((nr_pages - 1) as ULong);
                    nr_pages = 1;
                }
                exit = FolioExit::Activate;
            }
            if exit == FolioExit::Activate {
                if folio_test_swapcache(f) && (mem_cgroup_swap_full(f) || folio_test_mlocked(f)) {
                    folio_free_swap(f);
                }
                #[cfg(CONFIG_DEBUG_VM)]
                bug_activate_active(folio_test_active(f), f);
                if !folio_test_mlocked(f) {
                    let typ = folio_is_file_lru(f) as usize;
                    folio_set_active(f);
                    (*stat).nr_activate[typ] = (*stat).nr_activate[typ].wrapping_add(nr_pages);
                    count_memcg_folio_events(f, PGACTIVATE, nr_pages as ULong);
                }
                exit = FolioExit::KeepLocked;
            }
            if exit == FolioExit::KeepLocked {
                folio_unlock(f);
            }
            list_add(folio_lru_ptr(f), &mut ret);
            #[cfg(CONFIG_DEBUG_VM)]
            bug_keep_lru(folio_test_lru(f) || folio_test_unevictable(f), f);
        }
        let demoted = demote_folio_list(&mut demote, pgdat, memcg);
        reclaimed = reclaimed.wrapping_add(demoted);
        (*stat).nr_demoted = (*stat).nr_demoted.wrapping_add(demoted);
        if !list_empty(&demote) {
            list_splice_init(&mut demote, list);
            if (*sc).proactive() == 0 {
                do_demote = false;
                continue;
            }
        }
        break;
    }
    let activated = (*stat).nr_activate[0].wrapping_add((*stat).nr_activate[1]);
    mem_cgroup_uncharge_folios(&mut free);
    try_to_unmap_flush();
    free_unref_folios(&mut free);
    list_splice(&mut ret, list);
    count_vm_events(PGACTIVATE, activated as ULong);
    swap_write_submit(&mut ctx);
    reclaimed
}
#[no_mangle]
pub unsafe extern "C" fn reclaim_clean_pages_from_list(z: *mut zone, list: *mut list_head) -> u32 {
    let mut sc: scan_control = zeroed();
    sc.gfp_mask = GFP_KERNEL;
    sc.set_may_unmap(1);
    let mut stat: reclaim_stat = zeroed();
    let mut clean: list_head = zeroed();
    init_list_head(&mut clean);
    let mut entry = (*list).next;
    while entry != list {
        let f = folio_from_lru(entry);
        entry = (*entry).next;
        if page_has_movable_ops(folio_page(f)) {
            continue;
        }
        if !folio_test_hugetlb(f)
            && folio_is_file_lru(f)
            && !folio_test_dirty(f)
            && !folio_test_unevictable(f)
        {
            folio_clear_active(f);
            list_move(folio_lru_ptr(f), &mut clean);
        }
    }
    let saved = memalloc_noreclaim_save();
    let reclaimed = shrink_folio_list(
        &mut clean,
        (*z).zone_pgdat,
        &mut sc,
        &mut stat,
        true,
        null_mut(),
    );
    memalloc_noreclaim_restore(saved);
    list_splice(&mut clean, list);
    mod_node_page_state((*z).zone_pgdat, NR_ISOLATED_FILE, -(reclaimed as Long));
    mod_node_page_state(
        (*z).zone_pgdat,
        NR_ISOLATED_ANON,
        stat.nr_lazyfree_fail as Long,
    );
    mod_node_page_state(
        (*z).zone_pgdat,
        NR_ISOLATED_FILE,
        -(stat.nr_lazyfree_fail as Long),
    );
    reclaimed
}
unsafe fn update_lru_sizes(lv: *mut lruvec, lru: lru_list, taken: *const ULong) {
    for zid in 0..MAX_NR_ZONES as usize {
        if *taken.add(zid) != 0 {
            update_lru_size(
                lv,
                lru,
                zid as i32,
                (*taken.add(zid) as Long).wrapping_neg(),
            );
        }
    }
}
unsafe fn isolate_lru_folios(
    to_scan: ULong,
    lv: *mut lruvec,
    dst: *mut list_head,
    scanned: *mut ULong,
    sc: *mut scan_control,
    lru: lru_list,
) -> ULong {
    let src = (*lv).lists.as_mut_ptr().add(lru as usize);
    let (mut taken, mut skipped, mut total_scan, mut scan, mut max_skipped): (
        ULong,
        ULong,
        ULong,
        ULong,
        ULong,
    ) = (0, 0, 0, 0, 0);
    let mut zone_taken = [0 as ULong; MAX_NR_ZONES as usize];
    let mut zone_skipped = [0 as ULong; MAX_NR_ZONES as usize];
    let mut skip_list: list_head = zeroed();
    init_list_head(&mut skip_list);
    while scan < to_scan && !list_empty(src) {
        let mut move_to = src;
        let f = lru_to_folio(src);
        prefetch_prev_lru_flags(f, src);
        let nr = folio_nr_pages(f) as ULong;
        total_scan = total_scan.wrapping_add(nr);
        let zid = folio_zonenum(f) as usize;
        if max_skipped < SWAP_CLUSTER_MAX_SKIPPED as ULong && zid as i32 > (*sc).reclaim_idx as i32
        {
            zone_skipped[zid] = zone_skipped[zid].wrapping_add(nr);
            move_to = &mut skip_list;
            max_skipped += 1;
        } else {
            scan = scan.wrapping_add(nr);
            if folio_test_lru(f) && ((*sc).may_unmap() != 0 || !folio_mapped(f)) && folio_try_get(f)
            {
                if !folio_test_clear_lru(f) {
                    folio_put(f);
                } else {
                    taken = taken.wrapping_add(nr);
                    zone_taken[zid] = zone_taken[zid].wrapping_add(nr);
                    move_to = dst;
                }
            }
        }
        list_move(folio_lru_ptr(f), move_to);
    }
    if !list_empty(&skip_list) {
        list_splice(&mut skip_list, src);
        for zid in 0..MAX_NR_ZONES as usize {
            if zone_skipped[zid] == 0 {
                continue;
            }
            __count_zid_vm_events(PGSCAN_SKIP, zid as i32, zone_skipped[zid]);
            skipped = skipped.wrapping_add(zone_skipped[zid]);
        }
    }
    *scanned = total_scan;
    trace_mm_vmscan_lru_isolate(
        (*sc).reclaim_idx as i32,
        (*sc).order as i32,
        to_scan,
        total_scan,
        skipped,
        taken,
        lru as i32,
    );
    update_lru_sizes(lv, lru, zone_taken.as_ptr());
    taken
}
#[no_mangle]
pub unsafe extern "C" fn folio_isolate_lru(f: *mut folio) -> bool {
    #[cfg(CONFIG_DEBUG_VM)]
    bug_isolate_ref(folio_ref_count(f) == 0, f);
    if !folio_test_clear_lru(f) {
        return false;
    }
    folio_get(f);
    let lv = folio_lruvec_lock_irq(f);
    lruvec_del_folio(lv, f);
    lruvec_unlock_irq(lv);
    true
}
unsafe fn too_many_isolated(pgdat: *mut pglist_data, file: bool, sc: *mut scan_control) -> bool {
    if current_is_kswapd() || !writeback_throttling_sane(sc) {
        return false;
    }
    let mut inactive = node_page_state(
        pgdat,
        if file {
            NR_INACTIVE_FILE
        } else {
            NR_INACTIVE_ANON
        },
    );
    let isolated = node_page_state(
        pgdat,
        if file {
            NR_ISOLATED_FILE
        } else {
            NR_ISOLATED_ANON
        },
    );
    if gfp_has_io_fs((*sc).gfp_mask) {
        inactive >>= 3;
    }
    let too_many = isolated > inactive;
    if !too_many {
        wake_throttle_isolated(pgdat);
    }
    too_many
}
unsafe fn move_folios_to_lru(list: *mut list_head) -> u32 {
    let mut moved: i32 = 0;
    let mut lv = null_mut();
    let mut free: folio_batch = zeroed();
    folio_batch_init(&mut free);
    while !list_empty(list) {
        let f = lru_to_folio(list);
        lv = folio_lruvec_relock_irq(f, lv);
        #[cfg(CONFIG_DEBUG_VM)]
        bug_move_lru(folio_test_lru(f), f);
        list_del(folio_lru_ptr(f));
        if !folio_evictable(f) {
            lruvec_unlock_irq(lv);
            folio_putback_lru(f);
            lv = null_mut();
            continue;
        }
        folio_set_lru(f);
        if folio_put_testzero(f) {
            __folio_clear_lru_flags(f);
            folio_unqueue_deferred_split(f);
            if folio_batch_add(&mut free, f) == 0 {
                lruvec_unlock_irq(lv);
                mem_cgroup_uncharge_folios(&mut free);
                free_unref_folios(&mut free);
                lv = null_mut();
            }
            continue;
        }
        lruvec_add_folio(lv, f);
        let nr = folio_nr_pages(f) as i32;
        moved = moved.wrapping_add(nr);
        if folio_test_active(f) {
            workingset_age_nonresident(lv, nr as ULong);
        }
    }
    if !lv.is_null() {
        lruvec_unlock_irq(lv);
    }
    if free.nr != 0 {
        mem_cgroup_uncharge_folios(&mut free);
        free_unref_folios(&mut free);
    }
    moved as u32
}
unsafe fn current_may_throttle() -> bool {
    (*current()).flags & PF_LOCAL_THROTTLE == 0
}
unsafe fn handle_reclaim_writeback(
    taken: ULong,
    pgdat: *mut pglist_data,
    sc: *mut scan_control,
    stat: *mut reclaim_stat,
) {
    if (*stat).nr_unqueued_dirty as ULong == taken {
        wakeup_flusher_threads(WB_REASON_VMSCAN);
        if !writeback_throttling_sane(sc) {
            reclaim_throttle(pgdat, VMSCAN_THROTTLE_WRITEBACK);
        }
    }
    (*sc).nr.dirty = (*sc).nr.dirty.wrapping_add((*stat).nr_dirty);
    (*sc).nr.congested = (*sc).nr.congested.wrapping_add((*stat).nr_congested);
    (*sc).nr.writeback = (*sc).nr.writeback.wrapping_add((*stat).nr_writeback);
    (*sc).nr.immediate = (*sc).nr.immediate.wrapping_add((*stat).nr_immediate);
    (*sc).nr.taken = (*sc).nr.taken.wrapping_add(taken as u32);
}
unsafe fn shrink_inactive_list(
    to_scan: ULong,
    lv: *mut lruvec,
    sc: *mut scan_control,
    lru: lru_list,
) -> ULong {
    let mut list: list_head = zeroed();
    init_list_head(&mut list);
    let mut scanned = 0;
    let mut stat: reclaim_stat = zeroed();
    let file = is_file_lru(lru);
    let pgdat = lruvec_pgdat(lv);
    let mut stalled = false;
    while too_many_isolated(pgdat, file, sc) {
        if stalled {
            return 0;
        }
        stalled = true;
        reclaim_throttle(pgdat, VMSCAN_THROTTLE_ISOLATED);
        if fatal_signal_pending(current()) {
            return SWAP_CLUSTER_MAX as ULong;
        }
    }
    lru_add_drain();
    lruvec_lock_irq(lv);
    let taken = isolate_lru_folios(to_scan, lv, &mut list, &mut scanned, sc, lru);
    __mod_node_page_state(pgdat, NR_ISOLATED_ANON + file as u32, taken as Long);
    mod_lruvec_state(lv, PGSCAN_KSWAPD + reclaimer_offset(sc), scanned as Long);
    mod_lruvec_state(lv, PGSCAN_ANON + file as u32, scanned as Long);
    lruvec_unlock_irq(lv);
    if taken == 0 {
        return 0;
    }
    let reclaimed = shrink_folio_list(&mut list, pgdat, sc, &mut stat, false, lruvec_memcg(lv));
    move_folios_to_lru(&mut list);
    mod_lruvec_state(
        lv,
        PGDEMOTE_KSWAPD + reclaimer_offset(sc),
        stat.nr_demoted as Long,
    );
    mod_node_page_state(
        pgdat,
        NR_ISOLATED_ANON + file as u32,
        (taken as Long).wrapping_neg(),
    );
    mod_lruvec_state(lv, PGSTEAL_KSWAPD + reclaimer_offset(sc), reclaimed as Long);
    mod_lruvec_state(lv, PGSTEAL_ANON + file as u32, reclaimed as Long);
    if scanned > reclaimed as ULong {
        mod_lruvec_state(
            lv,
            PGROTATE_ANON + file as u32,
            scanned.wrapping_sub(reclaimed as ULong) as Long,
        );
    }
    handle_reclaim_writeback(taken, pgdat, sc, &mut stat);
    trace_mm_vmscan_lru_shrink_inactive(
        (*pgdat).node_id,
        scanned,
        reclaimed as ULong,
        &mut stat,
        (*sc).priority as i32,
        file as i32,
    );
    reclaimed as ULong
}
unsafe fn shrink_active_list(
    to_scan: ULong,
    lv: *mut lruvec,
    sc: *mut scan_control,
    lru: lru_list,
) {
    let mut scanned = 0;
    let mut flags: vma_flags_t = zeroed();
    let mut hold: list_head = zeroed();
    init_list_head(&mut hold);
    let mut active: list_head = zeroed();
    init_list_head(&mut active);
    let mut inactive: list_head = zeroed();
    init_list_head(&mut inactive);
    let mut rotated: u32 = 0;
    let file = is_file_lru(lru);
    let pgdat = lruvec_pgdat(lv);
    lru_add_drain();
    lruvec_lock_irq(lv);
    let taken = isolate_lru_folios(to_scan, lv, &mut hold, &mut scanned, sc, lru);
    __mod_node_page_state(pgdat, NR_ISOLATED_ANON + file as u32, taken as Long);
    mod_lruvec_state(lv, PGREFILL, scanned as Long);
    lruvec_unlock_irq(lv);
    while !list_empty(&hold) {
        cond_resched();
        let f = lru_to_folio(&mut hold);
        list_del(folio_lru_ptr(f));
        if !folio_evictable(f) {
            folio_putback_lru(f);
            continue;
        }
        if native_buffer_heads_over_limit() != 0 && folio_needs_release(f) && folio_trylock(f) {
            filemap_release_folio(f, 0);
            folio_unlock(f);
        }
        if folio_referenced(f, 0, (*sc).target_mem_cgroup, &mut flags) != 0
            && is_exec_file_folio(f, &flags)
        {
            rotated = rotated.wrapping_add(folio_nr_pages(f) as u32);
            list_add(folio_lru_ptr(f), &mut active);
            continue;
        }
        folio_clear_active(f);
        folio_set_workingset(f);
        list_add(folio_lru_ptr(f), &mut inactive);
    }
    let activated = move_folios_to_lru(&mut active);
    let deactivated = move_folios_to_lru(&mut inactive);
    count_vm_events(PGDEACTIVATE, deactivated as ULong);
    count_memcg_events(lruvec_memcg(lv), PGDEACTIVATE, deactivated as ULong);
    mod_node_page_state(
        pgdat,
        NR_ISOLATED_ANON + file as u32,
        (taken as Long).wrapping_neg(),
    );
    if rotated != 0 {
        mod_lruvec_state(lv, PGROTATE_ANON + file as u32, rotated as Long);
    }
    trace_mm_vmscan_lru_shrink_active(
        (*pgdat).node_id,
        taken,
        activated as ULong,
        deactivated as ULong,
        rotated as ULong,
        (*sc).priority as i32,
        file as i32,
    );
}
unsafe fn reclaim_folio_list(list: *mut list_head, pgdat: *mut pglist_data) -> u32 {
    let mut stat: reclaim_stat = zeroed();
    let mut sc: scan_control = zeroed();
    sc.gfp_mask = GFP_KERNEL;
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.set_may_swap(1);
    sc.set_no_demotion(1);
    let reclaimed = shrink_folio_list(list, pgdat, &mut sc, &mut stat, true, null_mut());
    while !list_empty(list) {
        let f = lru_to_folio(list);
        list_del(folio_lru_ptr(f));
        folio_putback_lru(f);
    }
    trace_mm_vmscan_reclaim_pages(
        (*pgdat).node_id,
        sc.nr_scanned,
        reclaimed as ULong,
        &mut stat,
    );
    reclaimed
}
#[no_mangle]
pub unsafe extern "C" fn reclaim_pages(list: *mut list_head) -> ULong {
    let mut reclaimed: u32 = 0;
    if list_empty(list) {
        return 0;
    }
    let mut node_list: list_head = zeroed();
    init_list_head(&mut node_list);
    let saved = memalloc_noreclaim_save();
    let mut nid = folio_nid(lru_to_folio(list));
    while !list_empty(list) {
        let f = lru_to_folio(list);
        if nid == folio_nid(f) {
            folio_clear_active(f);
            list_move(folio_lru_ptr(f), &mut node_list);
            continue;
        }
        reclaimed = reclaimed.wrapping_add(reclaim_folio_list(&mut node_list, node_data(nid)));
        nid = folio_nid(lru_to_folio(list));
    }
    reclaimed = reclaimed.wrapping_add(reclaim_folio_list(&mut node_list, node_data(nid)));
    memalloc_noreclaim_restore(saved);
    reclaimed as ULong
}
unsafe fn shrink_list(lru: lru_list, nr: ULong, lv: *mut lruvec, sc: *mut scan_control) -> ULong {
    if is_active_lru(lru) {
        if (*sc).may_deactivate() & (1 << is_file_lru(lru) as u32) != 0 {
            shrink_active_list(nr, lv, sc, lru);
        } else {
            (*sc).set_skipped_deactivate(1);
        }
        0
    } else {
        shrink_inactive_list(nr, lv, sc, lru)
    }
}
unsafe fn inactive_is_low(lv: *mut lruvec, inactive_lru: lru_list) -> bool {
    let inactive = lruvec_page_state(lv, NR_LRU_BASE + inactive_lru);
    let active = lruvec_page_state(lv, NR_LRU_BASE + inactive_lru + LRU_ACTIVE);
    let gb = inactive.wrapping_add(active) >> (30 - PAGE_SHIFT);
    let ratio = if gb != 0 {
        int_sqrt(gb.wrapping_mul(10))
    } else {
        1
    };
    inactive.wrapping_mul(ratio) < active
}
unsafe fn prepare_scan_control(pgdat: *mut pglist_data, sc: *mut scan_control) {
    if lru_gen_enabled() && !lru_gen_switching() {
        return;
    }
    let lv = mem_cgroup_lruvec((*sc).target_mem_cgroup, pgdat);
    mem_cgroup_flush_stats_ratelimited((*sc).target_mem_cgroup);
    spin_lock(addr_of_mut!((*lv).cost_lock));
    for f in 0..=1usize {
        let cost = addr_of_mut!((*lv).cost[f]);
        let rotated = lruvec_page_state_monotonic(lv, PGROTATE_ANON + f as u32);
        let mut io = lruvec_page_state_monotonic(lv, WORKINGSET_RESTORE_BASE + f as u32);
        if f == WORKINGSET_ANON as usize {
            io = io.wrapping_add(lruvec_page_state_monotonic(lv, NR_VMSCAN_WRITE));
        }
        let nr_rotated = rotated.wrapping_sub((*cost).last_rotated);
        let nr_io = io.wrapping_sub((*cost).last_io);
        (*cost).count = (*cost)
            .count
            .wrapping_add(nr_io.wrapping_mul(SWAP_CLUSTER_MAX as ULong))
            .wrapping_add(nr_rotated);
        (*cost).last_rotated = rotated;
        (*cost).last_io = io;
    }
    let anon = addr_of_mut!((*lv).cost[WORKINGSET_ANON as usize]);
    let file = addr_of_mut!((*lv).cost[WORKINGSET_FILE as usize]);
    let size = lruvec_page_state(lv, NR_INACTIVE_ANON)
        .wrapping_add(lruvec_page_state(lv, NR_ACTIVE_ANON))
        .wrapping_add(lruvec_page_state(lv, NR_INACTIVE_FILE))
        .wrapping_add(lruvec_page_state(lv, NR_ACTIVE_FILE));
    while (*anon).count.wrapping_add((*file).count) > size / 4 {
        (*anon).count /= 2;
        (*file).count /= 2;
    }
    (*sc).anon_cost = (*anon).count;
    (*sc).file_cost = (*file).count;
    spin_unlock(addr_of_mut!((*lv).cost_lock));
    if (*sc).force_deactivate() == 0 {
        let mut deactivate = (*sc).may_deactivate();
        if lruvec_page_state(lv, WORKINGSET_ACTIVATE_ANON)
            != (*lv).refaults[WORKINGSET_ANON as usize]
            || inactive_is_low(lv, LRU_INACTIVE_ANON)
        {
            deactivate |= DEACTIVATE_ANON;
        } else {
            deactivate &= !DEACTIVATE_ANON;
        }
        if lruvec_page_state(lv, WORKINGSET_ACTIVATE_FILE)
            != (*lv).refaults[WORKINGSET_FILE as usize]
            || inactive_is_low(lv, LRU_INACTIVE_FILE)
        {
            deactivate |= DEACTIVATE_FILE;
        } else {
            deactivate &= !DEACTIVATE_FILE;
        }
        (*sc).set_may_deactivate(deactivate);
    } else {
        (*sc).set_may_deactivate(DEACTIVATE_ANON | DEACTIVATE_FILE);
    }
    let file_size = lruvec_page_state(lv, NR_INACTIVE_FILE);
    (*sc).set_cache_trim_mode(
        ((file_size >> (*sc).priority) != 0
            && (*sc).may_deactivate() & DEACTIVATE_FILE == 0
            && (*sc).no_cache_trim_mode() == 0) as u32,
    );
    if !cgroup_reclaim(sc) {
        let free = sum_zone_node_page_state((*pgdat).node_id, NR_FREE_PAGES);
        let file_size = node_page_state(pgdat, NR_ACTIVE_FILE)
            .wrapping_add(node_page_state(pgdat, NR_INACTIVE_FILE));
        let mut high: ULong = 0;
        for zid in 0..MAX_NR_ZONES as usize {
            let z = (*pgdat).node_zones.as_mut_ptr().add(zid);
            if managed_zone(z) {
                high = high.wrapping_add(high_wmark_pages(z));
            }
        }
        let anon = node_page_state(pgdat, NR_INACTIVE_ANON);
        (*sc).set_file_is_tiny(
            (file_size.wrapping_add(free) <= high
                && (*sc).may_deactivate() & DEACTIVATE_ANON == 0
                && (anon >> (*sc).priority) != 0) as u32,
        );
    }
}
unsafe fn calculate_pressure_balance(
    sc: *mut scan_control,
    swappiness: i32,
    fraction: *mut u64,
    denominator: *mut u64,
) {
    let total = (*sc).anon_cost.wrapping_add((*sc).file_cost);
    let anon = total.wrapping_add((*sc).anon_cost);
    let file = total.wrapping_add((*sc).file_cost);
    let total = anon.wrapping_add(file);
    let ap = (swappiness as ULong).wrapping_mul(total.wrapping_add(1)) / anon.wrapping_add(1);
    let fp = ((MAX_SWAPPINESS as i32).wrapping_sub(swappiness) as ULong)
        .wrapping_mul(total.wrapping_add(1))
        / file.wrapping_add(1);
    *fraction.add(WORKINGSET_ANON as usize) = ap as u64;
    *fraction.add(WORKINGSET_FILE as usize) = fp as u64;
    *denominator = ap.wrapping_add(fp) as u64;
}
unsafe fn apply_proportional_protection(
    memcg: *mut mem_cgroup,
    sc: *mut scan_control,
    mut scan: ULong,
) -> ULong {
    let (mut minimum, mut low, mut usage) = (0, 0, 0);
    mem_cgroup_protection(
        (*sc).target_mem_cgroup,
        memcg,
        &mut minimum,
        &mut low,
        &mut usage,
    );
    if minimum != 0 || low != 0 {
        let protection = if (*sc).memcg_low_reclaim() == 0 && low > minimum {
            (*sc).set_memcg_low_skipped(1);
            low
        } else {
            minimum
        };
        usage = max(usage, protection);
        scan = scan.wrapping_sub(scan.wrapping_mul(protection) / usage.wrapping_add(1));
        scan = max(scan, SWAP_CLUSTER_MAX as ULong);
    }
    scan
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum ScanBalance {
    Equal,
    Fract,
    Anon,
    File,
}
unsafe fn get_scan_count(lv: *mut lruvec, sc: *mut scan_control, nr: *mut ULong) {
    let pgdat = lruvec_pgdat(lv);
    let memcg = lruvec_memcg(lv);
    let swappiness = sc_swappiness(sc, memcg);
    let mut fraction = [0u64; ANON_AND_FILE as usize];
    let mut denominator = 0;
    let balance = if swappiness == SWAPPINESS_ANON_ONLY as i32 {
        warn_anon_only((*sc).proactive() == 0);
        if !can_reclaim_anon_pages(memcg, (*pgdat).node_id, sc) {
            core::ptr::write_bytes(nr, 0, NR_LRU_LISTS as usize);
            return;
        }
        ScanBalance::Anon
    } else if (*sc).may_swap() == 0
        || !can_reclaim_anon_pages(memcg, (*pgdat).node_id, sc)
        || (cgroup_reclaim(sc) && swappiness == 0)
    {
        ScanBalance::File
    } else if (*sc).priority == 0 && swappiness != 0 {
        ScanBalance::Equal
    } else if (*sc).file_is_tiny() != 0 {
        ScanBalance::Anon
    } else if (*sc).cache_trim_mode() != 0 {
        ScanBalance::File
    } else {
        calculate_pressure_balance(sc, swappiness, fraction.as_mut_ptr(), &mut denominator);
        ScanBalance::Fract
    };
    for lru in LRU_INACTIVE_ANON..=LRU_ACTIVE_FILE {
        let file = is_file_lru(lru);
        let size = lruvec_lru_size(lv, lru, (*sc).reclaim_idx as i32);
        let mut scan = apply_proportional_protection(memcg, sc, size) >> (*sc).priority;
        if scan == 0 && !mem_cgroup_online(memcg) {
            scan = min(size, SWAP_CLUSTER_MAX as ULong);
        }
        match balance {
            ScanBalance::Equal => {}
            ScanBalance::Fract => {
                let product = (scan as u64).wrapping_mul(fraction[file as usize]);
                scan = if mem_cgroup_online(memcg) {
                    div64_u64(product, denominator)
                } else {
                    div64_u64_round_up(product, denominator)
                } as ULong;
            }
            ScanBalance::File | ScanBalance::Anon => {
                if (balance == ScanBalance::File) != file {
                    scan = 0;
                }
            }
        }
        *nr.add(lru as usize) = scan;
    }
}
unsafe fn can_age_anon_pages(lv: *mut lruvec, sc: *mut scan_control) -> bool {
    native_total_swap_pages() > 0 || can_demote((*lruvec_pgdat(lv)).node_id, sc, lruvec_memcg(lv))
}
