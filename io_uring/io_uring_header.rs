/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from io_uring.h. External kernel types and helpers are supplied by dependencies. */

#[repr(C)]
pub struct io_rings_layout {
    /* size of CQ + headers + SQ offset array */
    pub rings_size: usize,
    pub sq_size: usize,
    pub sq_array_offset: usize,
}

#[repr(C)]
pub struct io_ctx_config {
    pub p: io_uring_params,
    pub layout: io_rings_layout,
    pub uptr: *mut io_uring_params,
}

pub const IORING_FEAT_FLAGS: u32 = IORING_FEAT_SINGLE_MMAP | IORING_FEAT_NODROP |
    IORING_FEAT_SUBMIT_STABLE | IORING_FEAT_RW_CUR_POS | IORING_FEAT_CUR_PERSONALITY |
    IORING_FEAT_FAST_POLL | IORING_FEAT_POLL_32BITS | IORING_FEAT_SQPOLL_NONFIXED |
    IORING_FEAT_EXT_ARG | IORING_FEAT_NATIVE_WORKERS | IORING_FEAT_RSRC_TAGS |
    IORING_FEAT_CQE_SKIP | IORING_FEAT_LINKED_FILE | IORING_FEAT_REG_REG_RING |
    IORING_FEAT_RECVSEND_BUNDLE | IORING_FEAT_MIN_TIMEOUT | IORING_FEAT_RW_ATTR |
    IORING_FEAT_NO_IOWAIT;
pub const IORING_SETUP_FLAGS: u32 = IORING_SETUP_IOPOLL | IORING_SETUP_SQPOLL |
    IORING_SETUP_SQ_AFF | IORING_SETUP_CQSIZE | IORING_SETUP_CLAMP |
    IORING_SETUP_ATTACH_WQ | IORING_SETUP_R_DISABLED | IORING_SETUP_SUBMIT_ALL |
    IORING_SETUP_COOP_TASKRUN | IORING_SETUP_TASKRUN_FLAG | IORING_SETUP_SQE128 |
    IORING_SETUP_CQE32 | IORING_SETUP_SINGLE_ISSUER | IORING_SETUP_DEFER_TASKRUN |
    IORING_SETUP_NO_MMAP | IORING_SETUP_REGISTERED_FD_ONLY | IORING_SETUP_NO_SQARRAY |
    IORING_SETUP_HYBRID_IOPOLL | IORING_SETUP_CQE_MIXED | IORING_SETUP_SQE_MIXED |
    IORING_SETUP_SQ_REWIND;
pub const IORING_ENTER_FLAGS: u32 = IORING_ENTER_GETEVENTS | IORING_ENTER_SQ_WAKEUP |
    IORING_ENTER_SQ_WAIT | IORING_ENTER_EXT_ARG | IORING_ENTER_REGISTERED_RING |
    IORING_ENTER_ABS_TIMER | IORING_ENTER_EXT_ARG_REG | IORING_ENTER_NO_IOWAIT;
pub const SQE_VALID_FLAGS: u32 = IOSQE_FIXED_FILE | IOSQE_IO_DRAIN | IOSQE_IO_LINK |
    IOSQE_IO_HARDLINK | IOSQE_ASYNC | IOSQE_BUFFER_SELECT | IOSQE_CQE_SKIP_SUCCESS;
pub const IO_REQ_LINK_FLAGS: io_req_flags_t = REQ_F_LINK | REQ_F_HARDLINK;

/* Complaint timeout for io_uring cancelation exits, and for io-wq exit worker waiting. */
pub const IO_URING_EXIT_WAIT_MAX: u32 = HZ * 60 * 5;

pub const IOU_COMPLETE: i32 = 0;
pub const IOU_ISSUE_SKIP_COMPLETE: i32 = -EIOCBQUEUED;
pub const IOU_RETRY: i32 = -EAGAIN;
pub const IOU_REQUEUE: i32 = -3072;

