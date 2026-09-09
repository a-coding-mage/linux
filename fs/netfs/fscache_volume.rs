// SPDX-License-Identifier: GPL-2.0-or-later
/* Volume-level cache cookie handling.
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #define FSCACHE_DEBUG_LEVEL COOKIE
// Dependencies supplied by the surrounding kernel/FSCACHE translation unit.

const FSCACHE_VOLUME_HASH_SHIFT: usize = 10;
static mut FSCACHE_VOLUME_HASH: [hlist_bl_head; 1 << FSCACHE_VOLUME_HASH_SHIFT] =
    [hlist_bl_head::new(); 1 << FSCACHE_VOLUME_HASH_SHIFT];
static mut FSCACHE_VOLUME_DEBUG_ID: atomic_t = atomic_t::new(0);
static mut FSCACHE_VOLUMES: list_head = list_head::new();

unsafe fn fscache_create_volume_work(work: *mut work_struct);

pub unsafe fn fscache_get_volume(
    volume: *mut fscache_volume,
    where_: fscache_volume_trace,
) -> *mut fscache_volume {
    let mut ref_: i32 = 0;
    __refcount_inc(&mut (*volume).ref_, &mut ref_);
    trace_fscache_volume((*volume).debug_id, ref_ + 1, where_);
    volume
}

pub unsafe fn fscache_try_get_volume(
    volume: *mut fscache_volume,
    where_: fscache_volume_trace,
) -> *mut fscache_volume {
    let mut ref_: i32 = 0;
    if !__refcount_inc_not_zero(&mut (*volume).ref_, &mut ref_) {
        return core::ptr::null_mut();
    }
    trace_fscache_volume((*volume).debug_id, ref_ + 1, where_);
    volume
}

pub unsafe fn fscache_see_volume(volume: *mut fscache_volume, where_: fscache_volume_trace) {
    let ref_ = refcount_read(&(*volume).ref_);
    trace_fscache_volume((*volume).debug_id, ref_, where_);
}

/* Pin the cache behind a volume so that we can access it. */
unsafe fn __fscache_begin_volume_access(
    volume: *mut fscache_volume,
    cookie: *mut fscache_cookie,
    why: fscache_access_trace,
) {
    let n_accesses = atomic_inc_return(&mut (*volume).n_accesses);
    smp_mb__after_atomic();
    trace_fscache_access_volume(
        (*volume).debug_id,
        if !cookie.is_null() { (*cookie).debug_id } else { 0 },
        refcount_read(&(*volume).ref_),
        n_accesses,
        why,
    );
}

pub unsafe fn fscache_begin_volume_access(
    volume: *mut fscache_volume,
    cookie: *mut fscache_cookie,
    why: fscache_access_trace,
) -> bool {
    if !fscache_cache_is_live((*volume).cache) {
        return false;
    }
    __fscache_begin_volume_access(volume, cookie, why);
    if !fscache_cache_is_live((*volume).cache) {
        fscache_end_volume_access(volume, cookie, fscache_access_unlive);
        return false;
    }
    true
}

pub unsafe fn fscache_end_volume_access(
    volume: *mut fscache_volume,
    cookie: *mut fscache_cookie,
    why: fscache_access_trace,
) {
    smp_mb__before_atomic();
    let n_accesses = atomic_dec_return(&mut (*volume).n_accesses);
    trace_fscache_access_volume(
        (*volume).debug_id,
        if !cookie.is_null() { (*cookie).debug_id } else { 0 },
        refcount_read(&(*volume).ref_),
        n_accesses,
        why,
    );
    if n_accesses == 0 {
        wake_up_var(&mut (*volume).n_accesses);
    }
}

unsafe fn fscache_volume_same(a: *const fscache_volume, b: *const fscache_volume) -> bool {
    if (*a).key_hash != (*b).key_hash || (*a).cache != (*b).cache || (*(*a).key) != (*(*b).key) {
        return false;
    }
    let klen = round_up((*(*a).key) as usize + 1, core::mem::size_of::<__le32>());
    memcmp((*a).key as *const _, (*b).key as *const _, klen) == 0
}

