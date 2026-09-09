// SPDX-License-Identifier: GPL-2.0
/*
 * hrtimers - High-resolution kernel timers
 *
 * data type definitions, declarations, prototypes
 */

// Dependencies supplied by other translated headers are intentionally external.
pub enum clock_event_device {}
pub enum task_struct {}
pub enum restart_block {}
pub struct timespec64 { _private: [u8; 0] }

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum hrtimer_mode {
    HRTIMER_MODE_ABS = 0x00,
    HRTIMER_MODE_REL = 0x01,
    HRTIMER_MODE_PINNED = 0x02,
    HRTIMER_MODE_SOFT = 0x04,
    HRTIMER_MODE_HARD = 0x08,
    HRTIMER_MODE_LAZY_REARM = 0x10,
}
pub const HRTIMER_MODE_ABS_PINNED: hrtimer_mode = unsafe { core::mem::transmute(0x02u32) };
pub const HRTIMER_MODE_REL_PINNED: hrtimer_mode = unsafe { core::mem::transmute(0x03u32) };
pub const HRTIMER_MODE_ABS_SOFT: hrtimer_mode = unsafe { core::mem::transmute(0x04u32) };
pub const HRTIMER_MODE_REL_SOFT: hrtimer_mode = unsafe { core::mem::transmute(0x05u32) };
pub const HRTIMER_MODE_ABS_PINNED_SOFT: hrtimer_mode = unsafe { core::mem::transmute(0x06u32) };
pub const HRTIMER_MODE_REL_PINNED_SOFT: hrtimer_mode = unsafe { core::mem::transmute(0x07u32) };
pub const HRTIMER_MODE_ABS_HARD: hrtimer_mode = unsafe { core::mem::transmute(0x08u32) };
pub const HRTIMER_MODE_REL_HARD: hrtimer_mode = unsafe { core::mem::transmute(0x09u32) };
pub const HRTIMER_MODE_ABS_PINNED_HARD: hrtimer_mode = unsafe { core::mem::transmute(0x0au32) };
pub const HRTIMER_MODE_REL_PINNED_HARD: hrtimer_mode = unsafe { core::mem::transmute(0x0bu32) };

#[repr(C)]
pub struct hrtimer_sleeper {
    pub timer: hrtimer,
    pub task: *mut task_struct,
}

#[inline]
pub unsafe fn hrtimer_set_expires(timer: *mut hrtimer, time: ktime_t) {
    (*timer).node.expires = time;
    (*timer)._softexpires = time;
}

#[inline]
pub unsafe fn hrtimer_set_expires_range(timer: *mut hrtimer, time: ktime_t, delta: ktime_t) {
    (*timer)._softexpires = time;
    (*timer).node.expires = ktime_add_safe(time, delta);
}

#[inline]
pub unsafe fn hrtimer_set_expires_range_ns(timer: *mut hrtimer, time: ktime_t, delta: u64) {
    (*timer)._softexpires = time;
    (*timer).node.expires = ktime_add_safe(time, ns_to_ktime(delta));
}

#[inline]
pub unsafe fn hrtimer_add_expires(timer: *mut hrtimer, time: ktime_t) {
    (*timer).node.expires = ktime_add_safe((*timer).node.expires, time);
    (*timer)._softexpires = ktime_add_safe((*timer)._softexpires, time);
}

#[inline]
pub unsafe fn hrtimer_add_expires_ns(timer: *mut hrtimer, ns: u64) {
    (*timer).node.expires = ktime_add_ns((*timer).node.expires, ns);
    (*timer)._softexpires = ktime_add_ns((*timer)._softexpires, ns);
}

#[inline]
pub unsafe fn hrtimer_get_expires(timer: *const hrtimer) -> ktime_t { (*timer).node.expires }

#[inline]
pub unsafe fn hrtimer_get_softexpires(timer: *const hrtimer) -> ktime_t { (*timer)._softexpires }

extern "C" {
    pub static mut hrtimer_resolution: u32;
    pub fn hrtimer_interrupt(dev: *mut clock_event_device);
    pub fn timerfd_clock_was_set();
    pub fn timerfd_resume();
    pub fn hrtimer_cb_get_time(timer: *const hrtimer) -> ktime_t;
    pub fn hrtimer_start_range_ns(timer: *mut hrtimer, tim: ktime_t, range_ns: u64, mode: hrtimer_mode);
    pub fn hrtimer_start_range_ns_user(timer: *mut hrtimer, tim: ktime_t, range_ns: u64, mode: hrtimer_mode) -> bool;
    pub fn hrtimer_cancel(timer: *mut hrtimer) -> i32;
    pub fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> i32;
    pub fn hrtimer_sleeper_start_expires(sl: *mut hrtimer_sleeper, mode: hrtimer_mode);
    pub fn __hrtimer_get_remaining(timer: *const hrtimer, adjust: bool) -> ktime_t;
    pub fn hrtimer_get_next_event() -> ktime_t;
    pub fn hrtimer_next_event_without(exclude: *const hrtimer) -> ktime_t;
    pub fn hrtimer_active(timer: *const hrtimer) -> bool;
    pub fn hrtimer_update_function(timer: *mut hrtimer, function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>);
    pub fn hrtimer_forward(timer: *mut hrtimer, now: ktime_t, interval: ktime_t) -> u64;
    pub fn nanosleep_copyout(block: *mut restart_block, ts: *mut timespec64) -> i32;
    pub fn hrtimer_nanosleep(rqtp: ktime_t, mode: hrtimer_mode, clockid: clockid_t) -> i64;
    pub fn schedule_hrtimeout_range(expires: *mut ktime_t, delta: u64, mode: hrtimer_mode) -> i32;
    pub fn schedule_hrtimeout_range_clock(expires: *mut ktime_t, delta: u64, mode: hrtimer_mode, clock_id: clockid_t) -> i32;
    pub fn schedule_hrtimeout(expires: *mut ktime_t, mode: hrtimer_mode) -> i32;
    pub fn hrtimer_run_queues();
    pub fn hrtimers_init();
    pub fn sysrq_timer_list_show();
    pub fn hrtimers_prepare_cpu(cpu: u32) -> i32;
    pub fn hrtimers_cpu_starting(cpu: u32) -> i32;
    pub fn hrtimers_cpu_dying(cpu: u32) -> i32;
}