#[repr(C)]
pub struct io_defer_entry {
    pub list: list_head,
    pub req: *mut io_kiocb,
}

#[repr(C)]
pub struct io_wait_queue {
    pub wq: wait_queue_entry,
    pub ctx: *mut io_ring_ctx,
    pub cq_tail: u32,
    pub cq_min_tail: u32,
    pub nr_timeouts: u32,
    pub hit_timeout: i32,
    pub min_timeout: ktime_t,
    pub timeout: ktime_t,
    pub t: hrtimer,
    /* CONFIG_NET_RX_BUSY_POLL */
    pub napi_busy_poll_dt: ktime_t,
    pub napi_prefer_busy_poll: bool,
}

pub unsafe fn io_get_rings(ctx: *mut io_ring_ctx) -> *mut io_rings {
    rcu_dereference_check((*ctx).rings_rcu,
        lockdep_is_held(&(*ctx).uring_lock) || lockdep_is_held(&(*ctx).completion_lock))
}

pub unsafe fn io_should_wake(iowq: *mut io_wait_queue) -> bool {
    let ctx = (*iowq).ctx;
    let rings: *mut io_rings;
    let dist: i32;
    guard_rcu();
    rings = io_get_rings(ctx);
    /* Wake on enough events, or whenever a timeout occurred while waiting. */
    dist = READ_ONCE((*rings).cq.tail) as i32 - (*iowq).cq_tail as i32;
    dist >= 0 || atomic_read(&(*ctx).cq_timeouts) != (*iowq).nr_timeouts as i32
}

pub const IORING_MAX_ENTRIES: u32 = 32768;
pub const IORING_MAX_CQ_ENTRIES: u32 = 2 * IORING_MAX_ENTRIES;

extern "C" {
    pub fn io_prepare_config(config: *mut io_ctx_config) -> i32;
    pub fn io_cqe_cache_refill(ctx: *mut io_ring_ctx, overflow: bool, cqe32: bool) -> bool;
    pub fn io_req_defer_failed(req: *mut io_kiocb, res: i32);
    pub fn io_post_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64, res: i32, cflags: u32) -> bool;
    pub fn io_add_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64, res: i32, cflags: u32);
    pub fn io_req_post_cqe(req: *mut io_kiocb, res: i32, cflags: u32) -> bool;
    pub fn io_req_post_cqe32(req: *mut io_kiocb, src_cqe: *mut io_uring_cqe) -> bool;
    pub fn __io_commit_cqring_flush(ctx: *mut io_ring_ctx);
    pub fn io_linked_nr(req: *mut io_kiocb) -> u32;
    pub fn io_req_track_inflight(req: *mut io_kiocb);
    pub fn io_file_get_normal(req: *mut io_kiocb, fd: i32) -> *mut file;
    pub fn io_file_get_fixed(req: *mut io_kiocb, fd: i32, issue_flags: u32) -> *mut file;
    pub fn io_uring_ctx_get_file(fd: u32, registered: bool) -> *mut file;
    pub fn io_req_task_queue(req: *mut io_kiocb);
    pub fn io_req_task_complete(tw_req: io_tw_req, tw: io_tw_token_t);
    pub fn io_req_task_queue_fail(req: *mut io_kiocb, ret: i32);
    pub fn io_req_task_submit(tw_req: io_tw_req, tw: io_tw_token_t);
    pub fn io_uring_drop_tctx_refs(task: *mut task_struct);
    pub fn io_ring_add_registered_file(tctx: *mut io_uring_task, file: *mut file, start: i32, end: i32) -> i32;
    pub fn io_queue_iowq(req: *mut io_kiocb);
    pub fn io_poll_issue(req: *mut io_kiocb, tw: io_tw_token_t) -> i32;
    pub fn io_submit_sqes(ctx: *mut io_ring_ctx, nr: u32) -> i32;
    pub fn io_do_iopoll(ctx: *mut io_ring_ctx, force_nonspin: bool) -> i32;
    pub fn io_iopoll_try_reap_events(ctx: *mut io_ring_ctx);
    pub fn io_free_req(req: *mut io_kiocb);
    pub fn io_queue_next(req: *mut io_kiocb);
    pub fn io_task_refs_refill(tctx: *mut io_uring_task);
    pub fn __io_alloc_req_refill(ctx: *mut io_ring_ctx) -> bool;
    pub fn io_activate_pollwq(ctx: *mut io_ring_ctx);
    pub fn io_restriction_clone(dst: *mut io_restriction, src: *mut io_restriction);
    pub fn io_poison_req(req: *mut io_kiocb);
}

