// SPDX-License-Identifier: GPL-2.0
/*
 * Task work handling for io_uring
 */

// External kernel and project dependencies are supplied by the surrounding crate.

unsafe fn ctx_flush_and_put(ctx: *mut io_ring_ctx, _tw: io_tw_token_t) {
    if ctx.is_null() {
        return;
    }
    if (*ctx).flags & IORING_SETUP_TASKRUN_FLAG != 0 {
        atomic_andnot(IORING_SQ_TASKRUN, &mut (*(*ctx).rings).sq_flags);
    }

    io_submit_flush_completions(ctx);
    mutex_unlock(&mut (*ctx).uring_lock);
    percpu_ref_put(&mut (*ctx).refs);
}

pub unsafe fn io_tctx_fallback_work(work: *mut work_struct) {
    let tctx = container_of(work, io_uring_task, fallback_work);
    let mut count: u32 = 0;

    /*
     * Run the entries directly. We're in PF_KTHRED context, hence
     * io_should_terminate_tw() is true and they will be marked as canceled.
     */
    tctx_task_work_run(tctx, u32::MAX, &mut count);
    put_task_struct((*tctx).task);
}

unsafe fn io_fallback_tw(tctx: *mut io_uring_task) {
    /*
     * The task ref both keeps ->task valid and, as __io_uring_free() is
     * only called when the task itself is freed, ensures the tctx (and
     * the queued work) stay around until the drain has run.
     */
    get_task_struct((*tctx).task);
    if !queue_work(system_dfl_wq, &mut (*tctx).fallback_work) {
        put_task_struct((*tctx).task);
    }
}

/*
 * Run queued task_work, processing no more than max_entries, with the number
 * of entries processed added to *count. If more entries than max_entries are
 * available, the remainder simply stay on the queue for the next run.
 */
pub unsafe fn tctx_task_work_run(tctx: *mut io_uring_task, max_entries: u32, count: *mut u32) {
    let mut ctx: *mut io_ring_ctx = core::ptr::null_mut();
    let mut ts = io_tw_state::default();

    while *count < max_entries {
        let node = mpscq_pop(&mut (*tctx).task_list, &mut (*tctx).task_head);
        if node.is_null() {
            if mpscq_empty(&(*tctx).task_list) {
                break;
            }
            ctx_flush_and_put(ctx, ts);
            ctx = core::ptr::null_mut();
            cond_resched();
            continue;
        }
        let req = container_of(node, io_kiocb, io_task_work.node);
        if (*req).ctx != ctx {
            ctx_flush_and_put(ctx, ts);
            ctx = (*req).ctx;
            mutex_lock(&mut (*ctx).uring_lock);
            percpu_ref_get(&mut (*ctx).refs);
            ts.cancel = io_should_terminate_tw(ctx);
        }
        indirect_call_2((*req).io_task_work.func, io_poll_task_func, io_req_rw_complete,
                        io_tw_req { req }, ts);
        *count += 1;
        if mpscq_pop_emptied(&(*tctx).task_list, (*tctx).task_head) {
            break;
        }
        if unlikely(need_resched()) {
            ctx_flush_and_put(ctx, ts);
            ctx = core::ptr::null_mut();
            cond_resched();
        }
    }
    ctx_flush_and_put(ctx, ts);

    /* Relaxed read is enough as only the task itself sets ->in_cancel. */
    if unlikely(atomic_read(&(*tctx).in_cancel) != 0) && (*current).io_uring == tctx {
        io_uring_drop_tctx_refs(current);
    }

    trace_io_uring_task_work_run(tctx, *count);
}

pub unsafe fn tctx_task_work(cb: *mut callback_head) {
    let tctx = container_of(cb, io_uring_task, task_work);
    let mut count: u32 = 0;
    tctx_task_work_run(tctx, u32::MAX, &mut count);
}

/* Sets IORING_SQ_TASKRUN in the sq_flags shared with userspace. */
unsafe fn io_ctx_mark_taskrun(ctx: *mut io_ring_ctx) {
    lockdep_assert_in_rcu_read_lock();
    if (*ctx).flags & IORING_SETUP_TASKRUN_FLAG != 0 {
        let rings = rcu_dereference((*ctx).rings_rcu);
        atomic_or(IORING_SQ_TASKRUN, &mut (*rings).sq_flags);
    }
}

pub unsafe fn io_req_local_work_add(req: *mut io_kiocb, mut flags: u32) {
    let ctx = (*req).ctx;
    let mut nr_wait: i32;
    // guard(rcu)();
    if (*req).flags & IO_REQ_LINK_FLAGS != 0 {
        flags &= !IOU_F_TWQ_LAZY_WAKE;
    }
    if mpscq_push(&mut (*ctx).work_list, &mut (*req).io_task_work.node) {
        io_ctx_mark_taskrun(ctx);
        if data_race((*ctx).int_flags) & IO_RING_F_HAS_EVFD != 0 {
            io_eventfd_signal(ctx, false, flags & IOU_F_TWQ_IN_WAKE != 0);
        }
    }
    nr_wait = atomic_read(&(*ctx).cq_wait_nr);
    if nr_wait <= 0 { return; }
    if flags & IOU_F_TWQ_LAZY_WAKE != 0 {
        if !atomic_dec_and_test(&mut (*ctx).cq_wait_nr) { return; }
    } else if atomic_xchg(&mut (*ctx).cq_wait_nr, IO_CQ_WAKE_INIT) <= 0 {
        return;
    }
    wake_up_state((*ctx).submitter_task, TASK_INTERRUPTIBLE);
}