unsafe fn fscache_is_acquire_pending(volume: *mut fscache_volume) -> bool {
    test_bit(FSCACHE_VOLUME_ACQUIRE_PENDING, &(*volume).flags)
}

unsafe fn fscache_wait_on_volume_collision(
    candidate: *mut fscache_volume,
    collidee_debug_id: u32,
) {
    wait_on_bit_timeout(
        &mut (*candidate).flags,
        FSCACHE_VOLUME_ACQUIRE_PENDING,
        TASK_UNINTERRUPTIBLE,
        20 * HZ,
    );
    if fscache_is_acquire_pending(candidate) {
        pr_notice!("Potential volume collision new={:08x} old={:08x}", (*candidate).debug_id, collidee_debug_id);
        fscache_stat(&mut fscache_n_volumes_collision);
        wait_on_bit(&mut (*candidate).flags, FSCACHE_VOLUME_ACQUIRE_PENDING, TASK_UNINTERRUPTIBLE);
    }
}

unsafe fn fscache_hash_volume(candidate: *mut fscache_volume) -> bool {
    let bucket = ((*candidate).key_hash as usize) & ((1 << FSCACHE_VOLUME_HASH_SHIFT) - 1);
    let h = &mut FSCACHE_VOLUME_HASH[bucket];
    hlist_bl_lock(h);
    let mut cursor: *mut fscache_volume = core::ptr::null_mut();
    let mut p: *mut hlist_bl_node = core::ptr::null_mut();
    hlist_bl_for_each_entry!(cursor, p, h, hash_link, {
        if fscache_volume_same(candidate, cursor) {
            if !test_bit(FSCACHE_VOLUME_RELINQUISHED, &(*cursor).flags) {
                fscache_see_volume(cursor, fscache_volume_collision);
                hlist_bl_unlock(h);
                return false;
            }
            fscache_see_volume(cursor, fscache_volume_get_hash_collision);
            set_bit(FSCACHE_VOLUME_COLLIDED_WITH, &mut (*cursor).flags);
            set_bit(FSCACHE_VOLUME_ACQUIRE_PENDING, &mut (*candidate).flags);
            let collidee_debug_id = (*cursor).debug_id;
            hlist_bl_add_head(&mut (*candidate).hash_link, h);
            hlist_bl_unlock(h);
            fscache_wait_on_volume_collision(candidate, collidee_debug_id);
            return true;
        }
    });
    hlist_bl_add_head(&mut (*candidate).hash_link, h);
    hlist_bl_unlock(h);
    true
}

unsafe fn fscache_alloc_volume(
    volume_key: *const i8, cache_name: *const i8,
    coherency_data: *const core::ffi::c_void, coherency_len: usize,
) -> *mut fscache_volume {
    let klen = strlen(volume_key);
    if klen > NAME_MAX { return core::ptr::null_mut(); }
    let coherency_len = if coherency_data.is_null() { 0 } else { coherency_len };
    let cache = fscache_lookup_cache(cache_name, false);
    if IS_ERR(cache) { return core::ptr::null_mut(); }
    let volume = kzalloc_flex::<fscache_volume>(coherency_len);
    if volume.is_null() { fscache_put_cache(cache, fscache_cache_put_alloc_volume); fscache_stat(&mut fscache_n_volumes_nomem); return core::ptr::null_mut(); }
    (*volume).cache = cache;
    (*volume).coherency_len = coherency_len;
    if !coherency_data.is_null() { memcpy((*volume).coherency as *mut _, coherency_data, coherency_len); }
    INIT_LIST_HEAD(&mut (*volume).proc_link);
    INIT_WORK(&mut (*volume).work, fscache_create_volume_work);
    refcount_set(&mut (*volume).ref_, 1);
    spin_lock_init(&mut (*volume).lock);
    let hlen = round_up(1 + klen + 1, core::mem::size_of::<__le32>());
    let key = kzalloc(hlen, GFP_KERNEL);
    if key.is_null() { kfree(volume as *mut _); fscache_put_cache(cache, fscache_cache_put_alloc_volume); fscache_stat(&mut fscache_n_volumes_nomem); return core::ptr::null_mut(); }
    *key = klen as u8;
    memcpy(key.add(1) as *mut _, volume_key as *const _, klen);
    (*volume).key = key;
    (*volume).key_hash = fscache_hash(0, key as *const _, hlen);
    (*volume).debug_id = atomic_inc_return(&mut FSCACHE_VOLUME_DEBUG_ID) as u32;
    down_write(&mut fscache_addremove_sem);
    atomic_inc(&mut (*cache).n_volumes);
    list_add_tail(&mut (*volume).proc_link, &mut FSCACHE_VOLUMES);
    fscache_see_volume(volume, fscache_volume_new_acquire);
    fscache_stat(&mut fscache_n_volumes);
    up_write(&mut fscache_addremove_sem);
    volume
}

