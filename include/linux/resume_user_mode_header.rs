/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/sched.h, linux/task_work.h, linux/memcontrol.h, linux/rseq.h,
// and linux/blk-cgroup.h.

/**
 * set_notify_resume - cause resume_user_mode_work() to be called
 * @task:        task that will call resume_user_mode_work()
 *
 * Calling this arranges that @task will call resume_user_mode_work()
 * before returning to user mode.  If it's already running in user mode,
 * it will enter the kernel and call resume_user_mode_work() soon.
 * If it's blocked, it will not be woken.
 */
#[inline]
pub unsafe fn set_notify_resume(task: *mut task_struct) {
    if !test_and_set_tsk_thread_flag(task, TIF_NOTIFY_RESUME) {
        kick_process(task);
    }
}

/**
 * resume_user_mode_work - Perform work before returning to user mode
 * @regs:        user-mode registers of @current task
 *
 * This is called when %TIF_NOTIFY_RESUME has been set.  Now we are
 * about to return to user mode, and the user state in @regs can be
 * inspected or adjusted.  The caller in arch code has cleared
 * %TIF_NOTIFY_RESUME before the call.  If the flag gets set again
 * asynchronously, this will be called again before we return to
 * user mode.
 *
 * Called without locks.
 */
#[inline]
pub unsafe fn resume_user_mode_work(regs: *mut pt_regs) {
    clear_thread_flag(TIF_NOTIFY_RESUME);
    /*
     * This barrier pairs with task_work_add()->set_notify_resume() after
     * hlist_add_head(task->task_works);
     */
    smp_mb__after_atomic();
    if unlikely(task_work_pending(current)) {
        task_work_run();
    }

    // Corresponds to #ifdef CONFIG_KEYS_REQUEST_CACHE.
    #[cfg(CONFIG_KEYS_REQUEST_CACHE)]
    {
        if unlikely((*current).cached_requested_key != core::ptr::null_mut()) {
            key_put((*current).cached_requested_key);
            (*current).cached_requested_key = core::ptr::null_mut();
        }
    }

    mem_cgroup_handle_over_high(GFP_KERNEL);
    blkcg_maybe_throttle_current();

    rseq_handle_slowpath(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
