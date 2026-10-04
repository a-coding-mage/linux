// SPDX-License-Identifier: GPL-2.0-only
// Native-layout Rust owner for page_alloc.c: warn_alloc_show_mem onwards.
// Included in page_alloc.rs; the shared bindings own all target C layouts.

unsafe fn warn_alloc_show_mem(gfp: gfp_t, mask: *const nodemask_t) {
    let mut filter = RUST_PA_SHOW_MEM_FILTER_NODES;
    let t = rust_pa_late_current();
    if gfp & RUST_PA___GFP_NOMEMALLOC == 0 {
        if rust_pa_late_tsk_is_oom_victim(t)
            || (*t).flags & (RUST_PA_PF_MEMALLOC | RUST_PA_PF_EXITING) != 0
        {
            filter &= !RUST_PA_SHOW_MEM_FILTER_NODES;
        }
    }
    if !rust_pa_late_in_task() || gfp & RUST_PA___GFP_DIRECT_RECLAIM == 0 {
        filter &= !RUST_PA_SHOW_MEM_FILTER_NODES;
    }
    __show_mem(filter, mask, rust_pa_late_gfp_zone(gfp) as i32);
    rust_pa_late_mem_cgroup_show_protected_memory(null_mut());
}

// The public variadic entry performs va_list marshalling only. Its gate and
// post-message diagnostics live here, preserving short-circuit evaluation.
#[no_mangle]
pub unsafe extern "C" fn rust_pa_warn_alloc_allowed(gfp: gfp_t) -> bool {
    gfp & RUST_PA___GFP_NOWARN == 0
        && rust_pa_late_nopage_ratelimit()
        && !(gfp & RUST_PA___GFP_DMA != 0 && !rust_pa_late_has_managed_dma())
}
#[no_mangle]
pub unsafe extern "C" fn rust_pa_warn_alloc_finish(gfp: gfp_t, mask: *const nodemask_t) {
    rust_pa_late_cpuset_print_current_mems_allowed();
    rust_pa_late_print_newline();
    rust_pa_late_dump_stack();
    warn_alloc_show_mem(gfp, mask);
}

unsafe fn __alloc_pages_cpuset_fallback(
    gfp: gfp_t,
    order: u32,
    flags: u32,
    ac: *const alloc_context,
) -> *mut page {
    let mut p = pa_late_freelist(gfp, order, flags | RUST_PA_ALLOC_CPUSET, ac);
    if p.is_null() {
        p = pa_late_freelist(gfp, order, flags, ac);
    }
    p
}
unsafe fn __alloc_pages_may_oom(
    gfp: gfp_t,
    order: u32,
    ac: *const alloc_context,
    progress: *mut ULong,
) -> *mut page {
    let mut oc: oom_control = zeroed();
    oc.zonelist = (*ac).zonelist;
    oc.nodemask = (*ac).nodemask;
    oc.gfp_mask = gfp;
    oc.order = order as i32;
    *progress = 0;
    if !rust_pa_late_oom_trylock() {
        *progress = 1;
        schedule_timeout_uninterruptible(1);
        return null_mut();
    }
    let mut p = pa_late_freelist(
        (gfp | RUST_PA___GFP_HARDWALL) & !RUST_PA___GFP_DIRECT_RECLAIM,
        order,
        (*ac).alloc_flags | RUST_PA_ALLOC_WMARK_HIGH | RUST_PA_ALLOC_CPUSET,
        ac,
    );
    if p.is_null()
        && (*rust_pa_late_current()).flags & RUST_PA_PF_DUMPCORE == 0
        && order <= RUST_PA_PAGE_ALLOC_COSTLY_ORDER
        && gfp & (RUST_PA___GFP_RETRY_MAYFAIL | RUST_PA___GFP_THISNODE) == 0
        && (*ac).highest_zoneidx >= ZONE_NORMAL
        && !rust_pa_late_pm_suspended_storage()
    {
        if out_of_memory(&mut oc)
            || rust_pa_late_warn_nofail_oom(gfp & RUST_PA___GFP_NOFAIL != 0, gfp)
        {
            *progress = 1;
            if gfp & RUST_PA___GFP_NOFAIL != 0 {
                p = __alloc_pages_cpuset_fallback(
                    gfp,
                    order,
                    (*ac).alloc_flags | RUST_PA_ALLOC_NO_WATERMARKS,
                    ac,
                );
            }
        }
    }
    rust_pa_late_oom_unlock();
    p
}
const MAX_COMPACT_RETRIES: i32 = 16;