unsafe fn fscache_create_volume_work(work: *mut work_struct) {
    let volume = container_of!(work, fscache_volume, work);
    fscache_see_volume(volume, fscache_volume_see_create_work);
    let ops = (*(*volume).cache).ops;
    if !(*ops).acquire_volume.is_none() { ((*ops).acquire_volume.unwrap())(volume); }
    fscache_end_cache_access((*volume).cache, fscache_access_acquire_volume_end);
    clear_and_wake_up_bit(FSCACHE_VOLUME_CREATING, &mut (*volume).flags);
    fscache_put_volume(volume, fscache_volume_put_create_work);
}

pub unsafe fn fscache_create_volume(volume: *mut fscache_volume, wait: bool) {
    if test_and_set_bit(FSCACHE_VOLUME_CREATING, &mut (*volume).flags) { if wait { wait_on_bit(&mut (*volume).flags, FSCACHE_VOLUME_CREATING, TASK_UNINTERRUPTIBLE); } return; }
    if !(*volume).cache_priv.is_null() { clear_and_wake_up_bit(FSCACHE_VOLUME_CREATING, &mut (*volume).flags); return; }
    if !fscache_begin_cache_access((*volume).cache, fscache_access_acquire_volume) { clear_and_wake_up_bit(FSCACHE_VOLUME_CREATING, &mut (*volume).flags); return; }
    fscache_get_volume(volume, fscache_volume_get_create_work);
    if !schedule_work(&mut (*volume).work) { fscache_put_volume(volume, fscache_volume_put_create_work); }
    if wait { fscache_see_volume(volume, fscache_volume_wait_create_work); wait_on_bit(&mut (*volume).flags, FSCACHE_VOLUME_CREATING, TASK_UNINTERRUPTIBLE); }
}
pub unsafe fn __fscache_acquire_volume(
    volume_key: *const i8,
    cache_name: *const i8,
    coherency_data: *const core::ffi::c_void,
    coherency_len: usize,
) -> *mut fscache_volume {
    let volume = fscache_alloc_volume(volume_key, cache_name, coherency_data, coherency_len);
    if volume.is_null() { return ERR_PTR(-ENOMEM); }
    if !fscache_hash_volume(volume) { fscache_put_volume(volume, fscache_volume_put_hash_collision); return ERR_PTR(-EBUSY); }
    fscache_create_volume(volume, false);
    volume
}

pub unsafe fn fscache_put_volume(volume: *mut fscache_volume, where_: fscache_volume_trace) {
    if !volume.is_null() { let mut ref_: i32 = 0; let zero = __refcount_dec_and_test(&mut (*volume).ref_, &mut ref_); trace_fscache_volume((*volume).debug_id, ref_ - 1, where_); if zero { fscache_free_volume(volume); } }
}

pub unsafe fn __fscache_relinquish_volume(volume: *mut fscache_volume, coherency_data: *const core::ffi::c_void, invalidate: bool) {
    if WARN_ON(test_and_set_bit(FSCACHE_VOLUME_RELINQUISHED, &mut (*volume).flags)) { return; }
    if invalidate { set_bit(FSCACHE_VOLUME_INVALIDATE, &mut (*volume).flags); } else if !coherency_data.is_null() { memcpy((*volume).coherency as *mut _, coherency_data, (*volume).coherency_len); }
    fscache_put_volume(volume, fscache_volume_put_relinquish);
}

