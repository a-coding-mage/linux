// SPDX-License-Identifier: GPL-2.0
// mm/slub.c:2807-3323, percpu sheaf and per-node barn ownership.
unsafe fn __alloc_empty_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    mut alloc_flags: u32,
    capacity: u32,
) -> *mut slab_sheaf {
    if (*s).flags & RSL_SLAB_KMALLOC != 0 {
        alloc_flags |= RSL_SLAB_ALLOC_NO_RECURSE;
    }
    let sheaf_size = (capacity as usize)
        .checked_mul(size_of::<*mut Void>())
        .and_then(|n| n.checked_add(size_of::<slab_sheaf>()))
        .unwrap_or(usize::MAX);
    let sheaf = rust_slub_kmalloc_flags_sheaf(
        sheaf_size,
        gfp | RSL___GFP_ZERO,
        alloc_flags,
        RSL_NUMA_NO_NODE as i32,
    )
    .cast::<slab_sheaf>();
    if sheaf.is_null() {
        return null_mut();
    }
    (*sheaf).cache = s;
    stat(s, SHEAF_ALLOC);
    sheaf
}
#[inline]
unsafe fn alloc_empty_sheaf(
    s: *mut kmem_cache,
    mut gfp: gfp_t,
    alloc_flags: u32,
) -> *mut slab_sheaf {
    if alloc_flags & RSL_SLAB_ALLOC_NO_RECURSE != 0 {
        return null_mut();
    }
    gfp &= !RSL_OBJCGS_CLEAR_MASK;
    __alloc_empty_sheaf(s, gfp, alloc_flags, (*s).sheaf_capacity)
}
unsafe fn __free_empty_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf, free_flags: u32) {
    if (*s).flags & RSL_SLAB_KMALLOC != 0 {
        mark_obj_codetag_empty(sheaf.cast());
    }
    rust_slub_warn_sheaf_nonempty((*sheaf).size > 0);
    if free_flags & RSL_SLAB_FREE_NOLOCK != 0 {
        rust_slub_kfree_nolock(sheaf.cast());
    } else {
        kfree(sheaf.cast());
    }
    stat(s, SHEAF_FREE);
}
unsafe fn free_empty_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf) {
    __free_empty_sheaf(s, sheaf, RSL_SLAB_FREE_DEFAULT);
}
unsafe fn refill_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf, gfp: gfp_t) -> i32 {
    let to_fill = (*s).sheaf_capacity.wrapping_sub((*sheaf).size) as i32;
    if to_fill == 0 {
        return 0;
    }
    let filled = refill_objects(
        s,
        (*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize),
        gfp,
        to_fill as u32,
        to_fill as u32,
    ) as i32;
    (*sheaf).size = (*sheaf).size.wrapping_add(filled as u32);
    stat_add(s, SHEAF_REFILL, filled);
    if filled < to_fill {
        -(ENOMEM as i32)
    } else {
        0
    }
}
unsafe fn __sheaf_flush_main_batch(s: *mut kmem_cache) -> u32 {
    let mut objects = [null_mut(); RSL_PCS_BATCH_MAX as usize];
    rust_slub_pcs_assert_held(s);
    slab_attach_kprobe_locked();
    let pcs = rust_slub_this_cpu_sheaves(s);
    let sheaf = (*pcs).main;
    let batch = min(RSL_PCS_BATCH_MAX, (*sheaf).size);
    (*sheaf).size -= batch;
    core::ptr::copy_nonoverlapping(
        (*sheaf).objects.as_ptr().add((*sheaf).size as usize),
        objects.as_mut_ptr(),
        batch as usize,
    );
    let remaining = (*sheaf).size;
    rust_slub_pcs_unlock(s);
    __kmem_cache_free_bulk(s, batch as usize, objects.as_mut_ptr());
    stat_add(s, SHEAF_FLUSH, batch as i32);
    remaining
}
unsafe fn sheaf_flush_main(s: *mut kmem_cache) {
    loop {
        rust_slub_pcs_lock(s);
        if __sheaf_flush_main_batch(s) == 0 {
            break;
        }
    }
}
unsafe fn sheaf_try_flush_main(s: *mut kmem_cache) -> bool {
    let mut ret = false;
    loop {
        if !rust_slub_pcs_trylock(s) {
            return ret;
        }
        ret = true;
        if __sheaf_flush_main_batch(s) == 0 {
            return ret;
        }
    }
}
unsafe fn sheaf_flush_unused(s: *mut kmem_cache, sheaf: *mut slab_sheaf) {
    if (*sheaf).size == 0 {
        return;
    }
    stat_add(s, SHEAF_FLUSH, (*sheaf).size as i32);
    __kmem_cache_free_bulk(s, (*sheaf).size as usize, (*sheaf).objects.as_mut_ptr());
    (*sheaf).size = 0;
}
unsafe fn __rcu_free_sheaf_prepare(s: *mut kmem_cache, sheaf: *mut slab_sheaf) -> bool {
    let init = rust_slub_slab_want_init_on_free(s);
    let p = (*sheaf).objects.as_mut_ptr();
    let mut i = 0;
    let mut pfmemalloc = false;
    while i < (*sheaf).size {
        let slab = rust_slub_virt_to_slab(*p.add(i as usize));
        memcg_slab_free_hook(s, slab, p.add(i as usize), 1);
        alloc_tagging_slab_free_hook(s, slab, p.add(i as usize), 1);
        if !slab_free_hook(s, *p.add(i as usize), init, true) {
            (*sheaf).size -= 1;
            *p.add(i as usize) = *p.add((*sheaf).size as usize);
            continue;
        }
        if slab_test_pfmemalloc(slab) {
            pfmemalloc = true;
        }
        i += 1;
    }
    pfmemalloc
}
unsafe extern "C" fn rcu_free_sheaf_nobarn(head: *mut rcu_head) {
    let sheaf = rust_slub_sheaf_from_rcu(head);
    let s = (*sheaf).cache;
    __rcu_free_sheaf_prepare(s, sheaf);
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
}
unsafe fn pcs_flush_all(s: *mut kmem_cache) {
    rust_slub_pcs_lock(s);
    let pcs = rust_slub_this_cpu_sheaves(s);
    let spare = (*pcs).spare;
    (*pcs).spare = null_mut();
    let rcu_free = (*pcs).rcu_free;
    (*pcs).rcu_free = null_mut();
    rust_slub_pcs_unlock(s);
    if !spare.is_null() {
        sheaf_flush_unused(s, spare);
        free_empty_sheaf(s, spare);
    }
    if !rcu_free.is_null() {
        call_rcu(rust_slub_sheaf_rcu(rcu_free), Some(rcu_free_sheaf_nobarn));
    }
    sheaf_flush_main(s);
}
unsafe fn __pcs_flush_all_cpu(s: *mut kmem_cache, cpu: u32) {
    let pcs = rust_slub_percpu_sheaves(s, cpu);
    sheaf_flush_unused(s, (*pcs).main);
    if !(*pcs).spare.is_null() {
        sheaf_flush_unused(s, (*pcs).spare);
        free_empty_sheaf(s, (*pcs).spare);
        (*pcs).spare = null_mut();
    }
    if !(*pcs).rcu_free.is_null() {
        call_rcu(
            rust_slub_sheaf_rcu((*pcs).rcu_free),
            Some(rcu_free_sheaf_nobarn),
        );
        (*pcs).rcu_free = null_mut();
    }
}
unsafe fn pcs_destroy(s: *mut kmem_cache) {
    if (*s).cpu_sheaves.is_null() {
        return;
    }
    if rust_slub_cache_has_sheaves(s) {
        let mut cpu = rust_slub_next_possible_cpu(0);
        while cpu < rust_slub_nr_cpu_ids() {
            let pcs = rust_slub_percpu_sheaves(s, cpu);
            if !(*pcs).main.is_null() {
                rust_slub_warn_pcs_spare(!(*pcs).spare.is_null());
                rust_slub_warn_pcs_rcu(!(*pcs).rcu_free.is_null());
                if !rust_slub_warn_pcs_main((*(*pcs).main).size != 0) {
                    free_empty_sheaf(s, (*pcs).main);
                    (*pcs).main = null_mut();
                }
            }
            cpu = rust_slub_next_possible_cpu(cpu + 1);
        }
    }
    free_percpu((*s).cpu_sheaves.cast());
    (*s).cpu_sheaves = null_mut();
}
unsafe fn barn_get_empty_sheaf(barn: *mut node_barn, allow_spin: bool) -> *mut slab_sheaf {
    if rust_slub_data_race_uint(addr_of!((*barn).nr_empty)) == 0 {
        return null_mut();
    }
    let mut flags = 0;
    if allow_spin {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*barn).lock), &mut flags) {
        return null_mut();
    }
    let mut empty = null_mut();
    if (*barn).nr_empty != 0 {
        empty = rust_slub_sheaf_from_list((*barn).sheaves_empty.next);
        rust_slub_list_del(rust_slub_sheaf_list(empty));
        (*barn).nr_empty -= 1;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
    empty
}
unsafe fn barn_put_empty_sheaf(barn: *mut node_barn, sheaf: *mut slab_sheaf) {
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    rust_slub_list_add(
        rust_slub_sheaf_list(sheaf),
        addr_of_mut!((*barn).sheaves_empty),
    );
    (*barn).nr_empty += 1;
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
}
unsafe fn barn_put_full_sheaf(barn: *mut node_barn, sheaf: *mut slab_sheaf) {
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    rust_slub_list_add(
        rust_slub_sheaf_list(sheaf),
        addr_of_mut!((*barn).sheaves_full),
    );
    (*barn).nr_full += 1;
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
}
unsafe fn barn_get_full_or_empty_sheaf(barn: *mut node_barn) -> *mut slab_sheaf {
    if rust_slub_data_race_uint(addr_of!((*barn).nr_full)) == 0
        && rust_slub_data_race_uint(addr_of!((*barn).nr_empty)) == 0
    {
        return null_mut();
    }
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    let mut sheaf = null_mut();
    if (*barn).nr_full != 0 {
        sheaf = rust_slub_sheaf_from_list((*barn).sheaves_full.next);
        rust_slub_list_del(rust_slub_sheaf_list(sheaf));
        (*barn).nr_full -= 1;
    } else if (*barn).nr_empty != 0 {
        sheaf = rust_slub_sheaf_from_list((*barn).sheaves_empty.next);
        rust_slub_list_del(rust_slub_sheaf_list(sheaf));
        (*barn).nr_empty -= 1;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
    sheaf
}
unsafe fn barn_replace_empty_sheaf(
    barn: *mut node_barn,
    empty: *mut slab_sheaf,
    allow_spin: bool,
) -> *mut slab_sheaf {
    if rust_slub_data_race_uint(addr_of!((*barn).nr_full)) == 0 {
        return null_mut();
    }
    let mut flags = 0;
    if allow_spin {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*barn).lock), &mut flags) {
        return null_mut();
    }
    let mut full = null_mut();
    if (*barn).nr_full != 0 {
        full = rust_slub_sheaf_from_list((*barn).sheaves_full.next);
        rust_slub_list_del(rust_slub_sheaf_list(full));
        rust_slub_list_add(
            rust_slub_sheaf_list(empty),
            addr_of_mut!((*barn).sheaves_empty),
        );
        (*barn).nr_full -= 1;
        (*barn).nr_empty += 1;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
    full
}
unsafe fn barn_replace_full_sheaf(
    barn: *mut node_barn,
    full: *mut slab_sheaf,
    allow_spin: bool,
) -> *mut slab_sheaf {
    if rust_slub_data_race_uint(addr_of!((*barn).nr_full)) >= RSL_MAX_FULL_SHEAVES {
        return (-(E2BIG as isize)) as *mut slab_sheaf;
    }
    if rust_slub_data_race_uint(addr_of!((*barn).nr_empty)) == 0 {
        return (-(ENOMEM as isize)) as *mut slab_sheaf;
    }
    let mut flags = 0;
    if allow_spin {
        flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    } else if !rust_slub_spin_trylock_irqsave(addr_of_mut!((*barn).lock), &mut flags) {
        return (-(EBUSY as isize)) as *mut slab_sheaf;
    }
    let empty;
    if (*barn).nr_empty != 0 {
        empty = rust_slub_sheaf_from_list((*barn).sheaves_empty.next);
        rust_slub_list_del(rust_slub_sheaf_list(empty));
        rust_slub_list_add(
            rust_slub_sheaf_list(full),
            addr_of_mut!((*barn).sheaves_full),
        );
        (*barn).nr_empty -= 1;
        (*barn).nr_full += 1;
    } else {
        empty = (-(ENOMEM as isize)) as *mut slab_sheaf;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
    empty
}
unsafe fn barn_init(barn: *mut node_barn) {
    rust_slub_spin_lock_init(addr_of_mut!((*barn).lock));
    rust_slub_init_list_head(addr_of_mut!((*barn).sheaves_full));
    rust_slub_init_list_head(addr_of_mut!((*barn).sheaves_empty));
    (*barn).nr_full = 0;
    (*barn).nr_empty = 0;
}
unsafe fn barn_shrink(s: *mut kmem_cache, barn: *mut node_barn) {
    let mut empty_list: list_head = zeroed();
    let mut full_list: list_head = zeroed();
    rust_slub_init_list_head(&mut empty_list);
    rust_slub_init_list_head(&mut full_list);
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*barn).lock));
    rust_slub_list_splice_init(addr_of_mut!((*barn).sheaves_full), &mut full_list);
    (*barn).nr_full = 0;
    rust_slub_list_splice_init(addr_of_mut!((*barn).sheaves_empty), &mut empty_list);
    (*barn).nr_empty = 0;
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*barn).lock), flags);
    let mut entry = full_list.next;
    while entry != addr_of_mut!(full_list) {
        let next = (*entry).next;
        let sheaf = rust_slub_sheaf_from_list(entry);
        sheaf_flush_unused(s, sheaf);
        free_empty_sheaf(s, sheaf);
        entry = next;
    }
    entry = empty_list.next;
    while entry != addr_of_mut!(empty_list) {
        let next = (*entry).next;
        free_empty_sheaf(s, rust_slub_sheaf_from_list(entry));
        entry = next;
    }
}
unsafe fn __pcs_replace_empty_main(
    s: *mut kmem_cache,
    mut pcs: *mut slub_percpu_sheaves,
    gfp: gfp_t,
    alloc_flags: u32,
) -> *mut slub_percpu_sheaves {
    rust_slub_pcs_assert_held(s);
    slab_attach_kprobe_locked();
    if !rust_slub_cache_has_sheaves(s) {
        rust_slub_pcs_unlock(s);
        return null_mut();
    }
    if !(*pcs).spare.is_null() && (*(*pcs).spare).size > 0 {
        core::ptr::swap(addr_of_mut!((*pcs).main), addr_of_mut!((*pcs).spare));
        return pcs;
    }
    let barn = get_barn(s);
    if barn.is_null() {
        rust_slub_pcs_unlock(s);
        return null_mut();
    }
    let allow_spin = alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
    let mut full = barn_replace_empty_sheaf(barn, (*pcs).main, allow_spin);
    if !full.is_null() {
        stat(s, BARN_GET);
        (*pcs).main = full;
        return pcs;
    }
    stat(s, BARN_GET_FAIL);
    let mut empty = null_mut();
    if allow_spin {
        if !(*pcs).spare.is_null() {
            empty = (*pcs).spare;
            (*pcs).spare = null_mut();
        } else {
            empty = barn_get_empty_sheaf(barn, true);
        }
    }
    rust_slub_pcs_unlock(s);
    pcs = null_mut();
    if !allow_spin {
        return null_mut();
    }
    if empty.is_null() {
        empty = alloc_empty_sheaf(s, gfp, alloc_flags);
        if empty.is_null() {
            return null_mut();
        }
    }
    if refill_sheaf(s, empty, gfp | RSL___GFP_NOMEMALLOC | RSL___GFP_NOWARN) != 0 {
        sheaf_flush_unused(s, empty);
        free_empty_sheaf(s, empty);
        return null_mut();
    }
    full = empty;
    if rust_slub_pcs_trylock(s) {
        pcs = rust_slub_this_cpu_sheaves(s);
        if (*(*pcs).main).size == 0 {
            if (*pcs).spare.is_null() {
                (*pcs).spare = (*pcs).main;
            } else {
                barn_put_empty_sheaf(barn, (*pcs).main);
            }
            (*pcs).main = full;
            return pcs;
        }
        if (*pcs).spare.is_null() {
            (*pcs).spare = full;
            return pcs;
        }
        if (*(*pcs).spare).size == 0 {
            barn_put_empty_sheaf(barn, (*pcs).spare);
            (*pcs).spare = full;
            return pcs;
        }
    }
    barn_put_full_sheaf(barn, full);
    stat(s, BARN_PUT);
    pcs
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn alloc_from_pcs(s: *mut kmem_cache, gfp: gfp_t, alloc_flags: u32, node: i32) -> *mut Void {
    let node_requested = cfg!(CONFIG_NUMA) && node != RSL_NUMA_NO_NODE as i32;
    if node_requested && node != rust_slub_numa_mem_id() {
        stat(s, ALLOC_NODE_MISMATCH);
        return null_mut();
    }
    if !rust_slub_pcs_trylock(s) {
        return null_mut();
    }
    let mut pcs = rust_slub_this_cpu_sheaves(s);
    if (*(*pcs).main).size == 0 {
        pcs = __pcs_replace_empty_main(s, pcs, gfp, alloc_flags);
        if pcs.is_null() {
            return null_mut();
        }
    }
    let object = *(*(*pcs).main)
        .objects
        .as_ptr()
        .add((*(*pcs).main).size as usize - 1);
    if node_requested && rust_slub_page_to_nid(rust_slub_virt_to_page(object)) != node {
        rust_slub_pcs_unlock(s);
        stat(s, ALLOC_NODE_MISMATCH);
        return null_mut();
    }
    (*(*pcs).main).size -= 1;
    rust_slub_pcs_unlock(s);
    stat(s, ALLOC_FASTPATH);
    object
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn alloc_from_pcs_bulk(s: *mut kmem_cache, mut size: usize, mut p: *mut *mut Void) -> u32 {
    let mut allocated = 0;
    loop {
        if !rust_slub_pcs_trylock(s) {
            return allocated;
        }
        let pcs = rust_slub_this_cpu_sheaves(s);
        if (*(*pcs).main).size == 0 {
            if !rust_slub_cache_has_sheaves(s) {
                rust_slub_pcs_unlock(s);
                return allocated;
            }
            if !(*pcs).spare.is_null() && (*(*pcs).spare).size > 0 {
                core::ptr::swap(addr_of_mut!((*pcs).main), addr_of_mut!((*pcs).spare));
            } else {
                let barn = get_barn(s);
                if barn.is_null() {
                    rust_slub_pcs_unlock(s);
                    return allocated;
                }
                let full = barn_replace_empty_sheaf(barn, (*pcs).main, true);
                if !full.is_null() {
                    stat(s, BARN_GET);
                    (*pcs).main = full;
                } else {
                    stat(s, BARN_GET_FAIL);
                    rust_slub_pcs_unlock(s);
                    return allocated;
                }
            }
        }
        let main = (*pcs).main;
        let batch = min(size, (*main).size as usize) as u32;
        (*main).size -= batch;
        core::ptr::copy_nonoverlapping(
            (*main).objects.as_ptr().add((*main).size as usize),
            p,
            batch as usize,
        );
        rust_slub_pcs_unlock(s);
        stat_add(s, ALLOC_FASTPATH, batch as i32);
        allocated += batch;
        if batch as usize >= size {
            return allocated;
        }
        p = p.add(batch as usize);
        size -= batch as usize;
    }
}