#[cfg(CONFIG_COMPACTION)]
unsafe fn __alloc_pages_direct_compact(
    gfp: gfp_t,
    order: u32,
    flags: u32,
    ac: *const alloc_context,
    priority: compact_priority,
    result: *mut compact_result,
) -> *mut page {
    let mut cap: capture_control = zeroed();
    cap.migratetype = (*ac).migratetype;
    cap.order = order as i32;
    let mut compact_order = order as i32;
    if flags & RUST_PA_ALLOC_NOFRAGMENT != 0 && (*ac).migratetype != MIGRATE_MOVABLE as i32 {
        compact_order = max(order, pb_order()) as i32;
    }
    if compact_order == 0 {
        return null_mut();
    }
    let mut pflags: ULong = 0;
    rust_pa_late_psi_memstall_enter(&mut pflags);
    rust_pa_late_delayacct_compact_start();
    rust_pa_late_fs_reclaim_acquire(gfp);
    let noreclaim = rust_pa_late_memalloc_noreclaim_save();
    rust_pa_late_capture_publish(&mut cap);
    *result = try_to_compact_pages(gfp, compact_order as u32, flags, ac, priority, &mut cap);
    rust_pa_late_capture_clear();
    let mut p = rust_pa_late_capture_page(&cap);
    if !p.is_null() {
        *result = COMPACT_SUCCESS;
    }
    rust_pa_late_memalloc_noreclaim_restore(noreclaim);
    rust_pa_late_fs_reclaim_release(gfp);
    rust_pa_late_psi_memstall_leave(&mut pflags);
    rust_pa_late_delayacct_compact_end();
    if *result == COMPACT_SKIPPED || *result == COMPACT_DEFERRED {
        return null_mut();
    }
    rust_pa_late_count_vm_event(COMPACTSTALL);
    if !p.is_null() {
        prep_new_page(p, order, gfp, flags);
    }
    if p.is_null() {
        p = pa_late_freelist(gfp, order, flags, ac);
    }
    if !p.is_null() {
        let z = rust_pa_late_page_zone(p);
        (*z).compact_blockskip_flush = false;
        compaction_defer_reset(z, compact_order, true);
        rust_pa_late_count_vm_event(COMPACTSUCCESS);
        return p;
    }
    rust_pa_late_count_vm_event(COMPACTFAIL);
    rust_pa_late_cond_resched();
    null_mut()
}
#[cfg(not(CONFIG_COMPACTION))]
unsafe fn __alloc_pages_direct_compact(
    _gfp: gfp_t,
    _order: u32,
    _flags: u32,
    _ac: *const alloc_context,
    _priority: compact_priority,
    result: *mut compact_result,
) -> *mut page {
    *result = COMPACT_SKIPPED;
    null_mut()
}
#[cfg(CONFIG_COMPACTION)]
unsafe fn should_compact_retry(
    gfp: gfp_t,
    ac: *mut alloc_context,
    order: i32,
    flags: i32,
    result: compact_result,
    priority: *mut compact_priority,
    retries: *mut i32,
) -> bool {
    if order == 0 || rust_pa_late_fatal_signal_pending(rust_pa_late_current()) {
        return false;
    }
    let old_retries = *retries;
    let old_priority = *priority;
    let mut max_retries = MAX_COMPACT_RETRIES;
    let ret;
    if result == COMPACT_SKIPPED {
        ret = compaction_zonelist_suitable(ac, order, flags, gfp);
    } else {
        let mut success_retry = false;
        if result == COMPACT_SUCCESS {
            if order > RUST_PA_PAGE_ALLOC_COSTLY_ORDER as i32 {
                max_retries /= 4;
            }
            *retries += 1;
            success_retry = *retries <= max_retries;
        }
        if success_retry {
            ret = true;
        } else {
            let min_priority = if order > RUST_PA_PAGE_ALLOC_COSTLY_ORDER as i32 {
                MIN_COMPACT_COSTLY_PRIORITY
            } else {
                MIN_COMPACT_PRIORITY
            };
            ret = *priority > min_priority;
            if ret {
                *priority -= 1;
                *retries = 0;
            }
        }
    }
    rust_pa_late_trace_compact_retry(order, old_priority, result, old_retries, max_retries, ret);
    ret
}
#[cfg(not(CONFIG_COMPACTION))]
unsafe fn should_compact_retry(
    _gfp: gfp_t,
    ac: *mut alloc_context,
    order: i32,
    flags: i32,
    _result: compact_result,
    _priority: *mut compact_priority,
    _retries: *mut i32,
) -> bool {
    if order == 0 || order > RUST_PA_PAGE_ALLOC_COSTLY_ORDER as i32 {
        return false;
    }
    let mut z =
        rust_pa_late_first_zones_zonelist((*ac).zonelist, (*ac).highest_zoneidx, (*ac).nodemask);
    while !(*z).zone.is_null() {
        let zone = (*z).zone;
        if zone_watermark_ok(
            zone,
            0,
            rust_pa_late_min_wmark_pages(zone),
            (*ac).highest_zoneidx as i32,
            flags as u32,
        ) {
            return true;
        }
        z = rust_pa_late_next_zones_zonelist(z.add(1), (*ac).highest_zoneidx, (*ac).nodemask);
    }
    false
}

