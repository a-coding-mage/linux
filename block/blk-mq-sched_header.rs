/* SPDX-License-Identifier: GPL-2.0 */
// Translated from blk-mq-sched.h.  Declarations supplied by elevator.h and
// blk-mq.h are intentionally referenced but not defined here.

pub const MAX_SCHED_RQ: usize = 16 * BLKDEV_DEFAULT_RQ;

extern "C" {
    pub fn blk_mq_sched_try_merge(
        q: *mut request_queue,
        bio: *mut bio,
        nr_segs: c_uint,
        merged_request: *mut *mut request,
    ) -> bool;
    pub fn blk_mq_sched_bio_merge(
        q: *mut request_queue,
        bio: *mut bio,
        nr_segs: c_uint,
    ) -> bool;
    pub fn blk_mq_sched_try_insert_merge(
        q: *mut request_queue,
        rq: *mut request,
        free: *mut list_head,
    ) -> bool;
    pub fn blk_mq_sched_mark_restart_hctx(hctx: *mut blk_mq_hw_ctx);
    pub fn __blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_init_sched(
        q: *mut request_queue,
        e: *mut elevator_type,
        res: *mut elevator_resources,
    ) -> c_int;
    pub fn blk_mq_exit_sched(q: *mut request_queue, e: *mut elevator_queue);
    pub fn blk_mq_sched_free_rqs(q: *mut request_queue);
    pub fn blk_mq_alloc_sched_tags(
        set: *mut blk_mq_tag_set,
        nr_hw_queues: c_uint,
        nr_requests: c_uint,
    ) -> *mut elevator_tags;
    pub fn blk_mq_alloc_sched_res(
        q: *mut request_queue,
        type_: *mut elevator_type,
        res: *mut elevator_resources,
        nr_hw_queues: c_uint,
    ) -> c_int;
    pub fn blk_mq_alloc_sched_res_batch(
        elv_tbl: *mut xarray,
        set: *mut blk_mq_tag_set,
        nr_hw_queues: c_uint,
    ) -> c_int;
    pub fn blk_mq_alloc_sched_ctx_batch(
        elv_tbl: *mut xarray,
        set: *mut blk_mq_tag_set,
    ) -> c_int;
    pub fn blk_mq_free_sched_ctx_batch(elv_tbl: *mut xarray);
    pub fn blk_mq_free_sched_tags(et: *mut elevator_tags, set: *mut blk_mq_tag_set);
    pub fn blk_mq_free_sched_res(
        res: *mut elevator_resources,
        type_: *mut elevator_type,
        set: *mut blk_mq_tag_set,
    );
    pub fn blk_mq_free_sched_res_batch(elv_table: *mut xarray, set: *mut blk_mq_tag_set);
}

// blk_mq_alloc_sched_data() allocates scheduler-specific data.
// Returns allocated data, NULL when no allocation is needed, or ERR_PTR(-ENOMEM).
#[inline]
pub unsafe fn blk_mq_alloc_sched_data(q: *mut request_queue, e: *mut elevator_type) -> *mut c_void {
    if e.is_null() || (*e).ops.alloc_sched_data.is_none() {
        return core::ptr::null_mut();
    }
    let sched_data = ((*e).ops.alloc_sched_data.unwrap())(q);
    if !sched_data.is_null() { sched_data } else { ERR_PTR(-ENOMEM) }
}

#[inline]
pub unsafe fn blk_mq_free_sched_data(e: *mut elevator_type, data: *mut c_void) {
    if !e.is_null() {
        if let Some(free) = (*e).ops.free_sched_data { free(data); }
    }
}

#[inline]
pub unsafe fn blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx) {
    if test_bit(BLK_MQ_S_SCHED_RESTART, &mut (*hctx).state) {
        __blk_mq_sched_restart(hctx);
    }
}

#[inline]
pub unsafe fn bio_mergeable(bio: *mut bio) -> bool {
    ((*bio).bi_opf & REQ_NOMERGE_FLAGS) == 0
}

#[inline]
pub unsafe fn blk_mq_sched_allow_merge(
    q: *mut request_queue, rq: *mut request, bio: *mut bio,
) -> bool {
    if (*rq).rq_flags & RQF_USE_SCHED != 0 {
        let e = (*q).elevator;
        if let Some(allow_merge) = (*(*e).type_).ops.allow_merge {
            return allow_merge(q, rq, bio);
        }
    }
    true
}

#[inline]
pub unsafe fn blk_mq_sched_completed_request(rq: *mut request, now: u64) {
    if (*rq).rq_flags & RQF_USE_SCHED != 0 {
        let e = (*(*rq).q).elevator;
        if let Some(f) = (*(*e).type_).ops.completed_request { f(rq, now); }
    }
}

#[inline]
pub unsafe fn blk_mq_sched_requeue_request(rq: *mut request) {
    if (*rq).rq_flags & RQF_USE_SCHED != 0 {
        let e = (*(*rq).q).elevator;
        if let Some(f) = (*(*e).type_).ops.requeue_request { f(rq); }
    }
}

#[inline]
pub unsafe fn blk_mq_sched_has_work(hctx: *mut blk_mq_hw_ctx) -> bool {
    let e = (*(*hctx).queue).elevator;
    if !e.is_null() {
        if let Some(f) = (*(*e).type_).ops.has_work { return f(hctx); }
    }
    false
}

#[inline]
pub unsafe fn blk_mq_sched_needs_restart(hctx: *mut blk_mq_hw_ctx) -> bool {
    test_bit(BLK_MQ_S_SCHED_RESTART, &mut (*hctx).state)
}

#[inline]
pub unsafe fn blk_mq_set_min_shallow_depth(q: *mut request_queue, depth: c_uint) {
    let mut hctx: *mut blk_mq_hw_ctx = core::ptr::null_mut();
    let mut i: c_ulong = 0;
    queue_for_each_hw_ctx(q, &mut hctx, &mut i);
    sbitmap_queue_min_shallow_depth(&mut (*(*hctx).sched_tags).bitmap_tags, depth);
}

#[inline]
pub unsafe fn blk_mq_is_sync_read(opf: blk_opf_t) -> bool {
    op_is_sync(opf) && !op_is_write(opf)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
