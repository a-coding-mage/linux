/* SPDX-License-Identifier: GPL-2.0 */

/* Common definitions for cpuidle governors. */

/*
 * Idle state target residency threshold used for deciding whether or not to
 * check the time till the closest expected timer event.
 */
pub const RESIDENCY_THRESHOLD_NS: u64 = 15 * NSEC_PER_USEC;

/*
 * If the closest timer is in this range, the governor idle state selection need
 * not be adjusted after the scheduler tick has been stopped.
 */
pub const SAFE_TIMER_RANGE_NS: u64 = 2 * TICK_NSEC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