#[cfg(CONFIG_LOCKDEP)]
unsafe fn __need_reclaim(gfp: gfp_t) -> bool {
    gfp & RUST_PA___GFP_DIRECT_RECLAIM != 0
        && (*rust_pa_late_current()).flags & RUST_PA_PF_MEMALLOC == 0
        && gfp & RUST_PA___GFP_NOLOCKDEP == 0
}
#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn __fs_reclaim_acquire(ip: ULong) {
    rust_pa_late_lock_acquire_fs(ip);
}
#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn __fs_reclaim_release(ip: ULong) {
    rust_pa_late_lock_release_fs(ip);
}
#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn rust_pa_fs_reclaim_acquire(gfp: gfp_t, ip: ULong) {
    let gfp = rust_pa_late_current_gfp_context(gfp);
    if __need_reclaim(gfp) {
        if gfp & RUST_PA___GFP_FS != 0 {
            __fs_reclaim_acquire(ip);
        }
        #[cfg(CONFIG_MMU_NOTIFIER)]
        {
            rust_pa_late_mmu_lock_map_acquire();
            rust_pa_late_mmu_lock_map_release();
        }
    }
}
#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn rust_pa_fs_reclaim_release(gfp: gfp_t, ip: ULong) {
    let gfp = rust_pa_late_current_gfp_context(gfp);
    if __need_reclaim(gfp) && gfp & RUST_PA___GFP_FS != 0 {
        __fs_reclaim_release(ip);
    }
}
// Native entry thunks capture _RET_IP_ before entering Rust, retaining caller identity.
unsafe fn zonelist_iter_begin() -> u32 {
    #[cfg(CONFIG_MEMORY_HOTREMOVE)]
    {
        return rust_pa_late_read_zonelist_seq();
    }
    #[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
    {
        0
    }
}
unsafe fn check_retry_zonelist(seq: u32) -> u32 {
    #[cfg(CONFIG_MEMORY_HOTREMOVE)]
    {
        return rust_pa_late_retry_zonelist_seq(seq) as u32;
    }
    #[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
    {
        seq
    }
}
unsafe fn __perform_reclaim(gfp: gfp_t, order: u32, ac: *const alloc_context) -> ULong {
    rust_pa_late_cond_resched();
    rust_pa_late_cpuset_memory_pressure_bump();
    rust_pa_late_fs_reclaim_acquire(gfp);
    let saved = rust_pa_late_memalloc_noreclaim_save();
    let progress = try_to_free_pages((*ac).zonelist, order as i32, gfp, (*ac).nodemask);
    rust_pa_late_memalloc_noreclaim_restore(saved);
    rust_pa_late_fs_reclaim_release(gfp);
    rust_pa_late_cond_resched();
    progress
}
unsafe fn __alloc_pages_direct_reclaim(
    gfp: gfp_t,
    order: u32,
    flags: u32,
    ac: *const alloc_context,
    progress: *mut ULong,
) -> *mut page {
    let reclaim_order =
        if flags & RUST_PA_ALLOC_NOFRAGMENT != 0 && (*ac).migratetype != MIGRATE_MOVABLE as i32 {
            max(order, pb_order())
        } else {
            order
        };
    let mut pflags: ULong = 0;
    rust_pa_late_psi_memstall_enter(&mut pflags);
    *progress = __perform_reclaim(gfp, reclaim_order, ac);
    let mut p = null_mut();
    if *progress != 0 {
        p = pa_late_freelist(gfp, order, flags, ac);
        if p.is_null() {
            unreserve_highatomic_pageblock(ac, false);
            drain_all_pages(null_mut());
            p = pa_late_freelist(gfp, order, flags, ac);
        }
    }
    rust_pa_late_psi_memstall_leave(&mut pflags);
    p
}
unsafe fn wake_all_kswapds(order: u32, gfp: gfp_t, ac: *const alloc_context) {
    let order = if defrag_mode != 0 {
        max(order, pb_order())
    } else {
        order
    };
    let mut last = null_mut();
    let mut z =
        rust_pa_late_first_zones_zonelist((*ac).zonelist, (*ac).highest_zoneidx, (*ac).nodemask);
    while !(*z).zone.is_null() {
        let zone = (*z).zone;
        if rust_pa_late_managed_zone(zone) && last != (*zone).zone_pgdat {
            wakeup_kswapd(zone, gfp, order as i32, (*ac).highest_zoneidx);
            last = (*zone).zone_pgdat;
        }
        z = rust_pa_late_next_zones_zonelist(z.add(1), (*ac).highest_zoneidx, (*ac).nodemask);
    }
}
unsafe fn alloc_flags_nonblocking(gfp: gfp_t, order: u32) -> u32 {
    if gfp & (RUST_PA___GFP_DIRECT_RECLAIM | RUST_PA___GFP_NOMEMALLOC) != 0 {
        return 0;
    }
    let mut flags = RUST_PA_ALLOC_NON_BLOCK;
    if order > 0 && gfp & RUST_PA___GFP_HIGH != 0 {
        flags |= RUST_PA_ALLOC_HIGHATOMIC;
    }
    flags
}
unsafe fn alloc_flags_slowpath(gfp: gfp_t, order: u32) -> u32 {
    let mut flags = RUST_PA_ALLOC_WMARK_MIN | RUST_PA_ALLOC_CPUSET;
    if gfp & RUST_PA___GFP_HIGH != 0 {
        flags |= RUST_PA_ALLOC_MIN_RESERVE;
    }
    if gfp & RUST_PA___GFP_KSWAPD_RECLAIM != 0 {
        flags |= RUST_PA_ALLOC_KSWAPD;
    }
    flags |= alloc_flags_nonblocking(gfp, order);
    if gfp & RUST_PA___GFP_DIRECT_RECLAIM == 0 {
        if flags & RUST_PA_ALLOC_MIN_RESERVE != 0 {
            flags &= !RUST_PA_ALLOC_CPUSET;
        }
    } else if rust_pa_late_rt_or_dl_task(rust_pa_late_current()) && rust_pa_late_in_task() {
        flags |= RUST_PA_ALLOC_MIN_RESERVE;
    }
    flags |= alloc_flags_cma(gfp);
    if defrag_mode != 0 {
        flags |= RUST_PA_ALLOC_NOFRAGMENT;
    }
    flags
}
unsafe fn oom_reserves_allowed(t: *mut task_struct) -> bool {
    if !rust_pa_late_tsk_is_oom_victim(t) {
        return false;
    }
    #[cfg(not(CONFIG_MMU))]
    if !rust_pa_late_test_memdie() {
        return false;
    }
    true
}
unsafe fn __gfp_pfmemalloc_flags(gfp: gfp_t) -> i32 {
    if gfp & RUST_PA___GFP_NOMEMALLOC != 0 {
        return 0;
    }
    if gfp & RUST_PA___GFP_MEMALLOC != 0 {
        return RUST_PA_ALLOC_NO_WATERMARKS as i32;
    }
    let t = rust_pa_late_current();
    if rust_pa_late_in_serving_softirq() && (*t).flags & RUST_PA_PF_MEMALLOC != 0 {
        return RUST_PA_ALLOC_NO_WATERMARKS as i32;
    }
    if !rust_pa_late_in_interrupt() {
        if (*t).flags & RUST_PA_PF_MEMALLOC != 0 {
            return RUST_PA_ALLOC_NO_WATERMARKS as i32;
        }
        if oom_reserves_allowed(t) {
            return RUST_PA_ALLOC_OOM as i32;
        }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn gfp_pfmemalloc_allowed(gfp: gfp_t) -> bool {
    __gfp_pfmemalloc_flags(gfp) != 0
}

unsafe fn should_reclaim_retry(
    gfp: gfp_t,
    order: u32,
    ac: *mut alloc_context,
    flags: i32,
    progress: bool,
    loops: *mut i32,
) -> bool {
    if progress && order <= RUST_PA_PAGE_ALLOC_COSTLY_ORDER {
        *loops = 0;
    } else {
        *loops += 1;
    }
    let mut ret = false;
    if *loops <= RUST_PA_MAX_RECLAIM_RETRIES as i32 {
        let mut z = rust_pa_late_first_zones_zonelist(
            (*ac).zonelist,
            (*ac).highest_zoneidx,
            (*ac).nodemask,
        );
        while !(*z).zone.is_null() {
            let zone = (*z).zone;
            if !(rust_pa_late_cpusets_enabled()
                && flags & RUST_PA_ALLOC_CPUSET as i32 != 0
                && !rust_pa_late_cpuset_zone_allowed(zone, gfp))
            {
                let mark = rust_pa_late_min_wmark_pages(zone);
                let reclaimable = zone_reclaimable_pages(zone);
                let available = reclaimable
                    .wrapping_add(rust_pa_late_zone_page_state_snapshot(zone, NR_FREE_PAGES));
                let ok = __zone_watermark_ok(
                    zone,
                    order,
                    mark,
                    (*ac).highest_zoneidx as i32,
                    flags as u32,
                    available as Long,
                );
                rust_pa_late_trace_reclaim_retry_zone(
                    z,
                    order,
                    reclaimable,
                    available,
                    mark,
                    *loops,
                    ok,
                );
                if ok {
                    ret = true;
                    break;
                }
            }
            z = rust_pa_late_next_zones_zonelist(z.add(1), (*ac).highest_zoneidx, (*ac).nodemask);
        }
        if (*rust_pa_late_current()).flags & RUST_PA_PF_WQ_WORKER != 0 {
            schedule_timeout_uninterruptible(1);
        } else {
            rust_pa_late_cond_resched();
        }
    }
    if !ret {
        unreserve_highatomic_pageblock(ac, true)
    } else {
        ret
    }
}
unsafe fn check_retry_cpuset(cookie: u32, ac: *mut alloc_context) -> bool {
    if rust_pa_late_cpusets_enabled()
        && !(*ac).nodemask.is_null()
        && !rust_pa_late_cpuset_nodemask_valid((*ac).nodemask)
    {
        (*ac).nodemask = null();
        return true;
    }
    rust_pa_late_read_mems_allowed_retry(cookie)
}
unsafe fn check_alloc_stall_warn(gfp: gfp_t, mask: *const nodemask_t, order: u32, start: ULong) {
    let stall = rust_pa_late_jiffies_to_msecs(rust_pa_late_jiffies().wrapping_sub(start)) as ULong;
    if stall < ALLOC_STALL_WARN_MSECS as ULong {
        return;
    }
    if rust_pa_late_time_after_jiffies(core::ptr::read_volatile(core::ptr::addr_of!(
        alloc_stall_warn_jiffies
    ))) {
        return;
    }
    if gfp & RUST_PA___GFP_NOWARN != 0 || !rust_pa_late_stall_trylock() {
        return;
    }
    if rust_pa_late_time_after_jiffies(alloc_stall_warn_jiffies) {
        rust_pa_late_stall_unlock();
        return;
    }
    core::ptr::write_volatile(
        core::ptr::addr_of_mut!(alloc_stall_warn_jiffies),
        rust_pa_late_jiffies()
            .wrapping_add(rust_pa_late_msecs_to_jiffies(ALLOC_STALL_WARN_MSECS as u32)),
    );
    rust_pa_late_stall_unlock();
    rust_pa_late_print_stall(gfp, mask, order, stall / RUST_PA_MSEC_PER_SEC as ULong);
    rust_pa_warn_alloc_finish(gfp, mask);
}

unsafe fn __alloc_pages_slowpath(gfp: gfp_t, order: u32, ac: *mut alloc_context) -> *mut page {
    let can_reclaim = gfp & RUST_PA___GFP_DIRECT_RECLAIM != 0;
    let can_compact = can_reclaim && rust_pa_late_gfp_compaction_allowed(gfp);
    let nofail = gfp & RUST_PA___GFP_NOFAIL != 0;
    let costly = order > RUST_PA_PAGE_ALLOC_COSTLY_ORDER;
    let mut compact_first = false;
    let mut retry_reserves = true;
    let start = rust_pa_late_jiffies();
    if nofail {
        rust_pa_late_warn_nofail_reclaim(!can_reclaim);
        rust_pa_late_warn_nofail_memalloc(
            (*rust_pa_late_current()).flags & RUST_PA_PF_MEMALLOC != 0,
        );
    }
    'restart: loop {
        let mut compact_retries = 0;
        let mut no_progress_loops = 0;
        let mut compact_result = COMPACT_SKIPPED;
        let mut priority = DEF_COMPACT_PRIORITY;
        let cpuset_cookie = rust_pa_late_read_mems_allowed_begin();
        let zonelist_cookie = zonelist_iter_begin();
        if can_compact && (costly || (order > 0 && (*ac).migratetype != MIGRATE_MOVABLE as i32)) {
            compact_first = true;
            priority = INIT_COMPACT_PRIORITY;
        }
        rust_pa_late_warn_wmark_flags((*ac).alloc_flags & RUST_PA_ALLOC_WMARK_MASK != 0);
        let mut flags = (*ac).alloc_flags | alloc_flags_slowpath(gfp, order);
        (*ac).preferred_zoneref = rust_pa_late_first_zones_zonelist(
            (*ac).zonelist,
            (*ac).highest_zoneidx,
            (*ac).nodemask,
        );
        let mut enter_nopage = (*(*ac).preferred_zoneref).zone.is_null();
        if !enter_nopage
            && rust_pa_late_cpusets_insane_config()
            && gfp & RUST_PA___GFP_HARDWALL != 0
        {
            let z = rust_pa_late_first_zones_zonelist(
                (*ac).zonelist,
                (*ac).highest_zoneidx,
                rust_pa_late_current_mems_allowed(),
            );
            enter_nopage = (*z).zone.is_null();
        }
        'retry: loop {
            'attempt: {
                if enter_nopage {
                    enter_nopage = false;
                    break 'attempt;
                }
                if flags & RUST_PA_ALLOC_KSWAPD != 0 {
                    wake_all_kswapds(order, gfp, ac);
                }
                let p = pa_late_freelist(gfp, order, flags, ac);
                if !p.is_null() {
                    return p;
                }
                let reserves = __gfp_pfmemalloc_flags(gfp) as u32;
                if reserves != 0 {
                    flags = alloc_flags_cma(gfp)
                        | reserves
                        | (*ac).alloc_flags
                        | (flags & RUST_PA_ALLOC_KSWAPD);
                }
                if flags & RUST_PA_ALLOC_CPUSET == 0 || reserves != 0 {
                    (*ac).nodemask = null();
                    (*ac).preferred_zoneref = rust_pa_late_first_zones_zonelist(
                        (*ac).zonelist,
                        (*ac).highest_zoneidx,
                        (*ac).nodemask,
                    );
                    if retry_reserves {
                        retry_reserves = false;
                        continue 'retry;
                    }
                }
                if !can_reclaim {
                    if defrag_mode != 0
                        && flags & RUST_PA_ALLOC_NOFRAGMENT != 0
                        && gfp & RUST_PA___GFP_KSWAPD_RECLAIM != 0
                    {
                        flags &= !RUST_PA_ALLOC_NOFRAGMENT;
                        continue 'retry;
                    }
                    break 'attempt;
                }
                if (*rust_pa_late_current()).flags & RUST_PA_PF_MEMALLOC != 0 {
                    break 'attempt;
                }
                check_alloc_stall_warn(gfp, (*ac).nodemask, order, start);
                let mut progress: ULong = 0;
                if !compact_first {
                    let p = __alloc_pages_direct_reclaim(gfp, order, flags, ac, &mut progress);
                    if !p.is_null() {
                        return p;
                    }
                }
                let p = __alloc_pages_direct_compact(
                    gfp,
                    order,
                    flags,
                    ac,
                    priority,
                    &mut compact_result,
                );
                if !p.is_null() {
                    return p;
                }
                if compact_first {
                    if rust_pa_late_gfp_has_flags(
                        gfp,
                        RUST_PA___GFP_NORETRY | RUST_PA___GFP_THISNODE,
                    ) {
                        break 'attempt;
                    }
                    if gfp & RUST_PA___GFP_NORETRY == 0 {
                        priority = DEF_COMPACT_PRIORITY;
                    }
                    compact_first = false;
                    continue 'retry;
                }
                if gfp & RUST_PA___GFP_NORETRY != 0 {
                    break 'attempt;
                }
                if costly && (!can_compact || gfp & RUST_PA___GFP_RETRY_MAYFAIL == 0) {
                    break 'attempt;
                }
                if check_retry_cpuset(cpuset_cookie, ac)
                    || check_retry_zonelist(zonelist_cookie) != 0
                {
                    continue 'restart;
                }
                if should_reclaim_retry(
                    gfp,
                    order,
                    ac,
                    flags as i32,
                    progress > 0,
                    &mut no_progress_loops,
                ) {
                    continue 'retry;
                }
                if progress > 0
                    && can_compact
                    && should_compact_retry(
                        gfp,
                        ac,
                        order as i32,
                        flags as i32,
                        compact_result,
                        &mut priority,
                        &mut compact_retries,
                    )
                {
                    continue 'retry;
                }
                if defrag_mode != 0 && flags & RUST_PA_ALLOC_NOFRAGMENT != 0 {
                    flags &= !RUST_PA_ALLOC_NOFRAGMENT;
                    continue 'retry;
                }
                if check_retry_cpuset(cpuset_cookie, ac)
                    || check_retry_zonelist(zonelist_cookie) != 0
                {
                    continue 'restart;
                }
                let p = __alloc_pages_may_oom(gfp, order, ac, &mut progress);
                if !p.is_null() {
                    return p;
                }
                if rust_pa_late_tsk_is_oom_victim(rust_pa_late_current())
                    && (flags & RUST_PA_ALLOC_OOM != 0 || gfp & RUST_PA___GFP_NOMEMALLOC != 0)
                {
                    break 'attempt;
                }
                if progress != 0 {
                    no_progress_loops = 0;
                    continue 'retry;
                }
            }
            // Original nopage label: race check precedes the nofail reserve path.
            if check_retry_cpuset(cpuset_cookie, ac) || check_retry_zonelist(zonelist_cookie) != 0 {
                continue 'restart;
            }
            if nofail && can_reclaim {
                let p = __alloc_pages_cpuset_fallback(
                    gfp,
                    order,
                    (*ac).alloc_flags | RUST_PA_ALLOC_MIN_RESERVE,
                    ac,
                );
                if !p.is_null() {
                    return p;
                }
                rust_pa_late_cond_resched();
                continue 'retry;
            }
            warn_alloc(
                gfp,
                (*ac).nodemask,
                c"page allocation failure: order:%u".as_ptr(),
                order,
            );
            return null_mut();
        }
    }
}

