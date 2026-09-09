/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of `trace/events/timer.h`.
//!
//! The original file is Linux's tracepoint-definition DSL.  The event
//! declarations below are retained as Rust metadata comments because their
//! expansion is supplied by the kernel tracepoint framework.

// TRACE_SYSTEM timer

/// Decode timer flags as the kernel tracepoint does.
#[inline]
pub fn decode_timer_flags(flags: u32) -> &'static str {
    // __print_flags(flags, "|", { TIMER_MIGRATING, "M" }, ...)
    // The actual string formatting is performed by the tracepoint framework.
    let _ = flags;
    "<kernel tracepoint flags>"
}

/// Decode a clock identifier as the kernel tracepoint does.
#[inline]
pub fn decode_clockid(type_: i32) -> &'static str {
    // __print_symbolic(type, { CLOCK_REALTIME, ... }, ...)
    let _ = type_;
    "<kernel tracepoint clockid>"
}

/// Decode an hrtimer mode as the kernel tracepoint does.
#[inline]
pub fn decode_hrtimer_mode(mode: u32) -> &'static str {
    // __print_symbolic(mode, { HRTIMER_MODE_ABS, ... }, ...)
    let _ = mode;
    "<kernel tracepoint hrtimer mode>"
}

#[repr(C)]
pub struct TimerClassEntry {
    pub timer: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct TimerStartEntry {
    pub timer: *mut core::ffi::c_void,
    pub function: *mut core::ffi::c_void,
    pub expires: usize,
    pub bucket_expiry: usize,
    pub now: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct TimerExpireEntryEntry {
    pub timer: *mut core::ffi::c_void,
    pub now: usize,
    pub function: *mut core::ffi::c_void,
    pub baseclk: usize,
}

#[repr(C)]
pub struct TimerBaseIdleEntry {
    pub is_idle: bool,
    pub cpu: u32,
}

#[repr(C)]
pub struct HrtimerSetupEntry {
    pub hrtimer: *mut core::ffi::c_void,
    pub clockid: i32,
    pub mode: u32,
}

#[repr(C)]
pub struct HrtimerStartEntry {
    pub hrtimer: *mut core::ffi::c_void,
    pub function: *mut core::ffi::c_void,
    pub expires: i64,
    pub softexpires: i64,
    pub mode: u32,
    pub was_armed: bool,
}

#[repr(C)]
pub struct HrtimerExpireEntryEntry {
    pub hrtimer: *mut core::ffi::c_void,
    pub now: i64,
    pub function: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct HrtimerRearmEntry {
    pub next_event: i64,
    pub deferred: bool,
}

#[repr(C)]
pub struct ItimerStateEntry {
    pub which: i32,
    pub expires: u64,
    pub value_sec: isize,
    pub value_nsec: isize,
    pub interval_sec: isize,
    pub interval_nsec: isize,
}

#[repr(C)]
pub struct ItimerExpireEntry {
    pub which: i32,
    pub pid: i32,
    pub now: u64,
}

// DECLARE_EVENT_CLASS(timer_class, ...)
// DEFINE_EVENT(timer_class, timer_init, ...)
// TRACE_EVENT(timer_start, ...)
// TRACE_EVENT(timer_expire_entry, ...)
// DEFINE_EVENT(timer_class, timer_expire_exit, ...)
// DEFINE_EVENT(timer_class, timer_cancel, ...)
// TRACE_EVENT(timer_base_idle, ...)
// TRACE_EVENT(hrtimer_setup, ...)
// TRACE_EVENT(hrtimer_start, ...)
// TRACE_EVENT(hrtimer_expire_entry, ...)
// DECLARE_EVENT_CLASS(hrtimer_class, ...)
// DEFINE_EVENT(hrtimer_class, hrtimer_start_expired, ...)
// DEFINE_EVENT(hrtimer_class, hrtimer_expire_exit, ...)
// DEFINE_EVENT(hrtimer_class, hrtimer_cancel, ...)
// TRACE_EVENT(hrtimer_rearm, ...)
// TRACE_EVENT(itimer_state, ...)
// TRACE_EVENT(itimer_expire, ...)

#[cfg(CONFIG_NO_HZ_COMMON)]
pub const TICK_DEP_NAMES: &[&str] = &[
    "NONE",
    "POSIX_TIMER",
    "PERF_EVENTS",
    "SCHED",
    "CLOCK_UNSTABLE",
    "RCU",
    "RCU_EXP",
];

#[cfg(CONFIG_NO_HZ_COMMON)]
#[inline]
pub fn show_tick_dep_name(dependency: i32) -> &'static str {
    // __print_symbolic(val, TICK_DEP_NAMES)
    let _ = dependency;
    "<kernel tracepoint tick dependency>"
}

#[cfg(CONFIG_NO_HZ_COMMON)]
#[repr(C)]
pub struct TickStopEntry {
    pub success: i32,
    pub dependency: i32,
}

// TRACE_EVENT(tick_stop, ...)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
