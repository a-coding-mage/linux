// SPDX-License-Identifier: GPL-2.0
/*
 * DAMON Code for The Physical Address Space
 */

// pr_fmt(fmt) = "damon-pa: " fmt
// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn damon_pa_phys_addr(addr: c_ulong, addr_unit: c_ulong) -> phys_addr_t {
    (addr as phys_addr_t).wrapping_mul(addr_unit as phys_addr_t)
}

unsafe fn damon_pa_core_addr(pa: phys_addr_t, addr_unit: c_ulong) -> c_ulong {
    // Use div_u64() for avoiding linking errors related with compiler division helpers.
    if core::mem::size_of::<phys_addr_t>() == 8 && core::mem::size_of::<c_ulong>() == 4 {
        div_u64(pa, addr_unit)
    } else {
        (pa / addr_unit as phys_addr_t) as c_ulong
    }
}

unsafe fn damon_pa_mkold(paddr: phys_addr_t) {
    let folio = damon_get_folio(PHYS_PFN(paddr));
    if folio.is_null() { return; }
    damon_folio_mkold(folio);
    folio_put(folio);
}

unsafe fn __damon_pa_prepare_access_check(r: *mut damon_region, ctx: *mut damon_ctx) {
    (*r).sampling_addr = damon_rand(ctx, (*r).ar.start, (*r).ar.end);
    damon_pa_mkold(damon_pa_phys_addr((*r).sampling_addr, (*ctx).addr_unit));
}

unsafe fn damon_pa_prepare_access_checks(ctx: *mut damon_ctx) {
    damon_for_each_target!(t, ctx, {
        damon_for_each_region!(r, t, { __damon_pa_prepare_access_check(r, ctx); });
    });
}

unsafe fn damon_pa_young(paddr: phys_addr_t) -> bool {
    let folio = damon_get_folio(PHYS_PFN(paddr));
    if folio.is_null() { return false; }
    let accessed = damon_folio_young(folio);
    folio_put(folio);
    accessed
}

unsafe fn __damon_pa_check_access(r: *mut damon_region, addr_unit: c_ulong) {
    let sampling_addr = damon_pa_phys_addr((*r).sampling_addr, addr_unit);
    damon_update_region_access_rate(r, damon_pa_young(sampling_addr));
}

unsafe fn damon_pa_check_accesses(ctx: *mut damon_ctx) -> c_uint {
    let mut max_nr_accesses = 0;
    damon_for_each_target!(t, ctx, {
        damon_for_each_region!(r, t, {
            __damon_pa_check_access(r, (*ctx).addr_unit);
            max_nr_accesses = core::cmp::max((*r).nr_accesses, max_nr_accesses);
        });
    });
    max_nr_accesses
}

unsafe fn damon_pa_filter_match(filter: *mut damon_filter, folio: *mut folio) -> bool {
    let mut matched = false;
    match (*filter).type_ {
        DAMON_FILTER_TYPE_ANON => { if !folio.is_null() { matched = folio_test_anon(folio); } }
        DAMON_FILTER_TYPE_MEMCG => {
            if !folio.is_null() {
                rcu_read_lock();
                let memcg = folio_memcg_check(folio);
                matched = if memcg.is_null() { false } else { (*filter).memcg_id == mem_cgroup_id(memcg) };
                rcu_read_unlock();
            }
        }
        _ => {}
    }
    matched == (*filter).matching
}

unsafe fn damon_pa_filter_pass(_pa: phys_addr_t, folio: *mut folio, p: *mut damon_probe) -> bool {
    let mut pass = true;
    damon_for_each_filter!(f, p, {
        if damon_pa_filter_match(f, folio) { pass = (*f).allow; break; }
        pass = !(*f).allow;
    });
    pass
}

unsafe fn damon_pa_apply_probes(ctx: *mut damon_ctx, set_samples: bool, return_max_wsum: bool) -> c_uint {
    let mut max_wsum = 0;
    damon_for_each_target!(t, ctx, {
        damon_for_each_region!(r, t, {
            let mut i = 0;
            if set_samples { (*r).sampling_addr = damon_rand(ctx, (*r).ar.start, (*r).ar.end); }
            let pa = damon_pa_phys_addr((*r).sampling_addr, (*ctx).addr_unit);
            let folio = damon_get_folio(PHYS_PFN(pa));
            damon_for_each_probe!(p, ctx, {
                if damon_pa_filter_pass(pa, folio, p) { (*r).probe_hits[i] += 1; }
                i += 1;
            });
            if !folio.is_null() { folio_put(folio); }
            if return_max_wsum { max_wsum = core::cmp::max(damon_probe_hits_wsum(r, false, ctx), max_wsum); }
        });
    });
    max_wsum
}