unsafe fn prepare_alloc_pages(
    gfp: gfp_t,
    order: u32,
    preferred: i32,
    mask: *mut nodemask_t,
    ac: *mut alloc_context,
    alloc_gfp: *mut gfp_t,
    flags: *mut u32,
) -> bool {
    (*ac).highest_zoneidx = rust_pa_late_gfp_zone(gfp);
    (*ac).zonelist = rust_pa_late_node_zonelist(preferred, gfp);
    (*ac).nodemask = mask;
    (*ac).migratetype = rust_pa_late_gfp_migratetype(gfp);
    if rust_pa_late_cpusets_enabled() {
        *alloc_gfp |= RUST_PA___GFP_HARDWALL;
        if rust_pa_late_in_task() && (*ac).nodemask.is_null() {
            (*ac).nodemask = rust_pa_late_current_mems_allowed();
        } else {
            *flags |= RUST_PA_ALLOC_CPUSET;
        }
    }
    rust_pa_late_might_alloc(gfp);
    if *flags & RUST_PA_ALLOC_NOLOCK == 0 && rust_pa_late_should_fail_alloc_page(gfp, order) {
        return false;
    }
    *flags |= alloc_flags_cma(gfp);
    (*ac).spread_dirty_pages = gfp & RUST_PA___GFP_WRITE != 0;
    (*ac).preferred_zoneref =
        rust_pa_late_first_zones_zonelist((*ac).zonelist, (*ac).highest_zoneidx, (*ac).nodemask);
    true
}