pub unsafe fn io_lockdep_assert_cq_locked(ctx: *mut io_ring_ctx) {
    /* CONFIG_PROVE_LOCKING: lockdep assertions are retained by the external kernel layer. */
    let _ = ctx;
}

pub unsafe fn io_is_compat(ctx: *mut io_ring_ctx) -> bool {
    IS_ENABLED_CONFIG_COMPAT && unlikely((*ctx).int_flags & IO_RING_F_COMPAT != 0)
}

pub unsafe fn io_submit_flush_completions(ctx: *mut io_ring_ctx) {
    if !wq_list_empty(&(*ctx).submit_state.compl_reqs) || (*ctx).submit_state.cq_flush {
        __io_submit_flush_completions(ctx);
    }
}

/* for (pos = head; pos; pos = pos->link) */
pub unsafe fn io_for_each_link<F: FnMut(*mut io_kiocb)>(mut pos: *mut io_kiocb, mut f: F) {
    while !pos.is_null() { f(pos); pos = (*pos).link; }
}

pub unsafe fn io_get_cqe_overflow(ctx: *mut io_ring_ctx, ret: *mut *mut io_uring_cqe,
                                  overflow: bool, cqe32: bool) -> bool {
    io_lockdep_assert_cq_locked(ctx);
    if unlikely((*ctx).cqe_sentinel - (*ctx).cqe_cached < (cqe32 as usize + 1)) {
        if unlikely(!io_cqe_cache_refill(ctx, overflow, cqe32)) { return false; }
    }
    *ret = (*ctx).cqe_cached;
    (*ctx).cached_cq_tail += 1;
    (*ctx).cqe_cached = (*ctx).cqe_cached.add(1);
    if (*ctx).flags & IORING_SETUP_CQE32 != 0 ||
       cqe32 && (*ctx).flags & IORING_SETUP_CQE_MIXED != 0 {
        (*ctx).cqe_cached = (*ctx).cqe_cached.add(1);
        if (*ctx).flags & IORING_SETUP_CQE32 == 0 { (*ctx).cached_cq_tail += 1; }
    }
    WARN_ON_ONCE((*ctx).cqe_cached > (*ctx).cqe_sentinel);
    true
}

pub unsafe fn io_get_cqe(ctx: *mut io_ring_ctx, ret: *mut *mut io_uring_cqe, cqe32: bool) -> bool {
    io_get_cqe_overflow(ctx, ret, false, cqe32)
}

pub unsafe fn io_defer_get_uncommited_cqe(ctx: *mut io_ring_ctx, cqe_ret: *mut *mut io_uring_cqe) -> bool {
    io_lockdep_assert_cq_locked(ctx);
    (*ctx).submit_state.cq_flush = true;
    io_get_cqe(ctx, cqe_ret, (*ctx).flags & IORING_SETUP_CQE_MIXED != 0)
}

