// SPDX-License-Identifier: GPL-2.0
// Allocation/free half of mm/slub.c at e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Textually included by slub.rs. Layouts/constants come from configured native
// headers. Public *_body entry points take the caller captured by ABI thunks.

#[inline(always)]
unsafe fn alloc_context(
    s: *mut kmem_cache,
    caller: ULong,
    size: usize,
    flags: u32,
) -> slab_alloc_context {
    let mut ac: slab_alloc_context = zeroed();
    ac.caller_addr = caller;
    ac.orig_size = size;
    ac.alloc_flags = flags;
    let _ = s;
    ac
}

#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_alloc_node(
    mut s: *mut kmem_cache,
    gfpflags: gfp_t,
    mut node: i32,
    ac: *const slab_alloc_context,
) -> *mut Void {
    s = slab_pre_alloc_hook(s, gfpflags);
    if s.is_null() {
        return null_mut();
    }
    let mut object = rust_slub_kfence_alloc(s, (*ac).orig_size, gfpflags);
    if object.is_null() {
        node = apply_strict_numa_policy(node);
        object = alloc_from_pcs(s, gfpflags, (*ac).alloc_flags, node);
        if object.is_null() {
            object = ___slab_alloc(s, gfpflags, node, ac);
        }
        maybe_wipe_obj_freeptr(s, object);
    }
    slab_post_alloc_hook(s, gfpflags, 1, addr_of_mut!(object), ac);
    object
}

