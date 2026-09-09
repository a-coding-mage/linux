/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Internal definitions for network filesystem support. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Linux header dependencies are supplied by the surrounding translation unit. */

extern "C" {
    pub fn netfs_queue_read(rreq: *mut netfs_io_request, subreq: *mut netfs_io_subrequest);
    pub fn netfs_cache_read_terminated(priv_: *mut core::ffi::c_void, transferred_or_error: isize);
    pub fn netfs_prefetch_for_write(file: *mut file, folio: *mut folio, offset: usize, len: usize) -> i32;

    pub fn netfs_update_i_size(ctx: *mut netfs_inode, inode: *mut inode, pos: i64, copied: usize);

    pub static mut netfs_debug: u32;
    pub static mut netfs_io_requests: list_head;
    pub static mut netfs_proc_lock: spinlock_t;
    pub static mut netfs_request_pool: mempool_t;
    pub static mut netfs_subrequest_pool: mempool_t;
    pub static mut netfs_folioq_pool: mempool_t;

    pub fn netfs_buffer_make_space(rreq: *mut netfs_io_request, trace: netfs_folioq_trace) -> *mut folio_queue;
    pub fn netfs_reset_iter(subreq: *mut netfs_io_subrequest);
    pub fn netfs_wake_collector(rreq: *mut netfs_io_request);
    pub fn netfs_subreq_clear_in_progress(subreq: *mut netfs_io_subrequest);
    pub fn netfs_wait_for_in_progress_stream(rreq: *mut netfs_io_request, stream: *mut netfs_io_stream);
    pub fn netfs_wait_for_read(rreq: *mut netfs_io_request) -> isize;
    pub fn netfs_wait_for_write(rreq: *mut netfs_io_request) -> isize;
    pub fn netfs_wait_for_paused_read(rreq: *mut netfs_io_request);
    pub fn netfs_wait_for_paused_write(rreq: *mut netfs_io_request);

    pub fn netfs_alloc_request(mapping: *mut address_space, file: *mut file, start: i64, len: usize, origin: netfs_io_origin) -> *mut netfs_io_request;
    pub fn netfs_get_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace);
    pub fn netfs_clear_subrequests(rreq: *mut netfs_io_request);
    pub fn netfs_put_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace);
    pub fn netfs_put_failed_request(rreq: *mut netfs_io_request);
    pub fn netfs_alloc_subrequest(rreq: *mut netfs_io_request) -> *mut netfs_io_subrequest;

    pub fn netfs_read_collection(rreq: *mut netfs_io_request) -> bool;
    pub fn netfs_read_collection_worker(work: *mut work_struct);
    pub fn netfs_cancel_read(subreq: *mut netfs_io_subrequest, error: i32);
    pub fn netfs_pgpriv2_copy_to_cache(rreq: *mut netfs_io_request, folio: *mut folio);
    pub fn netfs_pgpriv2_end_copy_to_cache(rreq: *mut netfs_io_request);
    pub fn netfs_pgpriv2_unlock_copied_folios(wreq: *mut netfs_io_request) -> bool;
    pub fn netfs_retry_reads(rreq: *mut netfs_io_request);
    pub fn netfs_unlock_abandoned_read_pages(rreq: *mut netfs_io_request);

    pub fn netfs_folio_written_back(folio: *mut folio) -> i32;
    pub fn netfs_write_collection(wreq: *mut netfs_io_request) -> bool;
    pub fn netfs_write_collection_worker(work: *mut work_struct);
    pub fn netfs_create_write_req(mapping: *mut address_space, file: *mut file, start: i64, origin: netfs_io_origin) -> *mut netfs_io_request;
    pub fn netfs_prepare_write(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream, start: i64);
    pub fn netfs_reissue_write(stream: *mut netfs_io_stream, subreq: *mut netfs_io_subrequest, source: *mut iov_iter);
    pub fn netfs_issue_write(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream);
    pub fn netfs_advance_write(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream, start: i64, len: usize, to_eof: bool) -> usize;
    pub fn netfs_begin_writethrough(iocb: *mut kiocb, len: usize) -> *mut netfs_io_request;
    pub fn netfs_advance_writethrough(wreq: *mut netfs_io_request, wbc: *mut writeback_control, folio: *mut folio, copied: usize, to_page_end: bool, writethrough_cache: *mut *mut folio) -> i32;
    pub fn netfs_end_writethrough(wreq: *mut netfs_io_request, wbc: *mut writeback_control, writethrough_cache: *mut folio) -> isize;
    pub fn netfs_retry_writes(wreq: *mut netfs_io_request);

    pub fn fscache_begin_cache_access(cache: *mut fscache_cache, why: fscache_access_trace) -> bool;
    pub fn fscache_end_cache_access(cache: *mut fscache_cache, why: fscache_access_trace);
    pub fn fscache_lookup_cache(name: *const i8, is_cache: bool) -> *mut fscache_cache;
    pub fn fscache_put_cache(cache: *mut fscache_cache, where_: fscache_cache_trace);
    pub static mut fscache_cookie_jar: *mut kmem_cache;
    pub static mut fscache_cookie_lru_timer: timer_list;
    pub fn fscache_print_cookie(cookie: *mut fscache_cookie, prefix: i8);
    pub fn fscache_begin_cookie_access(cookie: *mut fscache_cookie, why: fscache_access_trace) -> bool;
    pub fn fscache_hash(salt: u32, data: *const core::ffi::c_void, len: usize) -> u32;
    pub fn fscache_init() -> i32;
    pub fn fscache_exit();
    pub fn fscache_proc_init() -> i32;
    pub fn fscache_proc_cleanup();
    pub fn fscache_get_volume(volume: *mut fscache_volume, where_: fscache_volume_trace) -> *mut fscache_volume;
    pub fn fscache_begin_volume_access(volume: *mut fscache_volume, cookie: *mut fscache_cookie, why: fscache_access_trace) -> bool;
    pub fn fscache_create_volume(volume: *mut fscache_volume, wait: bool);
}

