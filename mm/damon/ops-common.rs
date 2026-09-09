// SPDX-License-Identifier: GPL-2.0
/* Common Code for Data Access Monitoring */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn damon_get_folio(pfn: c_ulong) -> *mut folio {
    let page: *mut page = pfn_to_online_page(pfn);
    if page.is_null() {
        return core::ptr::null_mut();
    }

    let mut folio: *mut folio = page_folio(page);
    if !folio_try_get(folio) {
        return core::ptr::null_mut();
    }
    if page_folio(page) != folio || !folio_test_lru(folio) {
        folio_put(folio);
        folio = core::ptr::null_mut();
    }
    folio
}

pub unsafe fn damon_ptep_mkold(pte: *mut pte_t, vma: *mut vm_area_struct, addr: c_ulong) {
    let pteval: pte_t = ptep_get(pte);
    let pfn: c_ulong = if pte_present(pteval) {
        pte_pfn(pteval)
    } else {
        softleaf_to_pfn(softleaf_from_pte(pteval))
    };
    let folio = damon_get_folio(pfn);
    if folio.is_null() { return; }

    let mut young = false;
    if pte_present(pteval) {
        young |= ptep_test_and_clear_young(vma, addr, pte);
    }
    young |= mmu_notifier_clear_young((*vma).vm_mm, addr, addr + PAGE_SIZE);
    if young { folio_set_young(folio); }
    folio_set_idle(folio);
    folio_put(folio);
}

pub unsafe fn damon_pmdp_mkold(pmd: *mut pmd_t, vma: *mut vm_area_struct, addr: c_ulong) {
    // CONFIG_TRANSPARENT_HUGEPAGE
    let pmdval: pmd_t = pmdp_get(pmd);
    let pfn: c_ulong = if pmd_present(pmdval) {
        pmd_pfn(pmdval)
    } else {
        softleaf_to_pfn(softleaf_from_pmd(pmdval))
    };
    let folio = damon_get_folio(pfn);
    if folio.is_null() { return; }

    let mut young = false;
    if pmd_present(pmdval) {
        young |= pmdp_test_and_clear_young(vma, addr, pmd);
    }
    young |= mmu_notifier_clear_young((*vma).vm_mm, addr, addr + HPAGE_PMD_SIZE);
    if young { folio_set_young(folio); }
    folio_set_idle(folio);
    folio_put(folio);
    // end CONFIG_TRANSPARENT_HUGEPAGE
}

pub const DAMON_MAX_SUBSCORE: c_int = 100;
pub const DAMON_MAX_AGE_IN_LOG: c_int = 32;

pub unsafe fn damon_hot_score(c: *mut damon_ctx, r: *mut damon_region, s: *mut damos) -> c_int {
    let freq_weight = (*s).quota.weight_nr_accesses;
    let age_weight = (*s).quota.weight_age;
    let freq_subscore = mult_frac(damon_nr_accesses_mvsum(r, c), DAMON_MAX_SUBSCORE, damon_nr_samples_per_aggr(&(*c).attrs));
    let age_in_sec: c_uint = ((*r).age as c_ulong * (*c).attrs.aggr_interval / 1_000_000) as c_uint;
    let mut age_in_log: c_int = if age_in_sec != 0 { core::cmp::min(ilog2(age_in_sec) + 1, DAMON_MAX_AGE_IN_LOG) } else { 0 };
    if freq_subscore == 0 { age_in_log *= -1; }
    age_in_log += DAMON_MAX_AGE_IN_LOG;
    let age_subscore = age_in_log * DAMON_MAX_SUBSCORE / DAMON_MAX_AGE_IN_LOG / 2;
    let mut hotness = (freq_weight * freq_subscore + age_weight * age_subscore) as c_int;
    if freq_weight + age_weight != 0 { hotness /= (freq_weight + age_weight) as c_int; }
    hotness = hotness * DAMOS_MAX_SCORE / DAMON_MAX_SUBSCORE;
    core::cmp::max(core::cmp::min(hotness, DAMOS_MAX_SCORE), 0)
}

