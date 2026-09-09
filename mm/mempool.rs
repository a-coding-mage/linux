// SPDX-License-Identifier: GPL-2.0
/*
 * memory buffer pool support. Such pools are mostly used
 * for guaranteed, deadlock-free memory allocations during
 * extreme VM load.
 */

// Kernel dependencies supplied by other translation units.

static mut fail_mempool_alloc: fault_attr = fault_attr;
static mut fail_mempool_alloc_bulk: fault_attr = fault_attr;
static mut mempool_debug_enabled: static_key_false = static_key_false;

unsafe extern "C" {
    static mut _RET_IP_: usize;
}

unsafe fn mempool_debug_setup(_str: *mut c_char) -> c_int {
    static_branch_enable(&raw mut mempool_debug_enabled);
    1
}

unsafe fn mempool_faul_inject_init() -> c_int {
    let mut error: c_int;
    error = PTR_ERR_OR_ZERO(fault_create_debugfs_attr(
        c"fail_mempool_alloc".as_ptr(), core::ptr::null_mut(),
        &raw mut fail_mempool_alloc));
    if error != 0 { return error; }
    PTR_ERR_OR_ZERO(fault_create_debugfs_attr(
        c"fail_mempool_alloc_bulk".as_ptr(), core::ptr::null_mut(),
        &raw mut fail_mempool_alloc_bulk))
}

unsafe fn poison_error(pool: *mut mempool, element: *mut c_void, size: usize, byte: usize) {
    let nr = (*pool).curr_nr;
    let start = core::cmp::max(byte as isize - (BITS_PER_LONG / 8) as isize, 0) as usize;
    let end = core::cmp::min(byte + BITS_PER_LONG / 8, size);
    pr_err!("BUG: mempool element poison mismatch\n");
    pr_err!("Mempool %p size %zu\n", pool, size);
    pr_err!(" nr=%d @ %p: %s0x", nr, element, if start > 0 { "... " } else { "" });
    for i in start..end { pr_cont!("%x ", *((element as *mut u8).add(i))); }
    pr_cont!("%s\n", if end < size { "..." } else { "" });
    dump_stack();
}

unsafe fn __check_element(pool: *mut mempool, element: *mut c_void, size: usize) {
    let obj = element as *mut u8;
    for i in 0..size {
        let exp = if i < size - 1 { POISON_FREE } else { POISON_END };
        if *obj.add(i) != exp { poison_error(pool, element, size, i); return; }
    }
    memset(obj as *mut c_void, POISON_INUSE as c_int, size);
}

unsafe fn check_element(pool: *mut mempool, element: *mut c_void) {
    if kasan_enabled() { return; }
    if (*pool).free == Some(mempool_kfree) {
        __check_element(pool, element, (*pool).pool_data as usize);
    } else if (*pool).free == Some(mempool_free_slab) {
        __check_element(pool, element, kmem_cache_size((*pool).pool_data));
    } else if (*pool).free == Some(mempool_free_pages) {
        let order = (*pool).pool_data as isize as c_int;
        let addr = page_address(element as *mut page);
        __check_element(pool, addr, PAGE_SIZE << order);
    }
}

unsafe fn __poison_element(element: *mut c_void, size: usize) {
    let obj = element as *mut u8;
    memset(obj as *mut c_void, POISON_FREE as c_int, size - 1);
    *obj.add(size - 1) = POISON_END;
}

unsafe fn poison_element(pool: *mut mempool, element: *mut c_void) {
    if kasan_enabled() { return; }
    if (*pool).alloc == Some(mempool_kmalloc) {
        __poison_element(element, (*pool).pool_data as usize);
    } else if (*pool).alloc == Some(mempool_alloc_slab) {
        __poison_element(element, kmem_cache_size((*pool).pool_data));
    } else if (*pool).alloc == Some(mempool_alloc_pages) {
        let order = (*pool).pool_data as isize as c_int;
        let addr = page_address(element as *mut page);
        __poison_element(addr, PAGE_SIZE << order);
    }
}

unsafe fn kasan_poison_element(pool: *mut mempool, element: *mut c_void) -> bool {
    if (*pool).alloc == Some(mempool_alloc_slab) || (*pool).alloc == Some(mempool_kmalloc) {
        return kasan_mempool_poison_object(element);
    } else if (*pool).alloc == Some(mempool_alloc_pages) {
        return kasan_mempool_poison_pages(element, (*pool).pool_data as usize);
    }
    true
}

unsafe fn kasan_unpoison_element(pool: *mut mempool, element: *mut c_void) {
    if (*pool).alloc == Some(mempool_kmalloc) { kasan_mempool_unpoison_object(element, (*pool).pool_data as usize); }
    else if (*pool).alloc == Some(mempool_alloc_slab) { kasan_mempool_unpoison_object(element, kmem_cache_size((*pool).pool_data)); }
    else if (*pool).alloc == Some(mempool_alloc_pages) { kasan_mempool_unpoison_pages(element, (*pool).pool_data as usize); }
}

