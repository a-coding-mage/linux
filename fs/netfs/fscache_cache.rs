// SPDX-License-Identifier: GPL-2.0-or-later
/* FS-Cache cache handling */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/export.h, linux/slab.h, and internal.h.

static mut FSCACHE_CACHES: ListHead = ListHead::new();
static mut FSCACHE_ADDREMOVE_SEM: RwSemaphore = RwSemaphore::new();
static mut FSCACHE_CLEARANCE_WAITERS: WaitQueueHead = WaitQueueHead::new();
static mut FSCACHE_CACHE_DEBUG_ID: Atomic = Atomic::new(0);

unsafe fn fscache_alloc_cache(name: *const c_char) -> *mut fscache_cache {
    let cache = kzalloc_obj::<fscache_cache>();
    if !cache.is_null() {
        if !name.is_null() {
            (*cache).name = kstrdup(name, GFP_KERNEL);
            if (*cache).name.is_null() {
                kfree(cache);
                return core::ptr::null_mut();
            }
        }
        refcount_set(&mut (*cache).ref_, 1);
        INIT_LIST_HEAD(&mut (*cache).cache_link);
        (*cache).debug_id = atomic_inc_return(&mut FSCACHE_CACHE_DEBUG_ID);
    }
    cache
}

unsafe fn fscache_get_cache_maybe(
    cache: *mut fscache_cache,
    where_: fscache_cache_trace,
) -> bool {
    let mut ref_ = 0;
    let success = __refcount_inc_not_zero(&mut (*cache).ref_, &mut ref_);
    if success {
        trace_fscache_cache((*cache).debug_id, ref_ + 1, where_);
    }
    success
}

pub unsafe fn fscache_lookup_cache(name: *const c_char, is_cache: bool) -> *mut fscache_cache {
    let mut candidate: *mut fscache_cache;
    let mut cache: *mut fscache_cache;
    let mut unnamed: *mut fscache_cache = core::ptr::null_mut();

    down_read(&FSCACHE_ADDREMOVE_SEM);
    list_for_each_entry!(cache, FSCACHE_CACHES, cache_link, {
        if !(*cache).name.is_null() && !name.is_null()
            && strcmp((*cache).name, name) == 0
            && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
        { goto!(got_cache_r); }
        if (*cache).name.is_null() && name.is_null()
            && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
        { goto!(got_cache_r); }
    });
    if name.is_null() {
        list_for_each_entry!(cache, FSCACHE_CACHES, cache_link, {
            if !(*cache).name.is_null()
                && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
            { goto!(got_cache_r); }
        });
    }
    up_read(&FSCACHE_ADDREMOVE_SEM);
    candidate = fscache_alloc_cache(name);
    if candidate.is_null() { return ERR_PTR(-ENOMEM); }
    down_write(&FSCACHE_ADDREMOVE_SEM);
    list_for_each_entry!(cache, FSCACHE_CACHES, cache_link, {
        if !(*cache).name.is_null() && !name.is_null()
            && strcmp((*cache).name, name) == 0
            && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
        { goto!(got_cache_w); }
        if (*cache).name.is_null() {
            unnamed = cache;
            if name.is_null()
                && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
            { goto!(got_cache_w); }
        }
    });
    if !unnamed.is_null() && is_cache
        && fscache_get_cache_maybe(unnamed, fscache_cache_trace::GetAcquire)
    { cache = unnamed; (*cache).name = (*candidate).name; (*candidate).name = core::ptr::null_mut(); goto!(got_cache_w); }
    if name.is_null() {
        list_for_each_entry!(cache, FSCACHE_CACHES, cache_link, {
            if !(*cache).name.is_null()
                && fscache_get_cache_maybe(cache, fscache_cache_trace::GetAcquire)
            { goto!(got_cache_w); }
        });
    }
    list_add_tail(&mut (*candidate).cache_link, &mut FSCACHE_CACHES);
    trace_fscache_cache((*candidate).debug_id, refcount_read(&(*candidate).ref_), fscache_cache_trace::NewAcquire);
    up_write(&FSCACHE_ADDREMOVE_SEM);
    return candidate;
got_cache_r:
    up_read(&FSCACHE_ADDREMOVE_SEM);
    return cache;
got_cache_w:
    up_write(&FSCACHE_ADDREMOVE_SEM);
    kfree((*candidate).name);
    kfree(candidate);
    cache
}

