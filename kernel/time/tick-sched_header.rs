/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally: linux/hrtimer.h

#[repr(C)]
pub enum tick_device_mode {
    TICKDEV_MODE_PERIODIC,
    TICKDEV_MODE_ONESHOT,
}

#[repr(C)]
pub struct tick_device {
    pub evtdev: *mut clock_event_device,
    pub mode: tick_device_mode,
}

/* The CPU is in the tick idle mode */
pub const TS_FLAG_INIDLE: usize = BIT(0);
/* The idle tick has been stopped */
pub const TS_FLAG_STOPPED: usize = BIT(1);
/*
 * Indicator that the CPU is actively in the tick idle mode;
 * it is reset during irq handling phases.
 */
pub const TS_FLAG_IDLE_ACTIVE: usize = BIT(2);
/* CPU was the last one doing do_timer before going idle */
pub const TS_FLAG_DO_TIMER_LAST: usize = BIT(3);
/* NO_HZ is enabled */
pub const TS_FLAG_NOHZ: usize = BIT(4);
/* High resolution tick mode */
pub const TS_FLAG_HIGHRES: usize = BIT(5);

/**
 * struct tick_sched - sched tick emulation and no idle tick control/stats
 *
 * @flags:              State flags gathering the TS_FLAG_* features
 * @got_idle_tick:      Tick timer function has run with @inidle set
 * @stalled_jiffies:    Number of stalled jiffies detected across ticks
 * @last_tick_jiffies:  Value of jiffies seen on last tick
 * @sched_timer:        hrtimer to schedule the periodic tick in high
 *                      resolution mode
 * @last_tick:          Store the last tick expiry time when the tick
 *                      timer is modified for nohz sleeps. This is necessary
 *                      to resume the tick timer operation in the timeline
 *                      when the CPU returns from nohz sleep.
 * @next_tick:          Next tick to be fired when in dynticks mode.
 * @idle_waketime:      Time when the idle was interrupted
 * @idle_entrytime:     Time when the idle call was entered
 * @last_jiffies:       Base jiffies snapshot when next event was last computed
 * @timer_expires_base: Base time clock monotonic for @timer_expires
 * @timer_expires:      Anticipated timer expiration time (in case sched tick is stopped)
 * @next_timer:         Expiry time of next expiring timer for debugging purpose only
 * @idle_expires:       Next tick in idle, for debugging purpose only
 * @idle_calls:         Total number of idle calls
 * @idle_sleeps:        Number of idle calls, where the sched tick was stopped
 * @tick_dep_mask:      Tick dependency mask - is set, if someone needs the tick
 * @check_clocks:       Notification mechanism about clocksource changes
 */
#[repr(C)]
pub struct tick_sched {
    /* Common flags */
    pub flags: ::core::ffi::c_ulong,

    /* Tick handling: jiffies stall check */
    pub stalled_jiffies: ::core::ffi::c_uint,
    pub last_tick_jiffies: ::core::ffi::c_ulong,

    /* Tick handling */
    pub sched_timer: hrtimer,
    pub last_tick: ktime_t,
    pub next_tick: ktime_t,
    pub idle_waketime: ktime_t,
    pub got_idle_tick: ::core::ffi::c_uint,

    /* Idle entry */
    pub idle_entrytime: ktime_t,

    /* Tick stop */
    pub last_jiffies: ::core::ffi::c_ulong,
    pub timer_expires_base: u64,
    pub timer_expires: u64,
    pub next_timer: u64,
    pub idle_expires: ktime_t,
    pub idle_calls: ::core::ffi::c_ulong,
    pub idle_sleeps: ::core::ffi::c_ulong,

    /* Full dynticks handling */
    pub tick_dep_mask: atomic_t,

    /* Clocksource changes */
    pub check_clocks: ::core::ffi::c_ulong,
}

unsafe extern "C" {
    pub fn tick_get_tick_sched(cpu: ::core::ffi::c_int) -> *mut tick_sched;
    pub fn tick_setup_sched_timer(hrtimer: bool);
}

#[cfg(CONFIG_TICK_ONESHOT)]
unsafe extern "C" {
    pub fn tick_sched_timer_dying(cpu: ::core::ffi::c_int);
}

#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline]
pub fn tick_sched_timer_dying(_cpu: ::core::ffi::c_int) {}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)]
unsafe extern "C" {
    pub fn __tick_broadcast_oneshot_control(state: tick_broadcast_state) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST))]
#[inline]
pub fn __tick_broadcast_oneshot_control(_state: tick_broadcast_state) -> ::core::ffi::c_int {
    -(EBUSY as ::core::ffi::c_int)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
