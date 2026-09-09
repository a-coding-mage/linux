/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct blk_stat_callback {
    /* RCU list of callbacks for a request_queue. */
    pub list: list_head,
    /* Timer for the next callback invocation. */
    pub timer: timer_list,
    /* Per-cpu statistics buckets. */
    pub cpu_stat: *mut blk_rq_stat,
    /* Given a request, returns its statistics bucket, or -1. */
    pub bucket_fn: Option<unsafe extern "C" fn(*const request) -> core::ffi::c_int>,
    /* Number of statistics buckets. */
    pub buckets: core::ffi::c_uint,
    /* Array of statistics buckets. */
    pub stat: *mut blk_rq_stat,
    /* Callback function. */
    pub timer_fn: Option<unsafe extern "C" fn(*mut blk_stat_callback)>,
    /* Private pointer for the user. */
    pub data: *mut core::ffi::c_void,
    /* RCU list head. */
    pub rcu: rcu_head,
}

extern "C" {
    pub fn blk_alloc_queue_stats() -> *mut blk_queue_stats;
    pub fn blk_free_queue_stats(stats: *mut blk_queue_stats);

    pub fn blk_stat_add(rq: *mut request, now: u64);

    /* Record time/size info in request but do not add a callback. */
    pub fn blk_stat_enable_accounting(q: *mut request_queue);
    pub fn blk_stat_disable_accounting(q: *mut request_queue);

    pub fn blk_stat_alloc_callback(
        timer_fn: Option<unsafe extern "C" fn(*mut blk_stat_callback)>,
        bucket_fn: Option<unsafe extern "C" fn(*const request) -> core::ffi::c_int>,
        buckets: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
    ) -> *mut blk_stat_callback;

    pub fn blk_stat_add_callback(q: *mut request_queue, cb: *mut blk_stat_callback);
    pub fn blk_stat_remove_callback(q: *mut request_queue, cb: *mut blk_stat_callback);
    pub fn blk_stat_free_callback(cb: *mut blk_stat_callback);

    pub fn timer_pending(timer: *mut timer_list) -> bool;
    pub fn mod_timer(timer: *mut timer_list, expires: usize) -> core::ffi::c_int;
    pub fn nsecs_to_jiffies(nsecs: u64) -> usize;
    pub fn timer_delete_sync(timer: *mut timer_list) -> core::ffi::c_int;
    pub fn msecs_to_jiffies(msecs: core::ffi::c_uint) -> usize;

    pub fn blk_rq_stat_add(stat: *mut blk_rq_stat, value: u64);
    pub fn blk_rq_stat_sum(dst: *mut blk_rq_stat, src: *mut blk_rq_stat);
    pub fn blk_rq_stat_init(stat: *mut blk_rq_stat);
}

#[inline]
pub unsafe fn blk_stat_is_active(cb: *mut blk_stat_callback) -> bool {
    timer_pending(&mut (*cb).timer)
}

#[inline]
pub unsafe fn blk_stat_activate_nsecs(cb: *mut blk_stat_callback, nsecs: u64) {
    mod_timer(
        &mut (*cb).timer,
        jiffies.wrapping_add(nsecs_to_jiffies(nsecs)),
    );
}

#[inline]
pub unsafe fn blk_stat_deactivate(cb: *mut blk_stat_callback) {
    timer_delete_sync(&mut (*cb).timer);
}

#[inline]
pub unsafe fn blk_stat_activate_msecs(
    cb: *mut blk_stat_callback,
    msecs: core::ffi::c_uint,
) {
    mod_timer(
        &mut (*cb).timer,
        jiffies.wrapping_add(msecs_to_jiffies(msecs)),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