pub unsafe fn req_set_fail(req: *mut io_kiocb) {
    (*req).flags |= REQ_F_FAIL;
    if (*req).flags & REQ_F_CQE_SKIP != 0 {
        (*req).flags &= !REQ_F_CQE_SKIP;
        (*req).flags |= REQ_F_SKIP_LINK_CQES;
    }
}
pub unsafe fn io_req_set_res(req: *mut io_kiocb, res: i32, cflags: u32) {
    (*req).cqe.res = res; (*req).cqe.flags = cflags;
}
pub unsafe fn ctx_cqe32_flags(ctx: *mut io_ring_ctx) -> u32 {
    if (*ctx).flags & IORING_SETUP_CQE_MIXED != 0 { IORING_CQE_F_32 } else { 0 }
}
pub unsafe fn io_req_set_res32(req: *mut io_kiocb, res: i32, cflags: u32, extra1: u64, extra2: u64) {
    (*req).cqe.res = res; (*req).cqe.flags = cflags | ctx_cqe32_flags((*req).ctx);
    (*req).big_cqe.extra1 = extra1; (*req).big_cqe.extra2 = extra2;
}

pub unsafe fn io_uring_alloc_async_data(cache: *mut io_alloc_cache, req: *mut io_kiocb) -> *mut core::ffi::c_void {
    if !cache.is_null() {
        (*req).async_data = io_cache_alloc(cache, GFP_KERNEL);
    } else {
        let def = &io_issue_defs[(*req).opcode as usize];
        WARN_ON_ONCE(def.async_size == 0);
        (*req).async_data = kmalloc(def.async_size, GFP_KERNEL);
    }
    if !(*req).async_data.is_null() { (*req).flags |= REQ_F_ASYNC_DATA; }
    (*req).async_data
}
pub unsafe fn req_has_async_data(req: *mut io_kiocb) -> bool { (*req).flags & REQ_F_ASYNC_DATA != 0 }

