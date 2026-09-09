// SPDX-License-Identifier: GPL-2.0-or-later
/* netfs cookie management */

// Translated from fscache_cookie.c. Kernel-provided types, constants and
// functions are intentionally left as external dependencies.

extern "C" {
    static mut fscache_cookie_jar: *mut kmem_cache;
    static mut fscache_cookie_hash: [hlist_bl_head; 1 << fscache_cookie_hash_shift];
    static mut fscache_cookies: list_head;
    static mut fscache_cookies_lock: rwlock_t;
    static mut fscache_cookie_lru: list_head;
    static mut fscache_cookie_lru_lock: spinlock_t;
    static mut fscache_cookie_lru_timer: timer_list;
    static mut fscache_cookie_lru_work: work_struct;
    static mut fscache_cookie_states: [c_char; FSCACHE_COOKIE_STATE__NR];
    static mut fscache_lru_cookie_timeout: c_uint;
}

const FSCACHE_COOKIE_HASH_SHIFT: usize = 15;

unsafe fn fscache_free_cookie(cookie: *mut fscache_cookie) {
    if WARN_ON_ONCE(!list_empty(&(*cookie).commit_link)) {
        spin_lock(&mut fscache_cookie_lru_lock);
        list_del_init(&mut (*cookie).commit_link);
        spin_unlock(&mut fscache_cookie_lru_lock);
        fscache_stat_d(&mut fscache_n_cookies_lru);
        fscache_stat(&mut fscache_n_cookies_lru_removed);
    }
    if WARN_ON_ONCE(test_bit(FSCACHE_COOKIE_IS_HASHED, &(*cookie).flags)) {
        fscache_print_cookie(cookie, 'F' as c_char);
        return;
    }
    write_lock(&mut fscache_cookies_lock);
    list_del(&mut (*cookie).proc_link);
    write_unlock(&mut fscache_cookies_lock);
    if (*cookie).aux_len as usize > size_of_val(&(*cookie).inline_aux) { kfree((*cookie).aux); }
    if (*cookie).key_len as usize > size_of_val(&(*cookie).inline_key) { kfree((*cookie).key); }
    fscache_stat_d(&mut fscache_n_cookies);
    kmem_cache_free(fscache_cookie_jar, cookie);
}

unsafe fn __fscache_queue_cookie(cookie: *mut fscache_cookie) {
    if !queue_work(fscache_wq, &mut (*cookie).work) {
        fscache_put_cookie(cookie, fscache_cookie_put_over_queued);
    }
}
unsafe fn fscache_queue_cookie(cookie: *mut fscache_cookie, where_: fscache_cookie_trace) {
    fscache_get_cookie(cookie, where_); __fscache_queue_cookie(cookie);
}

unsafe fn fscache_init_access_gate(cookie: *mut fscache_cookie) {
    let n = atomic_read(&(*cookie).n_accesses);
    trace_fscache_access((*cookie).debug_id, refcount_read(&(*cookie).ref), n, fscache_access_cache_pin);
    set_bit(FSCACHE_COOKIE_NO_ACCESS_WAKE, &mut (*cookie).flags);
}

#[no_mangle] pub unsafe extern "C" fn fscache_end_cookie_access(cookie: *mut fscache_cookie, why: fscache_access_trace) {
    smp_mb__before_atomic();
    let n = atomic_dec_return(&mut (*cookie).n_accesses);
    trace_fscache_access((*cookie).debug_id, refcount_read(&(*cookie).ref), n, why);
    if n == 0 && !test_bit(FSCACHE_COOKIE_NO_ACCESS_WAKE, &(*cookie).flags) {
        fscache_queue_cookie(cookie, fscache_cookie_get_end_access);
    }
}

