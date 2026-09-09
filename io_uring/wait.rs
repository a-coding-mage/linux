// SPDX-License-Identifier: GPL-2.0
/*
 * Waiting for completion events
 */

unsafe fn io_wake_function(
    curr: *mut wait_queue_entry,
    mode: c_uint,
    wake_flags: c_int,
    key: *mut c_void,
) -> c_int {
    let iowq = container_of!(curr, io_wait_queue, wq);

    /*
     * Cannot safely flush overflowed CQEs from here, ensure we wake up
     * the task, and the next invocation will do it.
     */
    if io_should_wake(iowq) || io_has_work((*iowq).ctx) {
        return autoremove_wake_function(curr, mode, wake_flags, key);
    }
    -1
}

pub unsafe fn io_run_task_work_sig(ctx: *mut io_ring_ctx) -> c_int {
    if io_local_work_pending(ctx) {
        __set_current_state(TASK_RUNNING);
        if io_run_local_work(ctx, INT_MAX, IO_LOCAL_TW_DEFAULT_MAX) > 0 {
            return 0;
        }
    }
    if io_run_task_work() > 0 {
        return 0;
    }
    if task_sigpending(current) {
        return -EINTR;
    }
    0
}

unsafe fn current_pending_io() -> bool {
    let tctx = (*current).io_uring;

    if tctx.is_null() {
        return false;
    }
    percpu_counter_read_positive(&(*tctx).inflight)
}

unsafe fn io_cqring_timer_wakeup(timer: *mut hrtimer) -> hrtimer_restart {
    let iowq = container_of!(timer, io_wait_queue, t);

    WRITE_ONCE!((*iowq).hit_timeout, 1);
    (*iowq).min_timeout = 0;
    wake_up_process((*iowq).wq.private);
    HRTIMER_NORESTART
}

/*
 * Doing min_timeout portion. If we saw any timeouts, events, or have work,
 * wake up. If not, and we have a normal timeout, switch to that and keep
 * sleeping.
 */
unsafe fn io_cqring_min_timer_wakeup(timer: *mut hrtimer) -> hrtimer_restart {
    let iowq = container_of!(timer, io_wait_queue, t);
    let ctx = (*iowq).ctx;

    if (*iowq).timeout == KTIME_MAX
        || ktime_compare((*iowq).min_timeout, (*iowq).timeout) >= 0
    {
        return io_cqring_timer_wakeup(timer);
    }
    if io_has_work(ctx) {
        return io_cqring_timer_wakeup(timer);
    }
    let rings = io_get_rings(ctx);
    if (*iowq).cq_min_tail != READ_ONCE!((*rings).cq.tail)
        || io_cqring_events(ctx)
    {
        return io_cqring_timer_wakeup(timer);
    }
    if (*ctx).flags & IORING_SETUP_DEFER_TASKRUN != 0 {
        atomic_set(&(*ctx).cq_wait_nr, 1);
        smp_mb();
        if io_local_work_pending(ctx) {
            return io_cqring_timer_wakeup(timer);
        }
    }

    (*iowq).cq_tail = (*iowq).cq_min_tail + 1;
    hrtimer_update_function(&mut (*iowq).t, io_cqring_timer_wakeup);
    hrtimer_set_expires(timer, (*iowq).timeout);
    HRTIMER_RESTART
}

unsafe fn io_cqring_schedule_timeout(
    iowq: *mut io_wait_queue,
    clock_id: clockid_t,
    start_time: ktime_t,
) -> c_int {
    let timeout;
    if (*iowq).min_timeout != 0 {
        timeout = ktime_add_ns((*iowq).min_timeout, start_time);
        hrtimer_setup_on_stack(&mut (*iowq).t, io_cqring_min_timer_wakeup, clock_id, HRTIMER_MODE_ABS);
    } else {
        timeout = (*iowq).timeout;
        hrtimer_setup_on_stack(&mut (*iowq).t, io_cqring_timer_wakeup, clock_id, HRTIMER_MODE_ABS);
    }
    hrtimer_set_expires_range_ns(&mut (*iowq).t, timeout, 0);
    hrtimer_start_expires(&mut (*iowq).t, HRTIMER_MODE_ABS);
    if !READ_ONCE!((*iowq).hit_timeout) { schedule(); }
    hrtimer_cancel(&mut (*iowq).t);
    destroy_hrtimer_on_stack(&mut (*iowq).t);
    __set_current_state(TASK_RUNNING);
    if READ_ONCE!((*iowq).hit_timeout) { -ETIME } else { 0 }
}

unsafe fn __io_cqring_wait_schedule(
    ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue, ext_arg: *mut ext_arg,
    start_time: ktime_t,
) -> c_int {
    let mut ret = 0;
    if (*ext_arg).iowait && current_pending_io() { (*current).in_iowait = 1; }
    if (*iowq).timeout != KTIME_MAX || (*iowq).min_timeout != 0 {
        ret = io_cqring_schedule_timeout(iowq, (*ctx).clockid, start_time);
    } else { schedule(); }
    (*current).in_iowait = 0;
    ret
}