pub unsafe fn io_req_async_data_clear(req: *mut io_kiocb, extra_flags: io_req_flags_t) {
    (*req).flags &= !(REQ_F_ASYNC_DATA | extra_flags); (*req).async_data = core::ptr::null_mut();
}
pub unsafe fn io_req_async_data_free(req: *mut io_kiocb) {
    kfree((*req).async_data); io_req_async_data_clear(req, 0);
}
pub unsafe fn io_put_file(req: *mut io_kiocb) {
    if (*req).flags & REQ_F_FIXED_FILE == 0 && !(*req).file.is_null() { fput((*req).file); }
}
pub unsafe fn io_ring_submit_unlock(ctx: *mut io_ring_ctx, issue_flags: u32) {
    lockdep_assert_held(&(*ctx).uring_lock); if unlikely(issue_flags & IO_URING_F_UNLOCKED != 0) { mutex_unlock(&(*ctx).uring_lock); }
}
pub unsafe fn io_ring_submit_lock(ctx: *mut io_ring_ctx, issue_flags: u32) {
    if unlikely(issue_flags & IO_URING_F_UNLOCKED != 0) { mutex_lock(&(*ctx).uring_lock); }
    lockdep_assert_held(&(*ctx).uring_lock);
}
pub unsafe fn io_commit_cqring(ctx: *mut io_ring_ctx) { smp_store_release(&mut (*(*ctx).rings).cq.tail, (*ctx).cached_cq_tail); }
pub unsafe fn __io_wq_wake(wq: *mut wait_queue_head) {
    /* Pass EPOLLIN|EPOLL_URING_WAKE as the poll wakeup key. */
    if wq_has_sleeper(wq) { __wake_up(wq, TASK_NORMAL, 0, poll_to_key(EPOLL_URING_WAKE | EPOLLIN)); }
}
pub unsafe fn io_poll_wq_wake(ctx: *mut io_ring_ctx) { __io_wq_wake(&mut (*ctx).poll_wq); }
pub unsafe fn io_cqring_wake(ctx: *mut io_ring_ctx) { __io_wq_wake(&mut (*ctx).cq_wait); }
pub unsafe fn io_sqring_full(ctx: *mut io_ring_ctx) -> bool { guard_rcu(); __io_sqring_full(ctx) }
pub unsafe fn __io_sqring_full(ctx: *mut io_ring_ctx) -> bool {
    let r = io_get_rings(ctx); READ_ONCE((*r).sq.tail) - READ_ONCE((*r).sq.head) == (*ctx).sq_entries
}
pub unsafe fn io_sqring_entries(ctx: *mut io_ring_ctx) -> u32 { guard_rcu(); __io_sqring_entries(ctx) }
pub unsafe fn __io_sqring_entries(ctx: *mut io_ring_ctx) -> u32 {
    let rings = io_get_rings(ctx);
    min(smp_load_acquire(&(*rings).sq.tail) - (*ctx).cached_sq_head, (*ctx).sq_entries)
}
pub unsafe fn io_commit_cqring_flush(ctx: *mut io_ring_ctx) {
    if unlikely(data_race((*ctx).int_flags) & SHOULD_FLUSH_MASK != 0) { __io_commit_cqring_flush(ctx); }
}
pub unsafe fn io_req_complete_defer(req: *mut io_kiocb) {
    lockdep_assert_held(&(*(*req).ctx).uring_lock);
    wq_list_add_tail(&mut (*req).comp_list, &mut (*(*req).ctx).submit_state.compl_reqs);
}
pub const SHOULD_FLUSH_MASK: u32 = IO_RING_F_OFF_TIMEOUT_USED | IO_RING_F_HAS_EVFD | IO_RING_F_POLL_ACTIVATED;
pub unsafe fn io_get_task_refs(nr: i32) { let tctx = (*current()).io_uring; (*tctx).cached_refs -= nr; if unlikely((*tctx).cached_refs < 0) { io_task_refs_refill(tctx); } }
pub unsafe fn io_req_cache_empty(ctx: *mut io_ring_ctx) -> bool { (*ctx).submit_state.free_list.next.is_null() }
pub unsafe fn io_alloc_req(ctx: *mut io_ring_ctx, req: *mut *mut io_kiocb) -> bool {
    if unlikely(io_req_cache_empty(ctx)) && !__io_alloc_req_refill(ctx) { return false; }
    *req = io_extract_req(ctx); true
}
pub unsafe fn io_extract_req(ctx: *mut io_ring_ctx) -> *mut io_kiocb {
    let req = container_of((*ctx).submit_state.free_list.next, io_kiocb, comp_list);
    wq_stack_extract(&mut (*ctx).submit_state.free_list); req
}
pub unsafe fn io_req_queue_tw_complete(req: *mut io_kiocb, res: i32) { io_req_set_res(req, res, 0); (*req).io_task_work.func = Some(io_req_task_complete); io_req_task_work_add(req); }
pub unsafe fn io_file_can_poll(req: *mut io_kiocb) -> bool { if (*req).flags & REQ_F_CAN_POLL != 0 { return true; } if !(*req).file.is_null() && file_can_poll((*req).file) { (*req).flags |= REQ_F_CAN_POLL; return true; } false }
pub unsafe fn io_is_uring_cmd(req: *const io_kiocb) -> bool { (*req).opcode == IORING_OP_URING_CMD || (*req).opcode == IORING_OP_URING_CMD128 }
pub unsafe fn io_get_time(ctx: *mut io_ring_ctx) -> ktime_t { if (*ctx).clockid == CLOCK_MONOTONIC { ktime_get() } else { ktime_get_with_offset((*ctx).clock_offset) } }
pub const IO_CHECK_CQ_OVERFLOW_BIT: u32 = 0;
pub const IO_CHECK_CQ_DROPPED_BIT: u32 = 1;
pub unsafe fn io_has_work(ctx: *mut io_ring_ctx) -> bool { test_bit(IO_CHECK_CQ_OVERFLOW_BIT, &(*ctx).check_cq) || io_local_work_pending(ctx) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