pub unsafe fn damon_cold_score(c: *mut damon_ctx, r: *mut damon_region, s: *mut damos) -> c_int {
    DAMOS_MAX_SCORE - damon_hot_score(c, r, s)
}

unsafe fn damon_folio_mkold_one(folio: *mut folio, vma: *mut vm_area_struct, mut addr: c_ulong, _arg: *mut c_void) -> bool {
    let mut pvmw = folio_vma_walk::new(folio, vma, addr, 0);
    while page_vma_mapped_walk(&mut pvmw) {
        addr = pvmw.address;
        if !pvmw.pte.is_null() { damon_ptep_mkold(pvmw.pte, vma, addr); }
        else { damon_pmdp_mkold(pvmw.pmd, vma, addr); }
    }
    true
}

pub unsafe fn damon_folio_mkold(folio: *mut folio) {
    let mut rwc = rmap_walk_control { rmap_one: Some(damon_folio_mkold_one), anon_lock: Some(folio_lock_anon_vma_read), ..core::mem::zeroed() };
    if !folio_mapped(folio) || !folio_raw_mapping(folio) { folio_set_idle(folio); return; }
    if !folio_trylock(folio) { return; }
    rmap_walk(folio, &mut rwc);
    folio_unlock(folio);
}

unsafe fn damon_folio_young_one(folio: *mut folio, vma: *mut vm_area_struct, mut addr: c_ulong, arg: *mut c_void) -> bool {
    let accessed = arg as *mut bool;
    let mut pvmw = folio_vma_walk::new(folio, vma, addr, 0);
    *accessed = false;
    while page_vma_mapped_walk(&mut pvmw) {
        addr = pvmw.address;
        if !pvmw.pte.is_null() {
            let pte = ptep_get(pvmw.pte);
            *accessed = (pte_present(pte) && pte_young(pte)) || !folio_test_idle(folio) || mmu_notifier_test_young((*vma).vm_mm, addr);
        } else {
            // CONFIG_TRANSPARENT_HUGEPAGE
            let pmd = pmdp_get(pvmw.pmd);
            *accessed = (pmd_present(pmd) && pmd_young(pmd)) || !folio_test_idle(folio) || mmu_notifier_test_young((*vma).vm_mm, addr);
            // end CONFIG_TRANSPARENT_HUGEPAGE
        }
        if *accessed { page_vma_mapped_walk_done(&mut pvmw); break; }
    }
    *accessed
}

pub unsafe fn damon_folio_young(folio: *mut folio) -> bool {
    let mut accessed = false;
    let mut rwc = rmap_walk_control { arg: &mut accessed as *mut bool as *mut c_void, rmap_one: Some(damon_folio_young_one), anon_lock: Some(folio_lock_anon_vma_read), ..core::mem::zeroed() };
    if !folio_mapped(folio) || !folio_raw_mapping(folio) { return !folio_test_idle(folio); }
    if !folio_trylock(folio) { return false; }
    rmap_walk(folio, &mut rwc);
    folio_unlock(folio);
    accessed
}

pub unsafe fn damos_folio_filter_match(filter: *mut damos_filter, folio: *mut folio) -> bool {
    let matched = match (*filter).type_ {
        DAMOS_FILTER_TYPE_ANON => folio_test_anon(folio),
        DAMOS_FILTER_TYPE_ACTIVE => folio_test_active(folio),
        DAMOS_FILTER_TYPE_MEMCG => { rcu_read_lock(); let memcg = folio_memcg_check(folio); let m = !memcg.is_null() && (*filter).memcg_id == mem_cgroup_id(memcg); rcu_read_unlock(); m },
        DAMOS_FILTER_TYPE_YOUNG => { let m = damon_folio_young(folio); if m { damon_folio_mkold(folio); } m },
        DAMOS_FILTER_TYPE_HUGEPAGE_SIZE => { let sz = folio_size(folio); (*filter).sz_range.min <= sz && sz <= (*filter).sz_range.max },
        DAMOS_FILTER_TYPE_UNMAPPED => !folio_mapped(folio) || !folio_raw_mapping(folio),
        _ => false,
    };
    matched == (*filter).matching
}