#[no_mangle]
pub unsafe extern "C" fn alloc_pages_bulk_noprof(
    mut gfp: gfp_t,
    preferred: i32,
    mask: *mut nodemask_t,
    nr_pages: i32,
    pages: *mut *mut page,
) -> ULong {
    let mut populated = 0i32;
    let mut accounted = 0i32;
    while populated < nr_pages && !(*pages.add(populated as usize)).is_null() {
        populated += 1;
    }
    if nr_pages <= 0 || nr_pages - populated == 0 {
        return populated as ULong;
    }
    'bulk: {
        if rust_pa_late_memcg_kmem_online() && gfp & RUST_PA___GFP_ACCOUNT != 0 {
            break 'bulk;
        }
        if nr_pages - populated == 1 {
            break 'bulk;
        }
        #[cfg(CONFIG_PAGE_OWNER)]
        if rust_pa_late_page_owner_inited() {
            break 'bulk;
        }
        gfp &= gfp_allowed_mask;
        let mut ac: alloc_context = zeroed();
        // prepare_alloc_pages initializes every field used by this fast path;
        // alloc_flags is intentionally unused here, matching the native local.
        let mut flags = RUST_PA_ALLOC_WMARK_LOW;
        if !prepare_alloc_pages(gfp, 0, preferred, mask, &mut ac, &mut gfp, &mut flags) {
            return populated as ULong;
        }
        let mut z = ac.preferred_zoneref;
        let mut selected = null_mut();
        'zones: while !(*z).zone.is_null() {
            let zone = (*z).zone;
            if !(rust_pa_late_cpusets_enabled()
                && flags & RUST_PA_ALLOC_CPUSET != 0
                && !rust_pa_late_cpuset_zone_allowed(zone, gfp))
            {
                if rust_pa_late_nr_online_nodes() > 1
                    && zone != (*ac.preferred_zoneref).zone
                    && rust_pa_late_zone_to_nid(zone)
                        != rust_pa_late_zonelist_node_idx(ac.preferred_zoneref)
                {
                    break 'bulk;
                }
                cond_accept_memory(zone, 0, flags as i32);
                loop {
                    let mark = rust_pa_late_wmark_pages(zone, flags & RUST_PA_ALLOC_WMARK_MASK)
                        .wrapping_add((nr_pages - populated) as ULong);
                    if zone_watermark_fast(
                        zone,
                        0,
                        mark,
                        (*ac.preferred_zoneref).zone_idx as i32,
                        flags,
                        gfp,
                    ) {
                        selected = zone;
                        break 'zones;
                    }
                    if cond_accept_memory(zone, 0, flags as i32) {
                        continue;
                    }
                    if deferred_pages_enabled() && _deferred_grow_zone(zone, 0) {
                        continue;
                    }
                    break;
                }
            }
            z = rust_pa_late_next_zones_zonelist(z.add(1), ac.highest_zoneidx, ac.nodemask);
        }
        if selected.is_null() {
            break 'bulk;
        }
        let pcp = pcp_spin_trylock((*selected).per_cpu_pageset);
        if pcp.is_null() {
            break 'bulk;
        }
        let list =
            core::ptr::addr_of_mut!((*pcp).lists[order_to_pindex(ac.migratetype, 0) as usize]);
        while populated < nr_pages {
            if !(*pages.add(populated as usize)).is_null() {
                populated += 1;
                continue;
            }
            let p = __rmqueue_pcplist(selected, 0, ac.migratetype, flags, pcp, list);
            if p.is_null() {
                if accounted == 0 {
                    pcp_spin_unlock(pcp);
                    break 'bulk;
                }
                break;
            }
            accounted += 1;
            prep_new_page(p, 0, gfp, RUST_PA_ALLOC_DEFAULT);
            rust_pa_late_set_page_refcounted(p);
            *pages.add(populated as usize) = p;
            populated += 1;
        }
        pcp_spin_unlock(pcp);
        rust_pa_late_count_pgalloc(rust_pa_late_zone_idx(selected), accounted as Long);
        zone_statistics((*ac.preferred_zoneref).zone, selected, accounted as Long);
        return populated as ULong;
    }
    let p = __alloc_pages_noprof(gfp, 0, preferred, mask, RUST_PA_ALLOC_DEFAULT);
    if !p.is_null() {
        *pages.add(populated as usize) = p;
        populated += 1;
    }
    populated as ULong
}
#[no_mangle]
pub unsafe extern "C" fn free_pages_bulk(mut pages: *mut *mut page, mut count: ULong) {
    while count != 0 {
        let contiguous = rust_pa_late_num_pages_contiguous(pages, count);
        __free_contig_range(rust_pa_late_page_to_pfn(*pages), contiguous);
        count -= contiguous;
        pages = pages.add(contiguous as usize);
        rust_pa_late_cond_resched();
    }
}
unsafe fn alloc_order_allowed(gfp: gfp_t, order: u32, flags: u32) -> bool {
    if flags & RUST_PA_ALLOC_NOLOCK != 0 {
        return pcp_allowed_order(order);
    }
    !rust_pa_late_warn_alloc_order(order > RUST_PA_MAX_PAGE_ORDER, gfp)
}
unsafe fn alloc_nolock_allowed() -> bool {
    can_spin_trylock() && !deferred_pages_enabled()
}
const gfp_nolock: gfp_t =
    RUST_PA___GFP_NOWARN | RUST_PA___GFP_ZERO | RUST_PA___GFP_NOMEMALLOC | RUST_PA___GFP_COMP;