/* If this returns > 0, the caller should retry */
unsafe fn io_cqring_wait_schedule(
    ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue, ext_arg: *mut ext_arg,
    start_time: ktime_t,
) -> c_int {
    if unlikely(READ_ONCE!((*ctx).check_cq)) || unlikely(io_local_work_pending(ctx))
        || unlikely(task_work_pending(current)) { return 1; }
    if unlikely(task_sigpending(current)) { return -EINTR; }
    if unlikely(io_should_wake(iowq)) { return 0; }
    __io_cqring_wait_schedule(ctx, iowq, ext_arg, start_time)
}

/*
 * Wait until events become available, if we don't already have some. The
 * application must reap them itself, as they reside on the shared cq ring.
 */
pub unsafe fn io_cqring_wait(
    ctx: *mut io_ring_ctx, mut min_events: c_int, flags: u32, ext_arg: *mut ext_arg,
) -> c_int {
    min_events = min_t!(c_int, min_events, (*ctx).cq_entries);
    if !io_allowed_run_tw(ctx) { return -EEXIST; }
    if io_local_work_pending(ctx) { io_run_local_work(ctx, min_events, max!(IO_LOCAL_TW_DEFAULT_MAX, min_events)); }
    io_run_task_work();
    if unlikely(test_bit(IO_CHECK_CQ_OVERFLOW_BIT, &(*ctx).check_cq)) { io_cqring_do_overflow_flush(ctx); }
    let rings = io_get_rings(ctx);
    if __io_cqring_events_user(ctx) >= min_events { return 0; }
    let mut iowq: io_wait_queue = core::mem::zeroed();
    init_waitqueue_func_entry(&mut iowq.wq, io_wake_function);
    iowq.wq.private = current;
    INIT_LIST_HEAD(&mut iowq.wq.entry);
    iowq.ctx = ctx;
    iowq.cq_tail = READ_ONCE!((*rings).cq.head) + min_events as _;
    iowq.cq_min_tail = READ_ONCE!((*rings).cq.tail);
    let nr_wait = (iowq.cq_tail - READ_ONCE!((*rings).cq.tail)) as c_int;
    iowq.nr_timeouts = atomic_read(&(*ctx).cq_timeouts);
    iowq.hit_timeout = 0;
    iowq.min_timeout = (*ext_arg).min_time;
    iowq.timeout = KTIME_MAX;
    let start_time = io_get_time(ctx);
    if (*ext_arg).ts_set {
        iowq.timeout = timespec64_to_ktime((*ext_arg).ts);
        if flags & IORING_ENTER_ABS_TIMER != 0 { iowq.timeout = timens_ktime_to_host((*ctx).clockid, iowq.timeout); }
        else { iowq.timeout = ktime_add(iowq.timeout, start_time); }
    }
    io_napi_busy_loop(ctx, &mut iowq);
    trace_io_uring_cqring_wait(ctx, min_events);
    if !(*ext_arg).sig.is_null() {
        let ret = set_user_sigmask((*ext_arg).sig, (*ext_arg).argsz);
        if ret != 0 { return ret; }
    }
    let mut ret;
    loop {
        if (*ctx).flags & IORING_SETUP_DEFER_TASKRUN != 0 { atomic_set(&(*ctx).cq_wait_nr, nr_wait); set_current_state(TASK_INTERRUPTIBLE); }
        else { prepare_to_wait_exclusive(&mut (*ctx).cq_wait, &mut iowq.wq, TASK_INTERRUPTIBLE); }
        ret = io_cqring_wait_schedule(ctx, &mut iowq, ext_arg, start_time);
        __set_current_state(TASK_RUNNING);
        atomic_set(&(*ctx).cq_wait_nr, IO_CQ_WAKE_INIT);
        if io_local_work_pending(ctx) { io_run_local_work(ctx, nr_wait, nr_wait); }
        io_run_task_work();
        if ret < 0 { break; }
        let check_cq = READ_ONCE!((*ctx).check_cq);
        if unlikely(check_cq) {
            if check_cq & BIT!(IO_CHECK_CQ_OVERFLOW_BIT) != 0 { io_cqring_do_overflow_flush(ctx); }
            if check_cq & BIT!(IO_CHECK_CQ_DROPPED_BIT) != 0 { ret = -EBADR; break; }
        }
        if io_should_wake(&mut iowq) { ret = 0; break; }
        cond_resched();
        if !iowq.hit_timeout {
            iowq.cq_tail = READ_ONCE!((*io_get_rings(ctx)).cq.tail) + nr_wait as _;
        }
    }
    if (*ctx).flags & IORING_SETUP_DEFER_TASKRUN == 0 { finish_wait(&mut (*ctx).cq_wait, &mut iowq.wq); }
    restore_saved_sigmask_unless(ret == -EINTR);
    if READ_ONCE!((*io_get_rings(ctx)).cq.head) == READ_ONCE!((*io_get_rings(ctx)).cq.tail) { ret } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