unsafe fn add_element(pool: *mut mempool, element: *mut c_void) {
    BUG_ON!((*pool).min_nr != 0 && (*pool).curr_nr >= (*pool).min_nr);
    if static_branch_unlikely(&raw mut mempool_debug_enabled) { poison_element(pool, element); }
    if kasan_poison_element(pool, element) { *(*pool).elements.add((*pool).curr_nr as usize) = element; (*pool).curr_nr += 1; }
}

unsafe fn remove_element(pool: *mut mempool) -> *mut c_void {
    (*pool).curr_nr -= 1;
    BUG_ON!((*pool).curr_nr < 0);
    let element = *(*pool).elements.add((*pool).curr_nr as usize);
    kasan_unpoison_element(pool, element);
    if static_branch_unlikely(&raw mut mempool_debug_enabled) { check_element(pool, element); }
    element
}

pub unsafe fn mempool_exit(pool: *mut mempool) {
    while (*pool).curr_nr != 0 { let element = remove_element(pool); ((*pool).free.unwrap())(element, (*pool).pool_data); }
    kfree((*pool).elements as *mut c_void);
    (*pool).elements = core::ptr::null_mut();
}

pub unsafe fn mempool_destroy(pool: *mut mempool) {
    if pool.is_null() { return; }
    mempool_exit(pool);
    kfree(pool as *mut c_void);
}

pub unsafe fn mempool_init_node(pool: *mut mempool, min_nr: c_int, alloc_fn: mempool_alloc_t, free_fn: mempool_free_t, pool_data: *mut c_void, gfp_mask: gfp_t, node_id: c_int) -> c_int {
    spin_lock_init(&mut (*pool).lock); (*pool).min_nr = min_nr; (*pool).pool_data = pool_data; (*pool).alloc = Some(alloc_fn); (*pool).free = Some(free_fn); init_waitqueue_head(&mut (*pool).wait);
    (*pool).elements = kmalloc_array_node(core::cmp::max(1, min_nr) as usize, core::mem::size_of::<*mut c_void>(), gfp_mask, node_id) as *mut *mut c_void;
    if (*pool).elements.is_null() { return -ENOMEM; }
    while (*pool).curr_nr < core::cmp::max(1, (*pool).min_nr) { let element = alloc_fn(gfp_mask, pool_data); if element.is_null() { mempool_exit(pool); return -ENOMEM; } add_element(pool, element); }
    0
}

pub unsafe fn mempool_init_noprof(pool: *mut mempool, min_nr: c_int, alloc_fn: mempool_alloc_t, free_fn: mempool_free_t, pool_data: *mut c_void) -> c_int { mempool_init_node(pool, min_nr, alloc_fn, free_fn, pool_data, GFP_KERNEL, NUMA_NO_NODE) }

pub unsafe fn mempool_create_node_noprof(min_nr: c_int, alloc_fn: mempool_alloc_t, free_fn: mempool_free_t, pool_data: *mut c_void, gfp_mask: gfp_t, node_id: c_int) -> *mut mempool {
    let pool = kmalloc_node_noprof(core::mem::size_of::<mempool>(), gfp_mask | __GFP_ZERO, node_id) as *mut mempool;
    if pool.is_null() { return core::ptr::null_mut(); }
    if mempool_init_node(pool, min_nr, alloc_fn, free_fn, pool_data, gfp_mask, node_id) != 0 { kfree(pool as *mut c_void); return core::ptr::null_mut(); }
    pool
}

pub unsafe fn mempool_resize(pool: *mut mempool, new_min_nr: c_int) -> c_int {
    BUG_ON!(new_min_nr <= 0); might_sleep();
    let mut flags = 0usize; spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    if new_min_nr <= (*pool).min_nr { while new_min_nr < (*pool).curr_nr { let e = remove_element(pool); spin_unlock_irqrestore(&mut (*pool).lock, flags); ((*pool).free.unwrap())(e, (*pool).pool_data); spin_lock_irqsave(&mut (*pool).lock, &mut flags); } (*pool).min_nr = new_min_nr; spin_unlock_irqrestore(&mut (*pool).lock, flags); return 0; }
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
    let new_elements = kmalloc_objs::<*mut c_void>(new_min_nr as usize); if new_elements.is_null() { return -ENOMEM; }
    spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    if new_min_nr <= (*pool).min_nr { spin_unlock_irqrestore(&mut (*pool).lock, flags); kfree(new_elements as *mut c_void); return 0; }
    core::ptr::copy_nonoverlapping((*pool).elements, new_elements, (*pool).curr_nr as usize); kfree((*pool).elements as *mut c_void); (*pool).elements = new_elements; (*pool).min_nr = new_min_nr;
    while (*pool).curr_nr < (*pool).min_nr { spin_unlock_irqrestore(&mut (*pool).lock, flags); let e = ((*pool).alloc.unwrap())(GFP_KERNEL, (*pool).pool_data); if e.is_null() { return 0; } spin_lock_irqsave(&mut (*pool).lock, &mut flags); if (*pool).curr_nr < (*pool).min_nr { add_element(pool, e); } else { spin_unlock_irqrestore(&mut (*pool).lock, flags); ((*pool).free.unwrap())(e, (*pool).pool_data); return 0; } }
    spin_unlock_irqrestore(&mut (*pool).lock, flags); 0
}

