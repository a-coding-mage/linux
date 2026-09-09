/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations supplied by linux/sched.h are referenced here.

use core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    pub prio: c_int,
    pub policy: c_int,
    pub pi_top_task: *mut task_struct,
}

#[inline]
pub unsafe fn rt_prio(prio: c_int) -> bool {
    prio < MAX_RT_PRIO && prio >= MAX_DL_PRIO
}

#[inline]
pub unsafe fn rt_or_dl_prio(prio: c_int) -> bool {
    prio < MAX_RT_PRIO
}

/*
 * Returns true if a task has a priority that belongs to RT class. PI-boosted
 * tasks will return true. Use rt_policy() to ignore PI-boosted tasks.
 */
#[inline]
pub unsafe fn rt_task(p: *mut task_struct) -> bool {
    rt_prio((*p).prio)
}

/*
 * Returns true if a task has a priority that belongs to RT or DL classes.
 * PI-boosted tasks will return true. Use rt_or_dl_task_policy() to ignore
 * PI-boosted tasks.
 */
#[inline]
pub unsafe fn rt_or_dl_task(p: *mut task_struct) -> bool {
    rt_or_dl_prio((*p).prio)
}

/*
 * Returns true if a task has a policy that belongs to RT or DL classes.
 * PI-boosted tasks will return false.
 */
#[inline]
pub unsafe fn rt_or_dl_task_policy(tsk: *mut task_struct) -> bool {
    let policy = (*tsk).policy;

    if policy == SCHED_FIFO || policy == SCHED_RR {
        return true;
    }
    if policy == SCHED_DEADLINE {
        return true;
    }
    false
}

// CONFIG_RT_MUTEXES conditional declarations are preserved with cfg attributes.
#[cfg(CONFIG_RT_MUTEXES)]
extern "C" {
    pub fn rt_mutex_pre_schedule();
    pub fn rt_mutex_schedule();
    pub fn rt_mutex_post_schedule();
}

/*
 * Must hold either p->pi_lock or task_rq(p)->lock.
 */
#[cfg(CONFIG_RT_MUTEXES)]
#[inline]
pub unsafe fn rt_mutex_get_top_task(p: *mut task_struct) -> *mut task_struct {
    (*p).pi_top_task
}

#[cfg(CONFIG_RT_MUTEXES)]
extern "C" {
    pub fn rt_mutex_setprio(p: *mut task_struct, pi_task: *mut task_struct);
    pub fn rt_mutex_adjust_pi(p: *mut task_struct);
}

#[cfg(not(CONFIG_RT_MUTEXES))]
#[inline]
pub unsafe fn rt_mutex_get_top_task(_task: *mut task_struct) -> *mut task_struct {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_RT_MUTEXES))]
#[macro_export]
macro_rules! rt_mutex_adjust_pi {
    ($p:expr) => {{
        let _ = $p;
    }};
}

extern "C" {
    pub fn normalize_rt_tasks();
}

/*
 * default timeslice is 100 msecs (used only for SCHED_RR tasks).
 * Timeslices get refilled after they expire.
 */
pub const RR_TIMESLICE: c_int = 100 * HZ / 1000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