pub unsafe fn fscache_acquire_cache(name: *const c_char) -> *mut fscache_cache {
    ASSERT(!name.is_null());
    let cache = fscache_lookup_cache(name, true);
    if IS_ERR(cache) { return cache; }
    if !fscache_set_cache_state_maybe(cache, FSCACHE_CACHE_STATE::IsNotPresent, FSCACHE_CACHE_STATE::IsPreparing) {
        pr_warn!("Cache tag %s in use\n", name);
        fscache_put_cache(cache, fscache_cache_trace::PutCache);
        return ERR_PTR(-EBUSY);
    }
    cache
}

pub unsafe fn fscache_put_cache(cache: *mut fscache_cache, where_: fscache_cache_trace) {
    if IS_ERR_OR_NULL(cache) { return; }
    let debug_id = (*cache).debug_id;
    let mut ref_ = 0;
    let zero = __refcount_dec_and_test(&mut (*cache).ref_, &mut ref_);
    trace_fscache_cache(debug_id, ref_ - 1, where_);
    if zero {
        down_write(&FSCACHE_ADDREMOVE_SEM);
        list_del_init(&mut (*cache).cache_link);
        up_write(&FSCACHE_ADDREMOVE_SEM);
        kfree((*cache).name);
        kfree(cache);
    }
}

pub unsafe fn fscache_relinquish_cache(cache: *mut fscache_cache) {
    let where_ = if (*cache).state == FSCACHE_CACHE_STATE::IsPreparing { fscache_cache_trace::PutPrepFailed } else { fscache_cache_trace::PutRelinquish };
    (*cache).ops = core::ptr::null();
    (*cache).cache_priv = core::ptr::null_mut();
    fscache_set_cache_state(cache, FSCACHE_CACHE_STATE::IsNotPresent);
    fscache_put_cache(cache, where_);
}

pub unsafe fn fscache_add_cache(cache: *mut fscache_cache, ops: *const fscache_cache_ops, cache_priv: *mut c_void) -> c_int {
    BUG_ON(fscache_cache_state(cache) != FSCACHE_CACHE_STATE::IsPreparing);
    let n_accesses = atomic_inc_return(&mut (*cache).n_accesses);
    trace_fscache_access_cache((*cache).debug_id, refcount_read(&(*cache).ref_), n_accesses, fscache_access_trace::Pin);
    down_write(&FSCACHE_ADDREMOVE_SEM);
    (*cache).ops = ops;
    (*cache).cache_priv = cache_priv;
    fscache_set_cache_state(cache, FSCACHE_CACHE_STATE::IsActive);
    up_write(&FSCACHE_ADDREMOVE_SEM);
    pr_notice!("Cache \"%s\" added (type %s)\n", (*cache).name, (*ops).name);
    0
}

pub unsafe fn fscache_begin_cache_access(cache: *mut fscache_cache, why: fscache_access_trace) -> bool {
    if !fscache_cache_is_live(cache) { return false; }
    let n_accesses = atomic_inc_return(&mut (*cache).n_accesses);
    smp_mb__after_atomic();
    trace_fscache_access_cache((*cache).debug_id, refcount_read(&(*cache).ref_), n_accesses, why);
    if !fscache_cache_is_live(cache) { fscache_end_cache_access(cache, fscache_access_trace::Unlive); return false; }
    true
}

pub unsafe fn fscache_end_cache_access(cache: *mut fscache_cache, why: fscache_access_trace) {
    smp_mb__before_atomic();
    let n_accesses = atomic_dec_return(&mut (*cache).n_accesses);
    trace_fscache_access_cache((*cache).debug_id, refcount_read(&(*cache).ref_), n_accesses, why);
    if n_accesses == 0 { wake_up_var(&mut (*cache).n_accesses); }
}

pub unsafe fn fscache_io_error(cache: *mut fscache_cache) {
    if fscache_set_cache_state_maybe(cache, FSCACHE_CACHE_STATE::IsActive, FSCACHE_CACHE_STATE::GotIoError) { pr_err!("Cache '%s' stopped due to I/O error\n", (*cache).name); }
}

pub unsafe fn fscache_withdraw_cache(cache: *mut fscache_cache) {
    pr_notice!("Withdrawing cache \"%s\" (%u objs)\n", (*cache).name, atomic_read(&(*cache).object_count));
    fscache_set_cache_state(cache, FSCACHE_CACHE_STATE::IsWithdrawn);
    let n_accesses = atomic_dec_return(&mut (*cache).n_accesses);
    trace_fscache_access_cache((*cache).debug_id, refcount_read(&(*cache).ref_), n_accesses, fscache_access_trace::Unpin);
    wait_var_event!(&mut (*cache).n_accesses, atomic_read(&(*cache).n_accesses) == 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