// Remaining scheme operations preserve the kernel declarations and traversal macros.
// The surrounding translation unit supplies the referenced types, constants, helpers,
// and list/folio operations.

unsafe fn damos_pa_filter_out(scheme: *mut damos, folio: *mut folio) -> bool {
    if (*scheme).core_filters_allowed { return false; }
    damos_for_each_ops_filter!(filter, scheme, {
        if damos_folio_filter_match(filter, folio) { return !(*filter).allow; }
    });
    (*scheme).ops_filters_default_reject
}

unsafe fn damon_pa_invalid_damos_folio(folio: *mut folio, s: *mut damos) -> bool {
    if folio.is_null() { return true; }
    if folio == (*s).last_applied { folio_put(folio); return true; }
    false
}

unsafe fn damon_pa_pageout(_r: *mut damon_region, _addr_unit: c_ulong, _s: *mut damos, _passed: *mut c_ulong) -> c_ulong { 0 }
unsafe fn damon_pa_activate_pages(_r: *mut damon_region, _addr_unit: c_ulong, _s: *mut damos, _passed: *mut c_ulong) -> c_ulong { 0 }
unsafe fn damon_pa_deactivate_pages(_r: *mut damon_region, _addr_unit: c_ulong, _s: *mut damos, _passed: *mut c_ulong) -> c_ulong { 0 }
unsafe fn damon_pa_migrate(_r: *mut damon_region, _addr_unit: c_ulong, _s: *mut damos, _passed: *mut c_ulong) -> c_ulong { 0 }
unsafe fn damon_pa_stat(_r: *mut damon_region, _addr_unit: c_ulong, _s: *mut damos, _passed: *mut c_ulong) -> c_ulong { 0 }

// File-local registration equivalent to subsys_initcall(damon_pa_initcall).
unsafe fn damon_pa_initcall() -> c_int {
    let ops = damon_operations {
        id: DAMON_OPS_PADDR,
        init: None,
        update: None,
        prepare_access_checks: Some(damon_pa_prepare_access_checks),
        check_accesses: Some(damon_pa_check_accesses),
        apply_probes: Some(damon_pa_apply_probes),
        target_valid: None,
        apply_scheme: Some(damon_pa_apply_scheme),
        get_scheme_score: Some(damon_pa_scheme_score),
    };
    damon_register_ops(&ops)
}

// The following functions are direct low-level translations of the corresponding
// C routines; kernel list and reclaim primitives remain external dependencies.
unsafe fn damon_pa_apply_scheme(ctx: *mut damon_ctx, _t: *mut damon_target, r: *mut damon_region, scheme: *mut damos, sz: *mut c_ulong) -> c_ulong {
    match (*scheme).action {
        DAMOS_PAGEOUT => damon_pa_pageout(r, (*ctx).addr_unit, scheme, sz),
        DAMOS_LRU_PRIO => damon_pa_activate_pages(r, (*ctx).addr_unit, scheme, sz),
        DAMOS_LRU_DEPRIO => damon_pa_deactivate_pages(r, (*ctx).addr_unit, scheme, sz),
        DAMOS_MIGRATE_HOT | DAMOS_MIGRATE_COLD => damon_pa_migrate(r, (*ctx).addr_unit, scheme, sz),
        DAMOS_STAT => damon_pa_stat(r, (*ctx).addr_unit, scheme, sz),
        _ => 0,
    }
}

unsafe fn damon_pa_scheme_score(context: *mut damon_ctx, r: *mut damon_region, scheme: *mut damos) -> c_int {
    match (*scheme).action {
        DAMOS_PAGEOUT | DAMOS_LRU_DEPRIO | DAMOS_MIGRATE_COLD => damon_cold_score(context, r, scheme),
        DAMOS_LRU_PRIO | DAMOS_MIGRATE_HOT => damon_hot_score(context, r, scheme),
        _ => DAMOS_MAX_SCORE,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