#[no_mangle]
pub unsafe extern "C" fn __alloc_frozen_pages_noprof(
    mut gfp: gfp_t,
    order: u32,
    preferred: i32,
    mask: *mut nodemask_t,
    flags: u32,
) -> *mut page {
    let mut ac: alloc_context = zeroed();
    ac.alloc_flags = flags;
    let mut fast_flags = flags;
    if rust_pa_late_warn_alloc_flags(
        flags & !(RUST_PA_ALLOC_NOLOCK | RUST_PA_ALLOC_NO_CODETAG) != 0,
    ) {
        return null_mut();
    }
    if !alloc_order_allowed(gfp, order, flags) {
        return null_mut();
    }
    if flags & RUST_PA_ALLOC_NOLOCK != 0 {
        rust_pa_late_warn_nolock_gfp(gfp & !(RUST_PA___GFP_ACCOUNT | gfp_nolock) != 0);
        if !alloc_nolock_allowed() {
            return null_mut();
        }
        gfp |= gfp_nolock;
        fast_flags |= RUST_PA_ALLOC_WMARK_MIN;
    } else {
        fast_flags |= RUST_PA_ALLOC_WMARK_LOW;
    }
    gfp &= gfp_allowed_mask;
    gfp = rust_pa_late_current_gfp_context(gfp);
    let mut alloc_gfp = gfp;
    if !prepare_alloc_pages(
        gfp,
        order,
        preferred,
        mask,
        &mut ac,
        &mut alloc_gfp,
        &mut fast_flags,
    ) {
        return null_mut();
    }
    if flags & RUST_PA_ALLOC_NOLOCK == 0 {
        fast_flags |= alloc_flags_nofragment((*ac.preferred_zoneref).zone, gfp);
    }
    fast_flags |= alloc_flags_nonblocking(gfp, order) & RUST_PA_ALLOC_HIGHATOMIC;
    let mut p = pa_late_freelist(alloc_gfp, order, fast_flags, &ac);
    if p.is_null() && flags & RUST_PA_ALLOC_NOLOCK == 0 {
        alloc_gfp = gfp;
        ac.spread_dirty_pages = false;
        ac.nodemask = mask;
        p = __alloc_pages_slowpath(alloc_gfp, order, &mut ac);
    }
    if rust_pa_late_memcg_kmem_online()
        && gfp & RUST_PA___GFP_ACCOUNT != 0
        && !p.is_null()
        && rust_pa_late_memcg_kmem_charge_page(p, gfp, order as i32) != 0
    {
        __free_frozen_pages(
            p,
            order,
            if flags & RUST_PA_ALLOC_NOLOCK != 0 {
                FPI_NOLOCK
            } else {
                0
            },
        );
        p = null_mut();
    }
    rust_pa_late_trace_mm_page_alloc(p, order, alloc_gfp, ac.migratetype);
    rust_pa_late_kmsan_alloc_page(p, order, alloc_gfp);
    p
}
#[no_mangle]
pub unsafe extern "C" fn __alloc_pages_noprof(
    gfp: gfp_t,
    order: u32,
    preferred: i32,
    mask: *mut nodemask_t,
    flags: u32,
) -> *mut page {
    let p = __alloc_frozen_pages_noprof(gfp, order, preferred, mask, flags);
    if !p.is_null() {
        rust_pa_late_set_page_refcounted(p);
    }
    p
}
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_node_noprof(
    mut nid: i32,
    gfp: gfp_t,
    order: u32,
) -> *mut page {
    if nid == RUST_PA_NUMA_NO_NODE as i32 {
        nid = rust_pa_late_numa_mem_id();
    }
    rust_pa_late_warn_if_node_offline(nid, gfp);
    __alloc_pages_noprof(gfp, order, nid, null_mut(), RUST_PA_ALLOC_DEFAULT)
}
#[no_mangle]
pub unsafe extern "C" fn __folio_alloc_noprof(
    gfp: gfp_t,
    order: u32,
    nid: i32,
    mask: *mut nodemask_t,
) -> *mut folio {
    rust_pa_late_page_rmappable_folio(__alloc_pages_noprof(
        gfp | RUST_PA___GFP_COMP,
        order,
        nid,
        mask,
        RUST_PA_ALLOC_DEFAULT,
    ))
}
#[no_mangle]
pub unsafe extern "C" fn get_free_pages_noprof(gfp: gfp_t, order: u32) -> ULong {
    let p = rust_pa_late_alloc_pages_noprof(gfp & !RUST_PA___GFP_HIGHMEM, order);
    if p.is_null() {
        0
    } else {
        rust_pa_late_page_address(p) as ULong
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_zeroed_page_noprof(gfp: gfp_t) -> ULong {
    get_free_pages_noprof(gfp | RUST_PA___GFP_ZERO, 0)
}
unsafe fn ___free_pages(p: *mut page, mut order: u32, fpi: fpi_t) {
    let head = rust_pa_late_page_head(p);
    let tag = rust_pa_late_pgalloc_tag_get(p);
    if rust_pa_late_put_page_testzero(p) {
        __free_frozen_pages(p, order, fpi);
    } else if !head {
        pgalloc_tag_sub_pages(tag, (1i32 << order).wrapping_sub(1) as u32);
        while order > 0 {
            order -= 1;
            let tail = p.add(1usize << order);
            rust_pa_late_clear_page_tag_ref(tail);
            __free_frozen_pages(tail, order, fpi);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __free_pages(p: *mut page, order: u32) {
    ___free_pages(p, order, FPI_NONE);
}
#[no_mangle]
pub unsafe extern "C" fn free_pages_nolock(p: *mut page, order: u32) {
    ___free_pages(p, order, FPI_NOLOCK);
}
#[no_mangle]
pub unsafe extern "C" fn free_pages(addr: ULong, order: u32) {
    if addr != 0 {
        rust_pa_late_bug_bad_free_address(!rust_pa_late_virt_addr_valid(addr as *const Void));
        __free_pages(rust_pa_late_virt_to_page(addr as *const Void), order);
    }
}
unsafe fn make_alloc_exact(addr: ULong, order: u32, size: usize) -> *mut Void {
    if addr != 0 {
        let nr = size.wrapping_add(RUST_PA_PAGE_SIZE as usize - 1) / RUST_PA_PAGE_SIZE as usize;
        let mut p = rust_pa_late_virt_to_page(addr as *const Void);
        let mut last = p.add(nr);
        __split_page(p, order);
        loop {
            last = last.sub(1);
            if p >= last {
                break;
            }
            rust_pa_late_set_page_refcounted(last);
        }
        last = p.add(1usize << order);
        p = p.add(nr);
        while p < last {
            __free_pages_ok(p, 0, FPI_TO_TAIL);
            p = p.add(1);
        }
    }
    addr as *mut Void
}
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_exact_noprof(size: usize, mut gfp: gfp_t) -> *mut Void {
    let order = rust_pa_late_get_order(size);
    if rust_pa_late_warn_exact_flags(gfp & (RUST_PA___GFP_COMP | RUST_PA___GFP_HIGHMEM) != 0) {
        gfp &= !(RUST_PA___GFP_COMP | RUST_PA___GFP_HIGHMEM);
    }
    make_alloc_exact(get_free_pages_noprof(gfp, order), order, size)
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn alloc_pages_exact_nid_noprof(
    nid: i32,
    size: usize,
    mut gfp: gfp_t,
) -> *mut Void {
    let order = rust_pa_late_get_order(size);
    if rust_pa_late_warn_exact_nid_flags(gfp & (RUST_PA___GFP_COMP | RUST_PA___GFP_HIGHMEM) != 0) {
        gfp &= !(RUST_PA___GFP_COMP | RUST_PA___GFP_HIGHMEM);
    }
    let p = alloc_pages_node_noprof(nid, gfp, order);
    if p.is_null() {
        null_mut()
    } else {
        make_alloc_exact(rust_pa_late_page_address(p) as ULong, order, size)
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_pages_exact(ptr: *mut Void, size: usize) {
    let mut addr = ptr as ULong;
    let end = addr.wrapping_add(
        size.wrapping_add(RUST_PA_PAGE_SIZE as usize - 1) as ULong
            & !(RUST_PA_PAGE_SIZE as ULong - 1),
    );
    while addr < end {
        free_pages(addr, 0);
        addr = addr.wrapping_add(RUST_PA_PAGE_SIZE as ULong);
    }
}

// alloc_flags is unsigned in the slowpath but int in the native freelist API.
#[inline]
unsafe fn pa_late_freelist(
    gfp: gfp_t,
    order: u32,
    flags: u32,
    ac: *const alloc_context,
) -> *mut page {
    get_page_from_freelist(gfp, order, flags as i32, ac)
}
