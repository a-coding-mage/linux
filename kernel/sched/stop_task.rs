// SPDX-License-Identifier: GPL-2.0
/*
 * stop-task scheduling class.
 *
 * The stop task is the highest priority task in the system, it preempts
 * everything and will be preempted by nothing.
 *
 * See kernel/stop_machine.c
 */
// Dependency intent: declarations supplied by sched.h are provided elsewhere.

unsafe fn select_task_rq_stop(p: *mut task_struct, _cpu: i32, _flags: i32) -> i32 {
    task_cpu(p) /* stop tasks as never migrate */
}

unsafe fn balance_stop(rq: *mut rq, _rf: *mut rq_flags) -> i32 {
    sched_stop_runnable(rq)
}

unsafe fn wakeup_preempt_stop(_rq: *mut rq, _p: *mut task_struct, _flags: i32) {
    /* we're never preempted */
}

unsafe fn set_next_task_stop(rq: *mut rq, stop: *mut task_struct, _first: bool) {
    (*stop).se.exec_start = rq_clock_task(rq);
}

unsafe fn pick_task_stop(rq: *mut rq, _rf: *mut rq_flags) -> *mut task_struct {
    if !sched_stop_runnable(rq) {
        return core::ptr::null_mut();
    }

    (*rq).stop
}

unsafe fn enqueue_task_stop(rq: *mut rq, _p: *mut task_struct, _flags: i32) {
    add_nr_running(rq, 1);
}

unsafe fn dequeue_task_stop(rq: *mut rq, _p: *mut task_struct, _flags: i32) -> bool {
    sub_nr_running(rq, 1);
    true
}

unsafe fn yield_task_stop(_rq: *mut rq) {
    BUG(); /* the stop task should never yield, its pointless. */
}

unsafe fn put_prev_task_stop(rq: *mut rq, _prev: *mut task_struct, _next: *mut task_struct) {
    update_curr_common(rq);
}

/*
 * scheduler tick hitting a task of our scheduling class.
 *
 * NOTE: This function can be called remotely by the tick offload that
 * goes along full dynticks. Therefore no local assumption can be made
 * and everything must be accessed through the @rq and @curr passed in
 * parameters.
 */
unsafe fn task_tick_stop(_rq: *mut rq, _curr: *mut task_struct, _queued: i32) {}

unsafe fn switching_to_stop(_rq: *mut rq, _p: *mut task_struct) {
    BUG(); /* its impossible to change to this class */
}

unsafe fn prio_changed_stop(rq: *mut rq, p: *mut task_struct, oldprio: u64) {
    if (*p).prio == oldprio {
        return;
    }

    BUG(); /* how!?, what priority? */
}

unsafe fn update_curr_stop(_rq: *mut rq) {}

/*
 * Simple, special scheduling class for the per-CPU stop tasks:
 */
static mut sched_class_stop: sched_class = sched_class {
    enqueue_task: Some(enqueue_task_stop),
    dequeue_task: Some(dequeue_task_stop),
    yield_task: Some(yield_task_stop),

    wakeup_preempt: Some(wakeup_preempt_stop),

    pick_task: Some(pick_task_stop),
    put_prev_task: Some(put_prev_task_stop),
    set_next_task: Some(set_next_task_stop),

    balance: Some(balance_stop),
    select_task_rq: Some(select_task_rq_stop),
    set_cpus_allowed: Some(set_cpus_allowed_common),

    task_tick: Some(task_tick_stop),

    prio_changed: Some(prio_changed_stop),
    switching_to: Some(switching_to_stop),
    update_curr: Some(update_curr_stop),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