#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_alloc_noprof_body(
    s: *mut kmem_cache,
    flags: gfp_t,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(s, caller, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
    let ret = slab_alloc_node(s, flags, RSL_NUMA_NO_NODE, &ac);
    rust_slub_trace_cache_alloc(caller, ret, s, flags, RSL_NUMA_NO_NODE);
    ret
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_alloc_lru_noprof_body(
    s: *mut kmem_cache,
    lru: *mut list_lru,
    flags: gfp_t,
    caller: ULong,
) -> *mut Void {
    let mut ac = alloc_context(s, caller, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
    ac.lru = lru;
    let ret = slab_alloc_node(s, flags, RSL_NUMA_NO_NODE, &ac);
    rust_slub_trace_cache_alloc(caller, ret, s, flags, RSL_NUMA_NO_NODE);
    ret
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_alloc_node_noprof_body(
    s: *mut kmem_cache,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(s, caller, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
    let ret = slab_alloc_node(s, flags, node, &ac);
    rust_slub_trace_cache_alloc(caller, ret, s, flags, node);
    ret
}
#[no_mangle]
unsafe extern "C" fn kmem_cache_charge(objp: *mut Void, flags: gfp_t) -> bool {
    !rust_slub_memcg_kmem_online() || memcg_slab_post_charge(objp, flags)
}

unsafe fn __prefill_sheaf_pfmemalloc(
    s: *mut kmem_cache,
    sheaf: *mut slab_sheaf,
    gfp: gfp_t,
) -> i32 {
    let mut restricted = gfp | RSL___GFP_NOMEMALLOC;
    if rust_slub_gfp_pfmemalloc_allowed(gfp) {
        restricted |= RSL___GFP_NOWARN;
    }
    let ret = refill_sheaf(s, sheaf, restricted);
    if ret == 0 || !rust_slub_gfp_pfmemalloc_allowed(gfp) {
        return ret;
    }
    let ret = refill_sheaf(s, sheaf, gfp);
    rust_slub_sheaf_set_pfmemalloc(sheaf, true);
    ret
}

#[no_mangle]
unsafe extern "C" fn kmem_cache_prefill_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    size: u32,
) -> *mut slab_sheaf {
    if size == 0 {
        return null_mut();
    }
    if size > (*s).sheaf_capacity {
        let sheaf = __alloc_empty_sheaf(s, gfp, RSL_SLAB_ALLOC_DEFAULT, size);
        if sheaf.is_null() {
            return null_mut();
        }
        stat(s, SHEAF_PREFILL_OVERSIZE);
        rust_slub_sheaf_set_capacity(sheaf, size);
        if !__kmem_cache_alloc_bulk(s, gfp, size as usize, (*sheaf).objects.as_mut_ptr()) {
            free_empty_sheaf(s, sheaf);
            return null_mut();
        }
        (*sheaf).size = size;
        return sheaf;
    }
    let mut sheaf = null_mut();
    rust_slub_pcs_lock(s);
    let pcs = rust_slub_this_cpu_sheaves(s);
    if !(*pcs).spare.is_null() {
        sheaf = (*pcs).spare;
        (*pcs).spare = null_mut();
        stat(s, SHEAF_PREFILL_FAST);
    } else {
        let barn = get_barn(s);
        stat(s, SHEAF_PREFILL_SLOW);
        if !barn.is_null() {
            sheaf = barn_get_full_or_empty_sheaf(barn);
        }
        stat(
            s,
            if !sheaf.is_null() && (*sheaf).size != 0 {
                BARN_GET
            } else {
                BARN_GET_FAIL
            },
        );
    }
    rust_slub_pcs_unlock(s);
    if sheaf.is_null() {
        sheaf = alloc_empty_sheaf(s, gfp, RSL_SLAB_ALLOC_DEFAULT);
    }
    if !sheaf.is_null() {
        rust_slub_sheaf_set_capacity(sheaf, (*s).sheaf_capacity);
        rust_slub_sheaf_set_pfmemalloc(sheaf, false);
        if (*sheaf).size < size && __prefill_sheaf_pfmemalloc(s, sheaf, gfp) != 0 {
            sheaf_flush_unused(s, sheaf);
            free_empty_sheaf(s, sheaf);
            return null_mut();
        }
    }
    sheaf
}

#[no_mangle]
unsafe extern "C" fn kmem_cache_return_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    mut sheaf: *mut slab_sheaf,
) {
    if rust_slub_sheaf_capacity(sheaf) != (*s).sheaf_capacity || rust_slub_sheaf_pfmemalloc(sheaf) {
        sheaf_flush_unused(s, sheaf);
        free_empty_sheaf(s, sheaf);
        return;
    }
    rust_slub_pcs_lock(s);
    let pcs = rust_slub_this_cpu_sheaves(s);
    let barn = get_barn(s);
    if (*pcs).spare.is_null() {
        (*pcs).spare = sheaf;
        sheaf = null_mut();
        stat(s, SHEAF_RETURN_FAST);
    }
    rust_slub_pcs_unlock(s);
    if sheaf.is_null() {
        return;
    }
    stat(s, SHEAF_RETURN_SLOW);
    if barn.is_null()
        || rust_slub_barn_nr_full_racy(barn) >= RSL_MAX_FULL_SHEAVES
        || refill_sheaf(s, sheaf, gfp | RSL___GFP_NOMEMALLOC | RSL___GFP_NOWARN) != 0
    {
        sheaf_flush_unused(s, sheaf);
        free_empty_sheaf(s, sheaf);
        return;
    }
    barn_put_full_sheaf(barn, sheaf);
    stat(s, BARN_PUT);
}

#[no_mangle]
unsafe extern "C" fn kmem_cache_refill_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    sheafp: *mut *mut slab_sheaf,
    size: u32,
) -> i32 {
    if sheafp.is_null() || (*sheafp).is_null() {
        return -(RSL_EINVAL as i32);
    }
    let mut sheaf = *sheafp;
    if (*sheaf).size >= size {
        return 0;
    }
    let cap = rust_slub_sheaf_capacity(sheaf);
    if cap >= size {
        if cap == (*s).sheaf_capacity {
            return __prefill_sheaf_pfmemalloc(s, sheaf, gfp);
        }
        if !__kmem_cache_alloc_bulk(
            s,
            gfp,
            (cap - (*sheaf).size) as usize,
            (*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize),
        ) {
            return -(RSL_ENOMEM as i32);
        }
        (*sheaf).size = cap;
        return 0;
    }
    sheaf = kmem_cache_prefill_sheaf(s, gfp, size);
    if sheaf.is_null() {
        return -(RSL_ENOMEM as i32);
    }
    kmem_cache_return_sheaf(s, gfp, *sheafp);
    *sheafp = sheaf;
    0
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_alloc_from_sheaf_noprof_body(
    s: *mut kmem_cache,
    gfp: gfp_t,
    sheaf: *mut slab_sheaf,
    caller: ULong,
) -> *mut Void {
    let mut ret = null_mut();
    let ac = alloc_context(s, 0, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
    if (*sheaf).size != 0 {
        ret = rust_slub_kfence_alloc(s, (*s).object_size as usize, gfp);
        if ret.is_null() {
            (*sheaf).size -= 1;
            ret = *(*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize);
        }
        slab_post_alloc_hook(s, gfp | RSL___GFP_NOFAIL, 1, &mut ret, &ac);
    }
    rust_slub_trace_cache_alloc(caller, ret, s, gfp, RSL_NUMA_NO_NODE);
    ret
}
#[no_mangle]
unsafe extern "C" fn kmem_cache_sheaf_size(sheaf: *mut slab_sheaf) -> u32 {
    (*sheaf).size
}

unsafe fn ___kmalloc_large_node(size: usize, mut flags: gfp_t, node: i32) -> *mut Void {
    let order = rust_slub_get_order(size);
    let mut ptr = null_mut();
    if flags & RSL_GFP_SLAB_BUG_MASK != 0 {
        flags = rust_slub_kmalloc_fix_flags(flags);
    }
    flags |= RSL___GFP_COMP;
    let page = if node == RSL_NUMA_NO_NODE {
        rust_slub_alloc_frozen_pages_noprof(flags, order)
    } else {
        rust_slub_alloc_frozen_pages_node_noprof(flags, order, node)
    };
    if !page.is_null() {
        ptr = rust_slub_page_address(page);
        rust_slub_mod_lruvec_slab_bytes(page, (RSL_PAGE_SIZE as Long) << order);
        rust_slub_set_page_large_kmalloc(page);
    }
    ptr = rust_slub_kasan_kmalloc_large(ptr, size, flags);
    rust_slub_kmemleak_alloc(ptr, size, 1, flags);
    rust_slub_kmsan_kmalloc_large(ptr, size, flags);
    ptr
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_large_noprof_body(
    size: usize,
    flags: gfp_t,
    caller: ULong,
) -> *mut Void {
    rust_slub_kmalloc_large_node_noprof_body(size, flags, RSL_NUMA_NO_NODE, caller)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_large_node_noprof_body(
    size: usize,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ret = ___kmalloc_large_node(size, flags, node);
    rust_slub_trace_kmalloc(
        caller,
        ret,
        size,
        (RSL_PAGE_SIZE as usize) << rust_slub_get_order(size),
        flags,
        node,
    );
    ret
}

#[inline(always)]
unsafe fn __do_kmalloc_node(
    b: *mut kmem_buckets,
    flags: gfp_t,
    node: i32,
    token: kmalloc_token_t,
    ac: *const slab_alloc_context,
) -> *mut Void {
    let size = (*ac).orig_size;
    if size > RSL_KMALLOC_MAX_CACHE_SIZE as usize {
        // Preserve both trace events, as in __kmalloc_large_node_noprof + caller.
        let ret = __kmalloc_large_node_noprof(size, flags, node);
        rust_slub_trace_kmalloc(
            (*ac).caller_addr,
            ret,
            size,
            (RSL_PAGE_SIZE as usize) << rust_slub_get_order(size),
            flags,
            node,
        );
        return ret;
    }
    if size == 0 {
        return RSL_ZERO_SIZE_PTR as usize as *mut Void;
    }
    let s = rust_slub_kmalloc_slab(size, b, flags, token, (*ac).alloc_flags);
    let ret = slab_alloc_node(s, flags, node, ac);
    let ret = rust_slub_kasan_kmalloc(s, ret, size, flags);
    rust_slub_trace_kmalloc(
        (*ac).caller_addr,
        ret,
        size,
        (*s).size as usize,
        flags,
        node,
    );
    ret
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_node_noprof_body(
    size: usize,
    b: *mut kmem_buckets,
    token: kmalloc_token_t,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(null_mut(), caller, size, RSL_SLAB_ALLOC_DEFAULT);
    __do_kmalloc_node(b, flags, node, token, &ac)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_noprof_body(
    size: usize,
    token: kmalloc_token_t,
    flags: gfp_t,
    caller: ULong,
) -> *mut Void {
    rust_slub_kmalloc_node_noprof_body(size, null_mut(), token, flags, RSL_NUMA_NO_NODE, caller)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_node_track_caller_noprof_body(
    size: usize,
    b: *mut kmem_buckets,
    token: kmalloc_token_t,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    rust_slub_kmalloc_node_noprof_body(size, b, token, flags, node, caller)
}
unsafe fn __kmalloc_nolock_noprof(
    mut size: usize,
    token: kmalloc_token_t,
    mut flags: gfp_t,
    mut node: i32,
    ac: *const slab_alloc_context,
) -> *mut Void {
    rust_slub_vm_warn_nolock_spinning(rust_slub_alloc_flags_allow_spinning((*ac).alloc_flags));
    rust_slub_vm_warn_nolock_gfp(
        flags & !(RSL___GFP_ACCOUNT | RSL___GFP_ZERO | RSL___GFP_NOWARN | RSL___GFP_NOMEMALLOC)
            != 0,
    );
    flags |= RSL___GFP_NOWARN | RSL___GFP_NOMEMALLOC;
    if size == 0 {
        return RSL_ZERO_SIZE_PTR as usize as *mut Void;
    }
    if !rust_slub_can_spin_trylock() {
        return null_mut();
    }
    node = apply_strict_numa_policy(node);
    let mut can_retry = true;
    loop {
        if size > RSL_KMALLOC_MAX_CACHE_SIZE as usize {
            return null_mut();
        }
        let s = rust_slub_kmalloc_slab(size, null_mut(), flags, token, (*ac).alloc_flags);
        if (*s).flags & RSL___CMPXCHG_DOUBLE == 0 && !kmem_cache_debug(s) {
            return null_mut();
        }
        let mut ret = alloc_from_pcs(s, flags, (*ac).alloc_flags, node);
        if ret.is_null() {
            ret = ___slab_alloc(s, flags, node, ac);
            if ret.is_null() && can_retry {
                size = (*s).object_size as usize + 1;
                can_retry = false;
                continue;
            }
        }
        maybe_wipe_obj_freeptr(s, ret);
        slab_post_alloc_hook(s, flags, 1, &mut ret, ac);
        return rust_slub_kasan_kmalloc(s, ret, (*ac).orig_size, flags);
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_nolock_noprof_body(
    size: usize,
    token: kmalloc_token_t,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(null_mut(), caller, size, RSL_SLAB_ALLOC_NOLOCK);
    __kmalloc_nolock_noprof(size, token, flags, node, &ac)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_cache_noprof_body(
    s: *mut kmem_cache,
    flags: gfp_t,
    size: usize,
    caller: ULong,
) -> *mut Void {
    rust_slub_kmalloc_cache_node_noprof_body(s, flags, RSL_NUMA_NO_NODE, size, caller)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_cache_node_noprof_body(
    s: *mut kmem_cache,
    flags: gfp_t,
    node: i32,
    size: usize,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(s, caller, size, RSL_SLAB_ALLOC_DEFAULT);
    let ret = slab_alloc_node(s, flags, node, &ac);
    rust_slub_trace_kmalloc(caller, ret, size, (*s).size as usize, flags, node);
    rust_slub_kasan_kmalloc(s, ret, size, flags)
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmalloc_flags_noprof_body(
    size: usize,
    token: kmalloc_token_t,
    flags: gfp_t,
    alloc_flags: u32,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(null_mut(), caller, size, alloc_flags);
    if rust_slub_alloc_flags_allow_spinning(alloc_flags) {
        __do_kmalloc_node(null_mut(), flags, node, token, &ac)
    } else {
        __kmalloc_nolock_noprof(size, token, flags, node, &ac)
    }
}

#[inline(never)]
unsafe fn free_to_partial_list(
    s: *mut kmem_cache,
    slab: *mut slab,
    head: *mut Void,
    tail: *mut Void,
    bulk_cnt: i32,
    addr: ULong,
) {
    let n = get_node(s, rust_slub_slab_nid(slab));
    let mut slab_free = null_mut();
    let mut cnt = bulk_cnt;
    let handle = if (*s).flags & RSL_SLAB_STORE_USER != 0 {
        set_track_prepare(RSL___GFP_NOWARN)
    } else {
        0
    };
    let flags = rust_slub_node_lock_irqsave(n);
    if free_debug_processing(s, slab, head, tail, &mut cnt, addr, handle) {
        let prior = rust_slub_slab_freelist(slab);
        rust_slub_slab_set_inuse(slab, rust_slub_slab_inuse(slab).wrapping_sub(cnt as u32));
        set_freepointer(s, tail, prior);
        rust_slub_slab_set_freelist(slab, head);
        if rust_slub_slab_inuse(slab) == 0 && (*n).nr_partial >= (*s).min_partial {
            slab_free = slab;
        }
        if prior.is_null() {
            remove_full(s, n, slab);
            if slab_free.is_null() {
                add_partial(n, slab, ADD_TO_TAIL);
                stat(s, FREE_ADD_PARTIAL);
            }
        } else if !slab_free.is_null() {
            remove_partial(n, slab);
            stat(s, FREE_REMOVE_PARTIAL);
        }
    }
    if !slab_free.is_null() {
        dec_slabs_node(
            s,
            rust_slub_slab_nid(slab_free),
            rust_slub_slab_objects(slab_free) as i32,
        );
    }
    rust_slub_node_unlock_irqrestore(n, flags);
    if !slab_free.is_null() {
        stat(s, FREE_SLAB);
        free_slab(s, slab_free);
    }
}
unsafe fn __slab_try_return_freelist(
    s: *mut kmem_cache,
    slab: *mut slab,
    head: *mut Void,
    cnt: i32,
) -> bool {
    let mut old: freelist_counters = zeroed();
    let mut new: freelist_counters = zeroed();
    rust_slub_fc_init(
        &mut old,
        rust_slub_slab_freelist(slab),
        rust_slub_slab_counters(slab),
    );
    if !rust_slub_fc_freelist(&old).is_null() {
        return false;
    }
    rust_slub_fc_init(&mut new, head, rust_slub_fc_counters(&old));
    rust_slub_fc_set_inuse(&mut new, rust_slub_fc_inuse(&new).wrapping_sub(cnt as u32));
    slab_update_freelist(
        s,
        slab,
        &mut old,
        &mut new,
        c"__slab_try_return_freelist".as_ptr().cast::<CChar>(),
    )
}
unsafe fn __slab_free(
    s: *mut kmem_cache,
    slab: *mut slab,
    head: *mut Void,
    tail: *mut Void,
    cnt: i32,
    addr: ULong,
) {
    if cfg!(CONFIG_SLUB_TINY) || kmem_cache_debug(s) {
        free_to_partial_list(s, slab, head, tail, cnt, addr);
        return;
    }
    let mut old: freelist_counters = zeroed();
    let mut new: freelist_counters = zeroed();
    let mut n: *mut kmem_cache_node = null_mut();
    let mut flags = 0;
    let mut was_full;
    let mut on_node_partial = false;
    loop {
        if !n.is_null() {
            rust_slub_node_unlock_irqrestore(n, flags);
            n = null_mut();
        }
        rust_slub_fc_init(
            &mut old,
            rust_slub_slab_freelist(slab),
            rust_slub_slab_counters(slab),
        );
        was_full = rust_slub_fc_freelist(&old).is_null();
        set_freepointer(s, tail, rust_slub_fc_freelist(&old));
        rust_slub_fc_init(&mut new, head, rust_slub_fc_counters(&old));
        rust_slub_fc_set_inuse(&mut new, rust_slub_fc_inuse(&new).wrapping_sub(cnt as u32));
        if rust_slub_fc_inuse(&new) == 0 || was_full {
            n = get_node(s, rust_slub_slab_nid(slab));
            flags = rust_slub_node_lock_irqsave(n);
            on_node_partial = slab_test_node_partial(slab);
        }
        if slab_update_freelist(
            s,
            slab,
            &mut old,
            &mut new,
            c"__slab_free".as_ptr().cast::<CChar>(),
        ) {
            break;
        }
    }
    if n.is_null() {
        return;
    }
    if !was_full && !on_node_partial {
        rust_slub_node_unlock_irqrestore(n, flags);
        return;
    }
    if rust_slub_fc_inuse(&new) == 0 && (*n).nr_partial >= (*s).min_partial {
        if !was_full {
            remove_partial(n, slab);
            stat(s, FREE_REMOVE_PARTIAL);
        }
        rust_slub_node_unlock_irqrestore(n, flags);
        stat(s, FREE_SLAB);
        discard_slab(s, slab);
        return;
    }
    if was_full {
        add_partial(n, slab, ADD_TO_TAIL);
        stat(s, FREE_ADD_PARTIAL);
    }
    rust_slub_node_unlock_irqrestore(n, flags);
}

unsafe fn __pcs_install_empty_sheaf(
    s: *mut kmem_cache,
    pcs: *mut slub_percpu_sheaves,
    empty: *mut slab_sheaf,
    barn: *mut node_barn,
) {
    rust_slub_pcs_assert_held(s);
    slab_attach_kprobe_locked();
    if (*pcs).spare.is_null() {
        (*pcs).spare = (*pcs).main;
        (*pcs).main = empty;
        return;
    }
    if (*(*pcs).main).size < (*s).sheaf_capacity {
        barn_put_empty_sheaf(barn, empty);
        return;
    }
    if (*(*pcs).spare).size < (*s).sheaf_capacity {
        core::ptr::swap(addr_of_mut!((*pcs).main), addr_of_mut!((*pcs).spare));
        barn_put_empty_sheaf(barn, empty);
        return;
    }
    barn_put_full_sheaf(barn, (*pcs).main);
    stat(s, BARN_PUT);
    (*pcs).main = empty;
}
unsafe fn __pcs_replace_full_main(
    s: *mut kmem_cache,
    mut pcs: *mut slub_percpu_sheaves,
    allow_spin: bool,
) -> *mut slub_percpu_sheaves {
    loop {
        rust_slub_pcs_assert_held(s);
        slab_attach_kprobe_locked();
        if !rust_slub_cache_has_sheaves(s) {
            rust_slub_pcs_unlock(s);
            return null_mut();
        }
        let barn = get_barn(s);
        if barn.is_null() {
            rust_slub_pcs_unlock(s);
            return null_mut();
        }
        let mut put_fail = false;
        let mut empty;
        if (*pcs).spare.is_null() {
            empty = barn_get_empty_sheaf(barn, allow_spin);
            if !empty.is_null() {
                (*pcs).spare = (*pcs).main;
                (*pcs).main = empty;
                return pcs;
            }
        } else {
            if (*(*pcs).spare).size < (*s).sheaf_capacity {
                core::ptr::swap(addr_of_mut!((*pcs).main), addr_of_mut!((*pcs).spare));
                return pcs;
            }
            empty = barn_replace_full_sheaf(barn, (*pcs).main, allow_spin);
            if !rust_slub_is_err(empty.cast()) {
                stat(s, BARN_PUT);
                (*pcs).main = empty;
                return pcs;
            }
            if rust_slub_ptr_err(empty.cast()) == -(RSL_E2BIG as Long) && allow_spin {
                let to_flush = (*pcs).spare;
                stat(s, BARN_PUT_FAIL);
                (*pcs).spare = null_mut();
                rust_slub_pcs_unlock(s);
                sheaf_flush_unused(s, to_flush);
                if !rust_slub_pcs_trylock(s) {
                    barn_put_empty_sheaf(barn, to_flush);
                    return null_mut();
                }
                pcs = rust_slub_this_cpu_sheaves(s);
                __pcs_install_empty_sheaf(s, pcs, to_flush, barn);
                return pcs;
            }
            put_fail = true;
        }
        rust_slub_pcs_unlock(s);
        if !allow_spin {
            return null_mut();
        }
        empty = alloc_empty_sheaf(s, RSL_GFP_NOWAIT, RSL_SLAB_ALLOC_DEFAULT);
        if !empty.is_null() {
            if !rust_slub_pcs_trylock(s) {
                barn_put_empty_sheaf(barn, empty);
                return null_mut();
            }
            pcs = rust_slub_this_cpu_sheaves(s);
            __pcs_install_empty_sheaf(s, pcs, empty, barn);
            return pcs;
        }
        if put_fail {
            stat(s, BARN_PUT_FAIL);
        }
        if !sheaf_try_flush_main(s) || !rust_slub_pcs_trylock(s) {
            return null_mut();
        }
        pcs = rust_slub_this_cpu_sheaves(s);
        if (*(*pcs).main).size != (*s).sheaf_capacity {
            return pcs;
        }
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn free_to_pcs(s: *mut kmem_cache, object: *mut Void, allow_spin: bool) -> bool {
    if !rust_slub_pcs_trylock(s) {
        return false;
    }
    let mut pcs = rust_slub_this_cpu_sheaves(s);
    if (*(*pcs).main).size == (*s).sheaf_capacity {
        pcs = __pcs_replace_full_main(s, pcs, allow_spin);
        if pcs.is_null() {
            return false;
        }
    }
    let main = (*pcs).main;
    *(*main).objects.as_mut_ptr().add((*main).size as usize) = object;
    (*main).size += 1;
    rust_slub_pcs_unlock(s);
    stat(s, FREE_FASTPATH);
    true
}
unsafe extern "C" fn rcu_free_sheaf(head: *mut rcu_head) {
    let sheaf = rust_slub_sheaf_from_rcu(head);
    let s = (*sheaf).cache;
    let mut barn = null_mut();
    let mut flush = __rcu_free_sheaf_prepare(s, sheaf);
    if !flush {
        barn = get_barn_node(s, (*sheaf).node);
        flush = barn.is_null();
        if !flush && (*sheaf).size != 0 {
            if rust_slub_barn_nr_full_racy(barn) < RSL_MAX_FULL_SHEAVES {
                stat(s, BARN_PUT);
                barn_put_full_sheaf(barn, sheaf);
                return;
            }
            flush = true;
        }
    }
    if flush {
        stat(s, BARN_PUT_FAIL);
        sheaf_flush_unused(s, sheaf);
    }
    if !barn.is_null() && rust_slub_barn_nr_empty_racy(barn) < RSL_MAX_EMPTY_SHEAVES {
        barn_put_empty_sheaf(barn, sheaf);
        return;
    }
    free_empty_sheaf(s, sheaf);
}
#[no_mangle]
unsafe extern "C" fn __kfree_rcu_sheaf(
    s: *mut kmem_cache,
    obj: *mut Void,
    free_flags: u32,
) -> bool {
    let allow_spin = rust_slub_free_flags_allow_spinning(free_flags);
    rust_slub_vm_warn_rcu_sheaf_rt(cfg!(CONFIG_PREEMPT_RT) && allow_spin);
    if !cfg!(CONFIG_PREEMPT_RT) {
        rust_slub_rcu_sheaf_map_acquire();
    }
    let success = 'attempt: {
        if !rust_slub_pcs_trylock(s) {
            break 'attempt false;
        }
        let mut pcs = rust_slub_this_cpu_sheaves(s);
        if (*pcs).rcu_free.is_null() {
            let alloc_flags = rust_slub_to_alloc_flags(free_flags);
            let gfp = if allow_spin {
                RSL_GFP_NOWAIT
            } else {
                RSL___GFP_NOWARN
            };
            if !rust_slub_cache_has_sheaves(s) {
                rust_slub_pcs_unlock(s);
                break 'attempt false;
            }
            if !(*pcs).spare.is_null() && (*(*pcs).spare).size == 0 {
                (*pcs).rcu_free = (*pcs).spare;
                (*pcs).spare = null_mut();
            } else {
                let barn = get_barn(s);
                if barn.is_null() {
                    rust_slub_pcs_unlock(s);
                    break 'attempt false;
                }
                let mut empty = barn_get_empty_sheaf(barn, allow_spin);
                if !empty.is_null() {
                    (*pcs).rcu_free = empty;
                } else {
                    rust_slub_pcs_unlock(s);
                    empty = alloc_empty_sheaf(s, gfp, alloc_flags);
                    if empty.is_null() {
                        break 'attempt false;
                    }
                    if !rust_slub_pcs_trylock(s) {
                        __free_empty_sheaf(s, empty, free_flags);
                        break 'attempt false;
                    }
                    pcs = rust_slub_this_cpu_sheaves(s);
                    if !(*pcs).rcu_free.is_null() {
                        __free_empty_sheaf(s, empty, free_flags);
                    } else {
                        (*pcs).rcu_free = empty;
                    }
                }
            }
        }
        let rcu_sheaf = (*pcs).rcu_free;
        *(*rcu_sheaf)
            .objects
            .as_mut_ptr()
            .add((*rcu_sheaf).size as usize) = obj;
        (*rcu_sheaf).size += 1;
        if (*rcu_sheaf).size >= (*s).sheaf_capacity {
            (*pcs).rcu_free = null_mut();
            (*rcu_sheaf).node = rust_slub_numa_node_id();
            if !allow_spin && rust_slub_irqs_disabled() {
                let dpw = rust_slub_this_cpu_deferred_work();
                if rust_slub_llist_add(
                    rust_slub_sheaf_llnode(rcu_sheaf),
                    addr_of_mut!((*dpw).rcu_sheaves),
                ) {
                    rust_slub_irq_work_queue(addr_of_mut!((*dpw).work));
                }
            } else {
                rust_slub_call_rcu(rust_slub_sheaf_rcu(rcu_sheaf), Some(rcu_free_sheaf));
            }
        }
        // Enqueue before dropping local lock, synchronizing flush_all_rcu_sheaves.
        rust_slub_pcs_unlock(s);
        break 'attempt true;
    };
    stat(
        s,
        if success {
            FREE_RCU_SHEAF
        } else {
            FREE_RCU_SHEAF_FAIL
        },
    );
    if !cfg!(CONFIG_PREEMPT_RT) {
        rust_slub_rcu_sheaf_map_release();
    }
    success
}
#[inline(always)]
unsafe fn can_free_to_pcs(slab: *mut slab) -> bool {
    #[cfg(CONFIG_NUMA)]
    {
        let slab_node = rust_slub_slab_nid(slab);
        #[cfg(CONFIG_HAVE_MEMORYLESS_NODES)]
        if slab_node != rust_slub_numa_mem_id() {
            return false;
        }
        #[cfg(not(CONFIG_HAVE_MEMORYLESS_NODES))]
        {
            let local = rust_slub_numa_node_id();
            if slab_node != local && rust_slub_node_has_normal_memory(local) {
                return false;
            }
        }
    }
    !slab_test_pfmemalloc(slab)
}

unsafe fn __free_to_pcs_batch(s: *mut kmem_cache, size: usize, p: *mut *mut Void) -> u32 {
    if !rust_slub_pcs_trylock(s) {
        return 0;
    }
    let pcs = rust_slub_this_cpu_sheaves(s);
    if (*(*pcs).main).size == (*s).sheaf_capacity {
        let barn = get_barn(s);
        if barn.is_null() {
            rust_slub_pcs_unlock(s);
            return 0;
        }
        if (*pcs).spare.is_null() {
            let empty = barn_get_empty_sheaf(barn, true);
            if empty.is_null() {
                rust_slub_pcs_unlock(s);
                return 0;
            }
            (*pcs).spare = (*pcs).main;
            (*pcs).main = empty;
        } else if (*(*pcs).spare).size < (*s).sheaf_capacity {
            core::ptr::swap(addr_of_mut!((*pcs).main), addr_of_mut!((*pcs).spare));
        } else {
            let empty = barn_replace_full_sheaf(barn, (*pcs).main, true);
            if rust_slub_is_err(empty.cast()) {
                stat(s, BARN_PUT_FAIL);
                rust_slub_pcs_unlock(s);
                return 0;
            }
            stat(s, BARN_PUT);
            (*pcs).main = empty;
        }
    }
    let main = (*pcs).main;
    let batch = min(size, ((*s).sheaf_capacity - (*main).size) as usize);
    core::ptr::copy_nonoverlapping(
        p,
        (*main).objects.as_mut_ptr().add((*main).size as usize),
        batch,
    );
    (*main).size += batch as u32;
    rust_slub_pcs_unlock(s);
    stat_add(s, FREE_FASTPATH, batch as i32);
    batch as u32
}
unsafe fn free_to_pcs_bulk(s: *mut kmem_cache, mut size: usize, mut p: *mut *mut Void) {
    let init = rust_slub_slab_want_init_on_free(s);
    let remote_objects = p;
    let mut remote_nr = 0usize;
    let mut i = 0usize;
    while i < size {
        let slab = rust_slub_virt_to_slab(*p.add(i));
        memcg_slab_free_hook(s, slab, p.add(i), 1);
        alloc_tagging_slab_free_hook(s, slab, p.add(i), 1);
        if !slab_free_hook(s, *p.add(i), init, false) {
            size -= 1;
            *p.add(i) = *p.add(size);
            continue;
        }
        if !can_free_to_pcs(slab) {
            if i != remote_nr {
                core::ptr::swap(remote_objects.add(remote_nr), p.add(i));
            }
            remote_nr += 1;
        }
        i += 1;
    }
    p = p.add(remote_nr);
    size -= remote_nr;
    while size != 0 {
        let batch = __free_to_pcs_batch(s, size, p) as usize;
        if batch == 0 {
            __kmem_cache_free_bulk(s, size, p);
            stat_add(s, FREE_SLOWPATH, size as i32);
            break;
        }
        p = p.add(batch);
        size -= batch;
    }
    if remote_nr != 0 {
        __kmem_cache_free_bulk(s, remote_nr, remote_objects);
        stat_add(s, FREE_SLOWPATH, remote_nr as i32);
    }
}
#[no_mangle]
unsafe extern "C" fn deferred_percpu_work_fn(work: *mut irq_work) {
    let dpw = rust_slub_deferred_from_work(work);
    let mut pos = rust_slub_llist_del_all(addr_of_mut!((*dpw).objects));
    while !pos.is_null() {
        let next = (*pos).next;
        let slab = rust_slub_virt_to_slab(pos.cast());
        let s = (*slab).slab_cache;
        let x = pos.cast::<u8>().sub((*s).offset as usize).cast();
        set_freepointer(s, x, null_mut());
        rust_slub_free_one_at_current_ip(s, slab, x);
        stat(s, FREE_SLOWPATH);
        pos = next;
    }
    pos = rust_slub_llist_del_all(addr_of_mut!((*dpw).objects_by_rcu));
    while !pos.is_null() {
        let next = (*pos).next;
        let objp = rust_slub_kvmalloc_obj_start_addr(pos.cast());
        rust_slub_kvfree_call_rcu(pos.cast(), objp);
        pos = next;
    }
    pos = rust_slub_llist_del_all(addr_of_mut!((*dpw).rcu_sheaves));
    while !pos.is_null() {
        let next = (*pos).next;
        let sheaf = rust_slub_sheaf_from_llnode(pos);
        rust_slub_call_rcu(rust_slub_sheaf_rcu(sheaf), Some(rcu_free_sheaf));
        pos = next;
    }
}
unsafe fn defer_free(s: *mut kmem_cache, head: *mut Void) {
    rust_slub_preempt_disable();
    let head = rust_slub_kasan_reset_tag(head);
    let dpw = rust_slub_this_cpu_deferred_work();
    if rust_slub_llist_add(
        head.cast::<u8>().add((*s).offset as usize).cast(),
        addr_of_mut!((*dpw).objects),
    ) {
        rust_slub_irq_work_queue(addr_of_mut!((*dpw).work));
    }
    rust_slub_preempt_enable();
}
#[no_mangle]
unsafe extern "C" fn defer_kfree_rcu(head: *mut kvfree_rcu_head) {
    rust_slub_preempt_disable();
    let dpw = rust_slub_this_cpu_deferred_work();
    if rust_slub_llist_add(head.cast(), addr_of_mut!((*dpw).objects_by_rcu)) {
        rust_slub_irq_work_queue(addr_of_mut!((*dpw).work));
    }
    rust_slub_preempt_enable();
}
#[no_mangle]
unsafe extern "C" fn deferred_work_barrier() {
    let mut cpu = rust_slub_first_possible_cpu();
    while cpu < rust_slub_nr_cpu_ids() {
        let dpw = rust_slub_percpu_deferred_work(cpu);
        rust_slub_irq_work_sync(addr_of_mut!((*dpw).work));
        cpu = rust_slub_next_possible_cpu(cpu + 1);
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_free(s: *mut kmem_cache, slab: *mut slab, mut object: *mut Void, addr: ULong) {
    memcg_slab_free_hook(s, slab, &mut object, 1);
    alloc_tagging_slab_free_hook(s, slab, &mut object, 1);
    if !slab_free_hook(s, object, rust_slub_slab_want_init_on_free(s), false) {
        return;
    }
    if can_free_to_pcs(slab) && free_to_pcs(s, object, true) {
        return;
    }
    __slab_free(s, slab, object, object, 1, addr);
    stat(s, FREE_SLOWPATH);
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
unsafe extern "C" fn rust_slub_memcg_alloc_abort_single_body(
    s: *mut kmem_cache,
    mut object: *mut Void,
    caller: ULong,
) {
    let slab = rust_slub_virt_to_slab(object);
    alloc_tagging_slab_free_hook(s, slab, &mut object, 1);
    if slab_free_hook(s, object, rust_slub_slab_want_init_on_free(s), false) {
        __slab_free(s, slab, object, object, 1, caller);
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_free_bulk(
    s: *mut kmem_cache,
    slab: *mut slab,
    mut head: *mut Void,
    mut tail: *mut Void,
    p: *mut *mut Void,
    mut cnt: i32,
    addr: ULong,
) {
    memcg_slab_free_hook(s, slab, p, cnt);
    alloc_tagging_slab_free_hook(s, slab, p, cnt);
    if slab_free_freelist_hook(s, &mut head, &mut tail, &mut cnt) {
        __slab_free(s, slab, head, tail, cnt, addr);
        stat_add(s, FREE_SLOWPATH, cnt);
    }
}
#[cfg(CONFIG_SLUB_RCU_DEBUG)]
unsafe extern "C" fn slab_free_after_rcu_debug(head: *mut rcu_head) {
    let delayed = head
        .cast::<u8>()
        .sub(offset_of!(rcu_delayed_free, head))
        .cast::<rcu_delayed_free>();
    let object = (*delayed).object;
    let slab = rust_slub_virt_to_slab(object);
    kfree(delayed.cast());
    if rust_slub_warn(rust_slub_is_kfence_address(object)) || rust_slub_warn(slab.is_null()) {
        return;
    }
    let s = (*slab).slab_cache;
    if rust_slub_warn((*s).flags & RSL_SLAB_TYPESAFE_BY_RCU == 0) {
        return;
    }
    if slab_free_hook(s, object, rust_slub_slab_want_init_on_free(s), true) {
        rust_slub_free_one_at_current_ip(s, slab, object);
        stat(s, FREE_SLOWPATH);
    }
}
#[cfg(CONFIG_KASAN_GENERIC)]
#[no_mangle]
unsafe extern "C" fn ___cache_free(cache: *mut kmem_cache, x: *mut Void, addr: ULong) {
    __slab_free(cache, rust_slub_virt_to_slab(x), x, x, 1, addr);
    stat(cache, FREE_SLOWPATH);
}
#[inline(never)]
unsafe fn warn_free_bad_obj(s: *mut kmem_cache, obj: *mut Void) {
    let slab = rust_slub_virt_to_slab(obj);
    if rust_slub_warn_not_slab(s, obj, slab.is_null()) {
        return;
    }
    let cachep = (*slab).slab_cache;
    if rust_slub_warn_wrong_cache(s, obj, cachep, cachep != s) && !cachep.is_null() {
        print_tracking(cachep, obj);
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_free_body(
    s: *mut kmem_cache,
    x: *mut Void,
    caller: ULong,
) {
    let slab = rust_slub_virt_to_slab(x);
    if cfg!(CONFIG_SLAB_FREELIST_HARDENED)
        || rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_CONSISTENCY_CHECKS)
    {
        if slab.is_null() || (*slab).slab_cache != s {
            warn_free_bad_obj(s, x);
            return;
        }
    }
    rust_slub_trace_cache_free(caller, x, s);
    slab_free(s, slab, x, caller);
}
#[inline]
unsafe fn slab_ksize(slab: *mut slab) -> usize {
    let s = (*slab).slab_cache;
    #[cfg(CONFIG_SLUB_DEBUG)]
    if (*s).flags & (RSL_SLAB_RED_ZONE | RSL_SLAB_POISON) != 0 {
        return (*s).object_size as usize;
    }
    if (*s).flags & RSL_SLAB_KASAN != 0 {
        return (*s).object_size as usize;
    }
    if (*s).flags & (RSL_SLAB_TYPESAFE_BY_RCU | RSL_SLAB_STORE_USER) != 0
        || rust_slub_obj_exts_in_object(slab)
    {
        return (*s).inuse as usize;
    }
    (*s).size as usize
}
unsafe fn __ksize(object: *const Void) -> usize {
    if object as usize == RSL_ZERO_SIZE_PTR as usize {
        return 0;
    }
    let page = rust_slub_virt_to_page(object);
    if rust_slub_page_large_kmalloc(page) {
        return rust_slub_large_kmalloc_size(page);
    }
    let slab = rust_slub_page_slab(page);
    if rust_slub_warn(slab.is_null()) {
        return rust_slub_page_size(page);
    }
    #[cfg(CONFIG_SLUB_DEBUG)]
    skip_orig_size_check((*slab).slab_cache, object);
    slab_ksize(slab)
}
#[no_mangle]
unsafe extern "C" fn ksize(objp: *const Void) -> usize {
    if rust_slub_zero_or_null_ptr(objp) || !rust_slub_kasan_check_byte(objp) {
        return 0;
    }
    let kf = rust_slub_kfence_ksize(objp);
    if kf != 0 {
        kf
    } else {
        __ksize(objp)
    }
}
unsafe fn free_large_kmalloc(page: *mut page, object: *mut Void) {
    let order = rust_slub_compound_order(page);
    if rust_slub_warn_once_not_large(!rust_slub_page_large_kmalloc(page)) {
        rust_slub_dump_page(page, c"Not a kmalloc allocation".as_ptr().cast::<CChar>());
        return;
    }
    if rust_slub_warn_once_zero_order(order == 0) {
        rust_slub_warn_object_pointer(object);
    }
    rust_slub_kmemleak_free(object);
    rust_slub_kasan_kfree_large(object);
    rust_slub_kmsan_kfree_large(object);
    rust_slub_mod_lruvec_slab_bytes(page, -((RSL_PAGE_SIZE as Long) << order));
    rust_slub_clear_page_large_kmalloc(page);
    rust_slub_free_frozen_pages(page, order);
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kvfree_rcu_cb_body(head: *mut rcu_head, caller: ULong) {
    let obj = rust_slub_kvmalloc_obj_start_addr(head.cast());
    if rust_slub_is_vmalloc_addr(obj) {
        vfree(obj);
    } else {
        let page = rust_slub_virt_to_page(obj);
        let slab = rust_slub_page_slab(page);
        if !slab.is_null() {
            slab_free((*slab).slab_cache, slab, obj, caller);
        } else {
            free_large_kmalloc(page, obj);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kfree_body(object: *const Void, caller: ULong) {
    rust_slub_trace_kfree(caller, object);
    if rust_slub_zero_or_null_ptr(object) {
        return;
    }
    let page = rust_slub_virt_to_page(object);
    let slab = rust_slub_page_slab(page);
    if slab.is_null() {
        free_large_kmalloc(page, object.cast_mut());
        return;
    }
    slab_free((*slab).slab_cache, slab, object.cast_mut(), caller);
}
#[no_mangle]
unsafe extern "C" fn kfree_nolock(object: *const Void) {
    if rust_slub_zero_or_null_ptr(object) {
        return;
    }
    let slab = rust_slub_virt_to_slab(object);
    if slab.is_null() {
        rust_slub_warn_large_nolock();
        return;
    }
    let s = (*slab).slab_cache;
    let mut x = object.cast_mut();
    memcg_slab_free_hook(s, slab, &mut x, 1);
    alloc_tagging_slab_free_hook(s, slab, &mut x, 1);
    rust_slub_kmsan_slab_free(s, x);
    if rust_slub_kasan_slab_pre_free(s, x) {
        return;
    }
    rust_slub_kasan_slab_free(s, x, false, false, true);
    if can_free_to_pcs(slab) && free_to_pcs(s, x, false) {
        return;
    }
    defer_free(s, x);
}

#[inline(always)]
unsafe fn __do_krealloc(
    p: *const Void,
    new_size: usize,
    align: ULong,
    flags: gfp_t,
    nid: i32,
    token: kmalloc_token_t,
    caller: ULong,
) -> *mut Void {
    let mut ks = 0usize;
    let mut orig_size = 0usize;
    let mut s: *mut kmem_cache = null_mut();
    let mut reuse = !rust_slub_zero_or_null_ptr(p);
    if reuse {
        if !rust_slub_kasan_check_byte(p) {
            return null_mut();
        }
        if rust_slub_is_kfence_address(p) {
            ks = rust_slub_kfence_ksize(p);
            orig_size = ks;
        } else {
            let page = rust_slub_virt_to_page(p);
            let slab = rust_slub_page_slab(page);
            if slab.is_null() {
                ks = rust_slub_page_size(page);
                rust_slub_warn(ks <= RSL_KMALLOC_MAX_CACHE_SIZE as usize);
                rust_slub_warn(p != rust_slub_page_address(page));
            } else {
                s = (*slab).slab_cache;
                orig_size = get_orig_size(s, p.cast_mut()) as usize;
                ks = (*s).object_size as usize;
            }
        }
        reuse = !(flags & RSL___GFP_THISNODE != 0
            && nid != RSL_NUMA_NO_NODE
            && nid != rust_slub_page_to_nid(rust_slub_virt_to_page(p)))
            && new_size <= ks
            && (p as ULong & (align - 1)) == 0;
    }
    if reuse {
        if rust_slub_want_init_on_alloc(flags) {
            rust_slub_kasan_disable_current();
            let tagless = rust_slub_kasan_reset_tag(p).cast::<u8>();
            if orig_size != 0 && orig_size < new_size {
                core::ptr::write_bytes(tagless.add(orig_size), 0, new_size - orig_size);
            } else {
                core::ptr::write_bytes(tagless.add(new_size), 0, ks - new_size);
            }
            rust_slub_kasan_enable_current();
        }
        if !s.is_null() && rust_slub_slub_debug_orig_size(s) {
            set_orig_size(s, p.cast_mut(), new_size as ULong);
            if (*s).flags & RSL_SLAB_RED_ZONE != 0 && new_size < ks {
                rust_slub_memset_no_sanitize(
                    rust_slub_kasan_reset_tag(p)
                        .cast::<u8>()
                        .add(new_size)
                        .cast(),
                    RSL_SLUB_RED_ACTIVE as i32,
                    ks - new_size,
                );
            }
        }
        return rust_slub_kasan_krealloc(p, new_size, flags);
    }
    let ret = rust_slub_kmalloc_node_track_caller_noprof_body(
        new_size,
        null_mut(),
        token,
        flags,
        nid,
        caller,
    );
    if !ret.is_null() && !p.is_null() {
        rust_slub_kasan_disable_current();
        rust_slub_memcpy(
            ret,
            rust_slub_kasan_reset_tag(p),
            min(new_size, if orig_size != 0 { orig_size } else { ks }),
        );
        rust_slub_kasan_enable_current();
    }
    ret
}
#[no_mangle]
unsafe extern "C" fn rust_slub_krealloc_node_align_noprof_body(
    p: *const Void,
    new_size: usize,
    token: kmalloc_token_t,
    align: ULong,
    flags: gfp_t,
    nid: i32,
    caller: ULong,
) -> *mut Void {
    if new_size == 0 {
        kfree(p);
        return RSL_ZERO_SIZE_PTR as usize as *mut Void;
    }
    let ret = __do_krealloc(p, new_size, align, flags, nid, token, caller);
    if !ret.is_null() && rust_slub_kasan_reset_tag(p) != rust_slub_kasan_reset_tag(ret) {
        kfree(p);
    }
    ret
}
unsafe fn kmalloc_gfp_adjust(mut flags: gfp_t, size: usize) -> gfp_t {
    if size > RSL_PAGE_SIZE as usize {
        flags |= RSL___GFP_NOWARN;
        if flags & RSL___GFP_RETRY_MAYFAIL == 0 {
            flags &= !RSL___GFP_DIRECT_RECLAIM;
        }
        flags &= !RSL___GFP_NOFAIL;
    }
    flags
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kvmalloc_node_noprof_body(
    size: usize,
    b: *mut kmem_buckets,
    token: kmalloc_token_t,
    align: ULong,
    flags: gfp_t,
    node: i32,
    caller: ULong,
) -> *mut Void {
    let ac = alloc_context(null_mut(), caller, size, RSL_SLAB_ALLOC_DEFAULT);
    let ret = __do_kmalloc_node(b, kmalloc_gfp_adjust(flags, size), node, token, &ac);
    if !ret.is_null() || size <= RSL_PAGE_SIZE as usize {
        return ret;
    }
    if size > RSL_INT_MAX as usize {
        rust_slub_warn_large_size(flags & RSL___GFP_NOWARN == 0);
        return null_mut();
    }
    let allow_block = rust_slub_gfpflags_allow_blocking(flags);
    rust_slub_vmalloc_node_range(size, align, flags, allow_block, node, caller as *const Void)
}
#[no_mangle]
unsafe extern "C" fn kvfree(addr: *const Void) {
    if rust_slub_is_vmalloc_addr(addr) {
        vfree(addr);
    } else {
        kfree(addr);
    }
}
#[no_mangle]
unsafe extern "C" fn kvfree_atomic(addr: *const Void) {
    if rust_slub_is_vmalloc_addr(addr) {
        vfree_atomic(addr);
    } else {
        kfree(addr);
    }
}
#[no_mangle]
unsafe extern "C" fn kvfree_sensitive(addr: *const Void, len: usize) {
    if !rust_slub_zero_or_null_ptr(addr) {
        rust_slub_memzero_explicit(addr.cast_mut(), len);
        kvfree(addr);
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kvrealloc_node_align_noprof_body(
    p: *const Void,
    size: usize,
    token: kmalloc_token_t,
    align: ULong,
    flags: gfp_t,
    nid: i32,
    _caller: ULong,
) -> *mut Void {
    if rust_slub_is_vmalloc_addr(p) {
        return rust_slub_vrealloc_node_align(p, size, align, flags, nid);
    }
    let mut n =
        rust_slub_call_krealloc(p, size, token, align, kmalloc_gfp_adjust(flags, size), nid);
    if n.is_null() {
        n = rust_slub_call_kvmalloc(size, token, align, flags, nid);
        if n.is_null() {
            return null_mut();
        }
        if !p.is_null() {
            rust_slub_kasan_disable_current();
            rust_slub_memcpy(n, rust_slub_kasan_reset_tag(p), min(size, ksize(p)));
            rust_slub_kasan_enable_current();
            kfree(p);
        }
    }
    n
}

// Private Rust-only scratch state; never passed across the C ABI.
struct detached_freelist {
    slab: *mut slab,
    tail: *mut Void,
    freelist: *mut Void,
    cnt: i32,
    s: *mut kmem_cache,
}
#[inline]
unsafe fn build_detached_freelist(
    s: *mut kmem_cache,
    mut size: usize,
    p: *mut *mut Void,
    df: *mut detached_freelist,
) -> i32 {
    let mut lookahead = 3;
    size -= 1;
    let mut object = *p.add(size);
    let page = rust_slub_virt_to_page(object);
    let slab = rust_slub_page_slab(page);
    if s.is_null() {
        if slab.is_null() {
            free_large_kmalloc(page, object);
            (*df).slab = null_mut();
            return size as i32;
        }
        (*df).slab = slab;
        (*df).s = (*slab).slab_cache;
    } else {
        (*df).slab = slab;
        (*df).s = s;
    }
    (*df).tail = object;
    (*df).freelist = object;
    (*df).cnt = 1;
    if rust_slub_is_kfence_address(object) {
        return size as i32;
    }
    set_freepointer((*df).s, object, null_mut());
    let mut same = size;
    while size != 0 {
        size -= 1;
        object = *p.add(size);
        if (*df).slab == rust_slub_virt_to_slab(object) {
            set_freepointer((*df).s, object, (*df).freelist);
            (*df).freelist = object;
            (*df).cnt += 1;
            same -= 1;
            if size != same {
                core::ptr::swap(p.add(size), p.add(same));
            }
            continue;
        }
        lookahead -= 1;
        if lookahead == 0 {
            break;
        }
    }
    same as i32
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_free_bulk_internal_body(
    s: *mut kmem_cache,
    mut size: usize,
    p: *mut *mut Void,
    caller: ULong,
) {
    while size != 0 {
        let mut df: detached_freelist = zeroed();
        size = build_detached_freelist(s, size, p, &mut df) as usize;
        if df.slab.is_null() || rust_slub_kfence_free(df.freelist) {
            continue;
        }
        __slab_free(df.s, df.slab, df.freelist, df.tail, df.cnt, caller);
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_free_bulk_body(
    s: *mut kmem_cache,
    mut size: usize,
    p: *mut *mut Void,
    caller: ULong,
) {
    if size == 0 {
        return;
    }
    if !s.is_null() && rust_slub_cache_has_sheaves(s) {
        free_to_pcs_bulk(s, size, p);
        return;
    }
    while size != 0 {
        let mut df: detached_freelist = zeroed();
        size = build_detached_freelist(s, size, p, &mut df) as usize;
        if df.slab.is_null() {
            continue;
        }
        slab_free_bulk(
            df.s,
            df.slab,
            df.freelist,
            df.tail,
            p.add(size),
            df.cnt,
            caller,
        );
    }
}

#[no_mangle]
unsafe extern "C" fn rust_slub_refill_objects_node_body(
    s: *mut kmem_cache,
    p: *mut *mut Void,
    gfp: gfp_t,
    minimum: u32,
    maximum: u32,
    n: *mut kmem_cache_node,
    allow_spin: bool,
    caller: ULong,
) -> u32 {
    let mut pc: partial_bulk_context = zeroed();
    pc.flags = gfp;
    pc.min_objects = minimum;
    pc.max_objects = maximum;
    if !get_partial_node_bulk(s, n, &mut pc, allow_spin) {
        return 0;
    }
    let head = addr_of_mut!(pc.slabs);
    let mut refilled = 0;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let slab = rust_slub_slab_from_list(pos);
        rust_slub_list_del(pos);
        let mut count = 0;
        let mut object = get_freelist_nofreeze(s, slab, &mut count);
        while count != 0 && refilled < maximum {
            *p.add(refilled as usize) = object;
            object = get_freepointer(s, object);
            maybe_wipe_obj_freeptr(s, *p.add(refilled as usize));
            refilled += 1;
            count -= 1;
        }
        if count != 0 {
            let objects = object;
            if __slab_try_return_freelist(s, slab, objects, count as i32) {
                rust_slub_list_add(pos, head);
                break;
            }
            let mut tail;
            loop {
                tail = object;
                object = get_freepointer(s, object);
                if object.is_null() {
                    break;
                }
            }
            __slab_free(s, slab, objects, tail, count as i32, caller);
        }
        if refilled >= maximum {
            break;
        }
        pos = next;
    }
    if !rust_slub_list_empty(head) {
        let flags = rust_slub_node_lock_irqsave(n);
        pos = (*head).next;
        while pos != head {
            set_node_partial_state(n, rust_slub_slab_from_list(pos));
            pos = (*pos).next;
        }
        rust_slub_list_splice_tail(head, addr_of_mut!((*n).partial));
        rust_slub_node_unlock_irqrestore(n, flags);
    }
    refilled
}
#[cfg(CONFIG_NUMA)]
unsafe fn __refill_objects_any(
    s: *mut kmem_cache,
    mut p: *mut *mut Void,
    gfp: gfp_t,
    mut minimum: u32,
    mut maximum: u32,
) -> u32 {
    let highest_zoneidx = rust_slub_gfp_zone(gfp);
    let mut refilled = 0;
    if (*s).remote_node_defrag_ratio == 0
        || rust_slub_get_cycles() % 1024 > (*s).remote_node_defrag_ratio as ULong
    {
        return 0;
    }
    loop {
        let cookie = rust_slub_read_mems_allowed_begin();
        let zonelist = rust_slub_node_zonelist(rust_slub_mempolicy_slab_node(), gfp);
        let mut z = rust_slub_first_zoneref(zonelist, highest_zoneidx);
        loop {
            let zone = rust_slub_zonelist_zone(z);
            if zone.is_null() {
                break;
            }
            let n = get_node(s, rust_slub_zone_to_nid(zone));
            if !n.is_null()
                && rust_slub_cpuset_zone_allowed(zone, gfp)
                && (*n).nr_partial > (*s).min_partial
            {
                let r = __refill_objects_node(s, p, gfp, minimum, maximum, n, false);
                refilled += r;
                if r >= minimum {
                    return refilled;
                }
                p = p.add(r as usize);
                minimum -= r;
                maximum -= r;
            }
            z = rust_slub_next_zoneref(z, highest_zoneidx);
        }
        if !rust_slub_read_mems_allowed_retry(cookie) {
            return refilled;
        }
    }
}
#[cfg(not(CONFIG_NUMA))]
unsafe fn __refill_objects_any(
    _s: *mut kmem_cache,
    _p: *mut *mut Void,
    _gfp: gfp_t,
    _minimum: u32,
    _maximum: u32,
) -> u32 {
    0
}
unsafe fn refill_objects(
    s: *mut kmem_cache,
    p: *mut *mut Void,
    gfp: gfp_t,
    minimum: u32,
    maximum: u32,
) -> u32 {
    let local = rust_slub_numa_mem_id();
    let mut refilled = __refill_objects_node(s, p, gfp, minimum, maximum, get_node(s, local), true);
    if refilled >= minimum {
        return refilled;
    }
    refilled += __refill_objects_any(
        s,
        p.add(refilled as usize),
        gfp,
        minimum - refilled,
        maximum - refilled,
    );
    if refilled >= minimum {
        return refilled;
    }
    loop {
        let slab = new_slab(s, gfp, RSL_SLAB_ALLOC_DEFAULT, local);
        if slab.is_null() {
            return refilled;
        }
        stat(s, ALLOC_SLAB);
        refilled +=
            alloc_from_new_slab(s, slab, p.add(refilled as usize), maximum - refilled, true);
        if refilled >= minimum {
            return refilled;
        }
    }
}
#[no_mangle]
unsafe extern "C" fn rust_slub_kmem_cache_alloc_bulk_internal_body(
    s: *mut kmem_cache,
    flags: gfp_t,
    size: usize,
    p: *mut *mut Void,
    caller: ULong,
) -> bool {
    let mut i = 0usize;
    if cfg!(CONFIG_SLUB_TINY) || kmem_cache_debug(s) {
        let ac = alloc_context(s, caller, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
        while i < size {
            *p.add(i) = ___slab_alloc(s, flags, RSL_NUMA_NO_NODE, &ac);
            if (*p.add(i)).is_null() {
                __kmem_cache_free_bulk(s, i, p);
                return false;
            }
            maybe_wipe_obj_freeptr(s, *p.add(i));
            i += 1;
        }
    } else {
        i = refill_objects(s, p, flags, size as u32, size as u32) as usize;
        if i < size {
            __kmem_cache_free_bulk(s, i, p);
            return false;
        }
        stat_add(s, ALLOC_SLOWPATH, i as i32);
    }
    true
}
#[no_mangle]
unsafe extern "C" fn kmem_cache_alloc_bulk_noprof(
    mut s: *mut kmem_cache,
    flags: gfp_t,
    mut size: usize,
    p: *mut *mut Void,
) -> bool {
    let ac = alloc_context(s, 0, (*s).object_size as usize, RSL_SLAB_ALLOC_DEFAULT);
    if size == 0 {
        return false;
    }
    s = slab_pre_alloc_hook(s, flags);
    if s.is_null() {
        return false;
    }
    let kfence_obj = rust_slub_kfence_alloc(s, (*s).object_size as usize, flags);
    if !kfence_obj.is_null() {
        if size == 1 {
            *p = kfence_obj;
            return slab_post_alloc_hook(s, flags, size, p, &ac);
        }
        size -= 1;
    }
    let i = alloc_from_pcs_bulk(s, size, p) as usize;
    if i < size && !__kmem_cache_alloc_bulk(s, flags, size - i, p.add(i)) {
        if i > 0 {
            __kmem_cache_free_bulk(s, i, p);
        }
        if !kfence_obj.is_null() {
            rust_slub_kfence_free_direct(kfence_obj);
        }
        return false;
    }
    if !kfence_obj.is_null() {
        let idx = rust_slub_get_random_u32_below((size + 1) as u32) as usize;
        if idx != size {
            *p.add(size) = *p.add(idx);
        }
        *p.add(idx) = kfence_obj;
        size += 1;
    }
    slab_post_alloc_hook(s, flags, size, p, &ac)
}

#[no_mangle]
static mut slub_min_order: u32 = 0;
#[no_mangle]
static mut slub_max_order: u32 = if cfg!(CONFIG_SLUB_TINY) {
    1
} else {
    RSL_PAGE_ALLOC_COSTLY_ORDER
};
#[no_mangle]
static mut slub_min_objects: u32 = 0;
#[inline]
unsafe fn calc_slab_order(size: u32, minimum: u32, maximum: u32, fract_leftover: u32) -> u32 {
    let mut order = minimum;
    while order <= maximum {
        let slab_size = (RSL_PAGE_SIZE as u32) << order;
        if slab_size % size <= slab_size / fract_leftover {
            break;
        }
        order += 1;
    }
    order
}
#[inline]
unsafe fn calculate_order(size: u32) -> i32 {
    let mut min_objects = slub_min_objects;
    if min_objects == 0 {
        let mut cpus = rust_slub_num_present_cpus();
        if cpus <= 1 {
            cpus = rust_slub_nr_cpu_ids();
        }
        min_objects = 4 * (rust_slub_fls(cpus) + 1);
    }
    min_objects = min(min_objects, max(order_objects(slub_max_order, size), 1));
    let min_order = max(
        slub_min_order,
        rust_slub_get_order((min_objects * size) as usize),
    );
    if order_objects(min_order, size) > RSL_MAX_OBJS_PER_PAGE {
        return rust_slub_get_order((size * RSL_MAX_OBJS_PER_PAGE) as usize) as i32 - 1;
    }
    let mut fraction = 16;
    while fraction > 1 {
        let order = calc_slab_order(size, min_order, slub_max_order, fraction);
        if order <= slub_max_order {
            return order as i32;
        }
        fraction /= 2;
    }
    let order = rust_slub_get_order(size as usize);
    if order <= RSL_MAX_PAGE_ORDER {
        order as i32
    } else {
        -(RSL_ENOSYS as i32)
    }
}
unsafe fn init_kmem_cache_node(n: *mut kmem_cache_node) {
    (*n).nr_partial = 0;
    rust_slub_node_lock_init(n);
    rust_slub_init_list_head(addr_of_mut!((*n).partial));
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        rust_slub_atomic_long_set(addr_of_mut!((*n).nr_slabs), 0);
        rust_slub_atomic_long_set(addr_of_mut!((*n).total_objects), 0);
        rust_slub_init_list_head(addr_of_mut!((*n).full));
    }
}
#[cfg(CONFIG_SLUB_STATS)]
unsafe fn alloc_kmem_cache_stats(s: *mut kmem_cache) -> i32 {
    (*s).cpu_stats = rust_slub_alloc_percpu_stats();
    (!(*s).cpu_stats.is_null()) as i32
}
unsafe fn init_percpu_sheaves(s: *mut kmem_cache) -> i32 {
    let mut cpu = rust_slub_first_possible_cpu();
    while cpu < rust_slub_nr_cpu_ids() {
        let pcs = rust_slub_percpu_sheaves(s, cpu);
        rust_slub_pcs_lock_init(pcs);
        (*pcs).main = if (*s).sheaf_capacity == 0 {
            rust_slub_bootstrap_sheaf()
        } else {
            alloc_empty_sheaf(s, RSL_GFP_KERNEL, RSL_SLAB_ALLOC_DEFAULT)
        };
        if (*pcs).main.is_null() {
            return -(RSL_ENOMEM as i32);
        }
        cpu = rust_slub_next_possible_cpu(cpu + 1);
    }
    0
}
static mut kmem_cache_node: *mut kmem_cache = null_mut();
unsafe fn early_kmem_cache_node_alloc(node: i32) {
    rust_slub_bug_on((*kmem_cache_node).size < size_of::<kmem_cache_node>() as u32);
    let slab = new_slab(
        kmem_cache_node,
        RSL_GFP_NOWAIT,
        RSL_SLAB_ALLOC_DEFAULT,
        node,
    );
    rust_slub_bug_on(slab.is_null());
    if rust_slub_slab_nid(slab) != node {
        rust_slub_report_early_node_fallback(node);
    }
    let mut iter: slab_obj_iter = zeroed();
    init_slab_obj_iter(kmem_cache_node, slab, &mut iter, true);
    let mut n = next_slab_obj(kmem_cache_node, &mut iter).cast::<kmem_cache_node>();
    rust_slub_bug_on(n.is_null());
    rust_slub_slab_set_inuse(slab, 1);
    build_slab_freelist(kmem_cache_node, slab, &mut iter);
    #[cfg(CONFIG_SLUB_DEBUG)]
    init_object(kmem_cache_node, n.cast(), RSL_SLUB_RED_ACTIVE as u8);
    n = rust_slub_kasan_slab_alloc(kmem_cache_node, n.cast(), RSL_GFP_KERNEL, false).cast();
    (*kmem_cache_node)
        .per_node
        .as_mut_ptr()
        .add(node as usize)
        .as_mut()
        .unwrap_unchecked()
        .node = n;
    init_kmem_cache_node(n);
    inc_slabs_node(kmem_cache_node, node, rust_slub_slab_objects(slab) as i32);
    __add_partial(n, slab, ADD_TO_HEAD);
}
unsafe fn free_kmem_cache_nodes(s: *mut kmem_cache) {
    let mut node = rust_slub_first_node();
    while node < rust_slub_max_num_nodes() {
        let barn = get_barn_node(s, node);
        if !barn.is_null() {
            rust_slub_warn((*barn).nr_full != 0);
            rust_slub_warn((*barn).nr_empty != 0);
            kfree(barn.cast());
            (*(*s).per_node.as_mut_ptr().add(node as usize)).barn = null_mut();
        }
        node = rust_slub_next_node(node);
    }
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            (*(*s).per_node.as_mut_ptr().add(node as usize)).node = null_mut();
            kmem_cache_free(kmem_cache_node, n.cast());
        }
        node += 1;
    }
}
#[no_mangle]
unsafe extern "C" fn __kmem_cache_release(s: *mut kmem_cache) {
    cache_random_seq_destroy(s);
    pcs_destroy(s);
    #[cfg(CONFIG_SLUB_STATS)]
    rust_slub_free_percpu((*s).cpu_stats.cast());
    free_kmem_cache_nodes(s);
}
unsafe fn init_kmem_cache_nodes(s: *mut kmem_cache) -> i32 {
    let mut node = rust_slub_first_node_mask(addr_of!(slab_nodes));
    while node < rust_slub_max_num_nodes() {
        if slab_state == DOWN {
            early_kmem_cache_node_alloc(node);
        } else {
            let n = rust_slub_cache_alloc_node(kmem_cache_node, RSL_GFP_KERNEL, node)
                .cast::<kmem_cache_node>();
            if n.is_null() {
                return 0;
            }
            init_kmem_cache_node(n);
            (*(*s).per_node.as_mut_ptr().add(node as usize)).node = n;
        }
        node = rust_slub_next_node_mask(node, addr_of!(slab_nodes));
    }
    if slab_state == DOWN || !rust_slub_cache_has_sheaves(s) {
        return 1;
    }
    node = rust_slub_first_node_mask(addr_of!(slab_barn_nodes));
    while node < rust_slub_max_num_nodes() {
        let barn = rust_slub_alloc_barn_init_nodes(node).cast::<node_barn>();
        if barn.is_null() {
            return 0;
        }
        barn_init(barn);
        (*(*s).per_node.as_mut_ptr().add(node as usize)).barn = barn;
        node = rust_slub_next_node_mask(node, addr_of!(slab_barn_nodes));
    }
    1
}
unsafe fn calculate_sheaf_capacity(s: *mut kmem_cache, args: *mut kmem_cache_args) -> u32 {
    if cfg!(CONFIG_SLUB_TINY)
        || (*s).flags & RSL_SLAB_DEBUG_FLAGS != 0
        || (*s).flags & (RSL_SLAB_NO_SHEAVES | RSL_SLAB_NOLEAKTRACE) != 0
    {
        return 0;
    }
    let mut capacity = if (*s).size >= RSL_PAGE_SIZE as u32 {
        4
    } else if (*s).size >= 1024 {
        12
    } else if (*s).size >= 256 {
        26
    } else {
        60
    };
    let bytes = rust_slub_sheaf_struct_size(capacity);
    let bytes = kmalloc_size_roundup(bytes);
    capacity = ((bytes - rust_slub_sheaf_struct_size(0)) / size_of::<*mut Void>()) as u32;
    max(capacity, (*args).sheaf_capacity)
}
#[inline]
fn slub_align(value: u32, align: u32) -> u32 {
    value.wrapping_add(align - 1) & !(align - 1)
}
unsafe fn calculate_sizes(args: *mut kmem_cache_args, s: *mut kmem_cache) -> i32 {
    let flags = (*s).flags;
    let word = size_of::<*mut Void>() as u32;
    let mut size = slub_align((*s).object_size, word);
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if flags & RSL_SLAB_POISON != 0
            && flags & RSL_SLAB_TYPESAFE_BY_RCU == 0
            && (*s).ctor.is_none()
        {
            (*s).flags |= RSL___OBJECT_POISON;
        } else {
            (*s).flags &= !RSL___OBJECT_POISON;
        }
        if flags & RSL_SLAB_RED_ZONE != 0 && size == (*s).object_size {
            size += word;
        }
    }
    (*s).inuse = size;
    if (flags & RSL_SLAB_TYPESAFE_BY_RCU != 0 && !(*args).use_freeptr_offset)
        || flags & RSL_SLAB_POISON != 0
        || ((*s).ctor.is_some() && !(*args).use_freeptr_offset)
        || (flags & RSL_SLAB_RED_ZONE != 0
            && ((*s).object_size < word || rust_slub_slub_debug_orig_size(s)))
    {
        (*s).offset = size;
        size += word;
    } else if (flags & RSL_SLAB_TYPESAFE_BY_RCU != 0 || (*s).ctor.is_some())
        && (*args).use_freeptr_offset
    {
        (*s).offset = (*args).freeptr_offset;
    } else {
        (*s).offset = ((*s).object_size / 2) & !(word - 1);
    }
    #[cfg(CONFIG_SLUB_DEBUG)]
    if flags & RSL_SLAB_STORE_USER != 0 {
        size += (2 * size_of::<track>()) as u32;
        if flags & RSL_SLAB_KMALLOC != 0 {
            size += size_of::<ULong>() as u32;
        }
    }
    rust_slub_kasan_cache_create(s, &mut size, addr_of_mut!((*s).flags));
    #[cfg(CONFIG_SLUB_DEBUG)]
    if flags & RSL_SLAB_RED_ZONE != 0 {
        size += word;
        (*s).red_left_pad = slub_align(word, (*s).align);
        size += (*s).red_left_pad;
    }
    let aligned_size = slub_align(size, (*s).align);
    #[cfg(all(CONFIG_SLAB_OBJ_EXT, CONFIG_64BIT))]
    if rust_slub_slab_args_unmergeable(args, (*s).flags)
        && (aligned_size - size) as usize >= rust_slub_cache_obj_ext_size(s)
    {
        (*s).flags |= RSL_SLAB_OBJ_EXT_IN_OBJ;
    }
    size = aligned_size;
    (*s).size = size;
    (*s).reciprocal_size = rust_slub_reciprocal_value(size);
    let order = calculate_order(size);
    if order < 0 {
        return 0;
    }
    (*s).allocflags = RSL___GFP_COMP;
    if (*s).flags & RSL_SLAB_CACHE_DMA != 0 {
        (*s).allocflags |= RSL_GFP_DMA;
    }
    if (*s).flags & RSL_SLAB_CACHE_DMA32 != 0 {
        (*s).allocflags |= RSL_GFP_DMA32;
    }
    if (*s).flags & RSL_SLAB_RECLAIM_ACCOUNT != 0 {
        (*s).allocflags |= RSL___GFP_RECLAIMABLE;
    }
    if !rust_slub_is_kmalloc_cache(s) {
        (*s).sheaf_capacity = calculate_sheaf_capacity(s, args);
    }
    (*s).oo = oo_make(order as u32, size);
    (*s).min = oo_make(rust_slub_get_order(size as usize), size);
    (oo_objects((*s).oo) != 0) as i32
}

unsafe fn list_slab_objects(s: *mut kmem_cache, slab: *mut slab) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let addr = rust_slub_slab_address(slab);
        if !slab_add_kunit_errors() {
            slab_bug!(s, "Objects remaining on __kmem_cache_shutdown()");
        }
        rust_slub_object_map_lock();
        __fill_map(object_map.as_mut_ptr(), s, slab);
        let mut p = fixup_red_left(s, addr);
        let end = addr
            .cast::<u8>()
            .add((rust_slub_slab_objects(slab) * (*s).size) as usize) as usize;
        while (p as usize) < end {
            if !rust_slub_test_bit(
                rust_slub_obj_to_index(s, addr, p) as ULong,
                object_map.as_ptr(),
            ) && !slab_add_kunit_errors()
            {
                rust_slub_report_remaining_object(p, p as Long - addr as Long);
                print_tracking(s, p);
            }
            p = p.cast::<u8>().add((*s).size as usize).cast();
        }
        rust_slub_object_map_unlock();
        __slab_err(slab);
    }
}
unsafe fn free_partial(s: *mut kmem_cache, n: *mut kmem_cache_node) {
    let mut discard: list_head = zeroed();
    rust_slub_init_list_head(&mut discard);
    rust_slub_bug_on(rust_slub_irqs_disabled());
    rust_slub_node_lock_irq(n);
    let head = addr_of_mut!((*n).partial);
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let slab = rust_slub_slab_from_list(pos);
        if rust_slub_slab_inuse(slab) == 0 {
            remove_partial(n, slab);
            rust_slub_list_add(pos, &mut discard);
        } else {
            list_slab_objects(s, slab);
        }
        pos = next;
    }
    rust_slub_node_unlock_irq(n);
    let head = addr_of_mut!(discard);
    pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        discard_slab(s, rust_slub_slab_from_list(pos));
        pos = next;
    }
}
#[no_mangle]
unsafe extern "C" fn __kmem_cache_empty(s: *mut kmem_cache) -> bool {
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() && ((*n).nr_partial != 0 || node_nr_slabs(n) != 0) {
            return false;
        }
        node += 1;
    }
    true
}
#[no_mangle]
unsafe extern "C" fn __kmem_cache_shutdown(s: *mut kmem_cache) -> i32 {
    flush_all_cpus_locked(s);
    if rust_slub_cache_has_sheaves(s) {
        rust_slub_rcu_barrier();
    }
    let mut node = rust_slub_first_node();
    while node < rust_slub_max_num_nodes() {
        let barn = get_barn_node(s, node);
        if !barn.is_null() {
            barn_shrink(s, barn);
        }
        node = rust_slub_next_node(node);
    }
    node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            free_partial(s, n);
            if (*n).nr_partial != 0 || node_nr_slabs(n) != 0 {
                return 1;
            }
        }
        node += 1;
    }
    0
}
#[cfg(CONFIG_PRINTK)]
#[no_mangle]
unsafe extern "C" fn __kmem_obj_info(kpp: *mut kmem_obj_info, object: *mut Void, slab: *mut slab) {
    let s = (*slab).slab_cache;
    (*kpp).kp_ptr = object;
    (*kpp).kp_slab = slab;
    (*kpp).kp_slab_cache = s;
    let base = rust_slub_slab_address(slab);
    let objp0 = rust_slub_kasan_reset_tag(object);
    #[cfg(CONFIG_SLUB_DEBUG)]
    let objp = restore_red_left(s, objp0);
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    let objp = objp0;
    let objnr = rust_slub_obj_to_index_slab(s, slab, objp);
    (*kpp).kp_data_offset = (objp0 as usize).wrapping_sub(objp as usize) as ULong;
    let objp = base
        .cast::<u8>()
        .add((*s).size as usize * objnr as usize)
        .cast::<Void>();
    (*kpp).kp_objp = objp;
    if rust_slub_warn_obj_info(
        (objp as usize) < base as usize
            || objp as usize >= base as usize + (rust_slub_slab_objects(slab) * (*s).size) as usize
            || (objp as usize - base as usize) % (*s).size as usize != 0,
    ) || (*s).flags & RSL_SLAB_STORE_USER == 0
    {
        return;
    }
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let objp = fixup_red_left(s, objp);
        let trackp = get_track(s, objp, TRACK_ALLOC);
        (*kpp).kp_ret = (*trackp).addr as *mut Void;
        #[cfg(CONFIG_STACKDEPOT)]
        {
            let mut entries: *mut ULong = null_mut();
            let handle = rust_slub_track_handle_read(trackp);
            if handle != 0 {
                let nr = rust_slub_stack_depot_fetch(handle, &mut entries);
                let mut i = 0;
                while i < RSL_KS_ADDRS_COUNT as usize && i < nr as usize {
                    (*kpp).kp_stack[i] = *entries.add(i) as *mut Void;
                    i += 1;
                }
            }
            let trackp = get_track(s, objp, TRACK_FREE);
            let handle = rust_slub_track_handle_read(trackp);
            if handle != 0 {
                let nr = rust_slub_stack_depot_fetch(handle, &mut entries);
                let mut i = 0;
                while i < RSL_KS_ADDRS_COUNT as usize && i < nr as usize {
                    (*kpp).kp_free_stack[i] = *entries.add(i) as *mut Void;
                    i += 1;
                }
            }
        }
    }
}
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn setup_slub_min_order(str_: *const CChar, _kp: *const kernel_param) -> i32 {
    let ret = rust_slub_kstrtouint(str_, 0, addr_of_mut!(slub_min_order));
    if ret != 0 {
        return ret;
    }
    if slub_min_order > slub_max_order {
        slub_max_order = slub_min_order;
    }
    0
}
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn setup_slub_max_order(str_: *const CChar, _kp: *const kernel_param) -> i32 {
    let ret = rust_slub_kstrtouint(str_, 0, addr_of_mut!(slub_max_order));
    if ret != 0 {
        return ret;
    }
    slub_max_order = min(slub_max_order, RSL_MAX_PAGE_ORDER);
    if slub_min_order > slub_max_order {
        slub_min_order = slub_max_order;
    }
    0
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn setup_slab_strict_numa(_str: *const CChar, _kp: *const kernel_param) -> i32 {
    if rust_slub_nr_node_ids() > 1 {
        rust_slub_strict_numa_enable();
        rust_slub_report_strict_numa(true);
    } else {
        rust_slub_report_strict_numa(false);
    }
    0
}
#[cfg(CONFIG_HARDENED_USERCOPY)]
#[no_mangle]
unsafe extern "C" fn __check_heap_object(
    ptr: *const Void,
    n: ULong,
    slab: *const slab,
    to_user: bool,
) {
    let is_kfence = rust_slub_is_kfence_address(ptr);
    let ptr = rust_slub_kasan_reset_tag(ptr);
    let s = (*slab).slab_cache;
    let base = rust_slub_slab_address(slab);
    if (ptr as usize) < base as usize {
        rust_slub_usercopy_abort(
            c"SLUB object not in SLUB page?!".as_ptr().cast::<CChar>(),
            null(),
            to_user,
            0,
            n,
        );
    }
    let mut offset = if is_kfence {
        (ptr as usize - rust_slub_kfence_object_start(ptr) as usize) as u32
    } else {
        ((ptr as usize - base as usize) % (*s).size as usize) as u32
    };
    if !is_kfence && rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_RED_ZONE) {
        if offset < (*s).red_left_pad {
            rust_slub_usercopy_abort(
                c"SLUB object in left red zone".as_ptr().cast::<CChar>(),
                (*s).name,
                to_user,
                offset as ULong,
                n,
            );
        }
        offset = offset.wrapping_sub((*s).red_left_pad);
    }
    if offset >= (*s).useroffset
        && offset - (*s).useroffset <= (*s).usersize
        && n <= (*s)
            .useroffset
            .wrapping_sub(offset)
            .wrapping_add((*s).usersize) as ULong
    {
        return;
    }
    rust_slub_usercopy_abort(
        c"SLUB object".as_ptr().cast::<CChar>(),
        (*s).name,
        to_user,
        offset as ULong,
        n,
    );
}
unsafe fn __kmem_cache_do_shrink(s: *mut kmem_cache) -> i32 {
    let mut ret = 0;
    let mut node = rust_slub_first_node();
    while node < rust_slub_max_num_nodes() {
        let barn = get_barn_node(s, node);
        if !barn.is_null() {
            barn_shrink(s, barn);
        }
        node = rust_slub_next_node(node);
    }
    node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if n.is_null() {
            node += 1;
            continue;
        }
        let mut discard: list_head = zeroed();
        let mut promote: [list_head; RSL_SHRINK_PROMOTE_MAX as usize] = zeroed();
        rust_slub_init_list_head(&mut discard);
        for list in &mut promote {
            rust_slub_init_list_head(list);
        }
        let flags = rust_slub_node_lock_irqsave(n);
        let head = addr_of_mut!((*n).partial);
        let mut pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            let slab = rust_slub_slab_from_list(pos);
            let free = rust_slub_slab_objects(slab) as i32 - rust_slub_slab_inuse(slab) as i32;
            rust_slub_barrier();
            rust_slub_bug_on(free <= 0);
            if free as u32 == rust_slub_slab_objects(slab) {
                rust_slub_list_move(pos, &mut discard);
                clear_node_partial_state(n, slab);
                dec_slabs_node(s, node, rust_slub_slab_objects(slab) as i32);
            } else if free <= RSL_SHRINK_PROMOTE_MAX as i32 {
                rust_slub_list_move(pos, &mut promote[free as usize - 1]);
            }
            pos = next;
        }
        let mut i = RSL_SHRINK_PROMOTE_MAX as usize;
        while i != 0 {
            i -= 1;
            rust_slub_list_splice(&mut promote[i], head);
        }
        rust_slub_node_unlock_irqrestore(n, flags);
        let head = addr_of_mut!(discard);
        pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            free_slab(s, rust_slub_slab_from_list(pos));
            pos = next;
        }
        if node_nr_slabs(n) != 0 {
            ret = 1;
        }
        node += 1;
    }
    ret
}
#[no_mangle]
unsafe extern "C" fn __kmem_cache_shrink(s: *mut kmem_cache) -> i32 {
    flush_all(s);
    __kmem_cache_do_shrink(s)
}
unsafe fn slab_mem_going_offline_callback() -> i32 {
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let head = addr_of_mut!(slab_caches);
    let mut pos = (*head).next;
    while pos != head {
        let s = rust_slub_cache_from_list(pos);
        flush_all_cpus_locked(s);
        __kmem_cache_do_shrink(s);
        pos = (*pos).next;
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    0
}
unsafe fn slab_mem_going_online_callback(nid: i32) -> i32 {
    let mut ret = 0;
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let head = addr_of_mut!(slab_caches);
    let mut pos = (*head).next;
    while pos != head {
        let s = rust_slub_cache_from_list(pos);
        if !get_node(s, nid).is_null() {
            pos = (*pos).next;
            continue;
        }
        let mut barn = null_mut();
        if rust_slub_cache_has_sheaves(s) && get_barn_node(s, nid).is_null() {
            barn = rust_slub_alloc_barn_hotplug(nid).cast::<node_barn>();
            if barn.is_null() {
                ret = -(RSL_ENOMEM as i32);
                break;
            }
        }
        let n = rust_slub_cache_alloc(kmem_cache_node, RSL_GFP_KERNEL).cast::<kmem_cache_node>();
        if n.is_null() {
            kfree(barn.cast());
            ret = -(RSL_ENOMEM as i32);
            break;
        }
        init_kmem_cache_node(n);
        (*(*s).per_node.as_mut_ptr().add(nid as usize)).node = n;
        if !barn.is_null() {
            barn_init(barn);
            (*(*s).per_node.as_mut_ptr().add(nid as usize)).barn = barn;
        }
        pos = (*pos).next;
    }
    if ret == 0 {
        rust_slub_node_set(nid, addr_of_mut!(slab_nodes));
        rust_slub_node_set(nid, addr_of_mut!(slab_barn_nodes));
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    ret
}
#[no_mangle]
unsafe extern "C" fn slab_memory_callback(
    _self: *mut notifier_block,
    action: ULong,
    arg: *mut Void,
) -> i32 {
    let nid = (*(arg as *mut node_notify)).nid;
    let ret = if action == RSL_NODE_ADDING_FIRST_MEMORY as ULong {
        slab_mem_going_online_callback(nid)
    } else if action == RSL_NODE_REMOVING_LAST_MEMORY as ULong {
        slab_mem_going_offline_callback()
    } else {
        0
    };
    if ret != 0 {
        rust_slub_notifier_from_errno(ret)
    } else {
        RSL_NOTIFY_OK as i32
    }
}
#[link_section = ".init.text"]
unsafe fn bootstrap(static_cache: *mut kmem_cache) -> *mut kmem_cache {
    let s = rust_slub_cache_zalloc(kmem_cache, RSL_GFP_NOWAIT).cast::<kmem_cache>();
    rust_slub_memcpy(
        s.cast(),
        static_cache.cast(),
        (*kmem_cache).object_size as usize,
    );
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            let head = addr_of_mut!((*n).partial);
            let mut pos = (*head).next;
            while pos != head {
                (*rust_slub_slab_from_list(pos)).slab_cache = s;
                pos = (*pos).next;
            }
            #[cfg(CONFIG_SLUB_DEBUG)]
            {
                let head = addr_of_mut!((*n).full);
                let mut pos = (*head).next;
                while pos != head {
                    (*rust_slub_slab_from_list(pos)).slab_cache = s;
                    pos = (*pos).next;
                }
            }
        }
        node += 1;
    }
    rust_slub_list_add(addr_of_mut!((*s).list), addr_of_mut!(slab_caches));
    s
}
#[link_section = ".init.text"]
unsafe fn bootstrap_cache_sheaves(s: *mut kmem_cache) {
    let mut empty_args: kmem_cache_args = zeroed();
    rust_slub_vm_warn_bootstrap_sheaves(rust_slub_cache_has_sheaves(s));
    let capacity = calculate_sheaf_capacity(s, &mut empty_args);
    if capacity == 0 {
        return;
    }
    let mut node = rust_slub_first_node_mask(addr_of!(slab_barn_nodes));
    while node < rust_slub_max_num_nodes() {
        let barn = rust_slub_alloc_barn_bootstrap(node).cast::<node_barn>();
        if barn.is_null() {
            rust_slub_panic_cache_oom((*s).name);
        }
        barn_init(barn);
        (*(*s).per_node.as_mut_ptr().add(node as usize)).barn = barn;
        node = rust_slub_next_node_mask(node, addr_of!(slab_barn_nodes));
    }
    let mut cpu = rust_slub_first_possible_cpu();
    while cpu < rust_slub_nr_cpu_ids() {
        let pcs = rust_slub_percpu_sheaves(s, cpu);
        (*pcs).main = __alloc_empty_sheaf(s, RSL_GFP_KERNEL, RSL_SLAB_ALLOC_DEFAULT, capacity);
        if (*pcs).main.is_null() {
            rust_slub_panic_cache_oom((*s).name);
        }
        cpu = rust_slub_next_possible_cpu(cpu + 1);
    }
    (*s).sheaf_capacity = capacity;
}
#[link_section = ".init.text"]
unsafe fn bootstrap_kmalloc_sheaves() {
    let mut kind = KMALLOC_NORMAL;
    while kind < NR_KMALLOC_TYPES {
        let mut idx = 0;
        while idx < RSL_KMALLOC_SHIFT_HIGH + 1 {
            let s = rust_slub_kmalloc_cache_at(kind, idx);
            if !s.is_null() && !rust_slub_cache_has_sheaves(s) {
                bootstrap_cache_sheaves(s);
            }
            idx += 1;
        }
        kind += 1;
    }
}
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn kmem_cache_init() {
    slab_obj_ext_has_codetag_init();
    if rust_slub_debug_guardpage_minorder() != 0 {
        slub_max_order = 0;
    }
    rust_slub_hash_pointers_finalize(rust_slub_debug_enabled());
    kmem_cache_node = rust_slub_boot_cache_node();
    kmem_cache = rust_slub_boot_cache();
    let mut node = rust_slub_first_memory_node();
    while node < rust_slub_max_num_nodes() {
        rust_slub_node_set(node, addr_of_mut!(slab_nodes));
        node = rust_slub_next_memory_node(node);
    }
    node = rust_slub_first_online_node();
    while node < rust_slub_max_num_nodes() {
        rust_slub_node_set(node, addr_of_mut!(slab_barn_nodes));
        node = rust_slub_next_online_node(node);
    }
    rust_slub_create_boot_cache(
        kmem_cache_node,
        c"kmem_cache_node".as_ptr().cast::<CChar>(),
        size_of::<kmem_cache_node>() as u32,
        RSL_SLAB_HWCACHE_ALIGN | RSL_SLAB_NO_SHEAVES | RSL_SLAB_NO_OBJ_EXT,
        0,
        0,
    );
    rust_slub_register_memory_notifier();
    slab_state = PARTIAL;
    let cache_bytes = offset_of!(kmem_cache, per_node)
        + rust_slub_nr_node_ids() as usize * size_of::<kmem_cache_per_node_ptrs>();
    rust_slub_create_boot_cache(
        kmem_cache,
        c"kmem_cache".as_ptr().cast::<CChar>(),
        cache_bytes as u32,
        RSL_SLAB_HWCACHE_ALIGN | RSL_SLAB_NO_SHEAVES | RSL_SLAB_NO_OBJ_EXT,
        0,
        0,
    );
    kmem_cache = bootstrap(rust_slub_boot_cache());
    kmem_cache_node = bootstrap(rust_slub_boot_cache_node());
    rust_slub_setup_kmalloc_cache_index_table();
    rust_slub_create_kmalloc_caches();
    bootstrap_kmalloc_sheaves();
    init_freelist_randomization();
    rust_slub_cpuhp_setup(Some(slub_cpu_setup), Some(slub_cpu_dead));
    rust_slub_report_init(slub_min_order, slub_max_order, slub_min_objects);
}
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn kmem_cache_init_late() {
    flushwq = rust_slub_alloc_flush_workqueue();
    rust_slub_warn(flushwq.is_null());
    #[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
    rust_slub_prandom_init_once();
}
#[no_mangle]
unsafe extern "C" fn do_kmem_cache_create(
    s: *mut kmem_cache,
    name: *const CChar,
    size: u32,
    args: *mut kmem_cache_args,
    flags: slab_flags_t,
) -> i32 {
    let mut err = -(RSL_EINVAL as i32);
    (*s).name = name;
    (*s).size = size;
    (*s).object_size = size;
    (*s).flags = kmem_cache_flags(flags, (*s).name);
    #[cfg(CONFIG_SLAB_FREELIST_HARDENED)]
    {
        (*s).random = rust_slub_get_random_long();
    }
    (*s).align = (*args).align;
    (*s).ctor = (*args).ctor;
    #[cfg(CONFIG_HARDENED_USERCOPY)]
    {
        (*s).useroffset = (*args).useroffset;
        (*s).usersize = (*args).usersize;
    }
    'create: {
        if calculate_sizes(args, s) == 0 {
            break 'create;
        }
        if disable_higher_order_debug != 0
            && rust_slub_get_order((*s).size as usize)
                > rust_slub_get_order((*s).object_size as usize)
        {
            (*s).flags &= !RSL_DEBUG_METADATA_FLAGS;
            (*s).offset = 0;
            if calculate_sizes(args, s) == 0 {
                break 'create;
            }
        }
        if rust_slub_system_has_freelist_aba() && (*s).flags & RSL_SLAB_NO_CMPXCHG == 0 {
            (*s).flags |= RSL___CMPXCHG_DOUBLE;
        }
        (*s).min_partial = max(
            RSL_MIN_PARTIAL as ULong,
            min(
                RSL_MAX_PARTIAL as ULong,
                rust_slub_ilog2((*s).size) as ULong / 2,
            ),
        );
        (*s).cpu_sheaves = rust_slub_alloc_percpu_sheaves();
        if (*s).cpu_sheaves.is_null() {
            err = -(RSL_ENOMEM as i32);
            break 'create;
        }
        #[cfg(CONFIG_NUMA)]
        {
            (*s).remote_node_defrag_ratio = 1000;
        }
        if slab_state >= UP && init_cache_random_seq(s) != 0 {
            break 'create;
        }
        if init_kmem_cache_nodes(s) == 0 {
            break 'create;
        }
        #[cfg(CONFIG_SLUB_STATS)]
        if alloc_kmem_cache_stats(s) == 0 {
            break 'create;
        }
        err = init_percpu_sheaves(s);
        if err != 0 {
            break 'create;
        }
        if slab_state <= UP {
            break 'create;
        }
        if sysfs_slab_add(s) != 0 {
            rust_slub_report_sysfs_cache_error((*s).name, false);
        }
        if (*s).flags & RSL_SLAB_STORE_USER != 0 {
            debugfs_slab_add(s);
        }
    }
    if err != 0 {
        __kmem_cache_release(s);
    }
    err
}

#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe extern "C" fn count_inuse(slab: *mut slab) -> i32 {
    rust_slub_slab_inuse(slab) as i32
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe extern "C" fn count_total(slab: *mut slab) -> i32 {
    rust_slub_slab_objects(slab) as i32
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn validate_slab(s: *mut kmem_cache, slab: *mut slab, obj_map: *mut ULong) {
    let addr = rust_slub_slab_address(slab);
    if !validate_slab_ptr(slab) {
        slab_err!(s, slab, "Not a valid slab page");
        return;
    }
    if check_slab(s, slab) == 0 || !on_freelist(s, slab, null_mut()) {
        return;
    }
    __fill_map(obj_map, s, slab);
    let mut p = fixup_red_left(s, addr);
    let end = addr as usize + (rust_slub_slab_objects(slab) * (*s).size) as usize;
    while (p as usize) < end {
        let val = if rust_slub_test_bit(rust_slub_obj_to_index(s, addr, p) as ULong, obj_map) {
            RSL_SLUB_RED_INACTIVE
        } else {
            RSL_SLUB_RED_ACTIVE
        };
        if check_object(s, slab, p, val as u8) == 0 {
            break;
        }
        p = p.cast::<u8>().add((*s).size as usize).cast();
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn validate_slab_node(
    s: *mut kmem_cache,
    n: *mut kmem_cache_node,
    obj_map: *mut ULong,
) -> i32 {
    let mut count = 0 as ULong;
    let flags = rust_slub_node_lock_irqsave(n);
    let head = addr_of_mut!((*n).partial);
    let mut pos = (*head).next;
    while pos != head {
        validate_slab(s, rust_slub_slab_from_list(pos), obj_map);
        count += 1;
        pos = (*pos).next;
    }
    if count != (*n).nr_partial {
        rust_slub_report_slab_count((*s).name, count, (*n).nr_partial, true);
        slab_add_kunit_errors();
    }
    if (*s).flags & RSL_SLAB_STORE_USER != 0 {
        let head = addr_of_mut!((*n).full);
        let mut pos = (*head).next;
        while pos != head {
            validate_slab(s, rust_slub_slab_from_list(pos), obj_map);
            count += 1;
            pos = (*pos).next;
        }
        if count != node_nr_slabs(n) {
            rust_slub_report_slab_count((*s).name, count, node_nr_slabs(n), false);
            slab_add_kunit_errors();
        }
    }
    rust_slub_node_unlock_irqrestore(n, flags);
    count as i32
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
unsafe extern "C" fn validate_slab_cache(s: *mut kmem_cache) -> Long {
    let obj_map = rust_slub_bitmap_alloc(oo_objects((*s).oo), RSL_GFP_KERNEL);
    if obj_map.is_null() {
        return -(RSL_ENOMEM as Long);
    }
    flush_all(s);
    let mut count = 0 as ULong;
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            count += validate_slab_node(s, n, obj_map) as ULong;
        }
        node += 1;
    }
    rust_slub_bitmap_free(obj_map);
    count as Long
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
static mut slab_debugfs_root: *mut dentry = null_mut();
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe fn free_loc_track(t: *mut loc_track) {
    if (*t).max != 0 {
        rust_slub_free_pages(
            (*t).loc as ULong,
            rust_slub_get_order(size_of::<location>() * (*t).max as usize),
        );
    }
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe fn alloc_loc_track(t: *mut loc_track, maximum: ULong, flags: gfp_t) -> i32 {
    let order = rust_slub_get_order(size_of::<location>() * maximum as usize);
    let l = rust_slub_get_free_pages(flags, order) as *mut location;
    if l.is_null() {
        return 0;
    }
    if (*t).count != 0 {
        rust_slub_memcpy(
            l.cast(),
            (*t).loc.cast(),
            size_of::<location>() * (*t).count as usize,
        );
        free_loc_track(t);
    }
    (*t).max = maximum;
    (*t).loc = l;
    1
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe fn add_location(
    t: *mut loc_track,
    s: *mut kmem_cache,
    track: *const track,
    orig_size: u32,
) -> i32 {
    let age = rust_slub_jiffies().wrapping_sub((*track).when);
    #[cfg(CONFIG_STACKDEPOT)]
    let handle = rust_slub_track_handle_read(track);
    #[cfg(not(CONFIG_STACKDEPOT))]
    let handle = 0;
    let waste = (*s).object_size - orig_size;
    let mut start = -1 as Long;
    let mut end = (*t).count as Long;
    let mut pos;
    loop {
        pos = start + (end - start + 1) / 2;
        if pos == end {
            break;
        }
        let l = (*t).loc.add(pos as usize);
        let caddr = (*l).addr;
        let chandle = (*l).handle;
        let cwaste = (*l).waste;
        if (*track).addr == caddr && handle == chandle && waste as ULong == cwaste {
            (*l).count += 1;
            if (*track).when != 0 {
                (*l).sum_time = (*l).sum_time.wrapping_add(age as i64);
                if age < (*l).min_time as ULong {
                    (*l).min_time = age as Long;
                }
                if age > (*l).max_time as ULong {
                    (*l).max_time = age as Long;
                }
                if ((*track).pid as Long) < (*l).min_pid {
                    (*l).min_pid = (*track).pid as Long;
                }
                if (*track).pid as Long > (*l).max_pid {
                    (*l).max_pid = (*track).pid as Long;
                }
                rust_slub_location_cpu_set(l, (*track).cpu as u32);
            }
            rust_slub_node_set(
                rust_slub_page_to_nid(rust_slub_virt_to_page(track.cast())),
                addr_of_mut!((*l).nodes),
            );
            return 1;
        }
        if (*track).addr < caddr
            || ((*track).addr == caddr && handle < chandle)
            || ((*track).addr == caddr && handle == chandle && (waste as ULong) < cwaste)
        {
            end = pos;
        } else {
            start = pos;
        }
    }
    if (*t).count >= (*t).max && alloc_loc_track(t, 2 * (*t).max, RSL_GFP_ATOMIC) == 0 {
        return 0;
    }
    let l = (*t).loc.add(pos as usize);
    if (pos as ULong) < (*t).count {
        core::ptr::copy(l, l.add(1), ((*t).count - pos as ULong) as usize);
    }
    (*t).count += 1;
    (*l).count = 1;
    (*l).addr = (*track).addr;
    (*l).sum_time = age as i64;
    (*l).min_time = age as Long;
    (*l).max_time = age as Long;
    (*l).min_pid = (*track).pid as Long;
    (*l).max_pid = (*track).pid as Long;
    (*l).handle = handle;
    (*l).waste = waste as ULong;
    rust_slub_location_cpus_clear(l);
    rust_slub_location_cpu_set(l, (*track).cpu as u32);
    rust_slub_nodes_clear(addr_of_mut!((*l).nodes));
    rust_slub_node_set(
        rust_slub_page_to_nid(rust_slub_virt_to_page(track.cast())),
        addr_of_mut!((*l).nodes),
    );
    1
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe fn process_slab(
    t: *mut loc_track,
    s: *mut kmem_cache,
    slab: *mut slab,
    alloc: track_item,
    obj_map: *mut ULong,
) {
    let addr = rust_slub_slab_address(slab);
    __fill_map(obj_map, s, slab);
    let mut p = fixup_red_left(s, addr);
    let end = addr as usize + (rust_slub_slab_objects(slab) * (*s).size) as usize;
    while (p as usize) < end {
        if !rust_slub_test_bit(rust_slub_obj_to_index(s, addr, p) as ULong, obj_map) {
            let orig = if alloc == TRACK_ALLOC {
                get_orig_size(s, p) as u32
            } else {
                (*s).object_size
            };
            add_location(t, s, get_track(s, p, alloc), orig);
        }
        p = p.cast::<u8>().add((*s).size as usize).cast();
    }
}

#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe fn show_slab_objects(s: *mut kmem_cache, buf: *mut CChar, flags: ULong) -> Long {
    let nodes = rust_slub_alloc_slab_node_counts().cast::<ULong>();
    if nodes.is_null() {
        return -(RSL_ENOMEM as Long);
    }
    let mut total = 0 as ULong;
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            let mut x = 0i32;
            let mut all = false;
            #[cfg(CONFIG_SLUB_DEBUG)]
            if flags & RSL_SO_ALL as ULong != 0 {
                x = if flags & RSL_SO_TOTAL as ULong != 0 {
                    node_nr_objs(n) as i32
                } else if flags & RSL_SO_OBJECTS as ULong != 0 {
                    node_nr_objs(n).wrapping_sub(count_partial(n, count_free)) as i32
                } else {
                    node_nr_slabs(n) as i32
                };
                all = true;
            }
            if !all && flags & RSL_SO_PARTIAL as ULong != 0 {
                x = if flags & RSL_SO_TOTAL as ULong != 0 {
                    count_partial(n, count_total) as i32
                } else if flags & RSL_SO_OBJECTS as ULong != 0 {
                    count_partial(n, count_inuse) as i32
                } else {
                    (*n).nr_partial as i32
                };
            }
            total = total.wrapping_add(x as ULong);
            *nodes.add(node as usize) = (*nodes.add(node as usize)).wrapping_add(x as ULong);
        }
        node += 1;
    }
    let mut len = rust_slub_sysfs_emit_ulong_at(buf, 0, total);
    #[cfg(CONFIG_NUMA)]
    {
        let mut node = 0;
        while node < rust_slub_nr_node_ids() as i32 {
            if *nodes.add(node as usize) != 0 {
                len += rust_slub_sysfs_emit_node_at(buf, len, node, *nodes.add(node as usize));
            }
            node += 1;
        }
    }
    len += rust_slub_sysfs_emit_newline_at(buf, len);
    kfree(nodes.cast());
    len as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
macro_rules! slub_show_u32 {
    ($name:ident, $s:ident, $value:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $name($s: *mut kmem_cache, buf: *mut CChar) -> Long {
            rust_slub_sysfs_emit_u32(buf, $value) as Long
        }
    };
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
macro_rules! slub_show_flag {
    ($name:ident, $flag:ident) => {
        slub_show_u32!($name, s, (((*s).flags & $flag) != 0) as u32);
    };
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(slab_size_show, s, (*s).size);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(align_show, s, (*s).align);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(object_size_show, s, (*s).object_size);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(objs_per_slab_show, s, oo_objects((*s).oo));
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(order_show, s, oo_order((*s).oo));
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(sheaf_capacity_show, s, (*s).sheaf_capacity);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn min_partial_show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
    rust_slub_sysfs_emit_ulong(buf, (*s).min_partial) as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn min_partial_store(
    s: *mut kmem_cache,
    buf: *const CChar,
    length: usize,
) -> Long {
    let mut value = 0;
    let err = rust_slub_kstrtoul(buf, 10, &mut value);
    if err != 0 {
        return err as Long;
    }
    (*s).min_partial = value;
    length as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(cpu_partial_show, _s, 0);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn cpu_partial_store(
    _s: *mut kmem_cache,
    buf: *const CChar,
    length: usize,
) -> Long {
    let mut objects = 0;
    let err = rust_slub_kstrtouint(buf, 10, &mut objects);
    if err != 0 {
        return err as Long;
    }
    if objects != 0 {
        return -(RSL_EINVAL as Long);
    }
    length as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn ctor_show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
    match (*s).ctor {
        None => 0,
        Some(ctor) => rust_slub_sysfs_emit_symbol(buf, ctor as *const Void) as Long,
    }
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn aliases_show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
    rust_slub_sysfs_emit_i32(
        buf,
        if (*s).refcount < 0 {
            0
        } else {
            (*s).refcount - 1
        },
    ) as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn partial_show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
    show_slab_objects(s, buf, RSL_SO_PARTIAL as ULong)
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_u32!(cpu_slabs_show, _s, 0);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn objects_partial_show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
    show_slab_objects(s, buf, (RSL_SO_PARTIAL | RSL_SO_OBJECTS) as ULong)
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn slabs_cpu_partial_show(_s: *mut kmem_cache, buf: *mut CChar) -> Long {
    rust_slub_sysfs_emit_text(buf, c"0(0)\n".as_ptr().cast::<CChar>()) as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_flag!(reclaim_account_show, RSL_SLAB_RECLAIM_ACCOUNT);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_flag!(hwcache_align_show, RSL_SLAB_HWCACHE_ALIGN);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_ZONE_DMA))]
slub_show_flag!(cache_dma_show, RSL_SLAB_CACHE_DMA);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_HARDENED_USERCOPY))]
slub_show_u32!(usersize_show, s, (*s).usersize);
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
slub_show_flag!(destroy_by_rcu_show, RSL_SLAB_TYPESAFE_BY_RCU);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
macro_rules! slub_show_objects {
    ($name:ident, $flags:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $name(s: *mut kmem_cache, buf: *mut CChar) -> Long {
            show_slab_objects(s, buf, $flags as ULong)
        }
    };
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_objects!(slabs_show, RSL_SO_ALL);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_objects!(total_objects_show, RSL_SO_ALL | RSL_SO_TOTAL);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_objects!(objects_show, RSL_SO_ALL | RSL_SO_OBJECTS);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_flag!(sanity_checks_show, RSL_SLAB_CONSISTENCY_CHECKS);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_flag!(trace_show, RSL_SLAB_TRACE);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_flag!(red_zone_show, RSL_SLAB_RED_ZONE);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_flag!(poison_show, RSL_SLAB_POISON);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
slub_show_flag!(store_user_show, RSL_SLAB_STORE_USER);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
#[no_mangle]
unsafe extern "C" fn validate_show(_s: *mut kmem_cache, _buf: *mut CChar) -> Long {
    0
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_DEBUG))]
#[no_mangle]
unsafe extern "C" fn validate_store(s: *mut kmem_cache, buf: *const CChar, length: usize) -> Long {
    if *buf == b'1' as CChar && kmem_cache_debug(s) {
        let ret = validate_slab_cache(s) as i32;
        return if ret >= 0 {
            length as i32 as Long
        } else {
            ret as Long
        };
    }
    -(RSL_EINVAL as Long)
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_FAILSLAB))]
slub_show_flag!(failslab_show, RSL_SLAB_FAILSLAB);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_FAILSLAB))]
#[no_mangle]
unsafe extern "C" fn failslab_store(s: *mut kmem_cache, buf: *const CChar, length: usize) -> Long {
    if (*s).refcount > 1 {
        return -(RSL_EINVAL as Long);
    }
    let flags = if *buf == b'1' as CChar {
        (*s).flags | RSL_SLAB_FAILSLAB
    } else {
        (*s).flags & !RSL_SLAB_FAILSLAB
    };
    rust_slub_cache_flags_write_once(s, flags);
    length as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn shrink_show(_s: *mut kmem_cache, _buf: *mut CChar) -> Long {
    0
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn shrink_store(s: *mut kmem_cache, buf: *const CChar, length: usize) -> Long {
    if *buf != b'1' as CChar {
        return -(RSL_EINVAL as Long);
    }
    rust_slub_kmem_cache_shrink(s);
    length as Long
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_NUMA))]
slub_show_u32!(
    remote_node_defrag_ratio_show,
    s,
    (*s).remote_node_defrag_ratio / 10
);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_NUMA))]
#[no_mangle]
unsafe extern "C" fn remote_node_defrag_ratio_store(
    s: *mut kmem_cache,
    buf: *const CChar,
    length: usize,
) -> Long {
    let mut ratio = 0;
    let err = rust_slub_kstrtouint(buf, 10, &mut ratio);
    if err != 0 {
        return err as Long;
    }
    if ratio > 100 {
        return -(RSL_ERANGE as Long);
    }
    (*s).remote_node_defrag_ratio = ratio * 10;
    length as Long
}

#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_STATS))]
unsafe fn show_stat(s: *mut kmem_cache, buf: *mut CChar, si: stat_item) -> i32 {
    let data = rust_slub_alloc_stat_data().cast::<i32>();
    if data.is_null() {
        return -(RSL_ENOMEM as i32);
    }
    let mut sum = 0 as ULong;
    let mut cpu = rust_slub_first_online_cpu();
    while cpu < rust_slub_nr_cpu_ids() {
        let x = rust_slub_percpu_stat(s, cpu, si);
        *data.add(cpu as usize) = x as i32;
        sum += x as ULong;
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
    let mut len = rust_slub_sysfs_emit_ulong_at(buf, 0, sum);
    #[cfg(CONFIG_SMP)]
    {
        cpu = rust_slub_first_online_cpu();
        while cpu < rust_slub_nr_cpu_ids() {
            if *data.add(cpu as usize) != 0 {
                len += rust_slub_sysfs_emit_cpu_at(buf, len, cpu, *data.add(cpu as usize) as u32);
            }
            cpu = rust_slub_next_online_cpu(cpu + 1);
        }
    }
    kfree(data.cast());
    len += rust_slub_sysfs_emit_newline_at(buf, len);
    len
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_STATS))]
unsafe fn clear_stat(s: *mut kmem_cache, si: stat_item) {
    let mut cpu = rust_slub_first_online_cpu();
    while cpu < rust_slub_nr_cpu_ids() {
        rust_slub_percpu_stat_clear(s, cpu, si);
        cpu = rust_slub_next_online_cpu(cpu + 1);
    }
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_STATS))]
macro_rules! slub_stat_attr {
    ($show:ident, $store:ident, $si:ident) => {
        #[no_mangle]
        unsafe extern "C" fn $show(s: *mut kmem_cache, buf: *mut CChar) -> Long {
            show_stat(s, buf, $si) as Long
        }
        #[no_mangle]
        unsafe extern "C" fn $store(s: *mut kmem_cache, buf: *const CChar, length: usize) -> Long {
            if *buf != b'0' as CChar {
                return -(RSL_EINVAL as Long);
            }
            clear_stat(s, $si);
            length as Long
        }
    };
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_SLUB_STATS))]
mod allocation_stat_attrs {
    use super::*;
    slub_stat_attr!(alloc_fastpath_show, alloc_fastpath_store, ALLOC_FASTPATH);
    slub_stat_attr!(alloc_slowpath_show, alloc_slowpath_store, ALLOC_SLOWPATH);
    slub_stat_attr!(free_rcu_sheaf_show, free_rcu_sheaf_store, FREE_RCU_SHEAF);
    slub_stat_attr!(
        free_rcu_sheaf_fail_show,
        free_rcu_sheaf_fail_store,
        FREE_RCU_SHEAF_FAIL
    );
    slub_stat_attr!(free_fastpath_show, free_fastpath_store, FREE_FASTPATH);
    slub_stat_attr!(free_slowpath_show, free_slowpath_store, FREE_SLOWPATH);
    slub_stat_attr!(
        free_add_partial_show,
        free_add_partial_store,
        FREE_ADD_PARTIAL
    );
    slub_stat_attr!(
        free_remove_partial_show,
        free_remove_partial_store,
        FREE_REMOVE_PARTIAL
    );
    slub_stat_attr!(alloc_slab_show, alloc_slab_store, ALLOC_SLAB);
    slub_stat_attr!(
        alloc_node_mismatch_show,
        alloc_node_mismatch_store,
        ALLOC_NODE_MISMATCH
    );
    slub_stat_attr!(free_slab_show, free_slab_store, FREE_SLAB);
    slub_stat_attr!(order_fallback_show, order_fallback_store, ORDER_FALLBACK);
    slub_stat_attr!(
        cmpxchg_double_fail_show,
        cmpxchg_double_fail_store,
        CMPXCHG_DOUBLE_FAIL
    );
    slub_stat_attr!(sheaf_flush_show, sheaf_flush_store, SHEAF_FLUSH);
    slub_stat_attr!(sheaf_refill_show, sheaf_refill_store, SHEAF_REFILL);
    slub_stat_attr!(sheaf_alloc_show, sheaf_alloc_store, SHEAF_ALLOC);
    slub_stat_attr!(sheaf_free_show, sheaf_free_store, SHEAF_FREE);
    slub_stat_attr!(barn_get_show, barn_get_store, BARN_GET);
    slub_stat_attr!(barn_get_fail_show, barn_get_fail_store, BARN_GET_FAIL);
    slub_stat_attr!(barn_put_show, barn_put_store, BARN_PUT);
    slub_stat_attr!(barn_put_fail_show, barn_put_fail_store, BARN_PUT_FAIL);
    slub_stat_attr!(
        sheaf_prefill_fast_show,
        sheaf_prefill_fast_store,
        SHEAF_PREFILL_FAST
    );
    slub_stat_attr!(
        sheaf_prefill_slow_show,
        sheaf_prefill_slow_store,
        SHEAF_PREFILL_SLOW
    );
    slub_stat_attr!(
        sheaf_prefill_oversize_show,
        sheaf_prefill_oversize_store,
        SHEAF_PREFILL_OVERSIZE
    );
    slub_stat_attr!(
        sheaf_return_fast_show,
        sheaf_return_fast_store,
        SHEAF_RETURN_FAST
    );
    slub_stat_attr!(
        sheaf_return_slow_show,
        sheaf_return_slow_store,
        SHEAF_RETURN_SLOW
    );
}
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_KFENCE))]
slub_show_flag!(skip_kfence_show, RSL_SLAB_SKIP_KFENCE);
#[cfg(all(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)), CONFIG_KFENCE))]
#[no_mangle]
unsafe extern "C" fn skip_kfence_store(
    s: *mut kmem_cache,
    buf: *const CChar,
    length: usize,
) -> Long {
    if *buf == b'0' as CChar {
        (*s).flags &= !RSL_SLAB_SKIP_KFENCE;
    } else if *buf == b'1' as CChar {
        (*s).flags |= RSL_SLAB_SKIP_KFENCE;
    } else {
        return -(RSL_EINVAL as Long);
    }
    length as Long
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn slab_attr_show(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *mut CChar,
) -> Long {
    let attribute = attr
        .cast::<u8>()
        .sub(offset_of!(slab_attribute, attr))
        .cast::<slab_attribute>();
    let s = kobj
        .cast::<u8>()
        .sub(offset_of!(kmem_cache, kobj))
        .cast::<kmem_cache>();
    match (*attribute).show {
        Some(show) => show(s, buf),
        None => -(RSL_EIO as Long),
    }
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn slab_attr_store(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *const CChar,
    len: usize,
) -> Long {
    let attribute = attr
        .cast::<u8>()
        .sub(offset_of!(slab_attribute, attr))
        .cast::<slab_attribute>();
    let s = kobj
        .cast::<u8>()
        .sub(offset_of!(kmem_cache, kobj))
        .cast::<kmem_cache>();
    match (*attribute).store {
        Some(store) => store(s, buf, len),
        None => -(RSL_EIO as Long),
    }
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn kmem_cache_release(k: *mut kobject) {
    let s = k
        .cast::<u8>()
        .sub(offset_of!(kmem_cache, kobj))
        .cast::<kmem_cache>();
    rust_slub_slab_kmem_cache_release(s);
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
static mut slab_kset: *mut kset = null_mut();
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe fn cache_kset(_s: *mut kmem_cache) -> *mut kset {
    slab_kset
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe fn create_unique_id(s: *mut kmem_cache) -> *mut CChar {
    let name = rust_slub_alloc_unique_id().cast::<CChar>();
    if name.is_null() {
        return rust_slub_err_ptr(-(RSL_ENOMEM as Long)).cast();
    }
    let mut p = name;
    *p = b':' as CChar;
    p = p.add(1);
    for (flag, ch) in [
        (RSL_SLAB_CACHE_DMA, b'd'),
        (RSL_SLAB_CACHE_DMA32, b'D'),
        (RSL_SLAB_RECLAIM_ACCOUNT, b'a'),
        (RSL_SLAB_CONSISTENCY_CHECKS, b'F'),
        (RSL_SLAB_ACCOUNT, b'A'),
    ] {
        if (*s).flags & flag != 0 {
            *p = ch as CChar;
            p = p.add(1);
        }
    }
    if p != name.add(1) {
        *p = b'-' as CChar;
        p = p.add(1);
    }
    let count = rust_slub_snprintf_id(
        p,
        RSL_ID_STR_LENGTH as usize - (p as usize - name as usize),
        (*s).size,
    );
    p = p.add(count as usize);
    if rust_slub_warn(p as usize > name as usize + RSL_ID_STR_LENGTH as usize - 1) {
        kfree(name.cast());
        return rust_slub_err_ptr(-(RSL_EINVAL as Long)).cast();
    }
    rust_slub_kmsan_unpoison_memory(name.cast(), p as usize - name as usize);
    name
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
unsafe fn sysfs_slab_add(s: *mut kmem_cache) -> i32 {
    let kset = cache_kset(s);
    let unmergeable = rust_slub_slab_unmergeable(s)
        || (disable_higher_order_debug != 0 && slub_debug & RSL_DEBUG_METADATA_FLAGS != 0);
    let name;
    if unmergeable {
        rust_slub_sysfs_remove_link(addr_of_mut!((*slab_kset).kobj), (*s).name);
        name = (*s).name;
    } else {
        name = create_unique_id(s);
        if rust_slub_is_err(name.cast()) {
            return rust_slub_ptr_err(name.cast()) as i32;
        }
    }
    (*s).kobj.kset = kset;
    let err = rust_slub_kobject_init_add_slab(addr_of_mut!((*s).kobj), name);
    // Original intentionally omits kobject_put even on registration failure.
    if err == 0 && !unmergeable {
        sysfs_slab_alias(s, (*s).name);
    }
    if !unmergeable {
        kfree(name.cast());
    }
    err
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn sysfs_slab_unlink(s: *mut kmem_cache) {
    if rust_slub_kobject_in_sysfs(addr_of!((*s).kobj)) {
        rust_slub_kobject_del(addr_of_mut!((*s).kobj));
    }
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn sysfs_slab_release(s: *mut kmem_cache) {
    rust_slub_kobject_put(addr_of_mut!((*s).kobj));
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
static mut alias_list: *mut saved_alias = null_mut();
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[no_mangle]
unsafe extern "C" fn sysfs_slab_alias(s: *mut kmem_cache, name: *const CChar) -> i32 {
    if slab_state == FULL {
        rust_slub_sysfs_remove_link(addr_of_mut!((*slab_kset).kobj), name);
        return rust_slub_sysfs_create_link(
            addr_of_mut!((*slab_kset).kobj),
            addr_of_mut!((*s).kobj),
            name,
        );
    }
    let al = rust_slub_alloc_saved_alias().cast::<saved_alias>();
    if al.is_null() {
        return -(RSL_ENOMEM as i32);
    }
    (*al).s = s;
    (*al).name = name;
    (*al).next = alias_list;
    alias_list = al;
    rust_slub_kmsan_unpoison_memory(al.cast(), size_of::<saved_alias>());
    0
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[link_section = ".init.text"]
unsafe fn slab_kset_init() -> i32 {
    slab_kset = rust_slub_kset_create_slab();
    if slab_kset.is_null() {
        rust_slub_report_subsystem_error();
        return -(RSL_ENOMEM as i32);
    }
    0
}
#[cfg(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)))]
#[link_section = ".init.text"]
unsafe fn slab_sysfs_process_aliases() {
    while !alias_list.is_null() {
        let al = alias_list;
        alias_list = (*al).next;
        if sysfs_slab_alias((*al).s, (*al).name) != 0 {
            rust_slub_report_sysfs_alias_error((*al).name);
        }
        kfree(al.cast());
    }
}
#[cfg(any(
    all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY)),
    all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS)
))]
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn slab_late_init() -> i32 {
    rust_slub_mutex_lock(addr_of_mut!(slab_mutex));
    let err = slab_kset_init();
    if err == 0 {
        slab_debugfs_root_init();
        slab_state = FULL;
        let head = addr_of_mut!(slab_caches);
        let mut pos = (*head).next;
        while pos != head {
            let s = rust_slub_cache_from_list(pos);
            if sysfs_slab_add(s) != 0 {
                rust_slub_report_sysfs_cache_error((*s).name, true);
            }
            if (*s).flags & RSL_SLAB_STORE_USER != 0 {
                debugfs_slab_add(s);
            }
            pos = (*pos).next;
        }
        slab_sysfs_process_aliases();
    }
    rust_slub_mutex_unlock(addr_of_mut!(slab_mutex));
    err
}

#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debugfs_show(seq: *mut seq_file, _v: *mut Void) -> i32 {
    let t = (*seq).private.cast::<loc_track>();
    let idx = (*t).idx as ULong;
    if idx < (*t).count {
        let l = (*t).loc.add(idx as usize);
        rust_slub_seq_count(seq, (*l).count);
        if (*l).addr != 0 {
            rust_slub_seq_symbol(seq, (*l).addr as *const Void);
        } else {
            rust_slub_seq_puts(seq, c"<not-available>".as_ptr().cast::<CChar>());
        }
        if (*l).waste != 0 {
            rust_slub_seq_waste(seq, (*l).count.wrapping_mul((*l).waste), (*l).waste);
        }
        if (*l).sum_time != (*l).min_time as i64 {
            rust_slub_seq_age_range(
                seq,
                (*l).min_time,
                rust_slub_div_u64((*l).sum_time as u64, (*l).count as u32),
                (*l).max_time,
            );
        } else {
            rust_slub_seq_age(seq, (*l).min_time);
        }
        if (*l).min_pid != (*l).max_pid {
            rust_slub_seq_pid_range(seq, (*l).min_pid, (*l).max_pid);
        } else {
            rust_slub_seq_pid(seq, (*l).min_pid);
        }
        if rust_slub_num_online_cpus() > 1 && !rust_slub_location_cpus_empty(l) {
            rust_slub_seq_location_cpus(seq, l);
        }
        if rust_slub_nr_online_nodes() > 1 && !rust_slub_nodes_empty(addr_of!((*l).nodes)) {
            rust_slub_seq_nodes(seq, addr_of!((*l).nodes));
        }
        #[cfg(CONFIG_STACKDEPOT)]
        {
            let handle = rust_slub_location_handle_read(l);
            if handle != 0 {
                let mut entries: *mut ULong = null_mut();
                let nr = rust_slub_stack_depot_fetch(handle, &mut entries);
                rust_slub_seq_puts(seq, c"\n".as_ptr().cast::<CChar>());
                let mut j = 0;
                while j < nr {
                    rust_slub_seq_stack_symbol(seq, *entries.add(j as usize) as *const Void);
                    j += 1;
                }
            }
        }
        rust_slub_seq_puts(seq, c"\n".as_ptr().cast::<CChar>());
    }
    if idx == 0 && (*t).count == 0 {
        rust_slub_seq_puts(seq, c"No data\n".as_ptr().cast::<CChar>());
    }
    0
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debugfs_stop(_seq: *mut seq_file, _v: *mut Void) { /* Original callback is intentionally empty. */
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debugfs_next(
    seq: *mut seq_file,
    _v: *mut Void,
    ppos: *mut loff_t,
) -> *mut Void {
    let t = (*seq).private.cast::<loc_track>();
    *ppos += 1;
    (*t).idx = *ppos;
    if *ppos as ULong <= (*t).count {
        ppos.cast()
    } else {
        null_mut()
    }
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe extern "C" fn cmp_loc_by_count(a: *const Void, b: *const Void) -> i32 {
    let lhs = (*(b as *const location)).count;
    let rhs = (*(a as *const location)).count;
    (lhs > rhs) as i32 - (lhs < rhs) as i32
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debugfs_start(seq: *mut seq_file, ppos: *mut loff_t) -> *mut Void {
    let t = (*seq).private.cast::<loc_track>();
    (*t).idx = *ppos;
    ppos.cast()
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debug_trace_open(inode: *mut inode, filep: *mut file) -> i32 {
    let t = rust_slub_seq_open_private(filep).cast::<loc_track>();
    let s = rust_slub_file_inode_private(filep).cast::<kmem_cache>();
    if t.is_null() {
        return -(RSL_ENOMEM as i32);
    }
    let obj_map = rust_slub_bitmap_alloc(oo_objects((*s).oo), RSL_GFP_KERNEL);
    if obj_map.is_null() {
        rust_slub_seq_release_private(inode, filep);
        return -(RSL_ENOMEM as i32);
    }
    let alloc = rust_slub_debugfs_get_aux_num(filep) as track_item;
    if alloc_loc_track(
        t,
        RSL_PAGE_SIZE as ULong / size_of::<location>() as ULong,
        RSL_GFP_KERNEL,
    ) == 0
    {
        rust_slub_bitmap_free(obj_map);
        rust_slub_seq_release_private(inode, filep);
        return -(RSL_ENOMEM as i32);
    }
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() && node_nr_slabs(n) != 0 {
            let flags = rust_slub_node_lock_irqsave(n);
            let head = addr_of_mut!((*n).partial);
            let mut pos = (*head).next;
            while pos != head {
                process_slab(t, s, rust_slub_slab_from_list(pos), alloc, obj_map);
                pos = (*pos).next;
            }
            let head = addr_of_mut!((*n).full);
            let mut pos = (*head).next;
            while pos != head {
                process_slab(t, s, rust_slub_slab_from_list(pos), alloc, obj_map);
                pos = (*pos).next;
            }
            rust_slub_node_unlock_irqrestore(n, flags);
        }
        node += 1;
    }
    rust_slub_sort(
        (*t).loc.cast(),
        (*t).count as usize,
        size_of::<location>(),
        Some(cmp_loc_by_count),
    );
    rust_slub_bitmap_free(obj_map);
    0
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn slab_debug_trace_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data.cast::<seq_file>();
    free_loc_track((*seq).private.cast::<loc_track>());
    rust_slub_seq_release_private(inode, file)
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
unsafe fn debugfs_slab_add(s: *mut kmem_cache) {
    if slab_debugfs_root.is_null() {
        return;
    }
    let dir = rust_slub_debugfs_create_dir((*s).name, slab_debugfs_root);
    rust_slub_debugfs_create_trace(
        c"alloc_traces".as_ptr().cast::<CChar>(),
        dir,
        s,
        TRACK_ALLOC,
    );
    rust_slub_debugfs_create_trace(c"free_traces".as_ptr().cast::<CChar>(), dir, s, TRACK_FREE);
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[no_mangle]
unsafe extern "C" fn debugfs_slab_release(s: *mut kmem_cache) {
    if slab_debugfs_root.is_null() {
        return;
    }
    rust_slub_debugfs_lookup_and_remove((*s).name, slab_debugfs_root);
}
#[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_DEBUG_FS))]
#[link_section = ".init.text"]
unsafe fn slab_debugfs_root_init() {
    slab_debugfs_root = rust_slub_debugfs_create_dir(c"slab".as_ptr().cast::<CChar>(), null_mut());
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
unsafe extern "C" fn get_slabinfo(s: *mut kmem_cache, sinfo: *mut slabinfo) {
    let mut nr_slabs = 0 as ULong;
    let mut nr_objs = 0 as ULong;
    let mut nr_free = 0 as ULong;
    let mut node = 0;
    while node < rust_slub_nr_node_ids() as i32 {
        let n = get_node(s, node);
        if !n.is_null() {
            nr_slabs += node_nr_slabs(n);
            nr_objs += node_nr_objs(n);
            nr_free += count_partial_free_approx(n);
        }
        node += 1;
    }
    (*sinfo).active_objs = nr_objs - nr_free;
    (*sinfo).num_objs = nr_objs;
    (*sinfo).active_slabs = nr_slabs;
    (*sinfo).num_slabs = nr_slabs;
    (*sinfo).objects_per_slab = oo_objects((*s).oo);
    (*sinfo).cache_order = oo_order((*s).oo);
}

// Native capture records this Rust callsite for C's original _THIS_IP_ uses.
#[no_mangle]
unsafe extern "C" fn rust_slub_free_one_at_ip_body(
    s: *mut kmem_cache,
    slab: *mut slab,
    object: *mut Void,
    ip: ULong,
) {
    __slab_free(s, slab, object, object, 1, ip);
}
