// SPDX-License-Identifier: GPL-2.0-only
// External Linux kernel dependencies are supplied by other translation units.

#[repr(C)]
pub struct mb_cache {
    pub c_hash: *mut hlist_bl_head,
    pub c_bucket_bits: i32,
    pub c_max_entries: c_ulong,
    pub c_list_lock: spinlock_t,
    pub c_list: list_head,
    pub c_entry_count: c_ulong,
    pub c_shrink: *mut shrinker,
    pub c_shrink_work: work_struct,
}

extern "C" {
    static mut mb_entry_cache: *mut kmem_cache;
}

#[inline]
unsafe fn mb_cache_entry_head(cache: *mut mb_cache, key: u32) -> *mut hlist_bl_head {
    (*cache).c_hash.add(hash_32(key, (*cache).c_bucket_bits as u32) as usize)
}

pub const SYNC_SHRINK_BATCH: c_ulong = 64;

#[no_mangle]
pub unsafe extern "C" fn mb_cache_entry_create(
    cache: *mut mb_cache, mask: gfp_t, key: u32, value: u64, reusable: bool,
) -> i32 {
    let entry = kmem_cache_alloc(mb_entry_cache, mask) as *mut mb_cache_entry;
    if entry.is_null() { return -ENOMEM; }

    if (*cache).c_entry_count >= (*cache).c_max_entries { schedule_work(&mut (*cache).c_shrink_work); }
    if (*cache).c_entry_count >= 2 * (*cache).c_max_entries { mb_cache_shrink_impl(cache, SYNC_SHRINK_BATCH); }

    INIT_LIST_HEAD(&mut (*entry).e_list);
    atomic_set(&mut (*entry).e_refcnt, 2);
    (*entry).e_key = key;
    (*entry).e_value = value;
    (*entry).e_flags = 0;
    if reusable { set_bit(MBE_REUSABLE_B, &mut (*entry).e_flags); }
    let head = mb_cache_entry_head(cache, key);
    hlist_bl_lock(head);
    let mut dup_node = hlist_bl_first(head);
    while !dup_node.is_null() {
        let dup = hlist_bl_entry(dup_node, e_hash_list) as *mut mb_cache_entry;
        if (*dup).e_key == key && (*dup).e_value == value {
            hlist_bl_unlock(head); kmem_cache_free(mb_entry_cache, entry as *mut _); return -EBUSY;
        }
        dup_node = (*dup_node).next;
    }
    hlist_bl_add_head(&mut (*entry).e_hash_list, head);
    hlist_bl_unlock(head);
    spin_lock(&mut (*cache).c_list_lock);
    list_add_tail(&mut (*entry).e_list, &mut (*cache).c_list);
    (*cache).c_entry_count += 1;
    spin_unlock(&mut (*cache).c_list_lock);
    mb_cache_entry_put(cache, entry);
    0
}

pub unsafe extern "C" fn __mb_cache_entry_free(cache: *mut mb_cache, entry: *mut mb_cache_entry) {
    let head = mb_cache_entry_head(cache, (*entry).e_key);
    hlist_bl_lock(head); hlist_bl_del(&mut (*entry).e_hash_list); hlist_bl_unlock(head);
    kmem_cache_free(mb_entry_cache, entry as *mut _);
}

pub unsafe extern "C" fn mb_cache_entry_wait_unused(entry: *mut mb_cache_entry) {
    wait_var_event(&mut (*entry).e_refcnt, atomic_read(&(*entry).e_refcnt) <= 2);
}

unsafe fn __entry_find(cache: *mut mb_cache, old_entry: *mut mb_cache_entry, key: u32) -> *mut mb_cache_entry {
    let mut entry = old_entry;
    let head = mb_cache_entry_head(cache, key);
    hlist_bl_lock(head);
    let mut node = if !entry.is_null() && !hlist_bl_unhashed(&(*entry).e_hash_list) { (*entry).e_hash_list.next } else { hlist_bl_first(head) };
    while !node.is_null() {
        entry = hlist_bl_entry(node, e_hash_list) as *mut mb_cache_entry;
        if (*entry).e_key == key && test_bit(MBE_REUSABLE_B, &(*entry).e_flags) && atomic_inc_not_zero(&mut (*entry).e_refcnt) { break; }
        node = (*node).next;
    }
    if node.is_null() { entry = core::ptr::null_mut(); }
    hlist_bl_unlock(head);
    if !old_entry.is_null() { mb_cache_entry_put(cache, old_entry); }
    entry
}

pub unsafe extern "C" fn mb_cache_entry_find_first(cache: *mut mb_cache, key: u32) -> *mut mb_cache_entry { __entry_find(cache, core::ptr::null_mut(), key) }

pub unsafe extern "C" fn mb_cache_entry_find_next(cache: *mut mb_cache, entry: *mut mb_cache_entry) -> *mut mb_cache_entry { __entry_find(cache, entry, (*entry).e_key) }

pub unsafe extern "C" fn mb_cache_entry_get(cache: *mut mb_cache, key: u32, value: u64) -> *mut mb_cache_entry {
    let head = mb_cache_entry_head(cache, key); hlist_bl_lock(head);
    let mut node = hlist_bl_first(head);
    while !node.is_null() { let entry = hlist_bl_entry(node, e_hash_list) as *mut mb_cache_entry; if (*entry).e_key == key && (*entry).e_value == value && atomic_inc_not_zero(&mut (*entry).e_refcnt) { hlist_bl_unlock(head); return entry; } node = (*node).next; }
    hlist_bl_unlock(head); core::ptr::null_mut()
}

