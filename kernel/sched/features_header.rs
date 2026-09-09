/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Scheduler feature configuration translated from the C feature header.
 * C preprocessor configuration symbols are represented as Rust cfg features.
 */

pub const PLACE_LAG: bool = true;
pub const PLACE_DEADLINE_INITIAL: bool = true;
pub const PLACE_REL_DEADLINE: bool = true;
pub const RUN_TO_PARITY: bool = true;
pub const PREEMPT_SHORT: bool = true;

pub const NEXT_BUDDY: bool = false;
pub const PICK_BUDDY: bool = true;
pub const CACHE_HOT_BUDDY: bool = true;

pub const DELAY_DEQUEUE: bool = true;
pub const DELAY_ZERO: bool = true;

pub const PARANOID_AVG: bool = false;
pub const WAKEUP_PREEMPTION: bool = true;

#[cfg(feature = "CONFIG_HRTIMER_REARM_DEFERRED")]
pub const HRTICK: bool = true;
#[cfg(not(feature = "CONFIG_HRTIMER_REARM_DEFERRED"))]
pub const HRTICK: bool = false;

#[cfg(feature = "CONFIG_HRTIMER_REARM_DEFERRED")]
pub const HRTICK_DL: bool = true;
#[cfg(not(feature = "CONFIG_HRTIMER_REARM_DEFERRED"))]
pub const HRTICK_DL: bool = false;

pub const NONTASK_CAPACITY: bool = true;

#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub const TTWU_QUEUE: bool = false;
#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
pub const TTWU_QUEUE: bool = true;

pub const SIS_UTIL: bool = true;
pub const WARN_DOUBLE_CLOCK: bool = false;

/* HAVE_RT_PUSH_IPI controls whether this feature is defined in the C header. */
#[cfg(feature = "HAVE_RT_PUSH_IPI")]
#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub const RT_PUSH_IPI: bool = true;
#[cfg(feature = "HAVE_RT_PUSH_IPI")]
#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
pub const RT_PUSH_IPI: bool = false;

pub const RT_RUNTIME_SHARE: bool = false;
pub const LB_MIN: bool = false;
pub const ATTACH_AGE_LOAD: bool = true;

pub const WA_IDLE: bool = true;
pub const WA_WEIGHT: bool = true;
pub const WA_BIAS: bool = true;

/* UtilEstimation. Use estimated CPU utilization. */
pub const UTIL_EST: bool = true;

pub const LATENCY_WARN: bool = false;

pub const NI_RANDOM: bool = true;
pub const NI_RATE: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