unsafe fn __damon_migrate_folio_list(migrate_folios: *mut list_head, pgdat: *mut pglist_data, target_nid: c_int) -> c_uint {
    let mut nr_succeeded = 0;
    let mut mtc: migration_target_control = core::mem::zeroed();
    mtc.gfp_mask = (GFP_HIGHUSER_MOVABLE & !__GFP_RECLAIM) | __GFP_NOMEMALLOC | GFP_NOWAIT | __GFP_THISNODE;
    mtc.nid = target_nid;
    if (*pgdat).node_id == target_nid || target_nid == NUMA_NO_NODE || list_empty(migrate_folios) { return 0; }
    migrate_pages(migrate_folios, alloc_migration_target, core::ptr::null_mut(), &mut mtc as *mut _ as c_ulong, MIGRATE_ASYNC, MR_DAMON, &mut nr_succeeded);
    nr_succeeded
}

unsafe fn damon_migrate_folio_list(folio_list: *mut list_head, pgdat: *mut pglist_data, target_nid: c_int) -> c_uint {
    let mut nr_migrated = 0;
    let mut ret_folios: list_head = LIST_HEAD_INIT();
    let mut migrate_folios: list_head = LIST_HEAD_INIT();
    while !list_empty(folio_list) {
        cond_resched();
        let folio = lru_to_folio(folio_list); list_del(&mut (*folio).lru);
        if !folio_trylock(folio) { list_add(&mut (*folio).lru, &mut ret_folios); continue; }
        list_add(&mut (*folio).lru, &mut migrate_folios); folio_unlock(folio);
    }
    nr_migrated += __damon_migrate_folio_list(&mut migrate_folios, pgdat, target_nid);
    if !list_empty(&mut migrate_folios) { list_splice_init(&mut migrate_folios, folio_list); }
    try_to_unmap_flush(); list_splice(&mut ret_folios, folio_list);
    while !list_empty(folio_list) { let folio = lru_to_folio(folio_list); list_del(&mut (*folio).lru); node_stat_sub_folio(folio, NR_ISOLATED_ANON + folio_is_file_lru(folio)); folio_putback_lru(folio); }
    nr_migrated
}

pub unsafe fn damon_migrate_pages(folio_list: *mut list_head, target_nid: c_int) -> c_ulong {
    let mut nr_migrated = 0;
    if list_empty(folio_list) { return 0; }
    if target_nid < 0 || target_nid >= MAX_NUMNODES || !node_state(target_nid, N_MEMORY) {
        while !list_empty(folio_list) { let folio = lru_to_folio(folio_list); list_del(&mut (*folio).lru); node_stat_sub_folio(folio, NR_ISOLATED_ANON + folio_is_file_lru(folio)); folio_putback_lru(folio); }
        return 0;
    }
    let noreclaim_flag = memalloc_noreclaim_save();
    let mut node_folio_list: list_head = LIST_HEAD_INIT();
    let mut nid = folio_nid(lru_to_folio(folio_list));
    while !list_empty(folio_list) {
        let folio = lru_to_folio(folio_list);
        if nid == folio_nid(folio) { list_move(&mut (*folio).lru, &mut node_folio_list); continue; }
        nr_migrated += damon_migrate_folio_list(&mut node_folio_list, NODE_DATA(nid), target_nid) as c_ulong;
        nid = folio_nid(lru_to_folio(folio_list));
    }
    nr_migrated += damon_migrate_folio_list(&mut node_folio_list, NODE_DATA(nid), target_nid) as c_ulong;
    memalloc_noreclaim_restore(noreclaim_flag);
    nr_migrated
}

pub unsafe fn damos_ops_has_filter(s: *mut damos) -> bool {
    let mut f: *mut damos_filter = core::ptr::null_mut();
    // damos_for_each_ops_filter(f, s)
    while !f.is_null() { return true; }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
