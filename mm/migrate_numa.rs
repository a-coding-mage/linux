// SPDX-License-Identifier: GPL-2.0
// Included only under CONFIG_NUMA_BALANCING.
unsafe fn migrate_balanced_pgdat(pgdat: *mut pglist_data, nr_migrate_pages: c_ulong) -> bool {
    let mut z = pgdat_nr_zones(pgdat).wrapping_sub(1);
    while z >= 0 {
        let zone = pgdat_zone(pgdat, z);
        if managed_zone(zone)
            && zone_watermark_ok(
                zone,
                0,
                high_wmark_pages(zone).wrapping_add(nr_migrate_pages),
                ZONE_MOVABLE as _,
                RUST_MIGRATE_ALLOC_CMA as _,
            )
        {
            return true;
        }
        z = z.wrapping_sub(1);
    }
    false
}
unsafe extern "C" fn alloc_misplaced_dst_folio(src: *mut folio, data: c_ulong) -> *mut folio {
    let nid = data as c_int;
    let order = folio_order(src) as c_int;
    let mut gfp = RUST_MIGRATE___GFP_THISNODE as gfp_t;
    if order > 0 {
        gfp |= RUST_MIGRATE_GFP_TRANSHUGE_LIGHT as gfp_t;
    } else {
        gfp |= (RUST_MIGRATE_GFP_HIGHUSER_MOVABLE
            | RUST_MIGRATE___GFP_NOMEMALLOC
            | RUST_MIGRATE___GFP_NORETRY
            | RUST_MIGRATE___GFP_NOWARN) as gfp_t;
        gfp &= !(RUST_MIGRATE___GFP_RECLAIM as gfp_t);
    }
    __folio_alloc_node(gfp, order as _, nid)
}

#[no_mangle]
pub unsafe extern "C" fn migrate_misplaced_folio_prepare(
    folio: *mut folio,
    vma: *mut vm_area_struct,
    node: c_int,
) -> c_int {
    let nr_pages = folio_nr_pages(folio) as c_int;
    let pgdat = node_data(node);
    if folio_is_file_lru(folio) != 0 {
        if vma_flags(vma) & RUST_MIGRATE_VM_EXEC != 0 && folio_maybe_mapped_shared(folio) {
            return E_ACCES;
        }
        if folio_test_dirty(folio) {
            return E_AGAIN;
        }
    }
    if !migrate_balanced_pgdat(pgdat, nr_pages as c_ulong) {
        if numa_balancing_mode() & RUST_MIGRATE_NUMA_BALANCING_MEMORY_TIERING as c_int == 0 {
            return E_AGAIN;
        }
        let mut z = pgdat_nr_zones(pgdat).wrapping_sub(1);
        while z >= 0 {
            if managed_zone(pgdat_zone(pgdat, z)) {
                break;
            }
            z = z.wrapping_sub(1);
        }
        if z < 0 {
            return E_AGAIN;
        }
        wakeup_kswapd(
            pgdat_zone(pgdat, z),
            0,
            folio_order(folio) as _,
            ZONE_MOVABLE,
        );
        return E_AGAIN;
    }
    if !folio_isolate_lru(folio) {
        return E_AGAIN;
    }
    node_stat_mod_folio(folio, isolated_stat(folio), nr_pages as c_long);
    0
}

#[no_mangle]
pub unsafe extern "C" fn migrate_misplaced_folio(folio: *mut folio, node: c_int) -> c_int {
    let pgdat = node_data(node);
    let mut nr_succeeded: c_uint = 0;
    let mut migratepages: list_head = zeroed();
    init_list_head(&mut migratepages);
    let memcg = get_mem_cgroup_from_folio(folio);
    let lruvec = mem_cgroup_lruvec(memcg, pgdat);
    list_add(folio_lru(folio), &mut migratepages);
    let nr_remaining = migrate_pages(
        &mut migratepages,
        Some(alloc_misplaced_dst_folio),
        None,
        node as c_ulong,
        MIGRATE_ASYNC,
        MR_NUMA_MISPLACED,
        &mut nr_succeeded,
    );
    if nr_remaining != 0 && !list_empty(&migratepages) {
        putback_movable_pages(&mut migratepages);
    }
    if nr_succeeded != 0 {
        count_vm_numa_events(NUMA_PAGE_MIGRATE, nr_succeeded as _);
        count_memcg_events(memcg, NUMA_PAGE_MIGRATE, nr_succeeded as _);
        if numa_balancing_mode() & RUST_MIGRATE_NUMA_BALANCING_MEMORY_TIERING as c_int != 0
            && !node_is_toptier(folio_nid(folio))
            && node_is_toptier(node)
        {
            mod_lruvec_state(lruvec, PGPROMOTE_SUCCESS, nr_succeeded as c_long);
        }
    }
    mem_cgroup_put(memcg);
    bug_misplaced_list(!list_empty(&migratepages));
    if nr_remaining != 0 {
        E_AGAIN
    } else {
        0
    }
}
