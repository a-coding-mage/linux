/* SPDX-License-Identifier: GPL-2.0 */
// Translated from loop.c. Declarations referenced below are supplied by the
// corresponding io_uring, wait, and loop dependencies.

#[inline]
unsafe fn io_loop_nr_cqes(ctx: *const io_ring_ctx, lp: *const iou_loop_params) -> i32 {
    (*lp).cq_wait_idx - READ_ONCE((*(*ctx).rings).cq.tail)
}

#[inline]
unsafe fn io_loop_wait_start(ctx: *mut io_ring_ctx, nr_wait: u32) {
    atomic_set(&mut (*ctx).cq_wait_nr, nr_wait);
    set_current_state(TASK_INTERRUPTIBLE);
}

#[inline]
unsafe fn io_loop_wait_finish(ctx: *mut io_ring_ctx) {
    __set_current_state(TASK_RUNNING);
    atomic_set(&mut (*ctx).cq_wait_nr, IO_CQ_WAKE_INIT);
}

unsafe fn io_loop_wait(ctx: *mut io_ring_ctx, lp: *mut iou_loop_params, nr_wait: u32) {
    io_loop_wait_start(ctx, nr_wait);

    if (unlikely(io_local_work_pending(ctx) || io_loop_nr_cqes(ctx, lp) <= 0)
        || READ_ONCE((*ctx).check_cq))
    {
        io_loop_wait_finish(ctx);
        return;
    }

    mutex_unlock(&mut (*ctx).uring_lock);
    schedule();
    io_loop_wait_finish(ctx);
    mutex_lock(&mut (*ctx).uring_lock);
}

unsafe fn __io_run_loop(ctx: *mut io_ring_ctx) -> i32 {
    let mut lp: iou_loop_params = core::mem::zeroed();

    loop {
        let nr_wait: i32;
        let step_res: i32;

        if unlikely(!(*ctx).loop_step.is_some()) {
            return -EFAULT;
        }

        step_res = ((*ctx).loop_step.unwrap())(io_loop_mangle_ctx(ctx), &mut lp);
        if step_res == IOU_LOOP_STOP {
            break;
        }
        if step_res != IOU_LOOP_CONTINUE {
            return -EINVAL;
        }

        nr_wait = io_loop_nr_cqes(ctx, &lp);
        if nr_wait > 0 {
            io_loop_wait(ctx, &mut lp, nr_wait as u32);
        } else {
            nr_wait = 0;
        }

        if task_work_pending(current) {
            mutex_unlock(&mut (*ctx).uring_lock);
            io_run_task_work();
            mutex_lock(&mut (*ctx).uring_lock);
        }
        if unlikely(task_sigpending(current)) {
            return -EINTR;
        }
        io_run_local_work_locked(ctx, nr_wait as u32);

        if READ_ONCE((*ctx).check_cq) & BIT(IO_CHECK_CQ_OVERFLOW_BIT) != 0 {
            io_cqring_overflow_flush_locked(ctx);
        }
    }

    0
}

unsafe fn io_run_loop(ctx: *mut io_ring_ctx) -> i32 {
    let ret: i32;

    if !io_allowed_run_tw(ctx) {
        return -EEXIST;
    }

    mutex_lock(&mut (*ctx).uring_lock);
    ret = __io_run_loop(ctx);
    mutex_unlock(&mut (*ctx).uring_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
