// SPDX-License-Identifier: GPL-2.0-only
// Buddy fallback, per-CPU allocation/free policy, reserves and zonelist search.
static fallbacks: [[i32; MIGRATE_PCPTYPES as usize - 1]; MIGRATE_PCPTYPES as usize] = [
    [MIGRATE_RECLAIMABLE as i32, MIGRATE_MOVABLE as i32],
    [MIGRATE_RECLAIMABLE as i32, MIGRATE_UNMOVABLE as i32],
    [MIGRATE_UNMOVABLE as i32, MIGRATE_MOVABLE as i32],
];
#[inline]
unsafe fn __rmqueue_cma_fallback(z: *mut zone, order: u32) -> *mut page {
    #[cfg(CONFIG_CMA)]
    {
        __rmqueue_smallest(z, order, MIGRATE_CMA as i32)
    }
    #[cfg(not(CONFIG_CMA))]
    {
        null_mut()
    }
}
unsafe fn __move_freepages_block(z: *mut zone, start: ULong, old_mt: i32, new_mt: i32) -> i32 {
    pa_vm_warn!(start & (pb_pages() - 1) != 0);
    let end = pb_end(start);
    let mut pfn = start;
    let mut moved = 0i32;
    while pfn < end {
        let p = rust_pa_pfn_to_page(pfn);
        if !rust_pa_page_buddy(p) {
            pfn += 1;
            continue;
        }
        pa_vm_bug_page!(rust_pa_page_to_nid(p) != rust_pa_zone_to_nid(z), p);
        pa_vm_bug_page!(rust_pa_page_zone(p) != z, p);
        let order = rust_pa_buddy_order(p) as u32;
        move_to_free_list(p, z, order, old_mt, new_mt);
        pfn = pfn.wrapping_add(1 << order);
        moved = moved.wrapping_add(1 << order);
    }
    moved
}
unsafe fn prep_move_freepages_block(
    z: *mut zone,
    p: *mut page,
    start_pfn: *mut ULong,
    num_free: *mut i32,
    num_movable: *mut i32,
) -> bool {
    let pfn = rust_pa_page_to_pfn(p);
    let start = pb_start(pfn);
    let end = pb_end(pfn);
    if !zone_spans(z, start) || !zone_spans(z, end - 1) {
        return false;
    }
    *start_pfn = start;
    if !num_free.is_null() {
        *num_free = 0;
        *num_movable = 0;
        let mut pfn = start;
        while pfn < end {
            let p = rust_pa_pfn_to_page(pfn);
            if rust_pa_page_buddy(p) {
                let nr = 1 << rust_pa_buddy_order(p);
                *num_free += nr;
                pfn += nr as ULong;
                continue;
            }
            if rust_pa_page_lru(p) || rust_pa_page_has_movable_ops(p) {
                *num_movable += 1;
            }
            pfn += 1;
        }
    }
    true
}
unsafe fn move_freepages_block(z: *mut zone, p: *mut page, old_mt: i32, new_mt: i32) -> i32 {
    let mut start = 0;
    if !prep_move_freepages_block(z, p, &mut start, null_mut(), null_mut()) {
        return -1;
    }
    let res = __move_freepages_block(z, start, old_mt, new_mt);
    set_pageblock_migratetype(rust_pa_pfn_to_page(start), new_mt);
    res
}
#[cfg(CONFIG_MEMORY_ISOLATION)]
unsafe fn find_large_buddy(start: ULong) -> ULong {
    let mut order = if start != 0 {
        start.trailing_zeros()
    } else {
        RUST_PA_MAX_PAGE_ORDER as u32
    };
    let mut pfn = start;
    loop {
        let p = rust_pa_pfn_to_page(pfn);
        if rust_pa_page_buddy(p) {
            return if pfn.wrapping_add(1 << rust_pa_buddy_order(p)) > start {
                pfn
            } else {
                start
            };
        }
        order += 1;
        if order > RUST_PA_MAX_PAGE_ORDER as u32 {
            return start;
        }
        pfn &= ULong::MAX << order;
    }
}
#[cfg(CONFIG_MEMORY_ISOLATION)]
unsafe fn toggle_pageblock_isolate(p: *mut page, isolate: bool) {
    if isolate {
        rust_pa_set_pageblock_isolate(p);
    } else {
        rust_pa_clear_pageblock_isolate(p);
    }
}
#[cfg(CONFIG_MEMORY_ISOLATION)]
unsafe fn __move_freepages_block_isolate(z: *mut zone, p: *mut page, isolate: bool) -> bool {
    if isolate == rust_pa_get_pageblock_isolate(p) {
        rust_pa_warn_isolate_state(isolate);
        return false;
    }
    let mut start = 0;
    if !prep_move_freepages_block(z, p, &mut start, null_mut(), null_mut()) {
        return false;
    }
    if pb_order() != RUST_PA_MAX_PAGE_ORDER as u32 {
        let buddy_pfn = find_large_buddy(start);
        let buddy = rust_pa_pfn_to_page(buddy_pfn);
        let order = rust_pa_buddy_order(buddy) as u32;
        if rust_pa_page_buddy(buddy) && order > pb_order() {
            del_page_from_free_list(
                buddy,
                z,
                order,
                get_pfnblock_migratetype(buddy, buddy_pfn) as i32,
            );
            toggle_pageblock_isolate(p, isolate);
            split_large_buddy(z, buddy, buddy_pfn, order as i32, FPI_NONE);
            return true;
        }
    }
    let mt = __get_pfnblock_flags_mask(
        p,
        rust_pa_page_to_pfn(p),
        RUST_PA_PAGEBLOCK_MIGRATETYPE_MASK as ULong,
    ) as i32;
    let (from, to) = if isolate {
        (mt, MIGRATE_ISOLATE as i32)
    } else {
        (MIGRATE_ISOLATE as i32, mt)
    };
    __move_freepages_block(z, start, from, to);
    toggle_pageblock_isolate(rust_pa_pfn_to_page(start), isolate);
    true
}
#[cfg(CONFIG_MEMORY_ISOLATION)]
#[no_mangle]
pub unsafe extern "C" fn pageblock_isolate_and_move_free_pages(z: *mut zone, p: *mut page) -> bool {
    __move_freepages_block_isolate(z, p, true)
}
#[cfg(CONFIG_MEMORY_ISOLATION)]
#[no_mangle]
pub unsafe extern "C" fn pageblock_unisolate_and_move_free_pages(
    z: *mut zone,
    p: *mut page,
) -> bool {
    __move_freepages_block_isolate(z, p, false)
}
unsafe fn boost_watermark(z: *mut zone) -> bool {
    if watermark_boost_factor == 0 || pb_pages() * 4 > rust_pa_zone_managed_pages(z) {
        return false;
    }
    let high = (*z)._watermark[WMARK_HIGH as usize];
    let fraction = watermark_boost_factor as ULong;
    let boost = (high / 10000)
        .wrapping_mul(fraction)
        .wrapping_add((high % 10000).wrapping_mul(fraction) / 10000);
    if boost == 0 {
        return false;
    }
    (*z).watermark_boost = min(
        (*z).watermark_boost.wrapping_add(pb_pages()),
        max(pb_pages(), boost),
    );
    true
}
unsafe fn should_try_claim_block(order: u32, mt: i32) -> bool {
    order >= pb_order()
        || order >= pb_order() / 2
        || mt == MIGRATE_RECLAIMABLE as i32
        || mt == MIGRATE_UNMOVABLE as i32
        || page_group_by_mobility_disabled != 0
}
#[export_name = "rust_pa_impl_find_suitable_fallback"]
pub unsafe extern "C" fn find_suitable_fallback(
    area: *mut free_area,
    order: u32,
    mt: i32,
    claimable: bool,
    mt_out: *mut i32,
) -> fallback_result {
    if claimable && !should_try_claim_block(order, mt) {
        return FALLBACK_NOCLAIM;
    }
    if (*area).nr_free == 0 {
        return FALLBACK_EMPTY;
    }
    for &fallback in &fallbacks[mt as usize] {
        if !list_empty(addr_of!((*area).free_list[fallback as usize])) {
            if !mt_out.is_null() {
                *mt_out = fallback;
            }
            return FALLBACK_FOUND;
        }
    }
    FALLBACK_EMPTY
}
unsafe fn try_to_claim_block(
    z: *mut zone,
    p: *mut page,
    current_order: i32,
    order: i32,
    start_type: i32,
    block_type: i32,
    flags: u32,
) -> *mut page {
    if current_order >= pb_order() as i32 {
        del_page_from_free_list(p, z, current_order as u32, block_type);
        change_pageblock_range(p, current_order, start_type);
        let added = expand(z, p, order, current_order, start_type);
        account_freepages(z, added as i32, start_type);
        return p;
    }
    if boost_watermark(z) && flags & RUST_PA_ALLOC_KSWAPD as u32 != 0 {
        rust_pa_set_bit(ZONE_BOOSTED_WATERMARK as ULong, addr_of_mut!((*z).flags));
    }
    let (mut start, mut free, mut movable) = (0, 0, 0);
    if !prep_move_freepages_block(z, p, &mut start, &mut free, &mut movable) {
        return null_mut();
    }
    let alike = if start_type == MIGRATE_MOVABLE as i32 {
        movable
    } else if block_type == MIGRATE_MOVABLE as i32 {
        pb_pages() as i32 - free - movable
    } else {
        0
    };
    if free + alike >= 1i32 << (pb_order() - 1) || page_group_by_mobility_disabled != 0 {
        __move_freepages_block(z, start, block_type, start_type);
        set_pageblock_migratetype(rust_pa_pfn_to_page(start), start_type);
        return __rmqueue_smallest(z, order as u32, start_type);
    }
    null_mut()
}
unsafe fn __rmqueue_claim(z: *mut zone, order: i32, mt: i32, flags: u32) -> *mut page {
    let min_order = if order < pb_order() as i32 && flags & RUST_PA_ALLOC_NOFRAGMENT as u32 != 0 {
        pb_order() as i32
    } else {
        order
    };
    let mut current_order = RUST_PA_MAX_PAGE_ORDER as i32;
    while current_order >= min_order {
        let area = addr_of_mut!((*z).free_area[current_order as usize]);
        let mut fallback = 0;
        let result = find_suitable_fallback(area, current_order as u32, mt, true, &mut fallback);
        if result == FALLBACK_NOCLAIM {
            break;
        }
        if result != FALLBACK_EMPTY {
            let p = get_page_from_free_area(area, fallback);
            let p = try_to_claim_block(z, p, current_order, order, mt, fallback, flags);
            if !p.is_null() {
                rust_pa_trace_extfrag(p, order, current_order, mt, fallback);
                return p;
            }
        }
        current_order -= 1;
    }
    null_mut()
}
unsafe fn __rmqueue_steal(z: *mut zone, order: i32, mt: i32) -> *mut page {
    for current_order in order..RUST_PA_NR_PAGE_ORDERS as i32 {
        let area = addr_of_mut!((*z).free_area[current_order as usize]);
        let mut fallback = 0;
        if find_suitable_fallback(area, current_order as u32, mt, false, &mut fallback)
            == FALLBACK_EMPTY
        {
            continue;
        }
        let p = get_page_from_free_area(area, fallback);
        page_del_and_expand(z, p, order, current_order, fallback);
        rust_pa_trace_extfrag(p, order, current_order, mt, fallback);
        return p;
    }
    null_mut()
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum RmqueueMode {
    Normal,
    Cma,
    Claim,
    Steal,
}
unsafe fn __rmqueue(
    z: *mut zone,
    order: u32,
    mt: i32,
    flags: u32,
    mode: &mut RmqueueMode,
) -> *mut page {
    #[cfg(CONFIG_CMA)]
    if flags & RUST_PA_ALLOC_CMA as u32 != 0
        && rust_pa_zone_page_state(z, NR_FREE_CMA_PAGES)
            > rust_pa_zone_page_state(z, NR_FREE_PAGES) / 2
    {
        let p = __rmqueue_cma_fallback(z, order);
        if !p.is_null() {
            return p;
        }
    }
    if *mode == RmqueueMode::Normal {
        let p = __rmqueue_smallest(z, order, mt);
        if !p.is_null() {
            return p;
        }
    }
    if *mode == RmqueueMode::Normal || *mode == RmqueueMode::Cma {
        if flags & RUST_PA_ALLOC_CMA as u32 != 0 {
            let p = __rmqueue_cma_fallback(z, order);
            if !p.is_null() {
                *mode = RmqueueMode::Cma;
                return p;
            }
        }
    }
    if *mode != RmqueueMode::Steal {
        let p = __rmqueue_claim(z, order as i32, mt, flags);
        if !p.is_null() {
            *mode = RmqueueMode::Normal;
            return p;
        }
    }
    if flags & RUST_PA_ALLOC_NOFRAGMENT as u32 == 0 {
        let p = __rmqueue_steal(z, order as i32, mt);
        if !p.is_null() {
            *mode = RmqueueMode::Steal;
            return p;
        }
    }
    null_mut()
}
unsafe fn rmqueue_bulk(
    z: *mut zone,
    order: u32,
    count: ULong,
    list: *mut list_head,
    mt: i32,
    alloc_flags: u32,
) -> i32 {
    let mut mode = RmqueueMode::Normal;
    let mut flags = 0;
    if alloc_flags & RUST_PA_ALLOC_NOLOCK as u32 != 0 {
        if !rust_pa_spin_trylock_irqsave(addr_of_mut!((*z).lock), &mut flags) {
            return 0;
        }
    } else {
        flags = rust_pa_spin_lock_irqsave(addr_of_mut!((*z).lock));
    }
    let mut i = 0i32;
    while (i as ULong) < count {
        let p = __rmqueue(z, order, mt, alloc_flags, &mut mode);
        if p.is_null() {
            break;
        }
        list_add_tail(page_pcp_list(p), list);
        i += 1;
    }
    rust_pa_spin_unlock_irqrestore(addr_of_mut!((*z).lock), flags);
    i
}
#[no_mangle]
pub unsafe extern "C" fn decay_pcp_high(z: *mut zone, pcp: *mut per_cpu_pages) -> bool {
    let high_min = rust_pa_read_int(addr_of!((*pcp).high_min));
    let batch = rust_pa_read_int(addr_of!((*pcp).batch));
    let mut todo = false;
    if (*pcp).high > high_min {
        (*pcp).high = max(
            max(
                (*pcp).count - (batch << RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX),
                (*pcp).high - ((*pcp).high >> 3),
            ),
            high_min,
        );
        if (*pcp).high > high_min {
            todo = true;
        }
    }
    let mut drain = (*pcp).count - (*pcp).high;
    while drain > 0 {
        let n = min(drain, batch);
        pcp_spin_lock_nopin(pcp);
        free_pcppages_bulk(z, n, pcp, 0);
        pcp_spin_unlock_nopin(pcp);
        todo = true;
        drain -= n;
    }
    todo
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn drain_zone_pages(z: *mut zone, pcp: *mut per_cpu_pages) {
    let batch = rust_pa_read_int(addr_of!((*pcp).batch));
    let n = min((*pcp).count, batch);
    if n > 0 {
        pcp_spin_lock_nopin(pcp);
        free_pcppages_bulk(z, n, pcp, 0);
        pcp_spin_unlock_nopin(pcp);
    }
}
unsafe fn drain_pages_zone(cpu: u32, z: *mut zone) {
    let pcp = rust_pa_per_cpu_pcp((*z).per_cpu_pageset, cpu);
    loop {
        pcp_spin_lock_nopin(pcp);
        let mut count = (*pcp).count;
        if count != 0 {
            let n = min(count, (*pcp).batch << RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX);
            free_pcppages_bulk(z, n, pcp, 0);
            count -= n;
        }
        pcp_spin_unlock_nopin(pcp);
        if count == 0 {
            break;
        }
    }
}
unsafe fn drain_pages(cpu: u32) {
    let mut z = first_zone();
    while !z.is_null() {
        if (*z).present_pages != 0 {
            drain_pages_zone(cpu, z);
        }
        z = next_zone(z);
    }
}
#[no_mangle]
pub unsafe extern "C" fn drain_local_pages(z: *mut zone) {
    let cpu = rust_pa_smp_processor_id();
    if !z.is_null() {
        drain_pages_zone(cpu, z);
    } else {
        drain_pages(cpu);
    }
}
unsafe fn __drain_all_pages(z: *mut zone, force: bool) {
    static mut CPUS_WITH_PCPS: cpumask = unsafe { zeroed() };
    let mutex = addr_of_mut!(pcpu_drain_mutex);
    if !rust_pa_mutex_trylock(mutex) {
        if z.is_null() {
            return;
        }
        rust_pa_mutex_lock(mutex);
    }
    let ncpus = rust_pa_nr_cpu_ids();
    for cpu in 0..ncpus {
        if !rust_pa_cpu_online(cpu) {
            continue;
        }
        let mut has = force;
        if !has && !z.is_null() {
            has = (*rust_pa_per_cpu_pcp((*z).per_cpu_pageset, cpu)).count != 0;
        } else if !has {
            let mut zone = first_zone();
            while !zone.is_null() {
                if (*zone).present_pages != 0
                    && (*rust_pa_per_cpu_pcp((*zone).per_cpu_pageset, cpu)).count != 0
                {
                    has = true;
                    break;
                }
                zone = next_zone(zone);
            }
        }
        if has {
            rust_pa_cpumask_set_cpu(cpu, addr_of_mut!(CPUS_WITH_PCPS));
        } else {
            rust_pa_cpumask_clear_cpu(cpu, addr_of_mut!(CPUS_WITH_PCPS));
        }
    }
    for cpu in 0..ncpus {
        if rust_pa_cpumask_test_cpu(cpu, addr_of!(CPUS_WITH_PCPS)) {
            if !z.is_null() {
                drain_pages_zone(cpu, z);
            } else {
                drain_pages(cpu);
            }
        }
    }
    rust_pa_mutex_unlock(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn drain_all_pages(z: *mut zone) {
    __drain_all_pages(z, false);
}
unsafe fn nr_pcp_free(pcp: *mut per_cpu_pages, batch: i32, high: i32, free_high: bool) -> i32 {
    if free_high {
        return min((*pcp).count, batch << RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX);
    }
    if high < batch {
        return 1;
    }
    min(max((*pcp).free_count as i32, batch), high - batch)
}
unsafe fn nr_pcp_high(pcp: *mut per_cpu_pages, z: *mut zone, batch: i32, free_high: bool) -> i32 {
    let low = rust_pa_read_int(addr_of!((*pcp).high_min));
    let high_max = rust_pa_read_int(addr_of!((*pcp).high_max));
    let mut high = min(max((*pcp).high, low), high_max);
    (*pcp).high = high;
    if high == 0 {
        return 0;
    }
    if free_high {
        (*pcp).high = max(high - (batch << RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX), low);
        return 0;
    }
    if rust_pa_test_bit(ZONE_RECLAIM_ACTIVE as ULong, addr_of!((*z).flags)) {
        (*pcp).high = max(high - max((*pcp).free_count as i32, batch), low);
        return min(batch << 2, (*pcp).high);
    }
    if low == high_max {
        return high;
    }
    if rust_pa_test_bit(ZONE_BELOW_HIGH as ULong, addr_of!((*z).flags)) {
        (*pcp).high = max(high - max((*pcp).free_count as i32, batch), low);
        high = max((*pcp).count, low);
    } else if (*pcp).count >= high {
        let need = (*pcp).free_count as i32 + batch;
        if (*pcp).high < need {
            (*pcp).high = min(max(need, low), high_max);
        }
    }
    high
}
unsafe fn free_frozen_page_commit(
    z: *mut zone,
    mut pcp: *mut per_cpu_pages,
    p: *mut page,
    mt: i32,
    order: u32,
    fpi: fpi_t,
) -> bool {
    let cpu = rust_pa_smp_processor_id();
    let mut ret = true;
    let mut free_high = false;
    (*pcp).alloc_factor >>= 1;
    rust_pa_count_vm_events(PGFREE, 1 << order);
    let pindex = order_to_pindex(mt, order as i32);
    list_add(
        page_pcp_list(p),
        addr_of_mut!((*pcp).lists[pindex as usize]),
    );
    (*pcp).count += 1 << order;
    let batch = rust_pa_read_int(addr_of!((*pcp).batch));
    if order != 0 && order <= RUST_PA_PAGE_ALLOC_COSTLY_ORDER as u32 {
        free_high = (*pcp).free_count as i32 >= batch + (*pcp).high_min / 2
            && (*pcp).flags & RUST_PA_PCPF_PREV_FREE_HIGH_ORDER as u8 != 0
            && ((*pcp).flags & RUST_PA_PCPF_FREE_HIGH_BATCH as u8 == 0 || (*pcp).count >= batch);
        (*pcp).flags |= RUST_PA_PCPF_PREV_FREE_HIGH_ORDER as u8;
    } else if (*pcp).flags & RUST_PA_PCPF_PREV_FREE_HIGH_ORDER as u8 != 0 {
        (*pcp).flags &= !(RUST_PA_PCPF_PREV_FREE_HIGH_ORDER as u8);
    }
    if ((*pcp).free_count as i32) < batch << RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX {
        (*pcp).free_count = (*pcp).free_count.wrapping_add(1i16 << order);
    }
    if fpi & FPI_NOLOCK != 0 {
        return true;
    }
    let high = nr_pcp_high(pcp, z, batch, free_high);
    if (*pcp).count < high {
        return true;
    }
    let mut to_free = nr_pcp_free(pcp, batch, high, free_high);
    while to_free > 0 && (*pcp).count > 0 {
        let n = min(to_free, batch);
        free_pcppages_bulk(z, n, pcp, pindex as i32);
        to_free -= n;
        if to_free == 0 || (*pcp).count == 0 {
            break;
        }
        pcp_spin_unlock(pcp);
        pcp = pcp_spin_trylock((*z).per_cpu_pageset);
        if pcp.is_null() {
            ret = false;
            break;
        }
        if rust_pa_smp_processor_id() != cpu {
            pcp_spin_unlock(pcp);
            ret = false;
            break;
        }
    }
    if rust_pa_test_bit(ZONE_BELOW_HIGH as ULong, addr_of!((*z).flags))
        && zone_watermark_ok(
            z,
            0,
            wmark_pages(z, WMARK_HIGH as u32),
            ZONE_MOVABLE as i32,
            0,
        )
    {
        let pgdat = (*z).zone_pgdat;
        rust_pa_clear_bit(ZONE_BELOW_HIGH as ULong, addr_of_mut!((*z).flags));
        if rust_pa_kswapd_test_hopeless(pgdat)
            && rust_pa_next_memory_node((*pgdat).node_id) < RUST_PA_MAX_NUMNODES as i32
        {
            rust_pa_kswapd_clear_hopeless(pgdat);
        }
    }
    ret
}
unsafe fn __free_frozen_pages(p: *mut page, order: u32, fpi: fpi_t) {
    let pfn = rust_pa_page_to_pfn(p);
    if !pcp_allowed_order(order) {
        __free_pages_ok(p, order, fpi);
        return;
    }
    if !__free_pages_prepare(p, order, fpi) {
        return;
    }
    let z = rust_pa_page_zone(p);
    let mut mt = get_pfnblock_migratetype(p, pfn) as i32;
    if mt >= MIGRATE_PCPTYPES as i32 {
        if is_migrate_isolate(mt) {
            free_one_page(z, p, pfn, order, fpi);
            return;
        }
        mt = MIGRATE_MOVABLE as i32;
    }
    if fpi & FPI_NOLOCK != 0 && !can_spin_trylock() {
        add_page_to_zone_llist(z, p, order);
        return;
    }
    let pcp = pcp_spin_trylock((*z).per_cpu_pageset);
    if !pcp.is_null() {
        if !free_frozen_page_commit(z, pcp, p, mt, order, fpi) {
            return;
        }
        pcp_spin_unlock(pcp);
    } else {
        free_one_page(z, p, pfn, order, fpi);
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_frozen_pages(p: *mut page, o: u32) {
    __free_frozen_pages(p, o, FPI_NONE);
}
#[no_mangle]
pub unsafe extern "C" fn free_frozen_pages_nolock(p: *mut page, o: u32) {
    __free_frozen_pages(p, o, FPI_NOLOCK);
}
#[no_mangle]
pub unsafe extern "C" fn free_unref_folios(folios: *mut folio_batch) {
    let mut pcp: *mut per_cpu_pages = null_mut();
    let mut locked_zone = null_mut();
    let mut j = 0;
    for i in 0..(*folios).nr as usize {
        let f = (*folios).folios[i];
        let p = rust_pa_folio_page(f);
        let pfn = rust_pa_page_to_pfn(p);
        let order = rust_pa_folio_order(f);
        if !__free_pages_prepare(p, order, FPI_NONE) {
            continue;
        }
        if !pcp_allowed_order(order) {
            free_one_page(rust_pa_page_zone(p), p, pfn, order, FPI_NONE);
            continue;
        }
        rust_pa_set_folio_private(f, order as usize as *mut Void);
        if j != i {
            (*folios).folios[j] = f;
        }
        j += 1;
    }
    (*folios).nr = j as _;
    for i in 0..j {
        let f = (*folios).folios[i];
        let p = rust_pa_folio_page(f);
        let z = rust_pa_page_zone(p);
        let pfn = rust_pa_page_to_pfn(p);
        let order = rust_pa_folio_private(f) as usize as u32;
        rust_pa_set_folio_private(f, null_mut());
        let mut mt = get_pfnblock_migratetype(p, pfn) as i32;
        if z != locked_zone || is_migrate_isolate(mt) {
            if !pcp.is_null() {
                pcp_spin_unlock(pcp);
                locked_zone = null_mut();
                pcp = null_mut();
            }
            if is_migrate_isolate(mt) {
                free_one_page(z, p, pfn, order, FPI_NONE);
                continue;
            }
            pcp = pcp_spin_trylock((*z).per_cpu_pageset);
            if pcp.is_null() {
                free_one_page(z, p, pfn, order, FPI_NONE);
                continue;
            }
            locked_zone = z;
        }
        if mt >= MIGRATE_PCPTYPES as i32 {
            mt = MIGRATE_MOVABLE as i32;
        }
        rust_pa_trace_free_batched(p);
        if !free_frozen_page_commit(z, pcp, p, mt, order, FPI_NONE) {
            pcp = null_mut();
            locked_zone = null_mut();
        }
    }
    if !pcp.is_null() {
        pcp_spin_unlock(pcp);
    }
    rust_pa_folio_batch_reinit(folios);
}
unsafe fn __split_page(p: *mut page, order: u32) {
    pa_vm_warn_page!(rust_pa_page_compound(p), p);
    rust_pa_split_page_owner(p, order);
    rust_pa_pgalloc_tag_split(p, order);
    rust_pa_split_page_memcg(p, order);
}
#[no_mangle]
pub unsafe extern "C" fn split_page(p: *mut page, order: u32) {
    pa_vm_warn_page!(rust_pa_page_count(p) == 0, p);
    for i in 1..1usize << order {
        rust_pa_set_page_refcounted(p.add(i));
    }
    __split_page(p, order);
}
#[no_mangle]
pub unsafe extern "C" fn __isolate_free_page(mut p: *mut page, order: u32) -> i32 {
    let z = rust_pa_page_zone(p);
    let mt = get_pfnblock_migratetype(p, rust_pa_page_to_pfn(p)) as i32;
    if !is_migrate_isolate(mt) {
        let mark = (*z)._watermark[WMARK_MIN as usize].wrapping_add(1 << order);
        if !zone_watermark_ok(z, 0, mark, 0, RUST_PA_ALLOC_CMA as u32) {
            return 0;
        }
    }
    del_page_from_free_list(p, z, order, mt);
    if order >= pb_order() - 1 {
        let end = p.add((1usize << order) - 1);
        while p < end {
            let mt = get_pfnblock_migratetype(p, rust_pa_page_to_pfn(p)) as i32;
            if migratetype_is_mergeable(mt) {
                move_freepages_block(z, p, mt, MIGRATE_MOVABLE as i32);
            }
            p = p.add(pb_pages() as usize);
        }
    }
    ((1 as ULong) << order) as i32
}
#[no_mangle]
pub unsafe extern "C" fn __putback_isolated_page(p: *mut page, order: u32, mt: i32) {
    let z = rust_pa_page_zone(p);
    rust_pa_assert_zone_locked(z);
    __free_one_page(
        p,
        rust_pa_page_to_pfn(p),
        z,
        order,
        mt,
        FPI_SKIP_REPORT_NOTIFY | FPI_TO_TAIL,
    );
}
unsafe fn zone_statistics(preferred: *mut zone, z: *mut zone, n: Long) {
    #[cfg(CONFIG_NUMA)]
    {
        if !rust_pa_numa_stat_enabled() {
            return;
        }
        let local = if rust_pa_zone_to_nid(z) != rust_pa_numa_node_id() {
            NUMA_OTHER
        } else {
            NUMA_LOCAL
        };
        if rust_pa_zone_to_nid(z) == rust_pa_zone_to_nid(preferred) {
            rust_pa_count_numa_events(z, NUMA_HIT, n);
        } else {
            rust_pa_count_numa_events(z, NUMA_MISS, n);
            rust_pa_count_numa_events(preferred, NUMA_FOREIGN, n);
        }
        rust_pa_count_numa_events(z, local, n);
    }
}
unsafe fn rmqueue_buddy(
    preferred: *mut zone,
    z: *mut zone,
    order: u32,
    flags: u32,
    mt: i32,
) -> *mut page {
    let p = loop {
        let mut p = null_mut();
        let mut irqflags = 0;
        if flags & RUST_PA_ALLOC_NOLOCK as u32 != 0 {
            if !rust_pa_spin_trylock_irqsave(addr_of_mut!((*z).lock), &mut irqflags) {
                return null_mut();
            }
        } else {
            irqflags = rust_pa_spin_lock_irqsave(addr_of_mut!((*z).lock));
        }
        if flags & RUST_PA_ALLOC_HIGHATOMIC as u32 != 0 {
            p = __rmqueue_smallest(z, order, MIGRATE_HIGHATOMIC as i32);
        }
        if p.is_null() {
            let mut mode = RmqueueMode::Normal;
            p = __rmqueue(z, order, mt, flags, &mut mode);
            if p.is_null() && flags & (RUST_PA_ALLOC_OOM | RUST_PA_ALLOC_NON_BLOCK) as u32 != 0 {
                p = __rmqueue_smallest(z, order, MIGRATE_HIGHATOMIC as i32);
            }
        }
        rust_pa_spin_unlock_irqrestore(addr_of_mut!((*z).lock), irqflags);
        if p.is_null() {
            return p;
        }
        if !check_new_pages(p, order) {
            break p;
        }
    };
    if flags & RUST_PA_ALLOC_HIGHATOMIC as u32 != 0
        && flags & RUST_PA_ALLOC_WMARK_MASK as u32 == RUST_PA_ALLOC_WMARK_MIN as u32
    {
        reserve_highatomic_pageblock(p, order as i32, z);
    }
    rust_pa_count_zid_vm_events(
        RUST_PA_PGALLOC,
        rust_pa_zone_idx(rust_pa_page_zone(p)),
        1 << order,
    );
    zone_statistics(preferred, z, 1);
    p
}
unsafe fn nr_pcp_alloc(pcp: *mut per_cpu_pages, z: *mut zone, order: i32) -> i32 {
    let base = rust_pa_read_int(addr_of!((*pcp).batch));
    let low = rust_pa_read_int(addr_of!((*pcp).high_min));
    let high_max = rust_pa_read_int(addr_of!((*pcp).high_max));
    let mut high = min(max((*pcp).high, low), high_max);
    (*pcp).high = high;
    if high < base {
        return 1;
    }
    let mut batch = if order != 0 {
        base
    } else {
        base << (*pcp).alloc_factor
    };
    if low != high_max && !rust_pa_test_bit(ZONE_BELOW_HIGH as ULong, addr_of!((*z).flags)) {
        high = min(high + batch, high_max);
        (*pcp).high = high;
    }
    if order == 0 {
        let max_alloc = max(high - (*pcp).count - base, base);
        if batch <= max_alloc && (*pcp).alloc_factor < RUST_PA_CONFIG_PCP_BATCH_SCALE_MAX as _ {
            (*pcp).alloc_factor += 1;
        }
        batch = min(batch, max_alloc);
    }
    if batch > 1 {
        batch = max(batch >> order, 2);
    }
    batch
}
unsafe fn __rmqueue_pcplist(
    z: *mut zone,
    order: u32,
    mt: i32,
    flags: u32,
    pcp: *mut per_cpu_pages,
    list: *mut list_head,
) -> *mut page {
    loop {
        if list_empty(list) {
            let batch = nr_pcp_alloc(pcp, z, order as i32);
            if flags & RUST_PA_ALLOC_HIGHATOMIC as u32 != 0 {
                return null_mut();
            }
            let allocated = rmqueue_bulk(z, order, batch as ULong, list, mt, flags);
            (*pcp).count += allocated << order;
            if list_empty(list) {
                return null_mut();
            }
        }
        let p = rust_pa_page_from_pcp((*list).next);
        list_del(page_pcp_list(p));
        (*pcp).count -= 1 << order;
        if !check_new_pages(p, order) {
            return p;
        }
    }
}
unsafe fn rmqueue_pcplist(
    preferred: *mut zone,
    z: *mut zone,
    order: u32,
    mt: i32,
    flags: u32,
) -> *mut page {
    let pcp = pcp_spin_trylock((*z).per_cpu_pageset);
    if pcp.is_null() {
        return null_mut();
    }
    (*pcp).free_count >>= 1;
    let list = addr_of_mut!((*pcp).lists[order_to_pindex(mt, order as i32) as usize]);
    let p = __rmqueue_pcplist(z, order, mt, flags, pcp, list);
    pcp_spin_unlock(pcp);
    if !p.is_null() {
        rust_pa_count_zid_vm_events(
            RUST_PA_PGALLOC,
            rust_pa_zone_idx(rust_pa_page_zone(p)),
            1 << order,
        );
        zone_statistics(preferred, z, 1);
    }
    p
}
// The original __no_sanitize_memory exclusion must be retained by owner policy.
#[cfg_attr(CONFIG_KMSAN, no_sanitize(memory))]
unsafe fn rmqueue(
    preferred: *mut zone,
    z: *mut zone,
    order: u32,
    gfp: gfp_t,
    flags: u32,
    mt: i32,
) -> *mut page {
    let mut p = null_mut();
    if pcp_allowed_order(order) {
        p = rmqueue_pcplist(preferred, z, order, mt, flags);
    }
    if p.is_null() {
        p = rmqueue_buddy(preferred, z, order, flags, mt);
    }
    if flags & RUST_PA_ALLOC_KSWAPD as u32 != 0
        && rust_pa_test_bit(ZONE_BOOSTED_WATERMARK as ULong, addr_of!((*z).flags))
    {
        rust_pa_clear_bit(ZONE_BOOSTED_WATERMARK as ULong, addr_of_mut!((*z).flags));
        wakeup_kswapd(z, 0, 0, rust_pa_zone_idx(z) as zone_type);
    }
    pa_vm_bug_page!(!p.is_null() && bad_range(z, p), p);
    p
}
unsafe fn reserve_highatomic_pageblock(p: *mut page, order: i32, z: *mut zone) {
    let managed = rust_pa_zone_managed_pages(z) / 100;
    if managed < pb_pages() {
        return;
    }
    let max_managed = managed.wrapping_add(pb_pages() - 1) & !(pb_pages() - 1);
    if (*z).nr_reserved_highatomic >= max_managed {
        return;
    }
    let flags = rust_pa_spin_lock_irqsave(addr_of_mut!((*z).lock));
    if (*z).nr_reserved_highatomic < max_managed {
        let mt = get_pfnblock_migratetype(p, rust_pa_page_to_pfn(p)) as i32;
        if migratetype_is_mergeable(mt) {
            if order < pb_order() as i32 {
                if move_freepages_block(z, p, mt, MIGRATE_HIGHATOMIC as i32) != -1 {
                    (*z).nr_reserved_highatomic += pb_pages();
                }
            } else {
                change_pageblock_range(p, order, MIGRATE_HIGHATOMIC as i32);
                (*z).nr_reserved_highatomic += 1 << order;
            }
        }
    }
    rust_pa_spin_unlock_irqrestore(addr_of_mut!((*z).lock), flags);
}
unsafe fn unreserve_highatomic_pageblock(ac: *const alloc_context, force: bool) -> bool {
    let mut zr =
        rust_pa_first_zones_zonelist((*ac).zonelist, (*ac).highest_zoneidx as i32, (*ac).nodemask);
    let mut advance = false;
    loop {
        if advance {
            zr = rust_pa_next_zones_zonelist(
                zr.add(1),
                (*ac).highest_zoneidx as i32,
                (*ac).nodemask,
            );
        }
        advance = true;
        let z = (*zr).zone;
        if z.is_null() {
            break;
        }
        if !force && (*z).nr_reserved_highatomic <= pb_pages() {
            continue;
        }
        let flags = rust_pa_spin_lock_irqsave(addr_of_mut!((*z).lock));
        let mut found = false;
        for order in 0..RUST_PA_NR_PAGE_ORDERS as u32 {
            let p = get_page_from_free_area(
                addr_of_mut!((*z).free_area[order as usize]),
                MIGRATE_HIGHATOMIC as i32,
            );
            if p.is_null() {
                continue;
            }
            let mut size = max(pb_pages(), 1 << order);
            if rust_pa_warn_highatomic_underflow(size > (*z).nr_reserved_highatomic) {
                size = (*z).nr_reserved_highatomic;
            }
            (*z).nr_reserved_highatomic -= size;
            let ret = if order < pb_order() {
                move_freepages_block(z, p, MIGRATE_HIGHATOMIC as i32, (*ac).migratetype)
            } else {
                move_to_free_list(p, z, order, MIGRATE_HIGHATOMIC as i32, (*ac).migratetype);
                change_pageblock_range(p, order as i32, (*ac).migratetype);
                1
            };
            rust_pa_warn_highatomic_boundary(ret == -1);
            if ret > 0 {
                found = true;
                break;
            }
        }
        rust_pa_spin_unlock_irqrestore(addr_of_mut!((*z).lock), flags);
        if found {
            return true;
        }
    }
    false
}
unsafe fn __zone_watermark_unusable_free(z: *mut zone, order: u32, flags: u32) -> Long {
    let mut unusable = ((1i32 << order) - 1) as Long;
    if flags & RUST_PA_ALLOC_RESERVES as u32 == 0 {
        unusable =
            unusable.wrapping_add(rust_pa_read_ulong(addr_of!((*z).nr_free_highatomic)) as Long);
    }
    #[cfg(CONFIG_CMA)]
    if flags & RUST_PA_ALLOC_CMA as u32 == 0 {
        unusable = unusable.wrapping_add(rust_pa_zone_page_state(z, NR_FREE_CMA_PAGES) as Long);
    }
    unusable
}
#[no_mangle]
pub unsafe extern "C" fn __zone_watermark_ok(
    z: *mut zone,
    order: u32,
    mark: ULong,
    highest: i32,
    flags: u32,
    mut free: Long,
) -> bool {
    let mut minimum = mark as Long;
    free = free.wrapping_sub(__zone_watermark_unusable_free(z, order, flags));
    if flags & RUST_PA_ALLOC_RESERVES as u32 != 0 {
        if flags & RUST_PA_ALLOC_MIN_RESERVE as u32 != 0 {
            minimum -= minimum / 2;
            if flags & RUST_PA_ALLOC_NON_BLOCK as u32 != 0 {
                minimum -= minimum / 4;
            }
        }
        if flags & RUST_PA_ALLOC_OOM as u32 != 0 {
            minimum -= minimum / 2;
        }
    }
    // lowmem_reserve[] is signed long in native mmzone.h.
    if free <= minimum.wrapping_add((*z).lowmem_reserve[highest as usize] as Long) {
        return false;
    }
    if order == 0 {
        return true;
    }
    for o in order..RUST_PA_NR_PAGE_ORDERS as u32 {
        let area = addr_of_mut!((*z).free_area[o as usize]);
        if (*area).nr_free == 0 {
            continue;
        }
        for mt in 0..MIGRATE_PCPTYPES as usize {
            if !list_empty(addr_of!((*area).free_list[mt])) {
                return true;
            }
        }
        #[cfg(CONFIG_CMA)]
        if flags & RUST_PA_ALLOC_CMA as u32 != 0
            && !list_empty(addr_of!((*area).free_list[MIGRATE_CMA as usize]))
        {
            return true;
        }
        if flags & (RUST_PA_ALLOC_HIGHATOMIC | RUST_PA_ALLOC_OOM) as u32 != 0
            && !list_empty(addr_of!((*area).free_list[MIGRATE_HIGHATOMIC as usize]))
        {
            return true;
        }
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn zone_watermark_ok(
    z: *mut zone,
    order: u32,
    mark: ULong,
    highest: i32,
    flags: u32,
) -> bool {
    __zone_watermark_ok(
        z,
        order,
        mark,
        highest,
        flags,
        rust_pa_zone_page_state(z, NR_FREE_PAGES) as Long,
    )
}
unsafe fn zone_watermark_fast(
    z: *mut zone,
    order: u32,
    mark: ULong,
    highest: i32,
    flags: u32,
    gfp: gfp_t,
) -> bool {
    let free = rust_pa_zone_page_state(z, NR_FREE_PAGES) as Long;
    if order == 0 {
        let reserved = __zone_watermark_unusable_free(z, 0, flags);
        let usable = free.wrapping_sub(min(free, reserved));
        // C promotes the signed usable count to unsigned when compared to mark.
        if usable as ULong > mark.wrapping_add((*z).lowmem_reserve[highest as usize] as ULong) {
            return true;
        }
    }
    if __zone_watermark_ok(z, order, mark, highest, flags, free) {
        return true;
    }
    if order == 0
        && flags & RUST_PA_ALLOC_MIN_RESERVE as u32 != 0
        && (*z).watermark_boost != 0
        && flags & RUST_PA_ALLOC_WMARK_MASK as u32 == WMARK_MIN as u32
    {
        return __zone_watermark_ok(
            z,
            order,
            (*z)._watermark[WMARK_MIN as usize],
            highest,
            flags,
            free,
        );
    }
    false
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut node_reclaim_distance: i32 = RUST_PA_RECLAIM_DISTANCE as i32;
unsafe fn zone_allows_reclaim(local: *mut zone, z: *mut zone) -> bool {
    #[cfg(CONFIG_NUMA)]
    {
        rust_pa_node_distance(rust_pa_zone_to_nid(local), rust_pa_zone_to_nid(z))
            <= node_reclaim_distance
    }
    #[cfg(not(CONFIG_NUMA))]
    {
        true
    }
}
unsafe fn alloc_flags_nofragment(z: *mut zone, gfp: gfp_t) -> u32 {
    let mut flags = if gfp & RUST_PA___GFP_KSWAPD_RECLAIM as gfp_t != 0 {
        RUST_PA_ALLOC_KSWAPD as u32
    } else {
        0
    };
    if defrag_mode != 0 {
        return flags | RUST_PA_ALLOC_NOFRAGMENT as u32;
    }
    #[cfg(CONFIG_ZONE_DMA32)]
    {
        if z.is_null() || rust_pa_zone_idx(z) != ZONE_NORMAL as i32 {
            return flags;
        }
        if rust_pa_online_nodes() > 1 && (*z.sub(1)).present_pages == 0 {
            return flags;
        }
        flags |= RUST_PA_ALLOC_NOFRAGMENT as u32;
    }
    flags
}
unsafe fn alloc_flags_cma(gfp: gfp_t) -> u32 {
    #[cfg(CONFIG_CMA)]
    if rust_pa_gfp_migratetype(gfp) == MIGRATE_MOVABLE as i32 {
        return RUST_PA_ALLOC_CMA as u32;
    }
    RUST_PA_ALLOC_DEFAULT as u32
}
#[inline]
unsafe fn can_spin_trylock() -> bool {
    rust_pa_can_spin_trylock()
}
#[inline]
unsafe fn wmark_pages(z: *mut zone, mark: u32) -> ULong {
    (*z)._watermark[mark as usize].wrapping_add((*z).watermark_boost)
}
unsafe fn get_page_from_freelist(
    gfp: gfp_t,
    order: u32,
    mut flags: i32,
    ac: *const alloc_context,
) -> *mut page {
    let mut last_pgdat = null_mut();
    let mut last_dirty_ok = false;
    let mut skip_kswapd = rust_pa_online_nodes() > 1;
    let mut skipped = false;
    'retry: loop {
        let no_fallback = flags & RUST_PA_ALLOC_NOFRAGMENT as i32 != 0;
        let preferred = (*(*ac).preferred_zoneref).zone;
        let mut zr = (*ac).preferred_zoneref;
        let mut advance = false;
        loop {
            if advance {
                zr = rust_pa_next_zones_zonelist(
                    zr.add(1),
                    (*ac).highest_zoneidx as i32,
                    (*ac).nodemask,
                );
            }
            advance = true;
            let z = (*zr).zone;
            if z.is_null() {
                break;
            }
            if rust_pa_cpusets_enabled()
                && flags & RUST_PA_ALLOC_CPUSET as i32 != 0
                && !rust_pa_cpuset_zone_allowed(z, gfp)
            {
                continue;
            }
            if (*ac).spread_dirty_pages {
                if last_pgdat != (*z).zone_pgdat {
                    last_pgdat = (*z).zone_pgdat;
                    last_dirty_ok = node_dirty_ok(last_pgdat);
                }
                if !last_dirty_ok {
                    continue;
                }
            }
            if no_fallback
                && defrag_mode == 0
                && rust_pa_online_nodes() > 1
                && z != preferred
                && rust_pa_zone_to_nid(z) != rust_pa_zone_to_nid(preferred)
            {
                flags &= !(RUST_PA_ALLOC_NOFRAGMENT as i32);
                continue 'retry;
            }
            if skip_kswapd
                && !rust_pa_waitqueue_active(addr_of_mut!((*(*z).zone_pgdat).kswapd_wait))
            {
                skipped = true;
                continue;
            }
            cond_accept_memory(z, order, flags);
            let mut try_zone = false;
            if !rust_pa_test_bit(ZONE_BELOW_HIGH as ULong, addr_of!((*z).flags)) {
                if zone_watermark_fast(
                    z,
                    order,
                    wmark_pages(z, WMARK_HIGH as u32),
                    (*ac).highest_zoneidx as i32,
                    flags as u32,
                    gfp,
                ) {
                    try_zone = true;
                } else {
                    rust_pa_set_bit(ZONE_BELOW_HIGH as ULong, addr_of_mut!((*z).flags));
                }
            }
            if !try_zone {
                let mark = wmark_pages(z, (flags as u32) & RUST_PA_ALLOC_WMARK_MASK as u32);
                if !zone_watermark_fast(
                    z,
                    order,
                    mark,
                    (*ac).highest_zoneidx as i32,
                    flags as u32,
                    gfp,
                ) {
                    if cond_accept_memory(z, order, flags)
                        || (deferred_pages_enabled() && _deferred_grow_zone(z, order))
                        || flags & RUST_PA_ALLOC_NO_WATERMARKS as i32 != 0
                    {
                        try_zone = true;
                    } else {
                        if !rust_pa_node_reclaim_enabled() || !zone_allows_reclaim(preferred, z) {
                            continue;
                        }
                        if node_reclaim((*z).zone_pgdat, gfp, order) == 0 {
                            continue;
                        }
                        if !zone_watermark_ok(
                            z,
                            order,
                            mark,
                            (*ac).highest_zoneidx as i32,
                            flags as u32,
                        ) {
                            continue;
                        }
                    }
                }
            }
            loop {
                let p = rmqueue(preferred, z, order, gfp, flags as u32, (*ac).migratetype);
                if !p.is_null() {
                    prep_new_page(p, order, gfp, flags as u32);
                    return p;
                }
                if cond_accept_memory(z, order, flags)
                    || (deferred_pages_enabled() && _deferred_grow_zone(z, order))
                {
                    continue;
                }
                break;
            }
        }
        if skip_kswapd && skipped {
            skip_kswapd = false;
            continue;
        }
        if no_fallback && defrag_mode == 0 {
            flags &= !(RUST_PA_ALLOC_NOFRAGMENT as i32);
            continue;
        }
        return null_mut();
    }
}