pub unsafe fn fscache_withdraw_volume(volume: *mut fscache_volume) {
    let n_accesses = atomic_dec_return(&mut (*volume).n_accesses);
    trace_fscache_access_volume((*volume).debug_id, 0, refcount_read(&(*volume).ref_), n_accesses, fscache_access_cache_unpin);
    wait_var_event(&mut (*volume).n_accesses, atomic_read(&(*volume).n_accesses) == 0);
}

unsafe fn fscache_wake_pending_volume(volume: *mut fscache_volume, h: *mut hlist_bl_head) {
    let mut cursor: *mut fscache_volume = core::ptr::null_mut();
    let mut p: *mut hlist_bl_node = core::ptr::null_mut();
    hlist_bl_for_each_entry!(cursor, p, h, hash_link, {
        if fscache_volume_same(cursor, volume) {
            fscache_see_volume(cursor, fscache_volume_see_hash_wake);
            clear_and_wake_up_bit(FSCACHE_VOLUME_ACQUIRE_PENDING, &mut (*cursor).flags);
            return;
        }
    });
}

unsafe fn fscache_unhash_volume(volume: *mut fscache_volume) {
    let bucket = ((*volume).key_hash as usize) & ((1 << FSCACHE_VOLUME_HASH_SHIFT) - 1);
    let h = &mut FSCACHE_VOLUME_HASH[bucket];
    hlist_bl_lock(h);
    hlist_bl_del(&mut (*volume).hash_link);
    if test_bit(FSCACHE_VOLUME_COLLIDED_WITH, &(*volume).flags) { fscache_wake_pending_volume(volume, h); }
    hlist_bl_unlock(h);
}

unsafe fn fscache_free_volume(volume: *mut fscache_volume) {
    let cache = (*volume).cache;
    if !(*volume).cache_priv.is_null() {
        __fscache_begin_volume_access(volume, core::ptr::null_mut(), fscache_access_relinquish_volume);
        if !(*volume).cache_priv.is_null() { ((*(*cache).ops).free_volume.unwrap())(volume); }
        fscache_end_volume_access(volume, core::ptr::null_mut(), fscache_access_relinquish_volume_end);
    }
    down_write(&mut fscache_addremove_sem);
    list_del_init(&mut (*volume).proc_link);
    atomic_dec(&mut (*cache).n_volumes);
    up_write(&mut fscache_addremove_sem);
    if !hlist_bl_unhashed(&(*volume).hash_link) { fscache_unhash_volume(volume); }
    trace_fscache_volume((*volume).debug_id, 0, fscache_volume_free);
    kfree((*volume).key as *mut _);
    kfree(volume as *mut _);
    fscache_stat_d(&mut fscache_n_volumes);
    fscache_put_cache(cache, fscache_cache_put_volume);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn fscache_volumes_seq_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == &mut FSCACHE_VOLUMES as *mut _ as *mut _ {
        seq_puts(m, "VOLUME   REF   nCOOK ACC FL CACHE           KEY\n======== ===== ===== === == =============== ================\n");
        return 0;
    }
    let volume = list_entry!(v, fscache_volume, proc_link);
    seq_printf(m, "%08x %5d %5d %3d %02lx %-15.15s %s\n", (*volume).debug_id, refcount_read(&(*volume).ref_), atomic_read(&(*volume).n_cookies), atomic_read(&(*volume).n_accesses), (*volume).flags, if (*(*volume).cache).name.is_null() { "-" } else { (*(*volume).cache).name }, (*volume).key.add(1));
    0
}

#[cfg(CONFIG_PROC_FS)]
pub static FSCACHE_VOLUMES_SEQ_OPS: seq_operations = seq_operations {
    start: Some(fscache_volumes_seq_start), next: Some(fscache_volumes_seq_next),
    stop: Some(fscache_volumes_seq_stop), show: Some(fscache_volumes_seq_show),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