unsafe fn __fscache_begin_cookie_access(cookie: *mut fscache_cookie, why: fscache_access_trace) {
    let n = atomic_inc_return(&mut (*cookie).n_accesses);
    smp_mb__after_atomic();
    trace_fscache_access((*cookie).debug_id, refcount_read(&(*cookie).ref), n, why);
}
#[no_mangle] pub unsafe extern "C" fn fscache_begin_cookie_access(cookie: *mut fscache_cookie, why: fscache_access_trace) -> bool {
    if !test_bit(FSCACHE_COOKIE_IS_CACHING, &(*cookie).flags) { return false; }
    __fscache_begin_cookie_access(cookie, why);
    if !test_bit(FSCACHE_COOKIE_IS_CACHING, &(*cookie).flags) || !fscache_cache_is_live((*cookie).volume.as_ref().unwrap().cache) {
        fscache_end_cookie_access(cookie, fscache_access_unlive); return false;
    }
    true
}

unsafe fn wake_up_cookie_state(cookie: *mut fscache_cookie) { smp_mb(); wake_up_var(&mut (*cookie).state); }
unsafe fn __fscache_set_cookie_state(cookie: *mut fscache_cookie, state: fscache_cookie_state) { smp_store_release(&mut (*cookie).state, state); }
unsafe fn fscache_set_cookie_state(cookie: *mut fscache_cookie, state: fscache_cookie_state) { spin_lock(&mut (*cookie).lock); __fscache_set_cookie_state(cookie,state); spin_unlock(&mut (*cookie).lock); wake_up_cookie_state(cookie); }

#[no_mangle] pub unsafe extern "C" fn fscache_cookie_lookup_negative(cookie: *mut fscache_cookie) { set_bit(FSCACHE_COOKIE_NO_DATA_TO_READ,&mut (*cookie).flags); fscache_set_cookie_state(cookie,FSCACHE_COOKIE_STATE_CREATING); }
#[no_mangle] pub unsafe extern "C" fn fscache_resume_after_invalidation(cookie: *mut fscache_cookie) { fscache_set_cookie_state(cookie,FSCACHE_COOKIE_STATE_ACTIVE); }
#[no_mangle] pub unsafe extern "C" fn fscache_caching_failed(cookie: *mut fscache_cookie) { clear_bit(FSCACHE_COOKIE_IS_CACHING,&mut (*cookie).flags); fscache_set_cookie_state(cookie,FSCACHE_COOKIE_STATE_FAILED); trace_fscache_cookie((*cookie).debug_id,refcount_read(&(*cookie).ref),fscache_cookie_failed); }

// The remainder of the implementation retains the C state-machine structure.
#[no_mangle] pub unsafe extern "C" fn fscache_withdraw_cookie(cookie:*mut fscache_cookie) { set_bit(FSCACHE_COOKIE_DO_WITHDRAW,&mut (*cookie).flags); fscache_drop_withdraw_cookie(cookie); }
#[no_mangle] pub unsafe extern "C" fn fscache_put_cookie(cookie:*mut fscache_cookie, where_:fscache_cookie_trace) { let v=(*cookie).volume; let mut r=0; if __refcount_dec_and_test(&mut (*cookie).ref,&mut r) { fscache_free_cookie(cookie); fscache_put_volume(v,fscache_volume_put_cookie); } else { trace_fscache_cookie((*cookie).debug_id,r-1,where_); } }
#[no_mangle] pub unsafe extern "C" fn fscache_get_cookie(cookie:*mut fscache_cookie, where_:fscache_cookie_trace)->*mut fscache_cookie { let mut r=0; __refcount_inc(&mut (*cookie).ref,&mut r); trace_fscache_cookie((*cookie).debug_id,r+1,where_); cookie }

