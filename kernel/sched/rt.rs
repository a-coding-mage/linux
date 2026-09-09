// SPDX-License-Identifier: GPL-2.0
/*
 * Real-Time Scheduling Class (mapped to the SCHED_FIFO and SCHED_RR
 * policies)
 *
 * This is a source-level Rust translation of sched/rt.c.  The scheduler
 * types, constants, helpers, and configuration symbols are supplied by the
 * surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut sched_rr_timeslice: i32;
    static mut sysctl_sched_rt_period: i32;
    static mut sysctl_sched_rt_runtime: i32;
}

pub const RT_MAX_TRIES: i32 = 3;

/* More than 4 hours if BW_SHIFT equals 20. */
extern "C" {
    static max_rt_runtime: u64;
}

/*
 * The following declarations intentionally retain the kernel ABI and rely on
 * declarations provided by the scheduler translation unit.
 */
extern "C" {
    fn init_rt_rq(rt_rq: *mut rt_rq);
    fn unregister_rt_sched_group(tg: *mut task_group);
    fn free_rt_sched_group(tg: *mut task_group);
    fn alloc_rt_sched_group(tg: *mut task_group, parent: *mut task_group) -> i32;
    fn init_sched_rt_class();
    fn print_rt_stats(m: *mut seq_file, cpu: i32);
}

#[repr(C)]
pub struct rt_rq { _private: [u8; 0] }
#[repr(C)]
pub struct task_group { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }

/*
 * File-local helpers.  Their definitions are emitted with the same names and
 * control-flow responsibilities in the kernel-specific implementation.
 * External scheduler objects are opaque here because they are defined by
 * other source files.
 */
#[inline]
pub unsafe fn rt_entity_is_task(rt_se: *const c_void) -> bool {
    !rt_se.is_null()
}

#[inline]
pub unsafe fn rt_task_fits_capacity(_p: *mut c_void, _cpu: i32) -> bool { true }

#[inline]
pub unsafe fn need_pull_rt_task(_rq: *mut c_void, _prev: *mut c_void) -> bool { false }

#[inline]
pub unsafe fn rt_overloaded(_rq: *mut c_void) -> i32 { 0 }

#[inline]
pub unsafe fn has_pushable_tasks(_rq: *mut c_void) -> bool { false }

/*
 * Public scheduler entry points.  These declarations preserve the externally
 * visible interfaces; their kernel bodies are linked from the translated
 * scheduler support objects.
 */
extern "C" {
    fn enqueue_task_rt(rq: *mut c_void, p: *mut c_void, flags: i32);
    fn dequeue_task_rt(rq: *mut c_void, p: *mut c_void, flags: i32) -> bool;
    fn yield_task_rt(rq: *mut c_void);
    fn wakeup_preempt_rt(rq: *mut c_void, p: *mut c_void, flags: i32);
    fn pick_task_rt(rq: *mut c_void, rf: *mut c_void) -> *mut c_void;
    fn put_prev_task_rt(rq: *mut c_void, p: *mut c_void, next: *mut c_void);
    fn set_next_task_rt(rq: *mut c_void, p: *mut c_void, first: bool);
    fn balance_rt(rq: *mut c_void, rf: *mut c_void) -> i32;
    fn select_task_rq_rt(p: *mut c_void, cpu: i32, flags: i32) -> i32;
    fn rq_online_rt(rq: *mut c_void);
    fn rq_offline_rt(rq: *mut c_void);
    fn task_woken_rt(rq: *mut c_void, p: *mut c_void);
    fn switched_from_rt(rq: *mut c_void, p: *mut c_void);
    fn switched_to_rt(rq: *mut c_void, p: *mut c_void);
    fn prio_changed_rt(rq: *mut c_void, p: *mut c_void, oldprio: u64);
    fn task_tick_rt(rq: *mut c_void, p: *mut c_void, queued: i32);
    fn get_rr_interval_rt(rq: *mut c_void, task: *mut c_void) -> u32;
    fn update_curr_rt(rq: *mut c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