pub unsafe fn io_req_normal_work_add(req: *mut io_kiocb) {
    let tctx = (*req).tctx;
    let ctx = (*req).ctx;
    if !mpscq_push(&mut (*tctx).task_list, &mut (*req).io_task_work.node) { return; }
    if (*ctx).flags & IORING_SETUP_TASKRUN_FLAG != 0 {
        atomic_or(IORING_SQ_TASKRUN, &mut (*(*ctx).rings).sq_flags);
    }
    if (*ctx).flags & IORING_SETUP_SQPOLL != 0 {
        __set_notify_signal((*tctx).task);
        return;
    }
    if likely(!task_work_add((*tctx).task, &mut (*tctx).task_work, (*ctx).notify_method)) { return; }
    io_fallback_tw(tctx);
}

pub unsafe fn io_req_task_work_add_remote(req: *mut io_kiocb, flags: u32) {
    if warn_on_once((*req).ctx.flags & IORING_SETUP_DEFER_TASKRUN == 0) { return; }
    __io_req_task_work_add(req, flags);
}

pub unsafe fn io_cancel_local_task_work(ctx: *mut io_ring_ctx) {
    let ts = io_tw_state { cancel: true, ..Default::default() };
    // guard(mutex)(&ctx->uring_lock);
    while !mpscq_empty(&(*ctx).work_list) {
        let node = mpscq_pop(&mut (*ctx).work_list, &mut (*ctx).work_head);
        if node.is_null() { cond_resched(); continue; }
        let req = container_of(node, io_kiocb, io_task_work.node);
        ((*req).io_task_work.func)(io_tw_req { req }, ts);
    }
    io_submit_flush_completions(ctx);
}

unsafe fn io_run_local_work_continue(ctx: *mut io_ring_ctx, events: i32, min_events: i32) -> bool {
    if !io_local_work_pending(ctx) { return false; }
    if events < min_events { return true; }
    if (*ctx).flags & IORING_SETUP_TASKRUN_FLAG != 0 {
        atomic_or(IORING_SQ_TASKRUN, &mut (*(*ctx).rings).sq_flags);
    }
    false
}

unsafe fn __io_run_local_work_loop(ctx: *mut io_ring_ctx, tw: io_tw_token_t, events: i32) -> i32 {
    let mut ret = 0;
    while ret < events {
        let node = mpscq_pop(&mut (*ctx).work_list, &mut (*ctx).work_head);
        if node.is_null() { break; }
        let req = container_of(node, io_kiocb, io_task_work.node);
        indirect_call_2((*req).io_task_work.func, io_poll_task_func, io_req_rw_complete,
                        io_tw_req { req }, tw);
        ret += 1;
    }
    ret
}

unsafe fn __io_run_local_work(ctx: *mut io_ring_ctx, mut tw: io_tw_token_t, min_events: i32, max_events: i32) -> i32 {
    let mut loops = 0;
    let mut ret = 0;
    if warn_on_once((*ctx).submitter_task != current) { return -EEXIST; }
    if (*ctx).flags & IORING_SETUP_TASKRUN_FLAG != 0 { atomic_andnot(IORING_SQ_TASKRUN, &mut (*(*ctx).rings).sq_flags); }
    loop {
        if unlikely(loops != 0 && ret == 0) { cond_resched(); }
        tw.cancel = io_should_terminate_tw(ctx);
        let remaining = min_events - ret;
        ret = __io_run_local_work_loop(ctx, tw, max_events);
        loops += 1;
        if io_run_local_work_continue(ctx, ret, remaining) { continue; }
        io_submit_flush_completions(ctx);
        if io_run_local_work_continue(ctx, ret, remaining) { continue; }
        trace_io_uring_local_work_run(ctx, ret, loops);
        return ret;
    }
}

pub unsafe fn io_run_local_work_locked(ctx: *mut io_ring_ctx, min_events: i32) -> i32 {
    if !io_local_work_pending(ctx) { return 0; }
    __io_run_local_work(ctx, io_tw_state::default(), min_events, max(IO_LOCAL_TW_DEFAULT_MAX, min_events))
}

pub unsafe fn io_run_local_work(ctx: *mut io_ring_ctx, min_events: i32, max_events: i32) -> i32 {
    mutex_lock(&mut (*ctx).uring_lock);
    let ret = __io_run_local_work(ctx, io_tw_state::default(), min_events, max_events);
    mutex_unlock(&mut (*ctx).uring_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
