// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding translation unit:
// linux/sched.h, linux/percpu-refcount.h, linux/io_uring_types.h, and mpscq.h.

pub const IO_LOCAL_TW_DEFAULT_MAX: u32 = 20;

/*
 * Terminate the request if either of these conditions are true:
 *
 * 1) It's being executed by the original task, but that task is marked
 *    with PF_EXITING as it's exiting.
 * 2) PF_KTHREAD is set, in which case the invoker of the task_work is
 *    our fallback task_work.
 * 3) The ring has been closed and is going away.
 */
#[inline]
pub unsafe fn io_should_terminate_tw(ctx: *mut io_ring_ctx) -> bool {
    ((*current).flags & (PF_EXITING | PF_KTHREAD)) != 0
        || percpu_ref_is_dying(&(*ctx).refs)
}

extern "C" {
    pub fn io_req_task_work_add_remote(req: *mut io_kiocb, flags: ::core::ffi::c_uint);
    pub fn tctx_task_work(cb: *mut callback_head);
    pub fn io_tctx_fallback_work(work: *mut work_struct);
    pub fn io_run_local_work(ctx: *mut io_ring_ctx, min_events: ::core::ffi::c_int,
                             max_events: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn io_run_task_work_sig(ctx: *mut io_ring_ctx) -> ::core::ffi::c_int;
    pub fn io_cancel_local_task_work(ctx: *mut io_ring_ctx);
    pub fn io_run_local_work_locked(ctx: *mut io_ring_ctx, min_events: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    pub fn io_req_local_work_add(req: *mut io_kiocb, flags: ::core::ffi::c_uint);
    pub fn io_req_normal_work_add(req: *mut io_kiocb);
    pub fn tctx_task_work_run(tctx: *mut io_uring_task, max_entries: ::core::ffi::c_uint,
                              count: *mut ::core::ffi::c_uint);
}

#[inline]
pub unsafe fn __io_req_task_work_add(req: *mut io_kiocb, flags: ::core::ffi::c_uint) {
    if (*(*req).ctx).flags & IORING_SETUP_DEFER_TASKRUN != 0 {
        io_req_local_work_add(req, flags);
    } else {
        io_req_normal_work_add(req);
    }
}

#[inline]
pub unsafe fn io_req_task_work_add(req: *mut io_kiocb) {
    __io_req_task_work_add(req, 0);
}

#[inline]
pub unsafe fn io_run_task_work() -> bool {
    let mut ret = false;

    /*
     * Always check-and-clear the task_work notification signal. With how
     * signaling works for task_work, we can find it set with nothing to
     * run. We need to clear it for that case, like get_signal() does.
     */
    if test_thread_flag(TIF_NOTIFY_SIGNAL) {
        clear_notify_signal();
    }
    /*
     * PF_IO_WORKER never returns to userspace, so check here if we have
     * notify work that needs processing.
     */
    if ((*current).flags & PF_IO_WORKER) != 0 {
        if test_thread_flag(TIF_NOTIFY_RESUME) {
            __set_current_state(TASK_RUNNING);
            resume_user_mode_work(::core::ptr::null_mut());
        }
        if !(*current).io_uring.is_null() {
            let mut count: ::core::ffi::c_uint = 0;

            __set_current_state(TASK_RUNNING);
            tctx_task_work_run((*current).io_uring, UINT_MAX, &mut count);
            if count != 0 {
                ret = true;
            }
        }
    }
    if task_work_pending(current) {
        __set_current_state(TASK_RUNNING);
        task_work_run();
        ret = true;
    }

    ret
}

#[inline]
pub unsafe fn io_local_work_pending(ctx: *mut io_ring_ctx) -> bool {
    !mpscq_empty(&(*ctx).work_list)
}

#[inline]
pub unsafe fn io_task_work_pending(ctx: *mut io_ring_ctx) -> bool {
    task_work_pending(current) || io_local_work_pending(ctx)
}

#[inline]
pub unsafe fn io_tw_lock(_ctx: *mut io_ring_ctx, _tw: io_tw_token_t) {
    // lockdep_assert_held(&ctx->uring_lock);
}

#[inline]
pub unsafe fn io_allowed_defer_tw_run(ctx: *mut io_ring_ctx) -> bool {
    likely((*ctx).submitter_task == current)
}

#[inline]
pub unsafe fn io_allowed_run_tw(ctx: *mut io_ring_ctx) -> bool {
    likely(((*ctx).flags & IORING_SETUP_DEFER_TASKRUN) == 0
        || (*ctx).submitter_task == current)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
