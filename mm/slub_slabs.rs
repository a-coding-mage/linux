// SPDX-License-Identifier: GPL-2.0
// Original mm/slub.c slab lifecycle and partial-list algorithms.
#[inline]
unsafe fn alloc_slab_page(
    flags: gfp_t,
    node: i32,
    oo: kmem_cache_order_objects,
    allow_spin: bool,
) -> *mut slab {
    let order = oo_order(oo);
    let page = if !allow_spin {
        rust_slub_alloc_frozen_pages_nolock(0, node, order)
    } else if node == RSL_NUMA_NO_NODE as i32 {
        rust_slub_alloc_frozen_pages(flags, order)
    } else {
        rust_slub_alloc_frozen_pages_node_profiled(flags, order, node)
    };
    if page.is_null() {
        return null_mut();
    }
    rust_slub_set_page_slab(page);
    let slab = rust_slub_page_slab(page);
    if rust_slub_page_is_pfmemalloc(page) {
        slab_set_pfmemalloc(slab);
    }
    slab
}
unsafe fn init_cache_random_seq(s: *mut kmem_cache) -> i32 {
    #[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
    {
        let count = oo_objects((*s).oo);
        if !(*s).random_seq.is_null() {
            return 0;
        }
        let err = cache_random_seq_create(s, count, RSL_GFP_KERNEL);
        if err != 0 {
            rust_slub_log_random_seq_failure(s);
            return err;
        }
        if !(*s).random_seq.is_null() {
            for i in 0..count as usize {
                *(*s).random_seq.add(i) *= (*s).size;
            }
        }
    }
    0
}
#[cfg_attr(CONFIG_SLAB_FREELIST_RANDOM, link_section = ".init.text")]
unsafe fn init_freelist_randomization() {
    #[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
    {
        rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
        let mut l = slab_caches.next;
        while l != addr_of_mut!(slab_caches) {
            let s = l
                .byte_sub(offset_of!(kmem_cache, list))
                .cast::<kmem_cache>();
            init_cache_random_seq(s);
            l = (*l).next;
        }
        rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    }
}
#[inline(always)]
unsafe fn account_slab(
    slab: *mut slab,
    order: i32,
    s: *mut kmem_cache,
    gfp: gfp_t,
    alloc_flags: u32,
) {
    if rust_slub_memcg_kmem_online()
        && (*s).flags & RSL_SLAB_ACCOUNT != 0
        && rust_slub_slab_obj_exts(slab) == 0
    {
        alloc_slab_obj_exts(slab, s, gfp, alloc_flags | RSL_SLAB_ALLOC_NEW_SLAB);
    }
    rust_slub_mod_node_page_state(
        rust_slub_slab_pgdat(slab),
        rust_slub_cache_vmstat_idx(s),
        (RSL_PAGE_SIZE as Long) << order,
    );
}
#[inline(always)]
unsafe fn unaccount_slab(slab: *mut slab, order: i32, s: *mut kmem_cache, allow_spin: bool) {
    free_slab_obj_exts(slab, allow_spin);
    rust_slub_mod_node_page_state(
        rust_slub_slab_pgdat(slab),
        rust_slub_cache_vmstat_idx(s),
        -((RSL_PAGE_SIZE as Long) << order),
    );
}
unsafe fn allocate_slab(
    s: *mut kmem_cache,
    mut flags: gfp_t,
    alloc_flags: u32,
    node: i32,
) -> *mut slab {
    let allow_spin = alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
    let mut oo = (*s).oo;
    flags &= gfp_allowed_mask;
    flags |= (*s).allocflags;
    let mut alloc_gfp = (flags | RSL___GFP_NOWARN | RSL___GFP_NORETRY) & !RSL___GFP_NOFAIL;
    if alloc_gfp & RSL___GFP_DIRECT_RECLAIM != 0 && oo_order(oo) > oo_order((*s).min) {
        alloc_gfp = (alloc_gfp | RSL___GFP_NOMEMALLOC) & !RSL___GFP_RECLAIM;
    }
    let mut slab = alloc_slab_page(alloc_gfp, node, oo, allow_spin);
    if slab.is_null() {
        oo = (*s).min;
        slab = alloc_slab_page(flags, node, oo, allow_spin);
        if slab.is_null() {
            return null_mut();
        }
        stat(s, ORDER_FALLBACK);
    }
    rust_slub_slab_set_counters(slab, 0);
    rust_slub_slab_set_objects(slab, oo_objects(oo));
    #[cfg(CONFIG_64BIT)]
    if rust_slub_cache_needs_objcg(s) {
        rust_slub_slab_set_needs_objcg(slab, true);
    }
    (*slab).slab_cache = s;
    rust_slub_kasan_poison_slab(slab);
    let start = rust_slub_slab_address(slab);
    setup_slab_debug(s, slab, start);
    init_slab_obj_exts(slab);
    alloc_slab_obj_exts_early(s, slab);
    account_slab(slab, oo_order(oo) as i32, s, flags, alloc_flags);
    slab
}
unsafe fn new_slab(s: *mut kmem_cache, mut flags: gfp_t, alloc_flags: u32, node: i32) -> *mut slab {
    if flags & RSL_GFP_SLAB_BUG_MASK != 0 {
        flags = kmalloc_fix_flags(flags);
    }
    rust_slub_warn_ctor_zero((*s).ctor.is_some() && flags & RSL___GFP_ZERO != 0);
    flags &= RSL_GFP_RECLAIM_MASK | RSL_GFP_CONSTRAINT_MASK;
    allocate_slab(s, flags, alloc_flags, node)
}
unsafe fn __free_slab(s: *mut kmem_cache, slab: *mut slab, allow_spin: bool) {
    let page = rust_slub_slab_page(slab);
    let order = rust_slub_compound_order(page);
    __slab_clear_pfmemalloc(slab);
    rust_slub_page_clear_mapping(page);
    rust_slub_clear_page_slab(page);
    rust_slub_mm_account_reclaimed_pages(1 << order);
    unaccount_slab(slab, order as i32, s, allow_spin);
    if allow_spin {
        free_frozen_pages(page, order);
    } else {
        free_frozen_pages_nolock(page, order);
    }
}
unsafe fn free_new_slab_nolock(s: *mut kmem_cache, slab: *mut slab) {
    __free_slab(s, slab, false);
}
unsafe extern "C" fn rcu_free_slab(h: *mut rcu_head) {
    let slab = rust_slub_slab_from_rcu(h);
    __free_slab((*slab).slab_cache, slab, true);
}
unsafe fn free_slab(s: *mut kmem_cache, slab: *mut slab) {
    if rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_CONSISTENCY_CHECKS) {
        slab_pad_check(s, slab);
        let base = rust_slub_slab_address(slab);
        let end = base.byte_add((rust_slub_slab_objects(slab) * (*s).size) as usize);
        let mut p = fixup_red_left(s, base);
        while p < end {
            check_object(s, slab, p, SLUB_RED_INACTIVE as u8);
            p = p.byte_add((*s).size as usize);
        }
    }
    if (*s).flags & RSL_SLAB_TYPESAFE_BY_RCU != 0 {
        call_rcu(rust_slub_slab_rcu(slab), Some(rcu_free_slab));
    } else {
        __free_slab(s, slab, true);
    }
}
unsafe fn discard_slab(s: *mut kmem_cache, slab: *mut slab) {
    dec_slabs_node(
        s,
        rust_slub_slab_nid(slab),
        rust_slub_slab_objects(slab) as i32,
    );
    free_slab(s, slab);
}
#[inline]
unsafe fn slab_test_node_partial(slab: *const slab) -> bool {
    rust_slub_test_slab_flag(slab, SL_partial)
}
#[inline]
unsafe fn slab_set_node_partial(slab: *mut slab) {
    rust_slub_set_slab_flag(slab, SL_partial);
}
#[inline]
unsafe fn slab_clear_node_partial(slab: *mut slab) {
    rust_slub_clear_slab_flag(slab, SL_partial);
}
#[inline]
unsafe fn set_node_partial_state(n: *mut kmem_cache_node, slab: *mut slab) {
    slab_set_node_partial(slab);
    (*n).nr_partial += 1;
}
#[inline]
unsafe fn __add_partial(n: *mut kmem_cache_node, slab: *mut slab, mode: add_mode) {
    if mode == ADD_TO_TAIL {
        rust_slub_list_add_tail(rust_slub_slab_list(slab), addr_of_mut!((*n).partial));
    } else {
        rust_slub_list_add(rust_slub_slab_list(slab), addr_of_mut!((*n).partial));
    }
    set_node_partial_state(n, slab);
}
#[inline]
unsafe fn add_partial(n: *mut kmem_cache_node, slab: *mut slab, mode: add_mode) {
    rust_slub_assert_spin_held(addr_of_mut!((*n).list_lock));
    slab_attach_kprobe_locked();
    __add_partial(n, slab, mode);
}
#[inline]
unsafe fn clear_node_partial_state(n: *mut kmem_cache_node, slab: *mut slab) {
    slab_clear_node_partial(slab);
    (*n).nr_partial -= 1;
}
#[inline]
unsafe fn remove_partial(n: *mut kmem_cache_node, slab: *mut slab) {
    rust_slub_assert_spin_held(addr_of_mut!((*n).list_lock));
    slab_attach_kprobe_locked();
    rust_slub_list_del(rust_slub_slab_list(slab));
    clear_node_partial_state(n, slab);
}
unsafe fn alloc_single_from_partial(
    s: *mut kmem_cache,
    n: *mut kmem_cache_node,
    slab: *mut slab,
    orig_size: i32,
) -> *mut Void {
    rust_slub_assert_spin_held(addr_of_mut!((*n).list_lock));
    slab_attach_kprobe_locked();
    #[cfg(CONFIG_SLUB_DEBUG)]
    if (*s).flags & RSL_SLAB_CONSISTENCY_CHECKS != 0 && !validate_slab_ptr(slab) {
        slab_err_invalid_page(s, slab);
        return null_mut();
    }
    let object = rust_slub_slab_freelist(slab);
    rust_slub_slab_set_freelist(slab, get_freepointer(s, object));
    rust_slub_slab_set_inuse(slab, rust_slub_slab_inuse(slab) + 1);
    if !alloc_debug_processing(s, slab, object, orig_size) {
        remove_partial(n, slab);
        return null_mut();
    }
    if rust_slub_slab_inuse(slab) == rust_slub_slab_objects(slab) {
        remove_partial(n, slab);
        add_full(s, n, slab);
    }
    object
}
#[inline]
unsafe fn next_slab_obj(s: *mut kmem_cache, iter: *mut slab_obj_iter) -> *mut Void {
    #[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
    if (*iter).random {
        loop {
            let idx = *(*s).random_seq.add((*iter).pos as usize) as ULong;
            (*iter).pos += 1;
            if (*iter).pos >= (*iter).freelist_count {
                (*iter).pos = 0;
            }
            if idx < (*iter).page_limit {
                return setup_object(s, (*iter).start.byte_add(idx as usize));
            }
        }
    }
    let pos = (*iter).pos;
    (*iter).pos += 1;
    setup_object(s, (*iter).start.byte_add(pos as usize * (*s).size as usize))
}
#[inline]
unsafe fn build_slab_freelist(s: *mut kmem_cache, slab: *mut slab, iter: *mut slab_obj_iter) {
    let nr = rust_slub_slab_objects(slab) - rust_slub_slab_inuse(slab);
    if nr == 0 {
        rust_slub_slab_set_freelist(slab, null_mut());
        return;
    }
    let mut cur = next_slab_obj(s, iter);
    rust_slub_slab_set_freelist(slab, cur);
    for _ in 1..nr {
        let next = next_slab_obj(s, iter);
        set_freepointer(s, cur, next);
        cur = next;
    }
    set_freepointer(s, cur, null_mut());
}
#[inline]
unsafe fn init_slab_obj_iter(
    s: *mut kmem_cache,
    slab: *mut slab,
    iter: *mut slab_obj_iter,
    allow_spin: bool,
) {
    (*iter).pos = 0;
    (*iter).start = fixup_red_left(s, rust_slub_slab_address(slab));
    #[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
    {
        (*iter).random = rust_slub_slab_objects(slab) >= 2 && !(*s).random_seq.is_null();
        if !(*iter).random {
            return;
        }
        (*iter).freelist_count = oo_objects((*s).oo) as ULong;
        (*iter).page_limit = (rust_slub_slab_objects(slab) * (*s).size) as ULong;
        if allow_spin {
            (*iter).pos = rust_slub_get_random_u32_below((*iter).freelist_count as u32) as ULong;
        } else {
            let state = rust_slub_get_cpu_rnd_state();
            (*iter).pos = rust_slub_prandom_u32_state(state) as ULong % (*iter).freelist_count;
            rust_slub_put_cpu_rnd_state();
        }
    }
}
unsafe fn alloc_single_from_new_slab(
    s: *mut kmem_cache,
    slab: *mut slab,
    ac: *const slab_alloc_context,
) -> *mut Void {
    let allow_spin = (*ac).alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
    let mut iter: slab_obj_iter = zeroed();
    init_slab_obj_iter(s, slab, &mut iter, allow_spin);
    let object = next_slab_obj(s, &mut iter);
    rust_slub_slab_set_inuse(slab, 1);
    let needs_add_partial = rust_slub_slab_objects(slab) > 1;
    build_slab_freelist(s, slab, &mut iter);
    set_freepointer(s, object, rust_slub_slab_freelist(slab));
    if !alloc_debug_processing(s, slab, object, (*ac).orig_size as i32) {
        return null_mut();
    }
    let n = get_node(s, rust_slub_slab_nid(slab));
    let mut flags = 0;
    if allow_spin {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*n).list_lock), &mut flags) {
        free_new_slab_nolock(s, slab);
        return null_mut();
    }
    if needs_add_partial {
        add_partial(n, slab, ADD_TO_HEAD);
    } else {
        add_full(s, n, slab);
    }
    inc_slabs_node(
        s,
        rust_slub_slab_nid(slab),
        rust_slub_slab_objects(slab) as i32,
    );
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    object
}
unsafe fn get_partial_node_bulk(
    s: *mut kmem_cache,
    n: *mut kmem_cache_node,
    pc: *mut partial_bulk_context,
    allow_spin: bool,
) -> bool {
    if n.is_null() || rust_slub_data_race_ulong(addr_of!((*n).nr_partial)) == 0 {
        return false;
    }
    rust_slub_init_list_head(addr_of_mut!((*pc).slabs));
    let mut flags = 0;
    if allow_spin {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*n).list_lock), &mut flags) {
        return false;
    }
    let mut first: *mut slab = null_mut();
    let mut last: *mut slab = null_mut();
    let mut total_free: u32 = 0;
    let mut entry = (*n).partial.next;
    while entry != addr_of_mut!((*n).partial) {
        let next = (*entry).next;
        let slab = rust_slub_slab_from_list(entry);
        if !pfmemalloc_match(slab, (*pc).flags) {
            if !first.is_null() {
                rust_slub_list_bulk_move_tail(
                    addr_of_mut!((*pc).slabs),
                    rust_slub_slab_list(first),
                    rust_slub_slab_list(last),
                );
                first = null_mut();
            }
            entry = next;
            continue;
        }
        let mut flc: freelist_counters = zeroed();
        rust_slub_fc_set_counters(&mut flc, rust_slub_data_race_slab_counters(slab));
        let slab_free = rust_slub_fc_objects(&flc) - rust_slub_fc_inuse(&flc);
        if total_free >= (*pc).min_objects && total_free + slab_free > (*pc).max_objects {
            break;
        }
        if first.is_null() {
            first = slab;
        }
        last = slab;
        clear_node_partial_state(n, slab);
        total_free += slab_free;
        if total_free >= (*pc).max_objects {
            break;
        }
        entry = next;
    }
    if !first.is_null() {
        rust_slub_list_bulk_move_tail(
            addr_of_mut!((*pc).slabs),
            rust_slub_slab_list(first),
            rust_slub_slab_list(last),
        );
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    total_free > 0
}
unsafe fn get_from_partial_node(
    s: *mut kmem_cache,
    n: *mut kmem_cache_node,
    gfp_flags: gfp_t,
    ac: *const slab_alloc_context,
) -> *mut Void {
    if n.is_null() || (*n).nr_partial == 0 {
        return null_mut();
    }
    let mut flags = 0;
    if (*ac).alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0 {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*n).list_lock), &mut flags) {
        return null_mut();
    }
    let mut object = null_mut();
    let mut entry = (*n).partial.next;
    while entry != addr_of_mut!((*n).partial) {
        let next = (*entry).next;
        let slab = rust_slub_slab_from_list(entry);
        if !pfmemalloc_match(slab, gfp_flags) {
            entry = next;
            continue;
        }
        if cfg!(CONFIG_SLUB_TINY) || kmem_cache_debug(s) {
            object = alloc_single_from_partial(s, n, slab, (*ac).orig_size as i32);
            if !object.is_null() {
                break;
            }
            entry = next;
            continue;
        }
        let mut old: freelist_counters = zeroed();
        let mut new: freelist_counters = zeroed();
        loop {
            rust_slub_fc_init(
                &mut old,
                rust_slub_slab_freelist(slab),
                rust_slub_slab_counters(slab),
            );
            rust_slub_fc_init(
                &mut new,
                get_freepointer(s, rust_slub_fc_freelist(&old)),
                rust_slub_fc_counters(&old),
            );
            let inuse = rust_slub_fc_inuse(&new);
            rust_slub_fc_set_inuse(&mut new, inuse + 1);
            if __slab_update_freelist(
                s,
                slab,
                &mut old,
                &mut new,
                c"get_from_partial_node".as_ptr().cast::<CChar>(),
            ) {
                break;
            }
        }
        object = rust_slub_fc_freelist(&old);
        if rust_slub_fc_freelist(&new).is_null() {
            remove_partial(n, slab);
        }
        break;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    object
}
unsafe fn get_from_any_partial(
    s: *mut kmem_cache,
    gfp_flags: gfp_t,
    ac: *const slab_alloc_context,
) -> *mut Void {
    #[cfg(CONFIG_NUMA)]
    {
        let highest = rust_slub_gfp_zone(gfp_flags);
        let allow_spin = (*ac).alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
        if (*s).remote_node_defrag_ratio == 0
            || rust_slub_get_cycles() % 1024 > (*s).remote_node_defrag_ratio as ULong
        {
            return null_mut();
        }
        loop {
            let cookie = if allow_spin {
                rust_slub_read_mems_allowed_begin()
            } else {
                0
            };
            let zonelist = rust_slub_node_zonelist(mempolicy_slab_node(), gfp_flags);
            let mut z = rust_slub_first_zones_zonelist(zonelist, highest);
            while !(*z).zone.is_null() {
                let zone = (*z).zone;
                let n = get_node(s, rust_slub_zone_to_nid(zone));
                if !n.is_null()
                    && rust_slub_cpuset_zone_allowed(zone, gfp_flags)
                    && (*n).nr_partial > (*s).min_partial
                {
                    let object = get_from_partial_node(s, n, gfp_flags, ac);
                    if !object.is_null() {
                        return object;
                    }
                }
                z = rust_slub_next_zones_zonelist(z.add(1), highest);
            }
            if !allow_spin || !rust_slub_read_mems_allowed_retry(cookie) {
                break;
            }
        }
    }
    null_mut()
}
unsafe fn get_from_partial(
    s: *mut kmem_cache,
    node: i32,
    flags: gfp_t,
    ac: *const slab_alloc_context,
) -> *mut Void {
    let searchnode = if node == RSL_NUMA_NO_NODE as i32 {
        rust_slub_numa_mem_id()
    } else {
        node
    };
    let object = get_from_partial_node(s, get_node(s, searchnode), flags, ac);
    if !object.is_null() || (node != RSL_NUMA_NO_NODE as i32 && flags & RSL___GFP_THISNODE != 0) {
        return object;
    }
    get_from_any_partial(s, flags, ac)
}
unsafe fn has_pcs_used(cpu: i32, s: *mut kmem_cache) -> bool {
    if !rust_slub_cache_has_sheaves(s) {
        return false;
    }
    let pcs = rust_slub_percpu_sheaves(s, cpu as u32);
    !(*pcs).spare.is_null() || !(*pcs).rcu_free.is_null() || (*(*pcs).main).size != 0
}
unsafe extern "C" fn flush_cpu_sheaves(w: *mut work_struct) {
    let sfw = w
        .byte_sub(offset_of!(slub_flush_work, work))
        .cast::<slub_flush_work>();
    let s = (*sfw).s;
    if rust_slub_cache_has_sheaves(s) {
        pcs_flush_all(s);
    }
}
unsafe fn flush_all_cpus_locked(s: *mut kmem_cache) {
    rust_slub_assert_cpus_held();
    rust_slub_mutex_lock(addr_of_mut!(flush_lock));
    let mut cpu = rust_slub_next_online_cpu(0);
    while cpu < rust_slub_nr_cpu_ids() {
        let sfw = rust_slub_percpu_flush_work(cpu);
        if !has_pcs_used(cpu as i32, s) {
            (*sfw).skip = true;
        } else {
            rust_slub_init_work_flush(addr_of_mut!((*sfw).work), Some(flush_cpu_sheaves));
            (*sfw).skip = false;
            (*sfw).s = s;
            queue_work_on(cpu as i32, flushwq, addr_of_mut!((*sfw).work));
        }
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
    cpu = rust_slub_next_online_cpu(0);
    while cpu < rust_slub_nr_cpu_ids() {
        let sfw = rust_slub_percpu_flush_work(cpu);
        if !(*sfw).skip {
            flush_work(addr_of_mut!((*sfw).work));
        }
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
    rust_slub_mutex_unlock(addr_of_mut!(flush_lock));
}
unsafe fn flush_all(s: *mut kmem_cache) {
    cpus_read_lock();
    flush_all_cpus_locked(s);
    cpus_read_unlock();
}
unsafe extern "C" fn flush_rcu_sheaf(w: *mut work_struct) {
    let sfw = w
        .byte_sub(offset_of!(slub_flush_work, work))
        .cast::<slub_flush_work>();
    let s = (*sfw).s;
    rust_slub_pcs_lock(s);
    let pcs = rust_slub_this_cpu_sheaves(s);
    let rcu_free = (*pcs).rcu_free;
    (*pcs).rcu_free = null_mut();
    rust_slub_pcs_unlock(s);
    if !rcu_free.is_null() {
        call_rcu(rust_slub_sheaf_rcu(rcu_free), Some(rcu_free_sheaf_nobarn));
    }
}
#[no_mangle]
pub unsafe extern "C" fn flush_rcu_sheaves_on_cache(s: *mut kmem_cache) {
    rust_slub_assert_cpus_held();
    rust_slub_mutex_lock(addr_of_mut!(flush_lock));
    let mut cpu = rust_slub_next_online_cpu(0);
    while cpu < rust_slub_nr_cpu_ids() {
        let sfw = rust_slub_percpu_flush_work(cpu);
        rust_slub_init_work_rcu_flush(addr_of_mut!((*sfw).work), Some(flush_rcu_sheaf));
        (*sfw).s = s;
        queue_work_on(cpu as i32, flushwq, addr_of_mut!((*sfw).work));
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
    cpu = rust_slub_next_online_cpu(0);
    while cpu < rust_slub_nr_cpu_ids() {
        let sfw = rust_slub_percpu_flush_work(cpu);
        flush_work(addr_of_mut!((*sfw).work));
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
    rust_slub_mutex_unlock(addr_of_mut!(flush_lock));
}
#[no_mangle]
pub unsafe extern "C" fn flush_all_rcu_sheaves() {
    deferred_work_barrier();
    cpus_read_lock();
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let mut l = slab_caches.next;
    while l != addr_of_mut!(slab_caches) {
        let s = l
            .byte_sub(offset_of!(kmem_cache, list))
            .cast::<kmem_cache>();
        if rust_slub_cache_has_sheaves(s) {
            flush_rcu_sheaves_on_cache(s);
        }
        l = (*l).next;
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    cpus_read_unlock();
    rcu_barrier();
}
#[no_mangle]
unsafe extern "C" fn slub_cpu_setup(cpu: u32) -> i32 {
    let nid = rust_slub_cpu_to_node(cpu);
    if rust_slub_node_isset(nid, addr_of!(slab_barn_nodes)) {
        return 0;
    }
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let mut ret = 0;
    if !rust_slub_node_isset(nid, addr_of!(slab_barn_nodes)) {
        let mut l = slab_caches.next;
        while l != addr_of_mut!(slab_caches) {
            let s = l
                .byte_sub(offset_of!(kmem_cache, list))
                .cast::<kmem_cache>();
            l = (*l).next;
            if !rust_slub_cache_has_sheaves(s) || !get_barn_node(s, nid).is_null() {
                continue;
            }
            let barn = rust_slub_kmalloc_node(size_of::<node_barn>(), RSL_GFP_KERNEL, nid)
                .cast::<node_barn>();
            if barn.is_null() {
                ret = -(ENOMEM as i32);
                break;
            }
            barn_init(barn);
            (*s).per_node[nid as usize].barn = barn;
        }
        if ret == 0 {
            rust_slub_node_set(nid, addr_of_mut!(slab_barn_nodes));
        }
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    ret
}
#[no_mangle]
unsafe extern "C" fn slub_cpu_dead(cpu: u32) -> i32 {
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let mut l = slab_caches.next;
    while l != addr_of_mut!(slab_caches) {
        let s = l
            .byte_sub(offset_of!(kmem_cache, list))
            .cast::<kmem_cache>();
        if rust_slub_cache_has_sheaves(s) {
            __pcs_flush_all_cpu(s, cpu);
        }
        l = (*l).next;
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    0
}
#[inline]
unsafe fn pfmemalloc_match(slab: *mut slab, gfpflags: gfp_t) -> bool {
    !slab_test_pfmemalloc(slab) || rust_slub_gfp_pfmemalloc_allowed(gfpflags)
}
#[inline]
unsafe fn get_freelist_nofreeze(s: *mut kmem_cache, slab: *mut slab, count: *mut u32) -> *mut Void {
    let mut old: freelist_counters = zeroed();
    let mut new: freelist_counters = zeroed();
    loop {
        rust_slub_fc_init(
            &mut old,
            rust_slub_slab_freelist(slab),
            rust_slub_slab_counters(slab),
        );
        rust_slub_fc_init(&mut new, null_mut(), rust_slub_fc_counters(&old));
        rust_slub_warn_frozen_freelist(rust_slub_fc_frozen(&new));
        rust_slub_fc_set_inuse(&mut new, rust_slub_fc_objects(&old));
        if slab_update_freelist(
            s,
            slab,
            &mut old,
            &mut new,
            c"get_freelist_nofreeze".as_ptr().cast::<CChar>(),
        ) {
            break;
        }
    }
    *count = rust_slub_fc_objects(&old) - rust_slub_fc_inuse(&old);
    rust_slub_fc_freelist(&old)
}
#[inline(always)]
unsafe fn maybe_wipe_obj_freeptr(s: *mut kmem_cache, obj: *mut Void) {
    if rust_slub_slab_want_init_on_free(s) && !obj.is_null() && !freeptr_outside_object(s) {
        core::ptr::write_bytes(
            rust_slub_kasan_reset_tag(obj)
                .byte_add((*s).offset as usize)
                .cast::<u8>(),
            0,
            size_of::<*mut Void>(),
        );
    }
}
unsafe fn alloc_from_new_slab(
    s: *mut kmem_cache,
    slab: *mut slab,
    p: *mut *mut Void,
    mut count: u32,
    allow_spin: bool,
) -> u32 {
    let mut iter: slab_obj_iter = zeroed();
    let mut needs_add_partial = true;
    if count >= rust_slub_slab_objects(slab) {
        needs_add_partial = false;
        count = rust_slub_slab_objects(slab);
    }
    init_slab_obj_iter(s, slab, &mut iter, allow_spin);
    for i in 0..count as usize {
        *p.add(i) = next_slab_obj(s, &mut iter);
    }
    rust_slub_slab_set_inuse(slab, count);
    build_slab_freelist(s, slab, &mut iter);
    if needs_add_partial {
        let n = get_node(s, rust_slub_slab_nid(slab));
        let mut flags = 0;
        if allow_spin {
            flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
        } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*n).list_lock), &mut flags) {
            free_new_slab_nolock(s, slab);
            return 0;
        }
        add_partial(n, slab, ADD_TO_HEAD);
        rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    }
    inc_slabs_node(
        s,
        rust_slub_slab_nid(slab),
        rust_slub_slab_objects(slab) as i32,
    );
    count
}
unsafe fn ___slab_alloc(
    s: *mut kmem_cache,
    gfpflags: gfp_t,
    node: i32,
    ac: *const slab_alloc_context,
) -> *mut Void {
    let allow_spin = (*ac).alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
    let mut try_thisnode = true;
    stat(s, ALLOC_SLOWPATH);
    loop {
        let mut trynode_flags = gfpflags;
        if node != RSL_NUMA_NO_NODE as i32 && gfpflags & RSL___GFP_THISNODE == 0 && try_thisnode {
            trynode_flags &= RSL_GFP_NOWAIT | RSL___GFP_NOMEMALLOC | RSL___GFP_ACCOUNT;
            trynode_flags |= RSL___GFP_NOWARN | RSL___GFP_THISNODE;
        }
        let mut object = get_from_partial(s, node, trynode_flags, ac);
        if object.is_null() {
            let slab = new_slab(s, trynode_flags, (*ac).alloc_flags, node);
            if slab.is_null() {
                if node != RSL_NUMA_NO_NODE as i32
                    && gfpflags & RSL___GFP_THISNODE == 0
                    && try_thisnode
                {
                    try_thisnode = false;
                    continue;
                }
                slab_out_of_memory(s, gfpflags, node);
                return null_mut();
            }
            stat(s, ALLOC_SLAB);
            if cfg!(CONFIG_SLUB_TINY) || kmem_cache_debug(s) {
                object = alloc_single_from_new_slab(s, slab, ac);
            } else {
                if alloc_from_new_slab(s, slab, &mut object, 1, allow_spin) != 0 {
                    return object;
                }
                // The failed nonspinning partial-list lock path frees the new
                // slab after filling the output slot. That stale slot must not
                // be mistaken for allocation success.
                if allow_spin {
                    continue;
                }
                return null_mut();
            }
            if object.is_null() {
                if allow_spin {
                    continue;
                }
                return null_mut();
            }
        }
        if rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_STORE_USER) {
            set_track(s, object, TRACK_ALLOC, (*ac).caller_addr, gfpflags);
        }
        return object;
    }
}
#[inline(always)]
unsafe fn apply_strict_numa_policy(mut node: i32) -> i32 {
    #[cfg(CONFIG_NUMA)]
    if rust_slub_strict_numa_enabled() && node == RSL_NUMA_NO_NODE as i32 {
        let mpol = rust_slub_current_mempolicy();
        if !mpol.is_null()
            && ((*mpol).mode != MPOL_BIND as u16
                || !rust_slub_node_isset(rust_slub_numa_mem_id(), addr_of!((*mpol).nodes)))
        {
            node = mempolicy_slab_node();
        }
    }
    node
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_pre_alloc_hook(s: *mut kmem_cache, mut flags: gfp_t) -> *mut kmem_cache {
    flags &= gfp_allowed_mask;
    rust_slub_might_alloc(flags);
    if rust_slub_should_failslab(s, flags) {
        null_mut()
    } else {
        s
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_post_alloc_hook(
    s: *mut kmem_cache,
    flags: gfp_t,
    size: usize,
    p: *mut *mut Void,
    ac: *const slab_alloc_context,
) -> bool {
    let mut init = rust_slub_slab_want_init_on_alloc(flags, s);
    let mut zero_size = (*s).object_size;
    let init_flags = flags & gfp_allowed_mask;
    let mut kasan_init = false;
    if rust_slub_slub_debug_orig_size(s) {
        zero_size = (*ac).orig_size as u32;
    }
    if rust_slub_kasan_has_integrated_init() && !rust_slub_debug_enabled() {
        kasan_init = init;
        init = false;
    }
    for i in 0..size {
        let pp = p.add(i);
        *pp = rust_slub_kasan_slab_alloc(s, *pp, init_flags, kasan_init);
        if init && !(*pp).is_null() && !rust_slub_is_kfence_address(*pp) {
            core::ptr::write_bytes((*pp).cast::<u8>(), 0, zero_size as usize);
        }
        if (*ac).alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0 {
            rust_slub_kmemleak_alloc_recursive(
                *pp,
                (*s).object_size as usize,
                1,
                (*s).flags,
                init_flags,
            );
        }
        rust_slub_kmsan_slab_alloc(s, *pp, init_flags);
        alloc_tagging_slab_alloc_hook(s, *pp, flags, (*ac).alloc_flags);
    }
    memcg_slab_post_alloc_hook(s, flags, size, p, ac)
}
