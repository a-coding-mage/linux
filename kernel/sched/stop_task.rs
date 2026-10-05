// SPDX-License-Identifier: GPL-2.0
/*
 * stop-task scheduling class.
 *
 * The stop task is the highest priority task in the system, it preempts
 * everything and will be preempted by nothing.
 *
 * See kernel/stop_machine.c
 */
// Continued from 126a30fae3bba11420ec2fcbde51a0a01bab1b5b; source-only.
// Native DEFINE_SCHED_CLASS(stop) owns the class layout, const storage and section.
#![no_std]

#[cfg(CONFIG_RUST_SCHED_STOP_TASK)]
compile_error!("SOURCE ONLY HOLD: scheduler stop-task owner is not admitted");

use kernel::bindings::sched_stop_task_native::*;
use kernel::ffi::c_int;

// SAFETY CONTRACT: These callbacks are entered through the native sched_class
// with the same task/runqueue lifetime and locking requirements as stop_task.c.
#[export_name = "lupos_stop_task_select_task_rq"]
unsafe extern "C" fn select_task_rq_stop(p: *mut task_struct, _cpu: c_int, _flags: c_int) -> c_int {
    // SAFETY: The stop task is live and pinned to its native CPU.
    unsafe { lupos_stop_task_cpu(p) } /* stop tasks never migrate */
}

#[export_name = "lupos_stop_task_balance"]
unsafe extern "C" fn balance_stop(rq: *mut rq, _rf: *mut rq_flags) -> c_int {
    // SAFETY: The class caller holds the runqueue lock.
    unsafe { lupos_stop_task_runnable(rq) as c_int }
}

#[export_name = "lupos_stop_task_wakeup_preempt"]
unsafe extern "C" fn wakeup_preempt_stop(_rq: *mut rq, _p: *mut task_struct, _flags: c_int) {
    /* we're never preempted */
}

#[export_name = "lupos_stop_task_set_next_task"]
unsafe extern "C" fn set_next_task_stop(rq: *mut rq, stop: *mut task_struct, _first: bool) {
    // SAFETY: The locked native class transition owns this execution timestamp.
    unsafe { (*stop).se.exec_start = lupos_stop_task_rq_clock_task(rq) };
}

#[export_name = "lupos_stop_task_pick_task"]
unsafe extern "C" fn pick_task_stop(rq: *mut rq, _rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The native pick path holds rq's lock and its stop task remains live.
    unsafe {
        if !lupos_stop_task_runnable(rq) {
            return core::ptr::null_mut();
        }
        (*rq).stop
    }
}

#[export_name = "lupos_stop_task_enqueue_task"]
unsafe extern "C" fn enqueue_task_stop(rq: *mut rq, _p: *mut task_struct, _flags: c_int) {
    // SAFETY: The native enqueue caller holds rq's lock.
    unsafe { lupos_stop_task_add_nr_running(rq, 1) };
}

#[export_name = "lupos_stop_task_dequeue_task"]
unsafe extern "C" fn dequeue_task_stop(rq: *mut rq, _p: *mut task_struct, _flags: c_int) -> bool {
    // SAFETY: The native dequeue caller holds rq's lock.
    unsafe { lupos_stop_task_sub_nr_running(rq, 1) };
    true
}

#[export_name = "lupos_stop_task_yield_task"]
unsafe extern "C" fn yield_task_stop(_rq: *mut rq) {
    // SAFETY: Preserve the original native BUG diagnostic for impossible yield.
    unsafe { lupos_stop_task_bug_yield() }; /* the stop task should never yield */
}

#[export_name = "lupos_stop_task_put_prev_task"]
unsafe extern "C" fn put_prev_task_stop(rq: *mut rq, _prev: *mut task_struct, _next: *mut task_struct) {
    // SAFETY: The native class transition holds rq's lock and updates its clock.
    unsafe { update_curr_common(rq) };
}

/*
 * scheduler tick hitting a task of our scheduling class.
 *
 * NOTE: This function can be called remotely by the tick offload that
 * goes along full dynticks. Therefore no local assumption can be made
 * and everything must be accessed through the @rq and @curr passed in
 * parameters.
 */
#[export_name = "lupos_stop_task_task_tick"]
unsafe extern "C" fn task_tick_stop(_rq: *mut rq, _curr: *mut task_struct, _queued: c_int) {}

#[export_name = "lupos_stop_task_switching_to"]
unsafe extern "C" fn switching_to_stop(_rq: *mut rq, _p: *mut task_struct) {
    // SAFETY: Preserve the original native BUG diagnostic for an invalid class.
    unsafe { lupos_stop_task_bug_switching() }; /* impossible to change to this class */
}

#[export_name = "lupos_stop_task_prio_changed"]
unsafe extern "C" fn prio_changed_stop(_rq: *mut rq, p: *mut task_struct, oldprio: u64) {
    // SAFETY: The native caller retains p under the class priority-change lock.
    unsafe {
        // C compares signed int prio with u64 using the unsigned conversion.
        if (*p).prio as u64 == oldprio {
            return;
        }
        lupos_stop_task_bug_prio(); /* how!?, what priority? */
    }
}

#[export_name = "lupos_stop_task_update_curr"]
unsafe extern "C" fn update_curr_stop(_rq: *mut rq) {}

/*
 * Simple, special scheduling class for the per-CPU stop tasks:
 */
// Defined once by sched_stop_task_registration.inc in the native metadata
// owner, using the configured DEFINE_SCHED_CLASS(stop), with these Rust callbacks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