pub unsafe extern "C" fn mb_cache_entry_delete_or_get(cache: *mut mb_cache, key: u32, value: u64) -> *mut mb_cache_entry {
    let entry = mb_cache_entry_get(cache, key, value); if entry.is_null() { return core::ptr::null_mut(); }
    if atomic_cmpxchg(&mut (*entry).e_refcnt, 2, 0) != 2 { return entry; }
    spin_lock(&mut (*cache).c_list_lock); if !list_empty(&(*entry).e_list) { list_del_init(&mut (*entry).e_list); } (*cache).c_entry_count -= 1; spin_unlock(&mut (*cache).c_list_lock);
    __mb_cache_entry_free(cache, entry); core::ptr::null_mut()
}

pub unsafe extern "C" fn mb_cache_entry_touch(_cache: *mut mb_cache, entry: *mut mb_cache_entry) { set_bit(MBE_REFERENCED_B, &mut (*entry).e_flags); }

// The remaining cache lifecycle and shrinker entry points retain their kernel implementation contract.
pub const SHRINK_DIVISOR: c_ulong = 16;

unsafe fn mb_cache_count(shrink: *mut shrinker, _sc: *mut shrink_control) -> c_ulong {
    (*(*shrink).private_data.cast::<mb_cache>()).c_entry_count
}

unsafe fn mb_cache_shrink_impl(cache: *mut mb_cache, mut nr_to_scan: c_ulong) -> c_ulong {
    let mut shrunk = 0;
    spin_lock(&mut (*cache).c_list_lock);
    while nr_to_scan != 0 && !list_empty(&(*cache).c_list) {
        nr_to_scan -= 1;
        let entry = list_first_entry(&mut (*cache).c_list, e_list) as *mut mb_cache_entry;
        if test_bit(MBE_REFERENCED_B, &(*entry).e_flags) || atomic_cmpxchg(&mut (*entry).e_refcnt, 1, 0) != 1 {
            clear_bit(MBE_REFERENCED_B, &mut (*entry).e_flags);
            list_move_tail(&mut (*entry).e_list, &mut (*cache).c_list);
            continue;
        }
        list_del_init(&mut (*entry).e_list); (*cache).c_entry_count -= 1;
        spin_unlock(&mut (*cache).c_list_lock); __mb_cache_entry_free(cache, entry); shrunk += 1; cond_resched(); spin_lock(&mut (*cache).c_list_lock);
    }
    spin_unlock(&mut (*cache).c_list_lock); shrunk
}

unsafe fn mb_cache_scan(shrink: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    mb_cache_shrink_impl((*shrink).private_data.cast(), (*sc).nr_to_scan)
}

unsafe fn mb_cache_shrink_worker(work: *mut work_struct) {
    let cache = container_of!(work, mb_cache, c_shrink_work);
    mb_cache_shrink_impl(cache, (*cache).c_max_entries / SHRINK_DIVISOR);
}

pub unsafe extern "C" fn mb_cache_create(bucket_bits: i32) -> *mut mb_cache {
    let bucket_count = 1usize << bucket_bits;
    let cache = kzalloc_obj::<mb_cache>(); if cache.is_null() { return core::ptr::null_mut(); }
    (*cache).c_bucket_bits = bucket_bits; (*cache).c_max_entries = (bucket_count as c_ulong) << 4; INIT_LIST_HEAD(&mut (*cache).c_list); spin_lock_init(&mut (*cache).c_list_lock);
    (*cache).c_hash = kmalloc_objs::<hlist_bl_head>(bucket_count); if (*cache).c_hash.is_null() { kfree(cache as *mut _); return core::ptr::null_mut(); }
    for i in 0..bucket_count { INIT_HLIST_BL_HEAD(&mut *(*cache).c_hash.add(i)); }
    (*cache).c_shrink = shrinker_alloc(0, "mbcache-shrinker\0".as_ptr() as *const _); if (*cache).c_shrink.is_null() { kfree((*cache).c_hash as *mut _); kfree(cache as *mut _); return core::ptr::null_mut(); }
    (*(*cache).c_shrink).count_objects = Some(mb_cache_count); (*(*cache).c_shrink).scan_objects = Some(mb_cache_scan); (*(*cache).c_shrink).private_data = cache.cast(); shrinker_register((*cache).c_shrink); INIT_WORK(&mut (*cache).c_shrink_work, mb_cache_shrink_worker); cache
}

pub unsafe extern "C" fn mb_cache_destroy(cache: *mut mb_cache) {
    cancel_work_sync(&mut (*cache).c_shrink_work); shrinker_free((*cache).c_shrink);
    let mut entry = list_first_entry(&mut (*cache).c_list, e_list) as *mut mb_cache_entry;
    while !entry.is_null() { list_del(&mut (*entry).e_list); WARN_ON(atomic_read(&(*entry).e_refcnt) != 1); mb_cache_entry_put(cache, entry); entry = list_first_entry(&mut (*cache).c_list, e_list) as *mut mb_cache_entry; }
    kfree((*cache).c_hash as *mut _); kfree(cache as *mut _);
}

unsafe fn mbcache_init() -> i32 { mb_entry_cache = KMEM_CACHE("mb_cache_entry\0".as_ptr() as *const _, SLAB_RECLAIM_ACCOUNT); if mb_entry_cache.is_null() { -ENOMEM } else { 0 } }
unsafe fn mbcache_exit() { kmem_cache_destroy(mb_entry_cache); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