unsafe fn mempool_alloc_from_pool(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint, mut allocated: c_uint, gfp_mask: gfp_t) -> c_uint {
    let mut flags = 0usize; spin_lock_irqsave(&mut (*pool).lock, &mut flags); if (*pool).curr_nr < (count - allocated) as c_int { spin_unlock_irqrestore(&mut (*pool).lock, flags); return allocated; }
    while allocated < count { *elems.add(allocated as usize) = remove_element(pool); allocated += 1; } spin_unlock_irqrestore(&mut (*pool).lock, flags); smp_wmb(); for i in 0..count { kmemleak_update_trace(*elems.add(i as usize)); } allocated
}

unsafe fn mempool_adjust_gfp(gfp_mask: *mut gfp_t) -> gfp_t { *gfp_mask |= __GFP_NOMEMALLOC | __GFP_NORETRY | __GFP_NOWARN; *gfp_mask & !(__GFP_DIRECT_RECLAIM | __GFP_IO) }

pub unsafe fn mempool_alloc_bulk_noprof(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint) -> c_int { let mut mask = GFP_KERNEL; let temp = mempool_adjust_gfp(&mut mask); let mut allocated = 0; while allocated < count { *elems.add(allocated as usize) = ((*pool).alloc.unwrap())(temp, (*pool).pool_data); if (*elems.add(allocated as usize)).is_null() { allocated = mempool_alloc_from_pool(pool, elems, count, allocated, temp); continue; } allocated += 1; } 0 }

pub unsafe fn mempool_alloc_noprof(pool: *mut mempool, mut gfp_mask: gfp_t) -> *mut c_void { let mut temp = mempool_adjust_gfp(&mut gfp_mask); loop { let e = ((*pool).alloc.unwrap())(temp, (*pool).pool_data); if !e.is_null() { return e; } let mut out = core::ptr::null_mut(); if mempool_alloc_from_pool(pool, &mut out, 1, 0, temp) == 1 { return out; } if temp != gfp_mask { temp = gfp_mask; } else if gfp_mask & __GFP_DIRECT_RECLAIM == 0 { return core::ptr::null_mut(); } } }

pub unsafe fn mempool_alloc_preallocated(pool: *mut mempool) -> *mut c_void { let mut e = core::ptr::null_mut(); mempool_alloc_from_pool(pool, &mut e, 1, 0, GFP_NOWAIT); e }

pub unsafe fn mempool_free_bulk(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint) -> c_uint { smp_rmb(); let mut freed = 0; let mut flags = 0usize; if (*pool).curr_nr < (*pool).min_nr || ((*pool).min_nr == 0 && (*pool).curr_nr == 0) { spin_lock_irqsave(&mut (*pool).lock, &mut flags); while freed < count && (*pool).curr_nr < (*pool).min_nr || (freed == 0 && (*pool).min_nr == 0 && (*pool).curr_nr == 0) { add_element(pool, *elems.add(freed as usize)); freed += 1; } spin_unlock_irqrestore(&mut (*pool).lock, flags); } if freed != 0 && wq_has_sleeper(&mut (*pool).wait) { wake_up(&mut (*pool).wait); } freed }

pub unsafe fn mempool_free(element: *mut c_void, pool: *mut mempool) { if !element.is_null() && mempool_free_bulk(pool, &mut (element as *mut c_void), 1) == 0 { ((*pool).free.unwrap())(element, (*pool).pool_data); } }

pub unsafe fn mempool_alloc_slab(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void { let mem = pool_data as *mut kmem_cache; VM_BUG_ON!((*mem).ctor); kmem_cache_alloc_noprof(mem, gfp_mask) }
pub unsafe fn mempool_free_slab(element: *mut c_void, pool_data: *mut c_void) { kmem_cache_free(pool_data as *mut kmem_cache, element); }
pub unsafe fn mempool_kmalloc(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void { kmalloc_noprof(pool_data as usize, gfp_mask) }
pub unsafe fn mempool_kfree(element: *mut c_void, _pool_data: *mut c_void) { kfree(element); }
pub unsafe fn mempool_alloc_pages(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void { alloc_pages_noprof(gfp_mask, pool_data as isize as c_int) as *mut c_void }
pub unsafe fn mempool_free_pages(element: *mut c_void, pool_data: *mut c_void) { __free_pages(element as *mut page, pool_data as isize as c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
