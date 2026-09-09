/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/hrtimer.h, linux/ktime.h, linux/timerqueue.h, linux/seqlock.h

#[repr(C)]
pub struct hrtimer_clock_base {
    /// per cpu clock base
    pub cpu_base: *mut hrtimer_cpu_base,
    /// clock type index for per_cpu support when moving a timer to a base on another cpu.
    pub index: core::ffi::c_uint,
    /// clock id for per_cpu support
    pub clockid: clockid_t,
    /// seqcount around __run_hrtimer
    pub seq: seqcount_raw_spinlock_t,
    /// Absolute time of the next event in this clock base
    pub expires_next: ktime_t,
    /// pointer to the currently running hrtimer
    pub running: *mut hrtimer,
    /// red black tree root node for the active timers
    pub active: timerqueue_linked_head,
    /// offset of this clock to the monotonic base
    pub offset: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hrtimer_base_type {
    HRTIMER_BASE_MONOTONIC,
    HRTIMER_BASE_REALTIME,
    HRTIMER_BASE_BOOTTIME,
    HRTIMER_BASE_TAI,
    HRTIMER_BASE_MONOTONIC_SOFT,
    HRTIMER_BASE_REALTIME_SOFT,
    HRTIMER_BASE_BOOTTIME_SOFT,
    HRTIMER_BASE_TAI_SOFT,
    HRTIMER_MAX_CLOCK_BASES,
}

#[repr(C)]
pub struct hrtimer_cpu_base {
    /// lock protecting the base and associated clock bases and timers
    pub lock: raw_spinlock_t,
    /// cpu number
    pub cpu: core::ffi::c_uint,
    /// Bitfield to mark bases with active timers
    pub active_bases: core::ffi::c_uint,
    /// Sequence counter of clock was set events
    pub clock_was_set_seq: u32,
    /// State of high resolution mode
    pub hres_active: bool,
    /// A deferred rearm is pending
    pub deferred_rearm: bool,
    /// The deferred rearm must re-evaluate the first timer
    pub deferred_needs_update: bool,
    /// The last hrtimer interrupt detected a hang
    pub hang_detected: bool,
    /// displays, if the softirq is raised - update of softirq related settings is not required then.
    pub softirq_activated: bool,
    /// CPU is online from an hrtimers point of view
    pub online: bool,
    #[cfg(feature = "CONFIG_HIGH_RES_TIMERS")]
    pub nr_events: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_HIGH_RES_TIMERS")]
    pub nr_retries: core::ffi::c_ushort,
    #[cfg(feature = "CONFIG_HIGH_RES_TIMERS")]
    pub nr_hangs: core::ffi::c_ushort,
    #[cfg(feature = "CONFIG_HIGH_RES_TIMERS")]
    pub max_hang_time: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    pub softirq_expiry_lock: spinlock_t,
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    pub timer_waiters: atomic_t,
    /// Absolute time of the next event, required for remote hrtimer enqueue.
    /// It is the total first expiry time (hard and soft hrtimer are taken into account).
    pub expires_next: ktime_t,
    /// Pointer to the first expiring timer
    pub next_timer: *mut hrtimer,
    /// Time to check, if soft queues needs also to be expired
    pub softirq_expires_next: ktime_t,
    /// Pointer to the first expiring softirq based timer
    pub softirq_next_timer: *mut hrtimer,
    /// Cached expires next value for deferred rearm
    pub deferred_expires_next: ktime_t,
    /// Array of clock bases for this cpu
    pub clock_base: [hrtimer_clock_base; HRTIMER_MAX_CLOCK_BASES as usize],
    pub csd: call_single_data_t,
}

/*
 * Helper function to check, whether the timer is running the callback
 * function
 */
#[inline]
pub unsafe fn hrtimer_callback_running(timer: *mut hrtimer) -> core::ffi::c_int {
    ((*(*timer).base).running == timer) as core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