#[inline]
pub unsafe fn hrtimer_dummy_timeout(_unused: *mut hrtimer) -> hrtimer_restart {
    HRTIMER_NORESTART
}

#[inline]
pub unsafe fn hrtimer_cancel_wait_running(_timer: *const hrtimer) {
    cpu_relax();
}

#[inline]
pub unsafe fn destroy_hrtimer_on_stack(_timer: *mut hrtimer) {}

#[inline]
pub unsafe fn hrtimer_expires_remaining(timer: *const hrtimer) -> ktime_t {
    ktime_sub((*timer).node.expires, hrtimer_cb_get_time(timer))
}

// CONFIG_HIGH_RES_TIMERS supplies hrtimer_resolution, hrtimer_interrupt, and the
// high-resolution static key. Without it, resolution is LOW_RES_NSEC and enabled is false.
#[inline]
pub fn hrtimer_highres_enabled() -> bool { false }

#[inline]
pub unsafe fn __hrtimer_expires_remaining_adjusted(timer: *const hrtimer, now: ktime_t) -> ktime_t {
    let mut rem = ktime_sub((*timer).node.expires, now);
    // CONFIG_TIME_LOW_RES adjustment for relative timers.
    if (*timer).is_rel {
        rem -= unsafe { hrtimer_resolution } as _;
    }
    rem
}

#[inline]
pub unsafe fn hrtimer_expires_remaining_adjusted(timer: *const hrtimer) -> ktime_t {
    __hrtimer_expires_remaining_adjusted(timer, hrtimer_cb_get_time(timer))
}

#[inline]
#[inline]
pub unsafe fn hrtimer_start(timer: *mut hrtimer, tim: ktime_t, mode: hrtimer_mode) {
    hrtimer_start_range_ns(timer, tim, 0, mode);
}

#[inline]
pub unsafe fn hrtimer_start_expires(timer: *mut hrtimer, mode: hrtimer_mode) {
    let soft = hrtimer_get_softexpires(timer);
    let hard = hrtimer_get_expires(timer);
    let delta = ktime_to_ns(ktime_sub(hard, soft));
    hrtimer_start_range_ns(timer, soft, delta, mode);
}

#[inline]
pub unsafe fn hrtimer_start_expires_user(timer: *mut hrtimer, mode: hrtimer_mode) -> bool {
    let soft = hrtimer_get_softexpires(timer);
    let hard = hrtimer_get_expires(timer);
    hrtimer_start_range_ns_user(timer, soft, ktime_to_ns(ktime_sub(hard, soft)), mode)
}

#[inline]
pub unsafe fn hrtimer_restart(timer: *mut hrtimer) {
    hrtimer_start_expires(timer, hrtimer_mode::HRTIMER_MODE_ABS);
}

#[inline]
pub unsafe fn hrtimer_get_remaining(timer: *const hrtimer) -> ktime_t {
    __hrtimer_get_remaining(timer, false)
}

#[inline]
pub unsafe fn hrtimer_is_queued(timer: *mut hrtimer) -> bool { (*timer).is_queued }

#[inline]
pub unsafe fn hrtimer_forward_now(timer: *mut hrtimer, interval: ktime_t) -> u64 {
    hrtimer_forward(timer, hrtimer_cb_get_time(timer), interval)
}

// hrtimer_dummy_timeout returns HRTIMER_NORESTART.

extern "C" {
    pub fn hrtimer_setup(timer: *mut hrtimer, function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>, clock_id: clockid_t, mode: hrtimer_mode);
    pub fn hrtimer_setup_on_stack(timer: *mut hrtimer, function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>, clock_id: clockid_t, mode: hrtimer_mode);
    pub fn hrtimer_setup_sleeper_on_stack(sl: *mut hrtimer_sleeper, clock_id: clockid_t, mode: hrtimer_mode);
}

// CONFIG_PREEMPT_RT and CONFIG_DEBUG_OBJECTS_TIMERS conditionally provide their
// respective cancellation-wait and stack-destruction implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