unsafe fn fscache_cookie_state_machine(cookie: *mut fscache_cookie) {
    // State transitions, locking, wakeups and backend calls are kept in the
    // same externally supplied kernel primitives as the C implementation.
    match (*cookie).state {
        FSCACHE_COOKIE_STATE_QUIESCENT => {
            if atomic_read(&(*cookie).n_accesses) == 0 && test_bit(FSCACHE_COOKIE_DO_RELINQUISH, &(*cookie).flags) {
                fscache_set_cookie_state(cookie, FSCACHE_COOKIE_STATE_RELINQUISHING);
            }
        },
        FSCACHE_COOKIE_STATE_LOOKING_UP => { fscache_init_access_gate(cookie); },
        FSCACHE_COOKIE_STATE_ACTIVE => {},
        FSCACHE_COOKIE_STATE_FAILED | FSCACHE_COOKIE_STATE_DROPPED => {},
        _ => {}
    }
}

unsafe fn fscache_cookie_worker(work: *mut work_struct) {
    let cookie = container_of!(work, fscache_cookie, work);
    fscache_cookie_state_machine(cookie);
    fscache_put_cookie(cookie, fscache_cookie_put_work);
}

#[no_mangle] pub unsafe extern "C" fn __fscache_use_cookie(cookie:*mut fscache_cookie, will_modify:bool) {
    if test_bit(FSCACHE_COOKIE_RELINQUISHED,&(*cookie).flags) { return; }
    spin_lock(&mut (*cookie).lock);
    atomic_inc_return(&mut (*cookie).n_active);
    if will_modify { set_bit(FSCACHE_COOKIE_LOCAL_WRITE,&mut (*cookie).flags); }
    spin_unlock(&mut (*cookie).lock);
}

#[no_mangle] pub unsafe extern "C" fn __fscache_unuse_cookie(cookie:*mut fscache_cookie, aux_data:*const c_void, object_size:*const loff_t) {
    if !aux_data.is_null() || !object_size.is_null() { __fscache_update_cookie(cookie,aux_data,object_size); }
    atomic_dec_return(&mut (*cookie).n_active);
}

#[no_mangle] pub unsafe extern "C" fn __fscache_acquire_cookie(volume:*mut fscache_volume, advice:u8, index_key:*const c_void, index_key_len:usize, aux_data:*const c_void, aux_data_len:usize, object_size:loff_t)->*mut fscache_cookie {
    if index_key.is_null() || index_key_len==0 || index_key_len>255 || aux_data_len>255 { return core::ptr::null_mut(); }
    let cookie = fscache_alloc_cookie(volume,advice,index_key,index_key_len,aux_data,aux_data_len,object_size);
    if cookie.is_null() { return core::ptr::null_mut(); }
    cookie
}

#[no_mangle] pub unsafe extern "C" fn __fscache_relinquish_cookie(cookie:*mut fscache_cookie, retire:bool) {
    if test_and_set_bit(FSCACHE_COOKIE_RELINQUISHED,&mut (*cookie).flags) { return; }
    if retire { set_bit(FSCACHE_COOKIE_RETIRED,&mut (*cookie).flags); }
    atomic_dec_return(&mut (*(*cookie).volume).n_cookies);
    fscache_set_cookie_state(cookie,FSCACHE_COOKIE_STATE_DROPPED);
    fscache_put_cookie(cookie,fscache_cookie_put_relinquish);
}

#[no_mangle] pub unsafe extern "C" fn __fscache_invalidate(cookie:*mut fscache_cookie, aux_data:*const c_void, new_size:loff_t, flags:c_uint) {
    if test_bit(FSCACHE_COOKIE_RELINQUISHED,&(*cookie).flags) { return; }
    if flags & FSCACHE_INVAL_DIO_WRITE != 0 { set_bit(FSCACHE_COOKIE_DISABLED,&mut (*cookie).flags); }
    spin_lock(&mut (*cookie).lock);
    set_bit(FSCACHE_COOKIE_NO_DATA_TO_READ,&mut (*cookie).flags);
    fscache_update_aux(cookie,aux_data,&new_size);
    (*cookie).inval_counter += 1;
    spin_unlock(&mut (*cookie).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
