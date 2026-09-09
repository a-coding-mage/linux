/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the userspace portion of <linux/time.h>.
// The original header includes <linux/types.h> and <linux/time_types.h>.

#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct timespec {
	pub tv_sec: __kernel_old_time_t, // seconds
	pub tv_nsec: core::ffi::c_long,  // nanoseconds
}

#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct timeval {
	pub tv_sec: __kernel_old_time_t, // seconds
	pub tv_usec: __kernel_suseconds_t, // microseconds
}

#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct itimerspec {
	pub it_interval: timespec, // timer period
	pub it_value: timespec,    // timer expiration
}

#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct itimerval {
	pub it_interval: timeval, // timer interval
	pub it_value: timeval,    // current value
}

#[repr(C)]
pub struct timezone {
	pub tz_minuteswest: i32, // minutes west of Greenwich
	pub tz_dsttime: i32,     // type of dst correction
}

/*
 * Names of the interval timers, and structure
 * defining a timer setting:
 */
pub const ITIMER_REAL: i32 = 0;
pub const ITIMER_VIRTUAL: i32 = 1;
pub const ITIMER_PROF: i32 = 2;

/*
 * The IDs of the various system clocks (for POSIX.1b interval timers):
 */
pub const CLOCK_REALTIME: i32 = 0;
pub const CLOCK_MONOTONIC: i32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
pub const CLOCK_MONOTONIC_RAW: i32 = 4;
pub const CLOCK_REALTIME_COARSE: i32 = 5;
pub const CLOCK_MONOTONIC_COARSE: i32 = 6;
pub const CLOCK_BOOTTIME: i32 = 7;
pub const CLOCK_REALTIME_ALARM: i32 = 8;
pub const CLOCK_BOOTTIME_ALARM: i32 = 9;
/*
 * The driver implementing this got removed. The clock ID is kept as a
 * place holder. Do not reuse!
 */
pub const CLOCK_SGI_CYCLE: i32 = 10;
pub const CLOCK_TAI: i32 = 11;

pub const MAX_CLOCKS: i32 = 16;

/*
 * AUX clock support. AUXiliary clocks are dynamically configured by
 * enabling a clock ID. These clock can be steered independently of
 * the core timekeeper. The kernel can support up to 8 auxiliary clocks, but
 * the actual limit depends on eventual architecture constraints vs. VDSO.
 */
pub const CLOCK_AUX: i32 = MAX_CLOCKS;
pub const MAX_AUX_CLOCKS: i32 = 8;
pub const CLOCK_AUX_LAST: i32 = CLOCK_AUX + MAX_AUX_CLOCKS - 1;

pub const CLOCKS_MASK: i32 = CLOCK_REALTIME | CLOCK_MONOTONIC;
pub const CLOCKS_MONO: i32 = CLOCK_MONOTONIC;

/*
 * The various flags for setting POSIX.1b interval timers:
 */
pub const TIMER_ABSTIME: i32 = 0x01;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