/* Types are provided by the translated Linux dependencies. */
extern "C" {
    pub fn netfs_stat(stat: *mut atomic_t);
    pub fn netfs_stat_d(stat: *mut atomic_t);
}

#[inline]
pub unsafe fn netfs_see_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace) {
    trace_netfs_rreq_ref((*rreq).debug_id, refcount_read(&(*rreq).ref_), what);
}

#[inline]
pub unsafe fn netfs_see_subrequest(subreq: *mut netfs_io_subrequest, what: netfs_sreq_ref_trace) {
    trace_netfs_sreq_ref((*(*subreq).rreq).debug_id, (*subreq).debug_index, refcount_read(&(*subreq).ref_), what);
}

#[inline]
pub unsafe fn netfs_get_group(group: *mut netfs_group) -> *mut netfs_group {
    if !group.is_null() && group != NETFS_FOLIO_COPY_TO_CACHE { refcount_inc(&mut (*group).ref_); }
    group
}

#[inline]
pub unsafe fn netfs_put_group(group: *mut netfs_group) {
    if !group.is_null() && group != NETFS_FOLIO_COPY_TO_CACHE && refcount_dec_and_test(&mut (*group).ref_) { ((*group).free)(group); }
}

#[inline]
pub unsafe fn netfs_put_group_many(group: *mut netfs_group, nr: i32) {
    if !group.is_null() && group != NETFS_FOLIO_COPY_TO_CACHE && refcount_sub_and_test(nr, &mut (*group).ref_) { ((*group).free)(group); }
}

#[inline]
pub unsafe fn netfs_is_cache_enabled(ctx: *mut netfs_inode) -> bool {
    #[cfg(CONFIG_FSCACHE)]
    { let cookie = (*ctx).cache; return fscache_cookie_valid(cookie) && !(*cookie).cache_priv.is_null() && fscache_cookie_enabled(cookie); }
    #[cfg(not(CONFIG_FSCACHE))]
    { let _ = ctx; false }
}

#[inline]
pub unsafe fn netfs_is_cache_maybe_enabled(ctx: *mut netfs_inode) -> bool {
    #[cfg(CONFIG_FSCACHE)]
    { let cookie = (*ctx).cache; return fscache_cookie_valid(cookie) && test_bit(FSCACHE_COOKIE_IS_CACHING, &(*cookie).flags); }
    #[cfg(not(CONFIG_FSCACHE))]
    { let _ = ctx; false }
}

#[inline]
pub unsafe fn netfs_wake_rreq_flag(rreq: *mut netfs_io_request, flag: u32, trace: netfs_rreq_trace) {
    if test_bit(flag, &(*rreq).flags) {
        clear_bit_unlock(flag, &mut (*rreq).flags);
        smp_mb__after_atomic();
        trace_netfs_rreq(rreq, trace);
        wake_up(&mut (*rreq).waitq);
    }
}

#[inline]
pub unsafe fn netfs_check_rreq_in_progress(rreq: *const netfs_io_request) -> bool {
    test_bit_acquire(NETFS_RREQ_IN_PROGRESS, &(*rreq).flags)
}

#[inline]
pub unsafe fn netfs_check_subreq_in_progress(subreq: *const netfs_io_subrequest) -> bool {
    test_bit_acquire(NETFS_SREQ_IN_PROGRESS, &(*subreq).flags)
}

#[inline]
pub unsafe fn fscache_cache_state(cache: *const fscache_cache) -> fscache_cache_state {
    smp_load_acquire(&(*cache).state)
}

#[inline]
pub unsafe fn fscache_cache_is_live(cache: *const fscache_cache) -> bool {
    fscache_cache_state(cache) == FSCACHE_CACHE_IS_ACTIVE
}

#[inline]
pub unsafe fn fscache_set_cache_state(cache: *mut fscache_cache, state: fscache_cache_state) {
    smp_store_release(&mut (*cache).state, state);
}

#[inline]
pub unsafe fn fscache_set_cache_state_maybe(cache: *mut fscache_cache, old: fscache_cache_state, new_: fscache_cache_state) -> bool {
    try_cmpxchg_release(&mut (*cache).state, &old, new_)
}

#[inline]
pub unsafe fn fscache_see_cookie(cookie: *mut fscache_cookie, where_: fscache_cookie_trace) {
    trace_fscache_cookie((*cookie).debug_id, refcount_read(&(*cookie).ref_), where_);
}

#[cfg(not(CONFIG_FSCACHE))]
#[inline] pub unsafe fn fscache_init() -> i32 { 0 }
#[cfg(not(CONFIG_FSCACHE))]
#[inline] pub unsafe fn fscache_exit() {}
#[cfg(not(CONFIG_PROC_FS))]
#[inline] pub unsafe fn fscache_proc_init() -> i32 { 0 }
#[cfg(not(CONFIG_PROC_FS))]
#[inline] pub unsafe fn fscache_proc_cleanup() {}

/* The remaining C inline helpers and debug/assertion macros retain their semantics
 * through the corresponding kernel primitives supplied by the surrounding crate. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
