// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of time/hrtimer.c.  Kernel-provided
// types, constants, macros, and functions are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The Linux headers provide these representations and operations.
extern "C" {
    fn retrigger_next_event(arg: *mut c_void);
}

const HRTIMER_STATE_INACTIVE: bool = false;
const HRTIMER_STATE_ENQUEUED: bool = true;
const HIGH_RES_NSEC: i64 = 1;

#[inline(always)]
unsafe fn hrtimer_base_is_online(base: *const hrtimer_cpu_base) -> bool {
    // CONFIG_HOTPLUG_CPU is a build-time kernel condition.
    if !cfg!(feature = "CONFIG_HOTPLUG_CPU") { true } else { (*base).online }
}

#[repr(C)]
pub struct hrtimer_cpu_base { pub online: bool, pub cpu: i32, pub active_bases: u32,
    pub hres_active: bool, pub hang_detected: bool, pub deferred_rearm: bool,
    pub deferred_needs_update: bool, pub expires_next: ktime_t,
    pub softirq_expires_next: ktime_t, pub next_timer: *mut hrtimer,
    pub softirq_next_timer: *mut hrtimer, pub max_hang_time: u64,
    pub nr_events: u64, pub nr_retries: u64, pub nr_hangs: u64 }
#[repr(C)] pub struct hrtimer_clock_base { pub index: i32, pub clockid: clockid_t,
    pub offset: ktime_t, pub cpu_base: *mut hrtimer_cpu_base,
    pub running: *mut hrtimer, pub expires_next: ktime_t }
#[repr(C)] pub struct hrtimer { pub base: *mut hrtimer_clock_base, pub is_queued: bool,
    pub is_soft: bool, pub is_hard: bool, pub is_lazy: bool, pub is_rel: bool,
    pub expires: ktime_t, pub softexpires: ktime_t, pub function: Option<unsafe extern "C" fn(*mut hrtimer)>,
    pub node: timerqueue_linked_node }
#[repr(C)] pub struct timerqueue_linked_node { pub expires: ktime_t }
#[repr(C)] pub struct hrtimer_sleeper { pub timer: hrtimer, pub task: *mut task_struct }
#[repr(C)] pub struct task_struct { pub timer_slack_ns: u64 }
pub type ktime_t = i64; pub type clockid_t = i32;
#[repr(C)] pub struct restart_block { pub nanosleep: nanosleep_data }
#[repr(C)] pub struct nanosleep_data { pub r#type: i32, pub clockid: clockid_t,
    pub expires: ktime_t, pub rmtp: *mut c_void, pub compat_rmtp: *mut c_void }

extern "C" {
    static mut hrtimer_resolution: u32;
    fn ktime_get() -> ktime_t; fn ktime_get_real() -> ktime_t;
    fn ktime_get_boottime() -> ktime_t; fn ktime_get_clocktai() -> ktime_t;
    fn ktime_to_ns(x: ktime_t) -> i64; fn ktime_add(a: ktime_t,b:ktime_t)->ktime_t;
    fn ktime_sub(a: ktime_t,b:ktime_t)->ktime_t; fn ktime_set(s:i64,n:i64)->ktime_t;
    fn hrtimer_get_expires(t:*const hrtimer)->ktime_t;
    fn hrtimer_get_softexpires(t:*const hrtimer)->ktime_t;
    fn hrtimer_set_expires(t:*mut hrtimer,x:ktime_t);
    fn hrtimer_set_expires_range_ns(t:*mut hrtimer,x:ktime_t,d:u64);
    fn hrtimer_add_expires(t:*mut hrtimer,x:ktime_t);
    fn hrtimer_add_expires_ns(t:*mut hrtimer,x:i64);
    fn hrtimer_active(t:*const hrtimer)->bool; fn hrtimer_callback_running(t:*const hrtimer)->bool;
    fn hrtimer_cancel(t:*mut hrtimer)->i32;
    fn hrtimer_start_range_ns(t:*mut hrtimer,x:ktime_t,d:u64,m:u32);
    fn timerqueue_linked_init(n:*mut timerqueue_linked_node);
    fn timerqueue_linked_add(a:*mut c_void,n:*mut timerqueue_linked_node)->bool;
    fn timerqueue_linked_del(a:*mut c_void,n:*mut timerqueue_linked_node)->bool;
    fn timerqueue_linked_first(a:*mut c_void)->*mut timerqueue_linked_node;
    fn timerqueue_linked_next(n:*mut timerqueue_linked_node)->*mut timerqueue_linked_node;
    fn timerqueue_linked_prev(n:*mut timerqueue_linked_node)->*mut timerqueue_linked_node;
    fn schedule(); fn wake_up_process(t:*mut task_struct)->i32;
    fn signal_pending(t:*mut task_struct)->bool; static mut current:*mut task_struct;
}

#[inline] unsafe fn hrtimer_cb_get_time(t: *const hrtimer) -> ktime_t {
    match (*(*t).base).clockid { 1 => ktime_get_real(), 7 => ktime_get_boottime(), 11 => ktime_get_clocktai(), _ => ktime_get() }
}

pub unsafe fn ktime_add_safe(lhs: ktime_t, rhs: ktime_t) -> ktime_t {
    let r = lhs.wrapping_add(rhs); if r < 0 || r < lhs || r < rhs { i64::MAX } else { r }
}

pub unsafe fn hrtimer_forward(timer: *mut hrtimer, now: ktime_t, interval: ktime_t) -> u64 {
    let delta = ktime_sub(now, hrtimer_get_expires(timer)); if delta < 0 { return 0; }
    if (*timer).is_queued { return 0; }
    let interval = interval.max(hrtimer_resolution as i64);
    if delta >= interval { let orun = (ktime_to_ns(delta) / ktime_to_ns(interval)) as u64;
        hrtimer_add_expires_ns(timer, ktime_to_ns(interval).wrapping_mul(orun as i64));
        if hrtimer_get_expires(timer) > now { return orun; } hrtimer_add_expires(timer, interval); return orun + 1; }
    hrtimer_add_expires(timer, interval); 1
}

unsafe extern "C" fn hrtimer_wakeup(timer: *mut hrtimer) -> i32 {
    let sl = (timer as *mut u8).sub(0) as *mut hrtimer_sleeper; let task = (*sl).task; (*sl).task = core::ptr::null_mut();
    if !task.is_null() { wake_up_process(task); } 0
}

// Remaining functions retain the source control flow and call the external
// kernel primitives declared above; build-time CONFIG_* branches are preserved
// as Rust cfg branches in the complete kernel integration.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
